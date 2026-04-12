use std::{collections::HashMap, fs};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::{
    BaseType, BaseTypes, MapKeyType, MapKeyTypeObjectName, OrType, Property, Structure, TupleType,
    Type,
};

// TODO: Add CI to ensure the locally copied metaModel matches the one at this URL.
// const METAMODEL_URL: &str = "https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/metaModel/metaModel.json";

mod schema {
    // TODO: Add CI check to ensure that the locally copied schema still matches the GitHub source.
    typify::import_types!("metaModel.schema.json");
}

/// Converts from camelCase (or PascalCase) to snake_case.
fn camel_to_snake(camel: &str) -> String {
    let mut snake = String::with_capacity(camel.len() + 4);

    for (i, char) in camel.chars().enumerate() {
        if char.is_ascii_uppercase() {
            if i > 0 {
                snake.push('_');
            }
            snake.push(char.to_ascii_lowercase());
        } else {
            snake.push(char);
        }
    }

    snake
}

fn resolve_struct_properties(
    properties: Vec<Property>,
    extends: Vec<Type>,
    mixins: Vec<Type>,
    structs_map: &HashMap<String, Structure>,
) -> (Vec<Property>, Vec<TokenStream>) {
    let mut structure_props = properties;
    let mut mixin_props = Vec::with_capacity(extends.len() + mixins.len());
    mixins.into_iter().chain(extends).for_each(|type_| {
        let Type::ReferenceType(reference_type) = &type_ else {
            panic!("Expected mixin/extend type to be a reference: {:?}", type_);
        };
        // Inline mixin/extend structs which start with an underscore. This is for convenience.
        if reference_type.name.starts_with('_') {
            let type_ = structs_map.get(&reference_type.name);
            match type_ {
                Some(structure) => {
                    let (inner_sp, inner_mp) = resolve_struct_properties(
                        structure.properties.clone(),
                        structure.extends.clone(),
                        structure.mixins.clone(),
                        structs_map,
                    );
                    structure_props.extend(inner_sp);
                    mixin_props.extend(inner_mp);
                }
                _ => panic!("Could not inline type {}", reference_type.name),
            }
            return;
        }
        let prop_name = format_ident!("{}", camel_to_snake(&reference_type.name));
        let type_ = render_type(type_);
        mixin_props.push(quote! {
            #[serde(flatten)]
            pub #prop_name: #type_,
        });
    });
    (structure_props, mixin_props)
}

fn render_type(type_: Type) -> TokenStream {
    // Serde is stupid and always will be.
    // https://github.com/serde-rs/serde/issues/1475
    let type_ = if let Type::AndType(t) = type_ {
        match t.kind.as_str() {
            "tuple" => Type::TupleType(TupleType {
                items: t.items,
                kind: t.kind,
            }),
            "or" => Type::OrType(OrType {
                items: t.items,
                kind: t.kind,
            }),
            _ => Type::AndType(t),
        }
    } else {
        type_
    };

    match type_ {
        Type::ReferenceType(ref_type) => {
            let ident = format_ident!("{}", ref_type.name);
            quote! { #ident }
        }
        Type::ArrayType(array_type) => {
            let element_type = render_type(array_type.element);
            quote! { Vec<#element_type> }
        }
        Type::BaseType(base_type) => match base_type.name {
            BaseTypes::Uinteger => quote! { u32 },
            BaseTypes::Integer => quote! { i32 },
            // NOTE: Potentially pick a URI type and decode the latter two base types as that,
            // rather than as strings? This question has been the subject of controversy...
            BaseTypes::String | BaseTypes::RegExp | BaseTypes::Uri | BaseTypes::DocumentUri => {
                quote! { String }
            }
            BaseTypes::Boolean => quote! { bool },
            BaseTypes::Decimal => quote! { f32 },
            BaseTypes::Null => quote! { () },
        },
        Type::TupleType(tuple_type) => {
            let types = tuple_type.items.into_iter().map(render_type);
            quote! { (#( #types ),*) }
        }
        Type::MapType(map_type) => {
            let map_key_type = map_type.key;
            let key_type = match map_key_type {
                MapKeyType::ReferenceType(ref_type) => Type::ReferenceType(ref_type),
                MapKeyType::Object { kind, name } => {
                    let name = match name {
                        MapKeyTypeObjectName::Integer => BaseTypes::Integer,
                        MapKeyTypeObjectName::String => BaseTypes::String,
                        MapKeyTypeObjectName::Uri => BaseTypes::Uri,
                        MapKeyTypeObjectName::DocumentUri => BaseTypes::DocumentUri,
                    };
                    Type::BaseType(BaseType { kind, name })
                }
            };
            let key = render_type(key_type);
            let value = render_type(*map_type.value);
            quote! { HashMap<#key, #value> }
        }
        // TODO
        Type::OrType(or_type) => {
            quote! { i32 }
        }
        Type::StringLiteralType(string_literal) => {
            quote! { i32 }
        }
        t => panic!("Unsupported type: {t:?}"),
    }
}

fn main() {
    // Run the generator.
    let model_string =
        fs::read_to_string("xtask/metaModel.json").expect("No local metaModel copy found");

    let model: schema::MetaModel = serde_json::from_str(&model_string).unwrap();

    println!(
        "Generating types for LSP version {}...",
        model.meta_data.version
    );

    let structs_map: HashMap<String, Structure> = model
        .structures
        .clone()
        .into_iter()
        .map(|s| (s.name.clone(), s))
        .collect();

    let preamble = quote! {
        //! This file is generated by an xtask. Do not edit.
    };

    let imports = quote! {
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
    };

    let structures: Vec<TokenStream> = model
        .structures
        .into_iter()
        .flat_map(|structure| {
            // We inline these structs; consider them private and do not generate.
            if structure.name.starts_with('_') {
                return None;
            }

            // TODO: Add `Default` and/or `Copy` if all properties implement them.
            let mut attributes = quote! {
                #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
                #[serde(rename_all = "camelCase")]
            };
            let name = format_ident!("{}", structure.name);
            if let Some(note) = structure.deprecated {
                attributes = quote! {
                    #attributes
                    #[deprecated(note = #note)]
                };
            }
            let documentation = structure
                .documentation
                .map(|doc| {
                    let lines = doc.split('\n');
                    lines
                        .map(|line| {
                            let line = if line.is_empty() {
                                line.to_string()
                            } else {
                                [" ", line].concat()
                            };
                            quote! { #[doc = #line] }
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let has_kind = structure
                .properties
                .iter()
                .find(|property| property.name == "kind")
                .is_some();
            let (structure_props, mixin_props) = resolve_struct_properties(
                structure.properties,
                structure.extends,
                structure.mixins,
                &structs_map,
            );
            let properties = structure_props
                .into_iter()
                .map(|property| {
                    let deprecated = property.deprecated.map(|note| {
                        quote! {
                            #[deprecated(note = #note)]
                        }
                    });
                    let documentation = property
                        .documentation
                        .map(|doc| {
                            let lines = doc.split('\n');
                            lines
                                .map(|line| {
                                    let line = if line.is_empty() {
                                        line.to_string()
                                    } else {
                                        [" ", line].concat()
                                    };
                                    quote! { #[doc = #line] }
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    let (name, mut serde_attributes) = if property.name == "type" {
                        assert!(
                            !has_kind,
                            "Structure {} already has `kind` property",
                            structure.name
                        );
                        (format_ident!("kind"), quote! { #[serde(rename = "type")] })
                    } else {
                        (
                            format_ident!("{}", camel_to_snake(&property.name)),
                            quote! {},
                        )
                    };
                    let mut type_ = render_type(property.type_);

                    if property.optional == Some(true) {
                        serde_attributes = quote! {
                            #serde_attributes
                            #[serde(skip_serializing_if = "Option::is_none")]
                        };
                        type_ = quote! {
                            Option<#type_>
                        }
                    }

                    quote! {
                        #(#documentation)*
                        #deprecated
                        #serde_attributes
                        pub #name: #type_,
                    }
                })
                .chain(mixin_props);

            Some(quote! {
                #(#documentation)*
                #attributes
                pub struct #name {
                    #(#properties)*
                }
            })
        })
        .collect();

    let all_items = [&[preamble], &[imports], &structures[..]].concat();

    let formatted_items: Vec<_> = all_items
        .into_iter()
        .map(|request| {
            let syntax_tree = syn::parse2(request).unwrap();

            prettyplease::unparse(&syntax_tree)
        })
        .collect();

    fs::write("src/generated.rs", formatted_items.join("\n")).unwrap();

    println!("Generation complete! 🌟");
}

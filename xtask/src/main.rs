use std::{collections::HashMap, fs, iter, sync::LazyLock};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::{Captures, Regex};

use crate::schema::{
    BaseType, BaseTypes, Enumeration, EnumerationEntryValue, EnumerationTypeName, MapKeyType,
    MapKeyTypeObjectName, OrType, Property, ReferenceType, Structure, TupleType, Type, TypeAlias,
};

// TODO: Add CI to ensure the locally copied metaModel matches the one at this URL.
// const METAMODEL_URL: &str = "https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.18/metaModel/metaModel.json";

mod schema {
    // TODO: Add CI check to ensure that the locally copied schema still matches the GitHub source.
    typify::import_types!("metaModel.schema.json");
}

static LINK_RE_1: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{@link +(\w+) ([\w ]+)\}").unwrap());
static LINK_RE_2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{@link +(\w+)\.(\w+) ([\w \.`]+)\}").unwrap());
static LINK_RE_3: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{@link +(\w+)\}").unwrap());
static LINK_RE_4: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{@link(code)? +(\w+)\.(\w+)\}").unwrap());

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

/// Converts from camelCase to PascalCase.
fn camel_to_pascal(mut camel: String) -> String {
    camel[0..1].make_ascii_uppercase();
    camel
}

fn render_documentation(documentation: Option<String>) -> TokenStream {
    let toks = documentation.into_iter().flat_map(|doc| {
        // Reformat documentation strings.
        let doc = doc.replace('\u{200B}', "");
        let doc = LINK_RE_1.replace_all(&doc, |caps: &Captures| {
            format!("[{}][{}]", &caps[2], &caps[1])
        });
        let doc = LINK_RE_2.replace_all(&doc, |caps: &Captures| {
            format!("[{}][`{}::{}`]", &caps[3], &caps[1], &caps[2])
        });
        let doc = LINK_RE_3.replace_all(&doc, |caps: &Captures| format!("[{}]", &caps[1]));
        let doc = LINK_RE_4.replace_all(&doc, |caps: &Captures| {
            format!("[`{}::{}`]", &caps[2], &caps[3])
        });

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
    });

    quote! {
        #(#toks)*
    }
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
        let type_ = render_type(type_, None, None);
        mixin_props.push(quote! {
            #[serde(flatten)]
            pub #prop_name: #type_,
        });
    });
    (structure_props, mixin_props)
}

/// Render an LSP type.
///
/// Parameters:
///
/// * `type` - The type to be rendered.
/// * `parent_name` - The name of the parent struct for this property. Should be `None` if this type
///   does not represent a struct property.
/// * `optional` - Whether or not this property is optional. Should be `None` if this type does not
///   represent a struct property.
fn render_type(type_: Type, parent_name: Option<&str>, optional: Option<bool>) -> TokenStream {
    match type_ {
        Type::ReferenceType(ref_type) => {
            match ref_type.name.as_str() {
                "LSPAny" => return quote! { LspAny },
                "LSPObject" => return quote! { LspObject },
                "LSPArray" => return quote! { LspArray },
                _ => {}
            }
            let ident = format_ident!("{}", ref_type.name);
            // Add type indirection to prevent infinite struct size.
            if parent_name.is_some_and(|pt| pt == ref_type.name) {
                quote! { Box<#ident> }
            } else {
                quote! { #ident }
            }
        }
        Type::ArrayType(array_type) => {
            let element_type = render_type(array_type.element, None, None);
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
            let types = tuple_type
                .items
                .into_iter()
                .map(|item| render_type(item, None, None));
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
            let key = render_type(key_type, None, None);
            let value = render_type(*map_type.value, None, None);
            quote! { HashMap<#key, #value> }
        }
        Type::StringLiteralType(e) => {
            panic!("String literal types should be handled specially: {e:?}");
        }
        Type::OrType(or_type) => {
            let len = or_type.items.len();
            let ident = format_ident!("Or{}", len);
            let mut filtered = false;
            let vals = or_type
                .items
                .into_iter()
                .filter(|item| {
                    if optional != Some(false) {
                        return true;
                    }
                    if matches!(
                        item,
                        Type::BaseType(BaseType {
                            kind: _,
                            name: BaseTypes::Null
                        })
                    ) {
                        filtered = true;
                        false
                    } else {
                        true
                    }
                })
                .map(|item| render_type(item, None, None))
                .collect::<Vec<_>>();
            if filtered {
                if len == 2 {
                    quote! { Option<#(#vals),*> }
                } else {
                    let ident = format_ident!("Or{}", len - 1);
                    quote! { Option<#ident<#(#vals),*>> }
                }
            } else {
                quote! { #ident<#(#vals),*> }
            }
        }
        Type::StructureLiteralType(struct_lit) => {
            assert!(
                struct_lit.value.properties.is_empty(),
                "Currently only empty struct literals are supported."
            );
            quote! { LspObject }
        }
        t => panic!("Unsupported type: {t:?}"),
    }
}

fn get_struct_derives(
    structure: &Structure,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> Vec<&'static str> {
    // Start with the commonly shared derives.
    let mut derives = vec!["Serialize", "Deserialize", "PartialEq", "Debug", "Clone"];

    let mut eqable = true;
    let mut defaultable = true;
    let mut copyable = true;

    for (prop_type, optional) in structure
        .properties
        .iter()
        .map(|prop| (&prop.type_, prop.optional.unwrap_or_default()))
        .chain(structure.mixins.iter().map(|mix| (mix, false)))
        .chain(structure.extends.iter().map(|extend| (extend, false)))
    {
        if let Type::ReferenceType(ReferenceType { kind: _, name }) = prop_type
            && name == &structure.name
        {
            copyable = false;
            continue;
        }

        if eqable && has_float(prop_type, structs_map, type_aliases_map) {
            eqable = false;
        }
        if defaultable && !optional && !is_defaultable(prop_type, structs_map, type_aliases_map) {
            defaultable = false;
        }
        if copyable && !is_copyable(prop_type, structs_map, enums_map, type_aliases_map) {
            copyable = false;
        }
    }
    if eqable {
        derives.push("Eq");
    }
    if defaultable {
        derives.push("Default");
    }
    if copyable {
        derives.push("Copy");
    }
    derives
}

fn get_enum_derives(enumeration: &Enumeration) -> Vec<&'static str> {
    let mut derives = vec!["PartialEq", "Eq", "Debug", "Clone"];
    if matches!(
        enumeration.type_.name,
        EnumerationTypeName::Integer | EnumerationTypeName::Uinteger
    ) {
        derives.push("Copy");
    } else {
        derives.push("Serialize");
        derives.push("Deserialize");
        if enumeration.supports_custom_values != Some(true) {
            derives.push("Copy");
        }
    }
    derives
}

fn is_nullable(type_: &Type) -> bool {
    match type_ {
        Type::OrType(or_type) => or_type
            .items
            .iter()
            .find(|item| {
                matches!(
                    item,
                    Type::BaseType(BaseType {
                        kind: _,
                        name: BaseTypes::Null
                    })
                )
            })
            .is_some(),
        Type::BaseType(BaseType {
            kind: _,
            name: BaseTypes::Null,
        }) => true,
        _ => false,
    }
}

fn is_defaultable(
    type_: &Type,
    structs_map: &HashMap<String, Structure>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> bool {
    match type_ {
        Type::ArrayType(_)
        | Type::MapType(_)
        | Type::StringLiteralType(_)
        | Type::IntegerLiteralType(_)
        | Type::StructureLiteralType(_)
        | Type::BooleanLiteralType(_) => true,
        Type::BaseType(BaseType { kind: _, name }) => {
            !matches!(name, BaseTypes::DocumentUri | BaseTypes::Uri)
        }
        Type::OrType(or_type) => or_type
            .items
            .iter()
            .find(|item| {
                matches!(
                    item,
                    Type::BaseType(BaseType {
                        kind: _,
                        name: BaseTypes::Null
                    })
                )
            })
            .is_some(),
        Type::TupleType(tuple_type) => tuple_type
            .items
            .iter()
            .all(|item| is_defaultable(item, structs_map, type_aliases_map)),
        Type::AndType(and_type) => and_type
            .items
            .iter()
            .all(|item| is_defaultable(item, structs_map, type_aliases_map)),
        Type::ReferenceType(ref_type) => {
            if let Some(structure) = structs_map.get(&ref_type.name) {
                structure
                    .properties
                    .iter()
                    .filter_map(|prop| {
                        if prop.optional == Some(true) {
                            None
                        } else {
                            Some(&prop.type_)
                        }
                    })
                    .chain(&structure.mixins)
                    .chain(&structure.extends)
                    .all(|prop_type| is_defaultable(prop_type, structs_map, type_aliases_map))
            } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                is_defaultable(&type_alias.type_, structs_map, type_aliases_map)
            } else {
                false
            }
        }
    }
}

fn is_copyable(
    type_: &Type,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> bool {
    match type_ {
        Type::ArrayType(_)
        | Type::MapType(_)
        | Type::StringLiteralType(_)
        | Type::StructureLiteralType(_)
        | Type::AndType(_) => false,
        Type::IntegerLiteralType(_) | Type::BooleanLiteralType(_) => true,
        Type::BaseType(BaseType { kind: _, name }) => {
            matches!(
                name,
                BaseTypes::Boolean
                    | BaseTypes::Integer
                    | BaseTypes::Decimal
                    | BaseTypes::Uinteger
                    | BaseTypes::Null
            )
        }
        Type::OrType(or_type) => or_type
            .items
            .iter()
            .all(|item| is_copyable(item, structs_map, enums_map, type_aliases_map)),
        Type::TupleType(tuple_type) => tuple_type
            .items
            .iter()
            .all(|item| is_copyable(item, structs_map, enums_map, type_aliases_map)),
        Type::ReferenceType(ref_type) => {
            if let Some(structure) = structs_map.get(&ref_type.name) {
                structure
                    .properties
                    .iter()
                    .map(|prop| &prop.type_)
                    .chain(&structure.mixins)
                    .chain(&structure.extends)
                    .all(|prop_type| {
                        is_copyable(prop_type, structs_map, enums_map, type_aliases_map)
                    })
            } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                is_copyable(&type_alias.type_, structs_map, enums_map, type_aliases_map)
            } else if let Some(enumeration) = enums_map.get(&ref_type.name) {
                get_enum_derives(enumeration).contains(&"Copy")
            } else {
                false
            }
        }
    }
}

fn has_float(
    type_: &Type,
    structs_map: &HashMap<String, Structure>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> bool {
    match type_ {
        Type::BaseType(BaseType {
            kind: _,
            name: BaseTypes::Decimal,
        }) => true,
        Type::ReferenceType(ref_type) => {
            if let Some(structure) = structs_map.get(&ref_type.name) {
                for prop in structure
                    .properties
                    .iter()
                    .map(|prop| &prop.type_)
                    .chain(&structure.extends)
                    .chain(&structure.mixins)
                {
                    if has_float(prop, structs_map, type_aliases_map) {
                        return true;
                    }
                }
                return false;
            }
            if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                return has_float(&type_alias.type_, structs_map, type_aliases_map);
            }
            false
        }
        _ => false,
    }
}

fn render_structure(
    structure: Structure,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> Option<TokenStream> {
    // We inline these structs; consider them private and do not generate.
    if structure.name.starts_with('_') {
        return None;
    }

    let derives = get_struct_derives(&structure, structs_map, enums_map, type_aliases_map)
        .into_iter()
        .map(|derive| format_ident!("{derive}"));
    let mut attributes = quote! {
        #[derive(#(#derives),*)]
        #[serde(rename_all = "camelCase")]
    };
    let name = format_ident!("{}", structure.name);
    if let Some(note) = structure.deprecated {
        attributes = quote! {
            #attributes
            #[deprecated(note = #note)]
        };
    }
    let documentation = render_documentation(structure.documentation);
    let has_kind = structure
        .properties
        .iter()
        .find(|property| property.name == "kind")
        .is_some();
    let (structure_props, mixin_props) = resolve_struct_properties(
        structure.properties,
        structure.extends,
        structure.mixins,
        structs_map,
    );
    let mut string_lit_prop = None;

    let properties: Vec<_> = structure_props
        .into_iter()
        .flat_map(|property| {
            if matches!(property.type_, Type::StringLiteralType(_)) {
                string_lit_prop = Some(property);
                return None;
            }
            let deprecated = property.deprecated.map(|note| {
                quote! {
                    #[deprecated(note = #note)]
                }
            });
            let documentation = render_documentation(property.documentation);

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

            if is_nullable(&property.type_) {
                if property.optional == Some(true) {
                    serde_attributes = quote! {
                        #serde_attributes
                        #[serde(default, deserialize_with = "deserialize_some")]
                    };
                } else {
                    serde_attributes = quote! {
                        #serde_attributes
                        #[serde(deserialize_with = "Option::deserialize")]
                    };
                }
            }

            let mut type_ = render_type(
                property.type_,
                Some(&structure.name),
                property.optional.or(Some(false)),
            );

            if property.optional == Some(true) {
                serde_attributes = quote! {
                    #serde_attributes
                    #[serde(skip_serializing_if = "Option::is_none")]
                };
                type_ = quote! {
                    Option<#type_>
                }
            }

            Some(quote! {
                #documentation
                #deprecated
                #serde_attributes
                pub #name: #type_,
            })
        })
        .chain(mixin_props)
        .collect();

    let shadow = if let Some(strlit_prop) = string_lit_prop {
        let shadow_name = format!("Shadow{}", name);
        let ident = format_ident!("{}", shadow_name);
        let prop_name = format_ident!("{}", camel_to_snake(&strlit_prop.name));
        let Type::StringLiteralType(lit_type) = strlit_prop.type_ else {
            unreachable!()
        };
        let prop_value = lit_type.value;
        let err = format!("Invalid value for prop {}: {{}}", strlit_prop.name);
        let structure = &structs_map
            .get(&structure.name)
            .expect("Structure not found");
        let (try_from_props, from_props): (Vec<TokenStream>, Vec<TokenStream>) = structure
            .properties
            .iter()
            .flat_map(|prop| {
                if matches!(prop.type_, Type::StringLiteralType(_)) {
                    return None;
                }
                let orig_name = format_ident!("{}", camel_to_snake(&prop.name));
                Some((
                    quote! {
                        #orig_name: shadow.#orig_name,
                    },
                    quote! {
                        #orig_name: original.#orig_name,
                    },
                ))
            })
            .chain(
                structure
                    .mixins
                    .clone()
                    .into_iter()
                    .chain(structure.extends.clone())
                    .map(|prop| {
                        let Type::ReferenceType(ref_type) = prop else {
                            unreachable!()
                        };
                        let orig_name = format_ident!("{}", camel_to_snake(&ref_type.name));
                        (
                            quote! {
                                #orig_name: shadow.#orig_name,
                            },
                            quote! {
                                #orig_name: original.#orig_name,
                            },
                        )
                    }),
            )
            .unzip();
        let shadow = quote! {
            #attributes
            struct #ident {
                #(#properties)*
                pub #prop_name: String,
            }

            impl TryFrom<#ident> for #name {
                type Error = String;
                fn try_from(shadow: #ident) -> Result<Self, Self::Error> {
                    if shadow.#prop_name != #prop_value {
                        return Err(format!(#err, shadow.#prop_name));
                    }
                    Ok(#name { #(#try_from_props)* })
                }
            }

            impl From<#name> for #ident {
                fn from(original: #name) -> Self {
                    #ident {
                        #(#from_props)*
                        #prop_name: #prop_value.to_string(),
                    }
                }
            }
        };

        attributes = quote! {
            #attributes
            #[serde(try_from = #shadow_name, into = #shadow_name)]
        };

        Some(shadow)
    } else {
        None
    };

    Some(quote! {
        #documentation
        #attributes
        pub struct #name {
            #(#properties)*
        }
        #shadow
    })
}

fn render_enumeration(enumeration: Enumeration) -> TokenStream {
    let derives = get_enum_derives(&enumeration)
        .into_iter()
        .map(|derive| format_ident!("{derive}"));
    let mut attributes = quote! {
        #[derive(#(#derives),*)]
    };

    let documentation = render_documentation(enumeration.documentation);
    let is_int_enum = matches!(
        enumeration.type_.name,
        EnumerationTypeName::Integer | EnumerationTypeName::Uinteger
    );

    if let Some(note) = enumeration.deprecated {
        attributes = quote! {
            #attributes
            #[deprecated(note = #note)]
        };
    }

    let name = format_ident!("{}", enumeration.name);

    let (mut sers, mut desers) = if is_int_enum {
        (
            Vec::with_capacity(enumeration.values.len()),
            Vec::with_capacity(enumeration.values.len()),
        )
    } else {
        (Vec::new(), Vec::new())
    };

    let (serializer, value_type) = match enumeration.type_.name {
        EnumerationTypeName::Uinteger => (Some(quote! { serialize_u32 }), Some(quote! { u32 })),
        EnumerationTypeName::Integer => (Some(quote! { serialize_i32 }), Some(quote! { i32 })),
        EnumerationTypeName::String => (None, None),
    };

    let mut values: Vec<TokenStream> = enumeration
        .values
        .into_iter()
        .map(|item| {
            let documentation = render_documentation(item.documentation);
            let deprecated = item.deprecated.map(|note| {
                quote! { #[deprecated(note = #note)] }
            });
            if is_int_enum {
                let ident = format_ident!("{}", camel_to_pascal(item.name));
                let full_name = quote! { #name::#ident };
                let EnumerationEntryValue::Number(value) = item.value else {
                    panic!("Non-number item in integer enum: {:?}", item.value);
                };
                let value = match enumeration.type_.name {
                    EnumerationTypeName::Uinteger => {
                        let value = value as u32;
                        quote! { #value }
                    }
                    EnumerationTypeName::Integer => {
                        let value = value as i32;
                        quote! { #value }
                    }
                    EnumerationTypeName::String => unreachable!(),
                };
                desers.push(quote! { #value => Ok(#full_name), });
                sers.push(quote! { #full_name => serializer.#serializer(#value), });
                quote! {
                    #documentation
                    #deprecated
                    #ident,
                }
            } else {
                let EnumerationEntryValue::String(value) = item.value else {
                    panic!("Non-string item in string enum: {:?}", item.value);
                };
                let ident = format_ident!("{}", camel_to_pascal(item.name));
                quote! {
                    #documentation
                    #deprecated
                    #[serde(rename = #value)]
                    #ident,
                }
            }
        })
        .collect();
    if enumeration.supports_custom_values == Some(true) {
        let (type_, attr) = match enumeration.type_.name {
            EnumerationTypeName::Uinteger => (quote! { u32 }, quote! {}),
            EnumerationTypeName::Integer => (quote! { i32 }, quote! {}),
            EnumerationTypeName::String => (quote! { String }, quote! { #[serde(untagged)] }),
        };
        if is_int_enum {
            let full_name = quote! { #name::Custom(custom) };
            sers.push(quote! { #full_name => serializer.#serializer(*custom), });
            desers.push(quote! { custom => Ok(#full_name), });
        }
        values.push(quote! {
            /// A custom value.
            #attr
            Custom(#type_)
        });
    } else if is_int_enum {
        let message = format!(
            "Unexpected value when deserializing {}: {{e}}",
            enumeration.name
        );
        desers.push(quote! { e => Err(serde::de::Error::custom(format!(#message))) })
    }

    let enum_tokens = quote! {
        #documentation
        #attributes
        pub enum #name {
            #(#values)*
        }
    };

    let custom_serde = if is_int_enum {
        Some(quote! {
            impl Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: serde::Serializer
                {
                    match self {
                        #(#sers)*
                    }
                }
            }
            impl<'de> Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<#name, D::Error>
                    where D: serde::Deserializer<'de>
                {
                    let value = #value_type::deserialize(deserializer)?;
                    match value {
                        #(#desers)*
                    }
                }
            }
        })
    } else {
        None
    };

    quote! {
        #enum_tokens
        #custom_serde
    }
}

fn render_type_alias(type_alias: TypeAlias) -> TokenStream {
    let documentation = render_documentation(type_alias.documentation);
    let name = format_ident!("{}", type_alias.name);

    match type_alias.name.as_str() {
        "LSPObject" => {
            return quote! {
                #documentation
                pub type LspObject = HashMap<String, LspAny>;
            };
        }
        "LSPAny" => {
            return quote! {
                #documentation
                pub type LspAny = serde_json::Value;
            };
        }
        "LSPArray" => {
            return quote! {
                #documentation
                pub type LspArray = Vec<LspAny>;
            };
        }
        _ => {}
    };

    let type_ = render_type(type_alias.type_, None, None);

    quote! {
        #documentation
        pub type #name = #type_;
    }
}

fn fix_serde_stupidity(type_: &mut Type) {
    // Serde is stupid and always will be.
    // https://github.com/serde-rs/serde/issues/1475
    if let Type::AndType(t) = type_ {
        match t.kind.as_str() {
            "tuple" => {
                *type_ = Type::TupleType(TupleType {
                    items: t.items.clone(),
                    kind: t.kind.clone(),
                })
            }
            "or" => {
                *type_ = Type::OrType(OrType {
                    items: t.items.clone(),
                    kind: t.kind.clone(),
                })
            }
            _ => {}
        }
    }

    match type_ {
        Type::AndType(and_type) => {
            for type_ in &mut and_type.items {
                fix_serde_stupidity(type_);
            }
        }
        Type::TupleType(tuple_type) => {
            for type_ in &mut tuple_type.items {
                fix_serde_stupidity(type_);
            }
        }
        Type::OrType(or_type) => {
            for type_ in &mut or_type.items {
                fix_serde_stupidity(type_);
            }
        }
        Type::ArrayType(array_type) => {
            fix_serde_stupidity(&mut array_type.element);
        }
        Type::MapType(map_type) => {
            fix_serde_stupidity(&mut map_type.value);
        }
        _ => {}
    }
}

fn main() {
    // Run the generator.
    let model_string =
        fs::read_to_string("xtask/metaModel.json").expect("No local metaModel copy found");

    let mut model: schema::MetaModel = serde_json::from_str(&model_string).unwrap();

    println!(
        "Generating types for LSP version {}...",
        model.meta_data.version
    );

    // Iterate over every possible thing that can be a `Type` and correct its deserialization.
    // Thanks serde!
    for type_ in model.type_aliases.iter_mut().map(|ta| &mut ta.type_) {
        fix_serde_stupidity(type_);
    }
    for type_ in model.structures.iter_mut().flat_map(|structure| {
        structure
            .properties
            .iter_mut()
            .map(|prop| &mut prop.type_)
            .chain(&mut structure.extends)
            .chain(&mut structure.mixins)
    }) {
        fix_serde_stupidity(type_);
    }
    for type_ in model.notifications.iter_mut().flat_map(|noti| {
        noti.registration_options
            .iter_mut()
            .chain(noti.params.iter_mut().flat_map(|p| {
                p.subtype_0
                    .iter_mut()
                    .chain(p.subtype_1.iter_mut().flatten())
            }))
    }) {
        fix_serde_stupidity(type_);
    }
    for type_ in model.requests.iter_mut().flat_map(|req| {
        req.registration_options
            .iter_mut()
            .chain(req.error_data.iter_mut())
            .chain(req.partial_result.iter_mut())
            .chain(std::iter::once(&mut req.result))
            .chain(req.params.iter_mut().flat_map(|p| {
                p.subtype_0
                    .iter_mut()
                    .chain(p.subtype_1.iter_mut().flatten())
            }))
    }) {
        fix_serde_stupidity(type_);
    }

    let model = model;

    let structs_map: HashMap<String, Structure> = model
        .structures
        .clone()
        .into_iter()
        .map(|s| (s.name.clone(), s))
        .collect();

    let enums_map: HashMap<String, Enumeration> = model
        .enumerations
        .clone()
        .into_iter()
        .map(|e| (e.name.clone(), e))
        .collect();

    let type_aliases_map: HashMap<String, TypeAlias> = model
        .type_aliases
        .clone()
        .into_iter()
        .map(|ta| (ta.name.clone(), ta))
        .collect();

    let preamble = quote! {
        //! This file is generated by an xtask. Do not edit.
        #![allow(deprecated, clippy::doc_lazy_continuation, unreachable_patterns)]
    };

    let imports = quote! {
        use serde::{Deserialize, Deserializer, Serialize};
        use std::collections::HashMap;
    };

    let predefs = quote! {
        /// This allows a field to have two types.
        #[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
        #[serde(untagged)]
        pub enum Or2<T, U> {
            T(T),
            U(U),
        }

        /// This allows a field to have three types.
        #[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
        #[serde(untagged)]
        pub enum Or3<T, U, V> {
            T(T),
            U(U),
            V(V),
        }

        /// This allows a field to have four types.
        #[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
        #[serde(untagged)]
        pub enum Or4<T, U, V, W> {
            T(T),
            U(U),
            V(V),
            W(W),
        }

        /// This allows a field to have five types.
        #[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
        #[serde(untagged)]
        pub enum Or5<T, U, V, W, X> {
            T(T),
            U(U),
            V(V),
            W(W),
            X(X),
        }

        /// This allows a field to have six types.
        #[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
        #[serde(untagged)]
        pub enum Or6<T, U, V, W, X, Y> {
            T(T),
            U(U),
            V(V),
            W(W),
            X(X),
            Y(Y),
        }

        fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
        where
            T: Deserialize<'de>,
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Some)
        }
    };

    let structures = model.structures.into_iter().flat_map(|structure| {
        render_structure(structure, &structs_map, &enums_map, &type_aliases_map)
    });

    let enumerations = model.enumerations.into_iter().map(render_enumeration);

    let type_aliases = model.type_aliases.into_iter().map(render_type_alias);

    let all_items = iter::once(preamble)
        .chain(iter::once(imports))
        .chain(iter::once(predefs))
        .chain(structures)
        .chain(enumerations)
        .chain(type_aliases);

    let formatted_items: Vec<String> = all_items
        .map(|request| {
            let syntax_tree = syn::parse2(request).unwrap();

            prettyplease::unparse(&syntax_tree)
        })
        .collect();

    fs::write("src/generated.rs", formatted_items.join("\n")).unwrap();

    println!("Generation complete! 🌟");
}

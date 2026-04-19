use std::collections::{BTreeMap, HashMap};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    camel_to_pascal, camel_to_snake,
    derives::{get_enum_derives, get_struct_derives, has_float, is_copyable, is_hashable},
    is_nullable, method_to_pascal, render_documentation, resolve_struct_properties,
    schema::{
        BaseType, BaseTypes, Enumeration, EnumerationEntryValue, EnumerationTypeName, MapKeyType,
        MapKeyTypeObjectName, Notification, OrType, Request, Structure, Type, TypeAlias,
    },
};

fn collapse_null(type_: &mut Type) {
    if let Type::OrType(or_type) = type_ {
        or_type.items.retain(|item| {
            !matches!(
                item,
                Type::BaseType(BaseType {
                    kind: _,
                    name: BaseTypes::Null,
                })
            )
        });

        if or_type.items.len() == 1
            && let Some(item) = or_type.items.pop()
        {
            *type_ = item;
        }
    }
}

pub fn render_enum_ors(
    enum_or_types: &mut BTreeMap<String, (OrType, Option<TokenStream>)>,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> Vec<TokenStream> {
    let mut toks = Vec::new();

    while let Some((name, (or_type, documentation))) = enum_or_types.pop_first() {
        let mut derives = vec![
            "Serialize",
            "Deserialize",
            "PartialEq",
            "Debug",
            "Clone",
            "From",
        ];

        if or_type
            .items
            .iter()
            .all(|item| !has_float(item, structs_map, type_aliases_map))
        {
            derives.push("Eq");
            if or_type
                .items
                .iter()
                .all(|item| is_hashable(item, structs_map, type_aliases_map))
            {
                derives.push("Hash");
            }
        }

        if or_type
            .items
            .iter()
            .all(|item| is_copyable(item, structs_map, enums_map, type_aliases_map))
        {
            derives.push("Copy")
        }

        let all_prefixed = or_type.items.iter().all(|item| {
            if let Type::ReferenceType(ref_type) = item {
                ref_type.name.starts_with(&name) && ref_type.name.len() > name.len()
            } else {
                false
            }
        });
        let members = or_type.items.into_iter().map(|item| {
            if all_prefixed && let Type::ReferenceType(ref_type) = item.clone() {
                let member = ref_type.name.strip_prefix(&name).unwrap();
                let member_ident = format_ident!("{member}");
                let type_ = render_type(item, &ref_type.name, &None, enum_or_types);
                quote! {
                    #[from]
                    #member_ident(#type_)
                }
            } else {
                let mut attr = quote! { #[from] };
                let name = match item.clone() {
                    Type::ReferenceType(ref_type) => ref_type.name,
                    Type::BaseType(base_type) => match base_type.name {
                        BaseTypes::Integer | BaseTypes::Uinteger => String::from("Int"),
                        BaseTypes::Boolean => String::from("Bool"),
                        BaseTypes::DocumentUri | BaseTypes::Uri => String::from("Uri"),
                        BaseTypes::String => {
                            attr = quote! { #[from(String, &str, Box<str>, Cow<'_, str>, char)] };
                            String::from("String")
                        }
                        BaseTypes::Null => {
                            return quote! {
                                #[serde(rename = "null")]
                                #[from(())]
                                Null
                            };
                        }
                        a => unimplemented!("{a:?}"),
                    },
                    Type::TupleType(_) => String::from("Tuple"),
                    Type::StructureLiteralType(_) => String::from("Object"),
                    Type::ArrayType(array_type) => {
                        let inner_name = match array_type.element {
                            Type::ReferenceType(ref_type) => ref_type.name,
                            Type::BaseType(base_type) => match base_type.name {
                                BaseTypes::Integer | BaseTypes::Uinteger => String::from("Int"),
                                BaseTypes::Boolean => String::from("Bool"),
                                BaseTypes::DocumentUri | BaseTypes::Uri => String::from("Uri"),
                                BaseTypes::String => String::from("String"),
                                BaseTypes::Null => String::from("Null"),
                                a => unimplemented!("{a:?}"),
                            },
                            Type::TupleType(_) => String::from("Tuple"),
                            Type::StructureLiteralType(_) => String::from("Object"),
                            a => unimplemented!("{a:?}"),
                        };
                        format!("{inner_name}List")
                    }
                    a => unimplemented!("{a:?}"),
                };
                let type_ = render_type(item, &name, &None, &mut Default::default());
                let member_ident = format_ident!("{}", name);
                quote! {
                    #attr
                    #member_ident(#type_)
                }
            }
        });
        let name_ident = format_ident!("{name}");
        let derives = derives.iter().map(|d| format_ident!("{d}"));
        toks.push(quote! {
            #documentation
            #[derive(#(#derives),*)]
            #[serde(untagged)]
            pub enum #name_ident {
                #(#members),*
            }
        });
    }
    toks
}

pub fn render_request(
    request: Request,
    enum_or_types: &mut BTreeMap<String, (OrType, Option<TokenStream>)>,
) -> TokenStream {
    let Some(name) = request.type_name else {
        panic!("Unnamed request: {request:?}");
    };
    let documentation = render_documentation(request.documentation);
    let name_ident = format_ident!("{name}");
    let method = format_ident!("{}", method_to_pascal(&request.method));
    let message_direction =
        format_ident!("{}", camel_to_pascal(request.message_direction.to_string()));
    let params = if let Some(params) = request.params {
        assert_eq!(params.subtype_1, None);
        let mut params = params.subtype_0.expect("No request params found");
        let is_nullable = is_nullable(&params);
        if is_nullable {
            collapse_null(&mut params);
        }
        let mut type_ = render_type(params, &(name.clone() + "Params"), &None, enum_or_types);
        if is_nullable {
            type_ = quote! { Option<#type_> };
        }
        type_
    } else {
        quote! { () }
    };
    let mut result = request.result;
    let is_nullable = is_nullable(&result);
    if is_nullable {
        collapse_null(&mut result);
    }
    let mut result = render_type(result, &(name + "Response"), &None, enum_or_types);
    if is_nullable {
        result = quote! { Option<#result> };
    }
    quote! {
        #documentation
        #[derive(Debug)]
        pub struct #name_ident;

        impl Request for #name_ident {
            const METHOD: LspRequestMethods = LspRequestMethods::#method;
            const MESSAGE_DIRECTION: MessageDirection = MessageDirection::#message_direction;

            type Params = #params;
            type Result = #result;
        }
    }
}

pub fn render_notification(
    notification: Notification,
    enum_or_types: &mut BTreeMap<String, (OrType, Option<TokenStream>)>,
) -> TokenStream {
    let Some(name) = notification.type_name else {
        panic!("Unnamed request: {notification:?}");
    };
    let documentation = render_documentation(notification.documentation);
    let name_ident = format_ident!("{name}");
    let method = format_ident!("{}", method_to_pascal(&notification.method));
    let message_direction = format_ident!(
        "{}",
        camel_to_pascal(notification.message_direction.to_string())
    );
    let params = if let Some(params) = notification.params {
        assert_eq!(params.subtype_1, None);
        let mut param = params.subtype_0.expect("No request params found");
        let is_nullable = is_nullable(&param);
        if is_nullable {
            collapse_null(&mut param);
        }
        let mut type_ = render_type(param, &(name + "Params"), &None, enum_or_types);
        if is_nullable {
            type_ = quote! { Option<#type_> };
        }
        type_
    } else {
        quote! { () }
    };
    quote! {
        #documentation
        #[derive(Debug)]
        pub struct #name_ident;

        impl Notification for #name_ident {
            const METHOD: LspNotificationMethods = LspNotificationMethods::#method;
            const MESSAGE_DIRECTION: MessageDirection = MessageDirection::#message_direction;

            type Params = #params;
        }
    }
}

pub fn render_type_alias(
    type_alias: TypeAlias,
    enum_or_types: &mut BTreeMap<String, (OrType, Option<TokenStream>)>,
) -> Option<TokenStream> {
    let documentation = render_documentation(type_alias.documentation);
    let name = format_ident!("{}", type_alias.name);

    match type_alias.name.as_str() {
        "LSPObject" => {
            return Some(quote! {
                #documentation
                pub type LspObject = HashMap<String, LspAny>;
            });
        }
        "LSPAny" => {
            return Some(quote! {
                #documentation
                pub type LspAny = serde_json::Value;
            });
        }
        "LSPArray" => {
            return Some(quote! {
                #documentation
                pub type LspArray = Vec<LspAny>;
            });
        }
        _ => {}
    };

    let type_ = render_type(
        type_alias.type_,
        &type_alias.name,
        &Some(documentation.clone()),
        enum_or_types,
    );

    if enum_or_types.contains_key(&type_alias.name) {
        None
    } else {
        Some(quote! {
            #documentation
            pub type #name = #type_;
        })
    }
}

pub fn render_enumeration(enumeration: Enumeration) -> TokenStream {
    let derives = get_enum_derives(&enumeration)
        .into_iter()
        .map(|derive| format_ident!("{derive}"));
    let mut attributes = quote! {
        #[derive(#(#derives),*)]
    };

    let documentation = render_documentation(enumeration.documentation);

    if let Some(note) = enumeration.deprecated {
        attributes = quote! {
            #attributes
            #[deprecated(note = #note)]
        };
    }

    let name = if enumeration.name == "LSPErrorCodes" {
        String::from("LspErrorCodes")
    } else {
        enumeration.name
    };
    let name_ident = format_ident!("{}", name);

    let (mut sers, mut desers) = (
        Vec::with_capacity(enumeration.values.len()),
        Vec::with_capacity(enumeration.values.len()),
    );

    let value_type = match enumeration.type_.name {
        EnumerationTypeName::Uinteger => quote! { u32 },
        EnumerationTypeName::Integer => quote! { i32 },
        EnumerationTypeName::String => quote! { String },
    };

    let supports_custom = enumeration.supports_custom_values == Some(true);

    let value_type_str = value_type.to_string();
    if supports_custom {
        attributes = quote! {
            #attributes
            #[serde(into = #value_type_str, from = #value_type_str)]
        }
    } else {
        attributes = quote! {
            #attributes
            #[serde(into = #value_type_str, try_from = #value_type_str)]
        }
    }

    let is_str_enum = matches!(enumeration.type_.name, EnumerationTypeName::String);

    let mut values: Vec<TokenStream> = enumeration
        .values
        .into_iter()
        .map(|item| {
            let documentation = render_documentation(item.documentation);
            let deprecated = item.deprecated.map(|note| {
                quote! { #[deprecated(note = #note)] }
            });
            let ident = format_ident!("{}", camel_to_pascal(item.name));
            let value = match item.value {
                EnumerationEntryValue::Number(value) => {
                    if matches!(enumeration.type_.name, EnumerationTypeName::Uinteger) {
                        let value = value as u32;
                        quote! { #value }
                    } else {
                        let value = value as i32;
                        quote! { #value }
                    }
                }
                EnumerationEntryValue::String(string) => quote! { #string },
            };
            let full_name = quote! { #name_ident::#ident };
            if supports_custom {
                desers.push(quote! { #value => #full_name, });
            } else {
                desers.push(quote! { #value => Ok(#full_name), });
            }
            if is_str_enum {
                sers.push(quote! { #full_name => #value.to_string(), });
            } else {
                sers.push(quote! { #full_name => #value, });
            }
            quote! {
                #documentation
                #deprecated
                #ident,
            }
        })
        .collect();
    if supports_custom {
        values.push(quote! {
            /// A custom value.
            #[serde(untagged)]
            Custom(#value_type)
        });
        sers.push(quote! { #name_ident::Custom(any) => any, });
        desers.push(quote! { _ => #name_ident::Custom(v), });
    } else {
        let fmt = format!("Invalid {name_ident}: {{v}}");
        desers.push(quote! { _ => Err(format!(#fmt)), });
    }

    let enum_tokens = quote! {
        #documentation
        #attributes
        pub enum #name_ident {
            #(#values)*
        }
    };

    let (deser_trait, deser_method, trait_err, return_type) = if supports_custom {
        (
            format_ident!("From"),
            format_ident!("from"),
            None,
            quote! { Self },
        )
    } else {
        (
            format_ident!("TryFrom"),
            format_ident!("try_from"),
            Some(quote! { type Error = String; }),
            quote! { Result<Self, <Self as TryFrom<#value_type>>::Error> },
        )
    };
    let as_str = if is_str_enum {
        Some(quote! { .as_str() })
    } else {
        None
    };
    let display = if is_str_enum {
        let s = if supports_custom {
            quote! { self.clone() }
        } else {
            quote! { (*self) }
        };
        Some(quote! {
            impl fmt::Display for #name_ident {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let s: String = #s.into();
                    write!(f, "{s}")
                }
            }
        })
    } else {
        None
    };
    let traits = quote! {
        impl From<#name_ident> for #value_type {
            fn from(e: #name_ident) -> Self {
                match e {
                    #(#sers)*
                }
            }
        }

        impl #deser_trait<#value_type> for #name_ident {
            #trait_err

            fn #deser_method(v: #value_type) -> #return_type {
                match v #as_str {
                    #(#desers)*
                }
            }
        }

        #display
    };

    quote! {
        #enum_tokens
        #traits
    }
}

pub fn render_structure(
    structure: Structure,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
    enum_or_types: &mut BTreeMap<String, (OrType, Option<TokenStream>)>,
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
        .any(|property| property.name == "kind");
    let (structure_props, mixin_props) = resolve_struct_properties(
        structure.properties,
        structure.extends,
        structure.mixins,
        structs_map,
    );
    let mut string_lit_prop = None;

    let mut properties: Vec<_> = structure_props
        .clone()
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

            let is_nullable = is_nullable(&property.type_);

            let box_type = if let Type::ReferenceType(ref_type) = &property.type_
                && ref_type.name == structure.name
            {
                true
            } else if let Type::OrType(or_type) = &property.type_ {
                or_type.items.iter().any(|item| {
                    if let Type::ReferenceType(ref_type) = item
                        && ref_type.name == structure.name
                    {
                        true
                    } else {
                        false
                    }
                })
            } else {
                false
            };

            let mut or_name = camel_to_pascal(property.name);
            if structs_map.contains_key(&or_name)
                || enums_map.contains_key(&or_name)
                || type_aliases_map.contains_key(&or_name)
            {
                or_name = format!("{}{}", structure.name, or_name);
            } else if let Type::OrType(or_type) = &property.type_
                && let Some((enum_or, _)) = enum_or_types.get(&or_name)
                && enum_or != or_type
            {
                or_name = format!("{}{}", structure.name, or_name);
            }

            let is_optional = property.optional == Some(true);

            let mut type_ = property.type_;

            if is_nullable && !is_optional {
                collapse_null(&mut type_);
            }

            let mut type_ = render_type(type_, &or_name, &None, enum_or_types);

            if box_type {
                type_ = quote! { Box<#type_> };
            }

            if is_optional {
                if is_nullable {
                    serde_attributes = quote! {
                        #serde_attributes
                        #[serde(default, deserialize_with = "deserialize_some")]
                    };
                }
                serde_attributes = quote! {
                    #serde_attributes
                    #[serde(skip_serializing_if = "Option::is_none")]
                };
                type_ = quote! { Option<#type_> }
            } else if is_nullable {
                type_ = quote! { Option<#type_> }
            }

            Some(quote! {
                #documentation
                #deprecated
                #serde_attributes
                pub #name: #type_,
            })
        })
        .collect();

    properties.extend(mixin_props.clone().into_iter().map(|prop| {
        let name = format_ident!("{}", camel_to_snake(&prop.name));
        let type_ = render_type(
            prop.type_,
            &camel_to_pascal(prop.name),
            &None,
            enum_or_types,
        );
        quote! {
            #[serde(flatten)]
            pub #name: #type_,
        }
    }));

    let shadow = if let Some(strlit_prop) = string_lit_prop {
        let shadow_name = format!("Shadow{name}");
        let ident = format_ident!("{shadow_name}");
        let prop_name = format_ident!("{}", camel_to_snake(&strlit_prop.name));
        let Type::StringLiteralType(lit_type) = strlit_prop.type_ else {
            unreachable!()
        };
        let prop_value = lit_type.value;
        let err = format!("Invalid value for prop {}: {{}}", strlit_prop.name);
        let (try_from_props, from_props): (Vec<TokenStream>, Vec<TokenStream>) = structure_props
            .iter()
            .chain(&mixin_props)
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

/// Render an LSP type.
///
/// Parameters:
///
/// * `type` - The type to be rendered.
/// * `or_name` - The type name to give to an "or" type found within this type.
/// * `or_documentation` - The documentation for the cretaed "or" type, if any.
/// * `enum_or_types` - The "or" type to insert into if an "or" type is found. They will be aliased
///   and rendered as separate types, for better DX.
fn render_type(
    type_: Type,
    or_name: &str,
    or_documentation: &Option<TokenStream>,
    enum_or_types: &mut BTreeMap<String, (OrType, Option<TokenStream>)>,
) -> TokenStream {
    match type_ {
        Type::ReferenceType(ref_type) => {
            match ref_type.name.as_str() {
                "LSPAny" => return quote! { LspAny },
                "LSPObject" => return quote! { LspObject },
                "LSPArray" => return quote! { LspArray },
                _ => {}
            }
            let ident = format_ident!("{}", ref_type.name);
            quote! { #ident }
        }
        Type::ArrayType(array_type) => {
            let or_name = or_name.strip_suffix('s').unwrap_or(or_name);
            let element_type =
                render_type(array_type.element, or_name, or_documentation, enum_or_types);
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
                .map(|item| render_type(item, or_name, or_documentation, enum_or_types));
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
            let key = render_type(key_type, or_name, or_documentation, enum_or_types);
            let or_name = or_name.strip_suffix('s').unwrap_or(or_name);
            let value = render_type(*map_type.value, or_name, or_documentation, enum_or_types);
            quote! { HashMap<#key, #value> }
        }
        Type::StringLiteralType(e) => {
            panic!("String literal types should be handled specially: {e:?}");
        }
        Type::OrType(or_type) => {
            let ident = format_ident!("{or_name}");
            if let Some(enum_or_type) = enum_or_types.get(or_name)
                && or_type != enum_or_type.0
            {
                panic!("Definition conflict for {or_name}:\n\n{enum_or_type:?}\n\n{or_type:?}");
            }
            enum_or_types.insert(or_name.to_string(), (or_type, or_documentation.clone()));
            quote! { #ident }
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

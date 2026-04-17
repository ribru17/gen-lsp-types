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
                ref_type.name.starts_with(&name)
            } else {
                false
            }
        });
        let members = or_type.items.into_iter().map(|item| {
            if all_prefixed && let Type::ReferenceType(ref_type) = item.clone() {
                let member = ref_type.name.strip_prefix(&name).unwrap();
                let member_ident = format_ident!("{member}");
                let type_ = render_type(item, None, &ref_type.name, &None, enum_or_types);
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
                let type_ = render_type(item, None, &name, &None, &mut Default::default());
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
        render_type(
            params.subtype_0.expect("No request params found"),
            None,
            &(name.clone() + "Params"),
            &None,
            enum_or_types,
        )
    } else {
        quote! { () }
    };
    let result = render_type(
        request.result,
        None,
        &(name + "Response"),
        &None,
        enum_or_types,
    );
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
        render_type(
            params.subtype_0.expect("No request params found"),
            None,
            &(name + "Params"),
            &None,
            enum_or_types,
        )
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
        None,
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

    let name = if enumeration.name == "LSPErrorCodes" {
        String::from("LspErrorCodes")
    } else {
        enumeration.name
    };
    let name_ident = format_ident!("{}", name);

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
                let full_name = quote! { #name_ident::#ident };
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
            let full_name = quote! { #name_ident::Custom(custom) };
            sers.push(quote! { #full_name => serializer.#serializer(*custom), });
            desers.push(quote! { custom => Ok(#full_name), });
        }
        values.push(quote! {
            /// A custom value.
            #attr
            Custom(#type_)
        });
    } else if is_int_enum {
        let message = format!("Unexpected value when deserializing {}: {{e}}", name);
        desers.push(quote! { e => Err(serde::de::Error::custom(format!(#message))) })
    }

    let enum_tokens = quote! {
        #documentation
        #attributes
        pub enum #name_ident {
            #(#values)*
        }
    };

    let custom_serde = if is_int_enum {
        Some(quote! {
            impl Serialize for #name_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: serde::Serializer
                {
                    match self {
                        #(#sers)*
                    }
                }
            }
            impl<'de> Deserialize<'de> for #name_ident {
                fn deserialize<D>(deserializer: D) -> Result<#name_ident, D::Error>
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
        .find(|property| property.name == "kind")
        .is_some();
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

            if property.optional == Some(true) && is_nullable(&property.type_) {
                serde_attributes = quote! {
                    #serde_attributes
                    #[serde(default, deserialize_with = "deserialize_some")]
                };
            }

            // Generate these "or" types separately for better DX.
            let mut type_ = if (!is_nullable(&property.type_) || property.optional == Some(true))
                && let Type::OrType(or_type) = property.type_
            {
                let mut name = camel_to_pascal(property.name);
                // Name conflict: prefix structure name.
                if structs_map.contains_key(&name)
                    || enums_map.contains_key(&name)
                    || type_aliases_map.contains_key(&name)
                {
                    name = format!("{}{}", structure.name, name);
                } else if let Some((enum_or, _)) = enum_or_types.get(&name)
                    && *enum_or != or_type
                {
                    name = format!("{}{}", structure.name, name);
                }
                let ident = format_ident!("{name}");
                enum_or_types.insert(name, (or_type, None));
                quote! { #ident }
            } else {
                render_type(
                    property.type_,
                    Some(&structure.name),
                    &camel_to_pascal(property.name),
                    &None,
                    enum_or_types,
                )
            };

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
        .collect();

    properties.extend(mixin_props.clone().into_iter().map(|prop| {
        let name = format_ident!("{}", camel_to_snake(&prop.name));
        let type_ = render_type(
            prop.type_,
            None,
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
        let shadow_name = format!("Shadow{}", name);
        let ident = format_ident!("{}", shadow_name);
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
/// * `parent_name` - The name of the parent struct for this property. Should be `None` if this type
///   does not represent a struct property.
/// * `optional` - Whether or not this property is optional. Should be `None` if this type does not
///   represent a struct property.
/// * `or_name` - The type name to give to an "or" type found within this type.
/// * `enum_or_types` - The "or" type to insert into if an "or" type is found. They will be aliased
///   and rendered as separate types, for better DX.
fn render_type(
    type_: Type,
    parent_name: Option<&str>,
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
            // Add type indirection to prevent infinite struct size.
            if parent_name.is_some_and(|pt| pt == ref_type.name) {
                quote! { Box<#ident> }
            } else {
                quote! { #ident }
            }
        }
        Type::ArrayType(array_type) => {
            let or_name = or_name.strip_suffix('s').unwrap_or(or_name);
            let element_type = render_type(
                array_type.element,
                None,
                or_name,
                or_documentation,
                enum_or_types,
            );
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
                .map(|item| render_type(item, None, or_name, or_documentation, enum_or_types));
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
            let key = render_type(key_type, None, or_name, or_documentation, enum_or_types);
            let or_name = or_name.strip_suffix('s').unwrap_or(or_name);
            let value = render_type(
                *map_type.value,
                None,
                or_name,
                or_documentation,
                enum_or_types,
            );
            quote! { HashMap<#key, #value> }
        }
        Type::StringLiteralType(e) => {
            panic!("String literal types should be handled specially: {e:?}");
        }
        Type::OrType(or_type) => {
            if is_nullable(&Type::OrType(or_type.clone())) && or_type.items.len() == 2 {
                let type_ = or_type
                    .items
                    .into_iter()
                    .find(|item| {
                        *item
                            != Type::BaseType(BaseType {
                                name: BaseTypes::Null,
                                kind: "base".to_string(),
                            })
                    })
                    .expect("Should have non-null variant");
                let type_ =
                    render_type(type_, parent_name, or_name, or_documentation, enum_or_types);
                quote! { Option<#type_> }
            } else {
                let ident = format_ident!("{or_name}");
                enum_or_types.insert(or_name.to_string(), (or_type, or_documentation.clone()));
                quote! { #ident }
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

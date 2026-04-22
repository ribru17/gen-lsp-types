mod derives;
mod renderers;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs, iter,
    sync::LazyLock,
};

use proc_macro2::TokenStream;
use quote::quote;
use regex::{Captures, Regex};

use crate::{
    renderers::{
        render_enum_ors, render_enumeration, render_notification, render_request, render_structure,
        render_type_alias,
    },
    schema::{
        BaseType, BaseTypes, Enumeration, EnumerationEntry, EnumerationEntryValue, EnumerationType,
        EnumerationTypeName, MapKeyType, OrType, Property, Structure, TupleType, Type, TypeAlias,
    },
};

mod schema {
    typify::import_types!("metaModel.schema.json");
}

static LINK_RE_1: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{@link +(\w+) ([\w \[\]]+)\}").unwrap());
static LINK_RE_2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{@link +(\w+)\.(\w+) ([\w \.`]+)\}").unwrap());
static LINK_RE_3: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{@link +(\w+)\}").unwrap());
static LINK_RE_4: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{@link(code)? +(\w+)\.(\w+)\}").unwrap());

/// Convert a method name to `PascalCase`. E.g. `textDocument/diagnostic` => `TextDocumentDiagnostic`
fn method_to_pascal(method: &str) -> String {
    let mut result = String::with_capacity(method.len());
    let mut capitalize = true;

    for ch in method.chars() {
        match ch {
            '$' => {}
            '/' => capitalize = true,
            _ => {
                if capitalize {
                    result.push(ch.to_ascii_uppercase());
                    capitalize = false;
                } else {
                    result.push(ch);
                }
            }
        }
    }

    result
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

/// Converts from camelCase to `PascalCase`.
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

fn has_field_conflict(
    properties: &[Property],
    extends: &[Type],
    mixins: &[Type],
    structs_map: &HashMap<String, Structure>,
) -> bool {
    let mut seen = HashSet::from_iter(properties.iter().map(|p| p.name.as_str()));
    _has_field_conflict_impl(extends, mixins, structs_map, &mut seen)
}

fn _has_field_conflict_impl<'a: 'b, 'b>(
    extends: &'b [Type],
    mixins: &'b [Type],
    structs_map: &'a HashMap<String, Structure>,
    seen: &mut HashSet<&'b str>,
) -> bool {
    for type_ in mixins.iter().chain(extends) {
        let Type::ReferenceType(reference_type) = type_ else {
            panic!("Expected mixin/extend type to be a reference: {type_:?}");
        };
        if let Some(structure) = structs_map.get(&reference_type.name) {
            for prop in &structure.properties {
                let name = prop.name.as_str();
                if seen.contains(name) {
                    return true;
                } else {
                    seen.insert(name);
                }
            }
            if _has_field_conflict_impl(&structure.extends, &structure.mixins, structs_map, seen) {
                return true;
            }
        }
    }
    false
}

fn get_all_inner_properties(
    properties: Vec<Property>,
    mut extends: Vec<Type>,
    mut mixins: Vec<Type>,
    structs_map: &HashMap<String, Structure>,
    seen: &mut Option<HashSet<String>>,
) -> Vec<Property> {
    let mut result = Vec::new();
    if seen.is_none() {
        let set = HashSet::from_iter(properties.iter().map(|p| p.name.clone()));
        *seen = Some(set);
        result = properties;
    }

    mixins.append(&mut extends);

    while let Some(type_) = mixins.pop() {
        let Type::ReferenceType(reference_type) = type_ else {
            panic!("Expected mixin/extend type to be a reference: {type_:?}");
        };
        if let Some(structure) = structs_map.get(&reference_type.name) {
            for prop in &structure.properties {
                let name = &prop.name;
                if let Some(seen) = seen
                    && !seen.contains(name)
                {
                    seen.insert(name.clone());
                    result.push(prop.clone());
                }
            }
            mixins.extend(structure.mixins.clone());
            mixins.extend(structure.extends.clone());
        }
    }

    result
}

fn resolve_struct_properties(
    properties: Vec<Property>,
    extends: Vec<Type>,
    mixins: Vec<Type>,
    structs_map: &HashMap<String, Structure>,
) -> (Vec<Property>, Vec<Property>) {
    let has_conflict = has_field_conflict(&properties, &extends, &mixins, structs_map);
    if has_conflict {
        return (
            get_all_inner_properties(properties, extends, mixins, structs_map, &mut None),
            Vec::new(),
        );
    }
    let mut structure_props = properties;
    let mut mixin_props = Vec::with_capacity(extends.len() + mixins.len());
    mixins.into_iter().chain(extends).for_each(|type_| {
        let Type::ReferenceType(reference_type) = &type_ else {
            panic!("Expected mixin/extend type to be a reference: {type_:?}");
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
        // Create a fake property from the mixin type. These get flattened by the renderer, so we
        // don't need to resolve them.
        mixin_props.push(Property {
            name: reference_type.name.clone(),
            type_,
            deprecated: None,
            optional: None,
            documentation: None,
            proposed: None,
            since: None,
            since_tags: Vec::new(),
        });
    });
    (structure_props, mixin_props)
}

fn is_nullable(type_: &Type) -> bool {
    match type_ {
        Type::OrType(or_type) => or_type.items.iter().any(|item| {
            matches!(
                item,
                Type::BaseType(BaseType {
                    kind: _,
                    name: BaseTypes::Null
                })
            )
        }),
        _ => false,
    }
}

impl PartialEq for MapKeyType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MapKeyType::ReferenceType(a), MapKeyType::ReferenceType(b)) => a.name == b.name,
            (MapKeyType::Object { name: a, .. }, MapKeyType::Object { name: b, .. }) => a == b,
            _ => false,
        }
    }
}

impl Eq for MapKeyType {}

impl std::hash::Hash for MapKeyType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            MapKeyType::ReferenceType(r) => r.name.hash(state),
            MapKeyType::Object { kind: _, name } => name.hash(state),
        }
    }
}

impl PartialEq for OrType {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::BaseType(a), Type::BaseType(b)) => a.name == b.name,
            (Type::ReferenceType(a), Type::ReferenceType(b)) => a.name == b.name,
            (Type::MapType(a), Type::MapType(b)) => a.key == b.key && a.value == b.value,
            (Type::OrType(a), Type::OrType(b)) => a == b,
            (Type::AndType(a), Type::AndType(b)) => a.items == b.items,
            (Type::TupleType(a), Type::TupleType(b)) => a.items == b.items,
            (Type::ArrayType(a), Type::ArrayType(b)) => a.element == b.element,
            (Type::IntegerLiteralType(a), Type::IntegerLiteralType(b)) => a.value == b.value,
            (Type::BooleanLiteralType(a), Type::BooleanLiteralType(b)) => a.value == b.value,
            (Type::StringLiteralType(a), Type::StringLiteralType(b)) => a.value == b.value,
            (Type::StructureLiteralType(a), Type::StructureLiteralType(b)) => {
                assert!(a.value.properties.is_empty());
                assert!(b.value.properties.is_empty());
                true
            }
            _ => false,
        }
    }
}

impl Eq for Type {}

impl std::hash::Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Type::BaseType(b) => b.name.hash(state),
            Type::ReferenceType(b) => b.name.hash(state),
            Type::MapType(b) => {
                b.key.hash(state);
                b.value.hash(state);
            }
            Type::OrType(b) => b.items.hash(state),
            Type::AndType(b) => b.items.hash(state),
            Type::TupleType(b) => b.items.hash(state),
            Type::ArrayType(b) => b.element.hash(state),
            Type::IntegerLiteralType(b) => (b.value as i128).hash(state),
            Type::BooleanLiteralType(b) => b.value.hash(state),
            Type::StringLiteralType(b) => b.value.hash(state),
            Type::StructureLiteralType(b) => {
                assert!(b.value.properties.is_empty());
                // Do nothing; consider all structures equal.
            }
        }
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
        //! This file is automatically @generated by an xtask. Do not edit.
        #![allow(
            deprecated,
            clippy::doc_lazy_continuation,
            unreachable_patterns,
            clippy::large_enum_variant,
            clippy::too_many_arguments,
            rustdoc::invalid_codeblock_attributes
        )]
        #![cfg_attr(any(), rustfmt::skip)]
    };

    let imports = quote! {
        use serde::{de::DeserializeOwned, Deserialize, Deserializer, ser::SerializeSeq as _, Serialize};
        use std::{borrow::Cow, collections::HashMap, fmt};
    };

    let predefs = quote! {
        fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
        where
            T: Deserialize<'de>,
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Some)
        }

        /// Indicates in which direction a message is sent in the protocol.
        #[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
        pub enum MessageDirection {
            ClientToServer,
            ServerToClient,
            Both,
        }

        pub trait Notification {
            type Params: DeserializeOwned + Serialize + Send + Sync;
            const METHOD: LspNotificationMethods;
            const MESSAGE_DIRECTION: MessageDirection;
        }

        pub trait Request {
            type Params: DeserializeOwned + Serialize + Send + Sync;
            type Result: DeserializeOwned + Serialize + Send + Sync;
            const METHOD: LspRequestMethods;
            const MESSAGE_DIRECTION: MessageDirection;
        }

        #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
        /// URIs are transferred as strings. The URI's format is defined in https://tools.ietf.org/html/rfc3986.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct Uri(pub String);

        #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
        impl From<String> for Uri {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
        impl From<&str> for Uri {
            fn from(s: &str) -> Self {
                Self(s.into())
            }
        }

        #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
        impl From<Box<str>> for Uri {
            fn from(s: Box<str>) -> Self {
                Self(s.into())
            }
        }

        #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
        impl From<Cow<'_, str>> for Uri {
            fn from(s: Cow<'_, str>) -> Self {
                Self(s.into())
            }
        }

        #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
        impl AsRef<str> for Uri {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
        impl fmt::Display for Uri {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        #[cfg(feature = "url")]
        pub type Uri = url::Url;

        #[cfg(all(feature = "fluent-uri", not(feature = "url")))]
        pub type Uri = fluent_uri::Uri<String>;

        #[cfg(all(feature = "url", feature = "fluent-uri"))]
        compile_error!("Features 'url' and 'fluent-uri' are mutually exclusive and cannot be enabled together.");

        /// Represents a semantic token (serialized as five uintegers).
        #[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Hash)]
        pub struct SemanticToken {
            /// Token line number, relative to the start of the previous token.
            pub delta_line: u32,
            /// Token start character, relative to the start of the previous token (relative to 0 or
            /// the previous token’s start if they are on the same line).
            pub delta_start: u32,
            /// The length of the token.
            pub length: u32,
            /// Will be looked up in [`SemanticTokensLegend::token_types`]. We currently ask that
            /// `tokenType` < 65536.
            pub token_type: u32,
            /// Each set bit will be looked up in [`SemanticTokensLegend::token_modifiers`].
            pub token_modifiers_bitset: u32,
        }

        impl SemanticToken {
            fn deserialize_tokens<'de, D>(deserializer: D) -> Result<Vec<SemanticToken>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let data = Vec::<u32>::deserialize(deserializer)?;
                let chunks = data.chunks_exact(5);

                if !chunks.remainder().is_empty() {
                    return Result::Err(serde::de::Error::custom("Length is not divisible by 5"));
                }

                Result::Ok(
                    chunks
                        .map(|chunk| SemanticToken {
                            delta_line: chunk[0],
                            delta_start: chunk[1],
                            length: chunk[2],
                            token_type: chunk[3],
                            token_modifiers_bitset: chunk[4],
                        })
                        .collect(),
                )
            }

            fn serialize_tokens<S>(tokens: &[SemanticToken], serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(tokens.len() * 5))?;
                for token in tokens.iter() {
                    seq.serialize_element(&token.delta_line)?;
                    seq.serialize_element(&token.delta_start)?;
                    seq.serialize_element(&token.length)?;
                    seq.serialize_element(&token.token_type)?;
                    seq.serialize_element(&token.token_modifiers_bitset)?;
                }
                seq.end()
            }

            fn deserialize_optional_tokens<'de, D>(
                deserializer: D,
            ) -> Result<Option<Vec<SemanticToken>>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(transparent)]
                struct Wrapper {
                    #[serde(deserialize_with = "SemanticToken::deserialize_tokens")]
                    tokens: Vec<SemanticToken>,
                }

                Ok(Option::<Wrapper>::deserialize(deserializer)?.map(|wrapper| wrapper.tokens))
            }

            fn serialize_optional_tokens<S>(
                data: &Option<Vec<SemanticToken>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #[derive(Serialize)]
                #[serde(transparent)]
                struct Wrapper {
                    #[serde(serialize_with = "SemanticToken::serialize_tokens")]
                    tokens: Vec<SemanticToken>,
                }

                let opt = data.as_ref().map(|t| Wrapper { tokens: t.to_vec() });

                opt.serialize(serializer)
            }
        }
    };

    let mut enum_or_types = BTreeMap::new();

    let structures = model
        .structures
        .into_iter()
        .flat_map(|structure| {
            render_structure(
                structure,
                &structs_map,
                &enums_map,
                &type_aliases_map,
                &mut enum_or_types,
            )
        })
        .collect::<Vec<_>>();

    let enumerations = model
        .enumerations
        .into_iter()
        .chain(std::iter::once({
            let values = model
                .requests
                .iter()
                .map(|req| EnumerationEntry {
                    name: method_to_pascal(&req.method),
                    deprecated: None,
                    documentation: None,
                    proposed: None,
                    since: None,
                    since_tags: Vec::new(),
                    value: EnumerationEntryValue::String(req.method.to_string()),
                })
                .collect();
            Enumeration {
                deprecated: None,
                documentation: None,
                name: String::from("LspRequestMethods"),
                proposed: None,
                since: None,
                since_tags: Vec::new(),
                supports_custom_values: Some(true),
                type_: EnumerationType {
                    kind: "base".into(),
                    name: EnumerationTypeName::String,
                },
                values,
            }
        }))
        .chain(std::iter::once({
            let values = model
                .notifications
                .iter()
                .map(|noti| EnumerationEntry {
                    name: method_to_pascal(&noti.method),
                    deprecated: None,
                    documentation: None,
                    proposed: None,
                    since: None,
                    since_tags: Vec::new(),
                    value: EnumerationEntryValue::String(noti.method.to_string()),
                })
                .collect();
            Enumeration {
                deprecated: None,
                documentation: None,
                name: String::from("LspNotificationMethods"),
                proposed: None,
                since: None,
                since_tags: Vec::new(),
                supports_custom_values: Some(true),
                type_: EnumerationType {
                    kind: "base".into(),
                    name: EnumerationTypeName::String,
                },
                values,
            }
        }))
        .map(render_enumeration);

    let type_aliases = model
        .type_aliases
        .into_iter()
        .flat_map(|ta| render_type_alias(ta, &mut enum_or_types))
        .collect::<Vec<_>>();

    let requests = model
        .requests
        .into_iter()
        .map(|req| render_request(req, &mut enum_or_types))
        .collect::<Vec<_>>();

    let notifications = model
        .notifications
        .into_iter()
        .map(|noti| render_notification(noti, &mut enum_or_types))
        .collect::<Vec<_>>();

    let enum_ors = render_enum_ors(
        &mut enum_or_types,
        &structs_map,
        &enums_map,
        &type_aliases_map,
    );

    let all_items = iter::once(preamble)
        .chain(iter::once(imports))
        .chain(iter::once(predefs))
        .chain(structures)
        .chain(enumerations)
        .chain(type_aliases)
        .chain(enum_ors)
        .chain(requests)
        .chain(notifications);

    let formatted_items: Vec<String> = all_items
        .map(|request| {
            let syntax_tree = syn::parse2(request).unwrap();

            prettyplease::unparse(&syntax_tree)
        })
        .collect();

    fs::write("src/generated.rs", formatted_items.join("\n")).unwrap();

    println!("Generation complete! 🌟");
}

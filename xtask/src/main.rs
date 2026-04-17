use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs, iter,
    sync::LazyLock,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::{Captures, Regex};

use crate::schema::{
    BaseType, BaseTypes, Enumeration, EnumerationEntry, EnumerationEntryValue, EnumerationType,
    EnumerationTypeName, MapKeyType, MapKeyTypeObjectName, Notification, OrType, Property,
    ReferenceType, Request, Structure, TupleType, Type, TypeAlias,
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

/// Convert a method name to PascalCase. E.g. `textDocument/diagnostic` => `TextDocumentDiagnostic`
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
                    result.push(ch)
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
            panic!("Expected mixin/extend type to be a reference: {:?}", type_);
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
            panic!("Expected mixin/extend type to be a reference: {:?}", type_);
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
    let mut structure_props = properties;
    let mut mixin_props = Vec::with_capacity(extends.len() + mixins.len());
    if has_conflict {
        // TODO: Only inline the specific mixins/extends which cause conflicts?
        return (
            get_all_inner_properties(structure_props, extends, mixins, structs_map, &mut None),
            Vec::new(),
        );
    }
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

fn get_struct_derives(
    structure: &Structure,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> Vec<&'static str> {
    // Start with the commonly shared derives.
    let mut derives = vec!["Serialize", "Deserialize", "PartialEq", "Debug", "Clone"];

    let mut eqable = true;
    let mut hashable = true;
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

        let mut seen = HashSet::new();
        if eqable && has_float(prop_type, structs_map, type_aliases_map, &mut seen) {
            eqable = false;
        }
        let mut seen = HashSet::new();
        if eqable && hashable && !is_hashable(prop_type, structs_map, type_aliases_map, &mut seen) {
            hashable = false;
        }
        let mut seen = HashSet::new();
        if defaultable
            && !optional
            && !is_defaultable(prop_type, structs_map, type_aliases_map, &mut seen)
        {
            defaultable = false;
        }
        let mut seen = HashSet::new();
        if copyable
            && !is_copyable(
                prop_type,
                structs_map,
                enums_map,
                type_aliases_map,
                &mut seen,
            )
        {
            copyable = false;
        }
    }
    if eqable {
        derives.push("Eq");
        if hashable {
            derives.push("Hash");
        }
    }
    if defaultable {
        derives.push("Default");
    }
    if copyable {
        derives.push("Copy");
    }

    // Special derives.
    if matches!(structure.name.as_str(), "Position" | "Range") {
        derives.push("PartialOrd");
        derives.push("Ord");
    }

    derives
}

fn get_enum_derives(enumeration: &Enumeration) -> Vec<&'static str> {
    let mut derives = vec!["PartialEq", "Eq", "Hash", "Debug", "Clone"];
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

fn is_defaultable<'a: 'b, 'b>(
    type_: &'a Type,
    structs_map: &'b HashMap<String, Structure>,
    type_aliases_map: &'b HashMap<String, TypeAlias>,
    seen: &mut HashSet<&'b Type>,
) -> bool {
    if seen.contains(type_) {
        return true;
    }
    seen.insert(type_);
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
            .all(|item| is_defaultable(item, structs_map, type_aliases_map, seen)),
        Type::AndType(and_type) => and_type
            .items
            .iter()
            .all(|item| is_defaultable(item, structs_map, type_aliases_map, seen)),
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
                    .all(|prop_type| is_defaultable(prop_type, structs_map, type_aliases_map, seen))
            } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                is_defaultable(&type_alias.type_, structs_map, type_aliases_map, seen)
            } else {
                false
            }
        }
    }
}

fn is_copyable<'a: 'b, 'b>(
    type_: &'a Type,
    structs_map: &'b HashMap<String, Structure>,
    enums_map: &'b HashMap<String, Enumeration>,
    type_aliases_map: &'b HashMap<String, TypeAlias>,
    seen: &mut HashSet<&'b Type>,
) -> bool {
    if seen.contains(type_) {
        return true;
    }
    seen.insert(type_);
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
            .all(|item| is_copyable(item, structs_map, enums_map, type_aliases_map, seen)),
        Type::TupleType(tuple_type) => tuple_type
            .items
            .iter()
            .all(|item| is_copyable(item, structs_map, enums_map, type_aliases_map, seen)),
        Type::ReferenceType(ref_type) => {
            if let Some(structure) = structs_map.get(&ref_type.name) {
                structure
                    .properties
                    .iter()
                    .map(|prop| &prop.type_)
                    .chain(&structure.mixins)
                    .chain(&structure.extends)
                    .all(|prop_type| {
                        is_copyable(prop_type, structs_map, enums_map, type_aliases_map, seen)
                    })
            } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                is_copyable(
                    &type_alias.type_,
                    structs_map,
                    enums_map,
                    type_aliases_map,
                    seen,
                )
            } else if let Some(enumeration) = enums_map.get(&ref_type.name) {
                get_enum_derives(enumeration).contains(&"Copy")
            } else {
                false
            }
        }
    }
}

fn is_hashable<'a: 'b, 'b>(
    type_: &'a Type,
    structs_map: &'b HashMap<String, Structure>,
    type_aliases_map: &'b HashMap<String, TypeAlias>,
    seen: &mut HashSet<&'b Type>,
) -> bool {
    if seen.contains(type_) {
        return true;
    }
    seen.insert(type_);
    match type_ {
        Type::MapType(_) | Type::StructureLiteralType(_) => false,
        Type::StringLiteralType(_) | Type::IntegerLiteralType(_) | Type::BooleanLiteralType(_) => {
            true
        }
        Type::BaseType(BaseType { kind: _, name }) => !matches!(name, BaseTypes::Decimal),
        Type::ReferenceType(ref_type) => {
            if let Some(structure) = structs_map.get(&ref_type.name) {
                structure
                    .properties
                    .iter()
                    .map(|prop| &prop.type_)
                    .chain(&structure.mixins)
                    .chain(&structure.extends)
                    .all(|prop_type| is_hashable(prop_type, structs_map, type_aliases_map, seen))
            } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                is_hashable(&type_alias.type_, structs_map, type_aliases_map, seen)
            } else {
                true
            }
        }
        Type::ArrayType(array_type) => {
            is_hashable(&array_type.element, structs_map, type_aliases_map, seen)
        }
        Type::AndType(and_type) => and_type
            .items
            .iter()
            .all(|item| is_hashable(item, structs_map, type_aliases_map, seen)),
        Type::OrType(or_type) => or_type
            .items
            .iter()
            .all(|item| is_hashable(item, structs_map, type_aliases_map, seen)),
        Type::TupleType(tuple_type) => tuple_type
            .items
            .iter()
            .all(|item| is_hashable(item, structs_map, type_aliases_map, seen)),
    }
}

fn has_float<'a: 'b, 'b>(
    type_: &'a Type,
    structs_map: &'b HashMap<String, Structure>,
    type_aliases_map: &'b HashMap<String, TypeAlias>,
    seen: &mut HashSet<&'b Type>,
) -> bool {
    if seen.contains(type_) {
        return false;
    }
    seen.insert(type_);
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
                    if has_float(prop, structs_map, type_aliases_map, seen) {
                        return true;
                    }
                }
                return false;
            }
            if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                return has_float(&type_alias.type_, structs_map, type_aliases_map, seen);
            }
            false
        }
        Type::ArrayType(array_type) => {
            if let Type::ReferenceType(ref_type) = &array_type.element {
                if let Some(structure) = structs_map.get(&ref_type.name) {
                    for prop in structure
                        .properties
                        .iter()
                        .map(|prop| &prop.type_)
                        .chain(&structure.extends)
                        .chain(&structure.mixins)
                    {
                        if has_float(prop, structs_map, type_aliases_map, seen) {
                            return true;
                        }
                    }
                    return false;
                }
                if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                    return has_float(&type_alias.type_, structs_map, type_aliases_map, seen);
                }
            }
            false
        }
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
            (Type::StructureLiteralType(_), Type::StructureLiteralType(_)) => unimplemented!(),
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

fn render_structure(
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

            // Generate these "or" types separately for better DX. Ironically, this code has
            // terrible readability and could really use a refactor.
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
            } else if let Type::ArrayType(array_type) = property.type_ {
                if let Type::OrType(or_type) = array_type.element {
                    let mut name = camel_to_pascal(property.name);
                    name = name
                        .strip_suffix('s')
                        .map(|n| n.to_string())
                        .unwrap_or(name);
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
                    quote! { Vec<#ident> }
                } else {
                    render_type(
                        Type::ArrayType(array_type),
                        Some(&structure.name),
                        &camel_to_pascal(property.name),
                        &None,
                        enum_or_types,
                    )
                }
            } else if let Type::MapType(map_type) = property.type_ {
                if let Type::OrType(or_type) = *map_type.value {
                    let mut name = camel_to_pascal(property.name);
                    name = name
                        .strip_suffix('s')
                        .map(|n| n.to_string())
                        .unwrap_or(name);
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
                    enum_or_types.insert(name.clone(), (or_type, None));
                    render_type(
                        Type::MapType(schema::MapType {
                            key: map_type.key,
                            kind: "map".into(),
                            value: Type::ReferenceType(ReferenceType {
                                kind: "reference".into(),
                                name: camel_to_pascal(name.clone()),
                            })
                            .into(),
                        }),
                        Some(&structure.name),
                        &name,
                        &None,
                        enum_or_types,
                    )
                } else {
                    render_type(
                        Type::MapType(map_type),
                        Some(&structure.name),
                        camel_to_pascal(property.name).as_str(),
                        &None,
                        enum_or_types,
                    )
                }
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

fn render_type_alias(
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

fn render_enum_ors(
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

        let mut seen = HashSet::new();
        if or_type
            .items
            .iter()
            .all(|item| !has_float(item, structs_map, type_aliases_map, &mut seen))
        {
            derives.push("Eq");
            let mut seen = HashSet::new();
            if or_type
                .items
                .iter()
                .all(|item| is_hashable(item, structs_map, type_aliases_map, &mut seen))
            {
                derives.push("Hash");
            }
        }

        let mut seen = HashSet::new();
        if or_type
            .items
            .iter()
            .all(|item| is_copyable(item, structs_map, enums_map, type_aliases_map, &mut seen))
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

fn render_request(
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

fn render_notification(
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
        #![allow(deprecated, clippy::doc_lazy_continuation, unreachable_patterns, clippy::large_enum_variant)]
    };

    let imports = quote! {
        use derive_more::From;
        use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
        use std::{borrow::Cow, collections::HashMap};
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

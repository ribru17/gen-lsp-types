use std::collections::{HashMap, HashSet};

use crate::schema::{
    BaseType, BaseTypes, Enumeration, EnumerationTypeName, ReferenceType, Structure, Type,
    TypeAlias,
};

pub fn get_struct_derives(
    structure: &Structure,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> HashMap<&'static str, Option<&'static str>> {
    // Start with the commonly shared derives.
    let mut derives = HashMap::new();
    derives.insert("Serialize", None);
    derives.insert("Deserialize", None);
    derives.insert("PartialEq", None);
    derives.insert("Debug", None);
    derives.insert("Clone", None);
    derives.insert("New", Some("derive-new"));

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

        if eqable && has_float(prop_type, structs_map, type_aliases_map) {
            eqable = false;
        }
        if eqable && hashable && !is_hashable(prop_type, structs_map, type_aliases_map) {
            hashable = false;
        }
        if defaultable && !optional && !is_defaultable(prop_type, structs_map, type_aliases_map) {
            defaultable = false;
        }
        if copyable && !is_copyable(prop_type, structs_map, enums_map, type_aliases_map) {
            copyable = false;
        }
    }
    if eqable {
        derives.insert("Eq", None);
        if hashable {
            derives.insert("Hash", None);
        }
    }
    if defaultable {
        derives.insert("Default", None);
    }
    if copyable {
        derives.insert("Copy", None);
    }

    // Special derives.
    if matches!(structure.name.as_str(), "Position" | "Range") {
        derives.insert("PartialOrd", None);
        derives.insert("Ord", None);
    }

    derives
}

pub fn get_enum_derives(enumeration: &Enumeration) -> Vec<&'static str> {
    let mut derives = vec![
        "PartialEq",
        "Eq",
        "Hash",
        "Debug",
        "Clone",
        "Serialize",
        "Deserialize",
    ];
    if matches!(
        enumeration.type_.name,
        EnumerationTypeName::Integer | EnumerationTypeName::Uinteger
    ) || enumeration.supports_custom_values != Some(true)
    {
        derives.push("Copy");
    }
    derives
}

pub fn has_float(
    type_: &Type,
    structs_map: &HashMap<String, Structure>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> bool {
    fn has_float_<'a: 'b, 'b>(
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
                        if has_float_(prop, structs_map, type_aliases_map, seen) {
                            return true;
                        }
                    }
                    return false;
                }
                if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                    return has_float_(&type_alias.type_, structs_map, type_aliases_map, seen);
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
                            if has_float_(prop, structs_map, type_aliases_map, seen) {
                                return true;
                            }
                        }
                        return false;
                    }
                    if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                        return has_float_(&type_alias.type_, structs_map, type_aliases_map, seen);
                    }
                }
                false
            }
            _ => false,
        }
    }

    let seen = &mut HashSet::new();
    has_float_(type_, structs_map, type_aliases_map, seen)
}

pub fn is_defaultable(
    type_: &Type,
    structs_map: &HashMap<String, Structure>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> bool {
    fn is_defaultable_<'a: 'b, 'b>(
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
            Type::OrType(or_type) => or_type.items.iter().any(|item| {
                matches!(
                    item,
                    Type::BaseType(BaseType {
                        kind: _,
                        name: BaseTypes::Null
                    })
                )
            }),
            Type::TupleType(tuple_type) => tuple_type
                .items
                .iter()
                .all(|item| is_defaultable_(item, structs_map, type_aliases_map, seen)),
            Type::AndType(and_type) => and_type
                .items
                .iter()
                .all(|item| is_defaultable_(item, structs_map, type_aliases_map, seen)),
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
                        .all(|prop_type| {
                            is_defaultable_(prop_type, structs_map, type_aliases_map, seen)
                        })
                } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                    is_defaultable_(&type_alias.type_, structs_map, type_aliases_map, seen)
                } else {
                    false
                }
            }
        }
    }

    let seen = &mut HashSet::new();
    is_defaultable_(type_, structs_map, type_aliases_map, seen)
}

pub fn is_copyable(
    type_: &Type,
    structs_map: &HashMap<String, Structure>,
    enums_map: &HashMap<String, Enumeration>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> bool {
    fn is_copyable_<'a: 'b, 'b>(
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
                .all(|item| is_copyable_(item, structs_map, enums_map, type_aliases_map, seen)),
            Type::TupleType(tuple_type) => tuple_type
                .items
                .iter()
                .all(|item| is_copyable_(item, structs_map, enums_map, type_aliases_map, seen)),
            Type::ReferenceType(ref_type) => {
                if let Some(structure) = structs_map.get(&ref_type.name) {
                    structure
                        .properties
                        .iter()
                        .map(|prop| &prop.type_)
                        .chain(&structure.mixins)
                        .chain(&structure.extends)
                        .all(|prop_type| {
                            is_copyable_(prop_type, structs_map, enums_map, type_aliases_map, seen)
                        })
                } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                    is_copyable_(
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

    let seen = &mut HashSet::new();
    is_copyable_(type_, structs_map, enums_map, type_aliases_map, seen)
}

pub fn is_hashable(
    type_: &Type,
    structs_map: &HashMap<String, Structure>,
    type_aliases_map: &HashMap<String, TypeAlias>,
) -> bool {
    fn is_hashable_<'a: 'b, 'b>(
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
            Type::StringLiteralType(_)
            | Type::IntegerLiteralType(_)
            | Type::BooleanLiteralType(_) => true,
            Type::BaseType(BaseType { kind: _, name }) => !matches!(name, BaseTypes::Decimal),
            Type::ReferenceType(ref_type) => {
                if let Some(structure) = structs_map.get(&ref_type.name) {
                    structure
                        .properties
                        .iter()
                        .map(|prop| &prop.type_)
                        .chain(&structure.mixins)
                        .chain(&structure.extends)
                        .all(|prop_type| {
                            is_hashable_(prop_type, structs_map, type_aliases_map, seen)
                        })
                } else if let Some(type_alias) = type_aliases_map.get(&ref_type.name) {
                    is_hashable_(&type_alias.type_, structs_map, type_aliases_map, seen)
                } else {
                    true
                }
            }
            Type::ArrayType(array_type) => {
                is_hashable_(&array_type.element, structs_map, type_aliases_map, seen)
            }
            Type::AndType(and_type) => and_type
                .items
                .iter()
                .all(|item| is_hashable_(item, structs_map, type_aliases_map, seen)),
            Type::OrType(or_type) => or_type
                .items
                .iter()
                .all(|item| is_hashable_(item, structs_map, type_aliases_map, seen)),
            Type::TupleType(tuple_type) => tuple_type
                .items
                .iter()
                .all(|item| is_hashable_(item, structs_map, type_aliases_map, seen)),
        }
    }

    let seen = &mut HashSet::new();
    is_hashable_(type_, structs_map, type_aliases_map, seen)
}

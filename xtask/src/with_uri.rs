use std::collections::{BTreeMap, HashMap};

use crate::{
    camel_to_snake,
    schema::{BaseType, BaseTypes, ReferenceType, Structure, Type},
};

const URI_PROPERTY: &str = "uri";
const TEXT_DOCUMENT_PROPERTY: &str = "textDocument";

const fn is_uri_type(type_: &Type) -> bool {
    matches!(
        type_,
        Type::BaseType(BaseType {
            kind: _,
            name: BaseTypes::Uri | BaseTypes::DocumentUri,
        })
    )
}

pub fn get_structs_with_uri(
    structs_map: &HashMap<String, Structure>,
) -> BTreeMap<String, Vec<String>> {
    structs_map
        .iter()
        .filter_map(|(k, v)| get_uri_fields(structs_map, v).map(|x| (k.clone(), x)))
        .collect()
}

fn get_uri_fields(
    structs_map: &HashMap<String, Structure>,
    struct_: &Structure,
) -> Option<Vec<String>> {
    if let Some(prop) = struct_
        .properties
        .iter()
        .find(|p| p.name == URI_PROPERTY && p.optional != Some(true) && is_uri_type(&p.type_))
    {
        return Some(vec![camel_to_snake(&prop.name)]);
    }

    if let Some(prop) = struct_
        .properties
        .iter()
        .find(|p| p.name == TEXT_DOCUMENT_PROPERTY)
        && let Type::ReferenceType(ReferenceType { name, .. }) = &prop.type_
        && let Some(s) = structs_map.get(name)
        && let Some(mut fields) = get_uri_fields(structs_map, s)
    {
        let mut all_fields = Vec::with_capacity(fields.len() + 1);
        all_fields.push(camel_to_snake(&prop.name));
        all_fields.append(&mut fields);
        return Some(all_fields);
    }

    struct_.mixins.iter().chain(&struct_.extends).find_map(|p| {
        let Type::ReferenceType(ReferenceType { name, .. }) = p else {
            unreachable!("mixins are always references")
        };
        let s = structs_map.get(name)?;
        get_uri_fields(structs_map, s).map(|mut fields| {
            let mut all_fields = Vec::with_capacity(fields.len() + 1);
            all_fields.push(camel_to_snake(name));
            all_fields.append(&mut fields);
            all_fields
        })
    })
}

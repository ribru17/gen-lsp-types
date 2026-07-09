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

pub fn get_structs_with_uri(structs_map: &HashMap<String, Structure>) -> BTreeMap<String, String> {
    structs_map
        .iter()
        .filter_map(|(k, v)| get_uri_field(structs_map, v).map(|x| (k.clone(), x)))
        .collect()
}

fn get_uri_field(structs_map: &HashMap<String, Structure>, struct_: &Structure) -> Option<String> {
    if let Some(prop) = struct_
        .properties
        .iter()
        .find(|p| p.name == URI_PROPERTY && p.optional != Some(true) && is_uri_type(&p.type_))
    {
        return Some(camel_to_snake(&prop.name));
    }

    if let Some(prop) = struct_
        .properties
        .iter()
        .find(|p| p.name == TEXT_DOCUMENT_PROPERTY)
        && let Type::ReferenceType(ReferenceType { name, .. }) = &prop.type_
        && let Some(s) = structs_map.get(name)
        && get_uri_field(structs_map, s).is_some()
    {
        return Some(camel_to_snake(&prop.name));
    }

    struct_.mixins.iter().chain(&struct_.extends).find_map(|p| {
        let Type::ReferenceType(ReferenceType { name, .. }) = p else {
            unreachable!("mixins are always references")
        };
        let s = structs_map.get(name)?;
        get_uri_field(structs_map, s).map(|_| camel_to_snake(name))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Property;

    const NO_URI: &str = "NoUri";
    const FLAT_URI: &str = "FlatUri";
    const URI_FIELD: &str = "uri";
    const INNER: &str = "Inner";
    const OUTER: &str = "Outer";
    const MISSING: &str = "Missing";

    fn make_struct(
        name: &str,
        properties: Vec<Property>,
        extends: Vec<Type>,
        mixins: Vec<Type>,
    ) -> Structure {
        Structure {
            deprecated: None,
            documentation: None,
            extends,
            mixins,
            name: name.to_string(),
            properties,
            proposed: None,
            since: None,
            since_tags: Vec::new(),
        }
    }

    fn make_property(name: &str, type_: Type) -> Property {
        Property {
            deprecated: None,
            documentation: None,
            name: name.to_string(),
            optional: None,
            proposed: None,
            since: None,
            since_tags: Vec::new(),
            type_,
        }
    }

    fn uri_type() -> Type {
        Type::BaseType(BaseType {
            kind: "base".to_string(),
            name: BaseTypes::DocumentUri,
        })
    }

    fn string_type() -> Type {
        Type::BaseType(BaseType {
            kind: "base".to_string(),
            name: BaseTypes::String,
        })
    }

    fn make_optional_property(name: &str, type_: Type) -> Property {
        Property {
            optional: Some(true),
            ..make_property(name, type_)
        }
    }

    fn mixin_type(name: &str) -> Type {
        Type::ReferenceType(ReferenceType {
            kind: "reference".to_string(),
            name: name.to_string(),
        })
    }

    fn build_map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn no_uri_anywhere() {
        let structs_map = HashMap::from([(
            NO_URI.to_string(),
            make_struct(
                NO_URI,
                vec![make_property("label", string_type())],
                Vec::new(),
                Vec::new(),
            ),
        )]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(result, BTreeMap::new());
    }

    #[test]
    fn direct_uri_field() {
        let structs_map = HashMap::from([(
            FLAT_URI.to_string(),
            make_struct(
                FLAT_URI,
                vec![make_property(URI_FIELD, uri_type())],
                Vec::new(),
                Vec::new(),
            ),
        )]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(result, build_map(&[(FLAT_URI, URI_FIELD)]));
    }

    #[test]
    fn non_qualifying_uri_fields_are_ignored() {
        const NON_CANONICAL_URI_FIELD: &str = "oldUri";

        let structs_map = HashMap::from([(
            FLAT_URI.to_string(),
            make_struct(
                FLAT_URI,
                vec![
                    make_property(NON_CANONICAL_URI_FIELD, uri_type()),
                    make_optional_property(URI_FIELD, uri_type()),
                ],
                Vec::new(),
                Vec::new(),
            ),
        )]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(result, BTreeMap::new());
    }

    #[test]
    fn uri_via_mixin() {
        let outer_field = camel_to_snake(INNER);

        let structs_map = HashMap::from([
            (
                INNER.to_string(),
                make_struct(
                    INNER,
                    vec![make_property(URI_FIELD, uri_type())],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                OUTER.to_string(),
                make_struct(OUTER, Vec::new(), Vec::new(), vec![mixin_type(INNER)]),
            ),
        ]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(
            result,
            build_map(&[(INNER, URI_FIELD), (OUTER, &outer_field)])
        );
    }

    #[test]
    fn uri_via_extends() {
        let outer_field = camel_to_snake(INNER);

        let structs_map = HashMap::from([
            (
                INNER.to_string(),
                make_struct(
                    INNER,
                    vec![make_property(URI_FIELD, uri_type())],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                OUTER.to_string(),
                make_struct(OUTER, Vec::new(), vec![mixin_type(INNER)], Vec::new()),
            ),
        ]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(
            result,
            build_map(&[(INNER, URI_FIELD), (OUTER, &outer_field)])
        );
    }

    #[test]
    fn direct_uri_field_wins_over_mixin() {
        let structs_map = HashMap::from([
            (
                INNER.to_string(),
                make_struct(
                    INNER,
                    vec![make_property(URI_FIELD, uri_type())],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                OUTER.to_string(),
                make_struct(
                    OUTER,
                    vec![make_property(URI_FIELD, uri_type())],
                    Vec::new(),
                    vec![mixin_type(INNER)],
                ),
            ),
        ]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(result, build_map(&[(INNER, URI_FIELD), (OUTER, URI_FIELD)]));
    }

    #[test]
    fn uri_via_two_level_mixin_chain() {
        const MIDDLE: &str = "Middle";

        let outer_field = camel_to_snake(MIDDLE);

        let structs_map = HashMap::from([
            (
                INNER.to_string(),
                make_struct(
                    INNER,
                    vec![make_property(URI_FIELD, uri_type())],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                MIDDLE.to_string(),
                make_struct(MIDDLE, Vec::new(), Vec::new(), vec![mixin_type(INNER)]),
            ),
            (
                OUTER.to_string(),
                make_struct(OUTER, Vec::new(), Vec::new(), vec![mixin_type(MIDDLE)]),
            ),
        ]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(
            result,
            build_map(&[
                (INNER, URI_FIELD),
                (MIDDLE, &camel_to_snake(INNER)),
                (OUTER, &outer_field),
            ])
        );
    }

    #[test]
    fn dangling_mixin_reference_does_not_panic() {
        let structs_map = HashMap::from([(
            OUTER.to_string(),
            make_struct(OUTER, Vec::new(), Vec::new(), vec![mixin_type(MISSING)]),
        )]);

        let result = get_structs_with_uri(&structs_map);

        assert_eq!(result, BTreeMap::new());
    }
}

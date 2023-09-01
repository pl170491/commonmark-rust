use lazy_static::lazy_static;
use serde_json;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref HASHMAP: HashMap<String, String> = {
        let json = fs::read_to_string("./assets/entities.json").expect("no json file");
        serde_json::from_str(json.as_str()).expect("invalid json")
    };
}

pub fn html_entity(name: &str) -> Option<&str> {
    HASHMAP.get(name).map(|x| &**x)
}

pub fn is_html_entity(name: &str) -> bool {
    HASHMAP.contains_key(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_capital_aelig() {
        assert_eq!(html_entity("AElig").unwrap(), "\u{00c6}");
    }

    #[test]
    fn entity_capital_afr() {
        assert_eq!(html_entity("Afr").unwrap(), "\u{1d504}");
    }

    #[test]
    fn entity_amp() {
        assert_eq!(html_entity("amp").unwrap(), "&");
    }

    #[test]
    fn is_entity_amp() {
        assert!(is_html_entity("amp"));
    }

    #[test]
    fn non_entity() {
        assert!(!is_html_entity("madeUpEntity"));
    }
}

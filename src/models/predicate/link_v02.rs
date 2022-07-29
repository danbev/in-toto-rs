use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

use super::{PredicateLayout, PredicateVersion, PredicateWrapper};
use crate::interchange::{DataInterchange, Json};
use crate::models::byproducts::ByProducts;
use crate::models::step::Command;
use crate::models::{LinkMetadata, TargetDescription, VirtualTargetPath};
use crate::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
/// Predicate `LinkV02` means the predicate of original compatible format.
///
/// [LinkV02](https://in-toto.io/Link/v0.2)
/// can be used together with most states.
pub struct LinkV02 {
    name: String,
    materials: BTreeMap<VirtualTargetPath, TargetDescription>,
    env: Option<BTreeMap<String, String>>,
    command: Command,
    byproducts: ByProducts,
}

impl From<LinkMetadata> for LinkV02 {
    fn from(meta: LinkMetadata) -> LinkV02 {
        LinkV02 {
            name: meta.name().to_string(),
            materials: meta.materials().clone(),
            env: meta.env().clone(),
            command: meta.command().clone(),
            byproducts: meta.byproducts().clone(),
        }
    }
}

impl PredicateLayout for LinkV02 {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Json::canonicalize(&Json::serialize(self)?)
    }

    fn into_enum(self: Box<Self>) -> PredicateWrapper {
        PredicateWrapper::LinkV0_2(*self)
    }

    fn version(&self) -> PredicateVersion {
        PredicateVersion::LinkV0_2
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::BTreeMap;
    use std::str;

    use once_cell::sync::Lazy;
    use serde_json::json;

    use super::LinkV02;
    use crate::{
        interchange::{DataInterchange, Json},
        models::{
            byproducts::ByProducts, test::BLANK_META, PredicateLayout, PredicateVersion,
            PredicateWrapper,
        },
    };

    pub static STR_PREDICATE_LINK_V02: Lazy<String> = Lazy::new(|| {
        let raw_data = json!(
        {
            "byproducts": {
                "return-value": 0,
                "stderr": "",
                "stdout": ""
            },
            "command": "",
            "env": null,
            "materials": {},
            "name": ""
        });
        let value = serde_json::value::to_value(raw_data).unwrap();
        let bytes = Json::canonicalize(&value).unwrap();
        let data = str::from_utf8(&bytes).unwrap();
        data.to_string()
    });

    pub static PREDICATE_LINK_V02: Lazy<LinkV02> = Lazy::new(|| LinkV02 {
        name: "".to_string(),
        materials: BTreeMap::new(),
        env: None,
        command: "".into(),
        byproducts: ByProducts::new(),
    });

    #[test]
    fn into_trait_equal() {
        let predicate = PredicateWrapper::LinkV0_2(PREDICATE_LINK_V02.clone());
        let real = Box::new(PREDICATE_LINK_V02.clone()).into_enum();

        assert_eq!(predicate, real);
    }

    #[test]
    fn create_predicate_from_meta() {
        let predicate = PredicateWrapper::from_meta(BLANK_META.clone(), PredicateVersion::LinkV0_2);
        let real = Box::new(PREDICATE_LINK_V02.clone()).into_enum();

        assert_eq!(predicate, real);
    }

    #[test]
    fn serialize_predicate() {
        let predicate = Box::new(PREDICATE_LINK_V02.clone()).into_enum();
        let buf = predicate.into_trait().to_bytes().unwrap();
        let predicate_serialized = str::from_utf8(&buf).unwrap();

        assert_eq!(predicate_serialized, *STR_PREDICATE_LINK_V02);
    }

    #[test]
    fn deserialize_predicate() {
        let predicate = PredicateWrapper::from_bytes(
            STR_PREDICATE_LINK_V02.as_bytes(),
            PredicateVersion::LinkV0_2,
        )
        .unwrap();
        let real = Box::new(PREDICATE_LINK_V02.clone()).into_enum();

        assert_eq!(predicate, real);
    }

    #[test]
    fn deserialize_auto() {
        let predicate =
            PredicateWrapper::try_from_bytes(STR_PREDICATE_LINK_V02.as_bytes()).unwrap();
        let real = Box::new(PREDICATE_LINK_V02.clone()).into_enum();

        assert_eq!(predicate, real);
    }

    #[test]
    fn deserialize_wrong_patterns() {
        let wrong_patterns = vec!["{", "{}"];
        let wrong_inputs: Vec<Vec<u8>> = wrong_patterns
            .iter()
            .map(|e| e.as_bytes().to_vec())
            .collect();
        for inp in wrong_inputs {
            let predicate = PredicateWrapper::from_bytes(&inp, PredicateVersion::LinkV0_2);

            assert!(predicate.is_err());
        }
    }
}

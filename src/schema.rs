use crate::{
    ipi::{IpiBaseNum, IpiNameNum},
    iswc::Iswc,
};

impl schemars::JsonSchema for Iswc {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Iswc".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "pattern": "^T[0-9]{10}$"
        })
    }
}

impl schemars::JsonSchema for IpiNameNum {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        "IpiNameNum".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "integer",
            "format": "uint64",
            "minimum": 10,
            "maximum": 99999999999u64
        })
    }
}

impl schemars::JsonSchema for IpiBaseNum {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        "IpiBaseNum".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "pattern": "^I-[0-9]-{9}$"
        })
    }
}

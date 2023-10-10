// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;
use std::str::FromStr;

use chrono::{DateTime, Local};
use serde::{
    de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct CliInfo {
    #[serde(deserialize_with = "version_de", default = "default_version")]
    pub(crate) version: u32,
    #[serde(default)]
    pub(crate) hostname: String,
    #[serde(deserialize_with = "time_de", serialize_with = "time_se")]
    pub(crate) time: DateTime<Local>,
}

// This function is inspired by https://serde.rs/string-or-struct.html
fn version_de<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    struct IntegerOrString(PhantomData<fn() -> Option<u64>>);

    impl<'de> Visitor<'de> for IntegerOrString {
        type Value = u32;

        fn expecting(
            &self,
            formatter: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            formatter.write_str("unsigned integer or string")
        }

        fn visit_str<E>(self, value: &str) -> Result<u32, E>
        where
            E: de::Error,
        {
            if let Some(prefix_len) = value.strip_prefix("0x") {
                u32::from_str_radix(prefix_len, 16).map_err(de::Error::custom)
            } else {
                FromStr::from_str(value).map_err(de::Error::custom)
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<u32, E>
        where
            E: de::Error,
        {
            u32::try_from(value).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(IntegerOrString(PhantomData))
}

fn default_version() -> u32 {
    1u32
}

fn time_de<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateTimeVisitor(PhantomData<fn() -> DateTime<Local>>);

    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = DateTime<Local>;

        fn expecting(
            &self,
            formatter: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            formatter.write_str(
                "String of date time in RFC 3339 or \
                RFC 2822 format",
            )
        }

        fn visit_str<E>(self, value: &str) -> Result<DateTime<Local>, E>
        where
            E: de::Error,
        {
            DateTime::parse_from_rfc2822(value)
                .or_else(|_| DateTime::parse_from_rfc3339(value))
                .map_err(de::Error::custom)
                .map(|t| t.with_timezone(&Local))
        }
    }

    deserializer.deserialize_any(DateTimeVisitor(PhantomData))
}

fn time_se<S>(v: &DateTime<Local>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(v.to_rfc3339().as_str())
}

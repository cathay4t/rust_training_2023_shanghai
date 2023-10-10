// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;
use std::str::FromStr;

use chrono::{DateTime, Local};
use serde::{
    de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
struct Bar {
    data: String,
}

#[derive(Debug, Clone, Default)]
struct AbcError {
    msg: String,
}

impl std::fmt::Display for AbcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for AbcError {}

impl TryFrom<String> for Bar {
    type Error = AbcError;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        Ok(Self { data: v })
    }
}

impl From<Bar> for String {
    fn from(v: Bar) -> Self {
        v.data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Foo {
    #[serde(deserialize_with = "version_de", default = "default_version")]
    version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt1: Option<u32>,
    #[serde(deserialize_with = "time_de", serialize_with = "time_se")]
    time: DateTime<Local>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Foos {
    foos: Vec<Foo>,
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

#[derive(Debug, Clone, Default, Serialize)]
struct FooBar {
    data: String,
}

impl<'de> Deserialize<'de> for FooBar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;
        match v.as_str() {
            Some(s) => Ok(Self {
                data: s.to_string(),
            }),
            None => {
                Err(de::Error::custom("Invalid data type, expecting string"))
            }
        }
    }
}

fn main() {
    let time = chrono::Local::now();
    let foos = Foos {
        foos: vec![
            Foo {
                version: 1u32,
                time,
                ..Default::default()
            },
            Foo {
                version: 2u32,
                time,
                ..Default::default()
            },
        ],
    };
    println!("Serialize\n{}", serde_yaml::to_string(&foos).unwrap());

    let foos: Foos = serde_yaml::from_str(
        r#"---
        foos:
        - version: 1
          opt1: 99
          time: 2023-10-10T16:00:30.080553628+08:00
        - version: 2
          time: 2023-10-10T16:00:30.080553628+08:00"#,
    )
    .unwrap();
    println!("Deserialize\n{:?}", foos);

    let bar = Bar { data: "ABC".into() };
    println!("Serialize Bar\n{}", serde_yaml::to_string(&bar).unwrap());

    let foobar: FooBar = serde_yaml::from_str(
        r#"---
        AbcD"#,
    )
    .unwrap();
    println!("Deserialize FooBar\n{:?}", foobar);
}

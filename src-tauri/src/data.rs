use blake3::Hash;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Email(String),
    PhoneNumber(String),
    JsonDict(serde_json::Value),
    Code { data: String, lang: String },
}
impl Data {
    pub fn hash(&self) -> String {
        match self {
            Data::Email(email) => blake3::hash(email.as_bytes()),
            Data::PhoneNumber(phone) => blake3::hash(phone.as_bytes()),
            Data::JsonDict(json) => blake3::hash(json.to_string().as_bytes()),
            Data::Code { data, lang: _ } => blake3::hash(data.as_bytes()),
        }
        .to_hex()
        .to_string()
    }
    pub fn val(&self) -> String {
        match self {
            Data::Email(str) => str.clone(),
            Data::PhoneNumber(str) => str.clone(),
            Data::JsonDict(str) => serde_json::to_string(&str).unwrap_or("err str".into()),
            Data::Code { data, lang: _ } => data.clone(),
        }
    }
    pub fn partial(&self) -> String {
        match self {
            Data::Email(e) => e.clone(),
            Data::PhoneNumber(ph) => ph.clone(),
            Data::JsonDict(value) => todo!(),
            Data::Code { data, lang } => todo!(),
        }
    }
}
fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();

    email_regex.is_match(email)
}
fn is_valid_phone_number(phone: &str) -> bool {
    let phone_regex =
        Regex::new(r"^\+?[0-9]{1,3}?[-. ]?\(?[0-9]{2,4}\)?[-. ]?[0-9]{3,4}[-. ]?[0-9]{4}$")
            .unwrap();
    phone_regex.is_match(phone)
}

impl From<String> for Data {
    fn from(value: String) -> Self {
        let str_len = value.len();
        if str_len <= 15 && is_valid_phone_number(&value) {
            Data::PhoneNumber(value)
        } else if str_len < 256 && is_valid_email(&value) {
            Data::Email(value)
        } else {
            match serde_json::from_str::<serde_json::Value>(&value) {
                Ok(serde_json::Value::Object(d)) => Self::JsonDict(serde_json::Value::Object(d)),
                _ => Data::Code {
                    data: value,
                    lang: "string".into(),
                },
            }
        }
    }
}

//! This module handles all JWT manipulation and encryption.
extern crate serde;

use serde::{Deserialize, Deserializer};
use serde::de::{MapVisitor, Visitor};

#[derive(RustcEncodable, Debug, Clone)]
pub struct Jwt {
    pub secret_key: String,
    pub token: String,
}

enum JwtField {
    SecretKey,
    Token,
}

struct JwtVisitor;

impl Deserialize for Jwt {
    fn deserialize<D>(deserializer: &mut D) -> Result<Jwt, D::Error>
        where D: Deserializer
    {
        static FIELDS: &'static [&'static str] = &["secret_key, token"];

        deserializer.deserialize_struct("Jwt", FIELDS, JwtVisitor)
    }
}


impl Visitor for JwtVisitor {
    type Value = Jwt;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Jwt, V::Error>
        where V: MapVisitor
    {
        let mut secret_key = None;
        let mut token = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(JwtField::SecretKey) => {
                    secret_key = Some(try!(visitor.visit_value()));
                }
                Some(JwtField::Token) => {
                    token = Some(try!(visitor.visit_value()));
                }
                None => {
                    break;
                }
            }
        }

        let secret_key = match secret_key {
            Some(secret_key) => secret_key,
            None => try!(visitor.missing_field("secret_key")),
        };

        let token = match token {
            Some(token) => token,
            None => "".to_string(),
        };

        try!(visitor.end());

        Ok(Jwt {
            secret_key: secret_key,
            token: token,
        })
    }
}

impl Deserialize for JwtField {
    fn deserialize<D>(deserializer: &mut D) -> Result<JwtField, D::Error>
        where D: Deserializer
    {

        struct JwtFieldVisitor;

        impl Visitor for JwtFieldVisitor {
            type Value = JwtField;

            fn visit_str<E>(&mut self, value: &str) -> Result<JwtField, E>
                where E: serde::de::Error
            {
                match value {
                    "secret_key" => Ok(JwtField::SecretKey),
                    "token" => Ok(JwtField::Token),
                    _ => Err(serde::de::Error::custom("expected secret_key or token")),
                }
            }
        }

        deserializer.deserialize(JwtFieldVisitor)
    }
}

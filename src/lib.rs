pub mod errors;
use errors::*;

// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(untagged)]
// pub enum Value {
//     String(String),
//     Integer(i64),
//     Float(f64),
//     Boolean(bool),
//     Array(Vec<Value>),
//     Map(IndexMap<String, Value>),
//     Object(Box<Value>),
// }
use serde_json::Value;

pub fn parse_toml(input: &str) -> Result<Value> {
    let value: Value = toml::from_str(input).change_context(Error)?;
    Ok(value)
}

pub fn parse_yaml(input: &str) -> Result<Value> {
    let value: Value = serde_yml::from_str(input).change_context(Error)?;
    Ok(value)
}

pub fn parse_json(input: &str) -> Result<Value> {
    let value: Value = serde_json::from_str(input).change_context(Error)?;
    Ok(value)
}

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    Toml,
    Yaml,
    Json,
}

// impl Value {
pub fn parse(input: &str, format: Format) -> Result<Value> {
    match format {
        Format::Toml => parse_toml(input),
        Format::Yaml => parse_yaml(input),
        Format::Json => parse_json(input),
    }
}
pub fn try_parse_all(input: &str) -> Result<Value> {
    parse_json(input)
        .or_else(|_| parse_toml(input))
        .or_else(|_| parse_yaml(input))
        .change_context(Error)
}
// }

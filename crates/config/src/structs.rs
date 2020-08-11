use serde_yaml::Value;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "use")]
    pub module: String,
    pub response: Value,
    pub request: Value
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Handler {
    pub response: HandlerModule,
    pub request: HandlerModule
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HandlerModule {
    #[serde(rename = "use")]
    pub module: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "use")]
    pub(crate) module: String,
    pub params: Value,
    pub map: Map,
    pub output: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
    #[serde(rename = "use")]
    pub module: String,
    pub params: Value,
    pub map: Map,
    pub name: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub git: String,
    pub endpoint: String,
    pub port: i32,
    #[serde(rename = "use")]
    pub module: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Modules {
    pub modules: Vec<Modules>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub input: Input,
    pub output: Output,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

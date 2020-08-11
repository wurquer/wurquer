#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;


use std::collections::BTreeMap;

mod structs;

use structs::Config;

pub fn handler_configure(map: String) -> Config {
    serde_yaml::from_str(&map).unwrap()
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use crate::structs::{Config, Input, Output, Map};
    use crate::serde_yaml::Value;
    extern crate assert_type_eq;

    fn fake_configure() -> String {
        let config = Config {
            input: Input {
                module: "example_module".to_string(),
                params: Value::Null,
                map: Map {
                    module: "example_module".to_string(),
                    response: Value::Null,
                    request: Value::Null
                },
                output: "output_module".to_string(),
            },
            output: Output {
                module: "example_module".to_string(),
                params: Value::Null,
                map: Map {
                    module: "example_module".to_string(),
                    response: Value::Null,
                    request: Value::Null
                },
                name: "".to_string()
            },
            inputs: vec![],
            outputs: vec![]
        };

        serde_yaml::to_string(&config).unwrap()
    }

    #[test]
    fn handler_configure() {
        let yml = fake_configure();
        let config = crate::handler_configure(yml);
        assert_eq!(config.input.module, "example_module")
    }
}

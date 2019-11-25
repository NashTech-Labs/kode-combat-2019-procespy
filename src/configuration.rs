use std::fs::read_to_string;
use json;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use json::{JsonValue, Error};

pub fn fetch_configuration(config_file_path: &str ) -> Result<HashMap< String, String>, String> {
    let config_path = read_to_string(config_file_path);
    let mut configurations: HashMap<String, String, RandomState> = HashMap::new();
    match config_path{
        Ok(path) => {
            let data: Result<JsonValue, Error> = json::parse(path.as_ref());
            match data {
                Ok(values) => {
                    for (key, value) in values.entries(){
                        configurations.insert(key.to_string(),value.as_str().unwrap().to_string());
                    }
                    Ok(configurations)
                }
                Err(error) => Err(error.to_string())
            }
        }
        Err(error) => Err(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::fetch_configuration;

    static CONFIG_FILE: &str = "config.txt";
    static INVALID_CONFIG_FILE_PATH: &str ="configg.txt";

    #[test]
    fn test_variable_value_success() {
        assert!(fetch_configuration(CONFIG_FILE).is_ok());

    }

    #[test]
    fn test_variable_value_failure() {
        if let Err(error) = fetch_configuration(INVALID_CONFIG_FILE_PATH ){
            assert_eq!(error, "No such file or directory (os error 2)");
        }
    }
}
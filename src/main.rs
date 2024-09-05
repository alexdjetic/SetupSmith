mod yaml;
mod json;

use yaml::Yaml;
use json::Json;

fn open_yaml() -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to create a Yaml instance and load data
    let yaml = match Yaml::new("test_file/test.yaml") {
        Ok(yaml) => yaml,
        Err(e) => {
            return Err(e);
        }
    };

    // Access a specific key from the YAML data
    match yaml.get("name") {
        Some(value) => println!("Value for 'name': {:?}", value),
        None => println!("Key 'name' not found."),
    }

    // Print the entire data for verification
    println!("{:#?}", yaml.data);

    Ok(())
}

fn open_json() -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to create a Json instance and load data
    let json = match Json::new("test_file/test.json") {
        Ok(json) => json,
        Err(e) => {
            return Err(e);
        }
    };


    // Access a specific key from the JSON data
    match json.get("name") {
        Some(value) => println!("Value for 'name': {:?}", value),
        None => println!("Key 'name' not found."),
    }

    // Print the entire data for verification
    println!("{:#?}", json.data);

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    open_json()
}

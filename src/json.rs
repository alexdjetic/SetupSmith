use serde_json;
use std::collections::HashMap;
use std::fs::{File, metadata};
use std::io::Read;
use std::path::Path;

pub struct Json {
    pub file: String,
    pub data: HashMap<String, serde_json::Value>, // Add a field to store the JSON data
    _exists: bool,
    _readable: bool,
}

impl Json {
    // Constructor to initialize Json with file path and parsed data
    pub fn new(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Check if the file exists
        let exists = Self::exists(file_path)?;
        // Check if the file is readable
        let readable = if exists {
            Self::is_readable(file_path)?
        } else {
            false
        };

        if !exists {
            return Err(Box::from(format!("File {} does not exist", file_path)));
        }

        if !readable {
            return Err(Box::from(format!("File {} is not readable", file_path)));
        }

        // Extract the file data
        let (data, _) = Self::_extract(file_path)?;

        Ok(Json {
            file: file_path.to_string(),
            data,
            _exists: exists,
            _readable: readable,
        })
    }

    // Method to get data by key
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    // Method to check if the file exists
    fn exists(file_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        match metadata(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    // Method to check if the file is readable
    fn is_readable(file_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        // Attempt to open the file to check if it's readable
        match File::open(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    // Private method to handle file opening, reading, and parsing
    fn _extract(file_path: &str) -> Result<(HashMap<String, serde_json::Value>, String), Box<dyn std::error::Error>> {
        // Open the JSON file
        let mut file = File::open(file_path)?;

        // Read the contents of the file into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the JSON string into a HashMap
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&contents)?;

        Ok((data, contents))
    }
}

pub fn open_json(file_path: &str) -> Result<Json, Box<dyn std::error::Error>> {
    // Attempt to create a Json instance and load data
    let json = match Json::new(file_path) {
        Ok(json) => json,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(json)
}
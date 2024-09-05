// src/yaml.rs
use serde_yaml;
use std::collections::HashMap;
use std::fs::{File, metadata};
use std::io::Read;
use std::path::Path;

pub struct Yaml {
    pub file: String,
    pub data: HashMap<String, serde_yaml::Value>,
    _exists: bool,
    _readable: bool,
}

impl Yaml {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let exists = Self::exists(file_path)?;
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

        let (data, _) = Self::_extract(file_path)?;

        Ok(Yaml {
            file: file_path.to_string(),
            data,
            _exists: exists,
            _readable: readable,
        })
    }

    // Method to get data by key
    pub fn get(&self, key: &str) -> Option<&serde_yaml::Value> {
        self.data.get(key)
    }

    fn exists(file_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        match metadata(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn is_readable(file_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        match File::open(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn _extract(file_path: &str) -> Result<(HashMap<String, serde_yaml::Value>, String), Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&contents)?;
        Ok((data, contents))
    }
}

pub fn open_yaml(file_path: &str) -> Result<Yaml, Box<dyn std::error::Error>> {
    let yaml = Yaml::new(file_path)?;
    Ok(yaml)
}

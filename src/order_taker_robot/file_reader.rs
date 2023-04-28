use std::{fs::{self, File}, ffi::OsStr, io::{BufReader, Read}};
use serde_json::{Deserializer, Value};

pub struct FileReader {
    file_name: String
}

impl FileReader {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name
        }
    }
    pub fn read(&self) -> Result<String, String> {
        match File::open(&self.file_name) {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                let result = buf_reader.read_to_string(&mut contents);
                match result {
                    Ok(_) => {
                        Ok(contents)},

                    Err(msg) => {
                        println!("error with file");
                        Err(msg.to_string())
                    }
                }
            },
            Err(error) => {
                println!("error with file!!");
                Err(error.to_string())
            }
        }
/* 
        let stream = Deserializer::from_str(data).into_iter::<Value>();

        for value in stream {
            let v = value.unwrap();
            println!("{}", v);
        }
*/
    }
}
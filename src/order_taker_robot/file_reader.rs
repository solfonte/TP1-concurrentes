use std::{fs::File, io::{BufReader, Read}};

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
                        Ok(contents)
                    },

                    Err(msg) => {
                        Err(msg.to_string())
                    }
                }
            },
            Err(error) => {
                Err(error.to_string())
            }
        }
    }
}


#[cfg(test)]
mod file_reader_test {
    use super::FileReader;

    #[test]
    fn test01_when_opening_a_file_with_one_order_should_return_ok() {
        let file_reader = FileReader::new(String::from("src/test_order_files/one_order.json"));
        let result = file_reader.read();

        assert!(result.is_ok());
    }

    #[test]
    fn test02_when_opening_a_non_existing_file_should_return_only_error() {
        let file_reader = FileReader::new(String::from("src/test_order_files/non_existing_file.json"));
        let result = file_reader.read();

        assert!(result.is_err());
    }
}


pub struct FileReader {
    file_name: String,
}

impl FileReader {
    fn read_file(&self) -> String {
        let file_contents = fs::read_to_string()
        .expect("LogRocket: Should have been able to read the file");
    println!("info.txt context =\n{file_contents}");
    } 
}
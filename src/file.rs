pub struct File{
    file_path: String
}

impl File {
    pub fn new(file_path :&str) -> File{
        File{
            file_path: file_path.to_string()
        }
    }
    pub fn get_content(&self) -> Result<String, String> {
        let content = std::fs::read_to_string(&self.file_path)
            .map_err(|err| err.to_string())?;
        Ok(content)
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        File::new(&self.file_path)
    }
}
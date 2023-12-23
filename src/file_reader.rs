use std::io::BufReader;
use std::io::Read;
use std::fs::File;

pub struct FileReader{
    buffer_reader: BufReader<File>,
    front_byte: Option<u8>
}

impl FileReader {
    pub fn new(file_path :&str) -> Result<FileReader, std::io::Error>{
        let file = File::open(file_path)?;
        let buffer_reader = BufReader::new(file);
        Ok(FileReader{buffer_reader, front_byte: None})
    }
    pub fn pick(&mut self) -> Option<u8> {
        let mut byte = [0; 1]; // Buffer to store the read byte
        match self.buffer_reader.read_exact(&mut byte) {
            Ok(()) => {
                let old_val = self.front_byte;
                self.front_byte = Some(byte[0]);
                if old_val == None{
                    self.front_byte
                }
                else{
                    old_val
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    self.front_byte = None;
                    None
                } else {
                    panic!("Error reading byte: {:?}", err);
                }
            }
        }
    }
    pub fn front(&mut self) -> Option<u8>{
        match self.front_byte{
            Some(byte) => Some(byte),
            None => {
                self.pick()
            },
        }
    }
}
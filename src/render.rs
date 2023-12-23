use crate::{command_option::CommandOption, file_reader::FileReader};

trait Handler{
    fn print(&mut self, reader :&mut FileReader) -> Option<()>;
}

struct NumberHandler{
    pre_byte_is_enter: bool,
    line: u32
}

impl NumberHandler {
    fn new() -> NumberHandler{
        NumberHandler{
            pre_byte_is_enter: true,
            line: 1,
        }
    }
}

impl Handler for NumberHandler{
    fn print(&mut self, reader : &mut FileReader) -> Option<()>{
        match reader.front(){
            None => {return None;},
            Some(byte) =>{
                if self.pre_byte_is_enter == true{
                    print!("{}  ", self.line);
                    self.pre_byte_is_enter = false;
                    self.line += 1;
                }
                if byte == b'\n'{
                    self.pre_byte_is_enter = true;
                }
                Some(())
            }
        }
    }
}

struct ShowEndsHandler{
}

impl ShowEndsHandler {
    fn new() -> ShowEndsHandler{
        ShowEndsHandler{}
    }
}

impl Handler for ShowEndsHandler {
    fn print(&mut self, reader :&mut FileReader) -> Option<()> {
        match reader.front(){
            None => {return None;},
            Some(byte) =>{
                if byte == b'\n'{
                    print!("$");
                }
                Some(())
            }
        }
    }
}

struct SqueezeBlankHandler{
    pre_byte_is_enter: bool,
}

impl SqueezeBlankHandler {
    fn new() -> SqueezeBlankHandler{
        SqueezeBlankHandler{pre_byte_is_enter: false}
    }
}

impl Handler for SqueezeBlankHandler {
    fn print(&mut self, reader :&mut FileReader) -> Option<()> {
        match reader.front(){
            None => {return None;},
            Some(byte) =>{
                if byte != b'\n'{
                    self.pre_byte_is_enter = false;
                    return Some(());
                }
                if self.pre_byte_is_enter == true && byte == b'\n'{
                    let _ = reader.pick();
                }
                Some(())
            }
        }
    }
}

struct NormalHandler{
}

impl NormalHandler {
    fn new() -> NormalHandler{
        NormalHandler{}
    }
}

impl Handler for NormalHandler {
    fn print(&mut self, reader :&mut FileReader) -> Option<()> {
        match reader.front(){
            None => {return None;},
            Some(byte) =>{
                print!("{}", byte as char);
                let _ = reader.pick();
                Some(())
            }
        }
    }
}


pub struct Render{
    option: CommandOption,
    file_readers: Vec<FileReader>,
}

impl Render {
    pub fn new(option :&CommandOption) -> Render{
        let mut render = Render{
            option: option.clone(),
            file_readers: vec![],
        };
        for file in option.files.iter(){
            let file = FileReader::new(file).unwrap();
            render.file_readers.push(file);
        }
        render
    }
    pub fn render(&mut self){
        let mut handlers: Vec<Box::<dyn Handler>> = Vec::new();
        if self.option.squeeze_blank == true{
            handlers.push(Box::new(SqueezeBlankHandler::new()));
        }
        if self.option.number == true{
            handlers.push(Box::new(NumberHandler::new()));
        }
        if self.option.show_ends == true{
            handlers.push(Box::new(ShowEndsHandler::new()));
        }
        handlers.push(Box::new(NormalHandler::new()));
        for file_reader in self.file_readers.iter_mut(){
            loop{
                let mut is_eof = Some(());
                for handler in handlers.iter_mut(){
                    is_eof = handler.print(file_reader);
                }
                if is_eof == None{
                    break;
                }
            }
        }
    }
}
use crate::{command_option::CommandOption, file::File};

pub struct Render{
    option: CommandOption,
    files: Vec<File>,
}

impl Render {
    pub fn new(option :&CommandOption) -> Render{
        let mut render = Render{
            option: option.clone(),
            files: vec![],
        };
        for file in option.files.iter(){
            render.files.push(File::new(file));
        }
        render
    }
    pub fn paint(&self){
        let mut contents: Vec<Result<String, String>> = Vec::new();
        for file in self.files.iter(){
            contents.push(file.get_content());
        }
        if self.option.number == true{
            for content in contents.iter_mut(){
                match content {
                    Err(_) => {},
                    Ok(content) => Render::convert_number(content),
                }
            }
        }
        if self.option.show_ends == true{
            for content in contents.iter_mut(){
                match content {
                    Err(_) => {},
                    Ok(content) => Render::convert_show_ends(content),
                }
            }
        }
        if self.option.squeeze_blank == true{
            for content in contents.iter_mut(){
                match content {
                    Err(_) => {},
                    Ok(content) => Render::convert_squeeze_blank(content),
                }
            }
        }
        for content in contents.iter_mut(){
            match content{
                Err(err) => {println!("{}", err)},
                Ok(content) => {println!("{}", content)},
            }
        }
    }
    fn convert_number(text: &mut String){
        text.clone_from(&text.split('\n').enumerate().map(|(i, val)| format!("   {}   {}\n", i, val)).collect::<String>());
    }
    fn convert_squeeze_blank(text: &mut String){
        text.clone_from(&text.split('\n').filter(|line| line.trim().is_empty() == false).map(|line| format!("{}\n", line)).collect::<String>());
    }
    fn convert_show_ends(text: &mut String){
        text.clone_from(&text.split('\n').map(|line| format!("{}$\n", line)).collect::<String>());
    }
}
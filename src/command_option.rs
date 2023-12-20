use clap::Parser;
#[derive(Parser)]
#[command(name = "rcat")]
#[command(author = "LanQiDongLai")]
#[command(version = "0.1")]
#[command(about = "An implementation of the simple Linux command 'cat' using Rust.", long_about = None)]
pub struct CommandOption {
    /// 不重复输出空行
    #[arg(long, short)]
    pub squeeze_blank: bool,
    /// 对输出所有行进行编号
    #[arg(long, short)]
    pub number: bool,
    /// 在每行行末显示 "$"
    #[arg(long)]
    pub show_ends: bool,

    pub files: Vec<String>,
}

impl Clone for CommandOption{
    fn clone(&self) -> Self {
        CommandOption{
            squeeze_blank: self.squeeze_blank,
            number: self.number,
            show_ends: self.show_ends,
            files: self.files.clone()
        }
    }
}
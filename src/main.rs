
use rcat::render::Render;
use rcat::command_option::CommandOption;

use clap::Parser;

fn main() {
    let render = Render::new(&CommandOption::parse());
    render.paint();
}

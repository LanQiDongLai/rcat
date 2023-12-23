
use rcat::render::Render;
use rcat::command_option::CommandOption;

use clap::Parser;

fn main() {
    let mut render = Render::new(&CommandOption::parse());
    render.render();
}

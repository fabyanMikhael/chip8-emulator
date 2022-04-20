#![allow(non_snake_case)]

use display::Chip8Display;

pub mod vm;
pub mod display;

fn main() {
    let mut chip8 = Chip8Display::new(1920.0, 872.0);
    chip8.vm.load(512, include_bytes!("IBMLogo.ch8"));
    chip8.run();
}

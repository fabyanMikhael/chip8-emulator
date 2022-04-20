use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

use crate::vm::Chip8VM;

pub struct Chip8Display{
    pub vm: Chip8VM,
    width_ratio: f32,
    height_ratio: f32,
    width: f32,
    height: f32,
}

impl Chip8Display{
    pub fn new(width: f32, height: f32) -> Self{
        let width_ratio = width / 64.0;
        let height_ratio = height / 32.0;
        
        Chip8Display { 
            vm: Chip8VM::new(),
            width_ratio,
            height_ratio,
            width,
            height 
        }
    }

    #[inline(always)]
    pub fn pixels(&self){
        for y in 0..32{
            for x in 0..64{
                let screen_x = self.width_ratio * x as f32;
                let screen_y = self.height_ratio * y as f32;
                if self.vm.display_buffer[x as usize][y as usize]{
                    draw_rectangle(screen_x as f32, screen_y as f32, self.width_ratio as f32, self.height_ratio as f32, WHITE);
                }
            }
        }
        self.memoryView();
    }

    #[inline(always)]
    pub fn memoryView(&self){
        let x = self.width_ratio as f32 * 50.0;
        draw_text("--Memory Viewer--", x, self.height_ratio as f32 * 2.0, 35.0, GREEN);
        let offset = self.vm.registers.PC - 20;
        for i in 0..20{
            let i = i * 2;
            let x = x + 50.0;
            let y = self.height_ratio as f32  * 0.5 * (6+i) as f32;
            let address = offset + i;
            if address == self.vm.registers.PC{
                draw_text("=>", x - 25.0, y, 25.0, BLUE);
            }
            let left = self.vm.memory[address] as u16;
            let right = self.vm.memory[address + 1] as u16;
            let instruction: u16 = left << 8 | right;
            draw_text(&format!("[0x{:04X}] 0x{:04X}",offset+i, instruction), x, y, 25.0, GREEN);
            
        }
        let x = x + 50.0;
        let y = self.height_ratio as f32  * 0.5 * (47) as f32;
        let instruction = self.vm.instruction();
        draw_text(&format!("PC: {} - 0x{:04X}",self.vm.registers.PC, instruction), x, y, 25.0, GREEN);
    
        draw_text(&format!("    --registers-- "), self.width_ratio as f32 * 50.0, y + 35.0, 30.0, GREEN);
        draw_text(&format!("{:?}", &self.vm.registers.general_purpose[..8]), self.width_ratio as f32 * 51.0, y + 55.0, 22.0, GREEN);
        draw_text(&format!("{:?}", &self.vm.registers.general_purpose[8..]), self.width_ratio as f32 * 51.0, y + 80.0, 22.0, GREEN);
        draw_text(&format!("I: 0x{:04X} has 0x{:04X}", self.vm.registers.I, self.vm.memory[self.vm.registers.I as usize]), self.width_ratio as f32 * 51.5, y + 105.0, 22.0, GREEN);
    
    }

    pub fn run(mut self){
        macroquad::Window::from_config(
            Conf {
                window_height: self.height as i32,
                window_width: self.width as i32,
                sample_count: 4,
                window_title: "Chip 8 Emulator 🐸".to_string(),
                high_dpi: true,
                ..Default::default()
            },
            async move{
                loop {
                    if is_key_released(KeyCode::Space){
                        self.vm.running = !self.vm.running;
                    }
                    if self.vm.running{
                        self.vm.tick();
                        sleep(Duration::from_secs_f64(1.0/self.vm.tick_rate as f64));
                    }
                    else if is_key_released(KeyCode::Enter){
                        self.vm.tick();
                    }
                    clear_background(BLACK);
                    self.pixels();
                    next_frame().await
                }
            }
        )
    }
}
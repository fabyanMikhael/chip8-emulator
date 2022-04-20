pub struct Registers{
    pub general_purpose: [u8; 16],
    pub I: u16,//used only for memory addresses (first 12 bits used)

    //audio related
    pub DT: u8, 
    pub ST: u8,

    pub PC: usize, //program counter
    pub SP: u8, //stack pointer

    pub stack: [u16;16], 
}


impl Registers{
    pub fn new() -> Self{
        Registers { 
            general_purpose: [0;16],
            I: 0,
            DT: 0, 
            ST: 0, 
            PC: 0x200, 
            SP: 0, 
            stack: [0;16]
        }
    }
}

pub struct Chip8VM{
    pub memory: [u8; 4096],
    pub registers: Registers,
    pub display_buffer: [[bool;32];64],
    pub tick_rate: u16,
    pub running: bool
}

impl Chip8VM{
    pub fn new() -> Self{
        let mut memory = [0;4096];
        font(&mut memory[0x50] as *mut _);
        Chip8VM { 
            memory,
            registers: Registers::new(),
            display_buffer: [[false;32];64],
            tick_rate: 10,
            running: false
        }
    }

    pub fn instruction(&self) -> u16{
        let left = self.memory[self.registers.PC] as u16;
        let right = self.memory[self.registers.PC + 1] as u16;
        left << 8 | right
    }
    pub fn tick(&mut self){
        let instruction: u16 = self.instruction();
        self.registers.PC += 2;
        // println!("{:#01x}", instruction);
        let nibble  = (instruction & 0xF000) >> 12;
        let X       = (instruction & 0x0F00) >> 8;
        let Y       = (instruction & 0x00F0) >> 4;
        let N       = (instruction & 0x000F) >> 0;
        let NN      = (instruction & 0x00FF) >> 0;
        let NNN     = (instruction & 0x0FFF) >> 0;
        
        match nibble{
            0x0 => {
                match NNN{
                    0x0E0 => {
                        self.display_buffer
                        .iter_mut().for_each(|column| {
                            for pixel in column{
                                *pixel = false;
                            }
                        })
                    },
                    _ =>  {
                        self.dump_memory();
                        panic!("0x0 - instruction <{:#01x}> unsupported", instruction)
                    }
                }
            },
            0x1 => {
                self.registers.PC = NNN as usize;
            },
            0x6 => {
                self.registers.general_purpose[X as usize] = NN as u8;
            },
            0x7 => {
                self.registers.general_purpose[X as usize] += NN as u8;
            },
            0xA => {
                self.registers.I = NNN;
            },
            0xD => {
                let x = self.registers.general_purpose[X as usize] % 64;
                let mut x = x as usize;
                let y = self.registers.general_purpose[Y as usize] % 32;
                let mut y = y as usize;
                self.registers.general_purpose[15] = 0;
                let bits = [0b1000_0000,0b0100_0000,0b0010_0000,0b0001_0000,
                            0b0000_1000,0b0000_0100,0b0000_0010,0b0000_0001];
                for n in 0..N{
                    let sprite_data = self.memory[(self.registers.I + n) as usize];
                    let mut x = x;
                    for bit in bits{
                        if bit & sprite_data != 0{
                            self.display_buffer[x][y] = !self.display_buffer[x][y];
                            if !self.display_buffer[x][y]{
                                self.registers.general_purpose[15] = 1;
                            }
                        }
                        x += 1;
                        if x > 63{break;}
                    }
                    y += 1;
                    if y > 31{break;}
                }
            }
            _ => {
                self.dump_memory();
                panic!("instruction <{:#01x}> unsupported", instruction)
            }
        }
    }

    pub fn load(&mut self, mut starting: usize, bytes: &[u8]){
        for &byte in bytes.iter(){
            self.memory[starting] = byte;
            starting += 1;
        }
    }

    pub fn dump_memory(&self){
        #[cfg(not(debug_assertions))]
        std::process::exit(1);

        println!("dumping memory...");
        for (index, mem) in self.memory.chunks_exact(2).enumerate(){
            let index = index * 2;
            if let [a,b] = mem{
                println!("[{index}] {}", format!("{:02X}{:02X}", a, b));
            }
        }
        println!("-------------");
    }
}

fn font(start: *mut u8){
    let font_array: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80];// F
    font_array.into_iter()
    .enumerate()
    .for_each(|(index, val)|{
        unsafe { *start.add(index) = val }
    });
}
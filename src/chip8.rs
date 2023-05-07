/*
CHIP-8 has the following components:

    Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
    Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
    A program counter, often called just “PC”, which points at the current instruction in memory
    One 16-bit index register called “I” which is used to point at locations in memory
    A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
    An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
    An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
    16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
*/

const MEM_SIZE: usize = 4096;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
const REG_SIZE: usize = 16;

#[derive(derive_getters::Getters)]
pub struct Chip8 {
    memory: [u8; MEM_SIZE],
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    program_counter: usize,
    index: usize,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    register: [u8; REG_SIZE],
}

struct Decode {
    first: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: usize,
}

impl Chip8 {
    pub fn new(rom: &[u8]) -> Chip8 {
        let mut chip = Chip8 {
            memory: [0; MEM_SIZE],
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            program_counter: 0x200,
            index: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            register: [0; REG_SIZE],
        };

        for i in 0..rom.len() {
            chip.memory[i + chip.program_counter] = rom[i];
        }

        chip
    }

    pub fn step(&mut self) {
        let instruction: u16 = self.fetch();
        let decoded_instruction: Decode = Self::decode(instruction);
        self.execute(&decoded_instruction);
    }

    fn fetch(&mut self) -> u16 {
        let first_byte: u8 = self.memory[self.program_counter];
        let second_byte: u8 = self.memory[self.program_counter + 1];

        self.program_counter += 2;

        let mut instruction: u16 = first_byte as u16;
        instruction <<= 8;
        instruction |= second_byte as u16;

        instruction
    }

    fn decode(instruction: u16) -> Decode {
        let first: u8 = ((instruction & 0b_1111_0000_0000_0000) >> 12) as u8;
        let x: u8 = ((instruction & 0b_0000_1111_0000_0000) >> 8) as u8;
        let y: u8 = ((instruction & 0b_0000_0000_1111_0000) >> 4) as u8;
        let n: u8 = (instruction & 0b_0000_0000_0000_1111) as u8;
        let nn: u8 = (instruction & 0b_0000_0000_1111_1111) as u8;
        let nnn: usize = (instruction & 0b_0000_1111_1111_1111) as usize;

        Decode {
            first,
            x,
            y,
            n,
            nn,
            nnn,
        }
    }

    fn execute(&mut self, decoded_instruction: &Decode) {
        match decoded_instruction.first {
            0x0 => match decoded_instruction.nn {
                0xE0 => self.clear_screen(),
                0xEE => self.return_method(),
                _ => panic!(),
            },
            0x1 => self.jump(&decoded_instruction),
            0x2 => (),
            0x3 => (),
            0x4 => (),
            0x5 => (),
            0x6 => self.set_register(&decoded_instruction),
            0x7 => self.add(&decoded_instruction),
            0x8 => (),
            0x9 => (),
            0xA => self.set_index(&decoded_instruction),
            0xB => (),
            0xC => (),
            0xD => self.set_display(&decoded_instruction),
            0xE => (),
            0xF => (),
            _ => panic!("First nibble of instruction was bigger then 4 bits can handle."),
        }
    }

    //#######################################################
    // INSTRUCTIONS
    //#######################################################

    //00E0
    fn clear_screen(&mut self) {
        for i in 0..self.display.len() {
            self.display[i] = false;
        }
    }

    //00EE
    fn return_method(&mut self) {}

    //1NNN
    fn jump(&mut self, decoded_instruction: &Decode) {
        self.program_counter = decoded_instruction.nnn;
    }

    //6XNN
    fn set_register(&mut self, decoded_instruction: &Decode) {
        self.register[decoded_instruction.x as usize] = decoded_instruction.nn;
    }

    //7XNN
    fn add(&mut self, decoded_instruction: &Decode) {
        self.register[decoded_instruction.x as usize] += decoded_instruction.nn;
    }

    //ANNN
    fn set_index(&mut self, decoded_instruction: &Decode) {
        self.index = decoded_instruction.nnn;
    }

    //DXYN
    fn set_display(&mut self, decoded_instruction: &Decode) {
        let x = self.register[decoded_instruction.x as usize] as usize % DISPLAY_WIDTH;
        let y = self.register[decoded_instruction.y as usize] as usize % DISPLAY_HEIGHT;

        let mut x_iter = x;
        let mut y_iter = y;

        self.register[0xF] = 0;

        for i in 0..decoded_instruction.n {
            let data = self.memory[self.index + i as usize];

            let mut mask: u8 = 0b1000_0000;
            let mut current_bit = 7;

            while current_bit >= 0 {
                let mut bit = data & mask;
                bit >>= current_bit;
                mask >>= 1;
                current_bit -= 1;

                let bit = match bit {
                    0 => false,
                    1 => true,
                    _ => panic!(),
                };

                if bit {
                    self.display[y_iter * DISPLAY_HEIGHT + x_iter] =
                        match self.display[y_iter * DISPLAY_HEIGHT + x_iter] {
                            true => {
                                self.register[0xF] = 1;
                                false
                            }
                            false => true,
                        };
                }

                x_iter += 1;

                if x_iter == DISPLAY_WIDTH {
                    break;
                }
            }
            x_iter = x;
            y_iter += 1;

            if y_iter == DISPLAY_HEIGHT {
                return;
            }
        }
    }
}

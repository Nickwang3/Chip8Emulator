
use std::vec::Vec;

const MEMORY_SIZE: usize = 4096;
const VREGISTER_COUNT: usize = 16;
const GFX_SIZE: usize = 64 * 32;
const STACK_SIZE: usize = 16;
const KEY_SIZE: usize = 16;

pub struct Chip8 {
    opcode: u16,
    memory: [u8;MEMORY_SIZE],
    v: [u8;VREGISTER_COUNT],
    i: i16,
    pc: u16,
    gfx: [u8;GFX_SIZE],
    draw_sema: bool,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16;STACK_SIZE],
    sp: u8,
    key: [u8;KEY_SIZE]
}


impl Chip8 {

    pub fn origin() -> Chip8 {
        Chip8 {
            opcode: 0x0000,
            memory: [0;MEMORY_SIZE],
            v: [0;VREGISTER_COUNT],
            i: 0x0000,
            pc: 0x0000,
            gfx: [0;GFX_SIZE],
            draw_sema: false,
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: [0;STACK_SIZE],
            sp: 0,
            key: [0;KEY_SIZE],
        }
    }

    /* 
        Initialize memory and register contents
    */
    pub fn initialize(&mut self) {
        self.pc = 0x200;
        self.opcode = 0x0000;
        self.i = 0x0000;
        self.sp = 0;


        // Clear display
        for i in 0..GFX_SIZE {
            self.gfx[i] = 0x00;
        }

        // Clear stack
        for i in 0..STACK_SIZE {
            self.stack[i] = 0x00;
        }

        // Clear registers V0-VF
        for i in 0..VREGISTER_COUNT {
            self.v[i] = 0x00;
        }

        // Clear memory
        for i in 0..MEMORY_SIZE {
            self.memory[i] = 0x00;
        }

        // Load fontset
        for i in 0..FONTSET_SIZE {
            self.memory[i + 80] = CHIP8_FONTSET[i];
        }


        println!("Initialized cpu!");
    }

    /*
        Load program for cpu to run
    */
    pub fn load(&mut self) {

        let path_to_program: String = String::from("src/programs/TETRIS");

        let buffer: Vec<u8>;
        
        match std::fs::read(&path_to_program) {
            Ok(bytes) => {
                buffer = bytes;
            }
            Err(e) => {
                panic!("Failed to read file: {}, {}", path_to_program, e);
            }
        }

        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }

        println!("Loaded program!");
    }

    /*
        Emulate cpu cycle by fetching, decoding, executing opcode
    */
    pub fn emulate_cycle(&mut self) {
        
        //to slow down cycles for now
        std::thread::sleep(std::time::Duration::from_millis(100));

        //for debugging
        println!("pc: {}, opcode: {:#x}", self.pc, self.opcode);

        // fetch opcode by combining two consecutive addresses in memory
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16);

        // decode opcode (TODO: CLEAN UP UGLY SWITCH STATEMENT)
        match self.opcode & 0xF000 {

            0x0000 => {
                match self.opcode & 0x0FFF {
                    0x00E0 => {
                        todo!();
                    }

                    0x0EE => {
                        //00EE
                        //Returns from a subroutine.
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    }

                    _=> {
                        //0NNN
                        //Calls RCA 1802 program at address NNN. Not necessary for most ROMs.
                        unimplemented!();
                    }
                }
            }

            0x1000 => {
                //1NNN
                //Jumps to address NNN.
                self.pc = self.opcode & 0x0FFF;
            }

            0x2000 => {
                //2NNN
                //Calls subroutine at NNN.
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }

            0x3000 => {
                //3XNN
                //Skips the next instruction if VX equals NN.
                let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                let reg_value: u8 = self.v[x];
                let comp_value: u8 = (self.opcode & 0x00FF) as u8;
                if reg_value == comp_value {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x4000 => {
                //4XNN
                //Skips the next instruction if VX doesn't equal NN.
                let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                let reg_value: u8 = self.v[x];
                let comp_value: u8 = (self.opcode & 0x00FF) as u8;
                if reg_value != comp_value {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x5000 => {
                //5XY0
                //Skips the next instruction if VX equals VY.
                let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x6000 => {
                //6XNN
                //Sets VX to NN.
                let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                self.v[x] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            0x7000 => {
                //7XNN
                //Adds NN to VX.
                let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                self.v[x] += (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            0x8000 => {
                match self.opcode & 0x000F {

                    0x0000 => {
                        //8XY0
                        //Sets VX to the value of VY.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    }

                    0x0001 => {
                        //8XY1
                        //Sets VX to VX or VY
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                        self.v[x] = self.v[x] | self.v[y];
                        self.pc += 2;
                    }

                    0x0002 => {
                        //8XY2
                        //Sets VX to VX and VY.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                        self.v[x] = self.v[x] & self.v[y];
                        self.pc += 2;
                    }

                    0x0003 => {
                        //8XY3
                        //Sets VX to VX xor VY.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                        self.v[x] = self.v[x] ^ self.v[y];
                        self.pc += 2;
                    }

                    0x0004 => {
                        //8XY4
                        //Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                        match self.v[x].checked_add(self.v[y]) {
                            Some(_result) => {
                                self.v[VREGISTER_COUNT - 1] = 0;
                            }
                            None => {
                                self.v[VREGISTER_COUNT - 1] = 1;
                            }
                        }
                        self.v[x] += self.v[y];
                        self.pc += 2;
                    }

                    0x0005 => {
                        //8XY5
                        //VY is subtraced from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                        match self.v[x].checked_sub(self.v[y]) {
                            Some(_result) => {
                                self.v[VREGISTER_COUNT - 1] = 1;
                            }
                            None => {
                                self.v[VREGISTER_COUNT - 1] = 0;
                            }
                        }
                        self.v[x] += self.v[y];
                        self.pc += 2;
                    }

                    0x0006 => {
                        //8XY6
                        //Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.v[VREGISTER_COUNT - 1] = self.v[x] & 0x01;
                        self.v[x] >>= 1;
                    }

                    0x0007 => {
                        //8XY7
                        //Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                        match self.v[y].checked_sub(self.v[x]) {
                            Some(_result) => {
                                self.v[VREGISTER_COUNT - 1] = 1;
                            }
                            None => {
                                self.v[VREGISTER_COUNT - 1] = 0;
                            }
                        }
                        self.v[x] = self.v[y] - self.v[x];
                        self.pc += 2;
                    }

                    0x000E => {
                        //8XYE
                        //Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
                        let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.v[VREGISTER_COUNT - 1] = (self.v[x] & 0x80) >> 7;
                        self.v[x] <<= 1;
                    }

                    _=> {
                        println!("opcode: {:#x?} not found!", self.opcode);
                        self.pc += 2;
                    }
                }
            }

            0x9000 => {
                //9XY0
                //Skips the next instruction if VX doesn't equal VY. 
                let x: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                let y: usize = ((self.opcode & 0x00F0) >> 4) as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0xA000 => {
                //ANNN
                //Sets I to the addresss NNN
                self.i = (self.opcode & 0x0FFF) as i16;
                self.pc += 2;
            }

            0xB000 => {
                //BNNN
                //Jumps to the address NNN plus V0
                self.pc = (self.v[0] as u16) + (self.opcode & 0x0FFF);
            }



            _ => {
                println!("opcode: {:#x?} not found!", self.opcode);
                // for debugging before all opcodes are implemented
                self.pc += 2;
            }
        }
    }

}


const FONTSET_SIZE: usize = 80;
const CHIP8_FONTSET: [u8;80] =
[
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
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
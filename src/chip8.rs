
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
    stack: [u8;STACK_SIZE],
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
        for i in 0..(64 * 32) {
            self.gfx[i] = 0x00;
        }

        // Clear stack
        for i in 0..16 {
            self.stack[i] = 0x00;
        }

        // Clear registers V0-VF
        for i in 0..16 {
            self.v[i] = 0x00;
        }

        // Clear memory
        for i in 0..4096 {
            self.memory[i] = 0x00;
        }

        // Load fontset
        for i in 0..80 {
            self.memory[i + 80] = CHIP8_FONTSET[i];
        }


        println!("Initialized cpu!");
    }

    /*
        Load program for cpu to run
    */
    pub fn load(&mut self) {

        // buffer is program to be loaded (hard coded right now)
        let buffer: [u8; 1000] = [0;1000];
        let buffer_size = 1000;

        for i in 0..buffer_size {
            self.memory[i + 512] = buffer[i];
        }

        println!("Loaded program!");
    }

    /*
        Emulate cpu cycle by fetching, decoding, executing opcode
    */
    pub fn emulate_cycle(&mut self) {
        // fetch opcode by combining two consecutive addresses in memory
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16);

        // decode opcode
        match self.opcode & 0xF000 {
            0xA000 => {
                self.i = (self.opcode & 0x0FFF) as i16;
                self.pc += 2;
            }

            _ => {
                println!("opcode: {:#x?} not found!", self.opcode);
                // for debugging before all opcodes are implemented
                self.pc += 2;
            }
        }
    }

}



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
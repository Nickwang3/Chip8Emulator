
mod chip8;

fn main() {

    let mut cpu = chip8::Chip8::origin();
    cpu.initialize();
    cpu.load();


    loop {
        cpu.emulate_cycle();
    }

}
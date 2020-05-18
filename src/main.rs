
mod chip8;
mod app;

fn main() {

    let mut cpu = chip8::Chip8::origin();
    cpu.initialize();
    cpu.load();


    loop {
        cpu.emulate_cycle();

        app::display(&cpu.get_gfx());

        // if (cpu.check_draw_sema()) {
        //     app::display(&cpu.get_gfx());
        // }
    }

}
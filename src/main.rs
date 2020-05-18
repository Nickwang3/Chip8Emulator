
mod chip8;
mod app;

fn main() {

    let mut cpu = chip8::Chip8::new();
    cpu.initialize();
    cpu.load();

    let mut app = app::App::new();
    app.render();

    loop {
        cpu.emulate_cycle();

        if cpu.check_draw_sema() {
            app.update(&cpu.get_gfx());
            app.render();
        }
    }

}
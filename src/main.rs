
mod chip8;
mod app;

fn main() {

    let mut cpu = chip8::Chip8::new();
    cpu.initialize();
    cpu.load(String::from("src/programs/PONG2"));

    let mut app = app::App::new();
    app.render();

    loop {
        cpu.emulate_cycle();

        if cpu.check_draw_sema() {
            app.update(&cpu.get_gfx());
            app.render();
        }

        // if cpu.check_key_sema() {
        //     println!("key pressed was: {}", app.await_keypress());
        // }
        cpu.update_keystate(&app.get_keystate());
    }

}
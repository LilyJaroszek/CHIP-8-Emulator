mod emulator;
mod chip8;
mod io;

fn main(){
    emulator::emulator_loop();
}
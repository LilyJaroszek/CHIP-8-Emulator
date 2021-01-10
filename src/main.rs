mod chip8;
mod io;
use std::env;
use std::time;
use std::thread::sleep;
use std::time::Duration;


pub struct DebugInfo {
    pub opcode: u16,
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub delay_tmr: u8,
    pub sound_tmr: u8,
    pub stack: [u16; 16],
    pub sp: u16,
    pub opcode_trans: String,
    pub keypad: [u8; 16]
}



//TODO
//Code cleanup
//fix sound
//Add way to look at memory
//Instructions display hex
//Super Chip

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: Need to specify program location as Command Line Arg");
    }
    let mut debug = false;
    let mut step = false;
    for num_arg in 2..args.len() {
        let arg = &args[num_arg];
        if arg == "-debug"{
            debug = true;
        } else if arg == "-step"{
            step = true;
        } else {
            panic!("Option not recognized: {}",arg)
        }

    }
    
    
    let rom = io::load_rom(&args[1]);
    let mut emu = chip8::init(rom);
    let mut engine = io::init();
    let mut exit = false;
   
    let mut debug_info = DebugInfo {
        opcode: 0,
        v: [0; 16],
        i: 0,
        pc: 0,
        delay_tmr: 0,
        sound_tmr: 0,
        stack: [0; 16],
        sp: 0,
        opcode_trans: "".to_string(),
        keypad: [0; 16]
    };

    let mut next_step = false;

    let fps = 60;
    let frame_time_ms = (1/fps)*1000 as u128;

    let mut draw = false;
    let mut beep = false;

    

    while !exit {
        let start_time = time::Instant::now();
        if !step || next_step {
            emu.cycle(debug,&mut debug_info, &mut draw, &mut beep);
        }
        engine.draw(emu.gfx,debug,&mut debug_info,step, &mut draw);
        draw = false;
        engine.sound(&mut beep);
        let key_actions = engine.input(&mut emu.keypad);

        exit = key_actions.exit;
        next_step = key_actions.next_step;
        if key_actions.step {
            step = !step;
        }
        if key_actions.debug {
            debug = !debug;
        }

        let time_left = frame_time_ms.saturating_sub(start_time.elapsed().as_millis());
        if time_left > 0 {
            sleep(Duration::from_millis(time_left as u64));
        }

    }

    engine.deinit();
}



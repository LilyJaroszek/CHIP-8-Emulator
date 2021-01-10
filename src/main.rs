mod chip8;
mod io;
use std::env;
use std::time;
use std::thread::sleep;
use std::time::Duration;

//TODO
//fix sound
//Add way to look at memory
//Panic handler
//Info and documentation
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
   
    let mut debug_info = chip8::DebugInfo {
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

    let cpu_speed = 700;
    let cpu_time_ms = (1/cpu_speed)*1000000 as u128;

    let mut beep = false;
    let mut info_redraw = true;

    

    while !exit {
        let start_time_fps = time::Instant::now();
        let mut draw = false;
        
        while (frame_time_ms.saturating_sub(start_time_fps.elapsed().as_millis()) > 0  || !draw) && !exit {
            let start_time_cpu = time::Instant::now();
            if (!step || next_step) && !draw {
                emu.cycle(debug,&mut debug_info, &mut draw, &mut beep);
            }
            if info_redraw{
                engine.info_draw(&mut debug_info,debug,step);
            }
            engine.sound(&mut beep);

            let key_actions = engine.input(&mut emu.keypad);
            exit = key_actions.exit;
            next_step = key_actions.next_step;
            if key_actions.step {
                step = !step;
                info_redraw = true;
            }
            if key_actions.debug {
                debug = !debug;
                info_redraw = true;
            }

            let cpu_time_remaining = cpu_time_ms.saturating_sub(start_time_cpu.elapsed().as_micros());
            if cpu_time_remaining > 0 {
                sleep(Duration::from_micros(cpu_time_remaining as u64));
            }
        }
        engine.draw(emu.gfx);
    }

    engine.deinit();
}



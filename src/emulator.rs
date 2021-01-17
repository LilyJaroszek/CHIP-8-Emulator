use crate::chip8;
use crate::io;
use std::env;
use std::time;
use std::thread::sleep;
use std::time::Duration;

/*TODO:
Memory Dumper
Panic handler
How to use controls documentation
Instructions display hex
Super Chip implementation
*/

pub fn emulator_loop() {
    //Check the command line args for specified ROM and flags
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
    
    //Initialize the components of the emulator
    let rom = io::load_rom(&args[1]);
    let mut emu = chip8::init(rom);
    let mut engine = io::init();

    //Target graphics FPS of 60
    let fps = 60;
    let frame_time_ms = (1/fps)*1000 as u128;

    //Target cpu speed of 500 Hz
    let cpu_speed = 500;
    let cpu_time_ms = (1/cpu_speed)*1000 as u128;

    //Set to true when sound beep is needed
    let mut beep = false;

    //Set to true when debugging info needs to be redrawn on screen
    let mut debug_redraw = true;

    //Used when in step mode to step to the next CPU instruction
    let mut next_step = false;

    let mut exit = false;

    while !exit {

        let start_time_fps = time::Instant::now();
        let mut draw = false;
        
        //Keep cycling if there is still time until the screen needs to be drawn to the target FPS
        while (frame_time_ms.saturating_sub(start_time_fps.elapsed().as_millis()) > 0  || !draw) && !exit {
            let start_time_cpu = time::Instant::now();

            //If in step mode then wait for the next CPU instruction to be requested to emulate another cpu cycle
            //Also if a draw is requested wait to emulate the next CPU cycle until the screen is drawn
            if (!step || next_step) && !draw {
                emu.cycle(debug, &mut draw, &mut beep);
            }

            if debug_redraw{
                engine.info_draw(&mut emu.debug_info,debug,step);
            }

            engine.sound(&mut beep);

            let key_actions = engine.input(&mut emu.keypad);

            exit = key_actions.exit;
            next_step = key_actions.next_step;
            if key_actions.step {
                step = !step;
                debug_redraw = true;
            }
            if key_actions.debug {
                debug = !debug;
                debug_redraw = true;
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



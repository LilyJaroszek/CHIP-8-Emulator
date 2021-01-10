use std::path::Path;
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::time::Duration;

use crossterm::terminal::{Clear,ClearType,enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue, style};
use crossterm::cursor::{Hide,Show,MoveTo};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{DebugInfo, KeyActions};

pub struct Engine {
    keys: [char; 16],
}

impl Engine {
    pub fn draw (&mut self, gfx: [u8; 2048], debug: bool, debug_info: &mut DebugInfo, step: bool){
        let mut stdout = stdout();

        execute!(stdout,MoveTo(0, 0));
        for y in 0..32 {
            for x in 0..64 {
                if gfx[(y*64)+x] != 0 {
                    queue!(stdout,style::Print("#"));
                } else {
                    queue!(stdout,style::Print(" "));
                }
            }
            queue!(stdout,style::Print("\r\n"));  
        }
        stdout.flush().unwrap();

        println!();
        if debug {
            println!("Opcode: {:#06X} {:<32}",debug_info.opcode,debug_info.opcode_trans);
            println!("Program Counter: {:#06X}",debug_info.pc);
            println!("I: {:#06X}",debug_info.i);
            println!("Stack Pointer: {:#06X}",debug_info.sp);
            println!("Delay Timer: {:#04X}",debug_info.delay_tmr);
            println!("Sound Timer: {:#04X}",debug_info.sound_tmr);
            for num in 0..16 {
                println!("V{:X} : {:#04X} Keypad{:X}: {:#04X} Stack{:X} : {:#06X}",num,debug_info.v[num],num,debug_info.keypad[num],num,debug_info.stack[num]);
            }
        } else {
            for _y in 0..22 {
                for _x in 0..50 {
                    print!(" ")
                }
                println!()
            }
        }
        if step {
            println!("Press Down to step to next instruction...")
        } else {
            for _x in 0..50 {
                print!(" ")
            }
            println!()
        }
    }
   
   pub fn input(&mut self, keypad: &mut [u8; 16], key_actions: &mut KeyActions) {
        enable_raw_mode().unwrap();

        if let Ok(has_event) = poll(Duration::from_millis(0)){
            if has_event{
                if let Ok(current_event) = read(){
                    match current_event {
                        Event::Key(event) => {

                            if event == KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE) {
                                key_actions.exit = true;
                            } else if event == KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE){
                                key_actions.step = true;
                            } else if event == KeyEvent::new(KeyCode::Down, KeyModifiers::NONE){
                                key_actions.next_step = true;
                            } else if event == KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE){
                                key_actions.debug = true;
                            }else {
                                for key in 0..keypad.len() {
                                    if event == KeyEvent::new(KeyCode::Char(self.keys[key]), KeyModifiers::NONE){
                                        keypad[key] = 1;
                                    }
                                }
                            }

                        }
                        Event::Mouse(_event) => {

                        }
                        Event::Resize(_x,_y) => {

                        }
                    }
                }
            }
            
        } 
        disable_raw_mode().unwrap();
    }

    pub fn deinit(self) {
        execute!(stdout(),Show,LeaveAlternateScreen);
    }
}

pub fn init(scale: u32) -> Engine {
    execute!(stdout(),EnterAlternateScreen,Hide,Clear(ClearType::All));
    let mut engine = Engine {

        keys: ['x','1','2','3'
        ,'q','w','e','a'
        ,'s','d','z','c'
        ,'4','r','f','v']
    };
    return engine;
}



pub fn load_rom(file: &str) -> [u8; 3584] {
    let mut rom = [0; 3584];
    let p = &("roms/".to_owned()+file); 
    let path = Path::new(p);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let size = file.metadata().unwrap().len();
    if size > 3584 {
        panic!("ROM too big: {} Bytes out of max 3584 Bytes",size);
    } else {
        let _n = file.read(&mut rom);
    }
    return rom;
}







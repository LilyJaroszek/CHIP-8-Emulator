use std::path::Path;
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::time::Duration;
use std::time::SystemTime;

use crossterm::terminal::{Clear,ClearType,enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue, style};
use crossterm::cursor::{Hide,Show,MoveTo};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};

use rodio::{OutputStream,Sink,source};

use crate::chip8::DebugInfo;

pub struct Engine {
    sound_sink: Sink,
    beep_timer: u8,
    keys: [char; 16],
    key_timer: [u8; 16]
}

pub struct KeyActions {
    pub exit: bool,
    pub next_step: bool,
    pub step: bool,
    pub debug: bool,
    pub mem_dump: bool
}

impl Engine {

    //Draws the chip8 graphics screen
    pub fn draw (&mut self, gfx: [u8; 2048]){
        let mut stdout = stdout();
        let _r = queue!(stdout,MoveTo(0, 0));
        for y in 0..32 {
            for x in 0..64 {
                if gfx[(y*64)+x] != 0 {
                    let _r = queue!(stdout,style::Print("#"));
                } else {
                    let _r = queue!(stdout,style::Print(" "));
                }
            }
            let _r = queue!(stdout,style::Print("\r\n"));  
        }

        stdout.flush().unwrap();
    }
    
    //Draws the debug information and step information
    pub fn info_draw(&mut self, debug_info: DebugInfo, debug: bool, step: bool){
        let mut stdout = stdout();
        let _r = queue!(stdout,MoveTo(0, 32));
        let _r = queue!(stdout,style::Print("\r\n"));
        if debug {
            let _r = queue!(stdout,style::Print(format!("Opcode: {:#06X} {:<32}\r\n",debug_info.opcode,debug_info.opcode_trans)));
            let _r = queue!(stdout,style::Print(format!("Program Counter: {:#06X}\r\n",debug_info.pc)));
            let _r = queue!(stdout,style::Print(format!("I: {:#06X}\r\n",debug_info.i)));
            let _r = queue!(stdout,style::Print(format!("Stack Pointer: {:#06X}\r\n",debug_info.sp)));
            let _r = queue!(stdout,style::Print(format!("Delay Timer: {:#04X}\r\n",debug_info.delay_tmr)));
            let _r = queue!(stdout,style::Print(format!("Sound Timer: {:#04X}\r\n",debug_info.sound_tmr)));
            for num in 0..16 {
                let _r = queue!(stdout,style::Print(format!("V{:X} : {:#04X} Keypad{:X}: {:#04X} Stack{:X} : {:#06X}\r\n",num,debug_info.v[num],num,debug_info.keypad[num],num,debug_info.stack[num])));
            }
        } else {
            for _y in 0..22 {
                for _x in 0..50 {
                    let _r = queue!(stdout,style::Print(" "));
                }
                let _r = queue!(stdout,style::Print("\r\n"));
            }
        }
        if step {
            let _r = queue!(stdout,style::Print("Press Down to step to next instruction...\r\n"));
        } else {
            for _x in 0..50 {
                let _r = queue!(stdout,style::Print(" "));
            }
            let _r = queue!(stdout,style::Print("\r\n"));
        }

        stdout.flush().unwrap();
    }
   
    pub fn input(&mut self, keypad: &mut [u8; 16]) -> KeyActions {
        /*When a key is dectected as pressed a timer is used to keep the correponding key variable
        in the pressed value for a number of cycles since otherwise the CPU will not reliably
        dectect when the key is being pressed*/
        for i in 0..self.key_timer.len(){
            if self.key_timer[i] > 0 {
                self.key_timer[i]-=1;
                if self.key_timer[i] == 0 {
                    keypad[i] = 0;
                }
            } 
             
        }

        let mut key_actions = KeyActions {
            exit: false,
            next_step: false,
            step: false,
            debug: false,
            mem_dump: false,
        };

        for _x in 0..1 {
            if let Ok(has_event) = poll(Duration::from_micros(0)){
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
                                }else if event == KeyEvent::new(KeyCode::End, KeyModifiers::NONE){
                                    key_actions.mem_dump = true;
                                }else {
                                    for key in 0..keypad.len() {
                                        if event == KeyEvent::new(KeyCode::Char(self.keys[key]), KeyModifiers::NONE){
                                            keypad[key] = 1;
                                            self.key_timer[key] = 25;
                                        }
                                    }
                                }

                            }
                            Event::Mouse(_event) => {}
                            Event::Resize(_x,_y) => {}
                        }
                    }
                }
            
            } 
        }

        return key_actions;
        
    }

    //TODO: fix sound, currently does not beep when requested
    pub fn sound (&mut self, beep: &mut bool){
        if *beep {
            self.sound_sink.play();
            self.beep_timer = 25;
            *beep = false;
        }
        if self.beep_timer > 0 {
            self.beep_timer -= 1;
            if self.beep_timer == 0 {
                self.sound_sink.pause();
            }
        }
    }
    
    pub fn deinit(self) {
        let _r = execute!(stdout(),Show,LeaveAlternateScreen);
        disable_raw_mode().unwrap();
    }
}

pub fn init() -> Engine {
    let _r = execute!(stdout(),EnterAlternateScreen,Hide,Clear(ClearType::All));
    enable_raw_mode().unwrap();
    
    let engine = Engine {
        sound_sink: Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap(),
        beep_timer: 0,
        keys: ['x','1','2','3'
        ,'q','w','e','a'
        ,'s','d','z','c'
        ,'4','r','f','v'],
        key_timer: [0; 16]
    };
    engine.sound_sink.append(source::SineWave::new(500));
    engine.sound_sink.pause();

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

pub fn write_mem_dump_file(mem_dump: [u8; 4096]){
    //TODO: change path to folder, unique file names, and readable files
    let path = Path::new("dump.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(&mem_dump) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}







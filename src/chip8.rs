
use rand::Rng;

pub struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    pub gfx: [u8; 2048],
    delay_tmr: u8,
    sound_tmr: u8,
    stack: [u16; 16],
    sp: u16,
    pub keypad: [u8; 16],
    pub debug_info: DebugInfo

}

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

impl Chip8 {
    fn init(rom: [u8; 3584]) -> Chip8{
        let mut chip8 = Chip8{
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 2048],
            delay_tmr: 0,
            sound_tmr: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
            debug_info: DebugInfo {
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
            }
        };
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for byte in 0..fontset.len() {
            chip8.memory[byte] = fontset[byte];
        }
        for byte in 0..rom.len() {
            chip8.memory[0x200+byte] = rom[byte];
        }
        return chip8;
    }

    pub fn cycle(&mut self, debug: bool, draw: &mut bool, beep: &mut bool) {
        //Fetch the Opcode from memory
        let pc = self.pc as usize;
        let op1 = self.memory[pc] as u16;
        let op2 = self.memory[pc+1] as u16;
        let opcode = (op1 << 8) | op2;
        self.opcode = opcode;

        if debug {
            self.debug_info.opcode = self.opcode;
            self.debug_info.i = self.i;
            self.debug_info.pc = self.pc;
            self.debug_info.delay_tmr = self.delay_tmr;
            self.debug_info.sound_tmr = self.sound_tmr;
            self.debug_info.sp = self.sp;
            for i in 0..16 {
                self.debug_info.v[i] = self.v[i];
                self.debug_info.stack[i] = self.stack[i];
                self.debug_info.keypad[i] = self.keypad[i];
            }    
        }

        //Decode the instruction
        let first = self.opcode & 0xF000;
        let fl = self.opcode & 0xF00F;
        let ftl = self.opcode & 0xF0FF;
        
        if self.opcode == 0x00E0 { //Clear Screen
            for byte in 0..self.gfx.len() {
                self.gfx[byte] = 0;
            }
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "DISPLAY_CLEAR".to_string();
            }

        } else if self.opcode == 0x00EE { //Return from subroutine
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "RETURN".to_string();
            }

        } else if first == 0x1000 { //Jump to address NNN
            self.pc = self.opcode & 0x0FFF;

            if debug {
                self.debug_info.opcode_trans = "JUMP ".to_string();
                self.debug_info.opcode_trans.push_str(&self.pc.to_string());
            }

        } else if first == 0x2000 { //Call subroutine at NNN
            self.stack[self.sp as usize] = self.pc;
            self.sp += 1;
            self.pc = self.opcode & 0x0FFF;

            if debug {
                self.debug_info.opcode_trans = "SUBROUTINE ".to_string();
                self.debug_info.opcode_trans.push_str(&self.pc.to_string());
            }

        } else if first == 0x3000 { //Skip next instruction if Vx == NN
            self.pc += 2;
            let reg = ((self.opcode & 0x0F00) >> 8) as usize;
            let num = (self.opcode & 0x00FF) as u8;
            if self.v[reg] == num {
                self.pc += 2;
            }

            if debug {
                self.debug_info.opcode_trans = "SKIP_IF_EQUAL V".to_string();
                self.debug_info.opcode_trans.push_str(&reg.to_string());
                self.debug_info.opcode_trans.push_str(" ");
                self.debug_info.opcode_trans.push_str(&num.to_string());
            }

        } else if first == 0x4000 { //Skip next instruction if Vx != NN
            self.pc += 2;
            let reg = ((self.opcode & 0x0F00) >> 8) as usize;
            let num = (self.opcode & 0x00FF) as u8;
            if self.v[reg] != num {
                self.pc += 2;
            }

            if debug {
                self.debug_info.opcode_trans = "SKIP_IF_NOT_EQUAL V".to_string();
                self.debug_info.opcode_trans.push_str(&reg.to_string());
                self.debug_info.opcode_trans.push_str(" ");
                self.debug_info.opcode_trans.push_str(&num.to_string());
            }
        
        } else if fl == 0x5000 { //Skip next instruction if Vx == Vy
            self.pc += 2;
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            if self.v[reg_x] == self.v[reg_y] {
                self.pc += 2;
            }

            if debug {
                self.debug_info.opcode_trans = "SKIP_IF_EQUAL V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }
        
        } else if first == 0x6000 { //Set Vx == NN
            let reg = ((self.opcode & 0x0F00) >> 8) as usize;
            let num = (self.opcode & 0x00FF) as u8;
            self.v[reg] = num;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET V".to_string();
                self.debug_info.opcode_trans.push_str(&reg.to_string());
                self.debug_info.opcode_trans.push_str(" ");
                self.debug_info.opcode_trans.push_str(&num.to_string());
            }

        } else if first == 0x7000 { //Add NN to Vx
            let reg = ((self.opcode & 0x0F00) >> 8) as usize;
            let num = (self.opcode & 0x00FF) as u8;
            let result = self.v[reg].overflowing_add(num);
            self.v[reg] = result.0;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "ADD V".to_string();
                self.debug_info.opcode_trans.push_str(&reg.to_string());
                self.debug_info.opcode_trans.push_str(" ");
                self.debug_info.opcode_trans.push_str(&num.to_string());
            }


        } else if fl == 0x8000 { //Set Vx = Vy
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            self.v[reg_x] = self.v[reg_y];
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }

        } else if fl == 0x8001 { //Set Vx = Vx | Vy
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            self.v[reg_x] = self.v[reg_x] | self.v[reg_y];
            self.pc += 2;

            
            if debug {
                self.debug_info.opcode_trans = "OR V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }
    
        } else if fl == 0x8002 { //Set Vx = Vx & Vy
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            self.v[reg_x] = self.v[reg_x] & self.v[reg_y];
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "AND V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }
    
        } else if fl == 0x8003 { //Set Vx = Vx ^ Vy
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            self.v[reg_x] = self.v[reg_x] ^ self.v[reg_y];
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "XOR V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }
    
        } else if fl == 0x8004 { //Add Vy to Vx
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            
            let result = self.v[reg_x].overflowing_add(self.v[reg_y]);
            self.v[reg_x] = result.0;

            if result.1 {
                self.v[0xF] = 1;
            } else {
                self.v[0xF] = 0;
            }
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "ADD V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }
    
        } else if fl == 0x8005 { //Subtract Vy from Vx
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;

            let result = self.v[reg_x].overflowing_sub(self.v[reg_y]);
            self.v[reg_x] = result.0;

            if result.1 {
                self.v[0xF] = 0;
            } else {
                self.v[0xF] = 1;
            }
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SUB V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }
    
        } else if fl == 0x8006 { //Bitshift Vx right by 1
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            self.v[0xF] = self.v[reg_x] & 0x1;
            self.v[reg_x] = self.v[reg_x] >> 1;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "BITSHIFT_RIGHT V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }
    
        } else if fl == 0x8007 { //subtract Vx from Vy
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;

            let result = self.v[reg_y].overflowing_sub(self.v[reg_x]);
            self.v[reg_x] = result.0;

            if result.1 {
                self.v[0xF] = 0;
            } else {
                self.v[0xF] = 1;
            }
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SUB V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }
    
        } else if fl == 0x800E { //Bitshift Vx left by 1
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            self.v[0xF] = self.v[reg_x] >> 7;
            self.v[reg_x] = self.v[reg_x] << 1;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "BITSHIFT_LEFT V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }
    
        } else if fl == 0x9000 { //Skip next instruction if Vx != Vy
            self.pc += 2;
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            if self.v[reg_x] != self.v[reg_y] {
                self.pc += 2;
            }

            if debug {
                self.debug_info.opcode_trans = "SKIP_IF_NOT_EQUAL V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
            }

        } else if first == 0xA000 { //Set I to address NNN
            let address = self.opcode & 0x0FFF;
            self.i = address;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET_I ".to_string();
                self.debug_info.opcode_trans.push_str(&address.to_string());
            }


        } else if first == 0xB000 { //Jump to address V0+NNN
            let address = self.opcode & 0x0FFF;
            self.pc = self.v[0] as u16 + address;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "JUMP_TO_V0_ADD ".to_string();
                self.debug_info.opcode_trans.push_str(&address.to_string());
            }

        } else if first == 0xC000 { //Set Vx to a random number & NN
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let num = (self.opcode & 0x00FF) as u8;
            let mut rng = rand::thread_rng();
            self.v[reg_x] = rng.gen_range(0..256) as u8 & num;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET_TO_RANDOM_AND V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" ");
                self.debug_info.opcode_trans.push_str(&num.to_string());
            }

        } else if first == 0xD000 { //Draw sprite
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let reg_y = ((self.opcode & 0x00F0) >> 4) as usize;
            let height = self.opcode & 0x000F;
            self.v[0xF] = 0;
            *draw = true;

            for y in 0..height {
                let line = self.memory[(self.i+y) as usize];
                for x in 0..8 {

                    if (line & (0x80 >> x)) != 0 {

                        let pos = ((((y + self.v[reg_y] as u16)*64) + x + self.v[reg_x] as u16)%2048) as usize;
                        if self.gfx[pos] == 1 {
                            self.v[0xF] = 1;
                        }
                        self.gfx[pos] ^= 1;
                    } 
                }
            }

            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "DRAW_SPRITE_XYH V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
                self.debug_info.opcode_trans.push_str(" V");
                self.debug_info.opcode_trans.push_str(&reg_y.to_string());
                self.debug_info.opcode_trans.push_str(" ");
                self.debug_info.opcode_trans.push_str(&height.to_string());
            }

        } else if ftl == 0xE09E { //Skip next instruction if key specified is pressed
            self.pc += 2;
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let key = self.keypad[self.v[reg_x] as usize];
            if key != 0 {
                self.pc += 2;
            }

            if debug {
                self.debug_info.opcode_trans = "SKIP_IF_KEY_PRESSED ".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xE0A1 { //Skip next instruction if key specified is not pressed
            self.pc += 2;
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let key = self.keypad[self.v[reg_x] as usize];
            if key == 0 {
                self.pc += 2;
            }

            if debug {
                self.debug_info.opcode_trans = "SKIP_IF_KEY_NOT_PRESSED ".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }


        } else if ftl == 0xF007 { //Set Vx to delay timer
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            self.v[reg_x] = self.delay_tmr;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET_TO_DELAY_TIMER V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF00A { //Keypress is waited for and stored in Vx
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            for key in 0..self.keypad.len() {
                if self.keypad[key] != 0 {
                    self.v[reg_x] = key as u8;
                    self.pc += 2;
                    break;
                }
            }

            if debug {
                self.debug_info.opcode_trans = "WAIT_FOR_KEYPRESS_AND_STORE V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF015 { //Set delay timer to Vx
            let reg_x = ((self.opcode & 0x0F00) >> 8) as u8;
            self.delay_tmr = reg_x;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET_DELAY_TIMER V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF018 { //Set sound timer to Vx
            let reg_x = ((self.opcode & 0x0F00) >> 8) as u8;
            self.sound_tmr = reg_x;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET_SOUND_TIMER V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF01E { //Add Vx to I
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            self.i += self.v[reg_x] as u16;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "ADD_TO_I V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF029 { //Set I to location of a character sprite
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            self.i = (self.v[reg_x] as u16) * 5;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "SET_I_TO_SPRITE ".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF033 { //Stores BCD Representation at address at I
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            self.memory[self.i as usize] = self.v[reg_x] / 100;
            self.memory[(self.i+1) as usize] = (self.v[reg_x] % 100) / 10;
            self.memory[(self.i+2) as usize] = (self.v[reg_x] % 100) % 10;
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "STORE_BCD_AT_I V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF055 { //Store V0 to VX in memory starting at I
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let mut address = self.i as usize;
            for num in 0..=reg_x {
                self.memory[address] = self.v[num];
                address += 1;
            }
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "STORE_V0_TO_VX_AT_I V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else if ftl == 0xF065 { //Load V0 to VX from memory starting at I
            let reg_x = ((self.opcode & 0x0F00) >> 8) as usize;
            let mut address = self.i as usize;
            for num in 0..=reg_x {
                self.v[num] = self.memory[address];
                address += 1;
            }
            self.pc += 2;

            if debug {
                self.debug_info.opcode_trans = "LOAD_V0_TO_VX_AT_I V".to_string();
                self.debug_info.opcode_trans.push_str(&reg_x.to_string());
            }

        } else {
            panic!("Unknown opcode: [{:X}]",self.opcode)
        }
        
        //Increment the timers if they are running
        if self.delay_tmr > 0 {
            self.delay_tmr-=1;
        }
        if self.sound_tmr > 0 {
            self.sound_tmr-=1;
            if self.sound_tmr == 0 {
                *beep = true;
            }
        }

    }

}

pub fn init(rom: [u8; 3584]) -> Chip8 {
   Chip8{..Chip8::init(rom)}
}





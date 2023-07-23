struct CPU {
    // current_operation: u16, // ALL CHIP-8 opcodes are u16
    registers: [u8; 16],
    position_in_memory: usize, // more formally called **program counter**
    memory: [u8; 0x1000],      // 4096 bytes of RAM
    // after 16 nested function calls, overflow!
    stack: [u16; 16],
    stack_pointer: usize, // decremented by RETURN
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte_1 = self.memory[p] as u16;
        let op_byte_2 = self.memory[p + 1] as u16;

        // op_byte_1 needs to be cast as u16 first
        // otherwise, as u8, all its bits will be set to 0 when << 8
        op_byte_1 << 8 | op_byte_2
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer <= 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    // (1nnn) JUMP to `addr`
    fn jmp(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2; // points to the next instruction
            let _c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let _d = ((opcode & 0x000F) >> 0) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let op_minor = (opcode & 0x000F) as u8;

            let addr = opcode & 0xFFF;

            match opcode {
                // dispatches execution to the hardware circuit responsible for performing
                0x0000 => {
                    return;
                }
                0x00E0 => { /* CLEAR SCREEN */ }
                0x00EE => self.ret(),
                0x1000..=0x1FFF => {
                    self.jmp(addr);
                }
                0x2000..=0x2FFF => self.call(addr),
                0x3000..=0x3FFF => self.se(x, kk),
                0x4000..=0x4FFF => self.sne(x, kk),
                0x5000..=0x5FFF => self.se(x, y),
                0x6000..=0x6FFF => self.ld(x, kk),
                0x7000..=0x7FFF => self.add(x, kk),
                0x8000..=0x8FFF => match op_minor {
                    0 => self.ld(x, self.registers[y as usize]),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => self.add_xy(x, y),
                    _ => todo!("opcode: {:04x}", opcode),
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];
        self.registers[x as usize] = x_ & y_;
    }
    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];
        self.registers[x as usize] = x_ | y_;
    }
    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];
        self.registers[x as usize] = x_ ^ y_;
    }

    // LD sets the value `kk` into register `vx`
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    // STORE if equal
    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.position_in_memory += 2;
        }
    }
    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.position_in_memory += 2;
        }
    }

    // ADD adds value `kk` into register `vx`
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg_1 = self.registers[x as usize];
        let arg_2 = self.registers[y as usize];
        let (val, overflow_detected) = arg_1.overflowing_add(arg_2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        memory: [0; 4096],
        registers: [0; 16],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };
    /*
     * 8: the operation involves 2 registers
     * 0: maps to registers[0]
     * 1: maps to registers[1]
     * 4: addition
     */
    // cpu.current_operation = 0x8014;
    // cpu.registers[0] = 5;
    // cpu.registers[1] = 10;

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // opcode: 0x2100 - CALL the function at 0x100
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    // opcode: 0x2100 - CALL the function at 0x100
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    // set opcode to 0x0000
    // HALT
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;
    // loads opcode 0x8014, which adds register 1 to register 0
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    // loads opcode 0x8024, which adds register 2 to register 0
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    // set opcode to 0x00EE - RETURN
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

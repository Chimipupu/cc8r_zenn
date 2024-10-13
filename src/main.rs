use std::fmt;

// CPUの構造体
struct CC8R {
    registers: [u8; 8],
    pc: u8,
    sp: u8,
    flags: u8,
    memory: [u8; 256],
}

// フラグのビット位置
const FLAG_ZERO: u8 = 0b10000000;
const FLAG_CARRY: u8 = 0b01000000;
const FLAG_OVERFLOW: u8 = 0b00100000;
const FLAG_NEGATIVE: u8 = 0b00010000;

// オペコード
const OP_NOP: u8 = 0x00;
const OP_HALT: u8 = 0x10;
const OP_LDI: u8 = 0x14;
const OP_MV: u8 = 0x18;
const OP_ADD: u8 = 0x20;
const OP_SUB: u8 = 0x30;
const OP_MUL: u8 = 0x40;
const OP_DIV: u8 = 0x50;
const OP_AND: u8 = 0x60;
const OP_OR: u8 = 0x70;
const OP_XOR: u8 = 0x80;
const OP_SHL: u8 = 0x90;
const OP_SHR: u8 = 0xA0;
const OP_PUSH: u8 = 0xB0;
const OP_POP: u8 = 0xC0;
const OP_JMP: u8 = 0xD0;
const OP_JZ: u8 = 0xE0;
const OP_JNZ: u8 = 0xF0;

impl CC8R {
    fn new() -> Self {
        CC8R {
            registers: [0; 8],
            pc: 0,
            sp: 0xF0,
            flags: 0,
            memory: [0; 256],
        }
    }

    fn fetch(&mut self) -> u8 {
        let instruction = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        instruction
    }

    fn decode(&self, instruction: u8) -> String {
        match instruction {
            OP_NOP => "NOP".to_string(),
            OP_LDI => "LDI".to_string(),
            OP_MV => "MV".to_string(),
            OP_ADD => "ADD".to_string(),
            OP_SUB => "SUB".to_string(),
            OP_MUL => "MUL".to_string(),
            OP_DIV => "DIV".to_string(),
            OP_AND => "AND".to_string(),
            OP_OR => "OR".to_string(),
            OP_XOR => "XOR".to_string(),
            OP_SHL => "SHL".to_string(),
            OP_SHR => "SHR".to_string(),
            OP_PUSH => "PUSH".to_string(),
            OP_POP => "POP".to_string(),
            OP_JMP => "JMP".to_string(),
            OP_JZ => "JZ".to_string(),
            OP_JNZ => "JNZ".to_string(),
            OP_HALT => "HALT".to_string(),
            _ => "UNKNOWN".to_string(),
        }
    }

    fn execute(&mut self, instruction: u8) -> bool {
        match instruction {
            OP_NOP => {
                // No operation
                println!("NOP");
            }
            OP_HALT => {
                println!("HALT");
                return false;
            }
            OP_LDI => {
                let ra = self.fetch();
                let value = self.fetch();
                self.registers[ra as usize] = value;
                println!("LDI R{}, {}", ra, value);
                self.update_flags(value);
            }
            OP_MV => {
                let ra = self.fetch();
                let rb = self.fetch();
                self.registers[ra as usize] = self.registers[rb as usize];
                println!("MV R{}, R{}", ra, rb);
                self.update_flags(rb);
            }
            OP_ADD => {
                let ra = self.fetch();
                let rb = self.fetch();
                let (result, carry) = self.registers[ra as usize].overflowing_add(self.registers[rb as usize]);
                self.registers[0] = result;
                println!("ADD R{}, R{}", ra, rb);
                self.update_flags(result);
                if carry {
                    self.flags |= FLAG_CARRY;
                } else {
                    self.flags &= !FLAG_CARRY;
                }
            }
            OP_SUB => {
                let ra = self.fetch();
                let rb = self.fetch();
                let (result, borrow) = self.registers[ra as usize].overflowing_sub(self.registers[rb as usize]);
                self.registers[0] = result;
                println!("SUB R{}, R{}", ra, rb);
                self.update_flags(result);
                if borrow {
                    self.flags |= FLAG_CARRY;
                } else {
                    self.flags &= !FLAG_CARRY;
                }
            }
            OP_MUL => {
                let ra = self.fetch();
                let rb = self.fetch();
                let result = self.registers[ra as usize] as u16 * self.registers[rb as usize] as u16;
                self.registers[0] = result as u8;
                println!("MUL R{}, R{}", ra, rb);
                self.update_flags(self.registers[ra as usize]);
                if result > 255 {
                    self.flags |= FLAG_OVERFLOW;
                } else {
                    self.flags &= !FLAG_OVERFLOW;
                }
            }
            OP_DIV => {
                let ra = self.fetch();
                let rb = self.fetch();
                if self.registers[rb as usize] != 0 {
                    let result = self.registers[ra as usize] / self.registers[rb as usize];
                    self.registers[0] = result;
                    println!("DIV R{}, R{}", ra, rb);
                    self.update_flags(result);
                } else {
                    // Division by zero: set overflow flag
                    self.flags |= FLAG_OVERFLOW;
                    println!("Division by zero!");
                }
            }
            OP_AND => {
                let ra = self.fetch();
                let rb = self.fetch();
                let result  = self.registers[ra as usize] & self.registers[rb as usize];
                self.registers[0] = result;
                println!("AND R{}, R{}", ra, rb);
                self.update_flags(self.registers[ra as usize]);
            }
            OP_OR => {
                let ra = self.fetch();
                let rb = self.fetch();
                let result  = self.registers[ra as usize] | self.registers[rb as usize];
                self.registers[0] = result;
                println!("OR R{}, R{}", ra, rb);
                self.update_flags(self.registers[ra as usize]);
            }
            OP_XOR => {
                let ra = self.fetch();
                let rb = self.fetch();
                let result  = self.registers[ra as usize] ^ self.registers[rb as usize];
                self.registers[0] = result;
                println!("XOR R{}, R{}", ra, rb);
                self.update_flags(self.registers[ra as usize]);
            }
            OP_SHL => {
                let ra = self.fetch();
                let result = self.registers[ra as usize] << 1;
                self.registers[ra as usize] = result;
                println!("SHL R{}", ra);
                self.update_flags(result);
            }
            OP_SHR => {
                let ra = self.fetch();
                let result = self.registers[ra as usize] >> 1;
                self.registers[ra as usize] = result;
                println!("SHR R{}", ra);
                self.update_flags(result);
            }
            OP_PUSH => {
                let ra = self.fetch();
                self.sp = self.sp.wrapping_sub(1);
                self.memory[self.sp as usize] = self.registers[ra as usize];
                println!("PUSH R{}", ra);
            }
            OP_POP => {
                let ra = self.fetch();
                self.registers[ra as usize] = self.memory[self.sp as usize];
                self.sp = self.sp.wrapping_add(1);
                println!("POP R{}", ra);
                self.update_flags(self.registers[ra as usize]);
            }
            OP_JMP => {
                let addr = self.fetch();
                self.pc = addr;
                println!("JMP to 0x{:02X}", addr);
            }
            OP_JZ => {
                let addr = self.fetch();
                if self.flags & FLAG_ZERO != 0 {
                    self.pc = addr;
                    println!("JZ to 0x{:02X}", addr);
                } else {
                    println!("Skipping JZ");
                }
            }
            OP_JNZ => {
                let addr = self.fetch();
                if self.flags & FLAG_ZERO == 0 {
                    self.pc = addr;
                    println!("JNZ to 0x{:02X}", addr);
                } else {
                    println!("Skipping JNZ");
                }
            }
            _ => {
                println!("Unknown opcode: 0x{:02X}", instruction);
                return false;
            }
        }
        true
    }

    fn update_flags(&mut self, result: u8) {
        if result == 0 {
            self.flags |= FLAG_ZERO;
        } else {
            self.flags &= !FLAG_ZERO;
        }

        if result & 0x80 != 0 {
            self.flags |= FLAG_NEGATIVE;
        } else {
            self.flags &= !FLAG_NEGATIVE;
        }
    }

    fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            self.decode(instruction);

            if !self.execute(instruction) {
                break;
            }
        }
    }
}

impl fmt::Display for CC8R {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Register: {:?}", self.registers)?;
        writeln!(f, "Flag: 0x{:02X}", self.flags)?;
        writeln!(f, "SP: 0x{:02X}", self.sp)?;
        writeln!(f, "PC: 0x{:02X}", self.pc)?;
        Ok(())
    }
}

fn main() {
    let mut cpu = CC8R::new();

    // (5+3)x2 のプログラム
    let program = [
        0x14, 0x01, 0x05, // LDI R1, 5
        0x14, 0x02, 0x03, // LDI R2, 3
        0x14, 0x03, 0x02, // LDI R3, 2
        0x20, 0x01, 0x02, // ADD R1, R2
        0x18, 0x04, 0x00, // MV R4, R0
        0x40, 0x03, 0x04, // MUL R3, R4
        0x10,             // HALT
    ];

    cpu.load_program(&program);
    cpu.run();

    println!("{}", cpu);
}

#[cfg(test)]
mod tests {
    use super::*; // 現在のモジュールをインポート

    #[test]
    fn test_cpu_program() {
        let mut cpu = CC8R::new();

        // (5+3)x2 のプログラム
        let program = [
            0x14, 0x01, 0x05, // LDI R1, 5
            0x14, 0x02, 0x03, // LDI R2, 3
            0x14, 0x03, 0x02, // LDI R3, 2
            0x20, 0x01, 0x02, // ADD R1, R2
            0x18, 0x04, 0x00, // MV R4, R0
            0x40, 0x03, 0x04, // MUL R3, R4
            0x10,             // HALT
        ];

        cpu.load_program(&program);
        cpu.run();
        println!("{}", cpu);

        assert_eq!(cpu.registers[0], 16);
    }
}
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    f: u8,
    sp: u16,
    pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            f: 0x00,
            sp: 0xFFFF,
            pc: 0x0000,
        }
    }
}

pub struct Cpu {
    regs: Registers,
    mem: [u8; 0xFFFF],
    opcode: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: Registers::new(),
            mem: [0; 0xFFFF],
            opcode: 0x00
        }
    }

    pub fn step(&mut self) {
        self.opcode = self.fetch();
        println!("Fetched: 0x{:02X}", self.opcode);
        let decoded = self.decode();
        println!("Decoded: {:?}", decoded);
        println!("Executing.");
    }

    fn fetch(&mut self) -> u8 {
        let code = self.mem[self.regs.pc as usize];
        self.regs.pc += 1;
        code
    }

    fn decode(&mut self) -> (FnMut, Option<u8>, Option<u8>) {
        let co = self.opcode;
        let (page, yyy, zzz) = (co >> 6, co >> 3 & 0b111, co & 0b111);
    }
}

enum Args {
    Number(u16),
    Register(u16),
    Indirect(u16),

}
#[derive(Debug)]
enum Operation {
    Nop,
    Lxi,
    Stax,
    Shld,
    Inx,
    Inr,
    Dcr,
    Rlc,
    Bal,
    Daa,
    Stc,
    Dad,
    Ldax,
    Lda,
    Lhld,
    Dcx,
    Hlt,
    Add,
    Sub,
    Adc,
    Xra,
    Ana,
    Ora,
    Push,
    Adi,
    Sui,
    Ani,
    Ori,
    Di,
    Out,
    Xhtl,
    Jmp,
    Ret,
    Pchl,
    Jpe,
    Xchg,
    Ei,
    Sphl,
}

struct Instruction {
    code: u8,
    operation: Operation,

}

pub fn serialize_code(code: u8) -> Instruction {
    10
}

#[derive(Debug, Clone, Copy)]
struct InstAddr{
    data: u16
}

impl Default for InstAddr{
    fn default() -> Self{
        InstAddr{data: 0}
    }
}
impl InstAddr {
    pub fn from_fields(funct3: u8, funct7: bool, high_bit: bool, low_nibble: u8) -> Self {
        InstAddr{
            data:  ((funct3 as u16) << 6) | ((funct7 as u16) << 5) | ((high_bit as u16) << 4) | (low_nibble as u16)
        }
    }

    pub fn from_fields_b(funct3: u8, funct7: bool, opcode: u8) -> Self{
        InstAddr{
            data:  ((funct3 as u16) << 6) | ((funct7 as u16) << 5) | (opcode as u16)
        }  
    }

    pub fn get_low_nibble(&self) -> u16 {
        ( self.data & 0b000001111 ) >> 0
    }

    pub fn get_high_bit(&self) -> u16 {
        (self.data & 0b000010000) >> 4
    }

    pub fn get_opcode(&self) -> u16 {
        (self.get_high_bit() << 4) | self.get_low_nibble()
    }

    pub fn get_funct7_state(&self) -> u16 {
        ((self.data & 0b000100000) >> 5)
    }

    pub fn get_funct3(&self) -> u16 {
        (self.data & 0b111000000) >> 6
    }

}

type ControlLine = u32;


    const PC_ENC: ControlLine       = 0b00000000000000000000000000000001;
    const PC_LOAD: ControlLine      = 0b00000000000000000000000000000010;
    const PC_OUT: ControlLine       = 0b00000000000000000000000000000100;

    const ALU_INVA: ControlLine     = 0b00000000000000000000000000001000;
    const ALU_INVB: ControlLine     = 0b00000000000000000000000000010000;
    const ALU_OP_1: ControlLine     = 0b00000000000000000000000000100000;
    const ALU_OP_0: ControlLine     = 0b00000000000000000000000001000000;
    const ALU_INVOUT: ControlLine   = 0b00000000000000000000000010000000;
    const ALU_OUT: ControlLine      = 0b00000000000000000000000100000000;

    const REG_OUT: ControlLine      = 0b00000000000000000000001000000000;

    const CNTRL_IMMOUT: ControlLine = 0b00000000000000000000010000000000;

    const IR_ZR: ControlLine       = 0b00000000000000000000100000000000;
    const IR_CEN: ControlLine       = 0b00000000000000000001000000000000;
    const IR_LOAD: ControlLine      = 0b00000000000000000010000000000000;

    const RAM_ENO: ControlLine      = 0b00000000000000000100000000000000;
    const RAM_LOAD: ControlLine     = 0b00000000000000001000000000000000;


#[derive(Debug)]
struct Rom {
    data: [ControlLine; 0b1_000_000_000] // 9 bits of address
}
impl Rom {
    pub fn new() -> Self{
        Rom{
            data: [Default::default(); 0b1_000_000_000]
        }
    }
}
use std::ops::{Index, IndexMut};

impl Index<InstAddr> for Rom {
    type Output = ControlLine;
    fn index(&self, ind: InstAddr) -> &Self::Output {
        &self.data[ind.data as usize]
    }
}

impl IndexMut<InstAddr> for Rom {
    fn index_mut(&mut self, ind: InstAddr) -> &mut Self::Output {
        &mut self.data[ind.data as usize]
    }
}


fn main(){
    let mut r = Rom::new();

    // LOAD control lines
    for addr in 0..0b1_000_000_000 {
        r.data[addr] = PC_ENC | PC_OUT | RAM_ENO | IR_CEN | IR_LOAD
    }
    const OP_IMM: u8 = 0x04;
    for ANY in 0u8 ..= 1u8 {
        r[InstAddr::from_fields_b(0, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | IR_ZR; // ADDI
        r[InstAddr::from_fields_b(0, ANY != 0, OP_IMM)] &= !(PC_ENC | RAM_ENO);
    }
   // println!("{:?}", r);
   // 0x00178793
    use std::io::Write;
    let mut fout = std::fs::File::create("rom.bin").expect("Opening out file!");
    for addr in 0..0b1_000_000_000 {
        let b1 = ((r.data[addr] & 0x000000FF) >> 0) as u8;
        let b2 = ((r.data[addr] & 0x0000FF00) >> 8) as u8;
        let b3 = ((r.data[addr] & 0x00FF0000) >> 8*2) as u8;
        let b4 = ((r.data[addr] & 0xFF000000) >> 8*3) as u8;
        fout.write_all(&[b1, b2, b3, b4]).unwrap();
    }
}
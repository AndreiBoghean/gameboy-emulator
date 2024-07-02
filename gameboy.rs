use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("ello wold :D");

    // let data: Vec<u8> = fs::read("roms/dmg_boot.bin")?;
    let data: Vec<u8> = fs::read("roms/dmg_boot_noNintendo.bin")?;

    let mut A: u8 = 0;
    let mut F: u8 = 0; // note: not individually addressable, opperations usually only deal with AF and never F itself.
    let mut B: u8 = 0;
    let mut C: u8 = 0;
    let mut D: u8 = 0;
    let mut E: u8 = 0;
    let mut H: u8 = 0;
    let mut L: u8 = 0;

    macro_rules! gimme_flag {
        (z) => (F>>7 & 1);
        (n) => (F>>6 & 1);
        (h) => (F>>5 & 1);
        (c) => (F>>4 & 1);
    }
    macro_rules! raise_flag {
        (z) => (F |= 0b10000000);
        (n) => (F |= 0b01000000);
        (h) => (F |= 0b00100000);
        (c) => (F |= 0b00010000);
    }
    macro_rules! lower_flag {
        (z) => (F &= 0b01111111);
        (n) => (F &= 0b10111111);
        (h) => (F &= 0b11011111);
        (c) => (F &= 0b11101111);
    }

    let mut SP: u16 = 0;
    let mut PC: u16 = 0;

    // the stack holds addresses, thus it is the same type as our SP (stack pointer)
    let mut stack: Vec<u16> = Vec::new();

    loop {
        let mut current_instruction: u8 = data[SP as usize];
        print!("{:2X?} : {:2X?} - ", SP, current_instruction);

        match current_instruction {
            0x00 => {
                // used in boot rom; vompleted
                println!("NOP");
            },
            0x10 => {
                println!("STOP");
            },
            0x08 => {
                println!("load from SP");
            },
            0xF3 => {
                println!("DISABLE INTERRUPTS");
            },
            0xCB => {
                // used in boot rom
                println!("PREFIX INSTRUCTION LUL");
                SP += 1
            },
            0x2F => {
                println!("COMPLEMENT ACCUMULATOR");
            },
            0xCD => {
                // used in boot rom; completed
                // adds address of next instruction to stack, and then executes an implicit "JP" i.e. implicitly jumps
                println!("CALL {:4X?} {:4X?}", data[(SP+1) as usize], data[(SP+2) as usize]);

                stack.push(SP);
                // note: immediately contiguous word is lsb and then the one after is the msb of target jump address
                SP = (data[(SP+1) as usize] as u16) + 0x100 * (data[(SP+2) as usize] as u16);
                // SP += 2
                SP -= 1 // minus 1 because of automatic increment at the end of the while loop
            },
            0xCE => {
                // used in boot rom; completed
                println!("ADC literal {:X?}", data[(SP+1) as usize]);

                // execute sum and deal with overflow
                let mut result = A.overflowing_add(data[(SP+1) as usize]);
                if (result.1) { raise_flag!(c); }
                result = result.0.overflowing_add(gimme_flag!(c));
                if (result.1) { raise_flag!(c); }

                // rest of flags
                if (result.0 == 0) { raise_flag!(z) };
                lower_flag!(n);
                if ((A & 0xF0) != (result.0 & 0xF0)) { raise_flag!(h) };

                A = result.0;
                SP += 1;
                // println!("A is now {:X?} | F is now {:X?}", A, F);
            },
            0b00000001 | 0b00010001 | 0b00100001 | 0b00110001 => { // 0b00xx0001
                // used in boot rom
                println!("load from nn");
                SP += 2
            },
            0b00000011 | 0b00010011 | 0b00100011 | 0b00110011 => { // 0b00xx0011
                // used in boot rom
                println!("increment 16bit");
            },

            0b00000100 | 0b00001100 | 0b00010100 | 0b00011100 | 0b00100100 | 0b00101100 | 0b00110100 | 0b00111100 => { // 0b00xxx100
                // used in boot rom
                println!("increment 8bit register");
            },

            0b00000101 | 0b00001101 | 0b00010101 | 0b00011101 | 0b00100101 | 0b00101101 | 0b00110101 | 0b00111101 => { // 0b00xxx101
                // used in boot rom
                println!("decrement 8bit");
            },
            0b00000110 | 0b00001110 | 0b00010110 | 0b00011110 | 0b00100110 | 0b00101110 | 0b00110110 | 0b00111110 => { // 0b00xxx110
                // used in boot rom
                println!("load data n to 8bit reg");
                SP += 1
            },

            0b00001111 => {
                println!("ROTATE RIGHT circular");
            },
            0b00010111 => {
                // used in boot rom
                println!("ROTATE LEFT");
            },
            0b00011000 => {
                // used in boot rom
                println!("uncond jump");
                SP += 1
            },

            // 00xxx010 (maybe?)
            0b00000010 | 0b00010010 | 0b00100010 | 0b00110010 | 0b00001010 | 0b00011010 | 0b00101010 | 0b00111010 => {
                // used in boot rom
                println!("load data to/from RR addr to/from Areg");
                /*
                // 0b00000010 => // 00xxx010 (maybe?)
                println!("load data to BC addr from Areg");
                // 0b00010010 => // 00xxx010 (maybe?)
                println!("load data to DE addr from Areg");
                // 0b00100010 => // 00xxx010 (maybe?)
                println!("load data to HL+ addr from Areg");
                // 0b00110010 => // 00xxx010 (maybe?)
                println!("load data to HL- addr from Areg");
                // 0b00001010 => // 00xxx010 (maybe?)
                println!("load data from BC addr to Areg");
                // 0b00011010 => // 00xxx010 (maybe?)
                println!("load data from DE addr to Areg");
                // 0b00101010 => // 00xxx010 (maybe?)
                println!("load data from HL+ addr to Areg");
                // 0b00111010 => // 00xxx010 (maybe?)
                println!("load data from HL- addr to Areg");
                */
            },

            0b00100000 | 0b00101000 | 0b00110000 | 0b00111000 => { // 0b001xx000
                // used in boot rom
                println!("cond relative jump");
                SP += 1
            },
            0b00001001 | 0b00011001 | 0b00101001 | 0b00111001 => { // 0b00xx1001
                println!("add with 16bit & store");
            },
            0x76 => {
                println!("HALT");
            },
            0x40..=0x7F => { // matching anything under 0b01xxxyyy for a load instruction from register yyy to xxx (?)
                // used in boot rom
                println!("load");
            },
            0x80..=0x87 => {
                // used in boot rom
                println!("ADD");
            },
            0x88..=0x8F => {
                println!("ADC");
            },
            0x90..=0x97 => {
                // used in boot rom
                println!("SUB");
            },
            0x98..=0x9F => {
                println!("SBC");
            },
            0xA0..=0xA7 => {
                // used in boot rom
                println!("AND");
            },
            0xA8..=0xAF => {
                // used in boot rom
                println!("XOR");
            },
            0xB0..=0xB7 => { // not used rn
                println!("OR");
            },
            0xB8..=0xBF => {
                // used in boot rom
                println!("CP");
            },
            0b11000000 | 0b11001000 | 0b11010000 | 0b11011000 => { // 0b110xx000
                println!("ret");
            },

            0b11000001 | 0b11010001 | 0b11100001 | 0b11110001 => { // 0b11xx0001
                // used in boot rom
                println!("pop stack to rr reg");
            },

            0b11000010 | 0b11001010 | 0b11010010 | 0b11011010 => { // 0b110xx010
                println!("conditional jump");
                SP += 2
            },

            0b11100000 | 0b11110000 => { // 0b111x0000 (?)
                // used in boot rom
                println!("load from accumulator direct");
                SP += 1
            },

            0b11100010 | 0b11110010  => { // 0b111x0010 (?)
                // used in boot rom
                println!("load from accumulator indirect");
                // println!("load to accumulator indirect");
            },

            0b11101010 | 0b11111010  => { // 0b111x1010 (?)
                // used in boot rom
                println!("load to/from A to/from 16bit nn");
                // println!("load to A from 16bit nn");
                SP += 2
            },

            0b11111001 => {
                println!("load SP from HL");
            },

            0b11111011 => {
                println!("schedule to enable interrupts after next cycle");
            },

            0b11111110 => {
                // used in boot rom
                println!("compare immediate");
                SP += 1
            },

            0b11001001 => {
                // used in boot rom
                println!("unconditional ret");
            },
            0b11000101 | 0b11010101 | 0b11100101 | 0b11110101 => { // 0b11xx0101
                // used in boot rom
                println!("push to stack");
            },
            0b11000111 | 0b11001111 | 0b11010111 | 0b11011111 | 0b11100111 | 0b11101111 | 0b11110111 | 0b11111111 => { // 0b11xxx111
                println!("uncond func call");
            },
            _ => {
                // used in boot rom, with 0xED
                println!("anything, {:X?}, {:?}", current_instruction, current_instruction);
            }
        }

        SP += 1;
    }
}

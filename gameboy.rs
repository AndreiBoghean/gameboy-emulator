use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("ello wold :D");

    // let data: Vec<u8> = fs::read("roms/dmg_boot.bin")?;
    let data: Vec<u8> = fs::read("roms/dmg_boot_noNintendo.bin")?;

    let mut AF: u16 = 0;
    let mut BC: u16 = 0;
    let mut DE: u16 = 0;
    let mut HL: u16 = 0;
    let mut SP: u16 = 0;
    let mut PC: u16 = 0;

    loop {
        let mut current_instruction: u8 = data[SP as usize];

        match current_instruction {
            0x00 => {
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
                println!("PREFIX INSTRUCTION LUL");
            },
            0x2F => {
                println!("COMPLEMENT ACCUMULATOR");
            },
            0xCD => {
                println!("function call moment");
            },
            0xCE => {
                println!("add with carry");
            },
            0b00000001 => { // 0b00xx0001
                println!("load from nn instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00010001 => { // 0b00xx0001
                println!("load from nn instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00100001 => { // 0b00xx0001
                println!("load from nn instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00110001 => { // 0b00xx0001
                println!("load from nn instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00000011 => { // 0b00xx0011
                println!("increment 16bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00010011 => { // 0b00xx0011
                println!("increment 16bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00100011 => { // 0b00xx0011
                println!("increment 16bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00110011 => { // 0b00xx0011
                println!("increment 16bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },

            0b00000100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00001100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00010100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00011100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00100100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00101100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00110100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00111100 => { // 0b00xxx100
                println!("increment 8bit register instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },

            0b00000101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00001101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00010101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00011101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00100101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00101101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00110101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00111101 => { // 0b00xxx101
                println!("decrement 8bit instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00000110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00001110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00010110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00011110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00100110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00101110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00110110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00111110 => { // 0b00xxx110
                println!("load data n to 8bit reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },

            0b00001111 => {
                println!("ROTATE RIGHT circular instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00010111 => {
                println!("ROTATE LEFT instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00011000 => {
                println!("uncond jump instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },

            0b00000010 => { // 00xxx010 (maybe?)
                println!("load data to BC addr from Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00010010 => { // 00xxx010 (maybe?)
                println!("load data to DE addr from Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00100010 => { // 00xxx010 (maybe?)
                println!("load data to HL+ addr from Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00110010 => { // 00xxx010 (maybe?)
                println!("load data to HL- addr from Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00001010 => { // 00xxx010 (maybe?)
                println!("load data from BC addr to Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00011010 => { // 00xxx010 (maybe?)
                println!("load data from DE addr to Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00101010 => { // 00xxx010 (maybe?)
                println!("load data from HL+ addr to Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00111010 => { // 00xxx010 (maybe?)
                println!("load data from HL- addr to Areg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },

            0b00100000 => { // 0b001xx000
                println!("cond relative jump instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00101000 => { // 0b001xx000
                println!("cond relative jump instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00110000 => { // 0b001xx000
                println!("cond relative jump instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00111000 => { // 0b001xx000
                println!("cond relative jump instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00001001 => { // 0b00xx1001
                println!("add with 16bit & store instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00011001 => { // 0b00xx1001
                println!("add with 16bit & store instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00101001 => { // 0b00xx1001
                println!("add with 16bit & store instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b00111001 => { // 0b00xx1001
                println!("add with 16bit & store instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0x76 => {
                println!("HALT");
            },
            0x40..=0x7F => { // matching anything under 0b01xxxyyy for a load instruction from register yyy to xxx (?)
                println!("load instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0x80..=0x87 => {
                println!("ADD instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0x88..=0x8F => {
                println!("ADC instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0x90..=0x97 => {
                println!("SUB instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0x98..=0x9F => {
                println!("SBC instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0xA0..=0xA7 => {
                println!("AND instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0xA8..=0xAF => {
                println!("XOR instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0xB0..=0xB7 => { // not used rn
                println!("OR instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0xB8..=0xBF => {
                println!("CP instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11000000 => { // 0b110xx000
                println!("ret instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11001000 => { // 0b110xx000
                println!("ret instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11010000 => { // 0b110xx000
                println!("ret instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11011000 => { // 0b110xx000
                println!("ret instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },

            0b11000001 => { // 0b11xx0001
                println!("pop stack to rr reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11010001 => { // 0b11xx0001
                println!("pop stack to rr reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11100001 => { // 0b11xx0001
                println!("pop stack to rr reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11110001 => { // 0b11xx0001
                println!("pop stack to rr reg instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },

            0b11000010 => { // 0b110xx010
                println!("conditional jump instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },
            0b11001010 => { // 0b110xx010
                println!("conditional jump instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },
            0b11010010 => { // 0b110xx010
                println!("conditional jump instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },
            0b11011010 => { // 0b110xx010
                println!("conditional jump instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },

            0b11100000 => { // 0b111x0000 (?)
                println!("load from accumulator direct instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },
            0b11110000 => { // 0b111x0000 (?)
                println!("load from accumulator direct instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },

            0b11100010 => { // 0b111x0010 (?)
                println!("load from accumulator indirect instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },
            0b11110010 => { // 0b111x0010 (?)
                println!("load to accumulator indirect instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },

            0b11101010 => { // 0b111x1010 (?)
                println!("load from A to 16bit nn instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },
            0b11111010 => { // 0b111x1010 (?)
                println!("load to A from 16bit nn instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },

            0b11111001 => {
                println!("load SP from HL instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },

            0b11111011 => {
                println!("schedule to enable interrupts after next cycle instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },

            0b11111110 => {
                println!("compare immediate instruction moment for {:x?}, {:?}", current_instruction, current_instruction);
            },

            0b11001001 => {
                println!("unconditional ret instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11000101 => { // 0b11xx0101
                println!("push to stack instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11010101 => { // 0b11xx0101
                println!("push to stack instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11100101 => { // 0b11xx0101
                println!("push to stack instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11110101 => { // 0b11xx0101
                println!("push to stack instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11000111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11001111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11010111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11011111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11100111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11101111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11110111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            0b11111111 => { // 0b11xxx111
                println!("uncond func call instruction moment for {:X?}, {:?}", current_instruction, current_instruction);
            },
            _ => {
                println!("anything, {:X?}, {:?}", current_instruction, current_instruction);
            }
        }

        SP += 1;
    }
}

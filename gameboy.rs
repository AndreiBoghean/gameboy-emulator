use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("ello wold :D");

    let data: Vec<u8> = fs::read("roms/dmg_boot.bin")?;

    let mut AF: u16 = 0;
    let mut BC: u16 = 0;
    let mut DE: u16 = 0;
    let mut HL: u16 = 0;
    let mut SP: u16 = 0;
    let mut PC: u16 = 0;

    loop {
        let mut current_instruction: u8 = data[SP as usize];

        match current_instruction {
            0 => {
                println!("0");
            },
            1 => {
                println!("1");
            },
            2 => {
                println!("2");
            },
            3 => {
                println!("3");
            },
            4 => {
                println!("4");
            },
            5 => {
                println!("5");
            },
            6 => {
                println!("6");
            },
            7 => {
                println!("7");
            },
            8 => {
                println!("8");
            },
            9 => {
                println!("9");
            },
            10 => {
                println!("10");
            },
            11 => {
                println!("11");
            },
            12 => {
                println!("12");
            },
            13 => {
                println!("13");
            },
            14 => {
                println!("14");
            },
            15 => {
                println!("15");
            },
            _ => println!("anything, {:X?}, {:?}", current_instruction, current_instruction)
        }

        SP += 1;
    }
}

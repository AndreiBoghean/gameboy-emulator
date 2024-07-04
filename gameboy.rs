use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("ello wold :D");

    // let data: Vec<u8> = fs::read("roms/dmg_boot.bin")?;
    let data: Vec<u8> = fs::read("roms/dmg_boot_noNintendo.bin")?;

    // note: gameboi instructions use both "AF" and individual "A" and "F" registers. since there
    // are more 8bit registers than 16bit, I decided to define the 8bit ones and combine to form
    // the 16bit ones, instead of the other way around. this was done specifically because it would
    // mean less work combining/splitting registers.
    let mut A: u8 = 0;
    let mut F: u8 = 0; // note: not individually addressable, opperations usually only deal with AF and never F itself.
    let mut B: u8 = 0;
    let mut C: u8 = 0;
    let mut D: u8 = 0;
    let mut E: u8 = 0;
    let mut H: u8 = 0;
    let mut L: u8 = 0;

    let mut SP: u16 = 0;
    let mut PC: u16 = 0;

    macro_rules! set_16bit {
        ( $index:expr, $value_lsb:expr, $value_msb:expr ) => {
            match $index
            {
                0b00 => {
                    // println!("setting BC.");
                    B = $value_lsb;
                    C = $value_msb;
                }
                0b01 => {
                    // println!("setting DE.");
                    D = $value_lsb;
                    E = $value_msb;
                }
                0b10 => {
                    // println!("setting HL.");
                    H = $value_lsb;
                    L = $value_msb;
                }
                0b11 => {
                    // println!("setting SP.");
                    SP = (($value_msb as u16) << 8 | $value_lsb as u16);
                }
                _ => { println!("panik!"); }
            }
        };
    }

    /*
    let mut set_16bit_whole = |index: u8, value: u8|
    {
        match index
        {
            0b00 => {
                A = value >> 8;
                F = value & 0x00FF;
            }
            0b01 => {
                B = value >> 8;
                C = value & 0x00FF;
            }
            0b10 => {
                D = value >> 8;
                E = value & 0x00FF;
            }
            0b11 => {
                H = value >> 8;
                L = value & 0x00FF;
            }
            _ => { println!("panik!"); }
        }
    };
    */

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


    // this thing creates a u16 var "AF_", and 2 u8 pointers "A_" and "F_" which point to the Hi
    // and Lo sections of the u16 thing, respectively. for now, other solutions are being seeked
    // since I do not want to make the whole main functiom unsafe (which would be giving up)
    /*
    let mut AF_: u16 = 0x1234;
    let F_: *mut u8 = (&mut AF_ as *mut u16) as *mut u8;
    unsafe {
        let A_: *mut u8 = F_.offset(1);
        println!("AF_:{:X?}", AF_);
        println!("A_:{:X?} F_:{:X?}", *A_, *F_);
        *A_ = 0x69;
        *F_ = 0x88;
        println!("A_:{:X?} F_:{:X?}", *A_, *F_);
        println!("AF_:{:X?}", AF_);
    }
    */

    // let BC: *mut u16 =
    // let DE: *mut u16 =
    // let HL: *mut u16 =

    let mut stack: Vec<u16> = Vec::new();

    loop {
        let mut current_instruction: u8 = data[PC as usize];
        print!("{:2X?} : {:2X?} - ", PC, current_instruction);

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
                PC += 1
            },
            0x2F => {
                println!("COMPLEMENT ACCUMULATOR");
            },
            0xCD => {
                // used in boot rom; completed
                // adds address of next instruction to stack, and then executes an implicit "JP" i.e. implicitly jumps
                println!("CALL {:4X?} {:4X?}", data[(PC+1) as usize], data[(PC+2) as usize]);

                stack.push(PC);
                // note: immediately contiguous word is lsb and then the one after is the msb of target jump address
                PC = (data[(PC+1) as usize] as u16) + 0x100 * (data[(PC+2) as usize] as u16);
                // PC += 2
                PC -= 1 // minus 1 because of automatic increment at the end of the while loop
            },
            0xCE => {
                // used in boot rom; completed
                println!("ADC literal {:X?}", data[(PC+1) as usize]);

                // execute sum and deal with overflow
                let mut result = A.overflowing_add(data[(PC+1) as usize]);
                if result.1 { raise_flag!(c); }
                result = result.0.overflowing_add(gimme_flag!(c));
                if result.1 { raise_flag!(c); }

                // rest of flags
                if result.0 == 0 { raise_flag!(z) };
                lower_flag!(n);
                if (A & 0xF0 != result.0 & 0xF0) { raise_flag!(h) };

                A = result.0;
                PC += 1;
                // println!("A is now {:X?} | F is now {:X?}", A, F);
            },
            0b00000001 | 0b00010001 | 0b00100001 | 0b00110001 => { // 0b00xx0001
                // used in boot rom; completed
                let selected_register = (current_instruction >> 4) & 0b11;
                let byte1 = data[(PC+1) as usize];
                let byte2 = data[(PC+2) as usize];
                println!("load nn {:X?} {:X?} {:X?}", selected_register, byte1, byte2);

                set_16bit!(selected_register, byte1, byte2);

                PC += 2
            },
            0b00000011 | 0b00010011 | 0b00100011 | 0b00110011 => { // 0b00xx0011
                // used in boot rom; complete
                let selected_register = (current_instruction >> 4) & 0b11;
                println!("INC 16bit {:X?}", selected_register);

                fn increment (A: &mut u8, B: &mut u8)
                {
                    let result = (*A).overflowing_add(1);
                    *A = result.0;

                    if result.1 {
                        *B = (*B).overflowing_add(1).0;
                    }
                }

                match selected_register
                {
                    0b00 => {
                        // println!("setting BC.");
                        increment(&mut B, &mut C);
                    }
                    0b01 => {
                        // println!("setting DE.");
                        increment(&mut D, &mut E);
                    }
                    0b10 => {
                        // println!("setting HL.");
                        increment(&mut H, &mut L);
                    }
                    0b11 => {
                        // println!("setting SP.");
                        SP += 1;
                    }
                    _ => { println!("panik!"); }
                }
            },

            0b00000100 | 0b00001100 | 0b00010100 | 0b00011100 | 0b00100100 | 0b00101100 | 0b00110100 | 0b00111100 => { // 0b00xxx100
                // used in boot rom; completed
                let selected_register = (current_instruction >> 3) & 0b111;
                println!("INC 8bit {:X?}", selected_register);

                macro_rules! increment {
                    ($A:expr) =>
                    {
                        {
                            let result = $A.overflowing_add(1);

                            if result.0 == 0 { raise_flag!(z); } else { lower_flag!(z); }
                            if result.1 {  raise_flag!(c); } else { lower_flag!(c); }
                            if (result.0 == 0b00010000) {raise_flag!(h); } else { lower_flag!(h); }
                            lower_flag!(n);

                            A = result.0;
                        }
                    }
                }

                match selected_register
                {
                    0b00 => increment!(B),
                    0b00 => increment!(C),
                    0b01 => increment!(D),
                    0b01 => increment!(E),
                    0b10 => increment!(H),
                    0b10 => increment!(L),
                    0b11 => increment!(data[ ((H as u16) << 4 + L as u16) as usize]),
                    0b11 => increment!(A),
                    _ => println!("panik!")
                }
            },

            0b00000101 | 0b00001101 | 0b00010101 | 0b00011101 | 0b00100101 | 0b00101101 | 0b00110101 | 0b00111101 => { // 0b00xxx101
                // used in boot rom; complete

                let selected_register = (current_instruction >> 3) & 0b111;
                println!("DEC 8bit {:X?}", selected_register);

                macro_rules! decrement {
                    ($A:expr) =>
                    {
                        {
                            let result = $A.overflowing_add(0b11111110);

                            if result.0 == 0 { raise_flag!(z); } else { lower_flag!(z); }
                            if result.0 == 0b11111111 { raise_flag!(c);}  else { lower_flag!(c); }
                            if (result.0 == 0b00001111) {raise_flag!(h); } else { lower_flag!(h); }
                            raise_flag!(n);

                            A = result.0;
                        }
                    }
                }

                match selected_register
                {
                    0b00 => decrement!(B),
                    0b00 => decrement!(C),
                    0b01 => decrement!(D),
                    0b01 => decrement!(E),
                    0b10 => decrement!(H),
                    0b10 => decrement!(L),
                    0b11 => decrement!(data[ ((H as u16) << 4 + L as u16) as usize]),
                    0b11 => decrement!(A),
                    _ => println!("panik!")
                }
            },
            0b00000110 | 0b00001110 | 0b00010110 | 0b00011110 | 0b00100110 | 0b00101110 | 0b00110110 | 0b00111110 => { // 0b00xxx110
                // used in boot rom
                println!("load data n to 8bit reg");
                PC += 1
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
                PC += 1
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
                PC += 1
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
                PC += 2
            },

            0b11100000 | 0b11110000 => { // 0b111x0000 (?)
                // used in boot rom
                println!("load from accumulator direct");
                PC += 1
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
                PC += 2
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
                PC += 1
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

        PC += 1;
    }
}

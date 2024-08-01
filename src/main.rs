use std::fs;
use std::thread;
use std::time::Duration;

use std::sync::{Arc, Mutex};

extern crate winit;
use winit::{
    event_loop::{EventLoop, ActiveEventLoop, ControlFlow},
    window::{Window, WindowId},
    application::ApplicationHandler,
    event::{WindowEvent}
};

extern crate pixels;
use pixels::{Pixels, SurfaceTexture};


#[allow(non_snake_case)]

#[derive(Default)]
struct App {
    window: Option<Window>,
    data: Arc<Mutex<Vec<u8>>>,
    pixels: Option<Pixels>
}

// note: to enable scroll_debug feature, which shows the full 256x256 internal screen, in addition to highlighting the smaller 144x160 LCD window
// run with "cargo run --features scroll_debug"

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("we've resumed from something");
        // create winit window
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());

        // setup 'pixels' with aforementioned window
        let window = self.window.as_ref().unwrap();
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

        #[cfg(feature = "scroll_debug")]
        { self.pixels = Some(Pixels::new(256, 256, surface_texture).unwrap()); println!("scroll de bug!!!"); }
        #[cfg(not(feature = "scroll_debug"))]
        { self.pixels = Some(Pixels::new(144, 160, surface_texture).unwrap()); }


        // Clear the pixel buffer
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x00; // R
            pixel[1] = 0x00; // G
            pixel[2] = 0x99; // B
            pixel[3] = 0xff; // A
        }

        // Draw it to the `SurfaceTexture`
        self.pixels.as_mut().unwrap().render();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {

        // wait for the program to stabilise
        // thread::sleep(Duration::from_millis(5000));
        let mut data = self.data.lock().unwrap();

        // println!("tilemap:\n{:X?}", &data[0x9800..0x9C00]);
        // println!("tilemap:\n{:X?}", &data[0x8000..0x8200]);
        // println!("tilemap:\n{:X?}", &data[0x107..0x907]);
        // thread::sleep(Duration::from_millis(100000000));

        let tilemap = &data[0x9800..0x9C00];
        let tiledata = &data[0x8000..0x8800];
        // println!("tiledata: {:X?}", tiledata);

        let mut frame: Vec<u8> = Vec::new();
        frame.resize(4*256*256, 0);

        // for i in 0..tilemap.len() {
        for i in 0..tilemap.len() {
            let tile_id: u16 = tilemap[i] as u16;
            if true || tile_id == 0x19 {
                let tile_data = &tiledata[(tile_id*16) as usize .. ((tile_id+1)*16) as usize];
                // println!("I:{} tileID:{:X?} tile_data:{:X?}", i, tile_id, tile_data);

                let col: usize = i % 32 as usize;
                let row: usize = (i / 32) as u32 as usize;

                for tile_y in 0..8 {
                    let tile_row_vec = &tile_data[tile_y*2..(tile_y+1)*2];
                    // let tile_row: u16 = ((tile_row_vec[0] as u16) << 8) | (tile_row_vec[1] as u16);

                    let mut binA = tile_row_vec[0] as u16;
                    let mut binB = tile_row_vec[0] as u16;
                    // println!("binA:{}", format!("{binA:#b}"));
                    // println!("binB:{}", format!("{binB:#b}"));
                    for b_p in 0..8 {

                        binA = ((binA & (0xFFFF << 7-b_p)) << 1) | (binA & 0b01111111 >> (b_p));
                        binB = ((binB & (0xFFFF << 7-b_p)) << 1) | (binB & 0b01111111 >> (b_p));
                        // println!("bp:{} binA:{}", b_p, format!("{binA:#b}"));
                        // println!("bp:{} binB:{}", b_p, format!("{binB:#b}"));
                    }

                    binB = binB >> 1;
                    let tile_row: u16 = binA | binB;

                    for b_i in 0..8 {
                        let colour_id = (tile_row >> (14-b_i*2)) & 0b11;
                        // print!("{}|", colour_id);
                        frame[(row*(256*8)+col*8 + tile_y*256+b_i)*4+0] = (0xff * colour_id/4) as u8;
                        frame[(row*(256*8)+col*8 + tile_y*256+b_i)*4+1] = (0xff * colour_id/4) as u8;
                        frame[(row*(256*8)+col*8 + tile_y*256+b_i)*4+2] = (0xff * colour_id/4) as u8;
                        frame[(row*(256*8)+col*8 + tile_y*256+b_i)*4+3] = 0xff;
                    }
                    // println!("");
                }
            }
        }


        // TODO: replace these with the actual named labels for hardware flags. (once we fix those).
        // let start_x: u32 = data[0xFF43] as u32;
        // let start_y: u32 = data[0xFF42] as u32;
        let mut start_x: i32 = data[0xFF43] as i32;
        let mut start_y: i32 = data[0xFF42] as i32;
        data[0xFF42] = data[0xFF42].overflowing_add(5).0;

        /*
        println!("start_y:{}", start_y);
        if start_y <= 4 {
            data[0xFF43] = data[0xFF43].overflowing_add(5).0;
        }
        */
        
        // Clear the pixel buffer
        let real_frame = self.pixels.as_mut().unwrap().frame_mut();

        #[cfg(feature = "scroll_debug")]
        {
            for i in 0..256*256 {
                real_frame[i*4+0] = 0x00;
                real_frame[i*4+1] = 0x00;
                real_frame[i*4+2] = 0xff;
                real_frame[i*4+3] = 0xff;
            }
        }

        for i in 0..144*160 {
            // how many pixels from the left and from the top is the current pixel we're rending? (current pixel dictated by i)
            let viewport_x: i32 = i % 144;
            let viewport_y: i32 = (i / 144) as usize as i32;

            // what is the pixel number of the top left corner of the 256x256 canvas, accounting for offsets?
            let mut frame_offset: i32 = ((start_x+start_y*256));

            // what is the pixel number for the last pixel in the current row?
            let frame_row_end = ((viewport_y+1)*256);

            // what offset do we need to use in order to properly "wrap around"?
            let mut overflow_offset_x: i32 = 0;
            let mut overflow_offset_y: i32 = 0;
            // if frame_offset+(viewport_x+viewport_y*256) >= frame_row_end { overflow_offset_x -= 256; }
            if (overflow_offset_x+overflow_offset_y+frame_offset+(viewport_x+viewport_y*256)) >= frame_row_end { overflow_offset_x -= 256; }
            if (overflow_offset_x+overflow_offset_y+frame_offset+(viewport_x+viewport_y*256)) >= 256*256 { overflow_offset_y -= 256*256; }

            // println!("{} >= {}", frame_offset+(viewport_x+viewport_y*256), frame_row_end);

            let frame_i = overflow_offset_x+overflow_offset_y+frame_offset+(viewport_x+viewport_y*256);
            // println!("i:{} i^:{} sX:{} sY:{} overX:{} overY:{} frame:{} sum:{}",
            //     i, 144*160, start_x, start_y, overflow_offset_x, overflow_offset_y, frame_offset, frame_i );


            #[cfg(feature = "scroll_debug")]
            {
                // code for rendering a small window in a full 256*256 "Pixels" object
                real_frame[(frame_i*4+0) as usize] = frame[(frame_i*4+0) as usize];
                real_frame[(frame_i*4+1) as usize] = frame[(frame_i*4+1) as usize];
                real_frame[(frame_i*4+2) as usize] = frame[(frame_i*4+2) as usize];
                real_frame[(frame_i*4+3) as usize] = frame[(frame_i*4+3) as usize];
            }
            #[cfg(not(feature = "scroll_debug"))]
            {
                real_frame[(i*4+0) as usize] = frame[(frame_i*4+0) as usize];
                real_frame[(i*4+1) as usize] = frame[(frame_i*4+1) as usize];
                real_frame[(i*4+2) as usize] = frame[(frame_i*4+2) as usize];
                real_frame[(i*4+3) as usize] = frame[(frame_i*4+3) as usize];
            }
        }

        /*
        for i in 0..frame.len() {
            let row = i % 256;
            let col = (i / 256).floor();

            let sprite = tilemap
        }
        */

        /*
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = (data[0] & 0b11100000); // R
            pixel[1] = ((data[0]) & 0b00011100) << 3; // G
            pixel[2] = ((data[0]) & 0b00000111) << 5; // B
            pixel[3] = 0xff; // A
        }
        */

        // Draw it to the `SurfaceTexture`
        self.pixels.as_mut().unwrap().render();

        // drop data, thus releasing the mutex on it.
        drop(data);

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("ello wold :D");

    // let mut data: Vec<u8> = fs::read("roms/dmg_boot.bin")?;
    
    let mut data: Vec<u8> = fs::read("roms/Tetris.gb")?;
    let mut boot_data: Vec<u8> = fs::read("roms/dmg_boot.bin")?;
    for i in 0..0x100 { data[i] = boot_data[i]; }
    
    data.resize(0xFFFF+1, 0);
    let gimme_data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(data.clone()));

    let data_wanter = gimme_data.clone();
    let data_wanter2 = gimme_data.clone();

    let mut _data = data_wanter.lock().unwrap();
    _data[0xFF44] = 0x90;
    drop(_data);


    // CPU thread
    thread::spawn(move || {
        // some testing code
        /*
        let mut i: u8 = 0;
        loop {
            i += 1;
            let mut data_haver = data_wanter.lock().unwrap();
            data_haver[0] = i*4;
            drop(data_haver);
            thread::sleep(Duration::from_millis(1000));
        }
        */
        let mut i: u8 = 0;




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

        let mut stack: Vec<u8> = Vec::new();
        stack.resize(u16::MAX as usize + 1, 0);
        let mut SP: u16 = u16::MAX;
        let mut PC: u16 = 0;

        let mut IME: bool = false;

        // begin defining a whole lotta hardware registers (note: description | readable/writable | gb models)
        // btw we dont actually need these YET
        /*
        let JOYP = &mut data[0xFF00]; // Joypad | Mixed | All
        let SB = &mut data[0xFF01]; // Serial transfer data | R/W | All
        let SC = &mut data[0xFF02]; // Serial transfer control | R/W | Mixed
        let DIV = &mut data[0xFF04]; // Divider register | R/W | All
        let TIMA = &mut data[0xFF05]; // Timer counter | R/W | All
        let TMA = &mut data[0xFF06]; // Timer modulo | R/W | All
        let TAC = &mut data[0xFF07]; // Timer control | R/W | All
        let IF = &mut data[0xFF0F]; // Interrupt flag | R/W | All
        let NR10 = &mut data[0xFF10]; // Sound channel 1 sweep | R/W | All
        let NR11 = &mut data[0xFF11]; // Sound channel 1 length timer & duty cycle | Mixed | All
        let NR12 = &mut data[0xFF12]; // Sound channel 1 volume & envelope | R/W | All
        let NR13 = &mut data[0xFF13]; // Sound channel 1 period low | W | All
        let NR14 = &mut data[0xFF14]; // Sound channel 1 period high & control | Mixed | All
        let NR21 = &mut data[0xFF16]; // Sound channel 2 length timer & duty cycle | Mixed | All
        let NR22 = &mut data[0xFF17]; // Sound channel 2 volume & envelope | R/W | All
        let NR23 = &mut data[0xFF18]; // Sound channel 2 period low | W | All
        let NR24 = &mut data[0xFF19]; // Sound channel 2 period high & control | Mixed | All
        let NR30 = &mut data[0xFF1A]; // Sound channel 3 DAC enable | R/W | All
        let NR31 = &mut data[0xFF1B]; // Sound channel 3 length timer | W | All
        let NR32 = &mut data[0xFF1C]; // Sound channel 3 output level | R/W | All
        let NR33 = &mut data[0xFF1D]; // Sound channel 3 period low | W | All
        let NR34 = &mut data[0xFF1E]; // Sound channel 3 period high & control | Mixed | All
        let NR41 = &mut data[0xFF20]; // Sound channel 4 length timer | W | All
        let NR42 = &mut data[0xFF21]; // Sound channel 4 volume & envelope | R/W | All
        let NR43 = &mut data[0xFF22]; // Sound channel 4 frequency & randomness | R/W | All
        let NR44 = &mut data[0xFF23]; // Sound channel 4 control | Mixed | All
        let NR50 = &mut data[0xFF24]; // Master volume & VIN panning | R/W | All
        let NR51 = &mut data[0xFF25]; // Sound panning | R/W | All
        let NR52 = &mut data[0xFF26]; // Sound on/off | Mixed | All
        let F3F = &mut data[0xFF3]; // Wave RAM | Storage for one of the sound channels’ waveform | R/W | All
        let LCDC = &mut data[0xFF40]; // LCD control | R/W | All
        let STAT = &mut data[0xFF41]; // LCD status | Mixed | All
        let SCY = &mut data[0xFF42]; // Viewport Y position | R/W | All
        let SCX = &mut data[0xFF43]; // Viewport X position | R/W | All
        let LY = &mut data[0xFF44]; // LCD Y coordinate | R | All
        let LYC = &mut data[0xFF45]; // LY compare | R/W | All
        let DMA = &mut data[0xFF46]; // OAM DMA source address & start | R/W | All
        let BGP = &mut data[0xFF47]; // BG palette data | R/W | DMG
        let OBP0 = &mut data[0xFF48]; // OBJ palette 0 data | R/W | DMG
        let OBP1 = &mut data[0xFF49]; // OBJ palette 1 data | R/W | DMG
        let WY = &mut data[0xFF4A]; // Window Y position | R/W | All
        let WX = &mut data[0xFF4B]; // Window X position plus 7 | R/W | All
        let KEY1 = &mut data[0xFF4D]; // Prepare speed switch | Mixed | CGB
        let VBK = &mut data[0xFF4F]; // VRAM bank | R/W | CGB
        let HDMA1 = &mut data[0xFF51]; // VRAM DMA source high | W | CGB
        let HDMA2 = &mut data[0xFF52]; // VRAM DMA source low | W | CGB
        let HDMA3 = &mut data[0xFF53]; // VRAM DMA destination high | W | CGB
        let HDMA4 = &mut data[0xFF54]; // VRAM DMA destination low | W | CGB
        let HDMA5 = &mut data[0xFF55]; // VRAM DMA length/mode/start | R/W | CGB
        let RP = &mut data[0xFF56]; // Infrared communications port | Mixed | CGB
        let BCPS= &mut data[0xFF68]; //BGPI | Background color palette specification / Background palette index | R/W | CGB
        let BCPD= &mut data[0xFF69]; //BGPD | Background color palette data / Background palette data | R/W | CGB
        let OCPS= &mut data[0xFF6A]; //OBPI | OBJ color palette specification / OBJ palette index | R/W | CGB
        let OCPD= &mut data[0xFF6B]; //OBPD | OBJ color palette data / OBJ palette data | R/W | CGB
        let OPRI = &mut data[0xFF6C]; // Object priority mode | R/W | CGB
        let SVBK = &mut data[0xFF70]; // WRAM bank | R/W | CGB
        let PCM12 = &mut data[0xFF76]; // Audio digital outputs 1 & 2 | R | CGB
        let PCM34 = &mut data[0xFF77]; // Audio digital outputs 3 & 4 | R | CGB
        let IE = &mut data[0xFFFF]; // Interrupt enable | R/W | All
        */
        // end defining a whole lotta hardware registers

        macro_rules! eval_16bit {
            ($A:expr, $B:expr) => { ((($A as u16) << 8) | ($B as u16)) }
        }

        macro_rules! set_16bit {
            ( $index:expr, $value_lsb:expr, $value_msb:expr ) => {
                match $index
                {
                    0b00 => {
                        // println!("setting BC.");
                        B = $value_msb;
                        C = $value_lsb;
                    }
                    0b01 => {
                        // println!("setting DE.");
                        D = $value_msb;
                        E = $value_lsb;
                    }
                    0b10 => {
                        // println!("setting HL.");
                        H = $value_msb;
                        L = $value_lsb;
                    }
                    0b11 => {
                        // println!("setting SP.");
                        SP = (($value_msb as u16) << 8 | $value_lsb as u16);
                    }
                    _ => { println!("panik!"); }
                }
            };
        }

        macro_rules! repr_8bit {
            ($r:expr) => {
                match $r {
                    0b000 => "B",
                    0b001 => "C",
                    0b010 => "D",
                    0b011 => "E",
                    0b100 => "H",
                    0b101 => "L",
                    0b110 => "(HL)",
                    0b111 => "A",
                    _ => todo!()
                }
            }
        }

        macro_rules! repr_16bit {
            (SP, $r:expr) => {
                match $r {
                    0b00 => "BC",
                    0b01 => "DE",
                    0b10 => "HL",
                    0b11 => "SP",
                    _ => todo!()
                }
            };
            (HL, $r:expr) => {
                match $r {
                    0b00 => "BC",
                    0b01 => "DE",
                    0b10 => "HL+",
                    0b11 => "HL-",
                    _ => todo!()
                }
            };
            (AF, $r:expr) => {
                match $r {
                    0b00 => "BC",
                    0b01 => "DE",
                    0b10 => "HL",
                    0b11 => "AF",
                    _ => todo!()
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

        // renderer thread, a.k.a the PPU.

        let mut skip_increment = false;
        loop {
            let mut data = data_wanter.lock().unwrap();
            i = i.overflowing_add(1).0;
            data[0] = i.overflowing_mul(4).0;

            println!("\nS: {:X?}", &stack[u16::MAX as usize -5..]);
            // println!("LCDC: {:X?}", data[0xFF40]); // LCD control | R/W | All
            println!("A:{:X?} F:{:X?} B:{:X?} C:{:X?} D:{:X?} E:{:X?} H:{:X?} L:{:X?} | SP:{:X?}, HL:{:X?} | ZNHC____:{}", A, F, B, C, D, E, H, L, SP, eval_16bit!(H, L), format!("{F:#b}"));

            let current_instruction: u8 = data[PC as usize];
            // print!("S: {:?} | PC: {:2X?} | IR:{:2X?} - ", &stack[u16::MAX as usize -5..], PC, current_instruction);
            print!("PC: {:2X?} | IR:{:2X?} - ", PC, current_instruction);

            match current_instruction {
                0x00 => {
                    // used in boot rom; completed
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
                    // used in boot rom; completed
                    let prefix_instruction = data[(PC+1) as usize];
                    let selected_register = prefix_instruction & 0b111;
                    print!("PREFIX INSTRUCTION LUL {:X?} | ", prefix_instruction);

                    let reg: &mut u8 = match selected_register {
                        0b000 => &mut B,
                        0b001 => &mut C,
                        0b010 => &mut D,
                        0b011 => &mut E,
                        0b100 => &mut H,
                        0b101 => &mut L,
                        0b110 => &mut data[ eval_16bit!(H, L) as usize],
                        0b111 => &mut A,
                        _ => todo!()
                    };

                    match prefix_instruction
                    {
                        0b00010000..=0b00010111 => {
                            let carry: u8 = gimme_flag!(c);
                            println!("ROTATE LEFT {:X?}:{:}", selected_register, repr_8bit!(selected_register));

                            if *reg >> 7 == 1 { raise_flag!(c); } else { lower_flag!(c); }
                            *reg = *reg << 1 | carry;

                            lower_flag!(z);
                            lower_flag!(n);
                            lower_flag!(h);
                        }

                        0x40..=0x7F => {
                            let selected_bit = (prefix_instruction >> 3) & 0b111;
                            let bit = ((*reg) >> selected_bit) & 0b1;
                            println!("TEST BIT {:X?}:{:} {:X?} (bit val is {:X?})", selected_register, repr_8bit!(selected_register), selected_bit, bit);

                            if bit == 0 { raise_flag!(z); } else { lower_flag!(z); }
                            lower_flag!(n);
                            raise_flag!(h); // might seem wierd, but gbdev.io calls for this so this is
                                            // what I do. (I'm ignoring flags as hard as possible until
                                            // they become a problem)
                        },
                        _ => { println!("panik! {:X?}", selected_register); }
                    }

                    PC += 1
                },
                0x2F => {
                    println!("COMPLEMENT ACCUMULATOR");
                },
                0xCD => {
                    // used in boot rom; completed
                    // adds address of next instruction to stack, and then executes an implicit "JP" i.e. implicitly jumps
                    println!("CALL {:4X?} {:4X?}", data[(PC+1) as usize], data[(PC+2) as usize]);

                    stack.push(((PC+3) >> 8) as u8);
                    stack.push((PC+3) as u8);
                    SP -= 2;

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
                    if A & 0xF0 != result.0 & 0xF0 { raise_flag!(h) };

                    A = result.0;
                    PC += 1;
                    // println!("A is now {:X?} | F is now {:X?}", A, F);
                },
                0b00000001 | 0b00010001 | 0b00100001 | 0b00110001 => { // 0b00xx0001
                    // used in boot rom; completed
                    let selected_register = (current_instruction >> 4) & 0b11;
                    let lsb = data[(PC+1) as usize];
                    let msb = data[(PC+2) as usize];
                    println!("LD nn {:X?}:{:} {:X?} {:X?}", selected_register, repr_16bit!(SP, selected_register), msb, lsb);

                    set_16bit!(selected_register, lsb, msb);

                    PC += 2
                },
                0b00000011 | 0b00010011 | 0b00100011 | 0b00110011 => { // 0b00xx0011
                    // used in boot rom; complete
                    let selected_register = (current_instruction >> 4) & 0b11;
                    println!("INC 16bit {:X?}:{:}", selected_register, repr_16bit!(SP, selected_register));

                    fn increment (A: &mut u8, B: &mut u8)
                    {
                        let result = (*B).overflowing_add(1);
                        *B = result.0;

                        if result.1 {
                            *A = (*A).overflowing_add(1).0;
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
                    let selected_register: u8 = (current_instruction >> 3) & 0b111;
                    println!("INC 8bit {:X?}:{:}", selected_register, repr_8bit!(selected_register));

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
                        0b000 => increment!(B),
                        0b001 => increment!(C),
                        0b010 => increment!(D),
                        0b011 => increment!(E),
                        0b100 => increment!(H),
                        0b101 => increment!(L),
                        0b110 => increment!(data[ eval_16bit!(H, L) as usize]),
                        0b111 => increment!(A),
                        _ => println!("panik!")
                    }
                },

                0b00000101 | 0b00001101 | 0b00010101 | 0b00011101 | 0b00100101 | 0b00101101 | 0b00110101 | 0b00111101 => { // 0b00xxx101
                    // used in boot rom; complete

                    let selected_register = (current_instruction >> 3) & 0b111;
                    println!("DEC 8bit {:X?}:{:}", selected_register, repr_8bit!(selected_register));

                    macro_rules! decrement {
                        ($A:expr) =>
                        {
                            {
                                let result = $A.overflowing_add(0xFF);

                                if result.0 == 0 { raise_flag!(z); } else { lower_flag!(z); }
                                if result.0 == 0xFF { raise_flag!(c);}  else { lower_flag!(c); }
                                if (result.0 == 0xF0 ) {raise_flag!(h); } else { lower_flag!(h); }
                                raise_flag!(n);

                                $A = result.0;
                            }
                        }
                    }

                    match selected_register
                    {
                        0b000 => decrement!(B),
                        0b001 => decrement!(C),
                        0b010 => decrement!(D),
                        0b011 => decrement!(E),
                        0b100 => decrement!(H),
                        0b101 => decrement!(L),
                        0b110 => decrement!(data[ eval_16bit!(H, L) as usize]),
                        0b111 => decrement!(A),
                        _ => println!("panik!")
                    }
                },
                0b00000110 | 0b00001110 | 0b00010110 | 0b00011110 | 0b00100110 | 0b00101110 | 0b00110110 | 0b00111110 => { // 0b00xxx110
                    // used in boot rom; completed
                    let selected_register = (current_instruction >> 3) & 0b111;
                    let value = data[(PC+1) as usize];
                    println!("LD n {:X?}:{:} {:X?}", selected_register, repr_8bit!(selected_register), value);

                    match selected_register
                    {
                        0b000 => B = value,
                        0b001 => C = value,
                        0b010 => D = value,
                        0b011 => E = value,
                        0b100 => H = value,
                        0b101 => L = value,
                        0b110 => data[eval_16bit!(H, L) as usize] = value,
                        0b111 => A = value,
                        _ => { println!("panik!"); }
                    }

                    PC += 1
                },

                0b00001111 => {
                    println!("ROTATE RIGHT circular");
                },
                0b00010111 => {
                    // used in boot rom; complete

                    let carry: u8 = gimme_flag!(c);
                    println!("RLA A:{:X?} c:{:}", A, carry);


                    if A >> 7 == 1 { raise_flag!(c); } else { lower_flag!(c); }
                    A = A << 1 | carry;

                    lower_flag!(z);
                    lower_flag!(n);
                    lower_flag!(h);
                },
                0b00011000 => {
                    // used in boot rom(?); complete
                    let offset = data[(PC+1) as usize];
                    println!("JR relative, {:X?}", offset);

                    if (offset >> 7) & 0b1 == 0b1
                    { PC -= (offset ^ 0xFF).overflowing_add(1).0 as u16; }
                    else
                    { PC += offset as u16}
                },

                0b00000010 | 0b00010010 | 0b00100010 | 0b00110010 => { // 00xx0010 
                    // used in boot rom; complete
                    let selected_register = (current_instruction >> 4) & 0b11;
                    println!("LD A to 16bit addr {:X?}:({:})", selected_register, repr_16bit!(HL, selected_register));

                    match selected_register {
                        0b00 => { // 00xxx010 (maybe?)
                            // println!("load data to BC addr from Areg");
                            data[eval_16bit!(B, C) as usize] = A;
                        },
                        0b01 => { // 00xxx010 (maybe?)
                            // println!("load data to DE addr from Areg");
                            data[eval_16bit!(D, E) as usize] = A;
                        },
                        0b10 => { // 00xxx010 (maybe?)
                            // println!("load data to HL+ addr from Areg");
                            data[eval_16bit!(H, L) as usize] = A;

                            let newHL = eval_16bit!(H, L).overflowing_add(1).0;
                            set_16bit!(0b10, newHL as u8, (newHL >> 8) as u8)
                        },
                        0b11 => { // 00xxx010 (maybe?)
                            // println!("load data to HL- addr from Areg");
                            data[eval_16bit!(H, L) as usize] = A;

                            let newHL = eval_16bit!(H, L).overflowing_add(0xFFFF).0;
                            set_16bit!(0b10, newHL as u8, (newHL >> 8) as u8);
                        },
                        _ => { println!("panik! {:X?}", selected_register); }
                    }

                    // println!("tiledat: {:X?}", &data[0x8000..0x8200]);
                    // println!("tilemap: {:X?}", &data[0x9800..0x9C00]);
                    // println!("logo_cart: {:X?}", &data[0x104..0x104+16*3]);
                    // println!("logo_dmg : {:X?}", &data[0xA8..0xA8+16*3]);
                },

                0b00001010 | 0b00011010 | 0b00101010 | 0b00111010 => { // 00xx1010 
                    // used in boot rom; complete
                    let selected_register = (current_instruction >> 4) & 0b11;
                    println!("LD 16bit addr to A {:X?}:({:})", selected_register, repr_16bit!(HL, selected_register));

                    match selected_register {
                        0b00 => { // 00xxx010 (maybe?)
                            // println!("load data from BC addr to Areg");
                            A = data[eval_16bit!(B, C) as usize];
                        },
                        0b01 => { // 00xxx010 (maybe?)
                            // println!("load data from DE addr to Areg");
                            A = data[eval_16bit!(D, E) as usize];
                        },
                        0b10 => { // 00xxx010 (maybe?)
                            // println!("load data from HL+ addr to Areg");
                            A = data[eval_16bit!(H, L) as usize];
                            set_16bit!(0b10, eval_16bit!(H, L).overflowing_add(1).0 as u8, (eval_16bit!(H, L).overflowing_add(1).0 >> 8) as u8)
                        },
                        0b11 => { // 00xxx010 (maybe?)
                            // println!("load data from HL- addr to Areg");
                            A = data[eval_16bit!(H, L) as usize];
                            set_16bit!(0b10, eval_16bit!(H, L).overflowing_add(0xFE).0 as u8, (eval_16bit!(H, L).overflowing_add(0xFE).0 >> 8) as u8)
                        },
                        _ => { println!("panik! {:X?}", selected_register); }
                    }
                },

                0b00100000 | 0b00101000 | 0b00110000 | 0b00111000 => { // 0b001xx000
                    // used in boot rom; completed
                    let e = data[(PC+1) as usize] as u16;
                    let mut value = e;
                    let sign = e >> 7;
                    if sign == 1 { value = (value ^ 0xFF).overflowing_add(1).0; } // two's compliment moment

                    let relevant_flag = match (current_instruction) {
                        0b00100000 => 1-gimme_flag!(z),
                        0b00101000 => gimme_flag!(z),
                        0b00110000 => 1-gimme_flag!(c),
                        0b00111000 => gimme_flag!(c),
                        _ => todo!()
                    };

                    println!("cond relative jump x:{} dest:{} f:{}", e, format!("{e:#b}"), relevant_flag);

                    if relevant_flag != 0 {
                        if sign == 1
                        { PC = PC - value; }
                        else
                        { PC = PC + value; }
                    }

                    PC += 1; // move 1 forward because this is a 2 word instruction
                },
                0b00001001 | 0b00011001 | 0b00101001 | 0b00111001 => { // 0b00xx1001
                    println!("add with 16bit & store");
                    println!("NOT IMPLEMENTED!!!");
                },
                0x76 => {
                    println!("HALT");
                    println!("NOT IMPLEMENTED!!!");
                },
                0x40..=0x7F => { // matching anything under 0b01xxxyyy for a load instruction from register yyy to xxx (?)
                    // used in boot rom; complete
                    let selected_register_A = (current_instruction >> 3) & 0b111;
                    let selected_register_B = (current_instruction) & 0b111;
                    println!("load {:X?}:{:} {:X?}:{:}", selected_register_A, repr_8bit!(selected_register_A), selected_register_B, repr_8bit!(selected_register_B));

                    let reg2 = match selected_register_B {
                        0b000 => B,
                        0b001 => C,
                        0b010 => D,
                        0b011 => E,
                        0b100 => H,
                        0b101 => L,
                        0b110 => data[ eval_16bit!(H, L) as usize],
                        0b111 => A,
                        _ => todo!()
                    };

                    let reg1: &mut u8 = match selected_register_A {
                        0b000 => &mut B,
                        0b001 => &mut C,
                        0b010 => &mut D,
                        0b011 => &mut E,
                        0b100 => &mut H,
                        0b101 => &mut L,
                        0b110 => &mut data[ eval_16bit!(H, L) as usize],
                        0b111 => &mut A,
                        _ => todo!()
                    };
                    println!("reg1:{:X?} reg2:{:X?}", *reg1, reg2);
                    *reg1 = reg2;
                    println!("reg1:{:X?} reg2:{:X?}", *reg1, reg2);
                },
                0x80..=0x87 => {
                    // used in boot rom; complete
                    let selected_register = current_instruction & 0b111;
                    println!("ADD {:X?}:{:}", selected_register, repr_8bit!(selected_register));

                    let reg: &mut u8 = &mut match selected_register {
                        0b000 => B,
                        0b001 => C,
                        0b010 => D,
                        0b011 => E,
                        0b100 => H,
                        0b101 => L,
                        0b110 => data[ eval_16bit!(H, L) as usize],
                        0b111 => A,
                        _ => todo!()
                    };

                    let result = (A).overflowing_add(*reg);

                    if result.0 == 0 { raise_flag!(z); } else { lower_flag!(z); }
                    lower_flag!(n);
                    if (result.0 & 0xF0) != (A & 0xF0) { raise_flag!(h); } else { lower_flag!(h); }
                    if result.1 { raise_flag!(c); } else { lower_flag!(c); }

                    A = result.0;
                },
                0x88..=0x8F => {
                    println!("ADC");
                    println!("NOT IMPLEMENTED!!!");
                },
                0x90..=0x97 => {
                    // used in boot rom; completed
                    let selected_register = current_instruction & 0b111;
                    println!("SUB {:X?}:{:}", selected_register, repr_8bit!(selected_register));

                    let reg: &mut u8 = &mut match selected_register {
                        0b000 => B,
                        0b001 => C,
                        0b010 => D,
                        0b011 => E,
                        0b100 => H,
                        0b101 => L,
                        0b110 => data[ eval_16bit!(H, L) as usize],
                        0b111 => A,
                        _ => todo!()
                    };

                    let result = (A).overflowing_add( (0xFF ^ *reg).overflowing_add(1).0 ); // two's complement moment

                    if result.0 == 0 { raise_flag!(z); } else { lower_flag!(z); }
                    raise_flag!(n);
                    if (result.0 & 0xF0) != (A & 0xF0) { raise_flag!(h); } else { lower_flag!(h); }
                    if result.0 >= A { raise_flag!(c); } else { lower_flag!(c); }

                    A = result.0;

                },
                0x98..=0x9F => {
                    println!("SBC");
                    println!("NOT IMPLEMENTED!!!");
                },
                0xA0..=0xA7 => {
                    // used in boot rom; completed
                    let selected_register = current_instruction & 0b111;
                    println!("AND {:X?}:{:}", selected_register, repr_8bit!(selected_register));

                    match selected_register
                    {
                        0b000 => A &= B,
                        0b001 => A &= C,
                        0b010 => A &= D,
                        0b011 => A &= E,
                        0b100 => A &= H,
                        0b101 => A &= L,
                        0b110 => A &= data[eval_16bit!(H, L) as usize],
                        0b111 => A &= A,
                        _ => { println!("panik! {:X?}", selected_register); }
                    }

                    if A == 0 { raise_flag!(z); } else { lower_flag!(z); }
                    lower_flag!(n);
                    raise_flag!(h);
                    lower_flag!(c);
                },
                0xA8..=0xAF => {
                    // used in boot rom; completed
                    let selected_register = current_instruction & 0b111;
                    println!("XOR, {:X?}:{:}", selected_register, repr_8bit!(selected_register));

                    match selected_register
                    {
                        0b000 => A ^= B,
                        0b001 => A ^= C,
                        0b010 => A ^= D,
                        0b011 => A ^= E,
                        0b100 => A ^= H,
                        0b101 => A ^= L,
                        0b110 => A ^= data[eval_16bit!(H, L) as usize],
                        0b111 => A ^= A,
                        _ => { println!("panik!"); }
                    }
                },
                0xB0..=0xB7 => { // not used rn
                    println!("OR");
                    println!("NOT IMPLEMENTED!!!");
                },
                0xB8..=0xBF => {
                    // used in boot rom; completed
                    let selected_register = current_instruction & 0b111;
                    println!("CP {:X?}:{:}", selected_register, repr_8bit!(selected_register));

                    let reg: u8 = match selected_register {
                        0b000 => B,
                        0b001 => C,
                        0b010 => D,
                        0b011 => E,
                        0b100 => H,
                        0b101 => L,
                        0b110 => data[ eval_16bit!(H, L) as usize],
                        0b111 => A,
                        _ => todo!()
                    };
                    
                    let result = A.overflowing_add((reg ^ 0xFF).overflowing_add(1).0); // two's compliment moment

                    if result.0 == 0 { raise_flag!(z); } else { lower_flag!(z); }
                    raise_flag!(n);
                    if (result.0 & 0xF0) == (A & 0xF0) { raise_flag!(h); } else { lower_flag!(h); }
                    if result.0 >= A { raise_flag!(c); } else { lower_flag!(c); }

                    // A = result.0 the compare is not actually meant to store the result. it purely compares.
                },
                0b11000000 | 0b11001000 | 0b11010000 | 0b11011000 => { // 0b110xx000
                    println!("ret");
                    println!("NOT IMPLEMENTED!!!");
                },

                0b11000001 | 0b11010001 | 0b11100001 | 0b11110001 => { // 0b11xx0001
                    // used in boot rom; completed
                    let selected_register = (current_instruction >> 4) & 0b11;
                    println!("POP {:X?}:{:}", selected_register, repr_16bit!(AF, selected_register));

                    let reg1: &mut u8;
                    let reg2: &mut u8;

                    match selected_register
                    {
                        0b00 => { reg1 = &mut B; reg2 = &mut C; }
                        0b01 => { reg1 = &mut D; reg2 = &mut E; }
                        0b10 => { reg1 = &mut H; reg2 = &mut L; }
                        0b11 => { reg1 = &mut A; reg2 = &mut F; }
                        _ => { println!("panik! {:X?}", selected_register); reg1 = &mut A; reg2 = &mut F; }
                    }

                    *reg2 = stack.pop().unwrap();
                    *reg1 = stack.pop().unwrap();
                    SP += 2;
                },

                0b11000010 | 0b11001010 | 0b11010010 | 0b11011010 => { // 0b110xx010
                    println!("conditional jump");
                    println!("NOT IMPLEMENTED!!!");
                    PC += 2
                },

                0b11100000 | 0b11110000 | 0b11100010 | 0b11110010 => { // 0b111x0000 (?)
                    // used in boot rom; completed
                    // if 4th bit = 1: instruction is loading to A. else: instruction is loading to mem loc.
                    // if 2nd last bit = 1: insturction shall implicitly use C reg instead of following word.

                    let direction = (current_instruction >> 4) & 0b1;
                    let use_Creg = (current_instruction >> 1) & 0b1;
                    println!("load to/from accumulator (in)direct d:{:X?} C:{:X?}", direction, use_Creg);

                    let offset: usize;
                    if use_Creg == 1 { offset = (0xFF00 | C as u16) as usize; } else { offset = (0xFF00 | data[(PC+1) as usize] as u16) as usize; } // TODO: does rust have cond ? trueVal : FalsVal
                    if direction == 1 { A = data[offset]; } else { data[offset] = A; }

                    PC += 1;

                },

                0b11111010  => { // TODO: merge the similar logic in this with the next case (opcode 11101010)
                    // used in boot rom/ completed
                    let lsb = data[(PC+1) as usize] as u16;
                    let msb = (data[(PC+2) as usize] as u16) << 8;
                    let addr = lsb | msb;
                    
                    println!("load to A from 16bit addr {:X?}", addr);
                    A = data[addr as usize];
                    PC += 2
                },

                0b11101010 => {
                    // used in boot rom; completed
                    let lsb = data[(PC+1) as usize] as u16;
                    let msb = (data[(PC+2) as usize] as u16) << 8;
                    let addr = lsb | msb;
                    
                    println!("load from A to 16bit addr {:X?}", addr);
                    data[addr as usize] = A;
                    PC += 2
                },

                0b11111001 => {
                    println!("load SP from HL");
                    println!("NOT IMPLEMENTED!!!");
                },

                0b11111011 => {
                    // used in boot rom; completed(?)
                    println!("schedule to enable interrupts after next cycle");
                    IME = true;
                },

                0b11111110 => {
                    // used in boot rom; completed
                    let val = data[(PC.overflowing_add(1).0) as usize];
                    println!("CP n {:X?}", val);

                    let result = A.overflowing_add((val ^ 0xFF).overflowing_add(1).0); // two's compliment moment

                    if result.0 == 0 { raise_flag!(z); } else { lower_flag!(z); }
                    raise_flag!(n);
                    if (result.0 & 0xF0) == (A & 0xF0) { raise_flag!(h); } else { lower_flag!(h); }
                    if result.0 >= A { raise_flag!(c); } else { lower_flag!(c); }

                    // A = result.0 the compare is not actually meant to store the result. it purely compares.

                    PC += 1
                },

                0b11001001 => {
                    // used in boot rom; completed
                    println!("unconditional ret");

                    let lsb: u16 = stack.pop().unwrap() as u16;
                    let msb: u16 = (stack.pop().unwrap() as u16) << 8;
                    SP += 2;

                    PC = lsb | msb;

                    skip_increment = true; // we just set PC, so we dont want it incremented.
                },
                0b11000101 | 0b11010101 | 0b11100101 | 0b11110101 => { // 0b11xx0101
                    // used in boot rom; completed
                    let selected_register = (current_instruction >> 4) & 0b11;
                    println!("PUSH {:X?}:{:}", selected_register, repr_16bit!(AF, selected_register));

                    let reg1: &u8;
                    let reg2: &u8;

                    match selected_register
                    {
                        0b00 => { reg1 = &B; reg2 = &C; }
                        0b01 => { reg1 = &D; reg2 = &E; }
                        0b10 => { reg1 = &H; reg2 = &L; }
                        0b11 => { reg1 = &A; reg2 = &F; }
                        _ => { println!("panik! {:X?}", selected_register); reg1 = &A; reg2 = &F; }
                    }

                    stack.push(*reg1);
                    stack.push(*reg2);
                    SP -= 2;
                },
                0b11000111 | 0b11001111 | 0b11010111 | 0b11011111 | 0b11100111 | 0b11101111 | 0b11110111 | 0b11111111 => { // 0b11xxx111
                    println!("uncond func call");
                    println!("NOT IMPLEMENTED!!!");
                },
                _ => {
                    // used in boot rom, with 0xED
                    println!("anything, {:X?}, {:?}", current_instruction, current_instruction);
                }
            }

            if !skip_increment { PC += 1; }
            else { skip_increment = false; }

            // if PC >= 0x100 { break; }
            if PC == 0x66 { break; }
            // if PC >= 0x100 { PC = 0; }

            // thread::sleep(Duration::from_millis( 1000 * 1/4194304 ));
            // thread::sleep(Duration::from_millis(100));
    }

    });

    let event_loop = EventLoop::new().unwrap();
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);
    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);
    println!("y1");
    let mut app = App::default();
    app.data = data_wanter2;
    println!("y2");
    // todo: the blow run_app is blocking while the window is up. make it async.
    event_loop.run_app(&mut app);
    println!("y3");
    let window = app.window.unwrap();
    println!("y4");

    println!("LCDC: {:X?}", data[0xFF40]); // LCD control | R/W | All
    // loop { }
    Ok(())
}

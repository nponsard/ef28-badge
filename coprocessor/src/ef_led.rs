/// Driving WS2812B from the ULP RISC-V processor of the ESP32-S3.
/// This processor is clocked at around 17.5Mhz according to the datasheet.
/// This makes a clock cycle of around 60ns but I cannot find any information
/// about how many clock cycles an instruction takes to execute.
/// I ended up finding a sweet spot with trial and error where only a couple
/// NOP instructions are needed to match the timing requirements of the WS2812B.
///
/// It seems that there is not enough cycles available to be able to execute a
/// loop and still match the timings requirements of these LEDs.
///
/// My solution is to write a program in raw machine code from the data that
/// needs to be sent and then execute it once the whole program is written.
/// This allows to have the same timing for each bit for the 17 LEDs, at the
/// expense of memory useage.
///
use core::arch::asm;

use esp_lp_hal::{delay::Delay, gpio::Output};

// Shared address with the main processor where you can write debug codes
// const ADDRESS: u32 = 0x20;

// Number of NOPs to wait during each phase of a bit (following the WS2812B datasheet naming convention).
// const T0H: u8 = 0;
const T1H: u8 = 2;
const T0L: u8 = 4;
const T1L: u8 = 2;

pub const LED_COUNT: usize = 17;
pub type Rgb = (u8, u8, u8);
pub type LEDData = [Rgb; LED_COUNT];

// 17 leds
// 24 bits per led
// 16 bytes (8 compressed instructions) to encode 1 bit
// we need approx 5k
const BUFFER_SIZE: usize = 4908;

struct CodeBuffer {
    buffer: [u8; BUFFER_SIZE],
    pointer: usize,
}

impl CodeBuffer {
    pub fn new() -> Self {
        let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        Self { buffer, pointer: 0 }
    }
    // c1d0                    sw      a2,4(a1)
    pub fn turn_on(&mut self) {
        self.buffer[self.pointer] = 0xd0;
        self.buffer[self.pointer + 1] = 0xc1;
        self.pointer += 2;
    }

    // c590                    sw      a2,8(a1)
    pub fn turn_off(&mut self) {
        self.buffer[self.pointer] = 0x90;
        self.buffer[self.pointer + 1] = 0xc5;
        self.pointer += 2;
    }

    // 0001
    pub fn nop(&mut self) {
        self.buffer[self.pointer] = 0x01;
        self.buffer[self.pointer + 1] = 0x00;
        self.pointer += 2;
    }

    // 8082
    pub fn ret(&mut self) {
        self.buffer[self.pointer + 1] = 0x80;
        self.buffer[self.pointer] = 0x82;
        self.pointer += 2;
    }

    pub fn configure_registers(&mut self) {
        // 65a9 lui	        a1,0xa
        self.buffer[self.pointer] = 0xa9;
        self.buffer[self.pointer + 1] = 0x65;
        self.pointer += 2;

        // so we can use a compressed sw in turn_on and turn_off
        // 40058593                addi    a1,a1,1024
        self.buffer[self.pointer] = 0x93;
        self.buffer[self.pointer + 1] = 0x85;
        self.buffer[self.pointer + 2] = 0x05;
        self.buffer[self.pointer + 3] = 0x40;
        self.pointer += 4;

        //80 00 06 37       lui	a2,0x80000
        self.buffer[self.pointer] = 0x37;
        self.buffer[self.pointer + 1] = 0x06;
        self.buffer[self.pointer + 2] = 0x00;
        self.buffer[self.pointer + 3] = 0x80;
        self.pointer += 4;
    }

    // write the code for the one color (call this 3 times for one LED)
    pub fn color(&mut self, value: u8) {
        for i in 0..8 {
            // turn on
            self.turn_on();

            if (value >> (7 - i) & (1)) == 1 {
                // we need a 1, we need to wait 800ns
                for _ in 0..T1H {
                    self.nop();
                }
                // then low for 450ns

                // turn off
                self.turn_off();

                // NOPs
                for _ in 0..T1L {
                    self.nop();
                }
            } else {
                // we need a 0, we need to wait for 400ns
                // 0 nops
                // for _ in 0..T0H {
                // self.nop();
                // }
                // then low for 850 ns
                // turn off
                self.turn_off();

                // NOPs
                for _ in 0..T0L {
                    self.nop();
                }
            }
        }
    }
    // write the code for the 24 bits color
    pub fn colors(&mut self, rgb: &Rgb) {
        let (r, g, b) = rgb;
        // WS2812B gets first green, then red and then blue
        self.color(*g);
        self.color(*r);
        self.color(*b);
    }

    // used for debbuging purposes, write to 0x20 a debug code
    #[allow(dead_code)]
    pub fn write_debug(&mut self, value: u8) {
        // 00001737 lui	a4,0x1
        self.buffer[self.pointer] = 0x37;
        self.buffer[self.pointer + 1] = 0x17;
        self.buffer[self.pointer + 2] = 0x00;
        self.buffer[self.pointer + 3] = 0x00;

        // 07a00693 li	a3,122

        self.buffer[self.pointer + 4] = 0x93;
        self.buffer[self.pointer + 5] = 0x06;
        self.buffer[self.pointer + 6] = value << 4;
        self.buffer[self.pointer + 7] = value >> 4;

        // 80d72023 sw	a3,-2048(a4)
        self.buffer[self.pointer + 8] = 0x23;
        self.buffer[self.pointer + 9] = 0x20;
        self.buffer[self.pointer + 10] = 0xd7;
        self.buffer[self.pointer + 11] = 0x80;

        self.pointer += 12;
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }
}

pub fn run(gpio21: &mut Output<21>, colors_array: &LEDData) {
    // ensure we start at low and we pause for enough time (RES)
    gpio21.set_output(false);
    Delay.delay_millis(1);
    // let ptr = ADDRESS as *mut u32;

    // make the buffer
    let mut buffer = CodeBuffer::new();

    buffer.configure_registers();

    for c in colors_array.iter() {
        buffer.colors(c);
    }

    // return from the function
    buffer.ret();
    unsafe {
        // use this to debug the size of the buffer
        // ptr.write_volatile(pointer as u32);
        // Delay.delay_millis(1000);

        // jump to the code pointer
        let code_ptr = buffer.as_ptr();
        asm! {
            "jalr   ra,{x},0",
        x= in(reg) code_ptr}
    }
}

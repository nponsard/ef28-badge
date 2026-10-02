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
// const T0L: u8 = 4;
const T0L: u8 = T1H;
// const T1L: u8 = 0;

pub const LED_COUNT: usize = 17;
pub const LED_PIN: u8 = 21;
pub type Rgb = (u8, u8, u8);

pub type LedDataArr = [Rgb; LED_COUNT];

const PIN_REGISTER: u32 = 1 << (LED_PIN + 10);

// 17 leds
// 24 bits per led
// 16 bytes (8 compressed instructions) to encode 1 bit
// we need approx 5k
// const BUFFER_SIZE: usize = 4914;
const BUFFER_SIZE: usize = 224;
const BASE_BUFFER: [u8; BUFFER_SIZE] = [
    // === configure registers
    // Load pin register addr in a1
    //  65a9 lui a1,0xa
    0xa9,
    0x65,
    // 40058593 addi a1,a1,1024
    0x93,
    0x85,
    0x05,
    0x40,
    // Load which pins to toggle in a2
    // 80 00 06 37       lui	a2,PIN>>12
    0x37,
    0x06 | ((PIN_REGISTER >> 10) & 0xF0) as u8,
    (PIN_REGISTER >> 16) as u8,
    (PIN_REGISTER >> 24) as u8,
    // 00 80 06 13      addi a2, zero, PIN
    0x13,
    0x06,
    0x06 | (PIN_REGISTER << 4) as u8,
    (PIN_REGISTER >> 4) as u8,
    // 24 bits of color.
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    0x01,
    0x00,
    0x01,
    0x00,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    /// ============== THIS BIT IS ONE temporarily
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x01,
    0x00,
    0x01,
    0x00,
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // turn on c1d0 sw      a2,4(a1)
    0xd0,
    0xc1,
    // 0001 c.nop
    // 3 times,
    // place turn_off either in the first position for a 0, or in the last position for a 1.
    0x90,
    0xc5,
    0x01,
    0x00,
    0x01,
    0x00,
    // ret
    0x82,
    0x80,
];

struct CodeBuffer {
    buffer: [u8; BUFFER_SIZE],
    pointer: usize,
}

impl CodeBuffer {
    pub fn new() -> Self {
        let buffer: [u8; BUFFER_SIZE] = BASE_BUFFER;
        Self {
            buffer,
            pointer: 14,
        }
    }
    pub fn clear(&mut self) {
        self.pointer = 14;
    }

    pub fn clear_old(&mut self) {
        self.pointer = 0;
    }

    // c1d0                    sw      a2,4(a1)
    pub fn turn_on(&mut self) {
        self.buffer[self.pointer] = 0xd0;
        self.buffer[self.pointer + 1] = 0xc1;
        self.pointer += 2;
    }

    // c590                    sw      a2,8(a1)
    pub fn turn_off_at(&mut self, at: usize) {
        // each instruction is 2 bytes, first instructions is turn on.
        let at = at * 2 + 2;
        self.buffer[self.pointer + at] = 0x90;
        self.buffer[self.pointer + 1 + at] = 0xc5;
    }

    pub fn nop_at(&mut self, at: usize) {
        // each instruction is 2 bytes, first instructions is turn on.
        let at = at * 2 + 2;
        self.buffer[self.pointer + at] = 0x01;
        self.buffer[self.pointer + 1 + at] = 0x00;
    }

    // c590                    sw      a2,8(a1)
    pub fn turn_off(&mut self) {
        self.buffer[self.pointer] = 0x90;
        self.buffer[self.pointer + 1] = 0xc5;
        self.pointer += 2;
    }

    // 0001 two times
    pub fn nop(&mut self) {
        self.buffer[self.pointer] = 0x01;
        self.buffer[self.pointer + 1] = 0x00;
        self.buffer[self.pointer + 2] = 0x01;
        self.buffer[self.pointer + 3] = 0x00;
        self.pointer += 4;
    }

    // 8082
    pub fn ret(&mut self) {
        self.buffer[self.pointer + 1] = 0x80;
        self.buffer[self.pointer] = 0x82;
        self.pointer += 2;
    }

    pub fn configure_registers_old_a(&mut self) {
        self.pointer += 14;
    }
    pub fn configure_registers_old(&mut self) {
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

        // Store the pin offset in register a2.
        // firt 10 bits are reserved in RTC_GPIO_ENABLE_W1TC and RTC_GPIO_ENABLE_W1TS.
        let pin_register: u32 = 1 << (LED_PIN + 10);
        // let pin_register: u32 = 0xff_ff_ff_ff;

        // Load upper part of the pin.
        //80 00 06 37       lui	a2,PIN>>12
        self.buffer[self.pointer] = 0x37;
        self.buffer[self.pointer + 1] = 0x06 | ((pin_register >> 10) & 0xF0) as u8;
        // next 8 bits.
        self.buffer[self.pointer + 2] = (pin_register >> 16) as u8;
        // last 8 bits.
        self.buffer[self.pointer + 3] = (pin_register >> 24) as u8;
        self.pointer += 4;

        // Load lower part of the pin.
        // 00 80 06 13      addi a2, zero, PIN
        self.buffer[self.pointer] = 0x13;
        self.buffer[self.pointer + 1] = 0x06;
        // first 4 bits.
        self.buffer[self.pointer + 2] = 0x06 | (pin_register << 4) as u8;
        // next 8 bits.
        self.buffer[self.pointer + 3] = (pin_register >> 4) as u8;
        self.pointer += 4;
    }

    // write the code for the one color (call this 3 times for one LED)
    pub fn color_old(&mut self, value: u8) {
        for i in 0..8 {
            // turn on
            self.turn_on();

            if (value >> (7 - i) & (1)) == 1 {
                // we need a 1, we need to wait 800ns

                self.nop();

                // then low for 450ns

                // turn off
                self.turn_off();

                // NOPs
                // for _ in 0..T1L {
                //     self.nop();
                // }
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
                self.nop();
            }
        }
    }

    // write the code for the one color (call this 3 times for one LED)
    pub fn color(&mut self, value: u8) {
        for i in 0..8 {
            if (value >> (7 - i) & (1)) == 1 {
                self.nop_at(0);

                // we need a 1, we need to wait 800ns high
                self.turn_off_at(2);
            } else {
                //  low for 800 ns
                self.turn_off_at(0);
                self.nop_at(2);
            }
            self.pointer += 8;
        }
    }

    // write the code for the 24 bits color
    pub fn colors_old(&mut self, rgb: &Rgb) {
        let (r, g, b) = rgb;
        // WS2812B gets first green, then red and then blue
        self.color(*g);
        self.color(*r);
        self.color(*b);
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

pub fn run<'a, const PIN: u8>(led_pin: &mut Output<PIN>, colors_array: impl Iterator<Item = Rgb>) {
    // ensure we start at low and we pause for enough time (RES)
    led_pin.set_output(false);
    Delay.delay_millis(1);

    // let ptr = ADDRESS as *mut u32;

    // make the buffer
    let mut buffer = CodeBuffer::new();
    for c in colors_array {
        // Delay.delay_micros(30);
        // buffer.clear();
        // buffer.configure_registers();
        // buffer.colors(&c);
        // return from the function
        // buffer.ret();

        colors_asm(c);

        // let code_ptr = buffer.as_ptr();
        // unsafe {
        //     // use this to debug how many bytes are used in the buffer
        //     // ptr.write_volatile(pointer as u32);
        //     // Delay.delay_millis(1000);

        //     // jump to the code pointer
        //     asm! {
        //         "
        //         addi  sp,sp,-8
        //         sw     a1, 0(sp)
        //         sw     a2, 4(sp)
        //         jalr   ra,{x},0
        //         lw     a1, 0(sp)
        //         lw     a2, 4(sp)
        //         addi   sp,sp,8
        //         ",
        //         x= in(reg) code_ptr
        //     }
        //     // led_pin.set_output(true);
        // }
    }
}

pub fn colors_asm(rgb: Rgb) {
    let b: u32 = u32::from_le_bytes([0x00, rgb.0, rgb.1, rgb.2]);
    let mut counter = 24;
    unsafe {
        asm!(
         "

            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
            
            andi a3, a2, 0x01
            c.srli a2, 1
            c.beqz a3, 2f
            # bit 1
            sw a1, 4(a0) # on
            c.nop
            c.nop
            c.sw a1, 8(a0) # off
            c.j 3f
            
        2: # bit 0
            sw a1, 4(a0) # on
            c.sw a1, 8(a0) # off
            c.nop 
            c.nop 
            c.nop 
            c.nop
        3: 
        "
        ,
            in("a2") b,
            in("a1") PIN_REGISTER,
            in("a0") GPIO_REG,
            out("a3") _,
            // c = inout(reg) counter,
        );
    }
}

pub fn colors(led_pin: &mut Output<LED_PIN>, rgb: &Rgb) {
    let (r, g, b) = rgb;
    // WS2812B gets first green, then red and then blue
    color(led_pin, *g);
    color(led_pin, *r);
    color(led_pin, *b);
}

const GPIO_REG: u32 = 0xa400;

// write the code for the one color (call this 3 times for one LED)
pub fn color(led_pin: &mut Output<LED_PIN>, mut value: u8) {
    for _ in 0..8 {
        if (value & 1) == 1 {
            unsafe {
                asm!(
                         "
                sw {pin}, 4({reg})
                nop 
                nop
                nop
                sw {pin}, 8({reg})

                ",
                pin  = in (reg) PIN_REGISTER,
                reg = in(reg) GPIO_REG,
                )
            }
        } else {
            unsafe {
                asm!(
                         "
                sw {pin}, 4({reg})
                sw {pin}, 8({reg})
                nop
                ",
                pin  = in (reg) PIN_REGISTER,
                reg = in(reg) GPIO_REG,
                )
            }
        }
        // unsafe { &*RTC_IO::PTR }
        //     .out_w1tc()
        //     .write(|w| unsafe { w.out_data_w1tc().bits(1 << LED_PIN) });
        value = value >> 1;
        // unsafe {
        //     asm!(
        //         "
        //         nop
        //         "
        //     );
        // }
    }
}

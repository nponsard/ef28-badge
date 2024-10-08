#![no_std]
#![no_main]

use core::arch::asm;

use esp_lp_hal::{delay::Delay, gpio::Output, prelude::*};
use panic_halt as _;

const ADDRESS: u32 = 0x20;

// #define EFLED_PIN_LED_DATA 21
// #define EFLED_PIN_5VBOOST_ENABLE 9

// NOP :
// 0001
// ON :
// 40c5a223
// OFF :
// 40c5a423
// Ret :
// 8082

//65a9                	lui	a1,0xa
//80 00 0637          	lui	a2,0x80000

const T0H: u8 = 0;
const T1H: u8 = 2;
const T0L: u8 = 4;
const T1L: u8 = 2;

const LED_COUNT: usize = 13;

// 17 leds
// 24 bits per led
// 16 bytes to encode 1 bit
// we need approx 6.5k
const ICOUNT: usize = 5400;

// 40c5a223
pub fn turn_on(buffer: &mut [u8], pointer: &mut usize) {
    buffer[*pointer] = 0x23;
    buffer[*pointer + 1] = 0xa2;
    buffer[*pointer + 2] = 0xc5;
    buffer[*pointer + 3] = 0x40;
    *pointer += 4;
}

// 40c5a423
pub fn turn_off(buffer: &mut [u8], pointer: &mut usize) {
    buffer[*pointer] = 0x23;
    buffer[*pointer + 1] = 0xa4;
    buffer[*pointer + 2] = 0xc5;
    buffer[*pointer + 3] = 0x40;
    *pointer += 4;
}

// has been re-reverted
pub fn nop(buffer: &mut [u8], pointer: &mut usize) {
    buffer[*pointer] = 0x00;
    buffer[*pointer + 1] = 0x01;
    *pointer += 2;
}

pub fn write_debug(buffer: &mut [u8], pointer: &mut usize, value: u8) {
    // 00001737 lui	a4,0x1
    buffer[*pointer + 0] = 0x37;
    buffer[*pointer + 1] = 0x17;
    buffer[*pointer + 2] = 0x00;
    buffer[*pointer + 3] = 0x00;

    // 07a00693 li	a3,122

    buffer[*pointer + 4] = 0x93;
    buffer[*pointer + 5] = 0x06;
    buffer[*pointer + 6] = value << 4;
    buffer[*pointer + 7] = value >> 4;

    // 80d72023 sw	a3,-2048(a4)
    buffer[*pointer + 8] = 0x23;
    buffer[*pointer + 9] = 0x20;
    buffer[*pointer + 10] = 0xd7;
    buffer[*pointer + 11] = 0x80;

    *pointer += 12;
}

pub fn ret(buffer: &mut [u8], pointer: &mut usize) {
    buffer[*pointer] = 0x82;
    buffer[*pointer + 1] = 0x80;
    *pointer += 2;
}

pub fn configure_registers(buffer: &mut [u8], pointer: &mut usize) {
    //65a9 lui	        a1,0xa
    buffer[*pointer] = 0xa9;
    buffer[*pointer + 1] = 0x65;
    *pointer += 2;

    //80 00 06 37       lui	a2,0x80000
    buffer[*pointer] = 0x37;
    buffer[*pointer + 1] = 0x06;
    buffer[*pointer + 2] = 0x00;
    buffer[*pointer + 3] = 0x80;
    *pointer += 4;
}

// write the code for the one color (call this 3 times for one LED)
pub fn color(buffer: &mut [u8], pointer: &mut usize, value: u8) {
    for i in 0..8 {
        // turn on
        turn_on(buffer, pointer);

        if (value >> (7 - i) & (1)) == 1 {
            // we need a 1, we need to wait 800ns
            // 4 nops
            for _ in 0..T1H {
                nop(buffer, pointer);
            }
            // then low for 450ns

            // turn off
            turn_off(buffer, pointer);

            // NOPs
            for _ in 0..T1L {
                nop(buffer, pointer);
            }
        } else {
            // we need a 0, we need to wait for 400ns
            // 2 nops
            for _ in 0..T0H {
                nop(buffer, pointer);
            }
            // then low for 850 ns
            // turn off
            turn_off(buffer, pointer);

            // NOPs
            for _ in 0..T0L {
                nop(buffer, pointer);
            }
        }
    }
}

// #[derive(Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn run(gpio21: &mut Output<21>, colors_array: [RGB; LED_COUNT]) {
    // ensure we start at low and we pause for enough time
    gpio21.set_output(false);
    Delay.delay_millis(1);
    // make the buffer
    let ptr = ADDRESS as *mut u32;
    unsafe {
        ptr.write_volatile(130);
    }

    let mut buffer: [u8; ICOUNT] = [0; ICOUNT];
    unsafe {
        ptr.write_volatile(131);
    }
    let mut pointer = 0;

    configure_registers(&mut buffer, &mut pointer);

    for (count, c) in colors_array.into_iter().enumerate() {
        unsafe {
            ptr.write_volatile(count as u32 + 200);
        }
        colors(&mut buffer, &mut pointer, &c);
    }

    unsafe {
        ptr.write_volatile(300);
    }

    // return from the function
    ret(&mut buffer, &mut pointer);

    unsafe {
        ptr.write_volatile(301);
    }
    unsafe {
        ptr.write_volatile(pointer as u32);
        // Delay.delay_millis(10000);
        // ptr.write_volatile(223);
        // Delay.delay_millis(1000);

        // ptr.write_volatile(pointer as u32);
        // Delay.delay_millis(1000);

        // jump to the code pointer

        ptr.write_volatile(132);

        let code_ptr = buffer.as_ptr();
        asm! {
            "jalr   ra,{x},0",
        x= in(reg) code_ptr}

        // exec_fn(); // Call the binary code as a function
        // {
        //     let ptr = ADDRESS as *mut u32;
        //     ptr.write_volatile(225);
        // }
        // Delay.delay_millis(1000);
        ptr.write_volatile(133);
    }
}

pub fn colors(buffer: &mut [u8], pointer: &mut usize, rgb: &RGB) {
    // WS2812B gets first green, then red and then blue

    color(buffer, pointer, rgb.g);
    color(buffer, pointer, rgb.r);
    color(buffer, pointer, rgb.b);
}

pub fn zero(gpio21: &mut Output<21>) {
    // one clock cycle is around 60 ns
    // we need to wait 400ns
    gpio21.set_output(true);
    // unsafe { asm!("nop",) }
    // we need to wait 850ns
    gpio21.set_output(false);
    unsafe { asm!("nop", "nop", "nop") }
}

#[entry]
fn main(mut gpio21: Output<21>, mut gpio9: Output<9>) -> ! {
    gpio9.set_output(true);
    // let mut i: u32 = 0;

    let ptr = ADDRESS as *mut u32;
    unsafe {
        ptr.write_volatile(122);
    }
    // for _ in 0..3 {
    //     for _ in 0..24 {
    //         zero(&mut gpio21);
    //     }
    // }

    Delay.delay_millis(200);
    // let count = 5;
    loop {
        unsafe {
            ptr.write_volatile(111111);
        }

        for i in 0..20 {
            unsafe {
                ptr.write_volatile(i as u32);
            }
            run(
                &mut gpio21,
                [
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    RGB { r: i, g: 0, b: 0 },
                    // RGB { r: i, g: 0, b: 0 },
                    // RGB { r: i, g: 0, b: 0 },
                    // RGB { r: i, g: 0, b: 0 },
                    // RGB { r: i, g: 0, b: 0 },
                ],
            );
            Delay.delay_millis(10);
        }

        for i in 0..20 {
            unsafe {
                ptr.write_volatile(30 + i as u32);
            }
            run(
                &mut gpio21,
                [
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    RGB { r: 0, g: i, b: 0 },
                    // RGB { r: 0, g: i, b: 0 },
                    // RGB { r: 0, g: i, b: 0 },
                    // RGB { r: 0, g: i, b: 0 },
                    // RGB { r: 0, g: i, b: 0 },
                ],
            );
            Delay.delay_millis(10);
        }

        for i in 0..20 {
            run(
                &mut gpio21,
                [
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    RGB { r: 0, g: 0, b: i },
                    // RGB { r: 0, g: 0, b: i },
                    // RGB { r: 0, g: 0, b: i },
                    // RGB { r: 0, g: 0, b: i },
                    // RGB { r: 0, g: 0, b: i },
                ],
            );
            Delay.delay_millis(10);
        }

        // unsafe {
        //     ptr.write_volatile(4);
        // }

        // color_1_3(&mut gpio21);
        // color_3_1(&mut gpio21);
        // color_1_3(&mut gpio21);
        // Delay.delay_millis(1000);

        // }

        // for _ in 0..6 {
        //     for _ in 0..8 {
        //         gpio21.set_high();
        //         Delay.delay_nanos(395);
        //         gpio21.set_low();
        //         Delay.delay_nanos(835);
        //     }
        // }

        // for i in 0..3 {
        //     for j in 0..6 {
        //         gpio1.set_high();
        //         Delay.delay_nanos(375);
        //         gpio1.set_low();
        //         Delay.delay_nanos(825);
        //     }
        //     gpio1.set_high();
        //     Delay.delay_nanos(775);
        //     gpio1.set_low();
        //     Delay.delay_nanos(425);

        //     for j in 0..7{
        //         gpio1.set_high();
        //         Delay.delay_nanos(375);
        //         gpio1.set_low();
        //         Delay.delay_nanos(825);
        //     }

        //     for j in 0..7 {
        //         gpio1.set_high();
        //         Delay.delay_nanos(375);
        //         gpio1.set_low();
        //         Delay.delay_nanos(825);
        //     }
        // }
    }
}

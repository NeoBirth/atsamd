#![no_std]
#![no_main]

extern crate cortex_m;
extern crate trellis_m4 as hal;
extern crate panic_halt;
extern crate smart_leds;
extern crate ws2812_timer_delay as ws2812;
extern crate embedded_hal;
extern crate nb;

use nb::block;
use hal::prelude::*;
use hal::{entry, Peripherals};
use hal::{clock::GenericClockController, timer::TimerCounter};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);

    timer.start(5_000_000u32.hz());

    let mut neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);

    loop {
         block!(timer.wait()).ok();
         neopixel_pin.set_high();
         block!(timer.wait()).ok();
         neopixel_pin.set_low();

    }
}

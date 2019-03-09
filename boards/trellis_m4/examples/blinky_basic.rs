#![no_std]
#![no_main]

extern crate cortex_m;
extern crate trellis_m4 as hal;
extern crate panic_halt;
extern crate smart_leds;
extern crate ws2812_timer_delay as ws2812;

use hal::prelude::*;
use hal::{entry, Peripherals, CorePeripherals};
use hal::{clock::GenericClockController, delay::Delay};
use hal::timer::TimerCounter;

use smart_leds::{SmartLedsWrite, Color};
use smart_leds::colors::RED;
use smart_leds::brightness;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
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
    timer.start(3_000_000u32.hz());

    let mut neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = ws2812::Ws2812::new(timer, &mut neopixel_pin);
    let mut delay = Delay::new(core.SYST, &mut clocks);


    loop {
        let data = [RED; 1];
        neopixel.write(brightness(data.iter().cloned(), 32)).unwrap();
        delay.delay_ms(250u8);
        let data2 = [Color::default(); 1];
        neopixel.write(brightness(data2.iter().cloned(), 32)).unwrap();
        delay.delay_ms(250u8);
    }
}

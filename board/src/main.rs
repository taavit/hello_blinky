#![no_std]
#![no_main]

use core::cell::Cell;

use critical_section::Mutex;
use hello_blinky_driver::FinalBlinky;
use cortex_m::delay::Delay;
use hello_blinky_driver::LedDuration;
use hello_blinky_driver::LedSignal;
use rp_pico::entry;

use panic_halt as _;

use rp_pico::hal::gpio::PullDown;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

use rp_pico::hal::gpio;

static DEVICE: Mutex<
    Cell<
        Option<
            FinalBlinky<
                gpio::Pin<
                    gpio::bank0::Gpio25,
                    gpio::FunctionSioOutput,
                    PullDown
                >,
                Delay
            >
        >
    >
> = Mutex::new(Cell::new(None));

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let led_pin = pins.led.into_push_pull_output();
    
    let dev = FinalBlinky::new(led_pin, delay);
    critical_section::with(
        |cs| DEVICE.borrow(cs).replace(Some(dev))
    );

    loop {
        use_device();
    }
}

fn use_device() {
    critical_section::with(
        |cs| {
            let mut dev = DEVICE.borrow(cs).take().unwrap();
            dev.blink_times(4);
            dev.blink_sequence(
                &[
                    LedSignal::BLINK(LedDuration::LONG),
                    LedSignal::BLINK(LedDuration::SHORT),
                    LedSignal::BLINK(LedDuration::SHORT),
                    LedSignal::BLINK(LedDuration::LONG),
                ]);
            DEVICE.borrow(cs).set(Some(dev));
        }
    );
}

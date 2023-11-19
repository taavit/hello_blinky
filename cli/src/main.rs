use std::{thread, time::Duration, cell::Cell, sync::Mutex, convert::Infallible};
use hello_blinky_driver::FinalBlinky;
use embedded_hal::{digital::{OutputPin, ErrorType}, delay::DelayUs};

static BLINKER: Mutex<Cell<Option<FinalBlinky<PinMock, DelayerMock>>>> = Mutex::new(Cell::new(None));

struct PinMock;
struct DelayerMock;

impl DelayUs for DelayerMock {
    fn delay_ms(&mut self, ms: u32) {
        std::thread::sleep(Duration::from_millis(ms.into()));
    }
    fn delay_us(&mut self, us: u32) {
        std::thread::sleep(Duration::from_micros(us.into()));
    }
}

impl ErrorType for PinMock {
    type Error = Infallible;
}

impl OutputPin for PinMock {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        println!("SET HIGH");
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        println!("SET LOW");
        Ok(())
    }
    fn set_state(&mut self, state: embedded_hal::digital::PinState) -> Result<(), Self::Error> {
        println!("SET {:?}", state);
        Ok(())
    }
}

fn main() {
    println!("Blinky!");
    let pin = PinMock;
    let delayer = DelayerMock;
    let b = FinalBlinky::new(pin, delayer);
    BLINKER.lock().unwrap().replace(Some(b));
    loop {
        on_timer();
        thread::sleep(Duration::from_secs(15));
    }
}

fn on_timer() {
    print!("\x1B[2J\x1B[1;1H");
    let mut blinker = BLINKER.lock().unwrap().replace(None).unwrap();
    blinker.blink_times(5);
    BLINKER.lock().unwrap().replace(Some(blinker));
}

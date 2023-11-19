#![no_std]

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayMs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LedDuration {
    LONG,
    #[default]
    SHORT,
    CUSTOM(u32),
}

impl LedDuration {
    pub const SHORT_MS: u32 = 250;
    pub const LONG_MS: u32 = 750;

    pub fn new(ms: u32) -> Self {
        ms.into()
    }
}

impl From<u32> for LedDuration {
    fn from(ms: u32) -> Self {
        match ms {
            Self::SHORT_MS => LedDuration::SHORT,
            Self::LONG_MS => LedDuration::LONG,
            _ => LedDuration::CUSTOM(ms),
        }
    }
}

impl From<LedDuration> for u32 {
    fn from(value: LedDuration) -> Self {
        match value {
            LedDuration::SHORT => LedDuration::SHORT_MS,
            LedDuration::LONG => LedDuration::LONG_MS,
            LedDuration::CUSTOM(v) => v,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LedSignal {
    BLINK(LedDuration),
    PAUSE(LedDuration),
}

impl From<bool> for LedSignal {
    fn from(value: bool) -> Self {
        if value {
            Self::BLINK(LedDuration::default())
        } else {
            Self::PAUSE(LedDuration::default())
        }
    }
}

pub struct FinalBlinky<Pin: OutputPin, Delayer: DelayMs<u32>> {
    led: Pin,
    delayer: Delayer,
}

impl <Pin: OutputPin, Delayer: DelayMs<u32>> FinalBlinky<Pin, Delayer> {
    pub fn new(led: Pin, delayer: Delayer) -> Self {
        Self {
            led,
            delayer,
        }
    }

    pub fn blink_times(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.led.set_high();
            self.delayer.delay_ms(125);
            let _ = self.led.set_low();
            self.delayer.delay_ms(125);
        }
    }

    pub fn blink_scheme(&mut self, scheme: &[LedSignal]) {
        for signal in scheme {
            match *signal {
                LedSignal::BLINK(LedDuration::CUSTOM(0)) => {
                    self.delayer.delay_ms(250);
                }
                LedSignal::BLINK(duration) => {
                    let _ = self.led.set_high();
                    self.delayer.delay_ms(duration.into());
                    let _ = self.led.set_low();
                    self.delayer.delay_ms(125);
                }
                LedSignal::PAUSE(duration) => {
                    self.delayer.delay_ms(duration.into());
                }
            }
        }
    }
}

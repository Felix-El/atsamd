#![no_std]
#![no_main]

use bsp::hal;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use samd11_bare as bsp;

use cortex_m_rt::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::pwm::{Channel, Pwm0};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = bsp::Pins::new(peripherals.PORT);

    let _d1 = pins.d1.into_function_f(&mut pins.port);
    let _d14 = pins.d14.into_function_f(&mut pins.port);

    let gclk0 = clocks.gclk0();
    let mut pwm0 = Pwm0::new(
        &clocks.tcc0(&gclk0).unwrap(),
        1.khz(),
        peripherals.TCC0,
        &mut peripherals.PM,
    );
    let max_duty = pwm0.get_max_duty();

    loop {
        pwm0.set_duty(Channel::_0, max_duty / 2);
        pwm0.set_duty(Channel::_1, max_duty / 8);
        delay.delay_ms(1000u16);

        pwm0.set_duty(Channel::_0, max_duty / 8);
        pwm0.set_duty(Channel::_1, max_duty / 2);
        delay.delay_ms(1000u16);
    }
}

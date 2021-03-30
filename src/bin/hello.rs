#![no_main]
#![no_std]

use nucleo_h7xx as nucleo;
use nucleo::hal::prelude::*;
use nucleo::led::Led;


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, {{crate_name}}!");

    // - board setup ----------------------------------------------------------

    let board = nucleo::board::Board::take().unwrap();

    let dp = nucleo::pac::Peripherals::take().unwrap();

    let ccdr = board.freeze_clocks(dp.PWR.constrain(),
                                   dp.RCC.constrain(),
                                   &dp.SYSCFG);

    let pins = board.split_gpios(dp.GPIOA.split(ccdr.peripheral.GPIOA),
                                 dp.GPIOB.split(ccdr.peripheral.GPIOB),
                                 dp.GPIOC.split(ccdr.peripheral.GPIOC),
                                 dp.GPIOD.split(ccdr.peripheral.GPIOD),
                                 dp.GPIOE.split(ccdr.peripheral.GPIOE),
                                 dp.GPIOF.split(ccdr.peripheral.GPIOF),
                                 dp.GPIOG.split(ccdr.peripheral.GPIOG));

    let mut user_leds = nucleo::led::UserLeds::new(pins.user_leds);


    // - main loop ------------------------------------------------------------

    let one_second = ccdr.clocks.sys_ck().0;

    for n in 0..10 {
        user_leds.ld3.off();
        user_leds.ld1.on();
        cortex_m::asm::delay(one_second);

        user_leds.ld1.off();
        user_leds.ld2.on();
        cortex_m::asm::delay(one_second);

        user_leds.ld2.off();
        user_leds.ld3.on();
        cortex_m::asm::delay(one_second);

        defmt::info!("loop: {:?} of 10", n + 1);
    }

    {{crate_name}}::exit()
}

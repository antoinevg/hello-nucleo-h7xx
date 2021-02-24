#![no_main]
#![no_std]

use nucleo_h745zi as nucleo;
use nucleo::led::Led;


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, {{crate_name}}!");

    // - board setup ----------------------------------------------------------

    let board = nucleo::Board::take().unwrap();
    let user_leds = &mut board.user_leds;


    // - main loop ------------------------------------------------------------

    let one_second = board.clocks.sys_ck().0;

    for n in 0..10 {
        user_leds.LD3.off();
        user_leds.LD1.on();
        cortex_m::asm::delay(one_second);

        user_leds.LD1.off();
        user_leds.LD2.on();
        cortex_m::asm::delay(one_second);

        user_leds.LD2.off();
        user_leds.LD3.on();
        cortex_m::asm::delay(one_second);

        defmt::info!("loop: {:?} of 10", n + 1);
    }

    {{crate_name}}::exit()
}

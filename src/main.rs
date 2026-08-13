
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32g4xx_hal::{
   gpio::GpioExt, pac::{self, Peripherals}, prelude::*, pwr::{PwrExt, VoltageScale}, rcc::*, time::{ExtU32, RateExtU32},
};

#[entry]
fn main() -> ! {
    // 1. Acquire peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    // 2. Do standard STM32 hardware setup: configure power and clocks
    let pwr = dp.PWR.constrain().vos(VoltageScale::Range1 { enable_boost: true }).freeze();
    let mut rcc = dp.RCC.freeze(
        Config::pll()
            .pll_cfg(PllConfig {
                // Change to 24.MHz() on rover boards (check schematic)
                mux: PllSrc::HSE(8.MHz()),
                m: PllMDiv::DIV_2,
                n: PllNMul::MUL_28,
                r: Some(PllRDiv::DIV_2),
                q: Some(PllQDiv::DIV_2),
                p: None,
            }),
            pwr
        );

    // 3. Create peripherals provided by the HAL
    let mut delay_syst = cp.SYST.delay(&rcc.clocks);
    let gpioc = dp.GPIOC.split(&mut rcc);
    let mut pc6 = gpioc.pc6.into_push_pull_output();
    // 4. Endless loop
    loop {
        pc6.set_high();
        delay_syst.delay(1000.millis());
        pc6.set_low();
        delay_syst.delay(1000.millis());
    }
}
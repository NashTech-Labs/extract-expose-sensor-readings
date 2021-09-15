#![no_std]

pub use panic_itm;

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::{
    stm32f3xx_hal::{delay::Delay, prelude, stm32::i2c1},
    switch_hal,
};

pub use stm32f3_discovery::stm32f3xx_hal::{
    gpio::gpiob::{PB6, PB7},
    gpio::AF4,
    i2c::I2c,
    prelude::*,
    stm32::{self, I2C1},
};

pub use lsm303agr::{interface::I2cInterface, mode, Lsm303agr, MagOutputDataRate};

/// Function init() do the initialization for the leds, Delay, Lsm303, and for itm.
///
/// #RETURN
/// This function is going to return:(Lsm303agr, Delay, ITM)
/// Lsm303agr -> provide the access to the magnetometer and its registers.
/// Delay -> provide the delay process to the code.
/// ITM -> provide the method to print (x,y) axis value of MFD.
pub fn init() -> (
    Lsm303agr<I2cInterface<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>, mode::MagContinuous>,
    ITM,
) {
    let core_peripheral = cortex_m::Peripherals::take().unwrap();
    let device_peripheral = stm32::Peripherals::take().unwrap();

    let mut flash = device_peripheral.FLASH.constrain();
    let mut rcc = device_peripheral.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = device_peripheral.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::new(
        device_peripheral.I2C1,
        (scl, sda),
        400.khz(),
        clocks,
        &mut rcc.apb1,
    );

    let mut lsm = Lsm303agr::new_with_i2c(i2c);
    lsm.init().unwrap();
    lsm.set_mag_odr(MagOutputDataRate::Hz10).unwrap();

    (lsm.into_mag_continuous().ok().unwrap(), core_peripheral.ITM)
}

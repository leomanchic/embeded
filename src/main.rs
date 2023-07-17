#![no_std]
#![no_main]

use nb::block;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder,},
};
use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let yoffset = 1;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();
 
    let mut x: i32 = 128;
    loop {
    if x < -20{x=128}; x-=8;

    Circle::new(Point::new(x, yoffset), 20)
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    display.flush().unwrap();
    display.init().unwrap();
    block!(timer.wait()).unwrap();

    Circle::new(Point::new(x, yoffset), 16)
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    display.flush().unwrap();
    
    display.init().unwrap();

    block!(timer.wait()).unwrap();
    x-=8;

    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
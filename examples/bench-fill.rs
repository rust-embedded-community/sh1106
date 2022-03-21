//! Bounce a DVD player logo around the screen
//!
//! Like this, but with no color changing: https://bouncingdvdlogo.com/
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB9
//! (green)  SCL -> PB8
//! ```
//!
//! Run on a Blue Pill with `cargo run --example rtic_dvd`.

#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use heapless::String;
use panic_semihosting as _;
use rtic::app;
use sh1106::{prelude::*, Builder};
use stm32f1xx_hal::{
    gpio,
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac::{self, I2C1},
    prelude::*,
    timer::{CountDownTimer, Event, Timer},
};

type Display = GraphicsMode<
    I2cInterface<
        BlockingI2c<
            I2C1,
            (
                gpio::gpiob::PB8<gpio::Alternate<gpio::OpenDrain>>,
                gpio::gpiob::PB9<gpio::Alternate<gpio::OpenDrain>>,
            ),
        >,
    >,
>;

#[inline(always)]
fn stopwatch<F>(f: F) -> u32
where
    F: FnOnce() -> (),
{
    let start: u32 = pac::DWT::cycle_count();
    f();
    let end: u32 = pac::DWT::cycle_count();
    end.wrapping_sub(start)
}

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        display: Display,
        timer: CountDownTimer<pac::TIM1>,
        #[init(0)]
        frame: u32,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let dp = cx.device;
        let mut cp = cx.core;

        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

        let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
        let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

        let i2c = BlockingI2c::i2c1(
            dp.I2C1,
            (scl, sda),
            &mut afio.mapr,
            Mode::Fast {
                frequency: 400_000.hz(),
                duty_cycle: DutyCycle::Ratio2to1,
            },
            clocks,
            &mut rcc.apb1,
            1000,
            10,
            1000,
            1000,
        );

        let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

        display.init().unwrap();
        display.flush().unwrap();

        // Update framerate
        let fps = 1;

        let mut timer = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).start_count_down(fps.hz());

        timer.listen(Event::Update);

        // Init the static resources to use them later through RTIC
        init::LateResources { timer, display }
    }

    #[idle()]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Fix default wfi() behaviour breaking debug probe
            core::hint::spin_loop();
        }
    }

    #[task(binds = TIM1_UP, resources = [display, timer, frame])]
    fn update(cx: update::Context) {
        let update::Resources {
            display,
            timer,
            frame,
            ..
        } = cx.resources;

        display.clear();
        display.flush().unwrap();

        let center = display.bounding_box().center();

        // Only bench time taken to draw rectangles
        let time = stopwatch(|| {
            for x in 0i32..64 {
                // Square squares in center
                Rectangle::with_center(center, Size::new(x as u32, x as u32))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(display)
                    .unwrap();
            }

            for x in 0i32..64 {
                // Tall rectangles
                Rectangle::with_center(Point::new(x * 5, 20), Size::new(4, x as u32))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(display)
                    .unwrap();

                // Wide rectangles
                Rectangle::with_center(Point::new(0, x * 2), Size::new(x as u32, 4))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(display)
                    .unwrap();
            }
        });

        // Convert time to ms by dividing by sysclk * 1000
        let time = time / 72_000;

        let mut s: String<32> = String::new();

        write!(s, "{}ms", time).ok();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        Text::with_baseline(&s, Point::zero(), text_style, Baseline::Top)
            .draw(display)
            .unwrap();

        display.flush().unwrap();

        *frame += 1;

        // Clears the update flag
        timer.clear_update_interrupt_flag();
    }

    extern "C" {
        fn EXTI0();
    }
};

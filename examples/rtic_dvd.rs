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

use embedded_graphics::{
    geometry::Point,
    image::Image,
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
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
use tinybmp::Bmp;

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

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        display: Display,
        timer: CountDownTimer<pac::TIM1>,
        top_left: Point,
        velocity: Point,
        bmp: Bmp<Rgb565, 'static>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let dp = cx.device;

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
        let fps = 20;

        let mut timer = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).start_count_down(fps.hz());

        timer.listen(Event::Update);

        let bmp = Bmp::from_slice(include_bytes!("dvd.bmp")).unwrap();

        // Init the static resources to use them later through RTIC
        init::LateResources {
            timer,
            display,
            top_left: Point::new(5, 3),
            velocity: Point::new(1, 1),
            bmp,
        }
    }

    #[idle()]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Fix default wfi() behaviour breaking debug probe
            core::hint::spin_loop();
        }
    }

    #[task(binds = TIM1_UP, resources = [display, top_left, velocity, timer, bmp])]
    fn update(cx: update::Context) {
        let update::Resources {
            display,
            top_left,
            velocity,
            timer,
            bmp,
            ..
        } = cx.resources;

        let bottom_right = *top_left + bmp.bounding_box().size;

        // Erase previous image position with a filled black rectangle
        Rectangle::with_corners(*top_left, bottom_right)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(display)
            .unwrap();

        // Check if the image collided with a screen edge
        {
            if bottom_right.x > display.size().width as i32 || top_left.x < 0 {
                velocity.x = -velocity.x;
            }

            if bottom_right.y > display.size().height as i32 || top_left.y < 0 {
                velocity.y = -velocity.y;
            }
        }

        // Move the image
        *top_left += *velocity;

        // Draw image at new position
        Image::new(bmp, *top_left)
            .draw(&mut display.color_converted())
            .unwrap();

        // Write changes to the display
        display.flush().unwrap();

        // Clears the update flag
        timer.clear_update_interrupt_flag();
    }

    extern "C" {
        fn EXTI0();
    }
};

#![no_std]
#![no_main]

extern crate panic_halt;

mod mypins {
    use xiao_m0::hal;

    hal::bsp_pins!(
        PA25 {
            name: testpin
            aliases: {
                AlternateG: UsbDp
            }
        }
    );

    /*
            ******************************************
            ******************************************
            This is the rust-analyzer expansion
            ******************************************
            ******************************************

        // Recursive expansion of bsp_pins! macro
        // =======================================

        #[doc = " BSP replacement for the HAL"]
        #[doc = " [`Pins`](atsamd_hal::gpio::v2::Pins) type"]
        #[doc = ""]
        #[doc = " This type is intended to provide more meaningful names for the"]
        #[doc = " given pins."]
        pub struct Pins {
            port: Option<$crate::pac::PORT>,
        }
        impl Pins {
            #[doc = " Take ownership of the PAC [`PORT`] and split it into"]
            #[doc = " discrete [`Pin`]s."]
            #[doc = ""]


            [ ... REST OMITTED ... ]

            ******************************************
            ******************************************
            This is the cardo expand expansion (via rustc)
            ******************************************
            ******************************************

        /// BSP replacement for the HAL
        /// [`Pins`](atsamd_hal::gpio::v2::Pins) type
        ///
        /// This type is intended to provide more meaningful names for the
        /// given pins.
        pub struct Pins {
            port: Option<::atsamd_hal::pac::PORT>,
            /**
    This field can also be accessed using the [`pin_alias!`] macro with the following alternate names:
        */
            ///usb_dp,
            pub testpin: ::atsamd_hal::gpio::v2::Pin<
                ::atsamd_hal::gpio::v2::PA25,
                ::atsamd_hal::gpio::v2::Reset,
            >,
        }
        impl Pins {
            /// Take ownership of the PAC [`PORT`] and split it into
            /// discrete [`Pin`]s.

            [ ... REST OMITTED ... ]
        */

    fn test_pins_autocomplete() {
        // fake a value of the correct type so we can construct the pins object
        let fake_port = unsafe { core::mem::transmute::<_, xiao_m0::pac::PORT>(()) };

        // construct an instance of the Pins type declared through the macro
        // above...
        let pins = Pins::new(fake_port);

        // BUG:
        // BUG: rust-analyzer reports the type of _pin as `unknown`,
        // BUG: but it shold be ::atsamd_hal::Pin<...>...
        // BUG:
        // BUG: rustc correctly understands the type here
        // BUG:
        // BUG:
        let _pin = pins.testpin;
    }
}

fn main() -> ! {
    loop {}
}

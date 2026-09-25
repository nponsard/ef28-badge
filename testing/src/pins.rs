use ariel_os::hal::{peripherals, spi};

ariel_os::hal::define_peripherals!(Ulp {
    ulp: ULP_RISCV_CORE,

    smart_led: GPIO1,
    boost: GPIO9,
});

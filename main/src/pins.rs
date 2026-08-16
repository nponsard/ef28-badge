use ariel_os::hal::{peripherals, spi};

ariel_os::hal::define_peripherals!(Ulp {
    ulp: ULP_RISCV_CORE,

    smart_led: GPIO21,
    boost: GPIO9,
});

ariel_os::hal::define_peripherals!(Buttons {
    right_button: GPIO8,
    left_button: GPIO2,
});

pub type EpdSpi = spi::main::SPI2;
ariel_os::hal::define_peripherals!(Epd {
    spi_miso: GPIO11, // not connected
    spi_mosi: GPIO4,
    spi_sck: GPIO5,
    spi_cs: GPIO7,
    dc: GPIO17,
    reset: GPIO6,
    busy: GPIO18,
});

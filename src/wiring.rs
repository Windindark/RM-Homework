//! Complete the two TODOs in this file.

use embassy_stm32::usart::{self, BufferedUart, Config};
use embassy_stm32::{
    Peripherals,
    can::Can,
    gpio::{Flex, Level, Output, Speed},
};
use embassy_stm32::{bind_interrupts, peripherals};
use static_cell::StaticCell;

use crate::swd::Swd;

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

bind_interrupts!(struct Irqs {
    UART7 => usart::BufferedInterruptHandler<peripherals::UART7>;
});

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: Pick two exposed 3.3 V GPIOs and pass them to Swd::new.
    // Use Output for SWCLK (initially low), and Flex for bidirectional SWDIO.
    let swclk = Output::new(p.PE9, Level::Low, Speed::Low);
    let swdio = Flex::new(p.PE13);
    let swd = Some(Swd::new(swclk, swdio));

    // TODO 2: Configure a buffered UART at 115200 baud, 8N1.
    // Identify the peripheral, RX/TX pins and interrupt from the schematic.
    // Bind the interrupt and provide static RX and TX buffers.
    static TX_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    let tx_buf = TX_BUF.init([0; 64]);
    let rx_buf = RX_BUF.init([0; 64]);
    let config = Config::default();
    let uart =
        Some(BufferedUart::new(p.UART7, p.PE7, p.PE8, tx_buf, rx_buf, Irqs, config).unwrap());

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart }, swd)
}

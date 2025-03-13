//! Playground

#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
extern crate f3;

use core::marker::PhantomData;

#[doc(hidden)]
pub use f3::itm;

pub use f3::{led, time};

use f3::peripheral;

#[export_name = "_init"]
pub unsafe fn init() {
    f3::fpu::init();
    f3::itm::init();
    f3::led::init();
    f3::time::init();
}

/// Trait for types which are a placeholder of a value that will become
/// available at possible some later point in time.
pub trait Future {
    type Item;

    /// Check if this future has completed
    fn poll(&mut self) -> Async<Self::Item>;

    /// Drive a future to its completion by continuously calling `poll`
    fn wait(mut self) -> Self::Item
        where Self: Sized
    {
        loop {
            if let Async::Ready(item) = self.poll() {
                return item;
            }
        }
    }
}

/// Return type of future, indicating whether a value is ready or not.
pub enum Async<T> {
    Ready(T),
    NotReady,
}

pub struct SerialPort {
    _0: (),
}

pub struct Serial {
    pub rx: Rx,
    pub tx: Tx,
}

impl Serial {
    pub fn new() -> Option<Self> {
        unsafe {
            static mut YIELDED: bool = false;

            if YIELDED {
                None
            } else {
                YIELDED = true;

                let gpioa = peripheral::gpioa_mut();
                let rcc = peripheral::rcc_mut();
                let usart1 = peripheral::usart1_mut();

                // RCC: Enable USART1 and GPIOC
                rcc.apb2enr.modify(|_, w| w.usart1en(true));
                rcc.ahbenr.modify(|_, w| w.iopaen(true));

                // GPIO: configure PA9 as TX and PA10 as RX
                // AFRH9: USART1_TX
                // AFRH10: USART1_RX
                gpioa.afrh.modify(|_, w| w.afrh9(7).afrh10(7));
                // MODER9: Alternate mode
                // MODER10: Alternate mode
                gpioa.moder.modify(|_, w| w.moder9(0b10).moder10(0b10));

                // USART1: 115200 - 8N1
                usart1.cr2.write(|w| w.stop(0b00));

                // Disable hardware flow control
                usart1.cr3.write(|w| w.rtse(false).ctse(false));

                const APB2_CLOCK: u32 = 8_000_000;
                const BAUD_RATE: u32 = 115_200;
                let brr = (APB2_CLOCK / BAUD_RATE) as u16;
                usart1.brr.write(|w| {
                    w.div_fraction((brr & 0b1111) as u8)
                        .div_mantissa(brr >> 4)
                });

                // UE: Enable USART
                // RE: Enable the receiver
                // TE: Enable the transmitter
                // PCE: No parity
                // OVER8: Oversampling by 16 -- to set the baud rate
                usart1.cr1.write(|w| {
                    w.ue(true)
                        .re(true)
                        .te(true)
                        .pce(false)
                        .over8(false)
                });

                Some(Serial {
                    rx: Rx { _0: () },
                    tx: Tx { _0: () },
                })
            }
        }
    }
}

pub struct Rx {
    _0: (),
}

impl Rx {
    pub fn bytes(&mut self) -> Bytes {
        Bytes { _marker: PhantomData }
    }
}

pub struct Bytes<'a> {
    _marker: PhantomData<&'a mut Rx>,
}

impl<'a> Future for Bytes<'a> {
    type Item = u8;

    fn poll(&mut self) -> Async<u8> {
        let usart1 = peripheral::usart1();

        if usart1.isr.read().rxne() {
            // unsafe { bkpt!() };
            Async::Ready(usart1.rdr.read().rdr() as u8)
        } else {
            Async::NotReady
        }
    }
}

pub struct Tx {
    _0: (),
}

impl Tx {
    pub fn write(&mut self, byte: u8) -> Write {
        Write {
            _marker: PhantomData,
            byte: byte,
            done: false,
        }
    }
}

#[must_use = "futures do nothing unless polled"]
pub struct Write<'a> {
    _marker: PhantomData<&'a mut Tx>,
    byte: u8,
    done: bool,
}

impl<'a> Future for Write<'a> {
    type Item = ();

    fn poll(&mut self) -> Async<()> {
        if self.done {
            panic!("cannot poll Write twice");
        }

        // NOTE this future owns the TDR register
        let usart1 = unsafe { peripheral::usart1_mut() };

        if usart1.isr.read().txe() {
            usart1.tdr.write(|w| w.tdr(u16::from(self.byte)));
            self.done = true;
            Async::Ready(())
        } else {
            Async::NotReady
        }
    }
}

pub struct Timer {
    _0: (),
}

impl Timer {
    pub fn new() -> Option<Self> {
        unsafe {
            static mut YIELDED: bool = false;

            if YIELDED {
                None
            } else {
                YIELDED = true;

                let rcc = peripheral::rcc_mut();
                let tim7 = peripheral::tim7_mut();

                rcc.apb1enr.modify(|_, w| w.tim7en(true));
                tim7.psc.write(|w| w.psc(7_999));

                Some(Timer { _0: () })
            }
        }
    }

    pub fn periodic(&mut self, ms: u16) -> Periodic {
        unsafe {
            let tim7 = peripheral::tim7_mut();

            tim7.arr.write(|w| w.arr(ms));
            tim7.egr.write(|w| w.ug(true));
            tim7.sr.read();
            tim7.sr.write(|w| w);
            tim7.cr1.modify(|_, w| w.opm(false).cen(true));

            Periodic { _marker: PhantomData }
        }
    }
}

#[must_use = "futures do nothing unless polled"]
pub struct Periodic<'a> {
    _marker: PhantomData<&'a mut Timer>,
}

impl<'a> Future for Periodic<'a> {
    type Item = ();

    fn poll(&mut self) -> Async<()> {
        unsafe {
            let tim7 = peripheral::tim7_mut();

            if tim7.sr.read().uif() {
                tim7.sr.write(|w| w);
                Async::Ready(())
            } else {
                Async::NotReady
            }
        }
    }
}

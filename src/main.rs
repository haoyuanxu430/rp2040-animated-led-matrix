#![no_std]
#![no_main]
extern crate alloc;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024;

/**** low-level imports *****/
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_hal::{
    digital::v2::{OutputPin},
};

/***** board-specific imports *****/
use adafruit_feather_rp2040::hal as hal;
use hal::{
    pac::interrupt,
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio,
};
use adafruit_feather_rp2040::{
    Pins, XOSC_CRYSTAL_FREQ,
};

/**** project-specific imports ****/
use ws2812_pio::Ws2812;
mod animations;
use animations::{
    RightTiltAnimation,
    LeftTiltAnimation,
    ForwardTiltAnimation,
    BackwardTiltAnimation
};
use hal::{
    pio::PIOExt,
    Timer
};
use smart_leds::{RGB8, SmartLedsWrite};
use hal::{
    I2C,
    fugit::RateExtU32
};
mod lis3dh;
use lis3dh::Lis3dh;
use micromath::F32Ext;


// USB Device support
use usb_device::class_prelude::*;
// USB Communications Class Device support
mod usb_manager;
use usb_manager::UsbManager;
// Global USB objects & interrupt
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_MANAGER: Option<UsbManager> = None;

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    match USB_MANAGER.as_mut() {
        Some(manager) => manager.interrupt(),
        None => (),
    };
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(usb) = unsafe { USB_MANAGER.as_mut() } {
        writeln!(usb, "{}", panic_info).ok();
    }
    loop {}
}

#[entry]
fn main() -> ! {
    // Grab the singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    
    // Init the watchdog timer, to pass into the clock init
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    
    // Get the clock configurations
    let mut clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    // Create a timer for WS2812
    let system_freq = clocks.system_clock.freq().to_Hz();
    let peripheral_freq = clocks.peripheral_clock.freq();
    let ws2812_timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let countdown = ws2812_timer.count_down();
    
    // Setup USB with the USB clock
    let usb = unsafe {
        USB_BUS = Some(UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        )));
        USB_MANAGER = Some(UsbManager::new(USB_BUS.as_ref().unwrap()));
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
        USB_MANAGER.as_mut().unwrap()
    };

    // initialize the Single Cycle IO
    let sio = Sio::new(pac.SIO);
    
    // initialize the pins to default state
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut timer = cortex_m::delay::Delay::new(core.SYST, system_freq);
    let mut led_pin = pins.d13.into_push_pull_output();
    
    // Enable power to NeoMatrix
    let mut power_pin = pins.d10.into_push_pull_output();
    power_pin.set_high().unwrap();

    // Configure the WS2812 LED matrix
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut ws2812 = Ws2812::new(
        pins.d5.into_function(),
        &mut pio,
        sm0,
        peripheral_freq,
        countdown
    );

    let i2c = I2C::i2c1(
        pac.I2C1,
        pins.sda.into_function(),  // Pin 2
        pins.scl.into_function(),  // Pin 3
        48_000u32.Hz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock
    );

    let mut accel = Lis3dh::new(i2c);
    accel.init().unwrap();

    let mut right_anim = RightTiltAnimation::new();
    let mut left_anim = LeftTiltAnimation::new();
    let mut forward_anim = ForwardTiltAnimation::new();
    let mut backward_anim = BackwardTiltAnimation::new();
    let mut current_led = 0;  // Keep track of which LED to light

    // Main loop
    loop {
        if let Ok((x, y, z)) = accel.read_accel() {
            // Flip signs for x and y
            let x = -x;
            let y = -y;
            
            write!(usb, "Accel: x={:.2}, y={:.2}, z={:.2}\r\n", x, y, z).unwrap();
            
            let pixels = if x.abs() > y.abs() {
                if x > 0.2 {
                    // Tilt right
                    let pixels = right_anim.to_list();
                    right_anim.next();
                    pixels
                } else if x < -0.2 {
                    // Tilt left
                    let pixels = left_anim.to_list();
                    left_anim.next();
                    pixels
                } else {
                    [RGB8::default(); 64]
                }
            } else {
                if y > 0.2 {
                    // Tilt forward
                    let pixels = forward_anim.to_list();
                    forward_anim.next();
                    pixels
                } else if y < -0.2 {
                    // Tilt backward
                    let pixels = backward_anim.to_list();
                    backward_anim.next();
                    pixels
                } else {
                    [RGB8::default(); 64]
                }
            };
            
            ws2812.write(pixels.iter().cloned()).unwrap();
        }
        
        timer.delay_ms(50);  // Faster update rate for smoother animations
    }
}
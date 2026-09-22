// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
// Sigh. The check config system requires that the compiler be told what possible config values
// there might be.  This is completely impossible with both Kconfig and the DT configs, since the
// whole point is that we likely need to check for configs that aren't otherwise present in the
// build.  So, this is just always necessary.
#![allow(unexpected_cfgs)]

use log::{error, info};

use zephyr::device::gpio::GpioPin;
use zephyr::raw::ZR_GPIO_OUTPUT_ACTIVE;
use zephyr::time::{sleep, Duration};

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    info!("Starting blinky");

    do_blink();
}

fn do_blink() {
    info!("Inside of blinky");

    let mut led0 = zephyr::devicetree::aliases::led0::get_instance().unwrap();
    let mut gpio_token = unsafe { zephyr::device::gpio::GpioToken::get_instance().unwrap() };

    if !led0.is_ready() {
        error!("LED is not ready");
        loop {}
    }

    unsafe {
        led0.configure(&mut gpio_token, ZR_GPIO_OUTPUT_ACTIVE);
    }
    let duration = Duration::millis_at_least(250);
    loop {
        let state = if unsafe { led0.get(&mut gpio_token) } {
            "ON"
        } else {
            "OFF"
        };
        info!("Toggle. LED is now {}.", state);
        unsafe {
            led0.toggle_pin(&mut gpio_token);
        }
        sleep(duration);
    }
}

#![no_std]
#![no_main]

use bootloader_api::{config::BootloaderConfig, entry_point, BootInfo};
use kernel::hlt_loop;

pub static BOOTLOADER_CONFIG: BootloaderConfig = BootloaderConfig::new_default();

// add a `config` argument to the `entry_point` macro call
entry_point!(kmain, config = &BOOTLOADER_CONFIG);

fn kmain(_boot_info: &'static mut BootInfo) -> ! {
    hlt_loop();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    hlt_loop();
}

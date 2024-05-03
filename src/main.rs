// main.rs
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("GreenFN");


    loop {
        unsafe {
            asm!(
                "cli",
                "hlt",
                options(nomem, nostack)
            );
        }
        
    }
    panic!("Some Error Code");

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}

//cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
//qemu-system-x86_64 -drive format=raw,file=target/x86_64-noggin_os/debug/bootimage-noggin_os.bin
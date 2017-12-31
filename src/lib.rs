#![feature(lang_items, const_fn, unique, const_unique_new)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("ooooohaiii");
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

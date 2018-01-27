#![feature(lang_items, const_fn, unique, const_unique_new)]
#![no_std]

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;
mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_info: usize) {
    // ATTENTION: we have no guard page

    vga_buffer::clear_screen();

    let boot_info = unsafe{ multiboot2::load(multiboot_info) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    let elf_sections = boot_info.elf_sections_tag().expect("elf sections must be present");
    println!("kernel sections:");
    for section in elf_sections.sections() {
        println!("    start: 0x{:x}, size: 0x{:x} with flags:0x{:x}", section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    
    let multiboot_start = multiboot_info;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel address space: 0x{:x} to 0x{:x}", kernel_start, kernel_end);
    println!("multiboot info space: 0x{:x} to 0x{:x}", multiboot_start, multiboot_end);

    use memory::FrameAllocator;
    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());
    println!("{:?}", frame_allocator.allocate_frame());
    
    
    println!("ooooohaiii {}", {println!("asdasdad"); "qweweqweqwew"});

    loop{}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC!!!!111!1!!!one1! in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

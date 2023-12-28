#![no_std]
#![no_main]
mod lang_items;

//fn main() {
//        println!("Hello, world!");
//}

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> !{
    clear_bss();
    loop{}
}

//此处进行对.bss段的清零 我们需要保证被分配到.bss段的全局变量为0
//由于控制权已被转交给Rust,我们可以使用Rust来实现这一功能
fn clear_bss(){
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe{(a as *mut u8).write_volatile(0)}
    });
}


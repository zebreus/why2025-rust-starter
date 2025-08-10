#![feature(linkage)]
#![no_std]
#![no_main]

// TODO: Rewrite example to do something more interesting

unsafe extern "C" {
    fn printf(format: *const u8, ...) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    unsafe {
        printf(b"Hello, world! (from rust)\n\0".as_ptr());
    }
    let _ = sdl_main();
    121
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use core::ffi::CStr;
use sdl3_sys::{
    error::SDL_GetError,
    init::{SDL_Init, SDL_INIT_VIDEO},
    render::{SDL_CreateRenderer, SDL_RenderClear, SDL_RenderPresent, SDL_SetRenderDrawColor},
    timer::SDL_Delay,
    video::SDL_CreateWindow,
};

fn sdl_main() -> Result<(), &'static CStr> {
    if unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        unsafe {
            let window = SDL_CreateWindow(
                core::mem::transmute("title".as_bytes().as_ptr()),
                160,
                160,
                0,
            );

            let renderer = SDL_CreateRenderer(window, core::ptr::null_mut());
            loop {
                for i in 0..255 {
                    let (r, g, b, a) = (i, 0, 0, 255);
                    SDL_SetRenderDrawColor(renderer, r, g, b, a);
                    SDL_RenderClear(renderer);
                    SDL_RenderPresent(renderer);
                    SDL_Delay(50);
                }
            }
        };
    } else {
        Err(unsafe { CStr::from_ptr(SDL_GetError()) })
    }
}
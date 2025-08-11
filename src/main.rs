#![no_std]
#![no_main]

use core::ffi::CStr;
use sdl3_sys::{
    error::SDL_GetError,
    init::{SDL_INIT_VIDEO, SDL_Init},
    render::{SDL_CreateRenderer, SDL_RenderClear, SDL_RenderPresent, SDL_SetRenderDrawColor},
    timer::SDL_Delay,
    video::SDL_CreateWindow,
};
use why2025_badge_sys::printf;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// TODO: Rewrite example to do something more interesting

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    unsafe {
        printf(b"Hello, world! (from rust)\n\0".as_ptr());
    }
    sdl_main();
    0
}

fn sdl_main() {
    unsafe {
        if !SDL_Init(SDL_INIT_VIDEO) {
            let message = CStr::from_ptr(SDL_GetError());
            let mes = message.to_str().unwrap();
            panic!("{mes}");
        }
        let window = SDL_CreateWindow(
            core::mem::transmute("title".as_bytes().as_ptr()),
            300,
            300,
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
}

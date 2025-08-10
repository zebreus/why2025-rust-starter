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
        // println!("Successfully initialized SDL!");

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

            // SDL_Quit();
        };
        Ok(())
    } else {
        Err(unsafe { CStr::from_ptr(SDL_GetError()) })
    }
}

//pub fn sdl_main() {
//    let sdl_context = sdl3::init().unwrap();
//    let video_subsystem = sdl_context.video().unwrap();
//
//    let window = video_subsystem
//        .window("rust-sdl3 demo", 800, 600)
//        .position_centered()
//        .build()
//        .unwrap();
//
//    let mut canvas = window.into_canvas();
//
//    canvas.set_draw_color(Color::RGB(0, 255, 255));
//    canvas.clear();
//    canvas.present();
//    let mut event_pump = sdl_context.event_pump().unwrap();
//    let mut i = 0;
//    'running: loop {
//        i = (i + 1) % 255;
//        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//        canvas.clear();
//        for event in event_pump.poll_iter() {
//            match event {
//                Event::Quit { .. }
//                | Event::KeyDown {
//                    keycode: Some(Keycode::Escape),
//                    ..
//                } => break 'running,
//                _ => {}
//            }
//        }
//        // The rest of the game loop goes here...
//
//        canvas.present();
//        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//    }
//}
//

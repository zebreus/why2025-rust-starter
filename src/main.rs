#![no_std]
#![no_main]
extern crate alloc;

use embedded_graphics::prelude::*;
use mousefood::{TerminalAlignment, prelude::*};
use ratatui::{Frame, Terminal, widgets::Paragraph};
use why2025_badge_embedded_graphics::Why2025BadgeWindow;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    unsafe {
        why2025_badge_sys::printf(b"Hello, world! (from rust)\n\0".as_ptr());
    }
    let mut display = Why2025BadgeWindow::new_floating(
        Size {
            width: 400,
            height: 400,
        },
        "Mousefood Demo",
    );

    let config = EmbeddedBackendConfig {
        flush_callback: alloc::boxed::Box::new(|d: &mut Why2025BadgeWindow| {
            d.flush();
        }),
        font_regular: embedded_graphics_unicodefonts::MONO_9X15,
        font_bold: Some(embedded_graphics_unicodefonts::MONO_9X15_BOLD),
        font_italic: None,
        vertical_alignment: TerminalAlignment::Center,
        horizontal_alignment: TerminalAlignment::Center,
    };
    let backend = EmbeddedBackend::new(&mut display, config);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(draw).unwrap();
    }
}

/// Render the application. This is where you would draw the application UI. This example draws a
/// greeting.
fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World!");

    frame.render_widget(greeting, frame.area());
}

// Allocator and panic handler setup
use talc::{ClaimOnOom, Span, Talc, Talck};

const HEAP_SIZE: usize = 1024 * 300; // 300KB heap size
static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];
#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> =
    Talc::new(unsafe { ClaimOnOom::new(Span::from_array((&raw const HEAP).cast_mut())) }).lock();

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let maybe_msg = alloc::string::ToString::to_string(&panic_info.message());
        let msg = maybe_msg.as_ptr();
        why2025_badge_sys::printf(b"panic: %s\n\0".as_ptr(), msg);
        if let Some(location) = panic_info.location() {
            why2025_badge_sys::printf(
                b"in %s:%d\n\0".as_ptr(),
                location.file().as_ptr(),
                location.line() as i32,
            );
        } else {
            why2025_badge_sys::printf(b"no location information available :(\n\0".as_ptr());
        }
    }
    loop {}
}

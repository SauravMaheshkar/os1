//! # Basic Logger that writes to serial and frame buffer
//!
//! The `logger` module initializes the logger with the given frame buffer.
//! Fully based on the [logger implementation from the bootloader project](https://github.com/rust-osdev/bootloader/blob/main/common/src/logger.rs).

use bootloader_api::info::FrameBufferInfo;
use bootloader_boot_config::LevelFilter;
use bootloader_x86_64_common::init_logger;

/// Initializes the logger with the given frame buffer information, buffer, and
/// logger status.
///
/// # Arguments
/// * `info` - The [`FrameBufferInfo`] struct
/// * `buffer` - The frame buffer as a raw byte slice of type `&'static mut
///   [u8]`
/// * `frame_buffer_logger_status` - A boolean that determines if the frame
///   buffer
/// logger should be enabled
/// * `serial_logger_status` - A boolean that determines if the serial logger
/// should be enabled
pub fn init(
    info: FrameBufferInfo,
    buffer: &'static mut [u8],
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    init_logger(
        buffer,
        info,
        LevelFilter::Warn,
        frame_buffer_logger_status,
        serial_logger_status,
    );

    log::info!("Logger initialized");
    log::info!(
        r#"
        ,-~~-.___.
       / |  '     \         It was a dark and stormy night....
      (  )         0
       \_/-, ,----'
          ====           //
         /  \-'~;    /~~~(O)
        /  __/~|   /       |
      =(  _____| (_________|   W<
        "#
    );
}

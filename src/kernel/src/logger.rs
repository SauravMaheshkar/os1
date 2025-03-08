use bootloader_api::info::FrameBufferInfo;
use bootloader_boot_config::LevelFilter;
use bootloader_x86_64_common::init_logger;

pub fn init(
    info: FrameBufferInfo,
    buffer: &'static mut [u8],
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    init_logger(
        buffer,
        info,
        LevelFilter::Info,
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

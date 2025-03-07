use bootloader_api::BootInfo;
use bootloader_boot_config::LevelFilter;
use bootloader_x86_64_common::init_logger;

pub fn init(
    info: &'static mut BootInfo,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let framebuffer = info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

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

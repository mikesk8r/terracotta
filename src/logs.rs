pub fn setup_logger() -> Result<(), fern::InitError> {
    let dispatch = fern::Dispatch::new().format(|out, message, record| {
        let color: &'static str;
        match record.level() {
            log::Level::Info => color = "\x1b[34;1m",
            log::Level::Warn => color = "\x1b[33;1m",
            log::Level::Error => color = "\x1b[31;1m",
            log::Level::Debug => color = "\x1b[32;1m",
            log::Level::Trace => color = "\x1b[35;1m",
        }
        out.finish(format_args!(
            "\x1b[0;1m[{} {}{}\x1b[0;1m {}]\x1b[0;0m {}",
            chrono::Local::now().format("%H:%H:%S"),
            color,
            record.level(),
            record.target().split("::").min().unwrap(),
            message
        ))
    });

    #[cfg(not(debug_assertions))]
    dispatch
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    #[cfg(debug_assertions)]
    dispatch.chain(std::io::stdout()).apply()?;

    Ok(())
}

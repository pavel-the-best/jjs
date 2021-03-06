use anyhow::Context;
use slog_scope::debug;

fn install_color_backtrace() {
    #[cfg(feature = "beautiful_backtrace")]
    color_backtrace::install();
}

fn is_cli_mode() -> bool {
    std::env::args().count() > 1
}

fn make_sources(
    cfg_data: &util::cfg::CfgData,
) -> anyhow::Result<Vec<Box<dyn invoker::controller::TaskSource>>> {
    let mut sources: Vec<Box<dyn invoker::controller::TaskSource>> = Vec::new();
    if is_cli_mode() {
        let source = invoker::sources::CliSource::new();
        //let driver = invoker::drivers::CliDriver::new().context("failed to setup CLI Controller Driver")?;
        sources.push(Box::new(source));
    } else {
        let db_conn = db::connect_env().context("db connection failed")?;
        let source = invoker::sources::DbSource::new(db_conn, cfg_data);
        sources.push(Box::new(source))
    }
    Ok(sources)
}

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    if atty::is(atty::Stream::Stderr) {
        install_color_backtrace();
    }
    util::log::setup();
    util::wait::wait();

    let config = util::cfg::load_cfg_data()?;

    invoker::init::init().context("failed to initialize")?;

    //check_system().context("system configuration problem")?;
    debug!("system check passed");

    let backend = minion::setup();
    let driver = make_sources(&config).context("failed to initialize driver")?;
    let controller = invoker::controller::Controller::new(driver, backend.into(), config, 3)
        .context("failed to start controller")?;

    util::daemon_notify_ready();
    controller.run_forever();
    Ok(())
}

use crate as vanessa;

/// This just checks to make sure we don't accidentally
/// break logger macro usage.
/// This is how the logger will be used.
#[test]
fn logger_interface() {
    use vanessa::log::*;
    init();
    let logger2 = Logger::new("", LogLevel::HYPER, LogLevel::HYPER);
    vanessa::hyper!("Hyper log.");
    vanessa::shyper!(logger2, "Hyper log.");
    vanessa::debug!("Debug log.");
    vanessa::sdebug!(logger2, "Debug log.");
    vanessa::info!("Info log.");
    vanessa::sinfo!(logger2, "Info log.");
    vanessa::curio!("Curio log.");
    vanessa::scurio!(logger2, "Curio log.");
    vanessa::ok!("Ok log.");
    vanessa::sok!(logger2, "Ok log.");
    vanessa::warn!("Warn log.");
    vanessa::swarn!(logger2, "Warn log.");
    vanessa::error!("Error log.");
    vanessa::serror!(logger2, "Error log.");
    vanessa::fatal!("Fatal log.");
    vanessa::sfatal!(logger2, "Fatal log.");
}

/// This makes sure we don't accidentally break time usage.
/// This is how the time system will be used.
#[test]
fn time_interface() {
    use vanessa::time::*;

    let now = epoch_millis();
    epoch_days(now);
    epoch_months(now);
    epoch_years(now);
    current_month(now);
    date(now);
    hour(now);
    minute(now);
    second(now);
    timestamp(now);
    timestamp_now();
    epoch_nanos();
}

#[test]
/// This makes sure we don't accidentally break worker usage.
fn worker_interface() {
    use vanessa::worker::*;
    init();
    bg(|| {});
    shutdown_blocking();
}

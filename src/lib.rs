#[cfg(test)]
mod test;

/// Initialize every single component of the Vanessa runtime.
pub fn full_init() {
    crate::sdebug!(log::VANESSA_LOGGER, "Initializing vanessa::log");
    log::init();
}

/// This module provides the logging facilities of the Vanessa Runtime.
pub mod log;

/// This module deals with handling time. It does not have an initialization
/// step.
pub mod time;

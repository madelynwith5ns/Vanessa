# Vanessa

Vanessa is a utility library for Rust programs.

It provides an extremely easy logger and dead-simple threading.

### Logging

Logging is super simple, initialize it at the start of your program
and you can use the various log level macros anywhere in your program!
You can also create new loggers and log to them with the `s` macros
(like `sinfo!` or `sdebug!`). If you want to save multiple log files
instead of only storing the latest log, enable the `multilog` feature.

Loggers have 7 levels here:

- Hyper: Hyper is for really spammy debug messaging. The default logger
and any logger from `Logger::quick` has the log level set too high
for these.

- Debug: Debug is for debug messaging (obviously). The default logger
and any logger from `Logger::quick` will only show these when compiling
in debug mode.

- Info: Normal info messages.

- Warn: Warnings.

- Error: Errors.

- Fatal: Critical errors, this is for full-program crashes or other
similarly critical failures.

- Input: This is a special log level used to get input from the user.
Its macro returns an `Option<String>`.

```rust
use vanessa::{info,sinfo};

fn main() {
    // call this somewhere at the start of your program
    vanessa::log::init();
    // or just call vanessa::full_init() to init everything!

    info!("Hello World!");

    let logger2 = vanessa::log::Logger::quick("Another Logger");
    sinfo!(logger2, "You can also log to specific loggers which can have their own log levels!");
}
```

### Threading

Concurrency is done via background workers. Call the init function
at the start of your program and you can call the `bg` function from
anywhere to run a closure in the background!

The `workers` feature flag is enabled by default.

```rust
use vanessa::worker::bg;

fn main() {
    // call this somewhere at the start of your program
    vanessa::worker::init();
    // or just call vanessa::full_init() to init everything!
    // you can also call vanessa::worker::init_with(usize)
    // to initialize with a specified number of threads.

    bg(||{
        // background tasks!
    });
}
```

If you need to wait for the completion of a job, you can `require` it:

```rust
let handle = bg(||{ /* some heavy operation here */ }).unwrap();
/* do some other stuff while you wait */
bg.require(); // wait for the job to finish
```

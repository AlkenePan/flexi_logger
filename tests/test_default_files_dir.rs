extern crate flexi_logger;
#[macro_use]
extern crate log;

use flexi_logger::Logger;

#[test]
fn files_dir() {
    assert_eq!((),
               Logger::with_str("info")
                   .log_to_file()
                   .directory("log_files")
                   .start()
                   .unwrap());

    error!("This is an error message");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message - you must not see it!");
    trace!("This is a trace message - you must not see it!");
}
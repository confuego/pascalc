use log::{Level, debug, error, info, log_enabled};
mod reserved_words;

fn main() {
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
    info!("Hello, world!");
    println!(
        "is {} a reserved word? {}",
        "HELLO",
        reserved_words::is_reserved("HELLO")
    );
    println!(
        "is {} a reserved word? {}",
        "eNd",
        reserved_words::is_reserved("eNd")
    );
}

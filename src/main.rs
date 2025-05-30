// mod lexer;
// pub use crate::lexer::reserved_words;
mod cli;

fn main() {
    env_logger::init();
    cli::start();
}

use std::panic;

mod app;
mod export;

fn main() {
    let result = panic::catch_unwind(|| {
        if let Err(error) = app::run() {
            eprintln!("{}", error);
        }
    });

    if result.is_err() {
        eprintln!("An unrecoverable error occurred");
    }
}

use std::panic;

use shketch::app;

fn main() {
    let result = panic::catch_unwind(|| {
        if let Err(error) = app::run() {
            eprintln!("{}", error);
        }
    });

    if let Err(panic) = result {
        match panic.downcast::<String>() {
            Ok(error) => eprintln!("Unexpected error: {}", error),
            _ => eprintln!("An unknown error occurred"),
        }
    }
}

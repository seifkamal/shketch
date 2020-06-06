use std::panic;

use shketch::app;

fn main() {
    let directions = ["up", "down", "left", "right", "backward_diagonal", "forward_diagonal"];

    let matches = clap::App::new("Shketch")
        .version("0.1.0")
        .about("An ASCII drawing tool")
        .before_help(
            r#"
 ______                 ________________
|        |    |  |     /       |  |       |    |
|______  |____|  |____/_____   |  |       |____|
     /  /    /  /     \        |  |      /    /
____/  /    /  /       \____   |  \_____/    /
        "#,
        )
        .after_help("Run to start drawing on a new canvas")
        .args(
            &directions
                .iter()
                .map(|name| {
                    clap::Arg::with_name(name)
                        .short(&name[0..])
                        .help("Cursor character for this direction")
                        .takes_value(true)
                        .validator(|c| {
                            if c.len() > 1 {
                                Err("Cannot use more than 1 character per direction".into())
                            } else {
                                Ok(())
                            }
                        })
                })
                .collect::<Vec<clap::Arg>>(),
        )
        .get_matches();

    let char_set = {
        let mut set = terminal::grid::CharSet::default();
        directions.iter().for_each(|direction| {
            if let Some(value) = matches.value_of(direction) {
                *(match *direction {
                    "up" => &mut set.up,
                    "down" => &mut set.down,
                    "left" => &mut set.left,
                    "right" => &mut set.right,
                    "backward_diagonal" => &mut set.backward_diagonal,
                    "forward_diagonal" => &mut set.forward_diagonal,
                    _ => unreachable!(),
                }) = value.as_bytes()[0] as char;
            }
        });
        set
    };

    let result = panic::catch_unwind(|| {
        if let Err(error) = app::launch(app::Opts::new(char_set)) {
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

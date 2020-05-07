use clap;
use ward;

fn main() {
    let matches = clap::App::new("Ward")
        .arg(
            clap::Arg::with_name("mode")
                .possible_values(&["free", "line"])
                .required(true)
                .takes_value(true)
                .max_values(1)
        )
        .get_matches();

    match matches.value_of("mode") {
        Some("free") => start(ward::paint::FreeBrush::new()),
        Some("line") => start(ward::paint::LineBrush::new()),
        _ => unreachable!()
    }
}

fn start<B: ward::paint::Brush>(brush: B) {
    let mut canvas = ward::Canvas::default();
    canvas.clear();
    canvas.start(brush);
}

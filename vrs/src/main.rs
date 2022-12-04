use {
    clap::Parser,
    macroquad::prelude::*,
    std::path::PathBuf,
};

#[derive(Debug, Parser)]
struct CliArgs {
    novel: PathBuf,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "VRS Interpreter".to_owned(),
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args = CliArgs::parse();
    loop {
        clear_background(WHITE);

        next_frame().await;
    }
}

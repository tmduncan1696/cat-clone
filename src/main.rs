use std::error::Error;
use cat_clone::{cli::Cli, run};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::new();

    run(cli)
}


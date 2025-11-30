use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(name = "Cat Clone")]
pub struct Cli {
    #[arg(value_parser, num_args = 1.., value_delimiter = ' ')]
    pub files: Vec<String>,

    #[arg(short = 'E', long, action = ArgAction::SetTrue)]
    pub show_ends: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub number: bool,

    #[arg(short = 'b', long, action = ArgAction::SetTrue)]
    pub number_nonblank: bool,

    #[arg(short = 'T', long, action = ArgAction::SetTrue)]
    pub show_tabs: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub squeeze_blanks: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(name = "Cat Clone", about = "Concatenate FILES to standard output")]
pub struct Cli {
    #[arg(value_parser, num_args = 0.., value_delimiter = ' ')]
    pub files: Vec<String>,

    #[arg(
        short = 'A',
        long,
        action = ArgAction::SetTrue,
        help = "equivalent to -ET"
    )]
    pub show_all: bool,

    #[arg(
        short = 'E',
        long,
        action = ArgAction::SetTrue,
        default_value_if("show_all", "true", "true"),
        help = "display $ at the end of each line"
    )]
    pub show_ends: bool,

    #[arg(
        short,
        long,
        action = ArgAction::SetTrue,
        help = "number all output lines"
    )]
    pub number: bool,

    #[arg(
        short = 'b',
        long,
        action = ArgAction::SetTrue,
        help = "number non-empty output lines, overrides -n"
    )]
    pub number_nonblank: bool,

    #[arg(
        short = 'T',
        long,
        action = ArgAction::SetTrue,
        default_value_if("show_all", "true", "true"),
        help = "display TAB characters as ^I"
    )]
    pub show_tabs: bool,

    #[arg(
        short,
        long,
        action = ArgAction::SetTrue,
        help = "suppress repeated empty output lines"
    )]
    pub squeeze_blanks: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

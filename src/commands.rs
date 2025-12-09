use std::{fs, io, error::Error, process};

use crate::cli::Cli;

#[derive(Debug, PartialEq)]
pub enum Command {
    SqueezeBlanks,
    ShowNonblankLineNumbers,
    ShowLineNumbers,
    ShowTabs,
    ShowEnds,
}

#[derive(Debug, PartialEq)]
pub struct CatCommands {
    lines: Vec<String>,
    commands: Vec<Command>
}

impl CatCommands {
    pub fn from_cli(cli: &Cli) -> Self {
        let lines: Vec<String> = if cli.files.is_empty() {
            io::stdin().lines().map(|x| x.unwrap()).collect::<Vec<String>>()
        } else {
            get_lines_from_files(&cli.files).unwrap_or_else(|_err| {
                eprintln!("Cannot read files: {}", cli.files.join(", "));
                process::exit(1);
            })
        };

        let mut commands = Vec::new();

        if cli.squeeze_blanks {
            commands.push(Command::SqueezeBlanks)
        };
        if cli.number_nonblank {
            commands.push(Command::ShowNonblankLineNumbers)
        };
        if cli.number && !cli.number_nonblank {
            commands.push(Command::ShowLineNumbers)
        };
        if cli.show_tabs {
            commands.push(Command::ShowTabs)
        };
        if cli.show_ends {
            commands.push(Command::ShowEnds)
        };

        CatCommands {
            lines,
            commands
        }
    }

    pub fn modify_lines(self: CatCommands) -> CatCommands {
        let mut lines = self.lines.clone();
        for command in &self.commands {
            lines = match command {
                Command::SqueezeBlanks => squeeze_blanks(lines),
                Command::ShowNonblankLineNumbers => show_nonblank_line_numbers(lines),
                Command::ShowLineNumbers => show_line_numbers(lines),
                Command::ShowTabs => show_tabs(lines),
                Command::ShowEnds => show_ends(lines),
            };
        };

        CatCommands {
            lines,
            ..self
        }

    }

    pub fn print_lines(self: &CatCommands) -> () {
        for line in &self.lines {
            println!("{line}")
        };
        ()
    }
}

fn get_lines_from_files(files: &Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let contents: String = files.into_iter().map(|file| read_file(file)).collect::<Result<String, _>>()?;
    Ok(contents.lines().map(|s| s.to_string()).collect())
} 

fn read_file(file: &String) -> Result<String, Box<dyn Error>> {
    if *file == String::from("-") {
        Ok(io::stdin().lines().collect::<Result<String, _>>()? + "\n")
    } else {
        Ok(fs::read_to_string(file)?)
    }
}

fn squeeze_blanks(lines: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = lines;
    while out.join("\n").contains("\n\n\n") {
        out = out.join("\n").replace("\n\n\n", "\n\n").lines().map(|x| x.to_string()).collect();
    };
    out
}

fn show_line_numbers(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().enumerate().map(|(index, line)| "     ".to_string() + &(index + 1).to_string() + "  " + &line).collect()
}

fn show_nonblank_line_numbers(lines: Vec<String>) -> Vec<String> {
    let mut line_number: i32 = 0;
    let mut out: Vec<String> = Vec::new();

    for line in lines.clone().into_iter() {
        let line = if line.is_empty() {
            line
        } else {
            line_number += 1;
            "     ".to_string() + &line_number.to_string() + "  " + &line
        };
        out.push(line);
    }
    out
}

fn show_tabs(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().map(|line| line.replace("\t", "^I")).collect()
}

fn show_ends(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().map(|line| line + "$").collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static TEST_DATA: Lazy<String> = Lazy::new(|| {
        "\
test
test

test\ttest


test".to_string()
    });

    #[test]
    fn test_squeeze_blanks() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let out: Vec<String> = squeeze_blanks(lines);

        assert_eq!(
            out,
            vec!["test", "test", "", "test\ttest", "", "test"]
        );
    }

    #[test]
    fn test_show_line_numbers() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let out: Vec<String> = show_line_numbers(lines);

        assert_eq!(
            out,
            vec!["     1  test", "     2  test", "     3  ", "     4  test\ttest", "     5  ", "     6  ", "     7  test"]
        );
        
    }

    #[test]
    fn test_show_nonblank_line_numbers() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let out: Vec<String> = show_nonblank_line_numbers(lines);

        assert_eq!(
            out,
            vec!["     1  test", "     2  test", "", "     3  test\ttest", "", "", "     4  test"]
        );
    }

    #[test]
    fn test_show_tabs() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let out: Vec<String> = show_tabs(lines);

        assert_eq!(
            out,
            vec!["test", "test", "", "test^Itest", "", "", "test"]
        );
    }

    #[test]
    fn test_show_ends() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let out: Vec<String> = show_ends(lines);

        assert_eq!(
            out,
            vec!["test$", "test$", "$", "test\ttest$", "$", "$", "test$"]
        );
    }

    #[test]
    fn test_modify_lines() {
        let lines: Vec<String> = TEST_DATA.lines().map(|x| x.to_string()).collect();

        let commands: Vec<Command> = vec![Command::SqueezeBlanks, Command::ShowNonblankLineNumbers, Command::ShowTabs, Command::ShowEnds];

        let cat_commands: CatCommands = CatCommands {
            lines,
            commands
        };

        let cat_commands = cat_commands.modify_lines();

        assert_eq!(
            cat_commands,
            CatCommands {
                lines: vec!["     1  test$".to_string(), "     2  test$".to_string(), "$".to_string(), "     3  test^Itest$".to_string(), "$".to_string(), "     4  test$".to_string()],
                commands: vec![Command::SqueezeBlanks, Command::ShowNonblankLineNumbers, Command::ShowTabs, Command::ShowEnds]
            }
        );
    }

}

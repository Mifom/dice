use rustyline::{history::MemHistory, Config, Editor};

use crate::parser::roll;

mod parser;
mod roll;

fn main() {
    let mut rl = Editor::<(), _>::with_history(
        Config::builder().auto_add_history(true).build(),
        MemHistory::new(),
    )
    .unwrap();
    while let Ok(line) = rl.readline(">> ") {
        match roll(&line) {
            Ok((rest, roll)) => {
                if rest.is_empty() {
                    let result = roll.roll();
                    println!("{} => {} => {}", roll, result, result.calc());
                } else {
                    eprintln!("Stopped parsing at `{rest}`")
                }
            }
            Err(e) => {
                eprintln!(
                    "Stopped parsing at `{}`.\nParsing failed when trying to parse {}",
                    e.input,
                    e.code.description()
                );
            }
        }
    }
}

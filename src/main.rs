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
                    println!("{} => {}", roll, roll.roll());
                } else {
                    eprintln!("Result is `{roll}` but tail is `{rest}`")
                }
            }
            Err(e) => eprintln!("Parse error: {e:?}"),
        }
    }
}

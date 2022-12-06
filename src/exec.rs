//! Execution functions

use crate::{
    command::{Command, OutputFormat},
    helper::CliHelper,
    print_options::PrintOptions,
};
use datafusion::error::Result;
use datafusion::prelude::SessionContext;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

/// run and execute SQL statements and commands against a context with the given print options
pub async fn exec_from_repl(
    ctx: &mut SessionContext,
    print_options: &mut PrintOptions,
) -> rustyline::Result<()> {
    let mut rl = Editor::<CliHelper>::new()?;
    rl.set_helper(Some(CliHelper::default()));
    rl.load_history(".history").ok();

    let mut print_options = print_options.clone();

    loop {
        match rl.readline("❯ ") {
            Ok(line) => {
                rl.add_history_entry(line.trim_end());
                match exec_and_print(ctx, &print_options, line).await {
                    Ok(_) => {}
                    Err(err) => eprintln!("{:?}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\\q");
                break;
            }
            Err(err) => {
                eprintln!("Unknown error happened {:?}", err);
                break;
            }
        }
    }

    rl.save_history(".history")
}

async fn exec_and_print(
    ctx: &mut SessionContext,
    print_options: &PrintOptions,
    sql: String,
) -> Result<()> {
    let now = Instant::now();
    let df = ctx.sql(&sql).await?;
    let results = df.collect().await?;
    print_options.print_batches(&results, now)?;

    Ok(())
}

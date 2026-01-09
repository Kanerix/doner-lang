use std::path::PathBuf;

// use doner_eval::eval;
use doner_lexer::lex;
use doner_parser::parse;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::GlobalArgs;
use crate::error;

#[derive(clap::Parser, Debug)]
pub struct RunArgs {
    /// The file to run
    file: String,
}

/// Run a döner lang program.
pub async fn run(run: RunArgs, _global_args: Box<GlobalArgs>) -> error::Result<()> {
    let path = PathBuf::from(run.file);
    let mut file = File::open(path).await?;

    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;

    let mut lines = buf.lines();

    while let Some(line) = lines.next() {
        let lex = lex(&line)?;
        let ast = parse(&lex)?;
        println!("{:?}", ast);
        // let result = eval(ast)?;
    }

    Ok(())
}

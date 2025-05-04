use std::env;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use truck::parser::Parser;
use truck::eval;
use truck::types;
use truck::utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = {
        let mut args = env::args();
        args.next();
        args.next().ok_or(Error::new(ErrorKind::NotFound, "usage: cargo run --bin interp <filename>"))?
    };

    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;

    eprintln!("file contents = {:?}", contents);
    let mut e = Parser::new(&contents[..])
        .parse()
        .map_err(|err| Error::new(ErrorKind::Other, format!("{:?}", err)))?;

    types::Context::default()
        .type_expr(&mut e)
        .map_err(|err| Error::new(ErrorKind::Other, format!("{:?}", err)))?;

    eval::Context::default()
        .eval_expr(&e, Lifetime::global());

    Ok(())
}

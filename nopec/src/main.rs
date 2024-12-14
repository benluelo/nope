use std::{path::PathBuf, time::Instant};

use anyhow::Context;
use bytecode::Bytecode;
use clap::{Parser, Subcommand};
use vm::Vm;

#[derive(Parser)]
pub struct App {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Compile the provided nopelang script, writing the resulting bytecode to
    /// the specified output file.
    Build {
        path: PathBuf,
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
    /// Run a compiled nopelang bytecode object file, optionally specifying the
    /// gas limit for the execution.
    Eval {
        path: PathBuf,
        /// The gas limit for the execution of the bytecode.
        #[arg(long, default_value_t = u32::MAX)]
        max_gas: u32,
    },
}

fn main() -> anyhow::Result<()> {
    let app = App::parse();

    match app.cmd {
        Cmd::Build { path, output } => build(path, output),
        Cmd::Eval { path, max_gas } => eval(path, max_gas),
    }
}

fn build(path: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    let now = Instant::now();

    let input = std::fs::read_to_string(path).context("unable to read input file")?;

    let ast = lang::parse(&input).context("unable to parse input file")?;

    let bytecode = compiler::compile(ast);

    std::fs::write(&output, bytecode.compile()).context("unable to write output file")?;

    let elapsed = now.elapsed();

    println!(
        "compiled {} in {:.2}s",
        output.display(),
        elapsed.as_secs_f64(),
    );

    Ok(())
}

fn eval(path: PathBuf, max_gas: u32) -> Result<(), anyhow::Error> {
    let input = std::fs::read(path).context("unable to read input file")?;

    let bytecode = Bytecode::parse(input).context("unable to parse bytecode")?;

    let mut vm = Vm::new(max_gas);

    vm.run(bytecode).context("error executing bytecode")?;

    let gas_used = vm.gas_used();

    println!("execution successful, gas used: {gas_used}");

    Ok(())
}

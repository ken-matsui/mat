mod error;
mod hir;
mod parser;
mod sema;

use crate::error::Emit;
use clap::{ArgGroup, Parser};
use std::fs::read_to_string;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("dumps")
        .args(&[
            "dump-tokens",
            "dump-ast",
            "dump-ref",
            "dump-sema",
            "dump-mir",
            "dump-asm",
            "print-asm"
        ]),
))]
struct Args {
    /// Source file to compile
    source: String,

    /// Dump tokens
    #[clap(long)]
    dump_tokens: bool,

    /// Dump AST
    #[clap(long)]
    dump_ast: bool,

    /// Dump HIR
    #[clap(long)]
    dump_hir: bool,

    /// Dump MIR
    #[clap(long)]
    dump_mir: bool,

    /// Dump structured assembly
    #[clap(long)]
    dump_asm: bool,

    /// Print raw assembly
    #[clap(long)]
    print_asm: bool,
}

fn main() {
    let args = Args::parse();
    let code = read_to_string(args.source.clone()).expect("Failed to read file");

    match parser::parse(args.source, &code) {
        Err(errors) => errors.emit(&code),
        Ok(ast) => {
            dbg!("Parsing has completed successfully.");
            if args.dump_ast {
                println!("{:#?}", ast);
                return;
            }

            match sema::analyze(ast, &code) {
                Err(errors) => errors.emit(&code),
                Ok(hir) => {
                    dbg!("Semantic analysis has completed successfully.");
                    if args.dump_hir {
                        println!("{:#?}", hir);
                        return;
                    }
                }
            }
        }
    }
}

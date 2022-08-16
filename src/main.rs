mod parser;

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
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

    /// Dump AST with resolved references
    #[clap(long)]
    dump_ref: bool,

    /// Dump semantic analyzed AST
    #[clap(long)]
    dump_sema: bool,

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
    println!("Hello, world!: {:?}", args);
}

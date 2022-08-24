mod diag;
mod hir;
mod parser;
mod sema;
mod util;

use anyhow::bail;
use clap::{ArgGroup, Parser};
use debug_print::debug_println;
use diag::Emit;
use std::fs::read_to_string;
use std::path::Path;
use util::pluralize;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("dumps")
        .args(&[
            "dump-tokens",
            "dump-ast",
            "dump-hir",
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

fn parse<P: AsRef<Path>>(args: &Args, source: P, code: &str) -> Result<(), Box<dyn Emit>> {
    let ast = parser::parse(source, code)?;
    debug_println!("Info: Parse has been completed successfully.");
    if args.dump_ast {
        println!("{:#?}", ast);
        return Ok(());
    }

    let hir = sema::analyze(ast, code)?;
    debug_println!("Info: Semantic analysis has been completed successfully.");
    if args.dump_hir {
        println!("{:#?}", hir);
        return Ok(());
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let source = Path::new(&args.source);
    if source.extension().filter(|&ext| ext == "mat") == None {
        bail!("Source file extension should be `.mat`");
    }
    let code = read_to_string(source)?;

    if let Err(errors) = parse(&args, source, &code) {
        errors.emit(&code);
        bail!(
            "Could not compile `{:?}` due to {} previous {}",
            source,
            errors.count(),
            pluralize("error", errors.count()),
        );
    }
    Ok(())
}

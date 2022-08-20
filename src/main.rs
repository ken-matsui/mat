mod parser;

use ariadne::{sources, Color, Fmt, Label, Report, ReportKind};
use clap::{ArgGroup, Parser};
use std::fs::read_to_string;

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
    let source_id: String = args.source;
    let src = read_to_string(source_id.clone()).expect("Failed to read file");

    let (ast, errs) = parser::parse::parse(src.clone());
    if let Some(ast) = ast {
        if args.dump_ast {
            println!("{:#?}", ast);
        }
    } else {
        for e in errs {
            let message = match e.reason() {
                chumsky::error::SimpleReason::Unexpected
                | chumsky::error::SimpleReason::Unclosed { .. } => {
                    format!(
                        "{}{}, expected {}",
                        if e.found().is_some() {
                            "unexpected token"
                        } else {
                            "unexpected end of input"
                        },
                        if let Some(label) = e.label() {
                            format!(" while parsing {}", label.fg(Color::Green))
                        } else {
                            " something else".to_string()
                        },
                        if e.expected().count() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    )
                }
                chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
            };

            Report::build(ReportKind::Error, source_id.clone(), e.span().start)
                .with_message(message)
                .with_label(Label::new((source_id.clone(), e.span())).with_message(
                    match e.reason() {
                        chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                        _ => format!(
                            "Unexpected {}",
                            e.found()
                                .map(|c| format!("token {}", c.fg(Color::Red)))
                                .unwrap_or_else(|| "end of input".to_string())
                        ),
                    },
                ))
                .finish()
                .print(sources(vec![(source_id.clone(), src.clone())]))
                .unwrap();
        }
    }
}

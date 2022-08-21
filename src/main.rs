mod parser;
mod sema;

use crate::parser::lib::ParserError;
use crate::sema::error::SemanticError;
use crate::sema::local_resolver::LocalResolver;
use ariadne::{sources, Color, Fmt, Label, Report, ReportKind, Source as Sources};
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
    let source = Source {
        id: args.source.clone(),
        content: read_to_string(args.source).expect("Failed to read file"),
    };

    let (ast, errs) = parser::parse(source.content.clone());
    if let Some(ast) = ast {
        if args.dump_ast {
            println!("{:#?}", ast);
            return;
        }
        match LocalResolver::new().resolve(ast) {
            Ok(()) => {
                println!("Semantic analysis has completed successfully.");
            }
            Err(errors) => {
                let mut report = Report::build(ReportKind::Error, (), 0);
                for err in errors {
                    report = match err {
                        SemanticError::DuplicatedDef(pre_span, span) => report
                            .with_message("Duplicated definition")
                            .with_label(
                                Label::new(pre_span.range())
                                    .with_message("previous definition of the definition"),
                            )
                            .with_label(Label::new(span.range()).with_message("redefined here")),
                        SemanticError::UnresolvedRef(span) => report
                            .with_message("Unresolved reference")
                            .with_label(Label::new(span.range()).with_message("undefined ident")),
                    };
                }
                report
                    .finish()
                    .print(Sources::from(source.content))
                    .unwrap();
            }
        }
    } else {
        emit_errors(errs, source);
    }
}

struct Source {
    id: String,
    content: String,
}

fn emit_errors(errs: Vec<ParserError>, source: Source) {
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

        Report::build(ReportKind::Error, source.id.clone(), e.span().start)
            .with_message(message)
            .with_label(
                Label::new((source.id.clone(), e.span())).with_message(match e.reason() {
                    chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                    _ => format!(
                        "Unexpected {}",
                        e.found()
                            .map(|c| format!("token {}", c.fg(Color::Red)))
                            .unwrap_or_else(|| "end of input".to_string())
                    ),
                }),
            )
            .finish()
            .print(sources(vec![(source.id.clone(), source.content.clone())]))
            .unwrap();
    }
}

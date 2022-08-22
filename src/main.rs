mod parser;
mod sema;

use crate::parser::ast::SrcId;
use crate::parser::lib::ParserError;
use crate::sema::error::SemanticError;
use crate::sema::local_resolver::LocalResolver;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source, Span};
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
    let code = read_to_string(args.source.clone()).expect("Failed to read file");

    let (ast, errs) = parser::parse(SrcId::from_path(args.source), code.clone());
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
                let source = Source::from(code.clone());

                for err in errors {
                    match err {
                        SemanticError::DuplicatedDef(pre_span, span) => {
                            Report::build(ReportKind::Error, span.src(), span.start())
                                .with_message("Duplicated definition")
                                .with_label(
                                    Label::new(pre_span)
                                        .with_message("previous definition".fg(Color::Blue))
                                        .with_color(Color::Blue),
                                )
                                .with_label(
                                    Label::new(span)
                                        .with_message("redefined here".fg(Color::Red))
                                        .with_color(Color::Red),
                                )
                                .finish()
                                .print((span.src(), source.clone()))
                        }
                        SemanticError::UnresolvedRef(span) => {
                            Report::build(ReportKind::Error, span.src(), span.start())
                                .with_message("Unresolved reference")
                                .with_label(
                                    Label::new(span)
                                        .with_message("undefined ident".fg(Color::Red))
                                        .with_color(Color::Red),
                                )
                                .finish()
                                .print((span.src(), source.clone()))
                        }
                    }
                    .unwrap();
                }
            }
        }
    } else {
        emit_errors(errs, code);
    }
}

fn emit_errors(errs: Vec<ParserError>, code: String) {
    let source = Source::from(code.clone());

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

        Report::build(ReportKind::Error, e.span().src(), e.span().start())
            .with_message(message)
            .with_label(Label::new(e.span()).with_message(match e.reason() {
                chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                _ => format!(
                        "Unexpected {}",
                        e.found()
                            .map(|c| format!("token {}", c.fg(Color::Red)))
                            .unwrap_or_else(|| "end of input".to_string())
                    ),
            }))
            .finish()
            .print((e.span().src(), source.clone()))
            .unwrap();
    }
}

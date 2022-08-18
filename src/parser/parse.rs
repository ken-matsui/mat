use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::text::Padded;
use chumsky::{prelude::*, stream::Stream};
use std::collections::HashSet;

pub type Span = std::ops::Range<usize>;

// enum TypeNode {
//     Type(String)
// }
//
// enum TypeDefinition {
//     TypeNode(TypeNode),
//     Name(String),
// }
//
// enum Slot {
//
// }
//
// enum CompositeTypeDefinition {
//     Def(TypeDefinition),
//     Members(Vec<Slot>),
// }
//
// enum StructNode {
//
// }
//
// enum Declarations {
//     Imports(Vec<String>),
//     Vars(HashSet<DefinedVariable>),
//     Fns(HashSet<DefinedFunction>),
//     Consts(HashSet<Constant>),
//     Structs(HashSet<StructNode>),
//     Types(HashSet<TypedefNode>)
// }
//
// enum AST {
//     Import(Vec<String>),
//     Decls(Declarations)
// }

// #[derive(Clone, Debug)]
// enum Token {
//     Void,
//     Char(char),
//     I8(i8),
//     I16(i16),
//     I32(i32),
//     I64(i64),
//     U8(u8),
//     U16(u16),
//     U32(u32),
//     U64(u64),
//     Struct,
//     Union,
//     Enum,
//     Static,
//     Extern,
//     Const,
//     If,
//     Else,
//     Match,
//     While,
//     Do,
//     For,
//     Return,
//     Break,
//     Continue,
//     Type,
//     Import,
//     Sizeof,
//     Fn,
//     Let,
//     Mut,
// }

#[derive(Debug)]
pub(crate) enum Expr {
    Num(i32),
    Var(String),

    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    Call(String, Vec<Expr>),
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>,
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>,
    },
}

fn ident() -> impl Parser<char, String, Error = Simple<char>> + Clone {
    text::ident().padded()
}

#[derive(Debug, PartialEq)]
pub(crate) struct Import {
    id: String,
}

// import std.io;
fn import_stmt() -> impl Parser<char, Import, Error = Simple<char>> + Clone {
    text::keyword("import")
        .then(
            ident()
                .repeated()
                .separated_by(just('.'))
                .map(|i| i.into_iter().flatten().collect::<Vec<String>>().join(".")),
        )
        .then_ignore(just(';'))
        .map(|((), id)| Import { id })
        .labelled("import")
        .padded()
}

fn import_stmts() -> impl Parser<char, Vec<Import>, Error = Simple<char>> + Clone {
    import_stmt().repeated()
}

fn parser() -> impl Parser<char, Vec<Import>, Error = Simple<char>> + Clone {
    // Vec<(Expr, Span)>
    // let ident = text::ident().padded();

    // let expr = recursive(|expr| {
    //     let int = text::int(10)
    //         .map(|s: String| Expr::Num(s.parse().unwrap()))
    //         .padded();
    //
    //     let call = ident
    //         .then(
    //             expr.clone()
    //                 .separated_by(just(','))
    //                 .allow_trailing()
    //                 .delimited_by(just('('), just(')')),
    //         )
    //         .map(|(f, args)| Expr::Call(f, args));
    //
    //     let atom = int
    //         .or(expr.delimited_by(just('('), just(')')))
    //         .or(call)
    //         .or(ident.map(Expr::Var));
    //
    //     let op = |c| just(c).padded();
    //
    //     let unary = op('-')
    //         .repeated()
    //         .then(atom)
    //         .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));
    //
    //     let product = unary
    //         .clone()
    //         .then(
    //             op('*')
    //                 .to(Expr::Mul as fn(_, _) -> _)
    //                 .or(op('/').to(Expr::Div as fn(_, _) -> _))
    //                 .then(unary)
    //                 .repeated(),
    //         )
    //         .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));
    //
    //     let sum = product
    //         .clone()
    //         .then(
    //             op('+')
    //                 .to(Expr::Add as fn(_, _) -> _)
    //                 .or(op('-').to(Expr::Sub as fn(_, _) -> _))
    //                 .then(product)
    //                 .repeated(),
    //         )
    //         .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));
    //
    //     sum
    // });
    //
    // let compilation_unit = recursive(|decl| {
    //     let import = text::keyword("let")
    //         .ignore_then(ident)
    //         .then_ignore(just('='))
    //         .then(expr.clone())
    //         .then_ignore(just(';'))
    //         .then(decl.clone())
    //         .map(|((name, rhs), then)| Expr::Let {
    //             name,
    //             rhs: Box::new(rhs),
    //             then: Box::new(then),
    //         });
    //
    //     // let r#fn = text::keyword("fn")
    //     //     .ignore_then(ident)
    //     //     .then(ident.repeated())
    //     //     .then_ignore(just('='))
    //     //     .then(expr.clone())
    //     //     .then_ignore(just(';'))
    //     //     .then(decl)
    //     //     .map(|(((name, args), body), then)| Expr::Fn {
    //     //         name,
    //     //         args,
    //     //         body: Box::new(body),
    //     //         then: Box::new(then),
    //     //     });
    //
    //     import.padded()
    // });

    // compilation_unit
    //     .then_ignore(end())
    // .map_with_span(|tok, span| (tok, span))

    // let int = text::int(10)
    //     .map(|s: String| Expr::Num(s.parse::<i32>().unwrap()))
    //     .padded();

    import_stmts().then_ignore(end())
}

pub(crate) fn parse(src: String) -> Result<Vec<Import>, Vec<Simple<char>>> {
    parser().parse(src)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn import_stmt_test() {
        assert_eq!(
            import_stmt().parse("import std.io;"),
            Ok(Import {
                id: "std.io".to_string()
            })
        );
        assert_eq!(
            import_stmt().parse("import     std  .   io   ;"),
            Ok(Import {
                id: "std.io".to_string()
            })
        );
        assert_eq!(
            import_stmt().parse("import stdio;"),
            Ok(Import {
                id: "stdio".to_string()
            })
        );
        assert!(import_stmt().parse("import 1std.io;").is_err());
        assert!(import_stmt().parse("import std.1io;").is_err());
        assert!(import_stmt().parse("import std.io").is_err());
        assert!(import_stmt().parse("use std.io;").is_err());
    }

    #[test]
    fn import_stmts_test() {
        assert_eq!(
            import_stmts().parse("import std.io;\nimport stdio;"),
            Ok(vec![
                Import {
                    id: "std.io".to_string()
                },
                Import {
                    id: "stdio".to_string()
                }
            ])
        );
        assert_eq!(
            import_stmts().parse("import std.io;\n     \r  \nimport stdio;"),
            Ok(vec![
                Import {
                    id: "std.io".to_string()
                },
                Import {
                    id: "stdio".to_string()
                }
            ])
        );
    }
}

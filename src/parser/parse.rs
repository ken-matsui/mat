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

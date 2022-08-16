package mat.compiler;

import mat.ast.AST;
import mat.exception.CompileException;
import mat.exception.SemanticException;
import mat.parser.Parser;
import mat.type.TypeTable;
import mat.utils.ErrorHandler;

import java.io.File;

public class Compiler {
    private final Options opts = new Options();
    private final ErrorHandler errorHandler = new ErrorHandler();

    static public void main(String[] args) throws IllegalStateException {
        new Compiler().build(args);
    }

    private void build(String[] args) throws IllegalStateException {
        parseArgs(args);
        for (String src : opts.sources) {
            try {
                compile(src);
            } catch (CompileException ex) {
                System.err.println(ex.getMessage());
            }
        }
    }

    private void parseArgs(String[] args) throws IllegalStateException {
        for (String a : args) {
            switch (a) {
                case "--dump-tokens" -> opts.dumpTokens = true;
                case "--dump-ast" -> opts.dumpAST = true;
                case "--dump-ref" -> opts.dumpRef = true;
                case "--dump-sema" -> opts.dumpSema = true;
                default -> {
                    if (a.endsWith(".mat")) {
                        opts.sources.add(a);
                    } else {
                        System.out.println("WARNING: The CLI argument `" + a + "` will be ignored.");
                    }
                }
            }
        }
    }

    private void compile(String src) throws CompileException {
        AST ast = Parser.parseFile(new File(src), false);
        if (dumpAST(ast)) {
            return;
        }

        TypeTable types = opts.platform.typeTable();
        AST sema = semanticAnalyze(ast, types);
        if (dumpSema(sema)) {
            return;
        }
    }

    private boolean dumpAST(AST ast) {
        if (opts.dumpTokens) {
            ast.dumpTokens();
            return true;
        } else if (opts.dumpAST) {
            ast.dump();
            return true;
        }
        return false;
    }

    private AST semanticAnalyze(AST ast, TypeTable types) throws SemanticException {
        new LocalResolver(errorHandler).resolve(ast);
//        new TypeResolver(types, errorHandler).resolve(ast);
//        types.semanticCheck(errorHandler);
//        if (opts.dumpRef) {
//            ast.dump();
//            return ast;
//        }
//        new DereferenceChecker(types, errorHandler).check(ast);
//        new TypeChecker(types, errorHandler).check(ast);
        return ast;
    }

    private boolean dumpSema(AST ast) {
        if (opts.dumpRef) {
            return true;
        } else if (opts.dumpSema) {
            ast.dump();
            return true;
        }
        return false;
    }
}

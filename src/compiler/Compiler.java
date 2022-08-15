package mat.compiler;

import mat.ast.AST;
import mat.exception.CompileException;
import mat.parser.Parser;

import java.io.File;

public class Compiler {
    private final Options opts = new Options();

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
}

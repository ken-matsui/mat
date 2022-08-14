package mat.compiler;

import mat.ast.AST;
import mat.exception.CompileException;
import mat.parser.Parser;

import java.io.File;

public class Compiler {
    static public void main(String[] srcs) {
        new Compiler().build(srcs);
    }

    private void build(String[] srcs) {
        for (String src : srcs) {
            try {
                compile(src);
            } catch (CompileException ex) {
                System.err.println(ex.getMessage());
            }
        }
    }

    private void compile(String src) throws CompileException {
        AST ast = Parser.parseFile(new File(src), false);
        dumpAST(ast); // For now, just dump AST and finish
    }

    private void dumpAST(AST ast) {
        ast.dump();
    }
}

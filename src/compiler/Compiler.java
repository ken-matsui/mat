package mat.compiler;

import mat.ast.AST;
import mat.exception.FileException;
import mat.exception.SyntaxException;
import mat.parser.Parser;

import java.io.File;

public class Compiler {
    static public void main(String[] srcs) {
        new Compiler().compile(srcs);
    }

    private void compile(String[] srcs) {
        for (String src : srcs) {
            try {
                AST ast = Parser.parseFile(new File(src), false);
            } catch (SyntaxException ex) {
                System.err.println(ex.getMessage());
            } catch (FileException ex) {
                System.err.println(ex.getMessage());
            }
        }
    }
}

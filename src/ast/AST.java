package mat.ast;

import mat.parser.ParserConstants;
import mat.parser.Token;
import mat.utils.TextUtils;

import java.io.PrintStream;

public class AST extends Node {
    protected Location source;
    protected Declarations decls;

    public AST(Location source, Declarations decls) {
        super();
        this.source = source;
        this.decls = decls;
    }

    public Location location() {
        return source;
    }

    protected void _dump(Dumper d) {
        d.printNodeList("typedefs", decls.typedefs());
        d.printNodeList("variables", decls.defvars());
        d.printNodeList("constants", decls.constants());
        d.printNodeList("structs", decls.defstructs());
        d.printNodeList("functions", decls.defns());
    }

    public void dumpTokens() {
        dumpTokens(System.out);
    }

    static final private int NUM_LEFT_COLUMNS = 24;
    public void dumpTokens(PrintStream s) {
        for (Token t = source.token(); t != null; t = t.next) {
            s.printf("%-" + NUM_LEFT_COLUMNS + "s", ParserConstants.tokenImage[t.kind]);
            s.println(TextUtils.dumpString(t.image));
        }
    }
}

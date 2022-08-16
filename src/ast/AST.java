package mat.ast;

import mat.entity.*;
import mat.parser.ParserConstants;
import mat.parser.Token;
import mat.utils.TextUtils;

import java.io.PrintStream;
import java.util.ArrayList;
import java.util.List;

public class AST extends Node {
    protected Location source;
    protected Declarations decls;
    protected ToplevelScope scope;
    protected ConstantTable constantTable;

    public AST(Location source, Declarations decls) {
        super();
        this.source = source;
        this.decls = decls;
    }

    public Location location() {
        return source;
    }

    public List<Entity> declarations() {
        List<Entity> result = new ArrayList<>();
        result.addAll(decls.funcdecls);
        result.addAll(decls.vardecls);
        return result;
    }

    public List<Entity> definitions() {
        List<Entity> result = new ArrayList<>();
        result.addAll(decls.defvars);
        result.addAll(decls.defns);
        result.addAll(decls.constants);
        return result;
    }

    public List<DefinedVariable> definedVariables() {
        return decls.defvars();
    }

    public List<Constant> constants() {
        return decls.constants();
    }

    public List<DefinedFunction> definedFunctions() {
        return decls.defns();
    }

    // called by LocalResolver
    public void setScope(ToplevelScope scope) {
        if (this.scope != null) {
            throw new Error("must not happen: ToplevelScope set twice");
        }
        this.scope = scope;
    }

    // called by LocalResolver
    public void setConstantTable(ConstantTable table) {
        if (this.constantTable != null) {
            throw new Error("must not happen: ConstantTable set twice");
        }
        this.constantTable = table;
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

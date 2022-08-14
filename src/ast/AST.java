package mat.ast;

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
        d.printNodeList("variables", decls.defvars());
        d.printNodeList("functions", decls.defns());
    }
}

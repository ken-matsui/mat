package mat.mir;
import mat.ast.Location;

public class Return extends Stmt {
    protected Expr expr;

    public Return(Location loc, Expr expr) {
        super(loc);
        this.expr = expr;
    }

    public Expr expr() { return expr; }

    public <S,E> S accept(MIRVisitor<S,E> visitor) {
        return visitor.visit(this);
    }

    protected void _dump(Dumper d) {
        d.printMember("expr", expr);
    }
}

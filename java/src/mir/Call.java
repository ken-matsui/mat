package mat.mir;
import mat.entity.Function;
import mat.entity.Entity;
import mat.asm.Type;
import java.util.List;

public class Call extends Expr {
    private Expr expr;
    private List<Expr> args;

    public Call(Type type, Expr expr, List<Expr> args) {
        super(type);
        this.expr = expr;
        this.args = args;
    }

    public Expr expr() { return expr; }
    public List<Expr> args() { return args; }

    public long numArgs() {
        return args.size();
    }

    /** Returns true if this funcall is NOT a function pointer call. */
    public boolean isStaticCall() {
        return (expr.getEntityForce() instanceof Function);
    }

    /**
     * Returns a function object which is referred by expression.
     * This method expects this is static function call (isStaticCall()).
     */
    public Function function() {
        Entity ent = expr.getEntityForce();
        if (ent == null) {
            throw new Error("not a static funcall");
        }
        return (Function)ent;
    }

    public <S,E> E accept(MIRVisitor<S,E> visitor) {
        return visitor.visit(this);
    }

    protected void _dump(Dumper d) {
        d.printMember("expr", expr);
        d.printMembers("args", args);
    }
}

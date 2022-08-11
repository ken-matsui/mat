package mat.ast;

public class AST extends Node {
    protected ExprNode expr;

    public AST(ExprNode expr) {
        super();
        this.expr = expr;
    }
}

package mat.ast;

import mat.exception.SemanticError;
import mat.type.FunctionType;
import mat.type.Type;

import java.util.List;

public class FnCallNode extends ExprNode {
    protected ExprNode expr;
    protected List<ExprNode> args;

    public FnCallNode(ExprNode expr, List<ExprNode> args) {
        this.expr = expr;
        this.args = args;
    }

    public ExprNode expr() {
        return expr;
    }

    /**
     * Returns a type of return value of the function which is referred
     * by expr.  This method expects expr.type().isCallable() is true.
     */
    public Type type() {
        try {
            return functionType().returnType();
        }
        catch (ClassCastException err) {
            throw new SemanticError(err.getMessage());
        }
    }

    /**
     * Returns a type of function which is referred by expr.
     * This method expects expr.type().isCallable() is true.
     */
    public FunctionType functionType() {
        return expr.type().getPointerType().baseType().getFunctionType();
    }

    public long numArgs() {
        return args.size();
    }

    public List<ExprNode> args() {
        return args;
    }

    // called from TypeChecker
    public void replaceArgs(List<ExprNode> args) {
        this.args = args;
    }

    public Location location() {
        return expr.location();
    }

    protected void _dump(Dumper d) {
        d.printMember("expr", expr);
        d.printNodeList("args", args);
    }

    public <S,E> E accept(ASTVisitor<S,E> visitor) {
        return visitor.visit(this);
    }
}

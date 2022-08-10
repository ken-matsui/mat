package mat.ast;

public class BinaryOpNode extends ExprNode {
    protected String operator;
    protected ExprNode left, right;

    public BinaryOpNode(ExprNode left, String op, ExprNode right) {
        super();
        this.operator = op;
        this.left = left;
        this.right = right;
    }
}

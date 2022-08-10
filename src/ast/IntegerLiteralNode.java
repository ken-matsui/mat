package mat.ast;

public class IntegerLiteralNode extends LiteralNode {
    protected long value;

    public IntegerLiteralNode(long value) {
        super();
        this.value = value;
    }
}

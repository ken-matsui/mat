package mat.asm;
import mat.utils.TextUtils;

public class Comment extends Assembly {
    protected String string;
    protected int indentLevel;

    public Comment(String string) {
        this(string, 0);
    }

    public Comment(String string, int indentLevel) {
        this.string = string;
        this.indentLevel = indentLevel;
    }

    public boolean isComment() {
        return true;
    }

    public String toSource(SymbolTable table) {
        return "\t" + indent() + "# " + string;
    }

    protected String indent() {
        return "  ".repeat(Math.max(0, indentLevel));
    }

    public String dump() {
        return "(Comment " + TextUtils.dumpString(string) + ")";
    }
}

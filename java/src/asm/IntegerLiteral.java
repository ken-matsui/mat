package mat.asm;

public class IntegerLiteral implements Literal {
    protected long value;

    public IntegerLiteral(long n) {
        this.value = n;
    }

    public boolean equals(Object other) {
        return (other instanceof IntegerLiteral)
                && equals((IntegerLiteral)other);
    }

    public boolean equals(IntegerLiteral other) {
        return other.value == this.value;
    }

    public long value() {
        return this.value;
    }

    public boolean isZero() {
        return value == 0;
    }

    public IntegerLiteral plus(long diff) {
        return new IntegerLiteral(value + diff);
    }

    public IntegerLiteral integerLiteral() {
        return this;
    }

    public String toSource() {
        return Long.toString(value);
    }

    public String toSource(SymbolTable table) {
        return toSource();
    }

    public void collectStatistics(Statistics stats) {
        // does nothing
    }

    public String toString() {
        return Long.toString(value);
    }

    public int compareTo(Literal lit) {
        return -(lit.cmp(this));
    }

    public int cmp(IntegerLiteral i) {
        return Long.compare(value, i.value);
    }

    public int cmp(NamedSymbol sym) {
        return -1;
    }

    public int cmp(UnnamedSymbol sym) {
        return -1;
    }

    public int cmp(SuffixedSymbol sym) {
        return -1;
    }

    public String dump() {
        return "(IntegerLiteral " + Long.toString(value) + ")";
    }
}

package mat.entity;

import mat.asm.Label;
import mat.asm.Symbol;
import mat.ast.TypeNode;
import mat.type.Type;

import java.util.List;
import java.util.Objects;

abstract public class Function extends Entity {
    protected Symbol callingSymbol;
    protected Label label;

    public Function(boolean priv, TypeNode t, String name) {
        super(priv, t, name);
    }

    public boolean isInitialized() { return true; }

    abstract public boolean isDefined();
    abstract public List<Parameter> parameters();

    public Type returnType() {
        return type().getFunctionType().returnType();
    }

    public boolean isVoid() {
        return returnType().isVoid();
    }

    public void setCallingSymbol(Symbol sym) {
        if (this.callingSymbol != null) {
            throw new Error("must not happen: Function#callingSymbol was set again");
        }
        this.callingSymbol = sym;
    }

    public Symbol callingSymbol() {
        if (this.callingSymbol == null) {
            throw new Error("must not happen: Function#callingSymbol called but null");
        }
        return this.callingSymbol;
    }

    public Label label() {
        return Objects.requireNonNullElseGet(label, () -> label = new Label(callingSymbol()));
    }
}

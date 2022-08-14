package mat.entity;

import mat.ast.TypeNode;

abstract public class Variable extends Entity {
    public Variable(boolean priv, TypeNode type, String name) {
        super(priv, type, name);
    }
}

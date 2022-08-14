package mat.entity;

import mat.ast.TypeNode;
import mat.type.*;

abstract public class Variable extends Entity {
    public Variable(boolean priv, TypeNode type, String name) {
        super(priv, type, name);
    }
}

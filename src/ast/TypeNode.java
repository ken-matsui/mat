package mat.ast;

import mat.type.Type;
import mat.type.TypeRef;

public class TypeNode extends Node {
    TypeRef typeRef;
    Type type;

    public TypeNode(TypeRef ref) {
        super();
        this.typeRef = ref;
    }

    public TypeNode(Type type) {
        super();
        this.type = type;
    }

    public TypeRef typeRef() {
        return typeRef;
    }

    public boolean isResolved() {
        return (type != null);
    }

    public Type type() {
        if (type == null) {
            throw new Error("TypeNode not resolved: " + typeRef);
        }
        return type;
    }

    public Location location() {
        return typeRef == null ? null : typeRef.location();
    }

    protected void _dump(Dumper d) {
        d.printMember("typeref", typeRef);
        d.printMember("type", type);
    }
}

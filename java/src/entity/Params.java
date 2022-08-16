package mat.entity;

import mat.ast.Location;
import mat.type.ParamTypeRefs;
import mat.type.TypeRef;

import java.util.ArrayList;
import java.util.List;

public class Params extends ParamSlots<Parameter>
        implements mat.ast.Dumpable {
    public Params(Location loc, List<Parameter> paramDescs) {
        super(loc, paramDescs, false);
    }

    public List<Parameter> parameters() {
        return paramDescriptors;
    }

    public ParamTypeRefs parametersTypeRef() {
        List<TypeRef> typerefs = new ArrayList<TypeRef>();
        for (Parameter param : paramDescriptors) {
            typerefs.add(param.typeNode().typeRef());
        }
        return new ParamTypeRefs(location, typerefs, vararg);
    }

    public boolean equals(Object other) {
        return (other instanceof Params) && equals((Params)other);
    }

    public boolean equals(Params other) {
        return other.vararg == vararg
                && other.paramDescriptors.equals(paramDescriptors);
    }

    public void dump(mat.ast.Dumper d) {
        d.printNodeList("parameters", parameters());
    }
}

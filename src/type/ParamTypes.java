package mat.type;

import mat.ast.Location;
import mat.entity.ParamSlots;
import java.util.*;

public class ParamTypes extends ParamSlots<Type> {
    protected ParamTypes(Location loc, List<Type> paramDescs, boolean vararg) {
        super(loc, paramDescs, vararg);
    }

    public List<Type> types() {
        return paramDescriptors;
    }

    public boolean isSameType(ParamTypes other) {
        if (vararg != other.vararg) return false;
        if (minArgc() != other.minArgc()) return false;
        Iterator<Type> otherTypes = other.types().iterator();
        for (Type t : paramDescriptors) {
            if (! t.isSameType(otherTypes.next())) {
                return false;
            }
        }
        return true;
    }

    public boolean equals(Object other) {
        return (other instanceof ParamTypes) && equals((ParamTypes)other);
    }

    public boolean equals(ParamTypes other) {
        return vararg == other.vararg
                && paramDescriptors.equals(other.paramDescriptors);
    }
}

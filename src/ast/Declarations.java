package mat.ast;

import mat.entity.*;
import java.util.*;

public class Declarations {
    Set<DefinedVariable> defvars = new LinkedHashSet<>();
    Set<DefinedFunction> defns = new LinkedHashSet<>();

    ArrayList<String> imports = new ArrayList<>();

    public List<DefinedVariable> defvars() {
        return new ArrayList<>(defvars);
    }

    public List<DefinedFunction> defns() {
        return new ArrayList<>(defns);
    }

    public void addImports(ArrayList<String> imports) {
        this.imports = imports;
    }

    public void addDefvar(DefinedVariable var) {
        defvars.add(var);
    }

    public void addDefn(DefinedFunction func) {
        defns.add(func);
    }
}

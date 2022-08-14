package mat.ast;

import mat.entity.Constant;
import mat.entity.DefinedFunction;
import mat.entity.DefinedVariable;

import java.util.ArrayList;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Set;

public class Declarations {
    Set<DefinedVariable> defvars = new LinkedHashSet<>();
    Set<DefinedFunction> defns = new LinkedHashSet<>();
    Set<Constant> constants = new LinkedHashSet<>();
    Set<TypedefNode> typedefs = new LinkedHashSet<TypedefNode>();

    ArrayList<String> imports = new ArrayList<>();

    public List<DefinedVariable> defvars() {
        return new ArrayList<>(defvars);
    }

    public List<DefinedFunction> defns() {
        return new ArrayList<>(defns);
    }

    public List<Constant> constants() {
        return new ArrayList<>(constants);
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

    public void addConstant(Constant c) {
        constants.add(c);
    }

    public void addTypedef(TypedefNode n) {
        typedefs.add(n);
    }

    public List<TypedefNode> typedefs() {
        return new ArrayList<>(typedefs);
    }
}

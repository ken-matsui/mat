package mat.ast;

import mat.entity.*;

import java.util.ArrayList;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Set;

public class Declarations {
    Set<DefinedVariable> defvars = new LinkedHashSet<>();
    Set<UndefinedVariable> vardecls = new LinkedHashSet<>();
    Set<DefinedFunction> defns = new LinkedHashSet<>();
    Set<UndefinedFunction> funcdecls = new LinkedHashSet<>();
    Set<Constant> constants = new LinkedHashSet<>();
    Set<StructNode> defstructs = new LinkedHashSet<>();
    Set<TypedefNode> typedefs = new LinkedHashSet<>();

    ArrayList<String> imports = new ArrayList<>();

    public void addImports(ArrayList<String> imports) {
        this.imports = imports;
    }

    public void addDefvar(DefinedVariable var) {
        defvars.add(var);
    }

    public List<DefinedVariable> defvars() {
        return new ArrayList<>(defvars);
    }

    public void addDefn(DefinedFunction func) {
        defns.add(func);
    }

    public List<DefinedFunction> defns() {
        return new ArrayList<>(defns);
    }

    public void addConstant(Constant c) {
        constants.add(c);
    }

    public List<Constant> constants() {
        return new ArrayList<>(constants);
    }

    public void addDefstruct(StructNode n) {
        defstructs.add(n);
    }

    public List<StructNode> defstructs() {
        return new ArrayList<StructNode>(defstructs);
    }

    public void addTypedef(TypedefNode n) {
        typedefs.add(n);
    }

    public List<TypedefNode> typedefs() {
        return new ArrayList<>(typedefs);
    }
}

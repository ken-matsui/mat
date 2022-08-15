package mat.sysdep;

import mat.type.TypeTable;

public class X86Linux implements Platform {
    public TypeTable typeTable() {
        return TypeTable.ilp32();
    }
}

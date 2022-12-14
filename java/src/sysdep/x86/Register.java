package mat.sysdep.x86;
import mat.asm.Type;
import mat.asm.SymbolTable;

class Register extends mat.asm.Register {
    RegisterClass _class;
    Type type;

    Register(RegisterClass _class, Type type) {
        this._class = _class;
        this.type = type;
    }

    Register forType(Type t) {
        return new Register(_class, t);
    }

    public boolean isRegister() { return true; }

    public boolean equals(Object other) {
        return (other instanceof Register) && equals((Register)other);
    }

    /** size difference does NOT matter. */
    public boolean equals(Register reg) {
        return _class.equals(reg._class);
    }

    public int hashCode() {
        return _class.hashCode();
    }

    RegisterClass registerClass() {
        return _class;
    }

    String baseName() {
        return _class.toString().toLowerCase();
    }

    public String toSource(SymbolTable table) {
        // GNU assembler dependent
        return "%" + typedName();
    }

    private String typedName() {
        return switch (type) {
            case INT8 -> lowerByteRegister();
            case INT16 -> baseName();
            case INT32 -> "e" + baseName();
            case INT64 -> "r" + baseName();
            default -> throw new Error("unknown register Type: " + type);
        };
    }

    private String lowerByteRegister() {
        return switch (_class) {
            case AX, BX, CX, DX -> baseName().charAt(0) + "l";
            default -> throw new Error("does not have lower-byte register: " + _class);
        };
    }

    public String dump() {
        return "(Register " + _class.toString() + " " + type.toString() + ")";
    }
}

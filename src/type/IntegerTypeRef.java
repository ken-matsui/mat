package mat.type;

import mat.ast.Location;

public class IntegerTypeRef extends TypeRef {
    static public IntegerTypeRef i8Ref(Location loc) {
        return new IntegerTypeRef("i8", loc);
    }
    static public IntegerTypeRef i8Ref() {
        return new IntegerTypeRef("i8");
    }

    static public IntegerTypeRef i16Ref(Location loc) {
        return new IntegerTypeRef("i16", loc);
    }
    static public IntegerTypeRef i16Ref() {
        return new IntegerTypeRef("i16");
    }

    static public IntegerTypeRef i32Ref(Location loc) {
        return new IntegerTypeRef("i32", loc);
    }
    static public IntegerTypeRef i32Ref() {
        return new IntegerTypeRef("i32");
    }

    static public IntegerTypeRef i64Ref(Location loc) {
        return new IntegerTypeRef("i64", loc);
    }
    static public IntegerTypeRef i64Ref() {
        return new IntegerTypeRef("i64");
    }

    static public IntegerTypeRef u8Ref(Location loc) {
        return new IntegerTypeRef("u8", loc);
    }
    static public IntegerTypeRef u8Ref() {
        return new IntegerTypeRef("u8");
    }

    static public IntegerTypeRef u16Ref(Location loc) {
        return new IntegerTypeRef("u16", loc);
    }
    static public IntegerTypeRef u16Ref() {
        return new IntegerTypeRef("u16");
    }

    static public IntegerTypeRef u32Ref(Location loc) {
        return new IntegerTypeRef("u32", loc);
    }
    static public IntegerTypeRef u32Ref() {
        return new IntegerTypeRef("u32");
    }

    static public IntegerTypeRef u64Ref(Location loc) {
        return new IntegerTypeRef("u64", loc);
    }
    static public IntegerTypeRef u64Ref() {
        return new IntegerTypeRef("u64");
    }

    protected String name;

    public IntegerTypeRef(String name) {
        this(name, null);
    }

    public IntegerTypeRef(String name, Location loc) {
        super(loc);
        this.name = name;
    }

    public String name() {
        return name;
    }

    public boolean equals(Object other) {
        if (! (other instanceof IntegerTypeRef)) return false;
        IntegerTypeRef ref = (IntegerTypeRef)other;
        return name.equals(ref.name);
    }

    public String toString() {
        return name;
    }
}

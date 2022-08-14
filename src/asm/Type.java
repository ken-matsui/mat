package mat.asm;

public enum Type {
    INT8, INT16, INT32, INT64;

    static public Type get(long size) {
        return switch ((int) size) {
            case 1 -> INT8;
            case 2 -> INT16;
            case 4 -> INT32;
            case 8 -> INT64;
            default -> throw new Error("unsupported asm type size: " + size);
        };
    }

    public int size() {
        return switch (this) {
            case INT8 -> 1;
            case INT16 -> 2;
            case INT32 -> 4;
            case INT64 -> 8;
        };
    }
}

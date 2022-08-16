package mat.mir;

public enum Op {
    ADD,
    SUB,
    MUL,
    S_DIV,
    U_DIV,
    S_MOD,
    U_MOD,
    BIT_AND,
    BIT_OR,
    BIT_XOR,
    BIT_LSHIFT,
    BIT_RSHIFT,
    ARITH_RSHIFT,

    EQ,
    NEQ,
    S_GT,
    S_GTEQ,
    S_LT,
    S_LTEQ,
    U_GT,
    U_GTEQ,
    U_LT,
    U_LTEQ,

    UMINUS,
    BIT_NOT,
    NOT,

    S_CAST,
    U_CAST;

    static public Op internBinary(String op, boolean isSigned) {
        return switch (op) {
            case "+" -> Op.ADD;
            case "-" -> Op.SUB;
            case "*" -> Op.MUL;
            case "/" -> isSigned ? Op.S_DIV : Op.U_DIV;
            case "%" -> isSigned ? Op.S_MOD : Op.U_MOD;
            case "&" -> Op.BIT_AND;
            case "|" -> Op.BIT_OR;
            case "^" -> Op.BIT_XOR;
            case "<<" -> Op.BIT_LSHIFT;
            case ">>" -> isSigned ? Op.ARITH_RSHIFT : Op.BIT_RSHIFT;
            case "==" -> Op.EQ;
            case "!=" -> Op.NEQ;
            case "<" -> isSigned ? Op.S_LT : Op.U_LT;
            case "<=" -> isSigned ? Op.S_LTEQ : Op.U_LTEQ;
            case ">" -> isSigned ? Op.S_GT : Op.U_GT;
            case ">=" -> isSigned ? Op.S_GTEQ : Op.U_GTEQ;
            default -> throw new Error("unknown binary op: " + op);
        };
    }

    static public Op internUnary(String op) {
        return switch (op) {
            case "+" -> throw new Error("unary+ should not be in IR");
            case "-" -> Op.UMINUS;
            case "~" -> Op.BIT_NOT;
            case "!" -> Op.NOT;
            default -> throw new Error("unknown unary op: " + op);
        };
    }
}

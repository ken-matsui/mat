package mat.asm;

public interface Symbol extends Literal {
    String name();
    String toString();
    String dump();
}

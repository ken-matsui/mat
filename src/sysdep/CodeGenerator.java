package mat.sysdep;

public interface CodeGenerator {
    AssemblyCode generate(mat.mir.MIR mir);
}

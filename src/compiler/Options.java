package mat.compiler;

import mat.sysdep.Platform;
import mat.sysdep.X86Linux;

import java.util.ArrayList;
import java.util.List;

public class Options {
    boolean dumpTokens = false;
    boolean dumpAST = false;
    boolean dumpRef = false;
    boolean dumpSema = false;
    boolean dumpMIR = true; // TODO: For now, this is a default option.
    List<String> sources = new ArrayList<>();
    Platform platform = new X86Linux();
}

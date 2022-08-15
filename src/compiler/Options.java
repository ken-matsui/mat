package mat.compiler;

import java.util.ArrayList;
import java.util.List;

public class Options {
    boolean dumpTokens = false;
    boolean dumpAST = true; // TODO: For now, this is a default option.
    List<String> sources = new ArrayList<>();
}

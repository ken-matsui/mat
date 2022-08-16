package mat.utils;

import mat.ast.Location;

import java.io.PrintStream;

public class ErrorHandler {
    protected PrintStream stream = new PrintStream(System.err);
    protected long nError;
    protected long nWarning;

    private static final String errorPrefix = TermColor.RED + "error" + TermColor.RESET;
    private static final String warnPrefix = TermColor.YELLOW + "warning" + TermColor.RESET;

    public void error(Location loc, String msg) {
        error(loc.toString() + ": " + msg);
    }

    public void error(String msg) {
        stream.println(errorPrefix + ": " + msg + '\n');
        nError++;
    }

    public void warn(Location loc, String msg) {
        warn(loc.toString() + ": " + msg);
    }

    public void warn(String msg) {
        stream.println(warnPrefix + ": " + msg + '\n');
        nWarning++;
    }

    public boolean errorOccurred() {
        return (nError > 0);
    }
}

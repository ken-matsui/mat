package mat.utils;

import mat.ast.Location;

import java.io.PrintStream;

public class ErrorHandler {
    protected PrintStream stream = new PrintStream(System.err);
    protected long nError;
    protected long nWarning;

    public void error(Location loc, String msg) {
        error(loc.toString() + ": " + msg);
    }

    public void error(String msg) {
        stream.println("error: " + msg + '\n');
        nError++;
    }

    public void warn(Location loc, String msg) {
        warn(loc.toString() + ": " + msg);
    }

    public void warn(String msg) {
        stream.println("warning: " + msg + '\n');
        nWarning++;
    }

    public boolean errorOccurred() {
        return (nError > 0);
    }
}

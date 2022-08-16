package mat.utils;

import mat.ast.Location;

import java.io.PrintStream;
import java.util.Optional;

public class ErrorHandler {
    protected PrintStream stream = new PrintStream(System.err);
    protected long nError;
    protected long nWarning;

    public static final String errorPrefix = TermColor.RED + "error" + TermColor.RESET;
    public static final String warnPrefix = TermColor.YELLOW + "warning" + TermColor.RESET;
    public static final String arrow = TermColor.BLUE + "  --> " + TermColor.RESET;

    public void error(String msg) {
        error(msg, Optional.empty());
    }

    public void error(String msg, Location loc) {
        error(msg, Optional.of(loc));
    }

    public void error(String msg, Optional<Location> loc) {
        emit(errorPrefix, msg, loc);
        nError++;
    }

    public void warn(String msg) {
        warn(msg, Optional.empty());
    }

    public void warn(String msg, Location loc) {
        warn(msg, Optional.of(loc));
    }

    public void warn(String msg, Optional<Location> loc) {
        emit(warnPrefix, msg, loc);
        nWarning++;
    }

    private void emit(String prefix, String msg, Optional<Location> loc) {
        stream.println(prefix + ": " + msg);
        loc.ifPresent(location -> stream.println(arrow + location + '\n'));
    }

    public boolean errorOccurred() {
        return (nError > 0);
    }
}

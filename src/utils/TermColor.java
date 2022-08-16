package mat.utils;

public enum TermColor {
    // Reset
    RESET("\033[00m"),

    // Regular Colors
    BLACK("\033[30m"),
    RED("\033[31m"),
    GREEN("\033[32m"),
    YELLOW("\033[33m"),
    BLUE("\033[34m"),
    PURPLE("\033[35m"),
    CYAN("\033[36m"),
    WHITE("\033[37m"),
    ;

    private final String value;

    TermColor(String value) {
        this.value = value;
    }

    @Override
    public String toString() {
        return value;
    }
}

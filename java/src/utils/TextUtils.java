package mat.utils;

import mat.parser.Parser;

import java.io.UnsupportedEncodingException;

abstract public class TextUtils {
    static final private byte vtab = 013;

    static public String dumpString(String str) {
        try {
            return dumpString(str, Parser.SOURCE_ENCODING);
        } catch (UnsupportedEncodingException ex) {
            throw new Error("UTF-8 is not supported?: " + ex.getMessage());
        }
    }

    static public String dumpString(String string, String encoding) throws UnsupportedEncodingException {
        byte[] src = string.getBytes(encoding);
        StringBuilder buf = new StringBuilder();
        buf.append("\"");
        for (byte b : src) {
            int c = toUnsigned(b);
            switch (c) {
                case '"' -> buf.append("\\\"");
                case '\b' -> buf.append("\\b");
                case '\t' -> buf.append("\\t");
                case '\n' -> buf.append("\\n");
                case vtab -> buf.append("\\v");
                case '\f' -> buf.append("\\f");
                case '\r' -> buf.append("\\r");
                default -> {
                    if (isPrintable(c)) {
                        buf.append((char)c);
                    } else {
                        buf.append("\\").append(Integer.toOctalString(c));
                    }
                }
            }
        }
        buf.append("\"");
        return buf.toString();
    }

    static private int toUnsigned(byte b) {
        return b >= 0 ? b : 256 + b;
    }

    static public boolean isPrintable(int c) {
        return (' ' <= c) && (c <= '~');
    }
}

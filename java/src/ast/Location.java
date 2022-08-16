package mat.ast;

import mat.parser.Token;

import java.util.ArrayList;
import java.util.List;

public record Location(String sourceName, Token token) {
    public String toString() {
        return sourceName + ":" + lineno();
    }

    private List<Token> tokensWithoutFirstSpecials() {
        return buildTokenList(token, true);
    }

    private List<Token> buildTokenList(Token first, boolean rejectFirstSpecials) {
        List<Token> result = new ArrayList<>();
        boolean rejectSpecials = rejectFirstSpecials;
        for (Token t = first; t != null; t = t.next) {
            if (t.specialToken != null && !rejectSpecials) {
                Token s = specialTokenHead(t.specialToken);
                for (; s != null; s = s.next) {
                    result.add(s);
                }
            }
            result.add(t);
            rejectSpecials = false;
        }
        return result;
    }

    private Token specialTokenHead(Token firstSpecial) {
        Token s = firstSpecial;
        while (s.specialToken != null) {
            s = s.specialToken;
        }
        return s;
    }

    /** line number */
    public int lineno() {
        return token.beginLine;
    }

    public String line() {
        StringBuilder buf = new StringBuilder();
        for (Token t : tokensWithoutFirstSpecials()) {
            int idx = t.image.indexOf("\n");
            if (idx >= 0) {
                buf.append(t.image.substring(0, idx));
                break;
            }
            buf.append(t.image);
        }
        return buf.toString();
    }

    public String numberedLine() {
        return "line " + lineno() + ": " + line();
    }
}

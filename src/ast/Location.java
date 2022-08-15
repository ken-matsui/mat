package mat.ast;

import mat.parser.Token;

public record Location(String sourceName, Token token) {
    public String toString() {
        return sourceName + ":" + token.beginLine;
    }
}

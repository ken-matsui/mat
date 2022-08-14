package mat.ast;

import mat.parser.Token;

public record Location(String sourceName, Token token) {
}

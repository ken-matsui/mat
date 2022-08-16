package mat.compiler;

import mat.ast.*;
import mat.entity.*;
import mat.type.*;
import mat.utils.ErrorHandler;
import mat.exception.*;

import java.util.*;

class DereferenceChecker extends Visitor {
    // #@@range/ctor{
    private final TypeTable typeTable;
    private final ErrorHandler errorHandler;

    public DereferenceChecker(TypeTable typeTable, ErrorHandler h) {
        this.typeTable = typeTable;
        this.errorHandler = h;
    }
    // #@@}

    // #@@range/check_AST{
    public void check(AST ast) throws SemanticException {
        for (DefinedVariable var : ast.definedVariables()) {
            checkToplevelVariable(var);
        }
        for (DefinedFunction f : ast.definedFunctions()) {
            check(f.body());
        }
        if (errorHandler.errorOccurred()) {
            throw new SemanticException("compile failed.");
        }
    }
    // #@@}

    private void checkToplevelVariable(DefinedVariable var) {
        checkVariable(var);
        if (var.hasInitializer()) {
            checkConstant(var.initializer());
        }
    }

    private void checkConstant(ExprNode expr) {
        if (!expr.isConstant()) {
            errorHandler.error("not a constant", expr.location());
        }
    }

    // #@@range/check{
    private void check(StmtNode node) {
        node.accept(this);
    }

    private void check(ExprNode node) {
        node.accept(this);
    }
    // #@@}

    //
    // Statements
    //

    // #@@range/BlockNode{
    public Void visit(BlockNode node) {
        for (DefinedVariable var : node.variables()) {
            checkVariable(var);
        }
        for (StmtNode stmt : node.stmts()) {
            try {
                check(stmt);
            }
            catch (SemanticError err) {
                ;
            }
        }
        return null;
    }
    // #@@}

    private void checkVariable(DefinedVariable var) {
        if (var.hasInitializer()) {
            try {
                check(var.initializer());
            }
            catch (SemanticError err) {
                ;
            }
        }
    }

    //
    // Assignment Expressions
    //

    public Void visit(AssignNode node) {
        super.visit(node);
        checkAssignment(node);
        return null;
    }

    public Void visit(OpAssignNode node) {
        super.visit(node);
        checkAssignment(node);
        return null;
    }

    private void checkAssignment(AbstractAssignNode node) {
        if (! node.lhs().isAssignable()) {
            semanticError(node.location(), "invalid lhs expression");
        }
    }

    //
    // Expressions
    //

//    public Void visit(PrefixOpNode node) {
//        super.visit(node);
//        if (! node.expr().isAssignable()) {
//            semanticError(node.expr().location(),
//                    "cannot increment/decrement");
//        }
//        return null;
//    }
//
//    public Void visit(SuffixOpNode node) {
//        super.visit(node);
//        if (! node.expr().isAssignable()) {
//            semanticError(node.expr().location(),
//                    "cannot increment/decrement");
//        }
//        return null;
//    }

    public Void visit(FnCallNode node) {
        super.visit(node);
        if (! node.expr().isCallable()) {
            semanticError(node.location(),
                    "calling object is not a function");
        }
        return null;
    }

//    public Void visit(ArefNode node) {
//        super.visit(node);
//        if (! node.expr().isPointer()) {
//            semanticError(node.location(),
//                    "indexing non-array/pointer expression");
//        }
//        handleImplicitAddress(node);
//        return null;
//    }
//
//    public Void visit(MemberNode node) {
//        super.visit(node);
//        checkMemberRef(node.location(), node.expr().type(), node.member());
//        handleImplicitAddress(node);
//        return null;
//    }
//
//    public Void visit(PtrMemberNode node) {
//        super.visit(node);
//        if (! node.expr().isPointer()) {
//            undereferableError(node.location());
//        }
//        checkMemberRef(node.location(), node.dereferedType(), node.member());
//        handleImplicitAddress(node);
//        return null;
//    }

    private void checkMemberRef(Location loc, Type t, String memb) {
        if (! t.isCompositeType()) {
            semanticError(loc, "accessing member `" + memb
                    + "' for non-struct/union: " + t);
        }
        CompositeType type = t.getCompositeType();
        if (! type.hasMember(memb)) {
            semanticError(loc, type.toString()
                    + " does not have member: " + memb);
        }
    }

    // #@@range/DereferenceNode{
//    public Void visit(DereferenceNode node) {
//        super.visit(node);
//        if (! node.expr().isPointer()) {
//            undereferableError(node.location());
//        }
//        handleImplicitAddress(node);
//        return null;
//    }
    // #@@}

    // #@@range/AddressNode{
//    public Void visit(AddressNode node) {
//        super.visit(node);
//        if (! node.expr().isLvalue()) {
//            semanticError(node.location(), "invalid expression for &");
//        }
//        Type base = node.expr().type();
//        if (! node.expr().isLoadable()) {
//            // node.expr.type is already pointer.
//            node.setType(base);
//        }
//        else {
//            node.setType(typeTable.pointerTo(base));
//        }
//        return null;
//    }
    // #@@}

    public Void visit(VariableNode node) {
        super.visit(node);
        if (node.entity().isConstant()) {
            checkConstant(node.entity().value());
        }
        handleImplicitAddress(node);
        return null;
    }

    public Void visit(CastNode node) {
        super.visit(node);
        if (node.type().isArray()) {
            semanticError(node.location(), "cast specifies array type");
        }
        return null;
    }

    //
    // Utilities
    //

    private void handleImplicitAddress(LHSNode node) {
        if (! node.isLoadable()) {
            Type t = node.type();
            if (t.isArray()) {
                // int[4] ary; ary; should generate int*
                node.setType(typeTable.pointerTo(t.baseType()));
            }
            else {
                node.setType(typeTable.pointerTo(t));
            }
        }
    }

    private void undereferableError(Location loc) {
        semanticError(loc, "dereferencing non-pointer expression");
    }

    private void semanticError(Node n, String msg) {
        semanticError(n.location(), msg);
    }

    private void semanticError(Location loc, String msg) {
        errorHandler.error(msg, loc);
        throw new SemanticError("invalid expr");
    }
}

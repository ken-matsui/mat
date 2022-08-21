use crate::parser::ast::Ast;
use crate::sema::entity::Entity;
use crate::sema::error::SemanticError;
use crate::sema::toplevel_scope::ToplevelScope;

pub(crate) fn resolve(ast: Ast) -> Result<(), SemanticError> {
    // toplevel scope
    let mut toplevel = ToplevelScope::default();

    for stmt in ast.defs {
        // Convert DefVar & DefFn into Entities and define the entity.
        if let Ok(entity) = Entity::try_from((*stmt).clone()) {
            toplevel.define_entity(entity)?;
        }
    }

    Ok(())
    // toplevel scope end
}

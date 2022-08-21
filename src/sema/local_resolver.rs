/// Local Resolver
use crate::parser::ast::Ast;
use crate::sema::entity::Entity;
use crate::sema::error::SemanticError;
use crate::sema::toplevel_scope::ToplevelScope;

pub(crate) fn resolve(ast: Ast) -> Result<(), Vec<SemanticError>> {
    // toplevel scope
    let mut toplevel = ToplevelScope::default();
    let mut errors = Vec::<SemanticError>::new();

    for stmt in ast.defs {
        // Convert DefVar & DefFn into Entities and define the entity.
        if let Ok(entity) = Entity::try_from((*stmt).clone()) {
            if let Err(err) = toplevel.define_entity(entity) {
                errors.push(err);
            }
        }
    }
    // toplevel scope end

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn resolve_gvar_initializers() {}

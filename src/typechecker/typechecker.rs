use crate::parser::ast::{Exp, Program, Type};

pub enum TypeCheckError {}

pub struct TypeCheckRes {
    type_defs: Vec<Type>,
    fn_defs: Vec<FnDef>,
}

pub struct FnDef {
    reciever: Option<Type>,
    name: String,
    params: Vec<Type>,
    return_ty: Type,
}

/*
    Jobs:
    1) Save defined Types
    2) Check all functions return what they say they do
    3) Check all function calls get the types they want
    4) Check all the types of the variables
*/

pub fn typecheck(program: &mut Program) -> Result<TypeCheckRes, TypeCheckError> {
    Ok(())
}

pub fn typecheck_exp(exp: &mut Exp) -> Type {
    Type::Bool(false)
}

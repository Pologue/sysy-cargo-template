use crate::ast::CompUnit;
use koopa::front::ast::{FunDef, IntVal};
use koopa::front::builder::Builder;
use koopa::front::span::{Pos, Span};
use koopa::ir::entities::Program;


pub fn ast2ir(ast: &CompUnit) -> Program {
    let ret = IntVal::new_boxed(Span::new(Pos::new()), ast.func_def.block.stmt.num);
    let ret1 = IntVal::new_boxed(Span::new(Pos::new()), ast.func_def.block.stmt.num);
    let block = koopa::front::ast::Block::new_boxed(
        Span::new(Pos::new()),
        String::from("%entry"),
        vec![],
        vec![ret],
    );
    let func_def = FunDef::new_boxed(
        Span::new(Pos::new()),
        ast.func_def.ident.clone(),
        vec![],
        Some(ret1),
        vec![block],
    );

    let mut builder = Builder::new();
    builder.build_on(&func_def);

    builder.program()
}

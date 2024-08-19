use crate::ast::CompUnit;
use koopa::ir::builder::{BasicBlockBuilder, LocalInstBuilder, ValueBuilder};
use koopa::ir::entities::Program;
use koopa::ir::{FunctionData, Type};


pub fn ast2ir(ast: &CompUnit) -> Program {
    let name = ast.func_def.ident.clone();

    let mut program = Program::new();
    let main = program.new_func(
        FunctionData::new(format!("@{name}").into(), Vec::new(), Type::get_i32()),
    );
    let main_data = program.func_mut(main);

    let entry = main_data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
    let _ = main_data.layout_mut().bbs_mut().push_key_back(entry);

    let num = main_data.dfg_mut().new_value().integer(ast.func_def.block.stmt.num);
    let ret = main_data.dfg_mut().new_value().ret(Some(num));
    let _ = main_data.layout_mut().bb_mut(entry).insts_mut().push_key_back(ret);

    program
}

use koopa::ir::{entities::ValueData, FunctionData, Program, ValueKind};

pub trait GenerateAsm {
    fn generate(&self) -> String;
}

impl GenerateAsm for Program {
    fn generate(&self) -> String {
        let mut asm = String::new();

        let program = self;
        asm.push_str("  .text\n");
        
        for &func in program.func_layout() {
            let func_data = program.func(func);
            // remove '@'
            let name = func_data.name().trim_start_matches('@');
            asm.push_str(format!("  .global {}\n", name).as_str());
            asm.push_str(func_data.generate().as_str());
        }

        asm
    }
}

impl GenerateAsm for FunctionData {
    fn generate(&self) -> String {
        let mut asm = String::new();

        let func_data = self;
        // remove '@'
        let name = func_data.name().trim_start_matches('@');
        asm.push_str(&format!("{}:\n", name));

        for (&bb, node) in func_data.layout().bbs() {

            for &inst in node.insts().keys() {
                let value_data = func_data.dfg().value(inst);
                // asm.push_str(value_data.generate().as_str());
                match value_data.kind() {
                    ValueKind::Integer(int) => {
                        asm.push_str(&format!("  li a0, {:?}\n", int));
                    }
                    ValueKind::Return(ret) => {
                        asm.push_str(format!("  li a0, {:?}\n", ret.value().unwrap()).as_str());
                        asm.push_str("  ret\n");
                    }
                    _ => unreachable!(),
                }
            }
        }

        asm
    }
}

// impl GenerateAsm for ValueData {
//     fn generate(&self) -> String {
//         let mut asm = String::new();

//         let value_data = self;

//         match value_data.kind() {
//             ValueKind::Integer(int) => {
//                 asm.push_str(&format!("  mov rax, {:?}\n", int));
//             }
//             ValueKind::Return(ret) => {
//                 //TODO: Value(1073741824) -> 0
//                 let val = ret.value().unwrap();
//                 asm.push_str(format!("  li a0, {:?}\n", val).as_str());
//                 asm.push_str("  ret\n");
//             }
//             _ => unreachable!(),
//         }

//         asm
//     }
// }
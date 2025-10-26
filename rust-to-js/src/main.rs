use std::{collections::BTreeSet, ffi::CStr, path::Path};

use inkwell::{
    basic_block::BasicBlock, context::Context, memory_buffer::MemoryBuffer, module::Module,
    values::*,
};

fn main() {
    let context = Context::create();
    let path = Path::new("./sample-projects/hello-world.ll");
    let memory_buffer = MemoryBuffer::create_from_file(path).unwrap();
    let module = context.create_module_from_ir(memory_buffer).unwrap();
    generate(&module);
}

fn generate(module: &Module) {
    let mut fn_to_generates: BTreeSet<&CStr> = BTreeSet::new();
    let mut fn_generated: BTreeSet<&CStr> = BTreeSet::new();

    fn_to_generates.insert(CStr::from_bytes_with_nul(b"main\0").unwrap());

    loop {
        if fn_to_generates.is_empty() {
            break;
        }
        let function_name = fn_to_generates.pop_first().unwrap();
        fn_generated.insert(function_name);
        let mut function_generator = FunctionGenerator {
            fn_to_generates: &mut fn_to_generates,
            fn_generated: &mut fn_generated,
            module,
            local_unnamed_var_index: 0,
        };
        function_generator.generate_function(function_name);
    }
}

struct FunctionGenerator<'a, 'ctx> {
    fn_to_generates: &'a mut BTreeSet<&'ctx CStr>,
    fn_generated: &'a mut BTreeSet<&'ctx CStr>,
    module: &'a Module<'ctx>,
    local_unnamed_var_index: usize,
}
impl<'a, 'ctx> FunctionGenerator<'a, 'ctx> {
    fn generate_function(&mut self, function_name: &CStr) {
        let function_name = function_name.to_str().unwrap();
        let function = self.module.get_function(function_name).unwrap();

        print!("function {function_name}");

        print!("(");
        for (i, arg) in function.get_param_iter().enumerate() {
            if i > 0 {
                print!(", ");
            }

            self.print_local_name(arg.get_name());
        }
        print!(")");

        println!(" {{");

        for basic_block in function.get_basic_block_iter() {
            self.generate_bb(basic_block);
        }

        println!("}}");
    }

    fn generate_bb(&mut self, basic_block: BasicBlock<'_>) {
        let name = basic_block.get_name().to_str().unwrap();
        println!("function {name}() {{");
        for instruction in basic_block.get_instructions() {
            match instruction.get_opcode() {
                InstructionOpcode::Add => todo!(),
                InstructionOpcode::AddrSpaceCast => todo!(),
                InstructionOpcode::Alloca => todo!(),
                InstructionOpcode::And => todo!(),
                InstructionOpcode::AShr => todo!(),
                InstructionOpcode::AtomicCmpXchg => todo!(),
                InstructionOpcode::AtomicRMW => todo!(),
                InstructionOpcode::BitCast => todo!(),
                InstructionOpcode::Br => todo!(),
                InstructionOpcode::Call => todo!(),
                InstructionOpcode::CallBr => todo!(),
                InstructionOpcode::CatchPad => todo!(),
                InstructionOpcode::CatchRet => todo!(),
                InstructionOpcode::CatchSwitch => todo!(),
                InstructionOpcode::CleanupPad => todo!(),
                InstructionOpcode::CleanupRet => todo!(),
                InstructionOpcode::ExtractElement => todo!(),
                InstructionOpcode::ExtractValue => todo!(),
                InstructionOpcode::FNeg => todo!(),
                InstructionOpcode::FAdd => todo!(),
                InstructionOpcode::FCmp => todo!(),
                InstructionOpcode::FDiv => todo!(),
                InstructionOpcode::Fence => todo!(),
                InstructionOpcode::FMul => todo!(),
                InstructionOpcode::FPExt => todo!(),
                InstructionOpcode::FPToSI => todo!(),
                InstructionOpcode::FPToUI => todo!(),
                InstructionOpcode::FPTrunc => todo!(),
                InstructionOpcode::Freeze => todo!(),
                InstructionOpcode::FRem => todo!(),
                InstructionOpcode::FSub => todo!(),
                InstructionOpcode::GetElementPtr => todo!(),
                InstructionOpcode::ICmp => todo!(),
                InstructionOpcode::IndirectBr => todo!(),
                InstructionOpcode::InsertElement => todo!(),
                InstructionOpcode::InsertValue => todo!(),
                InstructionOpcode::IntToPtr => todo!(),
                InstructionOpcode::Invoke => todo!(),
                InstructionOpcode::LandingPad => todo!(),
                InstructionOpcode::Load => todo!(),
                InstructionOpcode::LShr => todo!(),
                InstructionOpcode::Mul => todo!(),
                InstructionOpcode::Or => todo!(),
                InstructionOpcode::Phi => todo!(),
                InstructionOpcode::PtrToInt => todo!(),
                InstructionOpcode::Resume => todo!(),
                InstructionOpcode::Return => todo!(),
                InstructionOpcode::SDiv => todo!(),
                InstructionOpcode::Select => todo!(),
                InstructionOpcode::SExt => {
                    println!("instruction: {instruction:?}");
                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    println!("left: {left:?}");
                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    self.print_local_name(instruction.get_name().unwrap());
                    let rhs = println!("ops_sext({left}, {width}, {dest_width});");
                }
                InstructionOpcode::Shl => todo!(),
                InstructionOpcode::ShuffleVector => todo!(),
                InstructionOpcode::SIToFP => todo!(),
                InstructionOpcode::SRem => todo!(),
                InstructionOpcode::Store => todo!(),
                InstructionOpcode::Sub => todo!(),
                InstructionOpcode::Switch => todo!(),
                InstructionOpcode::Trunc => todo!(),
                InstructionOpcode::UDiv => todo!(),
                InstructionOpcode::UIToFP => todo!(),
                InstructionOpcode::Unreachable => todo!(),
                InstructionOpcode::URem => todo!(),
                InstructionOpcode::UserOp1 => todo!(),
                InstructionOpcode::UserOp2 => todo!(),
                InstructionOpcode::VAArg => todo!(),
                InstructionOpcode::Xor => todo!(),
                InstructionOpcode::ZExt => todo!(),
            }
        }
        println!("}}");
    }
    fn print_local_name(&mut self, name: &CStr) {
        if name.is_empty() {
            print!("l{}", self.local_unnamed_var_index);
            self.local_unnamed_var_index += 1;
        } else {
            print!("{}", name.to_str().unwrap());
        };
    }
}

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ffi::CStr,
    path::Path,
};

use inkwell::{
    basic_block::BasicBlock, context::Context, llvm_sys::prelude::LLVMValueRef,
    memory_buffer::MemoryBuffer, module::Module, values::*,
};

fn main() {
    let context = Context::create();
    let path = Path::new("./sample-projects/hello-world.ll");
    let memory_buffer = MemoryBuffer::create_from_file(path).unwrap();
    let module = context.create_module_from_ir(memory_buffer).unwrap();
    generate(&module);
}

fn generate(module: &Module) {
    for function in module.get_functions() {
        let mut function_generator = FunctionGenerator {
            module,
            next_unnamed_local_name: Default::default(),
            unnamed_local_names: Default::default(),
        };
        function_generator.generate_function(function.get_name());
    }
}

struct FunctionGenerator<'a, 'ctx> {
    module: &'a Module<'ctx>,
    next_unnamed_local_name: usize,
    unnamed_local_names: HashMap<LLVMValueRef, usize>,
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

            self.print_value_name(arg);
        }
        print!(")");

        println!(" {{");

        for basic_block in function.get_basic_block_iter() {
            self.generate_bb(basic_block);
        }

        println!("}}");
    }

    fn generate_bb(&mut self, basic_block: BasicBlock<'ctx>) {
        let name = basic_block.get_name().to_str().unwrap();
        println!("function {name}() {{");
        for instruction in basic_block.get_instructions() {
            match instruction.get_opcode() {
                InstructionOpcode::Add => todo!(),
                InstructionOpcode::AddrSpaceCast => todo!(),
                InstructionOpcode::Alloca => {
                    todo!()
                }
                InstructionOpcode::And => todo!(),
                InstructionOpcode::AShr => todo!(),
                InstructionOpcode::AtomicCmpXchg => todo!(),
                InstructionOpcode::AtomicRMW => todo!(),
                InstructionOpcode::BitCast => todo!(),
                InstructionOpcode::Br => todo!(),
                InstructionOpcode::Call => {
                    self.print_value_name(instruction);
                    print!(" = ops_call(");
                    let num_operands = instruction.get_num_operands();
                    self.print_value_name(
                        instruction
                            .get_operand(num_operands - 1)
                            .unwrap()
                            .unwrap_left(),
                    );
                    for operand in instruction
                        .get_operands()
                        .take(num_operands as usize - 1)
                        .map(|x| x.unwrap().unwrap_left())
                    {
                        print!(", ");
                        self.print_value_name(operand);
                    }
                    println!(");")
                }
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
                InstructionOpcode::Return => {
                    print!("return ");
                    let Some(operand) = instruction.get_operand(0) else {
                        println!(";");
                        return;
                    };
                    self.print_value_name(operand.unwrap_left());
                    println!(";");
                }
                InstructionOpcode::SDiv => todo!(),
                InstructionOpcode::Select => todo!(),
                InstructionOpcode::SExt => {
                    self.print_value_name(instruction);
                    print!(" = ops_sext(");

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_value_name(left);

                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    println!(", {width}, {dest_width});");
                }
                InstructionOpcode::Shl => todo!(),
                InstructionOpcode::ShuffleVector => todo!(),
                InstructionOpcode::SIToFP => todo!(),
                InstructionOpcode::SRem => todo!(),
                InstructionOpcode::Store => todo!(),
                InstructionOpcode::Sub => todo!(),
                InstructionOpcode::Switch => todo!(),
                InstructionOpcode::Trunc => {
                    self.print_value_name(instruction);
                    print!(" = ops_trunc(");

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_value_name(left);

                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    println!(", {width}, {dest_width});");
                }
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
    fn print_value_name(&mut self, value: impl Into<AnyValueEnum<'ctx>>) {
        let value = value.into();
        let mut with_name = |name: &CStr| {
            if !name.is_empty() {
                print!("{}", name.to_str().unwrap());
            } else if let Some(index) = self.unnamed_local_names.get(&value.as_value_ref()) {
                print!("l{index}");
            } else {
                let name = self.next_unnamed_local_name;
                self.next_unnamed_local_name += 1;
                self.unnamed_local_names.insert(value.as_value_ref(), name);
                print!("l{name}");
            };
        };
        match value {
            AnyValueEnum::ArrayValue(array_value) => todo!(),
            AnyValueEnum::IntValue(int_value) => {
                if !int_value.is_constant_int() {
                    with_name(int_value.get_name());
                    return;
                }
                let value = int_value.get_zero_extended_constant().unwrap();
                let width = int_value.get_type().get_bit_width();
                match width {
                    1 => print!("{}", if value == 0 { "false" } else { "true" }),
                    8 => print!(
                        "{}",
                        i8::from_ne_bytes(value.to_ne_bytes()[0..1].try_into().unwrap())
                    ),
                    16 => print!(
                        "{}",
                        i16::from_ne_bytes(value.to_ne_bytes()[0..2].try_into().unwrap())
                    ),
                    32 => print!(
                        "{}",
                        i32::from_ne_bytes(value.to_ne_bytes()[0..4].try_into().unwrap())
                    ),
                    64 => print!(
                        "{}",
                        i64::from_ne_bytes(value.to_ne_bytes()[0..8].try_into().unwrap())
                    ),
                    _ => todo!(),
                }
            }
            AnyValueEnum::FloatValue(float_value) => with_name(float_value.get_name()),
            AnyValueEnum::PhiValue(phi_value) => todo!(),
            AnyValueEnum::FunctionValue(function_value) => with_name(function_value.get_name()),
            AnyValueEnum::PointerValue(pointer_value) => with_name(pointer_value.get_name()),
            AnyValueEnum::StructValue(struct_value) => todo!(),
            AnyValueEnum::VectorValue(vector_value) => todo!(),
            AnyValueEnum::ScalableVectorValue(scalable_vector_value) => todo!(),
            AnyValueEnum::InstructionValue(instruction_value) => {
                with_name(instruction_value.get_name().unwrap())
            }
            AnyValueEnum::MetadataValue(metadata_value) => todo!(),
        };
    }
}

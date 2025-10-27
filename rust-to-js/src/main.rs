use inkwell::{
    basic_block::BasicBlock,
    context::Context,
    llvm_sys::prelude::LLVMValueRef,
    memory_buffer::MemoryBuffer,
    module::Module,
    targets::{InitializationConfig, Target, TargetData},
    types::BasicType,
    values::*,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ffi::CStr,
    path::Path,
};

fn main() {
    let context = Context::create();
    let path = Path::new("./sample-projects/hello-world.ll");
    let memory_buffer = MemoryBuffer::create_from_file(path).unwrap();
    let module = context.create_module_from_ir(memory_buffer).unwrap();
    generate(&module);
}

fn generate(module: &Module) {
    Target::initialize_webassembly(&InitializationConfig::default());

    let engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();

    let target_data = engine.get_target_data();

    for function in module.get_functions() {
        if function.count_basic_blocks() == 0 {
            continue;
        }
        let function_generator = FunctionGenerator {
            module,
            next_unnamed_local_index: Default::default(),
            unnamed_local_names: Default::default(),
            target_data,
            used_local_names: Default::default(),
        };
        function_generator.generate_function(function.get_name());
    }

    println!("__main_void();");
}

struct FunctionGenerator<'a, 'ctx> {
    module: &'a Module<'ctx>,
    next_unnamed_local_index: usize,
    unnamed_local_names: HashMap<LLVMValueRef, String>,
    target_data: &'a TargetData,
    used_local_names: BTreeSet<String>,
}
impl<'a, 'ctx> FunctionGenerator<'a, 'ctx> {
    fn generate_function(mut self, function_name: &CStr) {
        let function_name = function_name.to_str().unwrap();
        let function = self.module.get_function(function_name).unwrap();

        print!("function {}", normalize_name(function_name));

        print!("(");
        for (i, arg) in function.get_param_iter().enumerate() {
            if i > 0 {
                print!(", ");
            }

            self.print_local(arg);
        }
        print!(")");

        println!(" {{");

        for basic_block in function.get_basic_block_iter() {
            self.generate_bb(basic_block);
        }

        for name in self.used_local_names.iter() {
            println!("    let {name};");
        }

        println!(
            "    {}();",
            normalize_name(
                function
                    .get_first_basic_block()
                    .unwrap()
                    .get_name()
                    .to_str()
                    .unwrap()
            )
        );

        println!("}}");
    }

    fn generate_bb(&mut self, basic_block: BasicBlock<'ctx>) {
        let name = basic_block.get_name().to_str().unwrap();
        println!("    function {}() {{", normalize_name(name));
        for instruction in basic_block.get_instructions() {
            print!("        ");
            match instruction.get_opcode() {
                InstructionOpcode::Add => todo!(),
                InstructionOpcode::AddrSpaceCast => todo!(),
                InstructionOpcode::Alloca => {
                    self.print_lhs_local(instruction);

                    let size = self
                        .target_data
                        .get_store_size(&instruction.get_allocated_type().unwrap());
                    println!(" = ops_alloca({size});");
                }
                InstructionOpcode::And => todo!(),
                InstructionOpcode::AShr => todo!(),
                InstructionOpcode::AtomicCmpXchg => todo!(),
                InstructionOpcode::AtomicRMW => todo!(),
                InstructionOpcode::BitCast => todo!(),
                InstructionOpcode::Br => todo!(),
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
                InstructionOpcode::GetElementPtr => {}
                InstructionOpcode::ICmp => todo!(),
                InstructionOpcode::IndirectBr => todo!(),
                InstructionOpcode::InsertElement => todo!(),
                InstructionOpcode::InsertValue => {
                    print!("ops_insert_value(");

                    let target = instruction
                        .get_operand(0)
                        .unwrap()
                        .unwrap_left()
                        .into_struct_value();

                    if target.is_undef() {
                        let type_size = self.target_data.get_store_size(&target.get_type());
                        print!("poisonStruct({type_size})");
                    } else {
                        self.print_local(target);
                    }

                    print!(", ");

                    let value = instruction.get_operand(1).unwrap().unwrap_left();
                    self.print_local(value);

                    let index = instruction.get_indices()[0];

                    let offset = target
                        .get_fields()
                        .take(index as usize)
                        .map(|field| self.target_data.get_store_size(&field.get_type()))
                        .sum::<u64>();
                    println!(", {});", offset);
                }
                InstructionOpcode::IntToPtr => todo!(),
                InstructionOpcode::LandingPad => unreachable!("we use panic=abort"),
                InstructionOpcode::Load => {
                    self.print_lhs_local(instruction);
                    print!(" = ops_load(");

                    let pointer = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_local(pointer);

                    print!(", ");

                    let size = self.target_data.get_store_size(&instruction.get_type());
                    print!("{size}");

                    println!(");");
                }
                InstructionOpcode::LShr => todo!(),
                InstructionOpcode::Mul => todo!(),
                InstructionOpcode::Or => todo!(),
                InstructionOpcode::Phi => todo!(),
                InstructionOpcode::PtrToInt => todo!(),
                InstructionOpcode::SDiv => todo!(),
                InstructionOpcode::Select => todo!(),
                InstructionOpcode::SExt => {
                    self.print_lhs_local(instruction);
                    print!(" = ops_sext(");

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_local(left);

                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    println!(", {width}, {dest_width});");
                }
                InstructionOpcode::Shl => todo!(),
                InstructionOpcode::ShuffleVector => todo!(),
                InstructionOpcode::SIToFP => todo!(),
                InstructionOpcode::SRem => todo!(),
                InstructionOpcode::Store => {
                    print!("ops_store(");

                    let pointer = instruction.get_operand(1).unwrap().unwrap_left();
                    self.print_local(pointer);

                    print!(", ");

                    let value = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_local(value);

                    println!(");");
                }
                InstructionOpcode::Sub => todo!(),
                InstructionOpcode::Switch => todo!(),
                InstructionOpcode::Trunc => {
                    self.print_lhs_local(instruction);
                    print!(" = ops_trunc(");

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_local(left);

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
                InstructionOpcode::ZExt => {
                    self.print_lhs_local(instruction);
                    print!(" = ops_zext(");

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_local(left);

                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    println!(", {width}, {dest_width});");
                }
                // Termiantors
                InstructionOpcode::Call => {
                    if !instruction.get_type().is_void_type() {
                        self.print_lhs_local(instruction);
                        print!(" = ")
                    }
                    print!("ops_call(");
                    let num_operands = instruction.get_num_operands();
                    self.print_local(
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
                        self.print_local(operand);
                    }
                    println!(");");
                }
                InstructionOpcode::Invoke => {
                    print!("try {{");
                    if !instruction.get_type().is_void_type() {
                        self.print_lhs_local(instruction);
                        print!(" = ")
                    }
                    print!("ops_invoke(");
                    let num_operands = instruction.get_num_operands();
                    self.print_local(
                        instruction
                            .get_operand(num_operands - 1)
                            .unwrap()
                            .unwrap_left(),
                    );
                    for operand in instruction
                        .get_operands()
                        .take(num_operands as usize - 3)
                        .map(|x| x.unwrap().unwrap_left())
                    {
                        print!(", ");
                        self.print_local(operand);
                    }

                    let to_name = instruction
                        .get_operand(num_operands - 3)
                        .unwrap()
                        .unwrap_right();
                    let unwind_name = instruction
                        .get_operand(num_operands - 2)
                        .unwrap()
                        .unwrap_right();
                    println!(
                        "); {}(); }} catch(_) {{ {}() }}",
                        normalize_name(to_name.get_name().to_str().unwrap()),
                        normalize_name(unwind_name.get_name().to_str().unwrap())
                    );
                }
                InstructionOpcode::Return => {
                    let Some(operand) = instruction.get_operand(0) else {
                        println!("return;");
                        continue;
                    };
                    print!("return ");
                    self.print_local(operand.unwrap_left());
                    println!(";");
                }
                InstructionOpcode::Resume => unreachable!("we use panic=abort"),
            }
        }

        println!("    }}");
    }

    fn print_local(&mut self, value: impl Into<AnyValueEnum<'ctx>>) {
        let value = value.into();
        let name = self.value_name(value);
        print!("{name}");
    }

    fn print_lhs_local(&mut self, value: impl Into<AnyValueEnum<'ctx>>) {
        let value = value.into();
        let name = self.value_name(value);
        self.used_local_names.insert(name.clone());
        print!("{name}");
    }

    fn value_name(&mut self, value: impl Into<AnyValueEnum<'ctx>>) -> String {
        let value = value.into();
        let mut name = normalize_name(&match value {
            AnyValueEnum::ArrayValue(array_value) => todo!(),
            AnyValueEnum::IntValue(int_value) => {
                if !int_value.is_constant_int() {
                    int_value.get_name().to_string()
                } else {
                    let value = int_value.get_zero_extended_constant().unwrap();
                    let width = int_value.get_type().get_bit_width();
                    match width {
                        1 => (if value == 0 { "false" } else { "true" }).to_string(),
                        8 => format!(
                            "{}",
                            i8::from_ne_bytes(value.to_ne_bytes()[0..1].try_into().unwrap())
                        ),
                        16 => format!(
                            "{}",
                            i16::from_ne_bytes(value.to_ne_bytes()[0..2].try_into().unwrap())
                        ),
                        32 => format!(
                            "{}",
                            i32::from_ne_bytes(value.to_ne_bytes()[0..4].try_into().unwrap())
                        ),
                        64 => format!(
                            "{}",
                            i64::from_ne_bytes(value.to_ne_bytes()[0..8].try_into().unwrap())
                        ),
                        _ => todo!(),
                    }
                }
            }
            AnyValueEnum::FloatValue(float_value) => float_value.get_name().to_string(),
            AnyValueEnum::PhiValue(phi_value) => todo!(),
            AnyValueEnum::FunctionValue(function_value) => function_value.get_name().to_string(),
            AnyValueEnum::PointerValue(pointer_value) => pointer_value.get_name().to_string(),
            AnyValueEnum::StructValue(struct_value) => struct_value.get_name().to_string(),
            AnyValueEnum::VectorValue(vector_value) => todo!(),
            AnyValueEnum::ScalableVectorValue(scalable_vector_value) => todo!(),
            AnyValueEnum::InstructionValue(instruction_value) => {
                instruction_value.get_name().unwrap().to_string()
            }
            AnyValueEnum::MetadataValue(metadata_value) => todo!(),
        });

        if name.is_empty() {
            if let Some(index) = self.unnamed_local_names.get(&value.as_value_ref()) {
                name = format!("l{index}");
            } else {
                name = format!("l{}", self.next_unnamed_local_index);
                self.next_unnamed_local_index += 1;
                self.unnamed_local_names
                    .insert(value.as_value_ref(), name.clone());
            }
        }

        name
    }
}

fn normalize_name(name: &str) -> String {
    name.replace(".", "_")
}

trait CStrHelper {
    fn to_string(self) -> String;
}
impl CStrHelper for &CStr {
    fn to_string(self) -> String {
        self.to_str().unwrap().to_string()
    }
}

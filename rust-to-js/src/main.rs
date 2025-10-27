use inkwell::{
    basic_block::BasicBlock,
    context::Context,
    llvm_sys::prelude::LLVMValueRef,
    memory_buffer::MemoryBuffer,
    module::Module,
    targets::{InitializationConfig, Target, TargetData},
    types::BasicTypeEnum,
    values::*,
};
use std::{
    collections::{BTreeSet, HashMap},
    ffi::CStr,
    path::Path,
};

fn main() {
    println!("{}", include_str!("glue.js"));
    let context = Context::create();
    let path = Path::new("./deps2/std-d63e8b2d5e91690b.ll");
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

            self.print_value(arg);
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
                InstructionOpcode::AddrSpaceCast => todo!(),
                InstructionOpcode::BitCast => todo!(),
                InstructionOpcode::CallBr => todo!(),
                InstructionOpcode::CatchPad => todo!(),
                InstructionOpcode::CatchRet => todo!(),
                InstructionOpcode::CatchSwitch => todo!(),
                InstructionOpcode::CleanupPad => todo!(),
                InstructionOpcode::CleanupRet => todo!(),
                InstructionOpcode::ExtractElement => todo!(),
                InstructionOpcode::FCmp => todo!(),
                InstructionOpcode::FPExt => todo!(),
                InstructionOpcode::FPToSI => todo!(),
                InstructionOpcode::FPToUI => todo!(),
                InstructionOpcode::FPTrunc => todo!(),
                InstructionOpcode::Freeze => todo!(),
                InstructionOpcode::ICmp => {
                    // eq: yields true if the operands are equal, false otherwise. No sign interpretation is necessary or performed.
                    // ne: yields true if the operands are unequal, false otherwise. No sign interpretation is necessary or performed.
                    // ugt: interprets the operands as unsigned values and yields true if op1 is greater than op2.
                    // uge: interprets the operands as unsigned values and yields true if op1 is greater than or equal to op2.
                    // ult: interprets the operands as unsigned values and yields true if op1 is less than op2.
                    // ule: interprets the operands as unsigned values and yields true if op1 is less than or equal to op2.
                    // sgt: interprets the operands as signed values and yields true if op1 is greater than op2.
                    // sge: interprets the operands as signed values and yields true if op1 is greater than or equal to op2.
                    // slt: interprets the operands as signed values and yields true if op1 is less than op2.
                    // sle: interprets the operands as signed values and yields true if op1 is less than or equal to op2.

                    // <result> = icmp eq i32 4, 5          ; yields: result=false
                    // <result> = icmp ne ptr %X, %X        ; yields: result=false
                    // <result> = icmp ult i16  4, 5        ; yields: result=true
                    // <result> = icmp sgt i16  4, 5        ; yields: result=false
                    // <result> = icmp ule i16 -4, 5        ; yields: result=false
                    // <result> = icmp sge i16  4, 5        ; yields: result=false

                    self.print_lhs_local(instruction);

                    print!(" = ");

                    match instruction.get_icmp_predicate().unwrap() {
                        inkwell::IntPredicate::EQ => print!("icmp_eq("),
                        inkwell::IntPredicate::NE => print!("icmp_ne("),
                        inkwell::IntPredicate::UGT => print!("icmp_ugt("),
                        inkwell::IntPredicate::UGE => print!("icmp_uge("),
                        inkwell::IntPredicate::ULT => print!("icmp_ult("),
                        inkwell::IntPredicate::ULE => print!("icmp_ule("),
                        inkwell::IntPredicate::SGT => print!("icmp_sgt("),
                        inkwell::IntPredicate::SGE => print!("icmp_sge("),
                        inkwell::IntPredicate::SLT => print!("icmp_slt("),
                        inkwell::IntPredicate::SLE => print!("icmp_sle("),
                    }

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    let right = instruction.get_operand(1).unwrap().unwrap_left();

                    println!(
                        "{}, {});",
                        self.as_bytes_string(left),
                        self.as_bytes_string(right)
                    );
                }
                InstructionOpcode::IndirectBr => todo!(),
                InstructionOpcode::InsertElement => todo!(),
                InstructionOpcode::ExtractValue => {
                    // <result> = extractvalue {i32, float} %agg, 0    ; yields i32

                    self.print_lhs_local(instruction);
                    print!(" = ops_extract_value(");

                    let value = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_value(value);

                    print!(", ");

                    let index = instruction.get_indices()[0];

                    let offset = value
                        .into_struct_value()
                        .get_fields()
                        .take(index as usize)
                        .map(|field| self.target_data.get_store_size(&field.get_type()))
                        .sum::<u64>();
                    println!("{offset});");
                }
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
                        self.print_value(target);
                    }

                    print!(", ");

                    let value = instruction.get_operand(1).unwrap().unwrap_left();
                    self.print_value(value);

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
                InstructionOpcode::Phi => todo!(),
                InstructionOpcode::PtrToInt => {
                    let int_type = instruction.get_type().into_int_type();
                    if int_type.get_bit_width() == 32 {
                        // no_op
                        continue;
                    }
                    todo!()
                }
                InstructionOpcode::Select => {
                    self.print_lhs_local(instruction);
                    print!(" = ops_select(");

                    let condition = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_value(condition);

                    print!(", ");

                    let true_value = instruction.get_operand(1).unwrap().unwrap_left();
                    self.print_value(true_value);

                    print!(", ");

                    let false_value = instruction.get_operand(2).unwrap().unwrap_left();
                    self.print_value(false_value);

                    println!(");");
                }
                InstructionOpcode::SExt => {
                    self.print_lhs_local(instruction);
                    print!(" = ops_sext(");

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_value(left);

                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    println!(", {width}, {dest_width});");
                }
                InstructionOpcode::ShuffleVector => todo!(),
                InstructionOpcode::SIToFP => todo!(),
                InstructionOpcode::Trunc => {
                    self.print_lhs_local(instruction);
                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    println!(" = ops_trunc({left}, {width}, {dest_width});");
                }
                InstructionOpcode::UIToFP => todo!(),
                InstructionOpcode::Unreachable => {
                    println!("throw new Error('unreachable');");
                }
                InstructionOpcode::UserOp1 => todo!(),
                InstructionOpcode::UserOp2 => todo!(),
                InstructionOpcode::VAArg => todo!(),
                InstructionOpcode::ZExt => {
                    self.print_lhs_local(instruction);
                    print!(" = ops_zext(");

                    let left = instruction.get_operand(0).unwrap().unwrap_left();
                    self.print_value(left);

                    let width = left.get_type().into_int_type().get_bit_width();
                    let dest_width = instruction.get_type().into_int_type().get_bit_width();
                    println!(", {width}, {dest_width});");
                }
                InstructionOpcode::FNeg => todo!(),

                // Memory Access and Addressing Operations
                InstructionOpcode::Alloca => {
                    self.print_lhs_local(instruction);

                    let size = self
                        .target_data
                        .get_store_size(&instruction.get_allocated_type().unwrap());
                    println!(" = ops_alloca({size});");
                }
                InstructionOpcode::Load => {
                    self.print_lhs_local(instruction);
                    let size = self.target_data.get_store_size(&instruction.get_type());
                    let pointer = instruction.get_operand(0).unwrap().unwrap_left();
                    println!(" = ops_load({}, {size});", self.as_bytes_string(pointer));
                }
                InstructionOpcode::Store => {
                    let value = instruction.get_operand(0).unwrap().unwrap_left();
                    let pointer = instruction.get_operand(1).unwrap().unwrap_left();
                    println!(
                        "ops_store({}, {});",
                        self.as_bytes_string(value),
                        self.as_bytes_string(pointer)
                    );
                }
                InstructionOpcode::Fence => {
                    println!("// fence;");
                }
                InstructionOpcode::AtomicCmpXchg => {
                    // cmpxchg [weak] [volatile] ptr <pointer>, <ty> <cmp>, <ty> <new> [syncscope("<target-scope>")] <success ordering> <failure ordering>[, align <alignment>] ; yields  { ty, i1 }

                    let ptr = instruction.get_operand(0).unwrap().unwrap_left();
                    let cmp = instruction.get_operand(1).unwrap().unwrap_left();
                    let new = instruction.get_operand(2).unwrap().unwrap_left();

                    println!(
                        "ops_cmpxchg({}, {}, {});",
                        self.value_name(ptr),
                        self.as_bytes_string(cmp),
                        self.as_bytes_string(new)
                    );
                }
                InstructionOpcode::AtomicRMW => {
                    let op_str = match instruction.get_atomic_rmw_bin_op().unwrap() {
                        inkwell::AtomicRMWBinOp::Xchg => "xchg",
                        inkwell::AtomicRMWBinOp::Add => "add",
                        inkwell::AtomicRMWBinOp::Sub => "sub",
                        inkwell::AtomicRMWBinOp::And => "and",
                        inkwell::AtomicRMWBinOp::Nand => "nand",
                        inkwell::AtomicRMWBinOp::Or => "or",
                        inkwell::AtomicRMWBinOp::Xor => "xor",
                        inkwell::AtomicRMWBinOp::Max => "max",
                        inkwell::AtomicRMWBinOp::Min => "min",
                        inkwell::AtomicRMWBinOp::UMax => "umax",
                        inkwell::AtomicRMWBinOp::UMin => "umin",
                        inkwell::AtomicRMWBinOp::FAdd => "fadd",
                        inkwell::AtomicRMWBinOp::FSub => "fsub",
                        inkwell::AtomicRMWBinOp::FMax => "fmax",
                        inkwell::AtomicRMWBinOp::FMin => "fmin",
                    };

                    let ptr = instruction.get_operand(0).unwrap().unwrap_left();
                    let value = instruction.get_operand(1).unwrap().unwrap_left();
                    println!(
                        "ops_atomic_rmw_{op_str}({}, {});",
                        self.value_name(ptr),
                        self.as_bytes_string(value)
                    );
                }
                InstructionOpcode::GetElementPtr => {
                    self.print_lhs_local(instruction);

                    let base_ptr = instruction.get_operand(0).unwrap().unwrap_left();
                    let base_ptr_name = self.value_name(base_ptr);

                    let mut current_type = instruction.get_gep_source_element_type().unwrap();

                    let mut offset_parts: Vec<String> = Vec::new();

                    let num_operands = instruction.get_num_operands();
                    for i in 1..num_operands {
                        let index_value = instruction.get_operand(i).unwrap().unwrap_left();

                        if i == 1 {
                            let element_size = self.target_data.get_store_size(&current_type);

                            match index_value {
                                BasicValueEnum::IntValue(int_val) => {
                                    if int_val.is_constant_int() {
                                        if let Some(idx) = int_val.get_zero_extended_constant() {
                                            let offset = idx * element_size;
                                            if offset > 0 {
                                                offset_parts.push(format!("{}", offset));
                                            }
                                        }
                                    } else {
                                        offset_parts.push(format!(
                                            "toInt({}) * {}",
                                            self.as_bytes_string(BasicValueEnum::IntValue(int_val)),
                                            element_size
                                        ));
                                    }
                                }
                                _ => {
                                    offset_parts.push(format!(
                                        "toInt({}) * {}",
                                        self.as_bytes_string(index_value),
                                        element_size
                                    ));
                                }
                            }
                            continue;
                        }

                        let element_size = match current_type {
                            BasicTypeEnum::ArrayType(arr_ty) => {
                                let size =
                                    self.target_data.get_store_size(&arr_ty.get_element_type());
                                current_type = arr_ty.get_element_type();
                                size
                            }
                            BasicTypeEnum::StructType(struct_ty) => {
                                if let BasicValueEnum::IntValue(int_val) = index_value
                                    && let Some(field_idx) = int_val.get_zero_extended_constant()
                                {
                                    let field_idx = field_idx as usize;
                                    let field_types = struct_ty.get_field_types();

                                    let field_offset: u64 = field_types
                                        .iter()
                                        .take(field_idx)
                                        .map(|field| self.target_data.get_store_size(field))
                                        .sum();

                                    if field_offset > 0 {
                                        offset_parts.push(format!("{}", field_offset));
                                    }

                                    if field_idx < field_types.len() {
                                        current_type = field_types[field_idx];
                                        continue;
                                    } else {
                                        panic!(
                                            "Struct field index {} out of bounds (struct has {} fields)",
                                            field_idx,
                                            field_types.len()
                                        );
                                    }
                                }
                                panic!("Struct GEP index must be constant");
                            }
                            BasicTypeEnum::PointerType(ptr_ty) => {
                                self.target_data.get_store_size(&ptr_ty)
                            }
                            _ => self.target_data.get_store_size(&current_type),
                        };

                        match index_value {
                            BasicValueEnum::IntValue(int_val) => {
                                if int_val.is_constant_int() {
                                    if let Some(idx) = int_val.get_zero_extended_constant() {
                                        let offset = idx * element_size;
                                        if offset > 0 {
                                            offset_parts.push(format!("{}", offset));
                                        }
                                    }
                                } else {
                                    offset_parts.push(format!(
                                        "toInt({}) * {}",
                                        self.as_bytes_string(BasicValueEnum::IntValue(int_val)),
                                        element_size
                                    ));
                                }
                            }
                            _ => {
                                offset_parts.push(format!(
                                    "toInt({}) * {}",
                                    self.as_bytes_string(index_value),
                                    element_size
                                ));
                            }
                        }
                    }

                    let offset_expr = if offset_parts.is_empty() {
                        "0".to_string()
                    } else {
                        offset_parts.join(" + ")
                    };

                    println!(" = ops_gep({}, {});", base_ptr_name, offset_expr);
                }

                // Bitwise Binary Operations
                InstructionOpcode::Shl
                | InstructionOpcode::LShr
                | InstructionOpcode::AShr
                | InstructionOpcode::And
                | InstructionOpcode::Or
                | InstructionOpcode::Xor => {
                    let fn_name = match instruction.get_opcode() {
                        InstructionOpcode::Shl => "shl",
                        InstructionOpcode::LShr => "lshr",
                        InstructionOpcode::AShr => "ashr",
                        InstructionOpcode::And => "and",
                        InstructionOpcode::Or => "or",
                        InstructionOpcode::Xor => "xor",
                        _ => unreachable!(),
                    };
                    self.print_lhs_local(instruction);
                    println!(
                        " = ops_{}({}, {});",
                        fn_name,
                        self.value_name(instruction.get_operand(0).unwrap().unwrap_left()),
                        self.value_name(instruction.get_operand(1).unwrap().unwrap_left())
                    );
                }

                // Binary Operations
                InstructionOpcode::Add
                | InstructionOpcode::FAdd
                | InstructionOpcode::Sub
                | InstructionOpcode::FSub
                | InstructionOpcode::Mul
                | InstructionOpcode::FMul
                | InstructionOpcode::UDiv
                | InstructionOpcode::SDiv
                | InstructionOpcode::FDiv
                | InstructionOpcode::URem
                | InstructionOpcode::SRem
                | InstructionOpcode::FRem => {
                    let fn_name = match instruction.get_opcode() {
                        InstructionOpcode::Add => "add",
                        InstructionOpcode::FAdd => "fadd",
                        InstructionOpcode::Sub => "sub",
                        InstructionOpcode::FSub => "fsub",
                        InstructionOpcode::Mul => "mul",
                        InstructionOpcode::FMul => "fmul",
                        InstructionOpcode::UDiv => "udiv",
                        InstructionOpcode::SDiv => "sdiv",
                        InstructionOpcode::FDiv => "fdiv",
                        InstructionOpcode::URem => "urem",
                        InstructionOpcode::SRem => "srem",
                        InstructionOpcode::FRem => "frem",
                        _ => unreachable!(),
                    };
                    self.print_lhs_local(instruction);
                    println!(
                        " = ops_{}({}, {});",
                        fn_name,
                        self.value_name(instruction.get_operand(0).unwrap().unwrap_left()),
                        self.value_name(instruction.get_operand(1).unwrap().unwrap_left())
                    );
                }
                InstructionOpcode::Call => {
                    if !instruction.get_type().is_void_type() {
                        self.print_lhs_local(instruction);
                        print!(" = ")
                    }
                    let num_operands = instruction.get_num_operands();
                    self.print_value(
                        instruction
                            .get_operand(num_operands - 1)
                            .unwrap()
                            .unwrap_left(),
                    );
                    print!("(");
                    for operand in instruction
                        .get_operands()
                        .take(num_operands as usize - 1)
                        .map(|x| x.unwrap().unwrap_left())
                    {
                        self.print_value(operand);
                        print!(", ");
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
                    self.print_value(
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
                        self.print_value(operand);
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
                    self.print_value(operand.unwrap_left());
                    println!(";");
                }
                InstructionOpcode::Resume => unreachable!("we use panic=abort"),
                InstructionOpcode::Br => {
                    if instruction.get_num_operands() == 1 {
                        println!(
                            "{}();",
                            normalize_name(
                                instruction
                                    .get_operand(0)
                                    .unwrap()
                                    .unwrap_right()
                                    .get_name()
                                    .to_str()
                                    .unwrap()
                            )
                        );
                    } else {
                        let condition = instruction.get_operand(0).unwrap().unwrap_left();
                        print!(
                            "if (truety({})) {{ {}(); }} else {{ {}(); }}",
                            self.as_bytes_string(condition),
                            normalize_name(
                                instruction
                                    .get_operand(1)
                                    .unwrap()
                                    .unwrap_right()
                                    .get_name()
                                    .to_str()
                                    .unwrap()
                            ),
                            normalize_name(
                                instruction
                                    .get_operand(2)
                                    .unwrap()
                                    .unwrap_right()
                                    .get_name()
                                    .to_str()
                                    .unwrap()
                            ),
                        );
                    }
                }
                InstructionOpcode::Switch => {
                    let mut iter = instruction.get_operands();
                    let value = iter.next().unwrap().unwrap().unwrap_left();
                    print!("switch (toInt({})) {{ ", self.as_bytes_string(value));
                    let otherwise = iter.next().unwrap().unwrap().unwrap_right();

                    while let Some(Some(value)) = iter.next() {
                        let value = value.unwrap_left();
                        let label = iter.next().unwrap().unwrap().unwrap_right();
                        println!(
                            "case {}: return {}();",
                            self.as_bytes_string(value),
                            normalize_name(label.get_name().to_str().unwrap())
                        );
                    }
                    println!(
                        "default: return {}(); }}",
                        normalize_name(otherwise.get_name().to_str().unwrap())
                    );
                }
            }
        }

        println!("    }}");
    }

    fn print_value(&mut self, value: impl Into<AnyValueEnum<'ctx>>) {
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
                    let str = int_value.print_to_string().to_string();
                    let str = str.split(" ").last().unwrap();
                    let width = int_value.get_type().get_bit_width();
                    if width == 128 {
                        format!("{str}n")
                    } else {
                        str.to_string()
                    }
                }
            }
            AnyValueEnum::FloatValue(float_value) => float_value.get_name().to_string(),
            AnyValueEnum::PhiValue(phi_value) => todo!(),
            AnyValueEnum::FunctionValue(function_value) => function_value.get_name().to_string(),
            AnyValueEnum::PointerValue(pointer_value) => {
                if pointer_value.is_null() {
                    "null".to_string()
                } else {
                    pointer_value.get_name().to_string()
                }
            }
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

    fn as_bytes_string(&mut self, value: BasicValueEnum<'ctx>) -> String {
        match value {
            BasicValueEnum::ArrayValue(array_value) => todo!(),
            BasicValueEnum::IntValue(int_value) => {
                let name = self.value_name(value);
                let width = int_value.get_type().get_bit_width();
                match width {
                    1 | 8 => format!("new Uint8Array([{name}])"),
                    16 => format!("new Uint16Array([{name}])"),
                    32 => format!("new Uint32Array([{name}])"),
                    64 => format!("new BigInt64Array([{name}])"),
                    128 => {
                        format!("new BigInt64Array([{name} >> 64n, {name} & 0xFFFFFFFFFFFFFFFFn])")
                    }
                    _ => todo!("size: {}", width),
                }
            }
            BasicValueEnum::FloatValue(float_value) => todo!(),
            BasicValueEnum::PointerValue(pointer_value) => {
                let name = self.value_name(value);
                format!("new Uint32Array([{name}])")
            }
            BasicValueEnum::StructValue(struct_value) => todo!(),
            BasicValueEnum::VectorValue(vector_value) => todo!(),
            BasicValueEnum::ScalableVectorValue(scalable_vector_value) => todo!(),
        }
    }

    fn print_as_bytes(&mut self, value: BasicValueEnum<'ctx>) {
        print!("{}", self.as_bytes_string(value));
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

#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_apfloat;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

mod body_local_decls;
#[cfg(test)]
mod tests;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::mir::visit::*;
use rustc_middle::mir::*;
use rustc_middle::ty::*;

struct JsTranspileCallback {
    pub output: String,
}

impl Callbacks for JsTranspileCallback {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        for id in tcx.hir_body_owners() {
            let def_id = id.to_def_id();

            let body = tcx.optimized_mir(def_id);

            let mut visitor = MyVisitor {
                tcx,
                def_id,
                output: String::new(),
                promoted: None,
            };
            visitor.visit_body(body);

            let promoteds = tcx.promoted_mir(def_id);
            for (i, promoted) in promoteds.iter().enumerate() {
                visitor.promoted = Some(i);
                visitor.visit_body(promoted);
            }

            self.output += &visitor.output;
        }

        self.output += "main();\n";

        Compilation::Stop
    }
}

struct MyVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    def_id: rustc_span::def_id::DefId,
    output: String,
    promoted: Option<usize>,
}

impl<'tcx> Visitor<'tcx> for MyVisitor<'tcx> {
    fn visit_body(&mut self, body: &Body<'tcx>) {
        match &self.promoted {
            Some(promoted) => {
                let fn_name = self.tcx.def_path_str(self.def_id);
                self.output += &format!("const {fn_name}__promoted_{promoted} = (() => {{\n");
            }
            None => {
                let fn_name = self
                    .tcx
                    .def_path_str(self.def_id)
                    .replace("::", "__")
                    .replace("<", "__")
                    .replace(">", "__")
                    .replace(" ", "__");
                self.output += &format!("function {fn_name}() {{\n");
            }
        }

        self.on_body_local_decls(body);

        self.super_body(body);
        self.output += "bb0();\nreturn _0;\n";

        if self.promoted.is_some() {
            self.output += "})();\n";
        } else {
            self.output += "}\n";
        }
    }

    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        println!("visit_basic_block_data: {block:?}, {data:?}");
        self.output += &format!("function bb{}() {{\n", block.as_u32());
        self.super_basic_block_data(block, data);
        self.output += "}\n";
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.on_statement(statement);
        self.super_statement(statement, location);
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        println!("visit_terminator: {terminator:?}");
        self.on_terminator(terminator);
        self.super_terminator(terminator, location);
    }
}

impl<'tcx> MyVisitor<'tcx> {
    fn on_statement(&mut self, statement: &Statement<'tcx>) {
        println!("visit_statement: {statement:?}");
        match &statement.kind {
            StatementKind::Assign(boxed) => {
                let (place, rvalue) = boxed.as_ref();

                self.on_place(place);
                self.output += ".assign(";
                self.on_rvalue(rvalue);
                self.output += ")";
            }
            StatementKind::FakeRead(_) => todo!(),
            StatementKind::SetDiscriminant {
                place: _,
                variant_index: _,
            } => todo!(),
            StatementKind::StorageLive(_local) => todo!(),
            StatementKind::StorageDead(_local) => todo!(),
            StatementKind::Retag(_retag_kind, _place) => todo!(),
            StatementKind::PlaceMention(_place) => todo!(),
            StatementKind::AscribeUserType(_, _variance) => todo!(),
            StatementKind::Coverage(_coverage_kind) => todo!(),
            StatementKind::Intrinsic(_non_diverging_intrinsic) => todo!(),
            StatementKind::ConstEvalCounter => todo!(),
            StatementKind::Nop => todo!(),
            StatementKind::BackwardIncompatibleDropHint {
                place: _,
                reason: _,
            } => todo!(),
        }

        self.output += ";\n";
    }

    fn on_rvalue(&mut self, rvalue: &Rvalue<'tcx>) {
        println!("visit_rvalue: {rvalue:?}");
        match rvalue {
            Rvalue::Use(operand) => self.on_operand(operand),
            Rvalue::Repeat(_operand, _) => todo!("Rvalue::Repeat"),
            Rvalue::Ref(_region, _borrow_kind, place) => {
                self.output += "_ref(";
                self.on_place(place);
                self.output += ")";
            }
            Rvalue::ThreadLocalRef(_def_id) => todo!("Rvalue::ThreadLocalRef"),
            Rvalue::RawPtr(_raw_ptr_kind, _place) => todo!("Rvalue::RawPtr"),
            Rvalue::Cast(_cast_kind, operand, _ty) => {
                self.on_operand(operand);
            }
            Rvalue::BinaryOp(bin_op, lr) => {
                let (left, right) = lr.as_ref();

                self.output += match bin_op {
                    BinOp::Add | BinOp::AddUnchecked | BinOp::AddWithOverflow => "_add",
                    BinOp::Sub | BinOp::SubUnchecked | BinOp::SubWithOverflow => "_sub",
                    BinOp::Mul | BinOp::MulUnchecked | BinOp::MulWithOverflow => "_mul",
                    BinOp::Div => "_div",
                    BinOp::Rem => "_rem",
                    BinOp::BitXor => "_xor",
                    BinOp::BitAnd => "_and",
                    BinOp::BitOr => "_or",
                    BinOp::Shl | BinOp::ShlUnchecked => "_shl",
                    BinOp::Shr | BinOp::ShrUnchecked => "_shr",
                    BinOp::Eq => "_eq",
                    BinOp::Lt => "_lt",
                    BinOp::Le => "_le",
                    BinOp::Ne => "_ne",
                    BinOp::Ge => "_ge",
                    BinOp::Gt => "_gt",
                    BinOp::Cmp => todo!(),
                    BinOp::Offset => todo!(),
                };
                self.output += "(";
                self.on_operand(left);
                self.output += ", ";
                self.on_operand(right);
                self.output += ")";
            }
            Rvalue::NullaryOp(null_op, ty) => match null_op {
                NullOp::SizeOf => {
                    let size = self.sizeof(ty);
                    self.output += &size.to_string();
                }
                NullOp::AlignOf => {
                    let size = self.sizeof(ty);
                    self.output += &size.to_string();
                }
                NullOp::OffsetOf(_raw_list) => todo!(),
                NullOp::UbChecks => todo!(),
                NullOp::ContractChecks => todo!(),
            },
            Rvalue::UnaryOp(un_op, operand) => {
                self.output += match un_op {
                    UnOp::Not => "_not(",
                    UnOp::Neg => "_neg(",
                    UnOp::PtrMetadata => todo!(),
                };
                self.on_operand(operand);
                self.output += ")";
            }
            Rvalue::Discriminant(place) => {
                self.output += "discriminant(";
                self.on_place(place);
                self.output += ")";
            }
            Rvalue::Aggregate(aggregate_kind, index_vec) => {
                println!(
                    "Aggregate -> aggregate_kind: {aggregate_kind:?}, index_vec: {index_vec:?}"
                );
                match aggregate_kind.as_ref() {
                    AggregateKind::Array(_ty) => {
                        self.output += "new Array([";
                        for (i, operand) in index_vec.iter().enumerate() {
                            self.on_operand(operand);
                            if i < index_vec.len() - 1 {
                                self.output += ", ";
                            }
                        }
                        self.output += "])";
                    }
                    AggregateKind::Tuple => {
                        self.output += "new Tuple([";
                        for (i, operand) in index_vec.iter().enumerate() {
                            self.on_operand(operand);
                            if i < index_vec.len() - 1 {
                                self.output += ", ";
                            }
                        }
                        self.output += "])";
                    }
                    AggregateKind::Adt(
                        def_id,
                        variant_idx,
                        raw_list,
                        user_type_annotation_index,
                        field_idx,
                    ) => {
                        let adt_def = self.tcx.adt_def(def_id);
                        println!("adt_def: {adt_def:?}");
                        println!("variant_idx: {variant_idx:?}");
                        println!("raw_list: {raw_list:?}");
                        println!("user_type_annotation_index: {user_type_annotation_index:?}");
                        println!("field_idx: {field_idx:?}");
                        println!("index_vec: {index_vec:?}");

                        let name = self.tcx.def_path_str(def_id);
                        if name == "std::option::Option" {
                            match variant_idx.as_usize() {
                                0 => self.output += "new Enum(undefined, 0)",
                                _ => {
                                    self.output += "new Enum(";
                                    self.on_operand(index_vec.iter().next().unwrap());
                                    self.output += ", 1)";
                                }
                            }
                        } else {
                            self.output += "[";
                            for (i, operand) in index_vec.iter().enumerate() {
                                self.on_operand(operand);
                                if i < index_vec.len() - 1 {
                                    self.output += ", ";
                                }
                            }
                            self.output += "]";
                        }
                    }
                    AggregateKind::Closure(_def_id, _raw_list) => todo!(),
                    AggregateKind::Coroutine(_def_id, _raw_list) => todo!(),
                    AggregateKind::CoroutineClosure(_def_id, _raw_list) => todo!(),
                    AggregateKind::RawPtr(_ty, _mutability) => todo!(),
                }
            }
            Rvalue::ShallowInitBox(operand, _ty) => {
                self.on_operand(operand);
            }
            Rvalue::CopyForDeref(_place) => todo!("Rvalue::CopyForDeref"),
            Rvalue::WrapUnsafeBinder(_operand, _ty) => todo!("Rvalue::WrapUnsafeBinder"),
        }
    }

    fn on_operand(&mut self, operand: &Operand<'tcx>) {
        println!("visit_operand: {operand:?}");
        match operand {
            Operand::Move(place) | Operand::Copy(place) => self.on_place(place),
            Operand::Constant(const_operand) => self.on_const_operarnd(const_operand),
        }
    }

    fn on_const_operarnd(&mut self, constant: &ConstOperand<'tcx>) {
        println!("constant: {:?}", constant.const_);
        match constant.const_ {
            rustc_middle::mir::Const::Ty(_ty, _const_) => panic!("rustc_middle::mir::Const::Ty"),
            rustc_middle::mir::Const::Unevaluated(unevaluated_const, ty) => {
                println!("unevaluated_const: {unevaluated_const:?}\n{ty:?}");
                match unevaluated_const.promoted {
                    Some(promoted) => {
                        self.output += &format!("main__promoted_{}", promoted.as_u32())
                    }
                    None => todo!(),
                }
            }
            rustc_middle::mir::Const::Val(const_value, ty) => match const_value {
                ConstValue::Scalar(scalar) => {
                    println!("scalar: {scalar:?}");
                    self.output += &match ty.kind() {
                        Bool => todo!(),
                        Char => todo!(),
                        Int(int_ty) => match int_ty {
                            IntTy::I8 => format!(
                                "new Int8({})",
                                scalar.to_int(rustc_abi::Size::from_bits(8)).unwrap()
                            ),
                            IntTy::I16 => format!(
                                "new Int16({})",
                                scalar.to_int(rustc_abi::Size::from_bits(16)).unwrap()
                            ),
                            IntTy::I32 => format!(
                                "new Int32({})",
                                scalar.to_int(rustc_abi::Size::from_bits(32)).unwrap()
                            ),
                            IntTy::I64 => format!(
                                "new Int64({})",
                                scalar.to_int(rustc_abi::Size::from_bits(64)).unwrap()
                            ),
                            IntTy::I128 => todo!(),
                            IntTy::Isize => todo!(),
                        },
                        Uint(uint_ty) => match uint_ty {
                            UintTy::U8 => format!(
                                "new Uint8({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(8)).unwrap()
                            ),
                            UintTy::U16 => format!(
                                "new Uint16({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(16)).unwrap()
                            ),
                            UintTy::U32 => format!(
                                "new Uint32({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(32)).unwrap()
                            ),
                            UintTy::U64 => format!(
                                "new Uint64({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(64)).unwrap()
                            ),
                            UintTy::U128 => todo!(),
                            UintTy::Usize => {
                                if cfg!(target_pointer_width = "64") {
                                    format!(
                                        "new Uint64({})",
                                        scalar.to_uint(rustc_abi::Size::from_bits(64)).unwrap()
                                    )
                                } else {
                                    format!(
                                        "new Uint32({})",
                                        scalar.to_uint(rustc_abi::Size::from_bits(32)).unwrap()
                                    )
                                }
                            }
                        },
                        Float(float_ty) => todo!(),
                        Adt(_, _) => todo!(),
                        Foreign(_) => todo!(),
                        Str => todo!(),
                        Array(_, _) => todo!(),
                        Pat(_, _) => todo!(),
                        Slice(_) => todo!(),
                        RawPtr(_, mutability) => todo!(),
                        Ref(_, _, mutability) => todo!(),
                        FnDef(_, _) => todo!(),
                        FnPtr(binder, fn_header) => todo!(),
                        UnsafeBinder(unsafe_binder_inner) => todo!(),
                        Dynamic(_, _) => todo!(),
                        Closure(_, _) => todo!(),
                        CoroutineClosure(_, _) => todo!(),
                        Coroutine(_, _) => todo!(),
                        CoroutineWitness(_, _) => todo!(),
                        Never => todo!(),
                        Tuple(_) => todo!(),
                        Alias(alias_ty_kind, alias_ty) => todo!(),
                        Param(_) => todo!(),
                        Bound(bound_var_index_kind, _) => todo!(),
                        Infer(infer_ty) => todo!(),
                        Error(_) => todo!(),
                        _ => todo!(),
                    };
                }
                ConstValue::ZeroSized => match constant.ty().kind() {
                    Bool => todo!("Bool"),
                    Char => todo!("Char"),
                    Int(_int_ty) => todo!("Int"),
                    Uint(_uint_ty) => todo!("Uint"),
                    Float(_float_ty) => todo!("Float"),
                    Adt(_, _) => todo!("Adt"),
                    Foreign(_) => todo!("Foreign"),
                    Str => todo!("Str"),
                    Array(_, _) => todo!("Array"),
                    Pat(_, _) => todo!("Pat"),
                    Slice(_) => todo!("Slice"),
                    RawPtr(_, _mutability) => todo!("RawPtr"),
                    Ref(_, _, _mutability) => todo!("Ref"),
                    FnDef(function_id, generic_args) => {
                        let fn_name = self.tcx.def_path_str(function_id);
                        let generic_args_string = generic_args
                            .iter()
                            .map(|arg| arg.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        println!("fn_name: {fn_name}");
                        self.output += &match fn_name.as_str() {
                            "core::fmt::rt::Argument::<'_>::new_display" => {
                                "core__fmt__rt__Argument__new_display".to_string()
                            }
                            "core::fmt::rt::<impl std::fmt::Arguments<'a>>::new_v1" => {
                                "core__fmt__rt__Arguments__new_v1".to_string()
                            }
                            "core::fmt::rt::<impl std::fmt::Arguments<'a>>::new_const" => {
                                "core__fmt__rt__Arguments__new_const".to_string()
                            }
                            "std::io::_print" => "std__io__print".to_string(),
                            "std::convert::From::from" => {
                                let from = generic_args[0];
                                let to = generic_args[1];

                                if let Some(from) = from.as_type()
                                    && from.to_string() == "std::string::String"
                                    && let Some(to) = to.as_type()
                                    && to.is_ref()
                                    && to.peel_refs().is_str()
                                {
                                    "".to_string()
                                } else {
                                    todo!()
                                }
                            }
                            "std::iter::IntoIterator::into_iter" => {
                                format!(
                                    "std__iter__IntoIterator__into_iter__{}",
                                    generic_args_string
                                )
                            }
                            "std::iter::Iterator::next" => "std__iter__Iterator__next".to_string(),
                            "alloc::alloc::exchange_malloc" => {
                                "alloc__alloc__exchange_malloc".to_string()
                            }
                            "std::slice::<impl [T]>::into_vec" => {
                                "std__slice__impl__T__into_vec".to_string()
                            }
                            _ => todo!("FnDef, {fn_name}"),
                        };
                    }
                    FnPtr(_binder, _fn_header) => todo!("FnPtr"),
                    UnsafeBinder(_unsafe_binder_inner) => todo!("UnsafeBinder"),
                    Dynamic(_, _) => todo!("Dynamic"),
                    Closure(_, _) => todo!("Closure"),
                    CoroutineClosure(_, _) => todo!("CoroutineClosure"),
                    Coroutine(_, _) => todo!("Coroutine"),
                    CoroutineWitness(_, _) => todo!("CoroutineWitness"),
                    Never => todo!("Never"),
                    Tuple(_) => todo!("Tuple"),
                    Alias(_alias_ty_kind, _alias_ty) => todo!("Alias"),
                    Param(_) => todo!("Param"),
                    Bound(_bound_var_index_kind, _) => todo!("Bound"),
                    Infer(_infer_ty) => todo!("Infer"),
                    Error(_) => todo!("Error"),
                    _ => todo!(),
                },
                ConstValue::Slice { alloc_id, meta: _ } => {
                    let alloc = self.tcx.try_get_global_alloc(alloc_id).unwrap();
                    match alloc {
                        interpret::GlobalAlloc::Function { instance: _ } => {
                            todo!("GlobalAlloc::Function")
                        }
                        interpret::GlobalAlloc::VTable(_ty, _raw_list) => {
                            todo!("GlobalAlloc::VTable")
                        }
                        interpret::GlobalAlloc::Static(_def_id) => todo!("GlobalAlloc::Static"),
                        interpret::GlobalAlloc::Memory(memory) => {
                            let inner = memory.inner();
                            let bytes = inner.get_bytes_unchecked((0..inner.len()).into());

                            self.output += "new Uint8Array([";
                            for (i, byte) in bytes.iter().enumerate() {
                                self.output += &format!("{byte}");
                                if i < bytes.len() - 1 {
                                    self.output += ", ";
                                }
                            }
                            self.output += "])";
                        }
                        interpret::GlobalAlloc::TypeId { ty: _ } => todo!("GlobalAlloc::TypeId"),
                    }
                }
                ConstValue::Indirect {
                    alloc_id: _,
                    offset: _,
                } => todo!(),
            },
        };
    }

    fn on_terminator(&mut self, terminator: &Terminator<'tcx>) {
        match &terminator.kind {
            TerminatorKind::Goto { target } => {
                self.output += &format!("return bb{}();\n", target.as_u32());
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                self.output += "switch (switchInt(";
                self.on_operand(discr);
                self.output += ")) {\n";
                println!("targets: {targets:?}");
                for (i, value) in targets.iter() {
                    self.output += &format!("case {i}:");
                    self.output += &format!("return bb{}();\n", value.as_u32());
                }
                self.output += &format!("default: return bb{}();\n", targets.otherwise().as_u32());
                self.output += "}\n";
            }
            TerminatorKind::UnwindResume => {
                self.output += "// UnwindResume\n";
            }
            TerminatorKind::UnwindTerminate(_unwind_terminate_reason) => todo!(),
            TerminatorKind::Return => {
                self.output += "return;\n";
            }
            TerminatorKind::Unreachable => {
                self.output += "throw new Error('unreachable');\n";
            }
            TerminatorKind::Drop {
                place: _,
                target,
                unwind: _,
                replace: _,
                drop: _,
                async_fut: _,
            } => {
                self.output += &format!("return bb{}();\n", target.as_u32());
            }
            TerminatorKind::Call {
                func,
                args,
                destination,
                target,
                unwind: _,
                call_source: _,
                fn_span: _,
            } => {
                self.on_place(destination);
                self.output += "=";
                self.on_operand(func);
                self.output += "(";
                for (i, arg) in args.iter().enumerate() {
                    self.on_operand(&arg.node);
                    if i < args.len() - 1 {
                        self.output += ", ";
                    }
                }
                self.output += ");\n";

                if let Some(target) = target {
                    self.output += &format!("return bb{}();\n", target.as_u32());
                }
            }
            TerminatorKind::TailCall {
                func: _,
                args: _,
                fn_span: _,
            } => todo!(),
            TerminatorKind::Assert {
                cond,
                expected,
                msg,
                target,
                unwind: _,
            } => {
                self.output += "if (_eq(";
                self.on_operand(cond);
                self.output += &format!(", {})) {{\n", expected);
                self.output += &format!("return bb{}();\n", target.as_u32());
                self.output += "} else {\n";
                self.output += "throw new Error('assert failed: ";
                self.output += &format!("{:?}", msg).escape_debug().to_string();
                self.output += "');\n";
                self.output += "}\n";
            }
            TerminatorKind::Yield {
                value: _,
                resume: _,
                resume_arg: _,
                drop: _,
            } => todo!(),
            TerminatorKind::CoroutineDrop => todo!(),
            TerminatorKind::FalseEdge {
                real_target: _,
                imaginary_target: _,
            } => todo!(),
            TerminatorKind::FalseUnwind {
                real_target: _,
                unwind: _,
            } => todo!(),
            TerminatorKind::InlineAsm {
                asm_macro: _,
                template: _,
                operands: _,
                options: _,
                line_spans: _,
                targets: _,
                unwind: _,
            } => todo!(),
        }
    }

    fn on_place(&mut self, place: &Place<'tcx>) {
        println!("visit_place: {place:?}");
        self.output += &format!("_{}", place.local.as_u32());
        for projection_element in place.projection {
            match projection_element.kind() {
                ProjectionElem::Deref => {
                    self.output += ".deref()";
                }
                ProjectionElem::Field(field_idx, _) => {
                    self.output += &format!(".field({})", field_idx.as_u32());
                }
                ProjectionElem::Index(_) => todo!(),
                ProjectionElem::ConstantIndex {
                    offset: _,
                    min_length: _,
                    from_end: _,
                } => todo!(),
                ProjectionElem::Subslice {
                    from: _,
                    to: _,
                    from_end: _,
                } => todo!(),
                ProjectionElem::Downcast(_symbol, _variant_idx) => {}
                ProjectionElem::OpaqueCast(_) => todo!(),
                ProjectionElem::UnwrapUnsafeBinder(_) => todo!(),
            }
        }
    }

    fn sizeof(&self, ty: &Ty<'tcx>) -> usize {
        let param_env = self.tcx.param_env(self.def_id);

        let param_env_and_ty = param_env.and(*ty);
        let typing_env = rustc_middle::ty::TypingEnv {
            param_env: param_env_and_ty.param_env,
            // Assuming layout computation happens after the main analysis
            typing_mode: rustc_middle::ty::TypingMode::PostAnalysis,
        };
        let query_input = typing_env.as_query_input(param_env_and_ty.value);
        let layout = self.tcx.layout_of(query_input).unwrap();

        layout.size.bytes() as usize
    }
}

pub fn run(path: &str) -> String {
    let mut callback = JsTranspileCallback {
        output: String::new(),
    };
    let args: Vec<String> = vec!["ignored".to_string(), path.to_string()];
    rustc_driver::run_compiler(&args, &mut callback);
    callback.output
}

#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_apfloat;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_type_ir;

mod body_local_decls;
mod fn_def;
mod name_convert;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::mir::visit::*;
use rustc_middle::mir::*;
use rustc_middle::ty::*;
use rustc_type_ir::EarlyBinder;

struct JsTranspileCallback {
    pub tx: Sender<String>,
}

impl Callbacks for JsTranspileCallback {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let mut fn_names = Default::default();
        let mut todo_instances: HashSet<Instance<'tcx>> = Default::default();

        let main_fn_id = tcx
            .hir_body_owners()
            .find(|id| {
                let def_id = id.to_def_id();
                tcx.def_path_str(def_id) == "main"
            })
            .expect("main not found")
            .to_def_id();

        let instance = rustc_middle::ty::Instance::try_resolve(
            tcx,
            rustc_middle::ty::TypingEnv {
                param_env: tcx.param_env(main_fn_id),
                typing_mode: rustc_middle::ty::TypingMode::PostAnalysis,
            },
            main_fn_id,
            GenericArgs::identity_for_item(tcx, main_fn_id),
        )
        .unwrap()
        .unwrap();
        self.run(tcx, instance, &mut fn_names, &mut todo_instances);

        while !todo_instances.is_empty() {
            let instance = *todo_instances.iter().next().unwrap();
            todo_instances.remove(&instance);
            self.run(tcx, instance, &mut fn_names, &mut todo_instances);
        }

        self.tx.send("main();\n".to_string()).unwrap();

        Compilation::Stop
    }
}

impl JsTranspileCallback {
    fn run<'tcx>(
        &self,
        tcx: TyCtxt<'tcx>,
        instance: Instance<'tcx>,
        fn_names: &mut HashMap<
            (
                rustc_hir::def_id::DefId,
                &'tcx rustc_middle::ty::List<GenericArg<'tcx>>,
            ),
            String,
        >,
        todo_instances: &mut HashSet<Instance<'tcx>>,
    ) {
        let body = tcx.instance_mir(instance.def);

        let body = tcx.instantiate_and_normalize_erasing_regions(
            instance.args,
            rustc_middle::ty::TypingEnv {
                param_env: tcx.param_env(instance.def_id()),
                typing_mode: rustc_middle::ty::TypingMode::PostAnalysis,
            },
            EarlyBinder::bind(body.clone()),
        );

        let mut visitor = MyVisitor {
            tcx,
            instance,
            tx: self.tx.clone(),
            promoted: None,
            fn_names,
            todo_instances,
        };
        visitor.visit_body(&body);

        let promoteds = tcx.promoted_mir(instance.def_id());
        for (i, promoted) in promoteds.iter().enumerate() {
            visitor.promoted = Some(i);
            visitor.visit_body(promoted);
        }
    }
}

struct MyVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    instance: Instance<'tcx>,
    tx: Sender<String>,
    promoted: Option<usize>,
    fn_names: &'a mut HashMap<
        (
            rustc_hir::def_id::DefId,
            &'tcx rustc_middle::ty::List<GenericArg<'tcx>>,
        ),
        String,
    >,
    todo_instances: &'a mut HashSet<Instance<'tcx>>,
}

impl<'tcx> Visitor<'tcx> for MyVisitor<'_, 'tcx> {
    fn visit_body(&mut self, body: &Body<'tcx>) {
        let fn_name = self.def_normalized_name(self.instance.def_id(), self.instance.args);
        println!("visit_body: {fn_name}");
        match &self.promoted {
            Some(promoted) => {
                self.out(format!(
                    "const {fn_name}__promoted_{promoted} = (() => {{\n"
                ));
            }
            None => {
                self.out(format!("function {fn_name}() {{\n"));
            }
        }

        self.on_body_local_decls(body);

        self.super_body(body);
        self.out("bb0();\nreturn _0;\n");

        if self.promoted.is_some() {
            self.out("})();\n");
        } else {
            self.out("}\n");
        }
    }

    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        println!("visit_basic_block_data: {block:?}, {data:?}");
        self.out(format!("function bb{}() {{\n", block.as_u32()));
        self.super_basic_block_data(block, data);
        self.out("}\n");
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

impl<'tcx> MyVisitor<'_, 'tcx> {
    fn out(&self, str: impl ToString) {
        self.tx.send(str.to_string()).unwrap();
    }
    fn try_resolve(
        &self,
        def_id: rustc_hir::def_id::DefId,
        generic_args: &'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>,
    ) -> Option<rustc_middle::ty::Instance<'tcx>> {
        let Ok(Some(instance)) = rustc_middle::ty::Instance::try_resolve(
            self.tcx,
            rustc_middle::ty::TypingEnv {
                param_env: self.tcx.param_env(self.instance.def_id()),
                typing_mode: rustc_middle::ty::TypingMode::PostAnalysis,
            },
            def_id,
            generic_args,
        ) else {
            println!("try_resolve failed: {def_id:?}, {generic_args:?}");
            return None;
        };
        Some(instance)
    }
    fn on_statement(&mut self, statement: &Statement<'tcx>) {
        println!("visit_statement: {statement:?}");
        match &statement.kind {
            StatementKind::Assign(boxed) => {
                let (place, rvalue) = boxed.as_ref();

                self.on_place(place);
                self.out(".assign(");
                self.on_rvalue(rvalue);
                self.out(")");
            }
            StatementKind::FakeRead(_) => todo!(),
            StatementKind::SetDiscriminant {
                place: _,
                variant_index: _,
            } => todo!(),
            StatementKind::Retag(_retag_kind, _place) => todo!(),
            StatementKind::PlaceMention(_place) => todo!(),
            StatementKind::AscribeUserType(_, _variance) => todo!(),
            StatementKind::Coverage(_coverage_kind) => todo!(),
            StatementKind::Intrinsic(non_diverging_intrinsic) => {
                match non_diverging_intrinsic.as_ref() {
                    NonDivergingIntrinsic::Assume(operand) => {
                        self.on_operand(operand);
                    }
                    NonDivergingIntrinsic::CopyNonOverlapping(_copy_non_overlapping) => todo!(),
                }
            }
            StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_)
            | StatementKind::ConstEvalCounter
            | StatementKind::Nop => {
                // ignored
                return;
            }
            StatementKind::BackwardIncompatibleDropHint {
                place: _,
                reason: _,
            } => todo!(),
        }

        self.out(";\n");
    }

    fn on_rvalue(&mut self, rvalue: &Rvalue<'tcx>) {
        println!("visit_rvalue: {rvalue:?}");
        match rvalue {
            Rvalue::Use(operand) => self.on_operand(operand),
            Rvalue::Repeat(_operand, _) => todo!("Rvalue::Repeat"),
            Rvalue::Ref(_region, _borrow_kind, place) => {
                self.out("_ref(");
                self.on_place(place);
                self.out(")");
            }
            Rvalue::ThreadLocalRef(_def_id) => todo!("Rvalue::ThreadLocalRef"),
            Rvalue::RawPtr(_raw_ptr_kind, _place) => {
                self.out("_raw_ptr(");
                self.on_place(_place);
                self.out(")");
            }
            Rvalue::Cast(_cast_kind, operand, _ty) => {
                self.on_operand(operand);
            }
            Rvalue::BinaryOp(bin_op, lr) => {
                let (left, right) = lr.as_ref();

                self.out(match bin_op {
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
                    BinOp::Offset => "_offset",
                });
                self.out("(");
                self.on_operand(left);
                self.out(", ");
                self.on_operand(right);
                self.out(")");
            }
            Rvalue::NullaryOp(null_op, ty) => match null_op {
                NullOp::SizeOf | NullOp::AlignOf => {
                    if let Some(size) = self.sizeof(ty) {
                        self.out(size);
                    } else {
                        self.out(format!("sizeof({ty})"));
                    }
                }
                NullOp::OffsetOf(_raw_list) => todo!(),
                NullOp::UbChecks => self.out("_ub_checks()"),
                NullOp::ContractChecks => todo!(),
            },
            Rvalue::UnaryOp(un_op, operand) => {
                self.out(match un_op {
                    UnOp::Not => "_not(",
                    UnOp::Neg => "_neg(",
                    UnOp::PtrMetadata => "_ptr_metadata(",
                });
                self.on_operand(operand);
                self.out(")");
            }
            Rvalue::Discriminant(place) => {
                self.out("discriminant(");
                self.on_place(place);
                self.out(")");
            }
            Rvalue::Aggregate(aggregate_kind, index_vec) => {
                println!(
                    "Aggregate -> aggregate_kind: {aggregate_kind:?}, index_vec: {index_vec:?}"
                );
                match aggregate_kind.as_ref() {
                    AggregateKind::Array(_ty) => {
                        self.out("new Array([");
                        for (i, operand) in index_vec.iter().enumerate() {
                            self.on_operand(operand);
                            if i < index_vec.len() - 1 {
                                self.out(", ");
                            }
                        }
                        self.out("])");
                    }
                    AggregateKind::Tuple => {
                        self.out("new Tuple([");
                        for (i, operand) in index_vec.iter().enumerate() {
                            self.on_operand(operand);
                            if i < index_vec.len() - 1 {
                                self.out(", ");
                            }
                        }
                        self.out("])");
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
                                0 => self.out("new Enum(undefined, 0)"),
                                _ => {
                                    self.out("new Enum(");
                                    self.on_operand(index_vec.iter().next().unwrap());
                                    self.out(", 1)");
                                }
                            }
                        } else {
                            self.out("[");
                            for (i, operand) in index_vec.iter().enumerate() {
                                self.on_operand(operand);
                                if i < index_vec.len() - 1 {
                                    self.out(", ");
                                }
                            }
                            self.out("]");
                        }
                    }
                    AggregateKind::Closure(_def_id, _raw_list) => todo!(),
                    AggregateKind::Coroutine(_def_id, _raw_list) => todo!(),
                    AggregateKind::CoroutineClosure(_def_id, _raw_list) => todo!(),
                    AggregateKind::RawPtr(_ty, _mutability) => {
                        self.out("_raw_ptr(");
                        assert_eq!(index_vec.len(), 2);
                        self.on_operand(index_vec.iter().next().unwrap());
                        self.on_operand(index_vec.iter().next().unwrap());
                        self.out(")");
                    }
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
            rustc_middle::mir::Const::Ty(_ty, const_) => match const_.kind() {
                rustc_type_ir::ConstKind::Param(param) => self.out(param.name),
                rustc_type_ir::ConstKind::Infer(_infer_const) => todo!("Infer"),
                rustc_type_ir::ConstKind::Bound(_bound_var_index_kind, _) => todo!("Bound"),
                rustc_type_ir::ConstKind::Placeholder(_) => todo!("Placeholder"),
                rustc_type_ir::ConstKind::Unevaluated(_unevaluated_const) => todo!("Unevaluated"),
                rustc_type_ir::ConstKind::Value(value) => self.out(value),
                rustc_type_ir::ConstKind::Error(_) => todo!("Error"),
                rustc_type_ir::ConstKind::Expr(_) => todo!("Expr"),
            },
            rustc_middle::mir::Const::Unevaluated(unevaluated_const, _ty) => {
                match unevaluated_const.promoted {
                    Some(promoted) => self.out(format!("main__promoted_{}", promoted.as_u32())),
                    None => {
                        println!("panic");
                        let instance = self
                            .try_resolve(unevaluated_const.def, unevaluated_const.args)
                            .unwrap_or_else(|| {
                                todo!("Instance not found {:?}", unevaluated_const);
                            });

                        self.check_fn_defined(unevaluated_const.def, unevaluated_const.args);
                        self.fn_name(unevaluated_const.def, unevaluated_const.args);
                    }
                }
            }
            rustc_middle::mir::Const::Val(const_value, ty) => match const_value {
                ConstValue::Scalar(scalar) => {
                    println!("scalar: {scalar:?}");
                    self.out(match ty.kind() {
                        Bool => format!("new Bool({})", scalar.to_bool().unwrap()),
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
                            IntTy::Isize => format!(
                                "new Int32({})",
                                scalar.to_int(rustc_abi::Size::from_bits(32)).unwrap()
                            ),
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
                                format!(
                                    "new Uint32({})",
                                    scalar.to_uint(rustc_abi::Size::from_bits(32)).unwrap()
                                )
                            }
                        },
                        Float(_float_ty) => todo!(),
                        Adt(adt_def, generic_args) => {
                            println!("Adt: {:?}", adt_def);
                            println!("generic_args: {:?}", generic_args);
                            format!(
                                "_adt(\"{}\", {})",
                                self.def_normalized_name(adt_def.did(), generic_args),
                                scalar
                            )
                        }
                        Foreign(_) => todo!(),
                        Str => todo!(),
                        Array(_, _) => todo!(),
                        Pat(_, _) => todo!(),
                        Slice(_) => todo!(),
                        RawPtr(_, _mutability) => todo!(),
                        Ref(_, _, _mutability) => todo!(),
                        FnDef(_, _) => todo!(),
                        FnPtr(_binder, _fn_header) => todo!(),
                        UnsafeBinder(_unsafe_binder_inner) => todo!(),
                        Dynamic(_, _) => todo!(),
                        Closure(_, _) => todo!(),
                        CoroutineClosure(_, _) => todo!(),
                        Coroutine(_, _) => todo!(),
                        CoroutineWitness(_, _) => todo!(),
                        Never => todo!(),
                        Tuple(_) => todo!(),
                        Alias(_alias_ty_kind, _alias_ty) => todo!(),
                        Param(_) => todo!(),
                        Bound(_bound_var_index_kind, _) => todo!(),
                        Infer(_infer_ty) => todo!(),
                        Error(_) => todo!(),
                        _ => todo!(),
                    })
                }
                ConstValue::ZeroSized => match constant.ty().kind() {
                    Bool => todo!("Bool"),
                    Char => todo!("Char"),
                    Int(_int_ty) => todo!("Int"),
                    Uint(_uint_ty) => todo!("Uint"),
                    Float(_float_ty) => todo!("Float"),
                    Adt(adt_def, generic_args) => {
                        let name = self.def_normalized_name(adt_def.did(), generic_args);
                        println!("name: {name}");
                        self.out(name);
                    }
                    Foreign(_) => todo!("Foreign"),
                    Str => todo!("Str"),
                    Array(_, _) => todo!("Array"),
                    Pat(_, _) => todo!("Pat"),
                    Slice(_) => todo!("Slice"),
                    RawPtr(_, _mutability) => todo!("RawPtr"),
                    Ref(_, _, _mutability) => todo!("Ref"),
                    FnDef(function_id, generic_args) => {
                        println!("Zero Sized, generic_args: {generic_args:?}");
                        self.check_fn_defined(*function_id, generic_args);
                        let a = self.fn_name(*function_id, generic_args).clone();
                        self.out(a);
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

                            self.out("new Uint8Array([");
                            for (i, byte) in bytes.iter().enumerate() {
                                self.out(format!("{byte}"));
                                if i < bytes.len() - 1 {
                                    self.out(", ");
                                }
                            }
                            self.out("])");
                        }
                        interpret::GlobalAlloc::TypeId { ty: _ } => todo!("GlobalAlloc::TypeId"),
                    }
                }
                ConstValue::Indirect { alloc_id, offset } => {
                    self.out(format!("_indirect({}, {})", alloc_id.0, offset.bytes()));
                }
            },
        };
    }

    fn on_terminator(&mut self, terminator: &Terminator<'tcx>) {
        match &terminator.kind {
            TerminatorKind::Goto { target } => {
                self.out(format!("return bb{}();\n", target.as_u32()));
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                self.out("switch (switchInt(");
                self.on_operand(discr);
                self.out(")) {\n");
                println!("targets: {targets:?}");
                for (i, value) in targets.iter() {
                    self.out(format!("case {i}:"));
                    self.out(format!("return bb{}();\n", value.as_u32()));
                }
                self.out(format!(
                    "default: return bb{}();\n",
                    targets.otherwise().as_u32()
                ));
                self.out("}\n");
            }
            TerminatorKind::UnwindResume => {
                self.out("// UnwindResume\n");
            }
            TerminatorKind::UnwindTerminate(_unwind_terminate_reason) => todo!(),
            TerminatorKind::Return => {
                self.out("return;\n");
            }
            TerminatorKind::Unreachable => {
                self.out("throw new Error('unreachable');\n");
            }
            TerminatorKind::Drop {
                place: _,
                target,
                unwind: _,
                replace: _,
                drop: _,
                async_fut: _,
            } => {
                self.out(format!("return bb{}();\n", target.as_u32()));
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
                self.out("=");
                self.on_operand(func);
                self.out("(");
                for (i, arg) in args.iter().enumerate() {
                    self.on_operand(&arg.node);
                    if i < args.len() - 1 {
                        self.out(", ");
                    }
                }
                self.out(");\n");

                if let Some(target) = target {
                    self.out(format!("return bb{}();\n", target.as_u32()));
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
                self.out("if (_eq(");
                self.on_operand(cond);
                self.out(format!(", {})) {{\n", expected));
                self.out(format!("return bb{}();\n", target.as_u32()));
                self.out("} else {\n");
                self.out("throw new Error('assert failed: ");
                self.out(format!("{:?}", msg).escape_debug().to_string());
                self.out("');\n");
                self.out("}\n");
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
        self.out(format!("_{}", place.local.as_u32()));
        for projection_element in place.projection {
            match projection_element {
                ProjectionElem::Deref => {
                    self.out(".deref()");
                }
                ProjectionElem::Field(field_idx, _) => {
                    self.out(format!(".field({})", field_idx.as_u32()));
                }
                ProjectionElem::Index(index) => {
                    self.out(format!(".index(_{})", index.as_u32()));
                }
                ProjectionElem::ConstantIndex {
                    offset: _,
                    min_length: _,
                    from_end: _,
                } => todo!("ConstantIndex"),
                ProjectionElem::Subslice {
                    from: _,
                    to: _,
                    from_end: _,
                } => todo!("Subslice"),
                ProjectionElem::Downcast(symbol, variant_idx) => {
                    self.out(format!(
                        ".downcast({}, {})",
                        symbol.unwrap(),
                        variant_idx.as_u32()
                    ));
                }
                ProjectionElem::OpaqueCast(_) => todo!("OpaqueCast"),
                ProjectionElem::UnwrapUnsafeBinder(_) => todo!("UnwrapUnsafeBinder"),
            }
        }
    }

    fn sizeof(&self, ty: &Ty<'tcx>) -> Option<usize> {
        let param_env = self.tcx.param_env(self.instance.def_id());
        let typing_env = rustc_middle::ty::TypingEnv {
            param_env,
            typing_mode: rustc_middle::ty::TypingMode::PostAnalysis,
        };
        let query_input = typing_env.as_query_input(*ty);
        let Ok(layout) = self.tcx.layout_of(query_input) else {
            panic!("layout_of failed: {query_input:?}");
        };

        Some(layout.size.bytes() as usize)
    }
}

fn build_custom_sysroot() -> std::path::PathBuf {
    use rustc_build_sysroot::{BuildMode, SysrootBuilder, SysrootConfig};

    let sysroot_dir = std::env::current_dir().unwrap().join("sysroot");
    SysrootBuilder::new(&sysroot_dir, "wasm32-wasip1-threads")
        .rustflags(["-Zalways-encode-mir"])
        .build_mode(BuildMode::Check)
        .sysroot_config(SysrootConfig::WithStd {
            std_features: vec![],
        })
        .build_from_source(std::path::Path::new("./std"))
        .expect("Failed to build sysroot");
여기 실패해. 잡아봐.
    sysroot_dir
}

pub fn run(path: &str) -> Receiver<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let path = path.to_string();
    std::thread::spawn(move || {
        let sysroot = build_custom_sysroot();

        let target = "wasm32-wasip1-threads";

        let lib_path = sysroot.join("lib").join("rustlib").join(target).join("lib");
        let mut callback = JsTranspileCallback { tx };
        let args: Vec<String> = vec![
            "ignored".to_string(),
            path,
            "--target=wasm32-wasip1-threads".to_string(),
            "--sysroot".to_string(),
            sysroot.to_str().unwrap().to_string(),
        ];
        rustc_driver::run_compiler(&args, &mut callback);
    });
    rx
}

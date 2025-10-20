#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

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
        println!("body: {body:#?}");

        match &self.promoted {
            Some(promoted) => {
                let fn_name = self.tcx.def_path_str(self.def_id);
                self.output += &format!("const {fn_name}__promoted_{promoted} = (() => {{\n");
            }
            None => {
                let fn_name = self.tcx.def_path_str(self.def_id);
                self.output += &format!("function {fn_name}() {{\n");
            }
        }

        for i in 0..body.local_decls.len() {
            self.output += &format!("let _{};\n", i);
        }
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
        println!("visit_statement: {statement:?}");
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
    fn on_statement(&mut self, statemenet: &Statement<'tcx>) {
        match &statemenet.kind {
            StatementKind::Assign(boxed) => {
                let (place, rvalue) = boxed.as_ref();

                self.output += &format!("_{} = ", place.local.as_u32());
                self.on_rvalue(rvalue);
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
            StatementKind::BackwardIncompatibleDropHint { place: _, reason: _ } => todo!(),
        }

        self.output += ";\n";
    }

    fn on_rvalue(&mut self, rvalue: &Rvalue<'tcx>) {
        println!("visit_rvalue: {rvalue:?}");
        match rvalue {
            Rvalue::Use(operand) => self.on_operand(operand),
            Rvalue::Repeat(_operand, _) => todo!("Rvalue::Repeat"),
            Rvalue::Ref(_region, _borrow_kind, place) => {
                self.output += &format!("_{}", place.local.as_u32());
            }
            Rvalue::ThreadLocalRef(_def_id) => todo!("Rvalue::ThreadLocalRef"),
            Rvalue::RawPtr(_raw_ptr_kind, _place) => todo!("Rvalue::RawPtr"),
            Rvalue::Cast(_cast_kind, _operand, _ty) => todo!("Rvalue::Cast"),
            Rvalue::BinaryOp(_bin_op, _) => todo!("Rvalue::BinaryOp"),
            Rvalue::NullaryOp(_null_op, _ty) => todo!("Rvalue::NullaryOp"),
            Rvalue::UnaryOp(_un_op, _operand) => todo!("Rvalue::UnaryOp"),
            Rvalue::Discriminant(_place) => todo!("Rvalue::Discriminant"),
            Rvalue::Aggregate(aggregate_kind, index_vec) => match aggregate_kind.as_ref() {
                AggregateKind::Array(_ty) => {
                    self.output.push('[');
                    for operand in index_vec {
                        self.on_operand(operand);
                        self.output.push(',');
                    }
                    self.output.push(']');
                }
                AggregateKind::Tuple => {
                    self.output.push('[');
                    for operand in index_vec {
                        self.on_operand(operand);
                        self.output.push(',');
                    }
                    self.output.push(']');
                }
                AggregateKind::Adt(
                    _def_id,
                    _variant_idx,
                    _raw_list,
                    _user_type_annotation_index,
                    _field_idx,
                ) => todo!(),
                AggregateKind::Closure(_def_id, _raw_list) => todo!(),
                AggregateKind::Coroutine(_def_id, _raw_list) => todo!(),
                AggregateKind::CoroutineClosure(_def_id, _raw_list) => todo!(),
                AggregateKind::RawPtr(_ty, _mutability) => todo!(),
            },
            Rvalue::ShallowInitBox(_operand, _ty) => todo!("Rvalue::ShallowInitBox"),
            Rvalue::CopyForDeref(_place) => todo!("Rvalue::CopyForDeref"),
            Rvalue::WrapUnsafeBinder(_operand, _ty) => todo!("Rvalue::WrapUnsafeBinder"),
        }
    }

    fn on_operand(&mut self, operand: &Operand<'tcx>) {
        match operand {
            Operand::Move(place) | Operand::Copy(place) => {
                self.output += &format!("_{}", place.local.as_u32());
                for projection_element in place.projection {
                    match projection_element.kind() {
                        ProjectionElem::Deref => todo!(),
                        ProjectionElem::Field(field_idx, _) => {
                            self.output += &format!("[{}]", field_idx.as_u32());
                        }
                        ProjectionElem::Index(_) => todo!(),
                        ProjectionElem::ConstantIndex {
                            offset: _,
                            min_length: _,
                            from_end: _,
                        } => todo!(),
                        ProjectionElem::Subslice { from: _, to: _, from_end: _ } => todo!(),
                        ProjectionElem::Downcast(_symbol, _variant_idx) => todo!(),
                        ProjectionElem::OpaqueCast(_) => todo!(),
                        ProjectionElem::UnwrapUnsafeBinder(_) => todo!(),
                    }
                }
            }
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
                    self.output += &if ty.is_bool() {
                        scalar.to_bool().unwrap().to_string()
                    } else {
                        format!("{scalar}")
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
                    FnDef(function_id, _generic_args) => {
                        let fn_name = self.tcx.def_path_str(function_id);
                        self.output += match fn_name.as_str() {
                            "core::fmt::rt::Argument::<'_>::new_display" => {
                                "core__fmt__rt__Argument__new_display"
                            }
                            "core::fmt::rt::<impl std::fmt::Arguments<'a>>::new_v1" => {
                                "core__fmt__rt__Arguments__new_v1"
                            }
                            "core::fmt::rt::<impl std::fmt::Arguments<'a>>::new_const" => {
                                "core__fmt__rt__Arguments__new_const"
                            }
                            "std::io::_print" => "std__io__print",
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
                            let string =
                                String::from_utf8_lossy(bytes).escape_default().to_string();
                            self.output += &format!("\"{string}\"");
                        }
                        interpret::GlobalAlloc::TypeId { ty: _ } => todo!("GlobalAlloc::TypeId"),
                    }
                }
                ConstValue::Indirect { alloc_id: _, offset: _ } => todo!(),
            },
        };
    }

    fn on_terminator(&mut self, terminator: &Terminator<'tcx>) {
        match &terminator.kind {
            TerminatorKind::Goto { target: _ } => todo!(),
            TerminatorKind::SwitchInt { discr: _, targets: _ } => todo!(),
            TerminatorKind::UnwindResume => todo!(),
            TerminatorKind::UnwindTerminate(_unwind_terminate_reason) => todo!(),
            TerminatorKind::Return => {
                self.output += "return;\n";
            }
            TerminatorKind::Unreachable => todo!(),
            TerminatorKind::Drop {
                place: _,
                target: _,
                unwind: _,
                replace: _,
                drop: _,
                async_fut: _,
            } => todo!(),
            TerminatorKind::Call {
                func,
                args,
                destination,
                target,
                unwind: _,
                call_source: _,
                fn_span: _,
            } => {
                self.output += &format!("_{} = ", destination.local.as_u32());
                self.on_operand(func);
                self.output += "(";
                for arg in args {
                    self.on_operand(&arg.node);
                    self.output += ", ";
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
                cond: _,
                expected: _,
                msg: _,
                target: _,
                unwind: _,
            } => todo!(),
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
}

pub fn run(path: &str) -> String {
    let mut callback = JsTranspileCallback {
        output: String::new(),
    };
    let args: Vec<String> = vec!["ignored".to_string(), path.to_string()];
    rustc_driver::run_compiler(&args, &mut callback);
    callback.output
}

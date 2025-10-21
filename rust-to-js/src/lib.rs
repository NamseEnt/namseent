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

                println!("projection: {:?}", place.projection);

                let mut projection_iter = place.projection.iter();

                self.output += &format!("_{}", place.local.as_u32());

                match projection_iter.len() {
                    0 => {
                        self.output += " = ";
                        self.on_rvalue(rvalue);
                    }
                    1 => {
                        let projection = projection_iter.next().unwrap();
                        match projection {
                            ProjectionElem::Deref => {
                                self.output += ".derefAssign(";
                                self.on_rvalue(rvalue);
                                self.output += ")";
                            }
                            _ => todo!(),
                        }
                    }
                    2 => {
                        let p1 = projection_iter.next().unwrap();
                        let p2 = projection_iter.next().unwrap();
                        match (p1, p2) {
                            (ProjectionElem::Deref, ProjectionElem::Field(field_idx, _)) => {
                                self.output += &format!(".deref()[{}] = ", field_idx.as_u32());
                                self.on_rvalue(rvalue);
                            }
                            _ => todo!(),
                        }
                    }
                    _ => todo!(),
                }
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
            Rvalue::Ref(_region, _borrow_kind, place) => self.on_place(place),
            Rvalue::ThreadLocalRef(_def_id) => todo!("Rvalue::ThreadLocalRef"),
            Rvalue::RawPtr(_raw_ptr_kind, _place) => todo!("Rvalue::RawPtr"),
            Rvalue::Cast(_cast_kind, operand, _ty) => {
                self.on_operand(operand);
            }
            Rvalue::BinaryOp(bin_op, lr) => {
                let (left, right) = lr.as_ref();
                self.on_operand(left);

                self.output += match bin_op {
                    BinOp::Add => "+",
                    BinOp::AddUnchecked => todo!(),
                    BinOp::AddWithOverflow => "+",
                    BinOp::Sub => "-",
                    BinOp::SubUnchecked => todo!(),
                    BinOp::SubWithOverflow => todo!(),
                    BinOp::Mul => "*",
                    BinOp::MulUnchecked => todo!(),
                    BinOp::MulWithOverflow => todo!(),
                    BinOp::Div => "/",
                    BinOp::Rem => "%",
                    BinOp::BitXor => "^",
                    BinOp::BitAnd => "&",
                    BinOp::BitOr => "|",
                    BinOp::Shl => "<<",
                    BinOp::ShlUnchecked => todo!(),
                    BinOp::Shr => ">>",
                    BinOp::ShrUnchecked => todo!(),
                    BinOp::Eq => "==",
                    BinOp::Lt => "<",
                    BinOp::Le => "<=",
                    BinOp::Ne => "!=",
                    BinOp::Ge => ">=",
                    BinOp::Gt => ">",
                    BinOp::Cmp => todo!(),
                    BinOp::Offset => todo!(),
                };
                self.on_operand(right);
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
                match un_op {
                    UnOp::Not => self.output += "!",
                    UnOp::Neg => self.output += "-",
                    UnOp::PtrMetadata => todo!(),
                }
                self.on_operand(operand);
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
                        self.output += "[";
                        for (i, operand) in index_vec.iter().enumerate() {
                            self.on_operand(operand);
                            if i < index_vec.len() - 1 {
                                self.output += ", ";
                            }
                        }
                        self.output += "]";
                    }
                    AggregateKind::Tuple => {
                        self.output += "[";
                        for (i, operand) in index_vec.iter().enumerate() {
                            self.on_operand(operand);
                            if i < index_vec.len() - 1 {
                                self.output += ", ";
                            }
                        }
                        self.output += "]";
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
                    self.output += &if ty.is_bool() {
                        scalar.to_bool().unwrap().to_string()
                    } else if ty.is_floating_point() {
                        if scalar.size().bytes() == 8 {
                            scalar
                                .to_float::<rustc_apfloat::ieee::Double>()
                                .unwrap()
                                .to_string()
                        } else {
                            scalar
                                .to_float::<rustc_apfloat::ieee::Single>()
                                .unwrap()
                                .to_string()
                        }
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
                    FnDef(function_id, generic_args) => {
                        let fn_name = self.tcx.def_path_str(function_id);
                        println!("fn_name: {fn_name}");
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
                            "std::convert::From::from" => {
                                let from = generic_args[0];
                                let to = generic_args[1];

                                if let Some(from) = from.as_type()
                                    && from.to_string() == "std::string::String"
                                    && let Some(to) = to.as_type()
                                    && to.is_ref()
                                    && to.peel_refs().is_str()
                                {
                                    ""
                                } else {
                                    todo!()
                                }
                            }
                            "std::iter::IntoIterator::into_iter" => {
                                "std__iter__IntoIterator__into_iter"
                            }
                            "std::iter::Iterator::next" => "std__iter__Iterator__next",
                            "alloc::alloc::exchange_malloc" => "alloc__alloc__exchange_malloc",
                            "std::slice::<impl [T]>::into_vec" => "std__slice__impl__T__into_vec",
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
                self.output += &format!("_{} = ", destination.local.as_u32());
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
                self.output += "if (";
                self.on_operand(cond);
                self.output += &format!("=== {}) {{\n", expected);
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
                ProjectionElem::Deref => {}
                ProjectionElem::Field(field_idx, _) => {
                    self.output += &format!("[{}]", field_idx.as_u32());
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
        match ty.kind() {
            Bool => size_of::<bool>(),
            Char => size_of::<char>(),
            Int(int_ty) => int_ty.bit_width().unwrap() as usize / 8,
            Uint(uint_ty) => uint_ty.bit_width().unwrap() as usize / 8,
            Float(float_ty) => float_ty.bit_width() as usize / 8,
            Adt(_, _) => todo!(),
            Foreign(_) => todo!(),
            Str => todo!(),
            Array(ty, n) => {
                let size = self.sizeof(ty);
                size * n.to_value().try_to_target_usize(self.tcx).unwrap() as usize
            }
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

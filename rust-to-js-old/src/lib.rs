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
mod on_const_operarnd;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::OnceLock;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Instant;

use crate::name_convert::*;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::mir::visit::*;
use rustc_middle::mir::*;
use rustc_middle::ty::*;
use rustc_type_ir::EarlyBinder;

struct JsTranspileCallback {
    pub tx: Sender<String>,
}

static START_TIME: OnceLock<Instant> = OnceLock::new();

impl Callbacks for JsTranspileCallback {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        let mut fn_names = Default::default();
        let mut todo_instances: HashSet<Instance<'tcx>> = Default::default();
        let mut handled_instances: HashSet<Instance<'tcx>> = Default::default();

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
        self.run(
            tcx,
            instance,
            &mut fn_names,
            &mut todo_instances,
            &mut handled_instances,
        );

        while !todo_instances.is_empty() {
            let instance = *todo_instances.iter().next().unwrap();
            todo_instances.remove(&instance);
            handled_instances.insert(instance);

            if !tcx.is_mir_available(instance.def_id()) {
                println!("instance is not available: {instance:?}");
                continue;
            }
            if is_extern(tcx, instance.def_id()) {
                println!("instance is extern: {instance:?}");
                continue;
            }
            self.run(
                tcx,
                instance,
                &mut fn_names,
                &mut todo_instances,
                &mut handled_instances,
            );
        }

        self.tx.send("main();\n".to_string()).unwrap();

        Compilation::Stop
    }
}

fn is_extern(tcx: TyCtxt<'_>, def_id: rustc_hir::def_id::DefId) -> bool {
    let parent_id = tcx.parent(def_id);
    let parent_kind = tcx.def_kind(parent_id);
    matches!(parent_kind, rustc_hir::def::DefKind::ForeignMod)
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
        handled_instances: &mut HashSet<Instance<'tcx>>,
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

        let mut alloc_ids = HashSet::new();

        let mut visitor = MyVisitor {
            tcx,
            instance,
            tx: self.tx.clone(),
            promoted: None,
            fn_name: def_normalized_name(tcx, &instance.def_id(), instance.args),
            fn_names,
            todo_instances,
            handled_instances,
            typing_env: rustc_middle::ty::TypingEnv {
                param_env: tcx.param_env(instance.def_id()),
                typing_mode: rustc_middle::ty::TypingMode::PostAnalysis,
            },
            body: &body,
            indent_size: 0,
            is_new_line: true,
            alloc_ids: &mut alloc_ids,
        };
        visitor.run();

        let promoteds = tcx.promoted_mir(instance.def_id());
        for (i, promoted) in promoteds.iter().enumerate() {
            visitor.promoted = Some(i);
            visitor.body = promoted;
            visitor.run();
        }
    }
}

struct MyVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    instance: Instance<'tcx>,
    tx: Sender<String>,
    promoted: Option<usize>,
    fn_name: String,
    fn_names: &'a mut HashMap<
        (
            rustc_hir::def_id::DefId,
            &'tcx rustc_middle::ty::List<GenericArg<'tcx>>,
        ),
        String,
    >,
    todo_instances: &'a mut HashSet<Instance<'tcx>>,
    handled_instances: &'a mut HashSet<Instance<'tcx>>,
    typing_env: rustc_middle::ty::TypingEnv<'tcx>,
    body: &'a Body<'tcx>,
    indent_size: i32,
    is_new_line: bool,
    alloc_ids: &'a mut HashSet<interpret::AllocId>,
}

impl<'tcx> Visitor<'tcx> for MyVisitor<'_, 'tcx> {
    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        println!("visit_basic_block_data: {block:?}, {data:?}");
        self.outln(format!("function bb{}() {{", block.as_u32()));
        self.indent(4);
        self.super_basic_block_data(block, data);
        self.indent(-4);
        self.outln("}");
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
    fn run(&mut self) {
        let fn_name = &self.fn_name;
        println!("visit_body: {fn_name}");
        match &self.promoted {
            Some(promoted) => {
                self.outln(format!("const {fn_name}__promoted_{promoted} = (() => {{"));
            }
            None => {
                let args = (0..self.body.arg_count)
                    .map(|i| format!("arg{i}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                self.outln(format!("function {fn_name}({args}) {{"));
            }
        }

        self.indent(4);

        self.outln("stackPush();");

        self.on_body_local_decls(self.body);

        self.super_body(self.body);
        self.outln("bb0();");
        self.outln("stackPop();");
        self.outln("return _0;");

        self.indent(-4);

        if self.promoted.is_some() {
            self.outln("})();");
        } else {
            self.outln("}");
        }
    }
    fn out(&mut self, str: impl ToString) {
        self.tx
            .send(format!(
                "{}{}",
                " ".repeat(if self.is_new_line {
                    self.indent_size
                } else {
                    0
                } as usize),
                str.to_string()
            ))
            .unwrap();

        self.is_new_line = false;
    }
    fn outln(&mut self, str: impl ToString) {
        self.tx
            .send(format!(
                "{}{}\n",
                " ".repeat(if self.is_new_line {
                    self.indent_size
                } else {
                    0
                } as usize),
                str.to_string()
            ))
            .unwrap();

        self.is_new_line = true;
    }
    fn indent(&mut self, size: i32) {
        self.indent_size += size;
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

                self.out("assign(");
                self.on_place(place);
                self.out(", ");
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

        self.outln(";");
    }

    fn on_rvalue(&mut self, rvalue: &Rvalue<'tcx>) {
        println!("visit_rvalue: {rvalue:?}");
        match rvalue {
            Rvalue::Use(operand) => self.on_operand(operand),
            Rvalue::Repeat(operand, const_) => {
                self.out("_repeat(");
                self.on_operand(operand);
                self.out(", ");
                self.out(const_);
                self.out(")");
            }
            Rvalue::Ref(_region, _borrow_kind, place) => {
                self.on_place(place);
                self.out(".ptr");
            }
            Rvalue::ThreadLocalRef(_def_id) => todo!("Rvalue::ThreadLocalRef"),
            Rvalue::RawPtr(_raw_ptr_kind, _place) => {
                self.on_place(_place);
                self.out(".ptr");
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
                // NullOp::SizeOf | NullOp::AlignOf => {
                //     if let Some(size) = self.sizeof(ty) {
                //         self.out(size);
                //     } else {
                //         self.out(format!("sizeof({ty})"));
                //     }
                // }
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
            Rvalue::Aggregate(aggregate_kind, index_vec) => match aggregate_kind.as_ref() {
                AggregateKind::Array(_ty) => {
                    self.out("stackAllocArray([");
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
                AggregateKind::Closure(id, args) => {
                    let fn_name = self.on_function(id, args);
                    self.out(fn_name);
                }
                AggregateKind::Coroutine(_def_id, _raw_list) => todo!(),
                AggregateKind::CoroutineClosure(_def_id, _raw_list) => todo!(),
                AggregateKind::RawPtr(_ty, _mutability) => {
                    self.out("_raw_ptr(");
                    assert_eq!(index_vec.len(), 2);
                    self.on_operand(index_vec.iter().next().unwrap());
                    self.on_operand(index_vec.iter().next().unwrap());
                    self.out(")");
                }
            },
            Rvalue::ShallowInitBox(operand, _ty) => {
                println!("Rvalue::ShallowInitBox, operand: {operand:?}");
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

    fn on_terminator(&mut self, terminator: &Terminator<'tcx>) {
        match &terminator.kind {
            TerminatorKind::Goto { target } => {
                self.outln(format!("bb{}();", target.as_u32()));
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                self.out("switch (switchInt(");
                self.on_operand(discr);
                self.outln(")) {");
                println!("targets: {targets:?}");
                for (i, value) in targets.iter() {
                    self.outln(format!("case {i}: bb{}();", value.as_u32()));
                }
                self.outln(format!("default: bb{}();", targets.otherwise().as_u32()));
                self.outln("}");
            }
            TerminatorKind::UnwindResume => {
                self.outln("// UnwindResume");
            }
            TerminatorKind::UnwindTerminate(_unwind_terminate_reason) => todo!(),
            TerminatorKind::Return => {
                self.outln("// Return");
            }
            TerminatorKind::Unreachable => {
                self.outln("throw new Error('unreachable');");
            }
            TerminatorKind::Drop {
                place,
                target,
                unwind,
                replace,
                drop,
                async_fut,
            } => {
                println!("place: {:?}", place);
                println!("target: {:?}", target);
                println!("unwind: {:?}", unwind);
                println!("replace: {:?}", replace);
                println!("drop: {:?}", drop);
                println!("async_fut: {:?}", async_fut);

                let a = self.body.local_decls[place.local].ty.ty_adt_def().unwrap();

                let drop_trait_def_id = self.tcx.lang_items().drop_trait().unwrap();
                self.tcx.for_each_relevant_impl(
                    drop_trait_def_id,
                    self.body.local_decls[place.local].ty,
                    |id| {
                        println!("id: {id:?}");
                    },
                );

                self.outln("// Drop");
                self.outln(format!("bb{}();", target.as_u32()));
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
                self.out("assign(");
                self.on_place(destination);
                self.out(", ");
                self.on_operand(func);
                self.out("(");
                for (i, arg) in args.iter().enumerate() {
                    self.on_operand(&arg.node);
                    if i < args.len() - 1 {
                        self.out(", ");
                    }
                }
                self.outln("));");

                if let Some(target) = target {
                    self.outln(format!("bb{}();", target.as_u32()));
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
                self.outln(format!(", {})) {{", expected));
                self.outln(format!("return bb{}();", target.as_u32()));
                self.outln("} else {");
                self.out("throw new Error('assert failed: ");
                self.out(format!("{:?}", msg).escape_debug().to_string());
                self.outln("');");
                self.outln("}");
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
        let ty = self.tcx.instantiate_and_normalize_erasing_regions(
            self.instance.args,
            self.typing_env,
            EarlyBinder::bind(*ty),
        );

        let query_input = self.typing_env.as_query_input(ty);
        let Ok(layout) = self.tcx.layout_of(query_input) else {
            panic!("layout_of failed: ty: {ty:?}, query_input: {query_input:?}");
        };

        Some(layout.size.bytes() as usize)
    }
}

pub fn run(path: &str) -> Receiver<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let path = path.to_string();
    std::thread::spawn(move || {
        let sysroot = "./custom-sysroot";

        let target = "wasm32-wasip1-threads";

        let mut callback = JsTranspileCallback { tx };
        let args: Vec<String> = vec![
            "ignored".to_string(),
            path,
            format!("--target={target}",),
            // "--sysroot".to_string(),
            // sysroot.to_string(),
            "-Zunstable-options".to_string(),
            "-Cpanic=immediate-abort".to_string(),
        ];
        START_TIME.get_or_init(Instant::now);
        rustc_driver::run_compiler(&args, &mut callback);
    });
    rx
}

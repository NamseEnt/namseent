use crate::*;

impl<'tcx> MyVisitor<'tcx> {
    pub fn on_body_local_decls(&mut self, body: &Body<'tcx>) {
        for (i, decl) in body.local_decls.iter().enumerate() {
            self.output += &format!("let _{} = ", i);
            println!("ty: {:?}", decl.ty);
            match decl.ty.kind() {
                Bool => {
                    self.output += "false";
                }
                Char => todo!(),
                Int(_int_ty) => {
                    self.output += "0";
                }
                Uint(_uint_ty) => {
                    self.output += "0";
                }
                Float(_float_ty) => {
                    self.output += "0";
                }
                Adt(adt_def, generic_args) => {
                    println!("Adt({:?}, {:?})", adt_def, generic_args);
                    self.output += "new UninitAdt()";
                }
                Foreign(_) => todo!(),
                Str => todo!(),
                Array(_, _) => {
                    self.output += "[]";
                }
                Pat(_, _) => todo!(),
                Slice(_) => todo!(),
                RawPtr(_ty, _mutability) => {
                    self.output += "new UninitPtr()";
                }
                Ref(_, _, _mutability) => {
                    self.output += "new UninitPtr()";
                }
                FnDef(_, _) => todo!(),
                FnPtr(_binder, _fn_header) => todo!(),
                UnsafeBinder(_unsafe_binder_inner) => todo!(),
                Dynamic(_, _) => todo!(),
                Closure(_, _) => todo!(),
                CoroutineClosure(_, _) => todo!(),
                Coroutine(_, _) => todo!(),
                CoroutineWitness(_, _) => todo!(),
                Never => todo!(),
                Tuple(_) => {
                    self.output += "[]";
                }
                Alias(_alias_ty_kind, _alias_ty) => todo!(),
                Param(_) => todo!(),
                Bound(_bound_var_index_kind, _) => todo!(),
                Infer(_infer_ty) => todo!(),
                Error(_) => todo!(),
                _ => todo!(),
            }
            self.output += ";\n";
        }
    }
}

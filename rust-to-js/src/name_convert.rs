use crate::*;

impl<'tcx> MyVisitor<'_, 'tcx> {
    pub fn def_normalized_name(
        &self,
        def_id: rustc_span::def_id::DefId,
        generic_args: &'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>,
    ) -> String {
        (self.tcx.def_path_str(def_id) + &self.generic_args_names(generic_args))
            .replace("::", "__")
            .replace("<", "__")
            .replace(">", "__")
            .replace(" ", "__")
            .replace("[", "__")
            .replace("]", "__")
            .replace(";", "__")
    }

    pub fn generic_args_names(
        &self,
        generic_args: &'tcx rustc_middle::ty::List<GenericArg<'tcx>>,
    ) -> String {
        generic_args
            .iter()
            .filter_map(|arg| match arg.kind() {
                rustc_type_ir::GenericArgKind::Lifetime(_) => None,
                rustc_type_ir::GenericArgKind::Type(ty) => Some(self.ty_name(&ty)),
                rustc_type_ir::GenericArgKind::Const(const_) => Some(self.const_name(const_)),
            })
            .collect::<Vec<_>>()
            .join("__")
    }

    pub fn const_name(&self, const_: rustc_middle::ty::Const<'tcx>) -> String {
        match const_.kind() {
            rustc_type_ir::ConstKind::Param(_) => todo!("Param"),
            rustc_type_ir::ConstKind::Infer(_infer_const) => todo!("Infer"),
            rustc_type_ir::ConstKind::Bound(_bound_var_index_kind, _) => todo!("Bound"),
            rustc_type_ir::ConstKind::Placeholder(_) => todo!("Placeholder"),
            rustc_type_ir::ConstKind::Unevaluated(_unevaluated_const) => todo!("Unevaluated"),
            rustc_type_ir::ConstKind::Value(value) => value.to_string(),
            rustc_type_ir::ConstKind::Error(_) => todo!("Error"),
            rustc_type_ir::ConstKind::Expr(_) => todo!("Expr"),
        }
    }

    pub fn ty_name(&self, ty: &Ty<'tcx>) -> String {
        match ty.kind() {
            Bool => "_ty_bool".to_string(),
            Char => "_ty_char".to_string(),
            Int(int_ty) => match int_ty {
                IntTy::Isize => "_ty_isize".to_string(),
                IntTy::I8 => "_ty_i8".to_string(),
                IntTy::I16 => "_ty_i16".to_string(),
                IntTy::I32 => "_ty_i32".to_string(),
                IntTy::I64 => "_ty_i64".to_string(),
                IntTy::I128 => "_ty_i128".to_string(),
            },
            Uint(uint_ty) => match uint_ty {
                UintTy::Usize => "_ty_usize".to_string(),
                UintTy::U8 => "_ty_u8".to_string(),
                UintTy::U16 => "_ty_u16".to_string(),
                UintTy::U32 => "_ty_u32".to_string(),
                UintTy::U64 => "_ty_u64".to_string(),
                UintTy::U128 => "_ty_u128".to_string(),
            },
            Float(float_ty) => match float_ty.bit_width() {
                32 => "_ty_f32".to_string(),
                64 => "_ty_f64".to_string(),
                _ => unreachable!(),
            },
            Adt(adt_def, generic_args) => self.def_normalized_name(adt_def.did(), generic_args),
            Foreign(_) => todo!(),
            Str => todo!(),
            Array(ty, const_) => {
                format!("_ty_array_{}_{}", ty, const_)
            }
            Pat(_, _) => todo!(),
            Slice(_) => todo!(),
            RawPtr(_, _mutability) => todo!(),
            Ref(_region, ty, _mutability) => {
                format!("_ref_{}", self.ty_name(ty))
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
            Tuple(tuple) => {
                let mut s = "_ty_tuple".to_string();
                for ty in tuple.iter() {
                    s.push_str(&format!("_{}", self.ty_name(&ty)));
                }
                s
            }
            Alias(_alias_ty_kind, _alias_ty) => todo!(),
            Param(_) => todo!(),
            Bound(_bound_var_index_kind, _) => todo!(),
            rustc_type_ir::TyKind::Placeholder(_) => todo!(),
            Infer(_infer_ty) => todo!(),
            Error(_) => todo!(),
        }
    }
}

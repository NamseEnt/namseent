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
                rustc_type_ir::GenericArgKind::Type(ty) => Some(self.ty_name(ty)),
                rustc_type_ir::GenericArgKind::Const(const_) => todo!(),
            })
            .collect::<Vec<_>>()
            .join("__")
    }

    pub fn ty_name(&self, ty: Ty<'tcx>) -> String {
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
            rustc_type_ir::TyKind::Placeholder(_) => todo!(),
            Infer(infer_ty) => todo!(),
            Error(_) => todo!(),
        }
    }
}

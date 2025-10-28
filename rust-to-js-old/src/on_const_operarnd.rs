use crate::*;

impl<'tcx> MyVisitor<'_, 'tcx> {
    pub fn on_const_operarnd(&mut self, constant: &ConstOperand<'tcx>) {
        println!("constant: {:?}", constant.const_);
        match constant.const_ {
            rustc_middle::mir::Const::Ty(_ty, const_) => match const_.kind() {
                rustc_type_ir::ConstKind::Param(param) => self.out(param.name),
                rustc_type_ir::ConstKind::Infer(_infer_const) => todo!("Infer"),
                rustc_type_ir::ConstKind::Bound(_bound_var_index_kind, _) => todo!("Bound"),
                rustc_type_ir::ConstKind::Placeholder(_) => todo!("Placeholder"),
                rustc_type_ir::ConstKind::Unevaluated(_unevaluated_const) => todo!("Unevaluated"),
                rustc_type_ir::ConstKind::Value(value) => match value.ty.kind() {
                    Bool => self.out(value.try_to_bool().unwrap()),
                    Char => todo!("Char"),
                    Int(int_ty) => {
                        let bits = value.try_to_bits(self.tcx, self.typing_env).unwrap();
                        let bytes = bits.to_ne_bytes();
                        self.out(match int_ty {
                            IntTy::I8 => {
                                i8::from_ne_bytes(bytes[0..1].try_into().unwrap()).to_string()
                            }
                            IntTy::I16 => {
                                i16::from_ne_bytes(bytes[0..2].try_into().unwrap()).to_string()
                            }
                            IntTy::I32 => {
                                i32::from_ne_bytes(bytes[0..4].try_into().unwrap()).to_string()
                            }
                            IntTy::I64 => {
                                i64::from_ne_bytes(bytes[0..8].try_into().unwrap()).to_string()
                            }
                            IntTy::I128 => {
                                i128::from_ne_bytes(bytes[0..16].try_into().unwrap()).to_string()
                            }
                            IntTy::Isize => {
                                i32::from_ne_bytes(bytes[0..4].try_into().unwrap()).to_string()
                            }
                        })
                    }
                    Uint(uint_ty) => {
                        let bits = value.try_to_bits(self.tcx, self.typing_env).unwrap();
                        self.out(match uint_ty {
                            UintTy::U8 => (bits & 0xFF).to_string(),
                            UintTy::U16 => (bits & 0xFFFF).to_string(),
                            UintTy::U32 => (bits & 0xFFFFFFFF).to_string(),
                            UintTy::U64 => (bits & 0xFFFFFFFFFFFFFFFF).to_string(),
                            UintTy::U128 => bits.to_string(),
                            UintTy::Usize => (bits & 0xFFFFFFFF).to_string(),
                        })
                    }
                    Float(float_ty) => todo!("Float"),
                    Adt(_, _) => todo!("Adt"),
                    Foreign(_) => todo!("Foreign"),
                    Str => todo!("Str"),
                    Array(_, _) => todo!("Array"),
                    Pat(_, _) => todo!("Pat"),
                    Slice(_) => todo!("Slice"),
                    RawPtr(_, mutability) => todo!("RawPtr"),
                    Ref(_, _, mutability) => todo!("Ref"),
                    FnDef(_, _) => todo!("FnDef"),
                    FnPtr(binder, fn_header) => todo!("FnPtr"),
                    UnsafeBinder(unsafe_binder_inner) => todo!("UnsafeBinder"),
                    Dynamic(_, _) => todo!("Dynamic"),
                    Closure(_, _) => todo!("Closure"),
                    CoroutineClosure(_, _) => todo!("CoroutineClosure"),
                    Coroutine(_, _) => todo!("Coroutine"),
                    CoroutineWitness(_, _) => todo!("CoroutineWitness"),
                    Never => todo!("Never"),
                    Tuple(_) => todo!("Tuple"),
                    Alias(alias_ty_kind, alias_ty) => todo!("Alias"),
                    Param(_) => todo!("Param"),
                    Bound(bound_var_index_kind, _) => todo!("Bound"),
                    rustc_type_ir::TyKind::Placeholder(_) => {
                        todo!("rustc_type_ir::TyKind::Placeholder")
                    }
                    Infer(infer_ty) => todo!("Infer"),
                    Error(_) => todo!("Error"),
                },
                rustc_type_ir::ConstKind::Error(_) => todo!("Error"),
                rustc_type_ir::ConstKind::Expr(_) => todo!("Expr"),
            },
            rustc_middle::mir::Const::Unevaluated(unevaluated_const, _ty) => {
                match unevaluated_const.promoted {
                    Some(promoted) => {
                        self.out(format!("{}__promoted_{}", self.fn_name, promoted.as_u32()))
                    }
                    None => {
                        let fn_name =
                            self.on_function(&unevaluated_const.def, unevaluated_const.args);
                        self.out(fn_name);
                    }
                }
            }
            rustc_middle::mir::Const::Val(const_value, ty) => match const_value {
                ConstValue::Scalar(scalar) => {
                    println!("scalar: {scalar:?}");
                    self.out(match ty.kind() {
                        Bool => format!("new Bool({})", scalar.to_bool().unwrap()),
                        Char => format!("new Char('{}')", scalar.to_char().unwrap()),
                        Int(int_ty) => match int_ty {
                            IntTy::I8 => format!(
                                "stackAllocInt8({})",
                                scalar.to_int(rustc_abi::Size::from_bits(8)).unwrap()
                            ),
                            IntTy::I16 => format!(
                                "stackAllocInt16({})",
                                scalar.to_int(rustc_abi::Size::from_bits(16)).unwrap()
                            ),
                            IntTy::I32 => format!(
                                "stackAllocInt32({})",
                                scalar.to_int(rustc_abi::Size::from_bits(32)).unwrap()
                            ),
                            IntTy::I64 => format!(
                                "stackAllocInt64({})",
                                scalar.to_int(rustc_abi::Size::from_bits(64)).unwrap()
                            ),
                            IntTy::I128 => todo!(),
                            IntTy::Isize => format!(
                                "stackAllocInt32({})",
                                scalar.to_int(rustc_abi::Size::from_bits(32)).unwrap()
                            ),
                        },
                        Uint(uint_ty) => match uint_ty {
                            UintTy::U8 => format!(
                                "stackAllocUint8({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(8)).unwrap()
                            ),
                            UintTy::U16 => format!(
                                "stackAllocUint16({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(16)).unwrap()
                            ),
                            UintTy::U32 => format!(
                                "stackAllocUint32({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(32)).unwrap()
                            ),
                            UintTy::U64 => format!(
                                "stackAllocUint64({})",
                                scalar.to_uint(rustc_abi::Size::from_bits(64)).unwrap()
                            ),
                            UintTy::U128 => todo!(),
                            UintTy::Usize => {
                                format!(
                                    "stackAllocUint32({})",
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
                                def_normalized_name(self.tcx, &adt_def.did(), generic_args),
                                scalar
                            )
                        }
                        Foreign(_) => todo!(),
                        Str => todo!(),
                        Array(_, _) => todo!(),
                        Pat(_, _) => todo!(),
                        Slice(_) => todo!(),
                        RawPtr(_, _mutability) => todo!(),
                        Ref(_, _, _mutability) => {
                            format!("_ref({})", scalar)
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
                        let name = def_normalized_name(self.tcx, &adt_def.did(), generic_args);
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
                    FnDef(id, args) => {
                        let fn_name = self.on_function(id, args);
                        println!("fn_name: {fn_name}");
                        self.out(fn_name);
                    }
                    FnPtr(_binder, _fn_header) => todo!("FnPtr"),
                    UnsafeBinder(_unsafe_binder_inner) => todo!("UnsafeBinder"),
                    Dynamic(_, _) => todo!("Dynamic"),
                    Closure(id, args) => {
                        let fn_name = self.on_function(id, args);
                        self.out(fn_name);
                    }
                    CoroutineClosure(_, _) => todo!("CoroutineClosure"),
                    Coroutine(_, _) => todo!("Coroutine"),
                    CoroutineWitness(_, _) => todo!("CoroutineWitness"),
                    Never => todo!("Never"),
                    Tuple(tys) => {
                        self.out("new Tuple([");
                        for (i, ty) in tys.iter().enumerate() {
                            ty_name(self.tcx, &ty);
                            if i < tys.len() - 1 {
                                self.out(", ");
                            }
                        }
                        self.out("])");
                    }
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
                            self.out("stackAllocBytes(new Uint8Array([".to_string());
                            let inner = memory.inner();
                            let bytes = inner.get_bytes_unchecked((0..inner.len()).into());

                            for (i, byte) in bytes.iter().enumerate() {
                                self.out(format!("{byte}"));
                                if i < bytes.len() - 1 {
                                    self.out(", ");
                                }
                            }
                            self.out("]))");
                        }
                        interpret::GlobalAlloc::TypeId { ty: _ } => todo!("GlobalAlloc::TypeId"),
                    }
                }
                ConstValue::Indirect { alloc_id, offset } => {
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
                            assert_eq!(offset.bytes(), 0);
                            self.out("stackAllocBytes(new Uint8Array([".to_string());
                            let inner = memory.inner();
                            let bytes = inner.get_bytes_unchecked((0..inner.len()).into());

                            for (i, byte) in bytes.iter().enumerate() {
                                self.out(format!("{byte}"));
                                if i < bytes.len() - 1 {
                                    self.out(", ");
                                }
                            }
                            self.out("]))");
                        }
                        interpret::GlobalAlloc::TypeId { ty: _ } => todo!("GlobalAlloc::TypeId"),
                    }
                }
            },
        };
    }
}

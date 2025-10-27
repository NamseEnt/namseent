; ModuleID = 'unwind.c0001114f2fa5739-cgu.0'
source_filename = "unwind.c0001114f2fa5739-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

; core::core_arch::wasm32::unreachable
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9core_arch6wasm3211unreachable17h4fc49d8de9bc2375E() unnamed_addr #0 !dbg !22 {
start:
  call void @llvm.trap(), !dbg !30
  unreachable, !dbg !30
}

; unwind::wasm::_Unwind_RaiseException
; Function Attrs: nounwind
define dso_local i32 @_ZN6unwind4wasm22_Unwind_RaiseException17ha86401ff8c97b5afE(ptr %exception) unnamed_addr #1 !dbg !31 {
start:
  %exception.dbg.spill = alloca [4 x i8], align 4
  store ptr %exception, ptr %exception.dbg.spill, align 4
    #dbg_declare(ptr %exception.dbg.spill, !65, !DIExpression(), !66)
; call core::core_arch::wasm32::unreachable
  call void @_ZN4core9core_arch6wasm3211unreachable17h4fc49d8de9bc2375E() #3, !dbg !67
  unreachable, !dbg !67
}

; unwind::wasm::_Unwind_DeleteException
; Function Attrs: nounwind
define dso_local void @_ZN6unwind4wasm23_Unwind_DeleteException17h681e99c9e311f412E(ptr %exception) unnamed_addr #1 !dbg !68 {
start:
  %exception_cleanup.dbg.spill = alloca [4 x i8], align 4
  %exception.dbg.spill = alloca [4 x i8], align 4
  %_2 = alloca [4 x i8], align 4
  store ptr %exception, ptr %exception.dbg.spill, align 4
    #dbg_declare(ptr %exception.dbg.spill, !72, !DIExpression(), !75)
  %0 = getelementptr inbounds i8, ptr %exception, i32 8, !dbg !76
  %1 = load ptr, ptr %0, align 8, !dbg !76
  store ptr %1, ptr %_2, align 4, !dbg !76
  %2 = load ptr, ptr %_2, align 4, !dbg !77
  %3 = ptrtoint ptr %2 to i32, !dbg !77
  %4 = icmp eq i32 %3, 0, !dbg !77
  %_3 = select i1 %4, i32 0, i32 1, !dbg !77
  %5 = trunc nuw i32 %_3 to i1, !dbg !78
  br i1 %5, label %bb1, label %bb2, !dbg !78

bb1:                                              ; preds = %start
  %exception_cleanup = load ptr, ptr %_2, align 4, !dbg !79
  store ptr %exception_cleanup, ptr %exception_cleanup.dbg.spill, align 4, !dbg !79
    #dbg_declare(ptr %exception_cleanup.dbg.spill, !73, !DIExpression(), !79)
  call void %exception_cleanup(i32 1, ptr %exception) #4, !dbg !80
  br label %bb2, !dbg !80

bb2:                                              ; preds = %bb1, %start
  ret void, !dbg !81

bb3:                                              ; No predecessors!
  unreachable, !dbg !82
}

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #2

attributes #0 = { inlinehint noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #3 = { noreturn nounwind }
attributes #4 = { nounwind }

!llvm.ident = !{!0}
!llvm.dbg.cu = !{!1}
!llvm.module.flags = !{!20, !21}

!0 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!1 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !2, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false, nameTableKind: None)
!2 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/unwind/src/lib.rs/@/unwind.c0001114f2fa5739-cgu.0", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/unwind")
!3 = !{!4}
!4 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "_Unwind_Reason_Code", scope: !6, file: !5, baseType: !8, size: 32, align: 32, flags: DIFlagEnumClass, elements: !9)
!5 = !DIFile(filename: "<unknown>", directory: "")
!6 = !DINamespace(name: "wasm", scope: !7)
!7 = !DINamespace(name: "unwind", scope: null)
!8 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!9 = !{!10, !11, !12, !13, !14, !15, !16, !17, !18, !19}
!10 = !DIEnumerator(name: "_URC_NO_REASON", value: 0, isUnsigned: true)
!11 = !DIEnumerator(name: "_URC_FOREIGN_EXCEPTION_CAUGHT", value: 1, isUnsigned: true)
!12 = !DIEnumerator(name: "_URC_FATAL_PHASE2_ERROR", value: 2, isUnsigned: true)
!13 = !DIEnumerator(name: "_URC_FATAL_PHASE1_ERROR", value: 3, isUnsigned: true)
!14 = !DIEnumerator(name: "_URC_NORMAL_STOP", value: 4, isUnsigned: true)
!15 = !DIEnumerator(name: "_URC_END_OF_STACK", value: 5, isUnsigned: true)
!16 = !DIEnumerator(name: "_URC_HANDLER_FOUND", value: 6, isUnsigned: true)
!17 = !DIEnumerator(name: "_URC_INSTALL_CONTEXT", value: 7, isUnsigned: true)
!18 = !DIEnumerator(name: "_URC_CONTINUE_UNWIND", value: 8, isUnsigned: true)
!19 = !DIEnumerator(name: "_URC_FAILURE", value: 9, isUnsigned: true)
!20 = !{i32 7, !"Dwarf Version", i32 4}
!21 = !{i32 2, !"Debug Info Version", i32 3}
!22 = distinct !DISubprogram(name: "unreachable", linkageName: "_ZN4core9core_arch6wasm3211unreachable17h4fc49d8de9bc2375E", scope: !24, file: !23, line: 31, type: !27, scopeLine: 31, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, templateParams: !29)
!23 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/wasm32/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "855e79eb4cdda24b35a73da48ddc7027")
!24 = !DINamespace(name: "wasm32", scope: !25)
!25 = !DINamespace(name: "core_arch", scope: !26)
!26 = !DINamespace(name: "core", scope: null)
!27 = !DISubroutineType(types: !28)
!28 = !{null}
!29 = !{}
!30 = !DILocation(line: 32, column: 5, scope: !22)
!31 = distinct !DISubprogram(name: "_Unwind_RaiseException", linkageName: "_ZN6unwind4wasm22_Unwind_RaiseException17ha86401ff8c97b5afE", scope: !6, file: !32, line: 42, type: !33, scopeLine: 42, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !1, templateParams: !29, retainedNodes: !64)
!32 = !DIFile(filename: "src/wasm.rs", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/unwind", checksumkind: CSK_MD5, checksum: "dfe2460ff30d51d2cab1af1b89ae324a")
!33 = !DISubroutineType(types: !34)
!34 = !{!4, !35}
!35 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut unwind::wasm::_Unwind_Exception", baseType: !36, size: 32, align: 32, dwarfAddressSpace: 0)
!36 = !DICompositeType(tag: DW_TAG_structure_type, name: "_Unwind_Exception", scope: !6, file: !5, size: 192, align: 64, flags: DIFlagPublic, elements: !37, templateParams: !29, identifier: "143b72a6d9acf5a5f1719582afd5a7c")
!37 = !{!38, !40, !58}
!38 = !DIDerivedType(tag: DW_TAG_member, name: "exception_class", scope: !36, file: !5, baseType: !39, size: 64, align: 64, flags: DIFlagPublic)
!39 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!40 = !DIDerivedType(tag: DW_TAG_member, name: "exception_cleanup", scope: !36, file: !5, baseType: !41, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!41 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<extern \22C\22 fn(unwind::wasm::_Unwind_Reason_Code, *mut unwind::wasm::_Unwind_Exception)>", scope: !42, file: !5, size: 32, align: 32, flags: DIFlagPublic, elements: !43, templateParams: !29, identifier: "e886b2d6c83c340e60a3da4758145bc0")
!42 = !DINamespace(name: "option", scope: !26)
!43 = !{!44}
!44 = !DICompositeType(tag: DW_TAG_variant_part, scope: !41, file: !5, size: 32, align: 32, elements: !45, templateParams: !29, identifier: "452af262fb7c4f8bba06199152ebee49", discriminator: !57)
!45 = !{!46, !53}
!46 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !44, file: !5, baseType: !47, size: 32, align: 32, extraData: i32 0)
!47 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !41, file: !5, size: 32, align: 32, flags: DIFlagPublic, elements: !29, templateParams: !48, identifier: "a71ea4bc3b5def1237da70014cdba56b")
!48 = !{!49}
!49 = !DITemplateTypeParameter(name: "T", type: !50)
!50 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "extern \22C\22 fn(unwind::wasm::_Unwind_Reason_Code, *mut unwind::wasm::_Unwind_Exception)", baseType: !51, size: 32, align: 32, dwarfAddressSpace: 0)
!51 = !DISubroutineType(types: !52)
!52 = !{null, !4, !35}
!53 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !44, file: !5, baseType: !54, size: 32, align: 32)
!54 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !41, file: !5, size: 32, align: 32, flags: DIFlagPublic, elements: !55, templateParams: !48, identifier: "b6d5060a0b224e9dcd0c7db0baa064e2")
!55 = !{!56}
!56 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !54, file: !5, baseType: !50, size: 32, align: 32, flags: DIFlagPublic)
!57 = !DIDerivedType(tag: DW_TAG_member, scope: !41, file: !5, baseType: !8, size: 32, align: 32, flags: DIFlagArtificial)
!58 = !DIDerivedType(tag: DW_TAG_member, name: "private", scope: !36, file: !5, baseType: !59, size: 64, align: 32, offset: 96, flags: DIFlagPublic)
!59 = !DICompositeType(tag: DW_TAG_array_type, baseType: !60, size: 64, align: 32, elements: !62)
!60 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !61, size: 32, align: 32, dwarfAddressSpace: 0)
!61 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!62 = !{!63}
!63 = !DISubrange(count: 2, lowerBound: 0)
!64 = !{!65}
!65 = !DILocalVariable(name: "exception", arg: 1, scope: !31, file: !32, line: 42, type: !35)
!66 = !DILocation(line: 42, column: 38, scope: !31)
!67 = !DILocation(line: 62, column: 13, scope: !31)
!68 = distinct !DISubprogram(name: "_Unwind_DeleteException", linkageName: "_ZN6unwind4wasm23_Unwind_DeleteException17h681e99c9e311f412E", scope: !6, file: !32, line: 36, type: !69, scopeLine: 36, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !1, templateParams: !29, retainedNodes: !71)
!69 = !DISubroutineType(types: !70)
!70 = !{null, !35}
!71 = !{!72, !73}
!72 = !DILocalVariable(name: "exception", arg: 1, scope: !68, file: !32, line: 36, type: !35)
!73 = !DILocalVariable(name: "exception_cleanup", scope: !74, file: !32, line: 37, type: !50, align: 32)
!74 = distinct !DILexicalBlock(scope: !68, file: !32, line: 37, column: 80)
!75 = !DILocation(line: 36, column: 39, scope: !68)
!76 = !DILocation(line: 37, column: 47, scope: !74)
!77 = !DILocation(line: 37, column: 38, scope: !74)
!78 = !DILocation(line: 37, column: 12, scope: !74)
!79 = !DILocation(line: 37, column: 17, scope: !74)
!80 = !DILocation(line: 38, column: 9, scope: !74)
!81 = !DILocation(line: 40, column: 2, scope: !68)
!82 = !DILocation(line: 36, column: 1, scope: !68)

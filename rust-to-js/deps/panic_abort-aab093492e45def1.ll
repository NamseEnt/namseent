; ModuleID = 'panic_abort.4a411ff1b59140ab-cgu.0'
source_filename = "panic_abort.4a411ff1b59140ab-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

@alloc_a500d906b91607583596fa15e63c2ada = private unnamed_addr constant [40 x i8] c"internal error: entered unreachable code", align 1
@alloc_73fbda40d053ce228ec09f31a1aa96b9 = private unnamed_addr constant [113 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_abort/src/lib.rs\00", align 1
@alloc_0ba91b400134e23df90d53924b10e3f9 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_73fbda40d053ce228ec09f31a1aa96b9, [12 x i8] c"p\00\00\00\1C\00\00\00\05\00\00\00" }>, align 4

; __rustc::__rust_start_panic
; Function Attrs: nounwind
define dso_local i32 @_RNvCsaKOfhLrNfTz_7___rustc18___rust_start_panic(ptr align 1 %_payload.0, ptr align 4 %_payload.1) unnamed_addr #0 !dbg !5 {
start:
  %_payload.dbg.spill = alloca [8 x i8], align 4
  store ptr %_payload.0, ptr %_payload.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %_payload.dbg.spill, i32 4
  store ptr %_payload.1, ptr %0, align 4
    #dbg_declare(ptr %_payload.dbg.spill, !25, !DIExpression(), !26)
; call __rustc::__rust_abort
  call void @_RNvCsaKOfhLrNfTz_7___rustc12___rust_abort() #5, !dbg !27
  unreachable, !dbg !27
}

; __rustc::__rust_panic_cleanup
; Function Attrs: nounwind
define dso_local void @_RNvCsaKOfhLrNfTz_7___rustc20___rust_panic_cleanup(ptr sret([8 x i8]) align 4 %_0, ptr %_1) unnamed_addr #0 !dbg !28 {
start:
  %_1.dbg.spill = alloca [4 x i8], align 4
  store ptr %_1, ptr %_1.dbg.spill, align 4
    #dbg_declare(ptr %_1.dbg.spill, !44, !DIExpression(), !45)
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h5f40038b331514a4E(ptr align 1 @alloc_a500d906b91607583596fa15e63c2ada, i32 40, ptr align 4 @alloc_0ba91b400134e23df90d53924b10e3f9) #5, !dbg !46
  unreachable, !dbg !46
}

; core::panicking::panic
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking5panic17h5f40038b331514a4E(ptr align 1 %expr.0, i32 %expr.1, ptr align 4 %0) unnamed_addr #1 !dbg !47 {
start:
  %expr.dbg.spill = alloca [8 x i8], align 4
  %_5 = alloca [8 x i8], align 4
  %_3 = alloca [24 x i8], align 4
  store ptr %expr.0, ptr %expr.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %expr.dbg.spill, i32 4
  store i32 %expr.1, ptr %1, align 4
    #dbg_declare(ptr %expr.dbg.spill, !83, !DIExpression(), !84)
  %2 = getelementptr inbounds nuw { ptr, i32 }, ptr %_5, i32 0, !dbg !85
  store ptr %expr.0, ptr %2, align 4, !dbg !85
  %3 = getelementptr inbounds i8, ptr %2, i32 4, !dbg !85
  store i32 %expr.1, ptr %3, align 4, !dbg !85
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_3, ptr align 4 %_5) #6, !dbg !86
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h31a9f2656db4c0fbE(ptr align 4 %_3, ptr align 4 %0) #5, !dbg !87
  unreachable, !dbg !87
}

; core::panicking::panic_fmt
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking9panic_fmt17h31a9f2656db4c0fbE(ptr align 4 %fmt, ptr align 4 %0) unnamed_addr #1 !dbg !88 {
start:
    #dbg_declare(ptr %fmt, !219, !DIExpression(), !231)
  call void @llvm.trap(), !dbg !232
  unreachable, !dbg !232
}

; __rustc::__rust_abort
; Function Attrs: noreturn nounwind
declare dso_local void @_RNvCsaKOfhLrNfTz_7___rustc12___rust_abort() unnamed_addr #2

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nounwind
declare dso_local void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4, ptr align 4) unnamed_addr #3

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #4

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { inlinehint noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #3 = { inlinehint nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #4 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #5 = { noreturn nounwind }
attributes #6 = { nounwind }

!llvm.ident = !{!0}
!llvm.dbg.cu = !{!1}
!llvm.module.flags = !{!3, !4}

!0 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!1 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !2, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)
!2 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_abort/src/lib.rs/@/panic_abort.4a411ff1b59140ab-cgu.0", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_abort")
!3 = !{i32 7, !"Dwarf Version", i32 4}
!4 = !{i32 2, !"Debug Info Version", i32 3}
!5 = distinct !DISubprogram(name: "__rust_start_panic", linkageName: "_RNvCsaKOfhLrNfTz_7___rustc18___rust_start_panic", scope: !7, file: !6, line: 33, type: !8, scopeLine: 33, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !1, templateParams: !17, retainedNodes: !24)
!6 = !DIFile(filename: "src/lib.rs", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_abort", checksumkind: CSK_MD5, checksum: "ab7247787d653ccb11f9cd408ed39fb0")
!7 = !DINamespace(name: "panic_abort", scope: null)
!8 = !DISubroutineType(types: !9)
!9 = !{!10, !11}
!10 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!11 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::panic::PanicPayload", file: !12, size: 64, align: 32, elements: !13, templateParams: !17, identifier: "67ba2170db1d53ab1e91431929796ef1")
!12 = !DIFile(filename: "<unknown>", directory: "")
!13 = !{!14, !18}
!14 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !11, file: !12, baseType: !15, size: 32, align: 32)
!15 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !16, size: 32, align: 32, dwarfAddressSpace: 0)
!16 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::panic::PanicPayload", file: !12, align: 8, elements: !17, identifier: "553ec5eb6080bf5f762a22dd2a24a55d")
!17 = !{}
!18 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !11, file: !12, baseType: !19, size: 32, align: 32, offset: 32)
!19 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 7]", baseType: !20, size: 32, align: 32, dwarfAddressSpace: 0)
!20 = !DICompositeType(tag: DW_TAG_array_type, baseType: !21, size: 224, align: 32, elements: !22)
!21 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!22 = !{!23}
!23 = !DISubrange(count: 7, lowerBound: 0)
!24 = !{!25}
!25 = !DILocalVariable(name: "_payload", arg: 1, scope: !5, file: !6, line: 33, type: !11)
!26 = !DILocation(line: 33, column: 34, scope: !5)
!27 = !DILocation(line: 50, column: 5, scope: !5)
!28 = distinct !DISubprogram(name: "__rust_panic_cleanup", linkageName: "_RNvCsaKOfhLrNfTz_7___rustc20___rust_panic_cleanup", scope: !7, file: !6, line: 27, type: !29, scopeLine: 27, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !1, templateParams: !17, retainedNodes: !43)
!29 = !DISubroutineType(types: !30)
!30 = !{!31, !41}
!31 = !DICompositeType(tag: DW_TAG_structure_type, name: "*mut (dyn core::any::Any + core::marker::Send)", file: !12, size: 64, align: 32, elements: !32, templateParams: !17, identifier: "f7b782cc162036cd5dc3538cdb11c32d")
!32 = !{!33, !36}
!33 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !31, file: !12, baseType: !34, size: 32, align: 32)
!34 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !35, size: 32, align: 32, dwarfAddressSpace: 0)
!35 = !DICompositeType(tag: DW_TAG_structure_type, name: "(dyn core::any::Any + core::marker::Send)", file: !12, align: 8, elements: !17, identifier: "1d1aa39a9f0ef7c085a062eeef752e17")
!36 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !31, file: !12, baseType: !37, size: 32, align: 32, offset: 32)
!37 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 4]", baseType: !38, size: 32, align: 32, dwarfAddressSpace: 0)
!38 = !DICompositeType(tag: DW_TAG_array_type, baseType: !21, size: 128, align: 32, elements: !39)
!39 = !{!40}
!40 = !DISubrange(count: 4, lowerBound: 0)
!41 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !42, size: 32, align: 32, dwarfAddressSpace: 0)
!42 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!43 = !{!44}
!44 = !DILocalVariable(arg: 1, scope: !28, file: !6, line: 27, type: !41)
!45 = !DILocation(line: 27, column: 47, scope: !28)
!46 = !DILocation(line: 28, column: 5, scope: !28)
!47 = distinct !DISubprogram(name: "panic", linkageName: "_ZN4core9panicking5panic17h5f40038b331514a4E", scope: !49, file: !48, line: 138, type: !51, scopeLine: 138, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, templateParams: !17, retainedNodes: !82)
!48 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/panicking.rs", directory: "", checksumkind: CSK_MD5, checksum: "b120da646d1a09f31201b8a519374e57")
!49 = !DINamespace(name: "panicking", scope: !50)
!50 = !DINamespace(name: "core", scope: null)
!51 = !DISubroutineType(types: !52)
!52 = !{null, !53, !58}
!53 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !12, size: 64, align: 32, elements: !54, templateParams: !17, identifier: "9277eecd40495f85161460476aacc992")
!54 = !{!55, !57}
!55 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !53, file: !12, baseType: !56, size: 32, align: 32)
!56 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !42, size: 32, align: 32, dwarfAddressSpace: 0)
!57 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !53, file: !12, baseType: !21, size: 32, align: 32, offset: 32)
!58 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::location::Location", baseType: !59, size: 32, align: 32, dwarfAddressSpace: 0)
!59 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !60, file: !12, size: 128, align: 32, flags: DIFlagPublic, elements: !62, templateParams: !17, identifier: "7c34cafe8ea1dcad4032b9360816105f")
!60 = !DINamespace(name: "location", scope: !61)
!61 = !DINamespace(name: "panic", scope: !50)
!62 = !{!63, !75, !76, !77}
!63 = !DIDerivedType(tag: DW_TAG_member, name: "filename", scope: !59, file: !12, baseType: !64, size: 64, align: 32, flags: DIFlagPrivate)
!64 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<str>", scope: !65, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !67, templateParams: !73, identifier: "88212fc410c4399fd5095990cc8304ca")
!65 = !DINamespace(name: "non_null", scope: !66)
!66 = !DINamespace(name: "ptr", scope: !50)
!67 = !{!68}
!68 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !64, file: !12, baseType: !69, size: 64, align: 32, flags: DIFlagPrivate)
!69 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const str", file: !12, size: 64, align: 32, elements: !70, templateParams: !17, identifier: "238a44609877474087c05adf26cd41fa")
!70 = !{!71, !72}
!71 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !69, file: !12, baseType: !56, size: 32, align: 32)
!72 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !69, file: !12, baseType: !21, size: 32, align: 32, offset: 32)
!73 = !{!74}
!74 = !DITemplateTypeParameter(name: "T", type: !42)
!75 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !59, file: !12, baseType: !10, size: 32, align: 32, offset: 64, flags: DIFlagPrivate)
!76 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !59, file: !12, baseType: !10, size: 32, align: 32, offset: 96, flags: DIFlagPrivate)
!77 = !DIDerivedType(tag: DW_TAG_member, name: "_filename", scope: !59, file: !12, baseType: !78, align: 8, offset: 128, flags: DIFlagPrivate)
!78 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&str>", scope: !79, file: !12, align: 8, flags: DIFlagPublic, elements: !17, templateParams: !80, identifier: "4cfc3eea77dd95eabd59051b67bd7e66")
!79 = !DINamespace(name: "marker", scope: !50)
!80 = !{!81}
!81 = !DITemplateTypeParameter(name: "T", type: !53)
!82 = !{!83}
!83 = !DILocalVariable(name: "expr", arg: 1, scope: !47, file: !48, line: 138, type: !53)
!84 = !DILocation(line: 138, column: 20, scope: !47)
!85 = !DILocation(line: 150, column: 42, scope: !47)
!86 = !DILocation(line: 150, column: 15, scope: !47)
!87 = !DILocation(line: 150, column: 5, scope: !47)
!88 = distinct !DISubprogram(name: "panic_fmt", linkageName: "_ZN4core9panicking9panic_fmt17h31a9f2656db4c0fbE", scope: !49, file: !48, line: 60, type: !89, scopeLine: 60, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !1, templateParams: !17, retainedNodes: !218)
!89 = !DISubroutineType(types: !90)
!90 = !{null, !91, !58}
!91 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !92, file: !12, size: 192, align: 32, flags: DIFlagPublic, elements: !93, templateParams: !17, identifier: "d691e62b2ee4847c2af32873f04bd10")
!92 = !DINamespace(name: "fmt", scope: !50)
!93 = !{!94, !100, !143}
!94 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !91, file: !12, baseType: !95, size: 64, align: 32, flags: DIFlagPrivate)
!95 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !12, size: 64, align: 32, elements: !96, templateParams: !17, identifier: "4e66b00a376d6af5b8765440fb2839f")
!96 = !{!97, !99}
!97 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !95, file: !12, baseType: !98, size: 32, align: 32)
!98 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !53, size: 32, align: 32, dwarfAddressSpace: 0)
!99 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !95, file: !12, baseType: !21, size: 32, align: 32, offset: 32)
!100 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !91, file: !12, baseType: !101, size: 64, align: 32, offset: 128, flags: DIFlagPrivate)
!101 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !102, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !103, templateParams: !17, identifier: "a638667a460b22fe10961f9a2f3202aa")
!102 = !DINamespace(name: "option", scope: !50)
!103 = !{!104}
!104 = !DICompositeType(tag: DW_TAG_variant_part, scope: !101, file: !12, size: 64, align: 32, elements: !105, templateParams: !17, identifier: "29af53ccc7f21f4d5671e352d673889a", discriminator: !142)
!105 = !{!106, !138}
!106 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !104, file: !12, baseType: !107, size: 64, align: 32, extraData: i32 0)
!107 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !101, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !17, templateParams: !108, identifier: "11ce4f4d10f67887bbe6bf59a521c479")
!108 = !{!109}
!109 = !DITemplateTypeParameter(name: "T", type: !110)
!110 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !12, size: 64, align: 32, elements: !111, templateParams: !17, identifier: "b0485535d7020130e949c24f3fc2aa00")
!111 = !{!112, !137}
!112 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !110, file: !12, baseType: !113, size: 32, align: 32)
!113 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !114, size: 32, align: 32, dwarfAddressSpace: 0)
!114 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !115, file: !12, size: 192, align: 32, flags: DIFlagPublic, elements: !116, templateParams: !17, identifier: "8cb06f9d78dc629c8f52fc3b5544996c")
!115 = !DINamespace(name: "rt", scope: !92)
!116 = !{!117, !118, !119, !136}
!117 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !114, file: !12, baseType: !21, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
!118 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !114, file: !12, baseType: !10, size: 32, align: 32, offset: 160, flags: DIFlagPublic)
!119 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !114, file: !12, baseType: !120, size: 64, align: 32, flags: DIFlagPublic)
!120 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !115, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !121, templateParams: !17, identifier: "2d7772037f5c744e87d41105441784d5")
!121 = !{!122}
!122 = !DICompositeType(tag: DW_TAG_variant_part, scope: !120, file: !12, size: 64, align: 32, elements: !123, templateParams: !17, identifier: "af14687975a61e1ae6bbcdaeb79a8a2", discriminator: !135)
!123 = !{!124, !129, !133}
!124 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !122, file: !12, baseType: !125, size: 64, align: 32, extraData: i16 0)
!125 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !120, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !126, templateParams: !17, identifier: "da16c9b5356522ffb015c0e99237342e")
!126 = !{!127}
!127 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !125, file: !12, baseType: !128, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!128 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!129 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !122, file: !12, baseType: !130, size: 64, align: 32, extraData: i16 1)
!130 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !120, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !131, templateParams: !17, identifier: "8d84b26eccf0f48fe70ea50c79b83fc9")
!131 = !{!132}
!132 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !130, file: !12, baseType: !21, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!133 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !122, file: !12, baseType: !134, size: 64, align: 32, extraData: i16 2)
!134 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !120, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !17, identifier: "e4d910bcc0c2da0048af65cce9b02bdf")
!135 = !DIDerivedType(tag: DW_TAG_member, scope: !120, file: !12, baseType: !128, size: 16, align: 16, flags: DIFlagArtificial)
!136 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !114, file: !12, baseType: !120, size: 64, align: 32, offset: 64, flags: DIFlagPublic)
!137 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !110, file: !12, baseType: !21, size: 32, align: 32, offset: 32)
!138 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !104, file: !12, baseType: !139, size: 64, align: 32)
!139 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !101, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !140, templateParams: !108, identifier: "b6f59188292a44db7736125146b92cb0")
!140 = !{!141}
!141 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !139, file: !12, baseType: !110, size: 64, align: 32, flags: DIFlagPublic)
!142 = !DIDerivedType(tag: DW_TAG_member, scope: !101, file: !12, baseType: !10, size: 32, align: 32, flags: DIFlagArtificial)
!143 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !91, file: !12, baseType: !144, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!144 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !12, size: 64, align: 32, elements: !145, templateParams: !17, identifier: "14634098cacc86d372c43019bc81f26f")
!145 = !{!146, !217}
!146 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !144, file: !12, baseType: !147, size: 32, align: 32)
!147 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !148, size: 32, align: 32, dwarfAddressSpace: 0)
!148 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !115, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !149, templateParams: !17, identifier: "14dca3c1b1040cd8e8db0eaa112c8216")
!149 = !{!150}
!150 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !148, file: !12, baseType: !151, size: 64, align: 32, flags: DIFlagPrivate)
!151 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !115, file: !12, size: 64, align: 32, flags: DIFlagPrivate, elements: !152, templateParams: !17, identifier: "fb1492950c21086074bab206592842dc")
!152 = !{!153}
!153 = !DICompositeType(tag: DW_TAG_variant_part, scope: !151, file: !12, size: 64, align: 32, elements: !154, templateParams: !17, identifier: "478e018ae6e38e2110d0d424641ab18", discriminator: !216)
!154 = !{!155, !212}
!155 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !153, file: !12, baseType: !156, size: 64, align: 32)
!156 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !151, file: !12, size: 64, align: 32, flags: DIFlagPrivate, elements: !157, templateParams: !17, identifier: "59bc7f5c5a99ab4be3c3f06b9190c327")
!157 = !{!158, !166, !207}
!158 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !156, file: !12, baseType: !159, size: 32, align: 32, flags: DIFlagPrivate)
!159 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !65, file: !12, size: 32, align: 32, flags: DIFlagPublic, elements: !160, templateParams: !164, identifier: "d9f2bcb64deb934daba9b509aea4a83e")
!160 = !{!161}
!161 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !159, file: !12, baseType: !162, size: 32, align: 32, flags: DIFlagPrivate)
!162 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !163, size: 32, align: 32, dwarfAddressSpace: 0)
!163 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!164 = !{!165}
!165 = !DITemplateTypeParameter(name: "T", type: !163)
!166 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !156, file: !12, baseType: !167, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!167 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !168, size: 32, align: 32, dwarfAddressSpace: 0)
!168 = !DISubroutineType(types: !169)
!169 = !{!170, !159, !187}
!170 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !171, file: !12, size: 8, align: 8, flags: DIFlagPublic, elements: !172, templateParams: !17, identifier: "613ace46ae0c395d39c31f05d3934750")
!171 = !DINamespace(name: "result", scope: !50)
!172 = !{!173}
!173 = !DICompositeType(tag: DW_TAG_variant_part, scope: !170, file: !12, size: 8, align: 8, elements: !174, templateParams: !17, identifier: "2bd67c77928327a5a86e1d970227dbc3", discriminator: !186)
!174 = !{!175, !182}
!175 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !173, file: !12, baseType: !176, size: 8, align: 8, extraData: i8 0)
!176 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !170, file: !12, size: 8, align: 8, flags: DIFlagPublic, elements: !177, templateParams: !179, identifier: "8e1fa5ea2cd8f77479a16f216aa53a42")
!177 = !{!178}
!178 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !176, file: !12, baseType: !163, align: 8, offset: 8, flags: DIFlagPublic)
!179 = !{!165, !180}
!180 = !DITemplateTypeParameter(name: "E", type: !181)
!181 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !92, file: !12, align: 8, flags: DIFlagPublic, elements: !17, identifier: "cac4d2a6635a122844ffbe3b52a15933")
!182 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !173, file: !12, baseType: !183, size: 8, align: 8, extraData: i8 1)
!183 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !170, file: !12, size: 8, align: 8, flags: DIFlagPublic, elements: !184, templateParams: !179, identifier: "bd8eb8fbb58ca24e2467a7f35c864471")
!184 = !{!185}
!185 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !183, file: !12, baseType: !181, align: 8, offset: 8, flags: DIFlagPublic)
!186 = !DIDerivedType(tag: DW_TAG_member, scope: !170, file: !12, baseType: !42, size: 8, align: 8, flags: DIFlagArtificial)
!187 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !188, size: 32, align: 32, dwarfAddressSpace: 0)
!188 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !92, file: !12, size: 128, align: 32, flags: DIFlagPublic, elements: !189, templateParams: !17, identifier: "9c19c8ef0b5ae3cad350e741e841742c")
!189 = !{!190, !196}
!190 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !188, file: !12, baseType: !191, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!191 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !92, file: !12, size: 64, align: 32, flags: DIFlagPublic, elements: !192, templateParams: !17, identifier: "8e7d20540a73fe2190308d0618721e3e")
!192 = !{!193, !194, !195}
!193 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !191, file: !12, baseType: !10, size: 32, align: 32, flags: DIFlagPrivate)
!194 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !191, file: !12, baseType: !128, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!195 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !191, file: !12, baseType: !128, size: 16, align: 16, offset: 48, flags: DIFlagPrivate)
!196 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !188, file: !12, baseType: !197, size: 64, align: 32, flags: DIFlagPrivate)
!197 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !12, size: 64, align: 32, elements: !198, templateParams: !17, identifier: "ed1fc41b72305de4afb5dbb44887680d")
!198 = !{!199, !202}
!199 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !197, file: !12, baseType: !200, size: 32, align: 32)
!200 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !201, size: 32, align: 32, dwarfAddressSpace: 0)
!201 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !12, align: 8, elements: !17, identifier: "3bd7022d6bc7a1bba9386a42dfa7db9d")
!202 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !197, file: !12, baseType: !203, size: 32, align: 32, offset: 32)
!203 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !204, size: 32, align: 32, dwarfAddressSpace: 0)
!204 = !DICompositeType(tag: DW_TAG_array_type, baseType: !21, size: 192, align: 32, elements: !205)
!205 = !{!206}
!206 = !DISubrange(count: 6, lowerBound: 0)
!207 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !156, file: !12, baseType: !208, align: 8, offset: 64, flags: DIFlagPrivate)
!208 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !79, file: !12, align: 8, flags: DIFlagPublic, elements: !17, templateParams: !209, identifier: "e71ee38df7dbfccdae82d3411c10d5bc")
!209 = !{!210}
!210 = !DITemplateTypeParameter(name: "T", type: !211)
!211 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !163, size: 32, align: 32, dwarfAddressSpace: 0)
!212 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !153, file: !12, baseType: !213, size: 64, align: 32, extraData: i32 0)
!213 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !151, file: !12, size: 64, align: 32, flags: DIFlagPrivate, elements: !214, templateParams: !17, identifier: "bcc61db69ea5777ac138ac099ea396b2")
!214 = !{!215}
!215 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !213, file: !12, baseType: !128, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!216 = !DIDerivedType(tag: DW_TAG_member, scope: !151, file: !12, baseType: !10, size: 32, align: 32, flags: DIFlagArtificial)
!217 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !144, file: !12, baseType: !21, size: 32, align: 32, offset: 32)
!218 = !{!219, !220}
!219 = !DILocalVariable(name: "fmt", arg: 1, scope: !88, file: !48, line: 60, type: !91)
!220 = !DILocalVariable(name: "pi", scope: !221, file: !48, line: 72, type: !222, align: 32)
!221 = distinct !DILexicalBlock(scope: !88, file: !48, line: 72, column: 5)
!222 = !DICompositeType(tag: DW_TAG_structure_type, name: "PanicInfo", scope: !223, file: !12, size: 96, align: 32, flags: DIFlagPublic, elements: !224, templateParams: !17, identifier: "74943ad5cfeaa8d7c3439d6f603267a6")
!223 = !DINamespace(name: "panic_info", scope: !61)
!224 = !{!225, !227, !228, !230}
!225 = !DIDerivedType(tag: DW_TAG_member, name: "message", scope: !222, file: !12, baseType: !226, size: 32, align: 32, flags: DIFlagPrivate)
!226 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::Arguments", baseType: !91, size: 32, align: 32, dwarfAddressSpace: 0)
!227 = !DIDerivedType(tag: DW_TAG_member, name: "location", scope: !222, file: !12, baseType: !58, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!228 = !DIDerivedType(tag: DW_TAG_member, name: "can_unwind", scope: !222, file: !12, baseType: !229, size: 8, align: 8, offset: 64, flags: DIFlagPrivate)
!229 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!230 = !DIDerivedType(tag: DW_TAG_member, name: "force_no_backtrace", scope: !222, file: !12, baseType: !229, size: 8, align: 8, offset: 72, flags: DIFlagPrivate)
!231 = !DILocation(line: 60, column: 24, scope: !88)
!232 = !DILocation(line: 62, column: 9, scope: !88)

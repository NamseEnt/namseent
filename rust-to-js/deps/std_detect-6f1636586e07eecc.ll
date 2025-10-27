; ModuleID = 'std_detect.6dbc85b02342858c-cgu.0'
source_filename = "std_detect.6dbc85b02342858c-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

@_ZN10std_detect6detect5cache5CACHE17h41e8835a3f73da2aE = internal global [12 x i8] zeroinitializer, align 4, !dbg !0

; std_detect::detect::features
; Function Attrs: nounwind
define dso_local void @_ZN10std_detect6detect8features17h221ac7a82cfc441eE(ptr sret([12 x i8]) align 4 %_0) unnamed_addr #0 !dbg !33 {
start:
  %_1 = alloca [12 x i8], align 4
  %0 = getelementptr inbounds i8, ptr %_1, i32 8, !dbg !70
  store i8 2, ptr %0, align 4, !dbg !70
; call <core::option::Option<T> as core::iter::traits::collect::IntoIterator>::into_iter
  call void @"_ZN91_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h8dcc6161949352d6E"(ptr sret([12 x i8]) align 4 %_0, ptr align 4 %_1) #3, !dbg !71
  ret void, !dbg !72
}

; <core::option::Option<T> as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: inlinehint nounwind
define dso_local void @"_ZN91_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h8dcc6161949352d6E"(ptr sret([12 x i8]) align 4 %_0, ptr align 4 %self) unnamed_addr #1 !dbg !73 {
start:
  %_2 = alloca [12 x i8], align 4
    #dbg_declare(ptr %self, !79, !DIExpression(), !80)
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_2, ptr align 4 %self, i32 12, i1 false), !dbg !81
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_0, ptr align 4 %_2, i32 12, i1 false), !dbg !82
  ret void, !dbg !83
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #2

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { inlinehint nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { nounwind }

!llvm.ident = !{!27}
!llvm.dbg.cu = !{!28}
!llvm.module.flags = !{!31, !32}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "CACHE", linkageName: "_ZN10std_detect6detect5cache5CACHE17h41e8835a3f73da2aE", scope: !2, file: !5, line: 65, type: !6, isLocal: true, isDefinition: true, align: 32)
!2 = !DINamespace(name: "cache", scope: !3)
!3 = !DINamespace(name: "detect", scope: !4)
!4 = !DINamespace(name: "std_detect", scope: null)
!5 = !DIFile(filename: "src/detect/cache.rs", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std_detect", checksumkind: CSK_MD5, checksum: "c622e53654066d071c495e8ec2d53c97")
!6 = !DICompositeType(tag: DW_TAG_array_type, baseType: !7, size: 96, align: 32, elements: !25)
!7 = !DICompositeType(tag: DW_TAG_structure_type, name: "Cache", scope: !2, file: !8, size: 32, align: 32, flags: DIFlagPrivate, elements: !9, templateParams: !24, identifier: "899d99165a0a23bbe4a7c2cbc70833af")
!8 = !DIFile(filename: "<unknown>", directory: "")
!9 = !{!10}
!10 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !7, file: !8, baseType: !11, size: 32, align: 32, flags: DIFlagPrivate)
!11 = !DICompositeType(tag: DW_TAG_structure_type, name: "AtomicUsize", scope: !12, file: !8, size: 32, align: 32, flags: DIFlagPublic, elements: !15, templateParams: !24, identifier: "210fde280431e7a7a2ebb6338b4a928")
!12 = !DINamespace(name: "atomic", scope: !13)
!13 = !DINamespace(name: "sync", scope: !14)
!14 = !DINamespace(name: "core", scope: null)
!15 = !{!16}
!16 = !DIDerivedType(tag: DW_TAG_member, name: "v", scope: !11, file: !8, baseType: !17, size: 32, align: 32, flags: DIFlagPrivate)
!17 = !DICompositeType(tag: DW_TAG_structure_type, name: "UnsafeCell<usize>", scope: !18, file: !8, size: 32, align: 32, flags: DIFlagPublic, elements: !19, templateParams: !22, identifier: "a03bd62ffdb893cb75c512d3055b1e6a")
!18 = !DINamespace(name: "cell", scope: !14)
!19 = !{!20}
!20 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !17, file: !8, baseType: !21, size: 32, align: 32, flags: DIFlagPrivate)
!21 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!22 = !{!23}
!23 = !DITemplateTypeParameter(name: "T", type: !21)
!24 = !{}
!25 = !{!26}
!26 = !DISubrange(count: 3, lowerBound: 0)
!27 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!28 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !29, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !30, splitDebugInlining: false, nameTableKind: None)
!29 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std_detect/src/lib.rs/@/std_detect.6dbc85b02342858c-cgu.0", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std_detect")
!30 = !{!0}
!31 = !{i32 7, !"Dwarf Version", i32 4}
!32 = !{i32 2, !"Debug Info Version", i32 3}
!33 = distinct !DISubprogram(name: "features", linkageName: "_ZN10std_detect6detect8features17h221ac7a82cfc441eE", scope: !3, file: !34, line: 96, type: !35, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !28, templateParams: !24)
!34 = !DIFile(filename: "src/detect/mod.rs", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std_detect", checksumkind: CSK_MD5, checksum: "dc839ad334dabe46983a90ca55d4316d")
!35 = !DISubroutineType(types: !36)
!36 = !{!37}
!37 = !DICompositeType(tag: DW_TAG_structure_type, name: "IntoIter<(&str, bool)>", scope: !38, file: !8, size: 96, align: 32, flags: DIFlagPublic, elements: !39, templateParams: !68, identifier: "f96286f1c5b96fb1996826a54524748a")
!38 = !DINamespace(name: "option", scope: !14)
!39 = !{!40}
!40 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !37, file: !8, baseType: !41, size: 96, align: 32, flags: DIFlagPrivate)
!41 = !DICompositeType(tag: DW_TAG_structure_type, name: "Item<(&str, bool)>", scope: !38, file: !8, size: 96, align: 32, flags: DIFlagPrivate, elements: !42, templateParams: !68, identifier: "c9a75232f13b83a33fbcb3ca5d10e94c")
!42 = !{!43}
!43 = !DIDerivedType(tag: DW_TAG_member, name: "opt", scope: !41, file: !8, baseType: !44, size: 96, align: 32, flags: DIFlagPrivate)
!44 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<(&str, bool)>", scope: !38, file: !8, size: 96, align: 32, flags: DIFlagPublic, elements: !45, templateParams: !24, identifier: "ac5a8e0ef286b6f6680eb13ac86c6b7d")
!45 = !{!46}
!46 = !DICompositeType(tag: DW_TAG_variant_part, scope: !44, file: !8, size: 96, align: 32, elements: !47, templateParams: !24, identifier: "fa6d0a58e154e48a3dfc3698644d2871", discriminator: !67)
!47 = !{!48, !63}
!48 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !46, file: !8, baseType: !49, size: 96, align: 32, extraData: i8 2)
!49 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !44, file: !8, size: 96, align: 32, flags: DIFlagPublic, elements: !24, templateParams: !50, identifier: "bf67612b8639361c71841ee7ef9bc380")
!50 = !{!51}
!51 = !DITemplateTypeParameter(name: "T", type: !52)
!52 = !DICompositeType(tag: DW_TAG_structure_type, name: "(&str, bool)", file: !8, size: 96, align: 32, elements: !53, templateParams: !24, identifier: "c584dc447f2aa716fb04829f6ec7311e")
!53 = !{!54, !61}
!54 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !52, file: !8, baseType: !55, size: 64, align: 32)
!55 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !8, size: 64, align: 32, elements: !56, templateParams: !24, identifier: "9277eecd40495f85161460476aacc992")
!56 = !{!57, !60}
!57 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !55, file: !8, baseType: !58, size: 32, align: 32)
!58 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !59, size: 32, align: 32, dwarfAddressSpace: 0)
!59 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!60 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !55, file: !8, baseType: !21, size: 32, align: 32, offset: 32)
!61 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !52, file: !8, baseType: !62, size: 8, align: 8, offset: 64)
!62 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!63 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !46, file: !8, baseType: !64, size: 96, align: 32)
!64 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !44, file: !8, size: 96, align: 32, flags: DIFlagPublic, elements: !65, templateParams: !50, identifier: "a7bd93f15ec60e4eed693dcd5cb9af28")
!65 = !{!66}
!66 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !64, file: !8, baseType: !52, size: 96, align: 32, flags: DIFlagPublic)
!67 = !DIDerivedType(tag: DW_TAG_member, scope: !44, file: !8, baseType: !59, size: 8, align: 8, offset: 64, flags: DIFlagArtificial)
!68 = !{!69}
!69 = !DITemplateTypeParameter(name: "A", type: !52)
!70 = !DILocation(line: 122, column: 14, scope: !33)
!71 = !DILocation(line: 122, column: 19, scope: !33)
!72 = !DILocation(line: 124, column: 2, scope: !33)
!73 = distinct !DISubprogram(name: "into_iter<(&str, bool)>", linkageName: "_ZN91_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h8dcc6161949352d6E", scope: !75, file: !74, line: 2254, type: !76, scopeLine: 2254, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !28, templateParams: !50, retainedNodes: !78)
!74 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs", directory: "", checksumkind: CSK_MD5, checksum: "8e84075e2ccbbe34be511c8d1355506d")
!75 = !DINamespace(name: "{impl#8}", scope: !38)
!76 = !DISubroutineType(types: !77)
!77 = !{!37, !44}
!78 = !{!79}
!79 = !DILocalVariable(name: "self", arg: 1, scope: !73, file: !74, line: 2254, type: !44)
!80 = !DILocation(line: 2254, column: 18, scope: !73)
!81 = !DILocation(line: 2255, column: 27, scope: !73)
!82 = !DILocation(line: 2255, column: 9, scope: !73)
!83 = !DILocation(line: 2256, column: 6, scope: !73)

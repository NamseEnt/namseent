; ModuleID = '6cj07db68eyvk7wg0ljwqor8b'
source_filename = "6cj07db68eyvk7wg0ljwqor8b"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

@alloc_3213114faf700a46436312d7d5d956d1 = private unnamed_addr constant [14 x i8] c"Hello, world!\0A", align 1
@alloc_9b968e9d68758268e4a8d45e405f65d0 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_3213114faf700a46436312d7d5d956d1, [4 x i8] c"\0E\00\00\00" }>, align 4
@vtable.0 = private constant <{ [12 x i8], ptr, ptr, ptr }> <{ [12 x i8] c"\00\00\00\00\04\00\00\00\04\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h5a384d49d70c0dc4E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h7dc4699b0a11021aE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h7dc4699b0a11021aE" }>, align 4, !dbg !0

; hello_world::main
; Function Attrs: nounwind
define hidden void @_ZN11hello_world4main17hae606c97875ba364E() unnamed_addr #0 !dbg !30 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_9b968e9d68758268e4a8d45e405f65d0) #4, !dbg !33
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h8a91acc252e41a6bE(ptr align 4 %_2) #4, !dbg !33
  ret void, !dbg !34
}

; std::rt::lang_start
; Function Attrs: nounwind
define hidden i32 @_ZN3std2rt10lang_start17h0fa245e299fdf3b8E(ptr %main, i32 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 !dbg !35 {
start:
  %sigpipe.dbg.spill = alloca [1 x i8], align 1
  %argv.dbg.spill = alloca [4 x i8], align 4
  %argc.dbg.spill = alloca [4 x i8], align 4
  %main.dbg.spill = alloca [4 x i8], align 4
  %_7 = alloca [4 x i8], align 4
  store ptr %main, ptr %main.dbg.spill, align 4
    #dbg_declare(ptr %main.dbg.spill, !44, !DIExpression(), !50)
  store i32 %argc, ptr %argc.dbg.spill, align 4
    #dbg_declare(ptr %argc.dbg.spill, !45, !DIExpression(), !51)
  store ptr %argv, ptr %argv.dbg.spill, align 4
    #dbg_declare(ptr %argv.dbg.spill, !46, !DIExpression(), !52)
  store i8 %sigpipe, ptr %sigpipe.dbg.spill, align 1
    #dbg_declare(ptr %sigpipe.dbg.spill, !47, !DIExpression(), !53)
  store ptr %main, ptr %_7, align 4, !dbg !54
; call std::rt::lang_start_internal
  %_0 = call i32 @_ZN3std2rt19lang_start_internal17ha8cc8b8233bcd6b9E(ptr align 1 %_7, ptr align 4 @vtable.0, i32 %argc, ptr %argv, i8 %sigpipe) #4, !dbg !55
  ret i32 %_0, !dbg !56
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h7dc4699b0a11021aE"(ptr align 4 %_1) unnamed_addr #1 !dbg !57 {
start:
  %_1.dbg.spill = alloca [4 x i8], align 4
  store ptr %_1, ptr %_1.dbg.spill, align 4
    #dbg_declare(ptr %_1.dbg.spill, !63, !DIExpression(DW_OP_deref), !64)
  %_4 = load ptr, ptr %_1, align 4, !dbg !65
; call std::sys::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9b84865b8448085eE(ptr %_4) #4, !dbg !66
; call <() as std::process::Termination>::report
  %_2 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfa3275beb296ac03E"() #4, !dbg !67
; call std::process::ExitCode::to_i32
  %_0 = call i32 @_ZN3std7process8ExitCode6to_i3217h26c62941ea1f0e8dE(i8 %_2) #4, !dbg !68
  ret i32 %_0, !dbg !69
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nounwind
define internal void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9b84865b8448085eE(ptr %f) unnamed_addr #2 !dbg !70 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %result.dbg.spill = alloca [0 x i8], align 1
    #dbg_declare(ptr %result.dbg.spill, !78, !DIExpression(), !82)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !77, !DIExpression(), !83)
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h1817a6f2ecfa7454E(ptr %f) #4, !dbg !84
; call core::hint::black_box
  call void @_ZN4core4hint9black_box17he36bdf5a01aa503dE() #4, !dbg !85
  ret void, !dbg !86
}

; std::process::ExitCode::to_i32
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN3std7process8ExitCode6to_i3217h26c62941ea1f0e8dE(i8 %0) unnamed_addr #1 !dbg !87 {
start:
  %self = alloca [1 x i8], align 1
  store i8 %0, ptr %self, align 1
    #dbg_declare(ptr %self, !102, !DIExpression(), !103)
; call std::sys::process::unsupported::ExitCode::as_i32
  %_0 = call i32 @_ZN3std3sys7process11unsupported8ExitCode6as_i3217hd96126fb8bad7f91E(ptr align 1 %self) #4, !dbg !104
  ret i32 %_0, !dbg !105
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h5a384d49d70c0dc4E"(ptr %_1) unnamed_addr #1 !dbg !106 {
start:
  %_1.dbg.spill = alloca [4 x i8], align 4
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 4
    #dbg_declare(ptr %_1.dbg.spill, !116, !DIExpression(), !121)
    #dbg_declare(ptr %_2, !117, !DIExpression(), !121)
  %0 = load ptr, ptr %_1, align 4, !dbg !121
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h1058c12d3ed4fb8dE(ptr %0) #4, !dbg !121
  ret i32 %_0, !dbg !121
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h1058c12d3ed4fb8dE(ptr %0) unnamed_addr #1 !dbg !122 {
start:
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [4 x i8], align 4
  store ptr %0, ptr %_1, align 4
    #dbg_declare(ptr %_1, !126, !DIExpression(), !128)
    #dbg_declare(ptr %_2, !127, !DIExpression(), !128)
; call std::rt::lang_start::{{closure}}
  %_0 = call i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h7dc4699b0a11021aE"(ptr align 4 %_1) #4, !dbg !128
  ret i32 %_0, !dbg !128
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ops8function6FnOnce9call_once17h1817a6f2ecfa7454E(ptr %_1) unnamed_addr #1 !dbg !129 {
start:
  %_1.dbg.spill = alloca [4 x i8], align 4
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 4
    #dbg_declare(ptr %_1.dbg.spill, !131, !DIExpression(), !135)
    #dbg_declare(ptr %_2, !132, !DIExpression(), !135)
  call void %_1() #4, !dbg !135
  ret void, !dbg !135
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfa3275beb296ac03E"() unnamed_addr #1 !dbg !136 {
start:
  %self.dbg.spill = alloca [0 x i8], align 1
  %_1.dbg.spill = alloca [0 x i8], align 1
    #dbg_declare(ptr %_1.dbg.spill, !142, !DIExpression(), !143)
    #dbg_declare(ptr %self.dbg.spill, !141, !DIExpression(), !143)
  ret i8 0, !dbg !144
}

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nounwind
declare dso_local void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4, ptr align 4) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: nounwind
declare dso_local void @_ZN3std2io5stdio6_print17h8a91acc252e41a6bE(ptr align 4) unnamed_addr #0

; std::rt::lang_start_internal
; Function Attrs: nounwind
declare dso_local i32 @_ZN3std2rt19lang_start_internal17ha8cc8b8233bcd6b9E(ptr align 1, ptr align 4, i32, ptr, i8) unnamed_addr #0

; core::hint::black_box
; Function Attrs: inlinehint nounwind
declare dso_local void @_ZN4core4hint9black_box17he36bdf5a01aa503dE() unnamed_addr #1

; std::sys::process::unsupported::ExitCode::as_i32
; Function Attrs: nounwind
declare dso_local i32 @_ZN3std3sys7process11unsupported8ExitCode6as_i3217hd96126fb8bad7f91E(ptr align 1) unnamed_addr #0

define i32 @__main_void() unnamed_addr #3 {
top:
; call std::rt::lang_start
  %0 = call i32 @_ZN3std2rt10lang_start17h0fa245e299fdf3b8E(ptr @_ZN11hello_world4main17hae606c97875ba364E, i32 0, ptr null, i8 0)
  ret i32 %0
}

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { inlinehint nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { noinline nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #3 = { "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #4 = { nounwind }

!llvm.ident = !{!24}
!llvm.dbg.cu = !{!25}
!llvm.module.flags = !{!28, !29}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}", file: !2, size: 192, align: 32, flags: DIFlagArtificial, elements: !4, vtableHolder: !14, templateParams: !23, identifier: "cd376088607aee46e9a7fd43f1a8881")
!4 = !{!5, !8, !10, !11, !12, !13}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !3, file: !2, baseType: !6, size: 32, align: 32)
!6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !7, size: 32, align: 32, dwarfAddressSpace: 0)
!7 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !3, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!9 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !3, file: !2, baseType: !9, size: 32, align: 32, offset: 64)
!11 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !3, file: !2, baseType: !6, size: 32, align: 32, offset: 96)
!12 = !DIDerivedType(tag: DW_TAG_member, name: "__method4", scope: !3, file: !2, baseType: !6, size: 32, align: 32, offset: 128)
!13 = !DIDerivedType(tag: DW_TAG_member, name: "__method5", scope: !3, file: !2, baseType: !6, size: 32, align: 32, offset: 160)
!14 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<()>", scope: !15, file: !2, size: 32, align: 32, elements: !18, templateParams: !23, identifier: "8eabcc21af62dfae99dea0d61abeb9e8")
!15 = !DINamespace(name: "lang_start", scope: !16)
!16 = !DINamespace(name: "rt", scope: !17)
!17 = !DINamespace(name: "std", scope: null)
!18 = !{!19}
!19 = !DIDerivedType(tag: DW_TAG_member, name: "main", scope: !14, file: !2, baseType: !20, size: 32, align: 32)
!20 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !21, size: 32, align: 32, dwarfAddressSpace: 0)
!21 = !DISubroutineType(types: !22)
!22 = !{null}
!23 = !{}
!24 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!25 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !26, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !27, splitDebugInlining: false, nameTableKind: None)
!26 = !DIFile(filename: "src/main.rs/@/6cj07db68eyvk7wg0ljwqor8b", directory: "/Users/namse/namseent/rust-to-js/sample-projects/hello-world")
!27 = !{!0}
!28 = !{i32 7, !"Dwarf Version", i32 4}
!29 = !{i32 2, !"Debug Info Version", i32 3}
!30 = distinct !DISubprogram(name: "main", linkageName: "_ZN11hello_world4main17hae606c97875ba364E", scope: !32, file: !31, line: 1, type: !21, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagMainSubprogram, unit: !25, templateParams: !23)
!31 = !DIFile(filename: "src/main.rs", directory: "/Users/namse/namseent/rust-to-js/sample-projects/hello-world", checksumkind: CSK_MD5, checksum: "b64abcb167a8b9dd4f282813c31ad0da")
!32 = !DINamespace(name: "hello_world", scope: null)
!33 = !DILocation(line: 2, column: 5, scope: !30)
!34 = !DILocation(line: 3, column: 2, scope: !30)
!35 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17h0fa245e299fdf3b8E", scope: !16, file: !36, line: 199, type: !37, scopeLine: 199, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !48, retainedNodes: !43)
!36 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "89cabe9a51031f57147bca0574d2ccca")
!37 = !DISubroutineType(types: !38)
!38 = !{!39, !20, !39, !40, !42}
!39 = !DIBasicType(name: "isize", size: 32, encoding: DW_ATE_signed)
!40 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !41, size: 32, align: 32, dwarfAddressSpace: 0)
!41 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !42, size: 32, align: 32, dwarfAddressSpace: 0)
!42 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!43 = !{!44, !45, !46, !47}
!44 = !DILocalVariable(name: "main", arg: 1, scope: !35, file: !36, line: 200, type: !20)
!45 = !DILocalVariable(name: "argc", arg: 2, scope: !35, file: !36, line: 201, type: !39)
!46 = !DILocalVariable(name: "argv", arg: 3, scope: !35, file: !36, line: 202, type: !40)
!47 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !35, file: !36, line: 203, type: !42)
!48 = !{!49}
!49 = !DITemplateTypeParameter(name: "T", type: !7)
!50 = !DILocation(line: 200, column: 5, scope: !35)
!51 = !DILocation(line: 201, column: 5, scope: !35)
!52 = !DILocation(line: 202, column: 5, scope: !35)
!53 = !DILocation(line: 203, column: 5, scope: !35)
!54 = !DILocation(line: 206, column: 10, scope: !35)
!55 = !DILocation(line: 205, column: 5, scope: !35)
!56 = !DILocation(line: 211, column: 2, scope: !35)
!57 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h7dc4699b0a11021aE", scope: !15, file: !36, line: 206, type: !58, scopeLine: 206, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !48, retainedNodes: !62)
!58 = !DISubroutineType(types: !59)
!59 = !{!60, !61}
!60 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!61 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 32, align: 32, dwarfAddressSpace: 0)
!62 = !{!63}
!63 = !DILocalVariable(name: "main", scope: !57, file: !36, line: 200, type: !20, align: 32)
!64 = !DILocation(line: 200, column: 5, scope: !57)
!65 = !DILocation(line: 206, column: 70, scope: !57)
!66 = !DILocation(line: 206, column: 18, scope: !57)
!67 = !DILocation(line: 206, column: 76, scope: !57)
!68 = !DILocation(line: 206, column: 85, scope: !57)
!69 = !DILocation(line: 206, column: 93, scope: !57)
!70 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h9b84865b8448085eE", scope: !72, file: !71, line: 154, type: !74, scopeLine: 154, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !80, retainedNodes: !76)
!71 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "e2cc8cb6b8d66d5c0e73f449e0e721de")
!72 = !DINamespace(name: "backtrace", scope: !73)
!73 = !DINamespace(name: "sys", scope: !17)
!74 = !DISubroutineType(types: !75)
!75 = !{null, !20}
!76 = !{!77, !78}
!77 = !DILocalVariable(name: "f", arg: 1, scope: !70, file: !71, line: 154, type: !20)
!78 = !DILocalVariable(name: "result", scope: !79, file: !71, line: 158, type: !7, align: 8)
!79 = distinct !DILexicalBlock(scope: !70, file: !71, line: 158, column: 5)
!80 = !{!81, !49}
!81 = !DITemplateTypeParameter(name: "F", type: !20)
!82 = !DILocation(line: 158, column: 9, scope: !79)
!83 = !DILocation(line: 154, column: 43, scope: !70)
!84 = !DILocation(line: 158, column: 18, scope: !70)
!85 = !DILocation(line: 161, column: 5, scope: !79)
!86 = !DILocation(line: 164, column: 2, scope: !70)
!87 = distinct !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217h26c62941ea1f0e8dE", scope: !89, file: !88, line: 2161, type: !98, scopeLine: 2161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !23, declaration: !100, retainedNodes: !101)
!88 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "c1d0d8af031f70a317b8c3ee91aaa453")
!89 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !90, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !91, templateParams: !23, identifier: "49b5b2f4f53a9f6fd199c399513414ad")
!90 = !DINamespace(name: "process", scope: !17)
!91 = !{!92}
!92 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !89, file: !2, baseType: !93, size: 8, align: 8, flags: DIFlagPrivate)
!93 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !94, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !96, templateParams: !23, identifier: "488c29c761b45b054ad80a00cfcb77bc")
!94 = !DINamespace(name: "unsupported", scope: !95)
!95 = !DINamespace(name: "process", scope: !73)
!96 = !{!97}
!97 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !93, file: !2, baseType: !42, size: 8, align: 8, flags: DIFlagPrivate)
!98 = !DISubroutineType(types: !99)
!99 = !{!60, !89}
!100 = !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217h26c62941ea1f0e8dE", scope: !89, file: !88, line: 2161, type: !98, scopeLine: 2161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!101 = !{!102}
!102 = !DILocalVariable(name: "self", arg: 1, scope: !87, file: !88, line: 2161, type: !89)
!103 = !DILocation(line: 2161, column: 19, scope: !87)
!104 = !DILocation(line: 2162, column: 16, scope: !87)
!105 = !DILocation(line: 2163, column: 6, scope: !87)
!106 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h5a384d49d70c0dc4E", scope: !108, file: !107, line: 250, type: !112, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !118, retainedNodes: !115)
!107 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "f10f7c44ec86506ef01d8c34efe59fc0")
!108 = !DINamespace(name: "FnOnce", scope: !109)
!109 = !DINamespace(name: "function", scope: !110)
!110 = !DINamespace(name: "ops", scope: !111)
!111 = !DINamespace(name: "core", scope: null)
!112 = !DISubroutineType(types: !113)
!113 = !{!60, !114}
!114 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 32, align: 32, dwarfAddressSpace: 0)
!115 = !{!116, !117}
!116 = !DILocalVariable(arg: 1, scope: !106, file: !107, line: 250, type: !114)
!117 = !DILocalVariable(arg: 2, scope: !106, file: !107, line: 250, type: !7)
!118 = !{!119, !120}
!119 = !DITemplateTypeParameter(name: "Self", type: !14)
!120 = !DITemplateTypeParameter(name: "Args", type: !7)
!121 = !DILocation(line: 250, column: 5, scope: !106)
!122 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h1058c12d3ed4fb8dE", scope: !108, file: !107, line: 250, type: !123, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !118, retainedNodes: !125)
!123 = !DISubroutineType(types: !124)
!124 = !{!60, !14}
!125 = !{!126, !127}
!126 = !DILocalVariable(arg: 1, scope: !122, file: !107, line: 250, type: !14)
!127 = !DILocalVariable(arg: 2, scope: !122, file: !107, line: 250, type: !7)
!128 = !DILocation(line: 250, column: 5, scope: !122)
!129 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h1817a6f2ecfa7454E", scope: !108, file: !107, line: 250, type: !74, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !133, retainedNodes: !130)
!130 = !{!131, !132}
!131 = !DILocalVariable(arg: 1, scope: !129, file: !107, line: 250, type: !20)
!132 = !DILocalVariable(arg: 2, scope: !129, file: !107, line: 250, type: !7)
!133 = !{!134, !120}
!134 = !DITemplateTypeParameter(name: "Self", type: !20)
!135 = !DILocation(line: 250, column: 5, scope: !129)
!136 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfa3275beb296ac03E", scope: !137, file: !88, line: 2559, type: !138, scopeLine: 2559, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !25, templateParams: !23, retainedNodes: !140)
!137 = !DINamespace(name: "{impl#63}", scope: !90)
!138 = !DISubroutineType(types: !139)
!139 = !{!89, !7}
!140 = !{!141, !142}
!141 = !DILocalVariable(name: "self", scope: !136, file: !88, line: 2559, type: !7, align: 8)
!142 = !DILocalVariable(arg: 1, scope: !136, file: !88, line: 2559, type: !7)
!143 = !DILocation(line: 2559, column: 15, scope: !136)
!144 = !DILocation(line: 2561, column: 6, scope: !136)

; ModuleID = 'rustc_literal_escaper.e450cde5bbe75a9f-cgu.0'
source_filename = "rustc_literal_escaper.e450cde5bbe75a9f-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

@alloc_ab7766fee1e1b94f8c00ce7ffec41c7e = private unnamed_addr constant [1 x i8] c"b", align 1
@alloc_18f2f0d3d4f38df552db2352111ec81f = private unnamed_addr constant [1 x i8] c"c", align 1

; rustc_literal_escaper::EscapeError::is_fatal
; Function Attrs: nounwind
define dso_local zeroext i1 @_ZN21rustc_literal_escaper11EscapeError8is_fatal17he3434808418726fbE(ptr align 1 %self) unnamed_addr #0 !dbg !44 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_2 = alloca [1 x i8], align 1
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !53, !DIExpression(), !54)
  %0 = load i8, ptr %self, align 1, !dbg !55
  %_3 = zext i8 %0 to i32, !dbg !55
  switch i32 %_3, label %bb1 [
    i32 21, label %bb2
    i32 22, label %bb2
  ], !dbg !56

bb1:                                              ; preds = %start
  store i8 0, ptr %_2, align 1, !dbg !56
  br label %bb3, !dbg !56

bb2:                                              ; preds = %start, %start
  store i8 1, ptr %_2, align 1, !dbg !56
  br label %bb3, !dbg !56

bb3:                                              ; preds = %bb1, %bb2
  %1 = load i8, ptr %_2, align 1, !dbg !57
  %2 = trunc nuw i8 %1 to i1, !dbg !57
  %_0 = xor i1 %2, true, !dbg !57
  ret i1 %_0, !dbg !58
}

; rustc_literal_escaper::Mode::prefix_noraw
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN21rustc_literal_escaper4Mode12prefix_noraw17h7a9c94dfeefa5aebE(i8 %self) unnamed_addr #0 !dbg !59 {
start:
  %self.dbg.spill = alloca [1 x i8], align 1
  %_0 = alloca [8 x i8], align 4
  store i8 %self, ptr %self.dbg.spill, align 1
    #dbg_declare(ptr %self.dbg.spill, !70, !DIExpression(), !71)
  %_2 = zext i8 %self to i32, !dbg !72
  switch i32 %_2, label %bb1 [
    i32 0, label %bb4
    i32 1, label %bb3
    i32 2, label %bb4
    i32 3, label %bb4
    i32 4, label %bb3
    i32 5, label %bb3
    i32 6, label %bb2
    i32 7, label %bb2
  ], !dbg !73

bb1:                                              ; preds = %start
  unreachable, !dbg !72

bb4:                                              ; preds = %start, %start, %start
  store ptr inttoptr (i32 1 to ptr), ptr %_0, align 4, !dbg !74
  %0 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !74
  store i32 0, ptr %0, align 4, !dbg !74
  br label %bb5, !dbg !74

bb3:                                              ; preds = %start, %start, %start
  store ptr @alloc_ab7766fee1e1b94f8c00ce7ffec41c7e, ptr %_0, align 4, !dbg !75
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !75
  store i32 1, ptr %1, align 4, !dbg !75
  br label %bb5, !dbg !75

bb2:                                              ; preds = %start, %start
  store ptr @alloc_18f2f0d3d4f38df552db2352111ec81f, ptr %_0, align 4, !dbg !76
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !76
  store i32 1, ptr %2, align 4, !dbg !76
  br label %bb5, !dbg !76

bb5:                                              ; preds = %bb2, %bb3, %bb4
  %3 = load ptr, ptr %_0, align 4, !dbg !77
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !77
  %5 = load i32, ptr %4, align 4, !dbg !77
  %6 = insertvalue { ptr, i32 } poison, ptr %3, 0, !dbg !77
  %7 = insertvalue { ptr, i32 } %6, i32 %5, 1, !dbg !77
  ret { ptr, i32 } %7, !dbg !77
}

; rustc_literal_escaper::Mode::in_double_quotes
; Function Attrs: nounwind
define dso_local zeroext i1 @_ZN21rustc_literal_escaper4Mode16in_double_quotes17hcbb992816246dda7E(i8 %self) unnamed_addr #0 !dbg !78 {
start:
  %self.dbg.spill = alloca [1 x i8], align 1
  %_0 = alloca [1 x i8], align 1
  store i8 %self, ptr %self.dbg.spill, align 1
    #dbg_declare(ptr %self.dbg.spill, !83, !DIExpression(), !84)
  %_2 = zext i8 %self to i32, !dbg !85
  switch i32 %_2, label %bb1 [
    i32 0, label %bb2
    i32 1, label %bb2
    i32 2, label %bb3
    i32 3, label %bb3
    i32 4, label %bb3
    i32 5, label %bb3
    i32 6, label %bb3
    i32 7, label %bb3
  ], !dbg !86

bb1:                                              ; preds = %start
  unreachable, !dbg !85

bb2:                                              ; preds = %start, %start
  store i8 0, ptr %_0, align 1, !dbg !87
  br label %bb4, !dbg !87

bb3:                                              ; preds = %start, %start, %start, %start, %start, %start
  store i8 1, ptr %_0, align 1, !dbg !88
  br label %bb4, !dbg !88

bb4:                                              ; preds = %bb3, %bb2
  %0 = load i8, ptr %_0, align 1, !dbg !89
  %1 = trunc nuw i8 %0 to i1, !dbg !89
  ret i1 %1, !dbg !89
}

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }

!llvm.ident = !{!0}
!llvm.dbg.cu = !{!1}
!llvm.module.flags = !{!42, !43}

!0 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!1 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !2, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false, nameTableKind: None)
!2 = !DIFile(filename: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rustc-literal-escaper-0.0.5/src/lib.rs/@/rustc_literal_escaper.e450cde5bbe75a9f-cgu.0", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rustc-literal-escaper-0.0.5")
!3 = !{!4, !32}
!4 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "EscapeError", scope: !6, file: !5, baseType: !7, size: 8, align: 8, flags: DIFlagEnumClass, elements: !8)
!5 = !DIFile(filename: "<unknown>", directory: "")
!6 = !DINamespace(name: "rustc_literal_escaper", scope: null)
!7 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!8 = !{!9, !10, !11, !12, !13, !14, !15, !16, !17, !18, !19, !20, !21, !22, !23, !24, !25, !26, !27, !28, !29, !30, !31}
!9 = !DIEnumerator(name: "ZeroChars", value: 0, isUnsigned: true)
!10 = !DIEnumerator(name: "MoreThanOneChar", value: 1, isUnsigned: true)
!11 = !DIEnumerator(name: "LoneSlash", value: 2, isUnsigned: true)
!12 = !DIEnumerator(name: "InvalidEscape", value: 3, isUnsigned: true)
!13 = !DIEnumerator(name: "BareCarriageReturn", value: 4, isUnsigned: true)
!14 = !DIEnumerator(name: "BareCarriageReturnInRawString", value: 5, isUnsigned: true)
!15 = !DIEnumerator(name: "EscapeOnlyChar", value: 6, isUnsigned: true)
!16 = !DIEnumerator(name: "TooShortHexEscape", value: 7, isUnsigned: true)
!17 = !DIEnumerator(name: "InvalidCharInHexEscape", value: 8, isUnsigned: true)
!18 = !DIEnumerator(name: "OutOfRangeHexEscape", value: 9, isUnsigned: true)
!19 = !DIEnumerator(name: "NoBraceInUnicodeEscape", value: 10, isUnsigned: true)
!20 = !DIEnumerator(name: "InvalidCharInUnicodeEscape", value: 11, isUnsigned: true)
!21 = !DIEnumerator(name: "EmptyUnicodeEscape", value: 12, isUnsigned: true)
!22 = !DIEnumerator(name: "UnclosedUnicodeEscape", value: 13, isUnsigned: true)
!23 = !DIEnumerator(name: "LeadingUnderscoreUnicodeEscape", value: 14, isUnsigned: true)
!24 = !DIEnumerator(name: "OverlongUnicodeEscape", value: 15, isUnsigned: true)
!25 = !DIEnumerator(name: "LoneSurrogateUnicodeEscape", value: 16, isUnsigned: true)
!26 = !DIEnumerator(name: "OutOfRangeUnicodeEscape", value: 17, isUnsigned: true)
!27 = !DIEnumerator(name: "UnicodeEscapeInByte", value: 18, isUnsigned: true)
!28 = !DIEnumerator(name: "NonAsciiCharInByte", value: 19, isUnsigned: true)
!29 = !DIEnumerator(name: "NulInCStr", value: 20, isUnsigned: true)
!30 = !DIEnumerator(name: "UnskippedWhitespaceWarning", value: 21, isUnsigned: true)
!31 = !DIEnumerator(name: "MultipleSkippedLinesWarning", value: 22, isUnsigned: true)
!32 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Mode", scope: !6, file: !5, baseType: !7, size: 8, align: 8, flags: DIFlagEnumClass, elements: !33)
!33 = !{!34, !35, !36, !37, !38, !39, !40, !41}
!34 = !DIEnumerator(name: "Char", value: 0, isUnsigned: true)
!35 = !DIEnumerator(name: "Byte", value: 1, isUnsigned: true)
!36 = !DIEnumerator(name: "Str", value: 2, isUnsigned: true)
!37 = !DIEnumerator(name: "RawStr", value: 3, isUnsigned: true)
!38 = !DIEnumerator(name: "ByteStr", value: 4, isUnsigned: true)
!39 = !DIEnumerator(name: "RawByteStr", value: 5, isUnsigned: true)
!40 = !DIEnumerator(name: "CStr", value: 6, isUnsigned: true)
!41 = !DIEnumerator(name: "RawCStr", value: 7, isUnsigned: true)
!42 = !{i32 7, !"Dwarf Version", i32 4}
!43 = !{i32 2, !"Debug Info Version", i32 3}
!44 = distinct !DISubprogram(name: "is_fatal", linkageName: "_ZN21rustc_literal_escaper11EscapeError8is_fatal17he3434808418726fbE", scope: !4, file: !45, line: 75, type: !46, scopeLine: 75, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !1, templateParams: !51, declaration: !50, retainedNodes: !52)
!45 = !DIFile(filename: "src/lib.rs", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rustc-literal-escaper-0.0.5", checksumkind: CSK_MD5, checksum: "bd1b3007760652ae04b23a81e857d162")
!46 = !DISubroutineType(types: !47)
!47 = !{!48, !49}
!48 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!49 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&rustc_literal_escaper::EscapeError", baseType: !4, size: 32, align: 32, dwarfAddressSpace: 0)
!50 = !DISubprogram(name: "is_fatal", linkageName: "_ZN21rustc_literal_escaper11EscapeError8is_fatal17he3434808418726fbE", scope: !4, file: !45, line: 75, type: !46, scopeLine: 75, flags: DIFlagPrototyped, spFlags: 0, templateParams: !51)
!51 = !{}
!52 = !{!53}
!53 = !DILocalVariable(name: "self", arg: 1, scope: !44, file: !45, line: 75, type: !49)
!54 = !DILocation(line: 75, column: 21, scope: !44)
!55 = !DILocation(line: 77, column: 13, scope: !44)
!56 = !DILocation(line: 76, column: 10, scope: !44)
!57 = !DILocation(line: 76, column: 9, scope: !44)
!58 = !DILocation(line: 80, column: 6, scope: !44)
!59 = distinct !DISubprogram(name: "prefix_noraw", linkageName: "_ZN21rustc_literal_escaper4Mode12prefix_noraw17h7a9c94dfeefa5aebE", scope: !32, file: !45, line: 635, type: !60, scopeLine: 635, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !1, templateParams: !51, declaration: !68, retainedNodes: !69)
!60 = !DISubroutineType(types: !61)
!61 = !{!62, !32}
!62 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !5, size: 64, align: 32, elements: !63, templateParams: !51, identifier: "9277eecd40495f85161460476aacc992")
!63 = !{!64, !66}
!64 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !62, file: !5, baseType: !65, size: 32, align: 32)
!65 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !7, size: 32, align: 32, dwarfAddressSpace: 0)
!66 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !62, file: !5, baseType: !67, size: 32, align: 32, offset: 32)
!67 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!68 = !DISubprogram(name: "prefix_noraw", linkageName: "_ZN21rustc_literal_escaper4Mode12prefix_noraw17h7a9c94dfeefa5aebE", scope: !32, file: !45, line: 635, type: !60, scopeLine: 635, flags: DIFlagPrototyped, spFlags: 0, templateParams: !51)
!69 = !{!70}
!70 = !DILocalVariable(name: "self", arg: 1, scope: !59, file: !45, line: 635, type: !32)
!71 = !DILocation(line: 635, column: 25, scope: !59)
!72 = !DILocation(line: 636, column: 15, scope: !59)
!73 = !DILocation(line: 636, column: 9, scope: !59)
!74 = !DILocation(line: 637, column: 54, scope: !59)
!75 = !DILocation(line: 638, column: 62, scope: !59)
!76 = !DILocation(line: 639, column: 43, scope: !59)
!77 = !DILocation(line: 641, column: 6, scope: !59)
!78 = distinct !DISubprogram(name: "in_double_quotes", linkageName: "_ZN21rustc_literal_escaper4Mode16in_double_quotes17hcbb992816246dda7E", scope: !32, file: !45, line: 623, type: !79, scopeLine: 623, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !1, templateParams: !51, declaration: !81, retainedNodes: !82)
!79 = !DISubroutineType(types: !80)
!80 = !{!48, !32}
!81 = !DISubprogram(name: "in_double_quotes", linkageName: "_ZN21rustc_literal_escaper4Mode16in_double_quotes17hcbb992816246dda7E", scope: !32, file: !45, line: 623, type: !79, scopeLine: 623, flags: DIFlagPrototyped, spFlags: 0, templateParams: !51)
!82 = !{!83}
!83 = !DILocalVariable(name: "self", arg: 1, scope: !78, file: !45, line: 623, type: !32)
!84 = !DILocation(line: 623, column: 29, scope: !78)
!85 = !DILocation(line: 624, column: 15, scope: !78)
!86 = !DILocation(line: 624, column: 9, scope: !78)
!87 = !DILocation(line: 631, column: 40, scope: !78)
!88 = !DILocation(line: 630, column: 32, scope: !78)
!89 = !DILocation(line: 633, column: 6, scope: !78)

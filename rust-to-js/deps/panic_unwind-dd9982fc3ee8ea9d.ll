; ModuleID = 'panic_unwind.68504bd31f300e9e-cgu.0'
source_filename = "panic_unwind.68504bd31f300e9e-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

@_ZN12panic_unwind3imp6CANARY17haae52ab13dfd4f85E = internal constant [1 x i8] zeroinitializer, align 1, !dbg !0
@alloc_5724e49f7be45aaf36d9cb4d4f27f68c = private unnamed_addr constant [114 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_unwind/src/gcc.rs\00", align 1
@alloc_486e0019510040af489775dc0f27fe3a = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_5724e49f7be45aaf36d9cb4d4f27f68c, [12 x i8] c"q\00\00\00`\00\00\007\00\00\00" }>, align 4
@alloc_ed8641ebea8e5515740d4eb49a916ff5 = private unnamed_addr constant [218 x i8] c"unsafe precondition(s) violated: ptr::read requires that the pointer argument is aligned and non-null\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_7f769184b15b2b9e487a8eddf4280fa4 = private unnamed_addr constant [113 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/unique.rs\00", align 1
@alloc_6e3aab891144f743e5cf3d2e7adafb34 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_7f769184b15b2b9e487a8eddf4280fa4, [12 x i8] c"p\00\00\00X\00\00\00$\00\00\00" }>, align 4
@alloc_560a59ed819b9d9a5841f6e731c4c8e5 = private unnamed_addr constant [210 x i8] c"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_fda0a185cdd8a48ed11700717a36520a = private unnamed_addr constant [115 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/alloc/layout.rs\00", align 1
@alloc_6dcff38ecac6deb22cc28f6145ba0975 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_fda0a185cdd8a48ed11700717a36520a, [12 x i8] c"r\00\00\00\E0\00\00\00\12\00\00\00" }>, align 4
@alloc_1be5ea12ba708d9a11b6e93a7d387a75 = private unnamed_addr constant [281 x i8] c"unsafe precondition(s) violated: Layout::from_size_align_unchecked requires that align is a power of 2 and the rounded-up allocation size does not exceed isize::MAX\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_dc195465c2ba08ac4896fcd3aeb3f123 = private unnamed_addr constant [109 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs\00", align 1
@alloc_88beea65e4fab49399d118295256fbde = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_dc195465c2ba08ac4896fcd3aeb3f123, [12 x i8] c"l\00\00\00_\01\00\00\1B\00\00\00" }>, align 4
@alloc_410e3011a7a5ea88f7eef411e235cc80 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_dc195465c2ba08ac4896fcd3aeb3f123, [12 x i8] c"l\00\00\00\BF\00\00\00\1B\00\00\00" }>, align 4

; __rustc::__rust_start_panic
; Function Attrs: nounwind
define dso_local i32 @_RNvCsaKOfhLrNfTz_7___rustc18___rust_start_panic(ptr align 1 %payload.0, ptr align 4 %payload.1) unnamed_addr #0 !dbg !66 {
start:
  %payload.dbg.spill3 = alloca [8 x i8], align 4
  %payload.dbg.spill = alloca [8 x i8], align 4
  store ptr %payload.0, ptr %payload.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %payload.dbg.spill, i32 4
  store ptr %payload.1, ptr %0, align 4
    #dbg_declare(ptr %payload.dbg.spill, !83, !DIExpression(), !96)
  %1 = getelementptr inbounds i8, ptr %payload.1, i32 16, !dbg !97
  %2 = load ptr, ptr %1, align 4, !dbg !97, !invariant.load !75, !nonnull !75
  %3 = call { ptr, ptr } %2(ptr align 1 %payload.0) #10, !dbg !98
  %_3.0 = extractvalue { ptr, ptr } %3, 0, !dbg !98
  %_3.1 = extractvalue { ptr, ptr } %3, 1, !dbg !98
; call alloc::boxed::Box<T>::from_raw
  %4 = call { ptr, ptr } @"_ZN5alloc5boxed12Box$LT$T$GT$8from_raw17hdae822c5d06d5e51E"(ptr %_3.0, ptr align 4 %_3.1) #10, !dbg !99
  %payload.01 = extractvalue { ptr, ptr } %4, 0, !dbg !99
  %payload.12 = extractvalue { ptr, ptr } %4, 1, !dbg !99
  store ptr %payload.01, ptr %payload.dbg.spill3, align 4, !dbg !99
  %5 = getelementptr inbounds i8, ptr %payload.dbg.spill3, i32 4, !dbg !99
  store ptr %payload.12, ptr %5, align 4, !dbg !99
    #dbg_declare(ptr %payload.dbg.spill3, !84, !DIExpression(), !100)
; call panic_unwind::imp::panic
  %_0 = call i32 @_ZN12panic_unwind3imp5panic17h07493e1a9864dee3E(ptr align 1 %payload.01, ptr align 4 %payload.12) #10, !dbg !101
  ret i32 %_0, !dbg !102
}

; __rustc::__rust_panic_cleanup
; Function Attrs: nounwind
define dso_local void @_RNvCsaKOfhLrNfTz_7___rustc20___rust_panic_cleanup(ptr sret([8 x i8]) align 4 %_0, ptr %payload) unnamed_addr #0 !dbg !103 {
start:
  %payload.dbg.spill = alloca [4 x i8], align 4
  store ptr %payload, ptr %payload.dbg.spill, align 4
    #dbg_declare(ptr %payload.dbg.spill, !112, !DIExpression(), !113)
; call panic_unwind::imp::cleanup
  %0 = call { ptr, ptr } @_ZN12panic_unwind3imp7cleanup17hd18d7d8d4f4804a6E(ptr %payload) #10, !dbg !114
  %_2.0 = extractvalue { ptr, ptr } %0, 0, !dbg !114
  %_2.1 = extractvalue { ptr, ptr } %0, 1, !dbg !114
; call alloc::boxed::Box<T>::into_raw
  %1 = call { ptr, ptr } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17hd58a19267a876565E"(ptr align 1 %_2.0, ptr align 4 %_2.1) #10, !dbg !115
  %2 = extractvalue { ptr, ptr } %1, 0, !dbg !115
  %3 = extractvalue { ptr, ptr } %1, 1, !dbg !115
  store ptr %2, ptr %_0, align 4, !dbg !115
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !115
  store ptr %3, ptr %4, align 4, !dbg !115
  ret void, !dbg !116
}

; panic_unwind::imp::panic::exception_cleanup
; Function Attrs: nounwind
define internal void @_ZN12panic_unwind3imp5panic17exception_cleanup17hecd110216a6b05d9E(i32 %_unwind_code, ptr %exception) unnamed_addr #0 !dbg !117 {
start:
  %exception.dbg.spill = alloca [4 x i8], align 4
  %_unwind_code.dbg.spill = alloca [4 x i8], align 4
  %_3 = alloca [4 x i8], align 4
  store i32 %_unwind_code, ptr %_unwind_code.dbg.spill, align 4
    #dbg_declare(ptr %_unwind_code.dbg.spill, !148, !DIExpression(), !150)
  store ptr %exception, ptr %exception.dbg.spill, align 4
    #dbg_declare(ptr %exception.dbg.spill, !149, !DIExpression(), !151)
; call alloc::boxed::Box<T>::from_raw
  %0 = call align 8 ptr @"_ZN5alloc5boxed12Box$LT$T$GT$8from_raw17h787409b512d29d17E"(ptr %exception) #10, !dbg !152
  store ptr %0, ptr %_3, align 4, !dbg !152
; call core::ptr::drop_in_place<alloc::boxed::Box<panic_unwind::imp::Exception>>
  call void @"_ZN4core3ptr74drop_in_place$LT$alloc..boxed..Box$LT$panic_unwind..imp..Exception$GT$$GT$17hd1b66519ba331a78E"(ptr align 4 %_3) #10, !dbg !153
; call __rustc::__rust_drop_panic
  call void @_RNvCsaKOfhLrNfTz_7___rustc17___rust_drop_panic() #11, !dbg !154
  unreachable, !dbg !154
}

; panic_unwind::imp::panic
; Function Attrs: nounwind
define internal i32 @_ZN12panic_unwind3imp5panic17h07493e1a9864dee3E(ptr align 1 %data.0, ptr align 4 %data.1) unnamed_addr #0 !dbg !155 {
start:
  %addr.dbg.spill.i1.i = alloca [4 x i8], align 4
  %addr.dbg.spill.i.i = alloca [4 x i8], align 4
  %exception_param.dbg.spill = alloca [4 x i8], align 4
  %exception.dbg.spill = alloca [4 x i8], align 4
  %data.dbg.spill = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  %_4 = alloca [24 x i8], align 8
  %_3 = alloca [40 x i8], align 8
  store ptr %data.0, ptr %data.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %data.dbg.spill, i32 4
  store ptr %data.1, ptr %0, align 4
    #dbg_declare(ptr %data.dbg.spill, !159, !DIExpression(), !170)
  store i32 0, ptr %addr.dbg.spill.i.i, align 4
    #dbg_declare(ptr %addr.dbg.spill.i.i, !171, !DIExpression(), !181)
  store i32 0, ptr %addr.dbg.spill.i1.i, align 4
    #dbg_declare(ptr %addr.dbg.spill.i1.i, !189, !DIExpression(), !195)
; call core::ptr::metadata::from_raw_parts
  %_0.i = call ptr @_ZN4core3ptr8metadata14from_raw_parts17h95d9158ce8c57919E(ptr null) #10, !dbg !197
  br label %repeat_loop_header, !dbg !198

repeat_loop_header:                               ; preds = %repeat_loop_body, %start
  %1 = phi i32 [ 0, %start ], [ %4, %repeat_loop_body ]
  %2 = icmp ult i32 %1, 2
  br i1 %2, label %repeat_loop_body, label %repeat_loop_next

repeat_loop_body:                                 ; preds = %repeat_loop_header
  %3 = getelementptr inbounds nuw ptr, ptr %_7, i32 %1
  store ptr %_0.i, ptr %3, align 4
  %4 = add nuw i32 %1, 1
  br label %repeat_loop_header

repeat_loop_next:                                 ; preds = %repeat_loop_header
  store i64 6076294132934528845, ptr %_4, align 8, !dbg !199
  %5 = getelementptr inbounds i8, ptr %_4, i32 8, !dbg !199
  store ptr @_ZN12panic_unwind3imp5panic17exception_cleanup17hecd110216a6b05d9E, ptr %5, align 8, !dbg !199
  %6 = getelementptr inbounds i8, ptr %_4, i32 12, !dbg !199
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %6, ptr align 4 %_7, i32 8, i1 false), !dbg !199
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %_3, ptr align 8 %_4, i32 24, i1 false), !dbg !200
  %7 = getelementptr inbounds i8, ptr %_3, i32 24, !dbg !200
  store ptr @_ZN12panic_unwind3imp6CANARY17haae52ab13dfd4f85E, ptr %7, align 8, !dbg !200
  %8 = getelementptr inbounds i8, ptr %_3, i32 28, !dbg !200
  store ptr %data.0, ptr %8, align 4, !dbg !200
  %9 = getelementptr inbounds i8, ptr %8, i32 4, !dbg !200
  store ptr %data.1, ptr %9, align 4, !dbg !200
    #dbg_declare(ptr %_3, !201, !DIExpression(), !212)
; call alloc::alloc::exchange_malloc
  %_4.i = call ptr @_ZN5alloc5alloc15exchange_malloc17h5d053562fbeb5adbE(i32 40, i32 8) #10, !dbg !214
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %_4.i, ptr align 8 %_3, i32 40, i1 false), !dbg !215
  store ptr %_4.i, ptr %exception.dbg.spill, align 4, !dbg !216
    #dbg_declare(ptr %exception.dbg.spill, !160, !DIExpression(), !217)
; call alloc::boxed::Box<T>::into_raw
  %_12 = call ptr @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h7cdb6077d8e206e4E"(ptr align 8 %_4.i) #10, !dbg !218
  store ptr %_12, ptr %exception_param.dbg.spill, align 4, !dbg !218
    #dbg_declare(ptr %exception_param.dbg.spill, !168, !DIExpression(), !219)
; call unwind::wasm::_Unwind_RaiseException
  %_13 = call i32 @_ZN6unwind4wasm22_Unwind_RaiseException17ha86401ff8c97b5afE(ptr %_12) #10, !dbg !220
  ret i32 %_13, !dbg !221
}

; panic_unwind::imp::cleanup
; Function Attrs: nounwind
define internal { ptr, ptr } @_ZN12panic_unwind3imp7cleanup17hd18d7d8d4f4804a6E(ptr %ptr) unnamed_addr #0 !dbg !222 {
start:
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %b.dbg.spill.i = alloca [4 x i8], align 4
  %a.dbg.spill.i = alloca [4 x i8], align 4
  %canary.dbg.spill = alloca [4 x i8], align 4
  %exception.dbg.spill2 = alloca [4 x i8], align 4
  %exception.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %exception = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !226, !DIExpression(), !236)
    #dbg_declare(ptr %exception, !234, !DIExpression(), !237)
  store ptr %ptr, ptr %exception.dbg.spill, align 4, !dbg !238
    #dbg_declare(ptr %exception.dbg.spill, !227, !DIExpression(), !239)
  %_4 = load i64, ptr %ptr, align 8, !dbg !240
  %_3 = icmp ne i64 %_4, 6076294132934528845, !dbg !240
  br i1 %_3, label %bb1, label %bb3, !dbg !240

bb3:                                              ; preds = %start
  store ptr %ptr, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !241, !DIExpression(), !252)
  store ptr %ptr, ptr %exception.dbg.spill2, align 4, !dbg !254
    #dbg_declare(ptr %exception.dbg.spill2, !229, !DIExpression(), !255)
  %_9 = getelementptr inbounds i8, ptr %ptr, i32 24, !dbg !256
; call core::ptr::const_ptr::<impl *const T>::read
  %canary = call ptr @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h6763137c6ca81c4dE"(ptr %_9, ptr align 4 @alloc_486e0019510040af489775dc0f27fe3a) #10, !dbg !257
  store ptr %canary, ptr %canary.dbg.spill, align 4, !dbg !257
    #dbg_declare(ptr %canary.dbg.spill, !232, !DIExpression(), !258)
  store ptr %canary, ptr %a.dbg.spill.i, align 4
    #dbg_declare(ptr %a.dbg.spill.i, !259, !DIExpression(), !266)
  store ptr @_ZN12panic_unwind3imp6CANARY17haae52ab13dfd4f85E, ptr %b.dbg.spill.i, align 4
    #dbg_declare(ptr %b.dbg.spill.i, !265, !DIExpression(), !268)
  %_0.i = icmp eq ptr %canary, @_ZN12panic_unwind3imp6CANARY17haae52ab13dfd4f85E, !dbg !269
  br i1 %_0.i, label %bb7, label %bb8, !dbg !270

bb1:                                              ; preds = %start
; call unwind::wasm::_Unwind_DeleteException
  call void @_ZN6unwind4wasm23_Unwind_DeleteException17h681e99c9e311f412E(ptr %ptr) #10, !dbg !271
; call __rustc::__rust_foreign_exception
  call void @_RNvCsaKOfhLrNfTz_7___rustc24___rust_foreign_exception() #11, !dbg !272
  unreachable, !dbg !272

bb8:                                              ; preds = %bb3
; call __rustc::__rust_foreign_exception
  call void @_RNvCsaKOfhLrNfTz_7___rustc24___rust_foreign_exception() #11, !dbg !273
  unreachable, !dbg !273

bb7:                                              ; preds = %bb3
; call alloc::boxed::Box<T>::from_raw
  %0 = call align 8 ptr @"_ZN5alloc5boxed12Box$LT$T$GT$8from_raw17h787409b512d29d17E"(ptr %ptr) #10, !dbg !274
  store ptr %0, ptr %exception, align 4, !dbg !274
  %_17 = load ptr, ptr %exception, align 4, !dbg !275
  %1 = getelementptr inbounds i8, ptr %_17, i32 28, !dbg !275
  %_0.0 = load ptr, ptr %1, align 4, !dbg !275
  %2 = getelementptr inbounds i8, ptr %1, i32 4, !dbg !275
  %_0.1 = load ptr, ptr %2, align 4, !dbg !275
; call <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9f990ada96c0379fE"(ptr align 4 %exception) #10, !dbg !276
  %3 = insertvalue { ptr, ptr } poison, ptr %_0.0, 0, !dbg !277
  %4 = insertvalue { ptr, ptr } %3, ptr %_0.1, 1, !dbg !277
  ret { ptr, ptr } %4, !dbg !277
}

; core::mem::size_of_val_raw
; Function Attrs: inlinehint nounwind
define dso_local i32 @_ZN4core3mem15size_of_val_raw17hd84cecf7ccb4d68fE(ptr %val) unnamed_addr #1 !dbg !278 {
start:
  %0 = alloca [4 x i8], align 4
  %val.dbg.spill = alloca [4 x i8], align 4
  store ptr %val, ptr %val.dbg.spill, align 4
    #dbg_declare(ptr %val.dbg.spill, !285, !DIExpression(), !286)
  store i32 40, ptr %0, align 4, !dbg !287
  %_0 = load i32, ptr %0, align 4, !dbg !287
  ret i32 %_0, !dbg !288
}

; core::mem::size_of_val_raw
; Function Attrs: inlinehint nounwind
define dso_local i32 @_ZN4core3mem15size_of_val_raw17hd873f665e55e7c88E(ptr %val.0, ptr align 4 %val.1) unnamed_addr #1 !dbg !289 {
start:
  %0 = alloca [4 x i8], align 4
  %val.dbg.spill = alloca [8 x i8], align 4
  store ptr %val.0, ptr %val.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %val.dbg.spill, i32 4
  store ptr %val.1, ptr %1, align 4
    #dbg_declare(ptr %val.dbg.spill, !297, !DIExpression(), !300)
  %2 = getelementptr inbounds i8, ptr %val.1, i32 4, !dbg !301
  %3 = load i32, ptr %2, align 4, !dbg !301, !invariant.load !75
  %4 = getelementptr inbounds i8, ptr %val.1, i32 8, !dbg !301
  %5 = load i32, ptr %4, align 4, !dbg !301, !invariant.load !75
  store i32 %3, ptr %0, align 4, !dbg !301
  %_0 = load i32, ptr %0, align 4, !dbg !301
  ret i32 %_0, !dbg !302
}

; core::mem::align_of_val_raw
; Function Attrs: inlinehint nounwind
define dso_local i32 @_ZN4core3mem16align_of_val_raw17h18382815f45778c9E(ptr %val.0, ptr align 4 %val.1) unnamed_addr #1 !dbg !303 {
start:
  %0 = alloca [4 x i8], align 4
  %val.dbg.spill = alloca [8 x i8], align 4
  store ptr %val.0, ptr %val.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %val.dbg.spill, i32 4
  store ptr %val.1, ptr %1, align 4
    #dbg_declare(ptr %val.dbg.spill, !305, !DIExpression(), !306)
  %2 = getelementptr inbounds i8, ptr %val.1, i32 4, !dbg !307
  %3 = load i32, ptr %2, align 4, !dbg !307, !invariant.load !75
  %4 = getelementptr inbounds i8, ptr %val.1, i32 8, !dbg !307
  %5 = load i32, ptr %4, align 4, !dbg !307, !invariant.load !75
  store i32 %5, ptr %0, align 4, !dbg !307
  %_0 = load i32, ptr %0, align 4, !dbg !307
  ret i32 %_0, !dbg !308
}

; core::mem::align_of_val_raw
; Function Attrs: inlinehint nounwind
define dso_local i32 @_ZN4core3mem16align_of_val_raw17h29d52bec7b897efdE(ptr %val) unnamed_addr #1 !dbg !309 {
start:
  %0 = alloca [4 x i8], align 4
  %val.dbg.spill = alloca [4 x i8], align 4
  store ptr %val, ptr %val.dbg.spill, align 4
    #dbg_declare(ptr %val.dbg.spill, !311, !DIExpression(), !312)
  store i32 8, ptr %0, align 4, !dbg !313
  %_0 = load i32, ptr %0, align 4, !dbg !313
  ret i32 %_0, !dbg !314
}

; core::ptr::drop_in_place<panic_unwind::imp::Exception>
; Function Attrs: nounwind
define dso_local void @"_ZN4core3ptr49drop_in_place$LT$panic_unwind..imp..Exception$GT$17h3a1897ef021f70cfE"(ptr align 8 %_1) unnamed_addr #0 !dbg !315 {
start:
  %_1.dbg.spill = alloca [4 x i8], align 4
  store ptr %_1, ptr %_1.dbg.spill, align 4
    #dbg_declare(ptr %_1.dbg.spill, !319, !DIExpression(), !320)
  %0 = getelementptr inbounds i8, ptr %_1, i32 28, !dbg !320
; call core::ptr::drop_in_place<alloc::boxed::Box<dyn core::any::Any+core::marker::Send>>
  call void @"_ZN4core3ptr91drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$$GT$17h4fd15b138eb3408dE"(ptr align 4 %0) #10, !dbg !320
  ret void, !dbg !320
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local ptr @_ZN4core3ptr4read17hcd1815b95cd75f77E(ptr %src, ptr align 4 %0) unnamed_addr #1 !dbg !321 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !352, !DIExpression(), !355)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h7d3209090a219533E() #10, !dbg !356
  br i1 %_2, label %bb2, label %bb4, !dbg !356

bb4:                                              ; preds = %bb2, %start
  %_0 = load ptr, ptr %src, align 4, !dbg !359
  ret ptr %_0, !dbg !360

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17h1c62c99544682c14E(ptr %src, i32 4, i1 zeroext false, ptr align 4 %0) #10, !dbg !361
  br label %bb4, !dbg !361
}

; core::ptr::read::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr4read18precondition_check17h1c62c99544682c14E(ptr %addr, i32 %align, i1 zeroext %is_zst, ptr align 4 %0) unnamed_addr #1 !dbg !362 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %is_zst.dbg.spill = alloca [1 x i8], align 1
  %align.dbg.spill = alloca [4 x i8], align 4
  %addr.dbg.spill = alloca [4 x i8], align 4
  %_8 = alloca [8 x i8], align 4
  %_6 = alloca [24 x i8], align 4
  store ptr %addr, ptr %addr.dbg.spill, align 4
    #dbg_declare(ptr %addr.dbg.spill, !367, !DIExpression(), !372)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !368, !DIExpression(), !372)
  %1 = zext i1 %is_zst to i8
  store i8 %1, ptr %is_zst.dbg.spill, align 1
    #dbg_declare(ptr %is_zst.dbg.spill, !369, !DIExpression(), !372)
  store ptr @alloc_ed8641ebea8e5515740d4eb49a916ff5, ptr %msg.dbg.spill, align 4, !dbg !373
  %2 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !373
  store i32 218, ptr %2, align 4, !dbg !373
    #dbg_declare(ptr %msg.dbg.spill, !370, !DIExpression(), !373)
; call core::ub_checks::maybe_is_aligned_and_not_null
  %_4 = call zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17ha3a1dc64a976744dE(ptr %addr, i32 %align, i1 zeroext %is_zst) #10, !dbg !374
  br i1 %_4, label %bb2, label %bb3, !dbg !374

bb3:                                              ; preds = %start
  %3 = getelementptr inbounds nuw { ptr, i32 }, ptr %_8, i32 0, !dbg !376
  store ptr @alloc_ed8641ebea8e5515740d4eb49a916ff5, ptr %3, align 4, !dbg !376
  %4 = getelementptr inbounds i8, ptr %3, i32 4, !dbg !376
  store i32 218, ptr %4, align 4, !dbg !376
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_6, ptr align 4 %_8) #10, !dbg !377
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h185a0c9c09e8b976E(ptr align 4 %_6, i1 zeroext false, ptr align 4 %0) #11, !dbg !378
  unreachable, !dbg !378

bb2:                                              ; preds = %start
  ret void, !dbg !379
}

; core::ptr::drop_in_place<dyn core::any::Any+core::marker::Send>
; Function Attrs: nounwind
define dso_local void @"_ZN4core3ptr66drop_in_place$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$17h9c94b09f3be9fe7bE"(ptr align 1 %_1.0, ptr align 4 %_1.1) unnamed_addr #0 !dbg !380 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 4
  store ptr %_1.0, ptr %_1.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %_1.dbg.spill, i32 4
  store ptr %_1.1, ptr %0, align 4
    #dbg_declare(ptr %_1.dbg.spill, !384, !DIExpression(), !385)
  %1 = getelementptr inbounds i8, ptr %_1.1, i32 0, !dbg !385
  %2 = load ptr, ptr %1, align 4, !dbg !385, !invariant.load !75
  %3 = icmp ne ptr %2, null, !dbg !385
  br i1 %3, label %is_not_null, label %bb1, !dbg !385

is_not_null:                                      ; preds = %start
  call void %2(ptr %_1.0) #10, !dbg !385
  br label %bb1, !dbg !385

bb1:                                              ; preds = %is_not_null, %start
  ret void, !dbg !385
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha82630dde1c4b48aE"(ptr %ptr) unnamed_addr #1 !dbg !386 {
start:
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !401, !DIExpression(), !402)
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %_2 = call ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h1b87b4c3f1268a7fE"(ptr %ptr, ptr align 4 @alloc_6e3aab891144f743e5cf3d2e7adafb34) #10, !dbg !403
  ret ptr %_2, !dbg !404
}

; core::ptr::unique::Unique<T>::new_unchecked
; Function Attrs: inlinehint nounwind
define dso_local { ptr, ptr } @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hf8bbe7c74bad6648E"(ptr %ptr.0, ptr align 4 %ptr.1) unnamed_addr #1 !dbg !405 {
start:
  %ptr.dbg.spill = alloca [8 x i8], align 4
  store ptr %ptr.0, ptr %ptr.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %ptr.dbg.spill, i32 4
  store ptr %ptr.1, ptr %0, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !418, !DIExpression(), !419)
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %1 = call { ptr, ptr } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h23d448e994bed79aE"(ptr %ptr.0, ptr align 4 %ptr.1, ptr align 4 @alloc_6e3aab891144f743e5cf3d2e7adafb34) #10, !dbg !420
  %_2.0 = extractvalue { ptr, ptr } %1, 0, !dbg !420
  %_2.1 = extractvalue { ptr, ptr } %1, 1, !dbg !420
  %2 = insertvalue { ptr, ptr } poison, ptr %_2.0, 0, !dbg !421
  %3 = insertvalue { ptr, ptr } %2, ptr %_2.1, 1, !dbg !421
  ret { ptr, ptr } %3, !dbg !421
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h879afa9afd8f0935E"(ptr %self.0, ptr align 4 %self.1) unnamed_addr #1 !dbg !422 {
start:
  %self.dbg.spill = alloca [8 x i8], align 4
  store ptr %self.0, ptr %self.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %self.dbg.spill, i32 4
  store ptr %self.1, ptr %0, align 4
    #dbg_declare(ptr %self.dbg.spill, !437, !DIExpression(), !438)
; call core::ptr::non_null::NonNull<T>::cast
  %_2 = call ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h0b54b3148c34d783E"(ptr %self.0, ptr align 4 %self.1) #10, !dbg !439
  ret ptr %_2, !dbg !440
}

; core::ptr::unique::Unique<T>::cast
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hf99e8766b267ebecE"(ptr %self) unnamed_addr #1 !dbg !441 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !447, !DIExpression(), !448)
; call core::ptr::non_null::NonNull<T>::cast
  %_2 = call ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h3d4f9fa96fc7113bE"(ptr %self) #10, !dbg !449
  ret ptr %_2, !dbg !450
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h38473301210de45fE"(ptr %self) unnamed_addr #1 !dbg !451 {
start:
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !456, !DIExpression(), !457)
  store ptr %self, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !458, !DIExpression(), !465)
  ret ptr %self, !dbg !467
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint nounwind
define dso_local { ptr, ptr } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17he3585bd48d4ed619E"(ptr %self.0, ptr align 4 %self.1) unnamed_addr #1 !dbg !468 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %self.dbg.spill = alloca [8 x i8], align 4
  store ptr %self.0, ptr %self.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %self.dbg.spill, i32 4
  store ptr %self.1, ptr %0, align 4
    #dbg_declare(ptr %self.dbg.spill, !473, !DIExpression(), !474)
  store ptr %self.0, ptr %self.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store ptr %self.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !475, !DIExpression(), !481)
  %2 = insertvalue { ptr, ptr } poison, ptr %self.0, 0, !dbg !483
  %3 = insertvalue { ptr, ptr } %2, ptr %self.1, 1, !dbg !483
  %_0.0 = extractvalue { ptr, ptr } %3, 0, !dbg !484
  %_0.1 = extractvalue { ptr, ptr } %3, 1, !dbg !484
  %4 = insertvalue { ptr, ptr } poison, ptr %_0.0, 0, !dbg !485
  %5 = insertvalue { ptr, ptr } %4, ptr %_0.1, 1, !dbg !485
  ret { ptr, ptr } %5, !dbg !485
}

; core::ptr::drop_in_place<alloc::boxed::Box<panic_unwind::imp::Exception>>
; Function Attrs: nounwind
define dso_local void @"_ZN4core3ptr74drop_in_place$LT$alloc..boxed..Box$LT$panic_unwind..imp..Exception$GT$$GT$17hd1b66519ba331a78E"(ptr align 4 %_1) unnamed_addr #0 !dbg !486 {
start:
  %_1.dbg.spill = alloca [4 x i8], align 4
  store ptr %_1, ptr %_1.dbg.spill, align 4
    #dbg_declare(ptr %_1.dbg.spill, !491, !DIExpression(), !494)
  %_6 = load ptr, ptr %_1, align 4, !dbg !494
; call core::ptr::drop_in_place<panic_unwind::imp::Exception>
  call void @"_ZN4core3ptr49drop_in_place$LT$panic_unwind..imp..Exception$GT$17h3a1897ef021f70cfE"(ptr align 8 %_6) #10, !dbg !494
; call <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9f990ada96c0379fE"(ptr align 4 %_1) #10, !dbg !494
  ret void, !dbg !494
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h1b87b4c3f1268a7fE"(ptr %ptr, ptr align 4 %0) unnamed_addr #1 !dbg !495 {
start:
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !500, !DIExpression(), !501)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h7d3209090a219533E() #10, !dbg !502
  br i1 %_2, label %bb2, label %bb3, !dbg !502

bb3:                                              ; preds = %bb2, %start
  ret ptr %ptr, !dbg !504

bb2:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h2ebe651468dffb0fE"(ptr %ptr, ptr align 4 %0) #10, !dbg !505
  br label %bb3, !dbg !505
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint nounwind
define dso_local { ptr, ptr } @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h23d448e994bed79aE"(ptr %ptr.0, ptr align 4 %ptr.1, ptr align 4 %0) unnamed_addr #1 !dbg !506 {
start:
  %ptr.dbg.spill = alloca [8 x i8], align 4
  store ptr %ptr.0, ptr %ptr.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %ptr.dbg.spill, i32 4
  store ptr %ptr.1, ptr %1, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !511, !DIExpression(), !512)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h7d3209090a219533E() #10, !dbg !513
  br i1 %_2, label %bb2, label %bb3, !dbg !513

bb3:                                              ; preds = %bb2, %start
  %2 = insertvalue { ptr, ptr } poison, ptr %ptr.0, 0, !dbg !515
  %3 = insertvalue { ptr, ptr } %2, ptr %ptr.1, 1, !dbg !515
  ret { ptr, ptr } %3, !dbg !515

bb2:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h2ebe651468dffb0fE"(ptr %ptr.0, ptr align 4 %0) #10, !dbg !516
  br label %bb3, !dbg !516
}

; core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h2ebe651468dffb0fE"(ptr %ptr, ptr align 4 %0) unnamed_addr #1 !dbg !517 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %_6 = alloca [8 x i8], align 4
  %_4 = alloca [24 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !523, !DIExpression(), !526)
  store ptr @alloc_560a59ed819b9d9a5841f6e731c4c8e5, ptr %msg.dbg.spill, align 4, !dbg !527
  %1 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !527
  store i32 210, ptr %1, align 4, !dbg !527
    #dbg_declare(ptr %msg.dbg.spill, !524, !DIExpression(), !527)
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_2 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hf42c4102b4c84c5dE"(ptr %ptr) #10, !dbg !528
  br i1 %_2, label %bb2, label %bb3, !dbg !530

bb3:                                              ; preds = %start
  ret void, !dbg !531

bb2:                                              ; preds = %start
  %2 = getelementptr inbounds nuw { ptr, i32 }, ptr %_6, i32 0, !dbg !532
  store ptr @alloc_560a59ed819b9d9a5841f6e731c4c8e5, ptr %2, align 4, !dbg !532
  %3 = getelementptr inbounds i8, ptr %2, i32 4, !dbg !532
  store i32 210, ptr %3, align 4, !dbg !532
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_4, ptr align 4 %_6) #10, !dbg !533
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h185a0c9c09e8b976E(ptr align 4 %_4, i1 zeroext false, ptr align 4 %0) #11, !dbg !534
  unreachable, !dbg !534
}

; core::ptr::non_null::NonNull<T>::cast
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h0b54b3148c34d783E"(ptr %self.0, ptr align 4 %self.1) unnamed_addr #1 !dbg !535 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %self.dbg.spill = alloca [8 x i8], align 4
  store ptr %self.0, ptr %self.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %self.dbg.spill, i32 4
  store ptr %self.1, ptr %0, align 4
    #dbg_declare(ptr %self.dbg.spill, !540, !DIExpression(), !541)
  store ptr %self.0, ptr %self.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store ptr %self.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !475, !DIExpression(), !542)
  %2 = insertvalue { ptr, ptr } poison, ptr %self.0, 0, !dbg !544
  %3 = insertvalue { ptr, ptr } %2, ptr %self.1, 1, !dbg !544
  %_4.0 = extractvalue { ptr, ptr } %3, 0, !dbg !545
  %_4.1 = extractvalue { ptr, ptr } %3, 1, !dbg !545
  ret ptr %_4.0, !dbg !546
}

; core::ptr::non_null::NonNull<T>::cast
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h3d4f9fa96fc7113bE"(ptr %self) unnamed_addr #1 !dbg !547 {
start:
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !552, !DIExpression(), !553)
  store ptr %self, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !458, !DIExpression(), !554)
  ret ptr %self, !dbg !556
}

; core::ptr::drop_in_place<alloc::boxed::Box<dyn core::any::Any+core::marker::Send>>
; Function Attrs: nounwind
define dso_local void @"_ZN4core3ptr91drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$$GT$17h4fd15b138eb3408dE"(ptr align 4 %_1) unnamed_addr #0 !dbg !557 {
start:
  %_1.dbg.spill = alloca [4 x i8], align 4
  store ptr %_1, ptr %_1.dbg.spill, align 4
    #dbg_declare(ptr %_1.dbg.spill, !562, !DIExpression(), !565)
  %_6.0 = load ptr, ptr %_1, align 4, !dbg !565
  %0 = getelementptr inbounds i8, ptr %_1, i32 4, !dbg !565
  %_6.1 = load ptr, ptr %0, align 4, !dbg !565
  %1 = getelementptr inbounds i8, ptr %_6.1, i32 0, !dbg !565
  %2 = load ptr, ptr %1, align 4, !dbg !565, !invariant.load !75
  %3 = icmp ne ptr %2, null, !dbg !565
  br i1 %3, label %is_not_null, label %bb2, !dbg !565

is_not_null:                                      ; preds = %start
  call void %2(ptr %_6.0) #10, !dbg !565
  br label %bb2, !dbg !565

bb2:                                              ; preds = %is_not_null, %start
; call <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfcf1c84a7897640aE"(ptr align 4 %_1) #10, !dbg !565
  ret void, !dbg !565
}

; core::ptr::alignment::Alignment::as_nonzero
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core3ptr9alignment9Alignment10as_nonzero17h031ac1317a479a71E(i32 %self) unnamed_addr #1 !dbg !566 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store i32 %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !586, !DIExpression(), !587)
  ret i32 %self, !dbg !588
}

; core::ptr::alignment::Alignment::as_usize
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core3ptr9alignment9Alignment8as_usize17hb36d330e2670920aE(i32 %self) unnamed_addr #1 !dbg !589 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store i32 %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !594, !DIExpression(), !595)
  ret i32 %self, !dbg !596
}

; core::ptr::const_ptr::<impl *const T>::read
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h6763137c6ca81c4dE"(ptr %self, ptr align 4 %0) unnamed_addr #1 !dbg !597 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !602, !DIExpression(), !603)
; call core::ptr::read
  %_0 = call ptr @_ZN4core3ptr4read17hcd1815b95cd75f77E(ptr %self, ptr align 4 %0) #10, !dbg !604
  ret ptr %_0, !dbg !605
}

; core::alloc::layout::Layout::for_value_raw
; Function Attrs: nounwind
define dso_local { i32, i32 } @_ZN4core5alloc6layout6Layout13for_value_raw17h41fcf0e2c2b6f065E(ptr %t.0, ptr align 4 %t.1) unnamed_addr #0 !dbg !606 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %t.dbg.spill = alloca [8 x i8], align 4
  store ptr %t.0, ptr %t.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %t.dbg.spill, i32 4
  store ptr %t.1, ptr %0, align 4
    #dbg_declare(ptr %t.dbg.spill, !618, !DIExpression(), !622)
; call core::mem::size_of_val_raw
  %_5 = call i32 @_ZN4core3mem15size_of_val_raw17hd873f665e55e7c88E(ptr %t.0, ptr align 4 %t.1) #10, !dbg !623
; call core::mem::align_of_val_raw
  %_6 = call i32 @_ZN4core3mem16align_of_val_raw17h18382815f45778c9E(ptr %t.0, ptr align 4 %t.1) #10, !dbg !624
  store i32 %_5, ptr %size.dbg.spill, align 4, !dbg !625
    #dbg_declare(ptr %size.dbg.spill, !619, !DIExpression(), !626)
  store i32 %_6, ptr %align.dbg.spill, align 4, !dbg !627
    #dbg_declare(ptr %align.dbg.spill, !621, !DIExpression(), !628)
; call core::alloc::layout::Layout::from_size_align_unchecked
  %1 = call { i32, i32 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h3599e2620243ac2fE(i32 %_5, i32 %_6, ptr align 4 @alloc_6dcff38ecac6deb22cc28f6145ba0975) #10, !dbg !629
  %_0.0 = extractvalue { i32, i32 } %1, 0, !dbg !629
  %_0.1 = extractvalue { i32, i32 } %1, 1, !dbg !629
  %2 = insertvalue { i32, i32 } poison, i32 %_0.0, 0, !dbg !630
  %3 = insertvalue { i32, i32 } %2, i32 %_0.1, 1, !dbg !630
  ret { i32, i32 } %3, !dbg !630
}

; core::alloc::layout::Layout::for_value_raw
; Function Attrs: nounwind
define dso_local { i32, i32 } @_ZN4core5alloc6layout6Layout13for_value_raw17hb269eae829dfffa3E(ptr %t) unnamed_addr #0 !dbg !631 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %t.dbg.spill = alloca [4 x i8], align 4
  store ptr %t, ptr %t.dbg.spill, align 4
    #dbg_declare(ptr %t.dbg.spill, !636, !DIExpression(), !640)
; call core::mem::size_of_val_raw
  %_5 = call i32 @_ZN4core3mem15size_of_val_raw17hd84cecf7ccb4d68fE(ptr %t) #10, !dbg !641
; call core::mem::align_of_val_raw
  %_6 = call i32 @_ZN4core3mem16align_of_val_raw17h29d52bec7b897efdE(ptr %t) #10, !dbg !642
  store i32 %_5, ptr %size.dbg.spill, align 4, !dbg !643
    #dbg_declare(ptr %size.dbg.spill, !637, !DIExpression(), !644)
  store i32 %_6, ptr %align.dbg.spill, align 4, !dbg !645
    #dbg_declare(ptr %align.dbg.spill, !639, !DIExpression(), !646)
; call core::alloc::layout::Layout::from_size_align_unchecked
  %0 = call { i32, i32 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h3599e2620243ac2fE(i32 %_5, i32 %_6, ptr align 4 @alloc_6dcff38ecac6deb22cc28f6145ba0975) #10, !dbg !647
  %_0.0 = extractvalue { i32, i32 } %0, 0, !dbg !647
  %_0.1 = extractvalue { i32, i32 } %0, 1, !dbg !647
  %1 = insertvalue { i32, i32 } poison, i32 %_0.0, 0, !dbg !648
  %2 = insertvalue { i32, i32 } %1, i32 %_0.1, 1, !dbg !648
  ret { i32, i32 } %2, !dbg !648
}

; core::alloc::layout::Layout::from_size_align_unchecked
; Function Attrs: inlinehint nounwind
define internal { i32, i32 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h3599e2620243ac2fE(i32 %size, i32 %align, ptr align 4 %0) unnamed_addr #1 !dbg !649 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !654, !DIExpression(), !656)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !655, !DIExpression(), !657)
  br label %bb1, !dbg !658

bb1:                                              ; preds = %start
; call core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
  call void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17h5a8e2c1454664ab4E(i32 %size, i32 %align, ptr align 4 %0) #10, !dbg !660
  br label %bb2, !dbg !660

bb2:                                              ; preds = %bb1
  %1 = insertvalue { i32, i32 } poison, i32 %align, 0, !dbg !661
  %2 = insertvalue { i32, i32 } %1, i32 %size, 1, !dbg !661
  ret { i32, i32 } %2, !dbg !661
}

; core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17h5a8e2c1454664ab4E(i32 %size, i32 %align, ptr align 4 %0) unnamed_addr #1 !dbg !662 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %align.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  %_5 = alloca [24 x i8], align 4
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !668, !DIExpression(), !672)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !669, !DIExpression(), !672)
  store ptr @alloc_1be5ea12ba708d9a11b6e93a7d387a75, ptr %msg.dbg.spill, align 4, !dbg !673
  %1 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !673
  store i32 281, ptr %1, align 4, !dbg !673
    #dbg_declare(ptr %msg.dbg.spill, !670, !DIExpression(), !673)
; call core::alloc::layout::Layout::is_size_align_valid
  %_3 = call zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h77ec66f10f926786E(i32 %size, i32 %align) #10, !dbg !674
  br i1 %_3, label %bb2, label %bb3, !dbg !674

bb3:                                              ; preds = %start
  %2 = getelementptr inbounds nuw { ptr, i32 }, ptr %_7, i32 0, !dbg !676
  store ptr @alloc_1be5ea12ba708d9a11b6e93a7d387a75, ptr %2, align 4, !dbg !676
  %3 = getelementptr inbounds i8, ptr %2, i32 4, !dbg !676
  store i32 281, ptr %3, align 4, !dbg !676
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_5, ptr align 4 %_7) #10, !dbg !677
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h185a0c9c09e8b976E(ptr align 4 %_5, i1 zeroext false, ptr align 4 %0) #11, !dbg !678
  unreachable, !dbg !678

bb2:                                              ; preds = %start
  ret void, !dbg !679
}

; core::alloc::layout::Layout::size
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %self) unnamed_addr #1 !dbg !680 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !686, !DIExpression(), !687)
  %0 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !688
  %_0 = load i32, ptr %0, align 4, !dbg !688
  ret i32 %_0, !dbg !689
}

; core::alloc::layout::Layout::align
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core5alloc6layout6Layout5align17h8d2f91b08a22bceaE(ptr align 4 %self) unnamed_addr #1 !dbg !690 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !693, !DIExpression(), !694)
  %_2 = load i32, ptr %self, align 4, !dbg !695
; call core::ptr::alignment::Alignment::as_usize
  %_0 = call i32 @_ZN4core3ptr9alignment9Alignment8as_usize17hb36d330e2670920aE(i32 %_2) #10, !dbg !696
  ret i32 %_0, !dbg !697
}

; core::alloc::layout::Layout::dangling
; Function Attrs: inlinehint nounwind
define internal ptr @_ZN4core5alloc6layout6Layout8dangling17h1c98afe0c3073c3dE(ptr align 4 %self) unnamed_addr #1 !dbg !698 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !703, !DIExpression(), !704)
  %_3 = load i32, ptr %self, align 4, !dbg !705
; call core::ptr::alignment::Alignment::as_nonzero
  %_2 = call i32 @_ZN4core3ptr9alignment9Alignment10as_nonzero17h031ac1317a479a71E(i32 %_3) #10, !dbg !706
; call core::ptr::non_null::NonNull<T>::without_provenance
  %_0 = call ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$18without_provenance17hb3cf249ff93074e0E"(i32 %_2) #10, !dbg !707
  ret ptr %_0, !dbg !708
}

; core::panicking::panic_nounwind_fmt
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking18panic_nounwind_fmt17h185a0c9c09e8b976E(ptr align 4 %fmt, i1 zeroext %force_no_backtrace, ptr align 4 %0) unnamed_addr #2 !dbg !709 {
start:
  %force_no_backtrace.dbg.spill = alloca [1 x i8], align 1
  %_3 = alloca [28 x i8], align 4
    #dbg_declare(ptr %fmt, !837, !DIExpression(), !839)
  %1 = zext i1 %force_no_backtrace to i8
  store i8 %1, ptr %force_no_backtrace.dbg.spill, align 1
    #dbg_declare(ptr %force_no_backtrace.dbg.spill, !838, !DIExpression(), !840)
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_3, ptr align 4 %fmt, i32 24, i1 false), !dbg !841
  %2 = getelementptr inbounds i8, ptr %_3, i32 24, !dbg !841
  %3 = zext i1 %force_no_backtrace to i8, !dbg !841
  store i8 %3, ptr %2, align 4, !dbg !841
  %4 = getelementptr inbounds i8, ptr %_3, i32 24, !dbg !844
  %5 = load i8, ptr %4, align 4, !dbg !844
  %6 = trunc nuw i8 %5 to i1, !dbg !844
; call core::panicking::panic_nounwind_fmt::runtime
  call void @_ZN4core9panicking18panic_nounwind_fmt7runtime17haa28c113f74a5518E(ptr align 4 %_3, i1 zeroext %6, ptr align 4 %0) #11, !dbg !844
  unreachable, !dbg !844
}

; core::panicking::panic_nounwind_fmt::runtime
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking18panic_nounwind_fmt7runtime17haa28c113f74a5518E(ptr align 4 %fmt, i1 zeroext %force_no_backtrace, ptr align 4 %0) unnamed_addr #2 !dbg !845 {
start:
    #dbg_declare(ptr %fmt, !848, !DIExpression(), !860)
  %force_no_backtrace.dbg.spill = alloca [1 x i8], align 1
  %1 = zext i1 %force_no_backtrace to i8
  store i8 %1, ptr %force_no_backtrace.dbg.spill, align 1
    #dbg_declare(ptr %force_no_backtrace.dbg.spill, !849, !DIExpression(), !860)
  call void @llvm.trap(), !dbg !861
  unreachable, !dbg !861
}

; core::ub_checks::maybe_is_aligned
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned17h6c66c7270555ff73E(ptr %ptr, i32 %align) unnamed_addr #1 !dbg !863 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !868, !DIExpression(), !870)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !869, !DIExpression(), !871)
; call core::ub_checks::maybe_is_aligned::runtime
  %_0 = call zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned7runtime17hd98e47256eeccaa1E(ptr %ptr, i32 %align) #10, !dbg !872
  ret i1 %_0, !dbg !874
}

; core::ub_checks::maybe_is_aligned::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned7runtime17hd98e47256eeccaa1E(ptr %ptr, i32 %align) unnamed_addr #1 !dbg !875 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !878, !DIExpression(), !880)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !879, !DIExpression(), !880)
; call core::ptr::const_ptr::<impl *const T>::is_aligned_to
  %_0 = call zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13is_aligned_to17h9d68787d1567a990E"(ptr %ptr, i32 %align) #10, !dbg !881
  ret i1 %_0, !dbg !883
}

; core::ub_checks::check_language_ub
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks17check_language_ub17h7d3209090a219533E() unnamed_addr #1 !dbg !884 {
start:
  %_0 = alloca [1 x i8], align 1
  br label %bb1, !dbg !887

bb1:                                              ; preds = %start
; call core::ub_checks::check_language_ub::runtime
  %0 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub7runtime17h8ff65f2b3cf59f9fE() #10, !dbg !888
  %1 = zext i1 %0 to i8, !dbg !888
  store i8 %1, ptr %_0, align 1, !dbg !888
  br label %bb3, !dbg !888

bb3:                                              ; preds = %bb1
  %2 = load i8, ptr %_0, align 1, !dbg !890
  %3 = trunc nuw i8 %2 to i1, !dbg !890
  ret i1 %3, !dbg !890

bb2:                                              ; No predecessors!
  unreachable
}

; core::ub_checks::check_language_ub::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks17check_language_ub7runtime17h8ff65f2b3cf59f9fE() unnamed_addr #1 !dbg !891 {
start:
  ret i1 true, !dbg !893
}

; core::ub_checks::maybe_is_aligned_and_not_null
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17ha3a1dc64a976744dE(ptr %ptr, i32 %align, i1 zeroext %is_zst) unnamed_addr #1 !dbg !894 {
start:
  %is_zst.dbg.spill = alloca [1 x i8], align 1
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !898, !DIExpression(), !901)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !899, !DIExpression(), !902)
  %0 = zext i1 %is_zst to i8
  store i8 %0, ptr %is_zst.dbg.spill, align 1
    #dbg_declare(ptr %is_zst.dbg.spill, !900, !DIExpression(), !903)
; call core::ub_checks::maybe_is_aligned
  %_4 = call zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned17h6c66c7270555ff73E(ptr %ptr, i32 %align) #10, !dbg !904
  br i1 %_4, label %bb2, label %bb3, !dbg !904

bb3:                                              ; preds = %start
  store i8 0, ptr %_0, align 1, !dbg !904
  br label %bb7, !dbg !904

bb2:                                              ; preds = %start
  br i1 %is_zst, label %bb4, label %bb5, !dbg !905

bb7:                                              ; preds = %bb4, %bb5, %bb3
  %1 = load i8, ptr %_0, align 1, !dbg !906
  %2 = trunc nuw i8 %1 to i1, !dbg !906
  ret i1 %2, !dbg !906

bb5:                                              ; preds = %bb2
; call core::ptr::const_ptr::<impl *const T>::is_null
  %_5 = call zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf56eacc16313c5f5E"(ptr %ptr) #10, !dbg !907
  %3 = xor i1 %_5, true, !dbg !908
  %4 = zext i1 %3 to i8, !dbg !908
  store i8 %4, ptr %_0, align 1, !dbg !908
  br label %bb7, !dbg !909

bb4:                                              ; preds = %bb2
  store i8 1, ptr %_0, align 1, !dbg !909
  br label %bb7, !dbg !909
}

; alloc::alloc::alloc_zeroed
; Function Attrs: inlinehint nounwind
define internal ptr @_ZN5alloc5alloc12alloc_zeroed17h625859128e9e5166E(i32 %0, i32 %1) unnamed_addr #1 !dbg !910 {
start:
  %layout = alloca [8 x i8], align 4
  store i32 %0, ptr %layout, align 4
  %2 = getelementptr inbounds i8, ptr %layout, i32 4
  store i32 %1, ptr %2, align 4
    #dbg_declare(ptr %layout, !916, !DIExpression(), !917)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsaKOfhLrNfTz_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #10, !dbg !918
; call core::alloc::layout::Layout::size
  %_3 = call i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %layout) #10, !dbg !919
; call core::alloc::layout::Layout::align
  %_5 = call i32 @_ZN4core5alloc6layout6Layout5align17h8d2f91b08a22bceaE(ptr align 4 %layout) #10, !dbg !920
; call __rustc::__rust_alloc_zeroed
  %_0 = call ptr @_RNvCsaKOfhLrNfTz_7___rustc19___rust_alloc_zeroed(i32 %_3, i32 %_5) #10, !dbg !921
  ret ptr %_0, !dbg !922
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint nounwind
define internal ptr @_ZN5alloc5alloc15exchange_malloc17h5d053562fbeb5adbE(i32 %size, i32 %align) unnamed_addr #1 !dbg !923 {
start:
  %ptr.dbg.spill = alloca [8 x i8], align 4
  %layout.dbg.spill = alloca [8 x i8], align 4
  %align.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %_4 = alloca [8 x i8], align 4
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !927, !DIExpression(), !940)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !928, !DIExpression(), !941)
; call core::alloc::layout::Layout::from_size_align_unchecked
  %0 = call { i32, i32 } @_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h3599e2620243ac2fE(i32 %size, i32 %align, ptr align 4 @alloc_88beea65e4fab49399d118295256fbde) #10, !dbg !942
  %layout.0 = extractvalue { i32, i32 } %0, 0, !dbg !942
  %layout.1 = extractvalue { i32, i32 } %0, 1, !dbg !942
  store i32 %layout.0, ptr %layout.dbg.spill, align 4, !dbg !942
  %1 = getelementptr inbounds i8, ptr %layout.dbg.spill, i32 4, !dbg !942
  store i32 %layout.1, ptr %1, align 4, !dbg !942
    #dbg_declare(ptr %layout.dbg.spill, !929, !DIExpression(), !943)
; call <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %2 = call { ptr, i32 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h51875d913a88c707E"(ptr align 1 inttoptr (i32 1 to ptr), i32 %layout.0, i32 %layout.1) #10, !dbg !944
  %3 = extractvalue { ptr, i32 } %2, 0, !dbg !944
  %4 = extractvalue { ptr, i32 } %2, 1, !dbg !944
  store ptr %3, ptr %_4, align 4, !dbg !944
  %5 = getelementptr inbounds i8, ptr %_4, i32 4, !dbg !944
  store i32 %4, ptr %5, align 4, !dbg !944
  %6 = load ptr, ptr %_4, align 4, !dbg !945
  %7 = getelementptr inbounds i8, ptr %_4, i32 4, !dbg !945
  %8 = load i32, ptr %7, align 4, !dbg !945
  %9 = ptrtoint ptr %6 to i32, !dbg !945
  %10 = icmp eq i32 %9, 0, !dbg !945
  %_6 = select i1 %10, i32 1, i32 0, !dbg !945
  %11 = trunc nuw i32 %_6 to i1, !dbg !946
  br i1 %11, label %bb4, label %bb5, !dbg !946

bb4:                                              ; preds = %start
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h1e5cf49dcf30c6c7E(i32 %layout.0, i32 %layout.1) #11, !dbg !947
  unreachable, !dbg !947

bb5:                                              ; preds = %start
  %ptr.0 = load ptr, ptr %_4, align 4, !dbg !948
  %12 = getelementptr inbounds i8, ptr %_4, i32 4, !dbg !948
  %ptr.1 = load i32, ptr %12, align 4, !dbg !948
  store ptr %ptr.0, ptr %ptr.dbg.spill, align 4, !dbg !948
  %13 = getelementptr inbounds i8, ptr %ptr.dbg.spill, i32 4, !dbg !948
  store i32 %ptr.1, ptr %13, align 4, !dbg !948
    #dbg_declare(ptr %ptr.dbg.spill, !931, !DIExpression(), !949)
; call core::ptr::non_null::NonNull<[T]>::as_mut_ptr
  %_0 = call ptr @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$10as_mut_ptr17hb0f82c01d41aa59eE"(ptr %ptr.0, i32 %ptr.1) #10, !dbg !950
  ret ptr %_0, !dbg !951

bb3:                                              ; No predecessors!
  unreachable, !dbg !945
}

; alloc::alloc::alloc
; Function Attrs: inlinehint nounwind
define internal ptr @_ZN5alloc5alloc5alloc17h8f42680c0ce0b0ceE(i32 %0, i32 %1) unnamed_addr #1 !dbg !952 {
start:
  %layout = alloca [8 x i8], align 4
  store i32 %0, ptr %layout, align 4
  %2 = getelementptr inbounds i8, ptr %layout, i32 4
  store i32 %1, ptr %2, align 4
    #dbg_declare(ptr %layout, !954, !DIExpression(), !955)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsaKOfhLrNfTz_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #10, !dbg !956
; call core::alloc::layout::Layout::size
  %_3 = call i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %layout) #10, !dbg !957
; call core::alloc::layout::Layout::align
  %_5 = call i32 @_ZN4core5alloc6layout6Layout5align17h8d2f91b08a22bceaE(ptr align 4 %layout) #10, !dbg !958
; call __rustc::__rust_alloc
  %_0 = call ptr @_RNvCsaKOfhLrNfTz_7___rustc12___rust_alloc(i32 %_3, i32 %_5) #10, !dbg !959
  ret ptr %_0, !dbg !960
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nounwind
define internal { ptr, i32 } @_ZN5alloc5alloc6Global10alloc_impl17h2f24deec91cfa471E(ptr align 1 %self, i32 %0, i32 %1, i1 zeroext %zeroed) unnamed_addr #1 !dbg !961 {
start:
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %residual.dbg.spill = alloca [0 x i8], align 1
  %zeroed.dbg.spill = alloca [1 x i8], align 1
  %self.dbg.spill = alloca [4 x i8], align 4
  %_10 = alloca [4 x i8], align 4
  %raw_ptr = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  %layout = alloca [8 x i8], align 4
  store i32 %0, ptr %layout, align 4
  %2 = getelementptr inbounds i8, ptr %layout, i32 4
  store i32 %1, ptr %2, align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !985, !DIExpression(), !1016)
    #dbg_declare(ptr %layout, !986, !DIExpression(), !1017)
  %3 = zext i1 %zeroed to i8
  store i8 %3, ptr %zeroed.dbg.spill, align 1
    #dbg_declare(ptr %zeroed.dbg.spill, !987, !DIExpression(), !1018)
    #dbg_declare(ptr %raw_ptr, !990, !DIExpression(), !1019)
    #dbg_declare(ptr %residual.dbg.spill, !994, !DIExpression(), !1020)
; call core::alloc::layout::Layout::size
  %size = call i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %layout) #10, !dbg !1021
  store i32 %size, ptr %size.dbg.spill, align 4, !dbg !1021
    #dbg_declare(ptr %size.dbg.spill, !988, !DIExpression(), !1022)
  %4 = icmp eq i32 %size, 0, !dbg !1023
  br i1 %4, label %bb3, label %bb2, !dbg !1023

bb3:                                              ; preds = %start
; call core::alloc::layout::Layout::dangling
  %_7 = call ptr @_ZN4core5alloc6layout6Layout8dangling17h1c98afe0c3073c3dE(ptr align 4 %layout) #10, !dbg !1024
; call core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
  %5 = call { ptr, i32 } @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h8a96af0019037d9dE"(ptr %_7, i32 0) #10, !dbg !1025
  %_6.0 = extractvalue { ptr, i32 } %5, 0, !dbg !1025
  %_6.1 = extractvalue { ptr, i32 } %5, 1, !dbg !1025
  store ptr %_6.0, ptr %_0, align 4, !dbg !1026
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1026
  store i32 %_6.1, ptr %6, align 4, !dbg !1026
  br label %bb16, !dbg !1027

bb2:                                              ; preds = %start
  br i1 %zeroed, label %bb6, label %bb7, !dbg !1028

bb16:                                             ; preds = %bb14, %bb13, %bb3
  %7 = load ptr, ptr %_0, align 4, !dbg !1029
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1029
  %9 = load i32, ptr %8, align 4, !dbg !1029
  %10 = insertvalue { ptr, i32 } poison, ptr %7, 0, !dbg !1029
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1, !dbg !1029
  ret { ptr, i32 } %11, !dbg !1029

bb7:                                              ; preds = %bb2
  %12 = load i32, ptr %layout, align 4, !dbg !1030
  %13 = getelementptr inbounds i8, ptr %layout, i32 4, !dbg !1030
  %14 = load i32, ptr %13, align 4, !dbg !1030
; call alloc::alloc::alloc
  %15 = call ptr @_ZN5alloc5alloc5alloc17h8f42680c0ce0b0ceE(i32 %12, i32 %14) #10, !dbg !1030
  store ptr %15, ptr %raw_ptr, align 4, !dbg !1030
  br label %bb8, !dbg !1030

bb6:                                              ; preds = %bb2
  %16 = load i32, ptr %layout, align 4, !dbg !1031
  %17 = getelementptr inbounds i8, ptr %layout, i32 4, !dbg !1031
  %18 = load i32, ptr %17, align 4, !dbg !1031
; call alloc::alloc::alloc_zeroed
  %19 = call ptr @_ZN5alloc5alloc12alloc_zeroed17h625859128e9e5166E(i32 %16, i32 %18) #10, !dbg !1031
  store ptr %19, ptr %raw_ptr, align 4, !dbg !1031
  br label %bb8, !dbg !1031

bb8:                                              ; preds = %bb6, %bb7
  %_13 = load ptr, ptr %raw_ptr, align 4, !dbg !1032
; call core::ptr::non_null::NonNull<T>::new
  %_12 = call ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h5968324df1351dc8E"(ptr %_13) #10, !dbg !1033
; call core::option::Option<T>::ok_or
  %_11 = call ptr @"_ZN4core6option15Option$LT$T$GT$5ok_or17h9504f9ce93f07665E"(ptr %_12) #10, !dbg !1034
; call <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
  %20 = call ptr @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h661768b3f16f6fc8E"(ptr %_11) #10, !dbg !1033
  store ptr %20, ptr %_10, align 4, !dbg !1033
  %21 = load ptr, ptr %_10, align 4, !dbg !1033
  %22 = ptrtoint ptr %21 to i32, !dbg !1033
  %23 = icmp eq i32 %22, 0, !dbg !1033
  %_14 = select i1 %23, i32 1, i32 0, !dbg !1033
  %24 = trunc nuw i32 %_14 to i1, !dbg !1033
  br i1 %24, label %bb14, label %bb13, !dbg !1033

bb14:                                             ; preds = %bb8
; call <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
  %25 = call { ptr, i32 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hbd212d42dd8e72f9E"(ptr align 4 @alloc_410e3011a7a5ea88f7eef411e235cc80) #10, !dbg !1035
  %26 = extractvalue { ptr, i32 } %25, 0, !dbg !1035
  %27 = extractvalue { ptr, i32 } %25, 1, !dbg !1035
  store ptr %26, ptr %_0, align 4, !dbg !1035
  %28 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1035
  store i32 %27, ptr %28, align 4, !dbg !1035
  br label %bb16, !dbg !1035

bb13:                                             ; preds = %bb8
  %ptr = load ptr, ptr %_10, align 4, !dbg !1033
  store ptr %ptr, ptr %ptr.dbg.spill, align 4, !dbg !1033
    #dbg_declare(ptr %ptr.dbg.spill, !992, !DIExpression(), !1036)
    #dbg_declare(ptr %ptr.dbg.spill, !1014, !DIExpression(), !1037)
; call core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
  %29 = call { ptr, i32 } @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h8a96af0019037d9dE"(ptr %ptr, i32 %size) #10, !dbg !1038
  %_16.0 = extractvalue { ptr, i32 } %29, 0, !dbg !1038
  %_16.1 = extractvalue { ptr, i32 } %29, 1, !dbg !1038
  store ptr %_16.0, ptr %_0, align 4, !dbg !1039
  %30 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1039
  store i32 %_16.1, ptr %30, align 4, !dbg !1039
  br label %bb16, !dbg !1040

bb12:                                             ; No predecessors!
  unreachable, !dbg !1033
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint nounwind
define internal void @_ZN5alloc5alloc7dealloc17hb7357e680edfb9d7E(ptr %ptr, i32 %0, i32 %1) unnamed_addr #1 !dbg !1041 {
start:
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %layout = alloca [8 x i8], align 4
  store i32 %0, ptr %layout, align 4
  %2 = getelementptr inbounds i8, ptr %layout, i32 4
  store i32 %1, ptr %2, align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !1045, !DIExpression(), !1047)
    #dbg_declare(ptr %layout, !1046, !DIExpression(), !1048)
; call core::alloc::layout::Layout::size
  %_3 = call i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %layout) #10, !dbg !1049
; call core::alloc::layout::Layout::align
  %_5 = call i32 @_ZN4core5alloc6layout6Layout5align17h8d2f91b08a22bceaE(ptr align 4 %layout) #10, !dbg !1050
; call __rustc::__rust_dealloc
  call void @_RNvCsaKOfhLrNfTz_7___rustc14___rust_dealloc(ptr %ptr, i32 %_3, i32 %_5) #10, !dbg !1051
  ret void, !dbg !1052
}

; alloc::boxed::Box<T>::from_raw
; Function Attrs: inlinehint nounwind
define dso_local align 8 ptr @"_ZN5alloc5boxed12Box$LT$T$GT$8from_raw17h787409b512d29d17E"(ptr %raw) unnamed_addr #1 !dbg !1053 {
start:
  %raw.dbg.spill = alloca [4 x i8], align 4
  store ptr %raw, ptr %raw.dbg.spill, align 4
    #dbg_declare(ptr %raw.dbg.spill, !1058, !DIExpression(), !1059)
; call alloc::boxed::Box<T,A>::from_raw_in
  %_0 = call align 8 ptr @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h70f6ff0daf4789b4E"(ptr %raw) #10, !dbg !1060
  ret ptr %_0, !dbg !1061
}

; alloc::boxed::Box<T>::from_raw
; Function Attrs: inlinehint nounwind
define dso_local { ptr, ptr } @"_ZN5alloc5boxed12Box$LT$T$GT$8from_raw17hdae822c5d06d5e51E"(ptr %raw.0, ptr align 4 %raw.1) unnamed_addr #1 !dbg !1062 {
start:
  %raw.dbg.spill = alloca [8 x i8], align 4
  store ptr %raw.0, ptr %raw.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %raw.dbg.spill, i32 4
  store ptr %raw.1, ptr %0, align 4
    #dbg_declare(ptr %raw.dbg.spill, !1066, !DIExpression(), !1067)
; call alloc::boxed::Box<T,A>::from_raw_in
  %1 = call { ptr, ptr } @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h2f3a84ec7149a747E"(ptr %raw.0, ptr align 4 %raw.1) #10, !dbg !1068
  %_0.0 = extractvalue { ptr, ptr } %1, 0, !dbg !1068
  %_0.1 = extractvalue { ptr, ptr } %1, 1, !dbg !1068
  %2 = insertvalue { ptr, ptr } poison, ptr %_0.0, 0, !dbg !1069
  %3 = insertvalue { ptr, ptr } %2, ptr %_0.1, 1, !dbg !1069
  ret { ptr, ptr } %3, !dbg !1069
}

; alloc::boxed::Box<T>::into_raw
; Function Attrs: inlinehint nounwind
define dso_local ptr @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h7cdb6077d8e206e4E"(ptr align 8 %b) unnamed_addr #1 !dbg !1070 {
start:
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %value.dbg.spill.i = alloca [4 x i8], align 4
  %b.dbg.spill = alloca [4 x i8], align 4
  %b1 = alloca [4 x i8], align 4
  store ptr %b, ptr %b.dbg.spill, align 4
    #dbg_declare(ptr %b.dbg.spill, !1074, !DIExpression(), !1081)
    #dbg_declare(ptr %b1, !1075, !DIExpression(), !1082)
  store ptr %b, ptr %value.dbg.spill.i, align 4
    #dbg_declare(ptr %value.dbg.spill.i, !1083, !DIExpression(), !1090)
  store ptr %b, ptr %b1, align 4, !dbg !1092
  store ptr %b1, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1093, !DIExpression(), !1101)
  %_5 = load ptr, ptr %b1, align 4, !dbg !1103
  ret ptr %_5, !dbg !1104
}

; alloc::boxed::Box<T>::into_raw
; Function Attrs: inlinehint nounwind
define dso_local { ptr, ptr } @"_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17hd58a19267a876565E"(ptr align 1 %b.0, ptr align 4 %b.1) unnamed_addr #1 !dbg !1105 {
start:
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %value.dbg.spill.i = alloca [8 x i8], align 4
  %b.dbg.spill = alloca [8 x i8], align 4
  %b = alloca [8 x i8], align 4
  store ptr %b.0, ptr %b.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %b.dbg.spill, i32 4
  store ptr %b.1, ptr %0, align 4
    #dbg_declare(ptr %b.dbg.spill, !1109, !DIExpression(), !1115)
    #dbg_declare(ptr %b, !1110, !DIExpression(), !1116)
  store ptr %b.0, ptr %value.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %value.dbg.spill.i, i32 4
  store ptr %b.1, ptr %1, align 4
    #dbg_declare(ptr %value.dbg.spill.i, !1117, !DIExpression(), !1123)
  %2 = insertvalue { ptr, ptr } poison, ptr %b.0, 0, !dbg !1125
  %3 = insertvalue { ptr, ptr } %2, ptr %b.1, 1, !dbg !1125
  %4 = extractvalue { ptr, ptr } %3, 0, !dbg !1126
  %5 = extractvalue { ptr, ptr } %3, 1, !dbg !1126
  store ptr %4, ptr %b, align 4, !dbg !1126
  %6 = getelementptr inbounds i8, ptr %b, i32 4, !dbg !1126
  store ptr %5, ptr %6, align 4, !dbg !1126
  store ptr %b, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1127, !DIExpression(), !1134)
  %_5.0 = load ptr, ptr %b, align 4, !dbg !1136
  %7 = getelementptr inbounds i8, ptr %b, i32 4, !dbg !1136
  %_5.1 = load ptr, ptr %7, align 4, !dbg !1136
  %8 = insertvalue { ptr, ptr } poison, ptr %_5.0, 0, !dbg !1137
  %9 = insertvalue { ptr, ptr } %8, ptr %_5.1, 1, !dbg !1137
  ret { ptr, ptr } %9, !dbg !1137
}

; alloc::boxed::Box<T,A>::from_raw_in
; Function Attrs: inlinehint nounwind
define dso_local { ptr, ptr } @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h2f3a84ec7149a747E"(ptr %raw.0, ptr align 4 %raw.1) unnamed_addr #1 !dbg !1138 {
start:
  %alloc.dbg.spill = alloca [0 x i8], align 1
  %raw.dbg.spill = alloca [8 x i8], align 4
  store ptr %raw.0, ptr %raw.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %raw.dbg.spill, i32 4
  store ptr %raw.1, ptr %0, align 4
    #dbg_declare(ptr %raw.dbg.spill, !1143, !DIExpression(), !1147)
    #dbg_declare(ptr %alloc.dbg.spill, !1144, !DIExpression(), !1148)
; call core::ptr::unique::Unique<T>::new_unchecked
  %1 = call { ptr, ptr } @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hf8bbe7c74bad6648E"(ptr %raw.0, ptr align 4 %raw.1) #10, !dbg !1149
  %_3.0 = extractvalue { ptr, ptr } %1, 0, !dbg !1149
  %_3.1 = extractvalue { ptr, ptr } %1, 1, !dbg !1149
  %2 = insertvalue { ptr, ptr } poison, ptr %_3.0, 0, !dbg !1150
  %3 = insertvalue { ptr, ptr } %2, ptr %_3.1, 1, !dbg !1150
  ret { ptr, ptr } %3, !dbg !1150
}

; alloc::boxed::Box<T,A>::from_raw_in
; Function Attrs: inlinehint nounwind
define dso_local align 8 ptr @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h70f6ff0daf4789b4E"(ptr %raw) unnamed_addr #1 !dbg !1151 {
start:
  %alloc.dbg.spill = alloca [0 x i8], align 1
  %raw.dbg.spill = alloca [4 x i8], align 4
  store ptr %raw, ptr %raw.dbg.spill, align 4
    #dbg_declare(ptr %raw.dbg.spill, !1155, !DIExpression(), !1158)
    #dbg_declare(ptr %alloc.dbg.spill, !1156, !DIExpression(), !1159)
; call core::ptr::unique::Unique<T>::new_unchecked
  %_3 = call ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha82630dde1c4b48aE"(ptr %raw) #10, !dbg !1160
  ret ptr %_3, !dbg !1161
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nounwind
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h4cc2aa304c89386fE"(ptr align 1 %self, ptr %ptr, i32 %0, i32 %1) unnamed_addr #1 !dbg !1162 {
start:
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %layout = alloca [8 x i8], align 4
  store i32 %0, ptr %layout, align 4
  %2 = getelementptr inbounds i8, ptr %layout, i32 4
  store i32 %1, ptr %2, align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1167, !DIExpression(), !1170)
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !1168, !DIExpression(), !1171)
    #dbg_declare(ptr %layout, !1169, !DIExpression(), !1172)
; call core::alloc::layout::Layout::size
  %_4 = call i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %layout) #10, !dbg !1173
  %3 = icmp eq i32 %_4, 0, !dbg !1174
  br i1 %3, label %bb4, label %bb2, !dbg !1174

bb4:                                              ; preds = %bb2, %start
  ret void, !dbg !1175

bb2:                                              ; preds = %start
  store ptr %ptr, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1176, !DIExpression(), !1182)
  %4 = load i32, ptr %layout, align 4, !dbg !1184
  %5 = getelementptr inbounds i8, ptr %layout, i32 4, !dbg !1184
  %6 = load i32, ptr %5, align 4, !dbg !1184
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17hb7357e680edfb9d7E(ptr %ptr, i32 %4, i32 %6) #10, !dbg !1184
  br label %bb4, !dbg !1184
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint nounwind
define internal { ptr, i32 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h51875d913a88c707E"(ptr align 1 %self, i32 %layout.0, i32 %layout.1) unnamed_addr #1 !dbg !1185 {
start:
  %layout.dbg.spill = alloca [8 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1189, !DIExpression(), !1191)
  store i32 %layout.0, ptr %layout.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %layout.dbg.spill, i32 4
  store i32 %layout.1, ptr %0, align 4
    #dbg_declare(ptr %layout.dbg.spill, !1190, !DIExpression(), !1192)
; call alloc::alloc::Global::alloc_impl
  %1 = call { ptr, i32 } @_ZN5alloc5alloc6Global10alloc_impl17h2f24deec91cfa471E(ptr align 1 %self, i32 %layout.0, i32 %layout.1, i1 zeroext false) #10, !dbg !1193
  %_0.0 = extractvalue { ptr, i32 } %1, 0, !dbg !1193
  %_0.1 = extractvalue { ptr, i32 } %1, 1, !dbg !1193
  %2 = insertvalue { ptr, i32 } poison, ptr %_0.0, 0, !dbg !1194
  %3 = insertvalue { ptr, i32 } %2, i32 %_0.1, 1, !dbg !1194
  ret { ptr, i32 } %3, !dbg !1194
}

; <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: inlinehint nounwind
define dso_local void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9f990ada96c0379fE"(ptr align 4 %self) unnamed_addr #1 !dbg !1195 {
start:
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %layout = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1200, !DIExpression(), !1205)
    #dbg_declare(ptr %layout, !1203, !DIExpression(), !1206)
  %ptr = load ptr, ptr %self, align 4, !dbg !1207
  store ptr %ptr, ptr %ptr.dbg.spill, align 4, !dbg !1207
    #dbg_declare(ptr %ptr.dbg.spill, !1201, !DIExpression(), !1208)
; call core::ptr::unique::Unique<T>::as_ptr
  %_5 = call ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h38473301210de45fE"(ptr %ptr) #10, !dbg !1209
; call core::alloc::layout::Layout::for_value_raw
  %0 = call { i32, i32 } @_ZN4core5alloc6layout6Layout13for_value_raw17hb269eae829dfffa3E(ptr %_5) #10, !dbg !1210
  %1 = extractvalue { i32, i32 } %0, 0, !dbg !1210
  %2 = extractvalue { i32, i32 } %0, 1, !dbg !1210
  store i32 %1, ptr %layout, align 4, !dbg !1210
  %3 = getelementptr inbounds i8, ptr %layout, i32 4, !dbg !1210
  store i32 %2, ptr %3, align 4, !dbg !1210
; call core::alloc::layout::Layout::size
  %_6 = call i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %layout) #10, !dbg !1211
  %4 = icmp eq i32 %_6, 0, !dbg !1212
  br i1 %4, label %bb7, label %bb4, !dbg !1212

bb7:                                              ; preds = %bb4, %start
  ret void, !dbg !1213

bb4:                                              ; preds = %start
  %_9 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1214
; call core::ptr::unique::Unique<T>::cast
  %_11 = call ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hf99e8766b267ebecE"(ptr %ptr) #10, !dbg !1215
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %_10 = call ptr @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h9667c3a18e98260fE"(ptr %_11) #10, !dbg !1216
  %5 = load i32, ptr %layout, align 4, !dbg !1214
  %6 = getelementptr inbounds i8, ptr %layout, i32 4, !dbg !1214
  %7 = load i32, ptr %6, align 4, !dbg !1214
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h4cc2aa304c89386fE"(ptr align 1 %_9, ptr %_10, i32 %5, i32 %7) #10, !dbg !1217
  br label %bb7, !dbg !1217
}

; <alloc::boxed::Box<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: inlinehint nounwind
define dso_local void @"_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfcf1c84a7897640aE"(ptr align 4 %self) unnamed_addr #1 !dbg !1218 {
start:
  %ptr.dbg.spill = alloca [8 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %layout = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1222, !DIExpression(), !1227)
    #dbg_declare(ptr %layout, !1225, !DIExpression(), !1228)
  %ptr.0 = load ptr, ptr %self, align 4, !dbg !1229
  %0 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1229
  %ptr.1 = load ptr, ptr %0, align 4, !dbg !1229
  store ptr %ptr.0, ptr %ptr.dbg.spill, align 4, !dbg !1229
  %1 = getelementptr inbounds i8, ptr %ptr.dbg.spill, i32 4, !dbg !1229
  store ptr %ptr.1, ptr %1, align 4, !dbg !1229
    #dbg_declare(ptr %ptr.dbg.spill, !1223, !DIExpression(), !1230)
; call core::ptr::unique::Unique<T>::as_ptr
  %2 = call { ptr, ptr } @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17he3585bd48d4ed619E"(ptr %ptr.0, ptr align 4 %ptr.1) #10, !dbg !1231
  %_5.0 = extractvalue { ptr, ptr } %2, 0, !dbg !1231
  %_5.1 = extractvalue { ptr, ptr } %2, 1, !dbg !1231
; call core::alloc::layout::Layout::for_value_raw
  %3 = call { i32, i32 } @_ZN4core5alloc6layout6Layout13for_value_raw17h41fcf0e2c2b6f065E(ptr %_5.0, ptr align 4 %_5.1) #10, !dbg !1232
  %4 = extractvalue { i32, i32 } %3, 0, !dbg !1232
  %5 = extractvalue { i32, i32 } %3, 1, !dbg !1232
  store i32 %4, ptr %layout, align 4, !dbg !1232
  %6 = getelementptr inbounds i8, ptr %layout, i32 4, !dbg !1232
  store i32 %5, ptr %6, align 4, !dbg !1232
; call core::alloc::layout::Layout::size
  %_6 = call i32 @_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E(ptr align 4 %layout) #10, !dbg !1233
  %7 = icmp eq i32 %_6, 0, !dbg !1234
  br i1 %7, label %bb7, label %bb4, !dbg !1234

bb7:                                              ; preds = %bb4, %start
  ret void, !dbg !1235

bb4:                                              ; preds = %start
  %_9 = getelementptr inbounds i8, ptr %self, i32 8, !dbg !1236
; call core::ptr::unique::Unique<T>::cast
  %_11 = call ptr @"_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h879afa9afd8f0935E"(ptr %ptr.0, ptr align 4 %ptr.1) #10, !dbg !1237
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %_10 = call ptr @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h9667c3a18e98260fE"(ptr %_11) #10, !dbg !1238
  %8 = load i32, ptr %layout, align 4, !dbg !1236
  %9 = getelementptr inbounds i8, ptr %layout, i32 4, !dbg !1236
  %10 = load i32, ptr %9, align 4, !dbg !1236
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h4cc2aa304c89386fE"(ptr align 1 %_9, ptr %_10, i32 %8, i32 %10) #10, !dbg !1239
  br label %bb7, !dbg !1239
}

; __rustc::__rust_drop_panic
; Function Attrs: noreturn nounwind
declare dso_local void @_RNvCsaKOfhLrNfTz_7___rustc17___rust_drop_panic() unnamed_addr #3

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #4

; unwind::wasm::_Unwind_RaiseException
; Function Attrs: nounwind
declare dso_local i32 @_ZN6unwind4wasm22_Unwind_RaiseException17ha86401ff8c97b5afE(ptr) unnamed_addr #0

; __rustc::__rust_foreign_exception
; Function Attrs: noreturn nounwind
declare dso_local void @_RNvCsaKOfhLrNfTz_7___rustc24___rust_foreign_exception() unnamed_addr #3

; unwind::wasm::_Unwind_DeleteException
; Function Attrs: nounwind
declare dso_local void @_ZN6unwind4wasm23_Unwind_DeleteException17h681e99c9e311f412E(ptr) unnamed_addr #0

; core::ptr::metadata::from_raw_parts
; Function Attrs: inlinehint nounwind
declare dso_local ptr @_ZN4core3ptr8metadata14from_raw_parts17h95d9158ce8c57919E(ptr) unnamed_addr #1

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nounwind
declare dso_local void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4, ptr align 4) unnamed_addr #1

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hf42c4102b4c84c5dE"(ptr) unnamed_addr #1

; core::alloc::layout::Layout::is_size_align_valid
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h77ec66f10f926786E(i32, i32) unnamed_addr #0

; core::ptr::non_null::NonNull<T>::without_provenance
; Function Attrs: inlinehint nounwind
declare dso_local ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$18without_provenance17hb3cf249ff93074e0E"(i32) unnamed_addr #1

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #5

; core::ptr::const_ptr::<impl *const T>::is_aligned_to
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13is_aligned_to17h9d68787d1567a990E"(ptr, i32) unnamed_addr #1

; core::ptr::const_ptr::<impl *const T>::is_null
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf56eacc16313c5f5E"(ptr) unnamed_addr #1

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind
declare dso_local void @_RNvCsaKOfhLrNfTz_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #0

; __rustc::__rust_alloc_zeroed
; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0)
declare dso_local noalias ptr @_RNvCsaKOfhLrNfTz_7___rustc19___rust_alloc_zeroed(i32, i32 allocalign) unnamed_addr #6

; core::ptr::non_null::NonNull<[T]>::as_mut_ptr
; Function Attrs: inlinehint nounwind
declare dso_local ptr @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$10as_mut_ptr17hb0f82c01d41aa59eE"(ptr, i32) unnamed_addr #1

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn nounwind optsize
declare dso_local void @_ZN5alloc5alloc18handle_alloc_error17h1e5cf49dcf30c6c7E(i32, i32) unnamed_addr #7

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0)
declare dso_local noalias ptr @_RNvCsaKOfhLrNfTz_7___rustc12___rust_alloc(i32, i32 allocalign) unnamed_addr #8

; core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
; Function Attrs: inlinehint nounwind
declare dso_local { ptr, i32 } @"_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h8a96af0019037d9dE"(ptr, i32) unnamed_addr #1

; core::ptr::non_null::NonNull<T>::new
; Function Attrs: inlinehint nounwind
declare dso_local ptr @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17h5968324df1351dc8E"(ptr) unnamed_addr #1

; core::option::Option<T>::ok_or
; Function Attrs: inlinehint nounwind
declare dso_local ptr @"_ZN4core6option15Option$LT$T$GT$5ok_or17h9504f9ce93f07665E"(ptr) unnamed_addr #1

; <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind
declare dso_local ptr @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h661768b3f16f6fc8E"(ptr) unnamed_addr #1

; <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
; Function Attrs: inlinehint nounwind
declare dso_local { ptr, i32 } @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17hbd212d42dd8e72f9E"(ptr align 4) unnamed_addr #1

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free")
declare dso_local void @_RNvCsaKOfhLrNfTz_7___rustc14___rust_dealloc(ptr allocptr, i32, i32) unnamed_addr #9

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint nounwind
declare dso_local ptr @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h9667c3a18e98260fE"(ptr) unnamed_addr #1

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { inlinehint nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { inlinehint noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #3 = { noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #6 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) "alloc-family"="__rust_alloc" "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #7 = { cold minsize noreturn nounwind optsize "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #8 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsaKOfhLrNfTz_7___rustc19___rust_alloc_zeroed" "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #9 = { nounwind allockind("free") "alloc-family"="__rust_alloc" "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #10 = { nounwind }
attributes #11 = { noreturn nounwind }

!llvm.ident = !{!6}
!llvm.dbg.cu = !{!7}
!llvm.module.flags = !{!64, !65}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "CANARY", linkageName: "_ZN12panic_unwind3imp6CANARY17haae52ab13dfd4f85E", scope: !2, file: !4, line: 48, type: !5, isLocal: true, isDefinition: true, align: 8)
!2 = !DINamespace(name: "imp", scope: !3)
!3 = !DINamespace(name: "panic_unwind", scope: null)
!4 = !DIFile(filename: "src/gcc.rs", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_unwind", checksumkind: CSK_MD5, checksum: "6aebb7ec23b887cf46b0477c7002bf35")
!5 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!6 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!7 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !8, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !9, globals: !63, splitDebugInlining: false, nameTableKind: None)
!8 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_unwind/src/lib.rs/@/panic_unwind.68504bd31f300e9e-cgu.0", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_unwind")
!9 = !{!10, !26}
!10 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "_Unwind_Reason_Code", scope: !12, file: !11, baseType: !14, size: 32, align: 32, flags: DIFlagEnumClass, elements: !15)
!11 = !DIFile(filename: "<unknown>", directory: "")
!12 = !DINamespace(name: "wasm", scope: !13)
!13 = !DINamespace(name: "unwind", scope: null)
!14 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!15 = !{!16, !17, !18, !19, !20, !21, !22, !23, !24, !25}
!16 = !DIEnumerator(name: "_URC_NO_REASON", value: 0, isUnsigned: true)
!17 = !DIEnumerator(name: "_URC_FOREIGN_EXCEPTION_CAUGHT", value: 1, isUnsigned: true)
!18 = !DIEnumerator(name: "_URC_FATAL_PHASE2_ERROR", value: 2, isUnsigned: true)
!19 = !DIEnumerator(name: "_URC_FATAL_PHASE1_ERROR", value: 3, isUnsigned: true)
!20 = !DIEnumerator(name: "_URC_NORMAL_STOP", value: 4, isUnsigned: true)
!21 = !DIEnumerator(name: "_URC_END_OF_STACK", value: 5, isUnsigned: true)
!22 = !DIEnumerator(name: "_URC_HANDLER_FOUND", value: 6, isUnsigned: true)
!23 = !DIEnumerator(name: "_URC_INSTALL_CONTEXT", value: 7, isUnsigned: true)
!24 = !DIEnumerator(name: "_URC_CONTINUE_UNWIND", value: 8, isUnsigned: true)
!25 = !DIEnumerator(name: "_URC_FAILURE", value: 9, isUnsigned: true)
!26 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "AlignmentEnum", scope: !27, file: !11, baseType: !14, size: 32, align: 32, flags: DIFlagEnumClass, elements: !30)
!27 = !DINamespace(name: "alignment", scope: !28)
!28 = !DINamespace(name: "ptr", scope: !29)
!29 = !DINamespace(name: "core", scope: null)
!30 = !{!31, !32, !33, !34, !35, !36, !37, !38, !39, !40, !41, !42, !43, !44, !45, !46, !47, !48, !49, !50, !51, !52, !53, !54, !55, !56, !57, !58, !59, !60, !61, !62}
!31 = !DIEnumerator(name: "_Align1Shl0", value: 1, isUnsigned: true)
!32 = !DIEnumerator(name: "_Align1Shl1", value: 2, isUnsigned: true)
!33 = !DIEnumerator(name: "_Align1Shl2", value: 4, isUnsigned: true)
!34 = !DIEnumerator(name: "_Align1Shl3", value: 8, isUnsigned: true)
!35 = !DIEnumerator(name: "_Align1Shl4", value: 16, isUnsigned: true)
!36 = !DIEnumerator(name: "_Align1Shl5", value: 32, isUnsigned: true)
!37 = !DIEnumerator(name: "_Align1Shl6", value: 64, isUnsigned: true)
!38 = !DIEnumerator(name: "_Align1Shl7", value: 128, isUnsigned: true)
!39 = !DIEnumerator(name: "_Align1Shl8", value: 256, isUnsigned: true)
!40 = !DIEnumerator(name: "_Align1Shl9", value: 512, isUnsigned: true)
!41 = !DIEnumerator(name: "_Align1Shl10", value: 1024, isUnsigned: true)
!42 = !DIEnumerator(name: "_Align1Shl11", value: 2048, isUnsigned: true)
!43 = !DIEnumerator(name: "_Align1Shl12", value: 4096, isUnsigned: true)
!44 = !DIEnumerator(name: "_Align1Shl13", value: 8192, isUnsigned: true)
!45 = !DIEnumerator(name: "_Align1Shl14", value: 16384, isUnsigned: true)
!46 = !DIEnumerator(name: "_Align1Shl15", value: 32768, isUnsigned: true)
!47 = !DIEnumerator(name: "_Align1Shl16", value: 65536, isUnsigned: true)
!48 = !DIEnumerator(name: "_Align1Shl17", value: 131072, isUnsigned: true)
!49 = !DIEnumerator(name: "_Align1Shl18", value: 262144, isUnsigned: true)
!50 = !DIEnumerator(name: "_Align1Shl19", value: 524288, isUnsigned: true)
!51 = !DIEnumerator(name: "_Align1Shl20", value: 1048576, isUnsigned: true)
!52 = !DIEnumerator(name: "_Align1Shl21", value: 2097152, isUnsigned: true)
!53 = !DIEnumerator(name: "_Align1Shl22", value: 4194304, isUnsigned: true)
!54 = !DIEnumerator(name: "_Align1Shl23", value: 8388608, isUnsigned: true)
!55 = !DIEnumerator(name: "_Align1Shl24", value: 16777216, isUnsigned: true)
!56 = !DIEnumerator(name: "_Align1Shl25", value: 33554432, isUnsigned: true)
!57 = !DIEnumerator(name: "_Align1Shl26", value: 67108864, isUnsigned: true)
!58 = !DIEnumerator(name: "_Align1Shl27", value: 134217728, isUnsigned: true)
!59 = !DIEnumerator(name: "_Align1Shl28", value: 268435456, isUnsigned: true)
!60 = !DIEnumerator(name: "_Align1Shl29", value: 536870912, isUnsigned: true)
!61 = !DIEnumerator(name: "_Align1Shl30", value: 1073741824, isUnsigned: true)
!62 = !DIEnumerator(name: "_Align1Shl31", value: 2147483648, isUnsigned: true)
!63 = !{!0}
!64 = !{i32 7, !"Dwarf Version", i32 4}
!65 = !{i32 2, !"Debug Info Version", i32 3}
!66 = distinct !DISubprogram(name: "__rust_start_panic", linkageName: "_RNvCsaKOfhLrNfTz_7___rustc18___rust_start_panic", scope: !3, file: !67, line: 106, type: !68, scopeLine: 106, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !82)
!67 = !DIFile(filename: "src/lib.rs", directory: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/panic_unwind", checksumkind: CSK_MD5, checksum: "1d5fcf624416392bfee77aef51170eaf")
!68 = !DISubroutineType(types: !69)
!69 = !{!14, !70}
!70 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::panic::PanicPayload", file: !11, size: 64, align: 32, elements: !71, templateParams: !75, identifier: "67ba2170db1d53ab1e91431929796ef1")
!71 = !{!72, !76}
!72 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !70, file: !11, baseType: !73, size: 32, align: 32)
!73 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !74, size: 32, align: 32, dwarfAddressSpace: 0)
!74 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::panic::PanicPayload", file: !11, align: 8, elements: !75, identifier: "553ec5eb6080bf5f762a22dd2a24a55d")
!75 = !{}
!76 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !70, file: !11, baseType: !77, size: 32, align: 32, offset: 32)
!77 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 7]", baseType: !78, size: 32, align: 32, dwarfAddressSpace: 0)
!78 = !DICompositeType(tag: DW_TAG_array_type, baseType: !79, size: 224, align: 32, elements: !80)
!79 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!80 = !{!81}
!81 = !DISubrange(count: 7, lowerBound: 0)
!82 = !{!83, !84}
!83 = !DILocalVariable(name: "payload", arg: 1, scope: !66, file: !67, line: 106, type: !70)
!84 = !DILocalVariable(name: "payload", scope: !85, file: !67, line: 108, type: !86, align: 32)
!85 = distinct !DILexicalBlock(scope: !66, file: !67, line: 108, column: 9)
!86 = !DICompositeType(tag: DW_TAG_structure_type, name: "alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>", file: !11, size: 64, align: 32, elements: !87, templateParams: !75, identifier: "65e66d2c722abf4caf4dbf3cd3d2fd")
!87 = !{!88, !91}
!88 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !86, file: !11, baseType: !89, size: 32, align: 32)
!89 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !90, size: 32, align: 32, dwarfAddressSpace: 0)
!90 = !DICompositeType(tag: DW_TAG_structure_type, name: "(dyn core::any::Any + core::marker::Send)", file: !11, align: 8, elements: !75, identifier: "1d1aa39a9f0ef7c085a062eeef752e17")
!91 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !86, file: !11, baseType: !92, size: 32, align: 32, offset: 32)
!92 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 4]", baseType: !93, size: 32, align: 32, dwarfAddressSpace: 0)
!93 = !DICompositeType(tag: DW_TAG_array_type, baseType: !79, size: 128, align: 32, elements: !94)
!94 = !{!95}
!95 = !DISubrange(count: 4, lowerBound: 0)
!96 = !DILocation(line: 106, column: 34, scope: !66)
!97 = !DILocation(line: 108, column: 37, scope: !66)
!98 = !DILocation(line: 108, column: 45, scope: !66)
!99 = !DILocation(line: 108, column: 23, scope: !66)
!100 = !DILocation(line: 108, column: 13, scope: !85)
!101 = !DILocation(line: 110, column: 9, scope: !85)
!102 = !DILocation(line: 112, column: 2, scope: !66)
!103 = distinct !DISubprogram(name: "__rust_panic_cleanup", linkageName: "_RNvCsaKOfhLrNfTz_7___rustc20___rust_panic_cleanup", scope: !3, file: !67, line: 99, type: !104, scopeLine: 99, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !111)
!104 = !DISubroutineType(types: !105)
!105 = !{!106, !110}
!106 = !DICompositeType(tag: DW_TAG_structure_type, name: "*mut (dyn core::any::Any + core::marker::Send)", file: !11, size: 64, align: 32, elements: !107, templateParams: !75, identifier: "f7b782cc162036cd5dc3538cdb11c32d")
!107 = !{!108, !109}
!108 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !106, file: !11, baseType: !89, size: 32, align: 32)
!109 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !106, file: !11, baseType: !92, size: 32, align: 32, offset: 32)
!110 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !5, size: 32, align: 32, dwarfAddressSpace: 0)
!111 = !{!112}
!112 = !DILocalVariable(name: "payload", arg: 1, scope: !103, file: !67, line: 99, type: !110)
!113 = !DILocation(line: 99, column: 47, scope: !103)
!114 = !DILocation(line: 100, column: 28, scope: !103)
!115 = !DILocation(line: 100, column: 14, scope: !103)
!116 = !DILocation(line: 101, column: 2, scope: !103)
!117 = distinct !DISubprogram(name: "exception_cleanup", linkageName: "_ZN12panic_unwind3imp5panic17exception_cleanup17hecd110216a6b05d9E", scope: !118, file: !4, line: 74, type: !119, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !147)
!118 = !DINamespace(name: "panic", scope: !2)
!119 = !DISubroutineType(types: !120)
!120 = !{null, !10, !121}
!121 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut unwind::wasm::_Unwind_Exception", baseType: !122, size: 32, align: 32, dwarfAddressSpace: 0)
!122 = !DICompositeType(tag: DW_TAG_structure_type, name: "_Unwind_Exception", scope: !12, file: !11, size: 192, align: 64, flags: DIFlagPublic, elements: !123, templateParams: !75, identifier: "143b72a6d9acf5a5f1719582afd5a7c")
!123 = !{!124, !126, !142}
!124 = !DIDerivedType(tag: DW_TAG_member, name: "exception_class", scope: !122, file: !11, baseType: !125, size: 64, align: 64, flags: DIFlagPublic)
!125 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!126 = !DIDerivedType(tag: DW_TAG_member, name: "exception_cleanup", scope: !122, file: !11, baseType: !127, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
!127 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<extern \22C\22 fn(unwind::wasm::_Unwind_Reason_Code, *mut unwind::wasm::_Unwind_Exception)>", scope: !128, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !129, templateParams: !75, identifier: "e886b2d6c83c340e60a3da4758145bc0")
!128 = !DINamespace(name: "option", scope: !29)
!129 = !{!130}
!130 = !DICompositeType(tag: DW_TAG_variant_part, scope: !127, file: !11, size: 32, align: 32, elements: !131, templateParams: !75, identifier: "452af262fb7c4f8bba06199152ebee49", discriminator: !141)
!131 = !{!132, !137}
!132 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !130, file: !11, baseType: !133, size: 32, align: 32, extraData: i32 0)
!133 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !127, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !75, templateParams: !134, identifier: "a71ea4bc3b5def1237da70014cdba56b")
!134 = !{!135}
!135 = !DITemplateTypeParameter(name: "T", type: !136)
!136 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "extern \22C\22 fn(unwind::wasm::_Unwind_Reason_Code, *mut unwind::wasm::_Unwind_Exception)", baseType: !119, size: 32, align: 32, dwarfAddressSpace: 0)
!137 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !130, file: !11, baseType: !138, size: 32, align: 32)
!138 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !127, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !139, templateParams: !134, identifier: "b6d5060a0b224e9dcd0c7db0baa064e2")
!139 = !{!140}
!140 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !138, file: !11, baseType: !136, size: 32, align: 32, flags: DIFlagPublic)
!141 = !DIDerivedType(tag: DW_TAG_member, scope: !127, file: !11, baseType: !14, size: 32, align: 32, flags: DIFlagArtificial)
!142 = !DIDerivedType(tag: DW_TAG_member, name: "private", scope: !122, file: !11, baseType: !143, size: 64, align: 32, offset: 96, flags: DIFlagPublic)
!143 = !DICompositeType(tag: DW_TAG_array_type, baseType: !144, size: 64, align: 32, elements: !145)
!144 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !5, size: 32, align: 32, dwarfAddressSpace: 0)
!145 = !{!146}
!146 = !DISubrange(count: 2, lowerBound: 0)
!147 = !{!148, !149}
!148 = !DILocalVariable(name: "_unwind_code", arg: 1, scope: !117, file: !4, line: 75, type: !10)
!149 = !DILocalVariable(name: "exception", arg: 2, scope: !117, file: !4, line: 76, type: !121)
!150 = !DILocation(line: 75, column: 9, scope: !117)
!151 = !DILocation(line: 76, column: 9, scope: !117)
!152 = !DILocation(line: 79, column: 37, scope: !117)
!153 = !DILocation(line: 79, column: 79, scope: !117)
!154 = !DILocation(line: 80, column: 13, scope: !117)
!155 = distinct !DISubprogram(name: "panic", linkageName: "_ZN12panic_unwind3imp5panic17h07493e1a9864dee3E", scope: !2, file: !4, line: 61, type: !156, scopeLine: 61, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !158)
!156 = !DISubroutineType(types: !157)
!157 = !{!14, !86}
!158 = !{!159, !160, !168}
!159 = !DILocalVariable(name: "data", arg: 1, scope: !155, file: !4, line: 61, type: !86)
!160 = !DILocalVariable(name: "exception", scope: !161, file: !4, line: 62, type: !162, align: 32)
!161 = distinct !DILexicalBlock(scope: !155, file: !4, line: 62, column: 5)
!162 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>", baseType: !163, size: 32, align: 32, dwarfAddressSpace: 0)
!163 = !DICompositeType(tag: DW_TAG_structure_type, name: "Exception", scope: !2, file: !11, size: 320, align: 64, flags: DIFlagPrivate, elements: !164, templateParams: !75, identifier: "7a5e394566f4b15feca34cd38708bfee")
!164 = !{!165, !166, !167}
!165 = !DIDerivedType(tag: DW_TAG_member, name: "_uwe", scope: !163, file: !11, baseType: !122, size: 192, align: 64, flags: DIFlagPrivate)
!166 = !DIDerivedType(tag: DW_TAG_member, name: "canary", scope: !163, file: !11, baseType: !144, size: 32, align: 32, offset: 192, flags: DIFlagPrivate)
!167 = !DIDerivedType(tag: DW_TAG_member, name: "cause", scope: !163, file: !11, baseType: !86, size: 64, align: 32, offset: 224, flags: DIFlagPrivate)
!168 = !DILocalVariable(name: "exception_param", scope: !169, file: !4, line: 71, type: !121, align: 32)
!169 = distinct !DILexicalBlock(scope: !161, file: !4, line: 71, column: 5)
!170 = !DILocation(line: 61, column: 28, scope: !155)
!171 = !DILocalVariable(name: "addr", arg: 1, scope: !172, file: !173, line: 883, type: !79)
!172 = distinct !DISubprogram(name: "without_provenance<()>", linkageName: "_ZN4core3ptr18without_provenance17h96b55dbad507fa57E", scope: !28, file: !173, line: 883, type: !174, scopeLine: 883, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !179, retainedNodes: !178)
!173 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "8857c34524728cc5887872677b8e1917")
!174 = !DISubroutineType(types: !175)
!175 = !{!176, !79}
!176 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !177, size: 32, align: 32, dwarfAddressSpace: 0)
!177 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!178 = !{!171}
!179 = !{!180}
!180 = !DITemplateTypeParameter(name: "T", type: !177)
!181 = !DILocation(line: 883, column: 36, scope: !172, inlinedAt: !182)
!182 = distinct !DILocation(line: 838, column: 20, scope: !183, inlinedAt: !188)
!183 = distinct !DISubprogram(name: "null<u8>", linkageName: "_ZN4core3ptr4null17h7c14cf324bd4186eE", scope: !28, file: !173, line: 837, type: !184, scopeLine: 837, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !186)
!184 = !DISubroutineType(types: !185)
!185 = !{!144}
!186 = !{!187}
!187 = !DITemplateTypeParameter(name: "T", type: !5)
!188 = distinct !DILocation(line: 66, column: 23, scope: !155)
!189 = !DILocalVariable(name: "addr", arg: 1, scope: !190, file: !173, line: 922, type: !79)
!190 = distinct !DISubprogram(name: "without_provenance_mut<()>", linkageName: "_ZN4core3ptr22without_provenance_mut17hd169848e44db62c7E", scope: !28, file: !173, line: 922, type: !191, scopeLine: 922, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !179, retainedNodes: !194)
!191 = !DISubroutineType(types: !192)
!192 = !{!193, !79}
!193 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut ()", baseType: !177, size: 32, align: 32, dwarfAddressSpace: 0)
!194 = !{!189}
!195 = !DILocation(line: 922, column: 40, scope: !190, inlinedAt: !196)
!196 = distinct !DILocation(line: 884, column: 5, scope: !172, inlinedAt: !182)
!197 = !DILocation(line: 838, column: 5, scope: !183, inlinedAt: !188)
!198 = !DILocation(line: 66, column: 22, scope: !155)
!199 = !DILocation(line: 63, column: 15, scope: !155)
!200 = !DILocation(line: 62, column: 30, scope: !155)
!201 = !DILocalVariable(name: "x", arg: 1, scope: !202, file: !203, line: 260, type: !163)
!202 = distinct !DISubprogram(name: "new<panic_unwind::imp::Exception>", linkageName: "_ZN5alloc5boxed12Box$LT$T$GT$3new17h326702f25dbace84E", scope: !204, file: !203, line: 260, type: !207, scopeLine: 260, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, retainedNodes: !209)
!203 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs", directory: "", checksumkind: CSK_MD5, checksum: "acd6052f47042be3287430901fcaf1c8")
!204 = !DINamespace(name: "{impl#0}", scope: !205)
!205 = !DINamespace(name: "boxed", scope: !206)
!206 = !DINamespace(name: "alloc", scope: null)
!207 = !DISubroutineType(types: !208)
!208 = !{!162, !163}
!209 = !{!201}
!210 = !{!211}
!211 = !DITemplateTypeParameter(name: "T", type: !163)
!212 = !DILocation(line: 260, column: 16, scope: !202, inlinedAt: !213)
!213 = distinct !DILocation(line: 62, column: 21, scope: !155)
!214 = !DILocation(line: 261, column: 16, scope: !202, inlinedAt: !213)
!215 = !DILocation(line: 261, column: 24, scope: !202, inlinedAt: !213)
!216 = !DILocation(line: 62, column: 21, scope: !155)
!217 = !DILocation(line: 62, column: 9, scope: !161)
!218 = !DILocation(line: 71, column: 27, scope: !161)
!219 = !DILocation(line: 71, column: 9, scope: !169)
!220 = !DILocation(line: 72, column: 21, scope: !169)
!221 = !DILocation(line: 83, column: 2, scope: !155)
!222 = distinct !DISubprogram(name: "cleanup", linkageName: "_ZN12panic_unwind3imp7cleanup17hd18d7d8d4f4804a6E", scope: !2, file: !4, line: 85, type: !223, scopeLine: 85, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !225)
!223 = !DISubroutineType(types: !224)
!224 = !{!86, !110}
!225 = !{!226, !227, !229, !232, !234}
!226 = !DILocalVariable(name: "ptr", arg: 1, scope: !222, file: !4, line: 85, type: !110)
!227 = !DILocalVariable(name: "exception", scope: !228, file: !4, line: 87, type: !121, align: 32)
!228 = distinct !DILexicalBlock(scope: !222, file: !4, line: 87, column: 9)
!229 = !DILocalVariable(name: "exception", scope: !230, file: !4, line: 93, type: !231, align: 32)
!230 = distinct !DILexicalBlock(scope: !228, file: !4, line: 93, column: 9)
!231 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut panic_unwind::imp::Exception", baseType: !163, size: 32, align: 32, dwarfAddressSpace: 0)
!232 = !DILocalVariable(name: "canary", scope: !233, file: !4, line: 96, type: !144, align: 32)
!233 = distinct !DILexicalBlock(scope: !230, file: !4, line: 96, column: 9)
!234 = !DILocalVariable(name: "exception", scope: !235, file: !4, line: 105, type: !162, align: 32)
!235 = distinct !DILexicalBlock(scope: !233, file: !4, line: 105, column: 9)
!236 = !DILocation(line: 85, column: 30, scope: !222)
!237 = !DILocation(line: 105, column: 13, scope: !235)
!238 = !DILocation(line: 87, column: 25, scope: !222)
!239 = !DILocation(line: 87, column: 13, scope: !228)
!240 = !DILocation(line: 88, column: 12, scope: !228)
!241 = !DILocalVariable(name: "self", arg: 1, scope: !242, file: !243, line: 31, type: !121)
!242 = distinct !DISubprogram(name: "cast<unwind::wasm::_Unwind_Exception, panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$4cast17h44b23534f257eb99E", scope: !244, file: !243, line: 31, type: !246, scopeLine: 31, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !249, retainedNodes: !248)
!243 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "b0bbe11126e084b85a45fba4c5663912")
!244 = !DINamespace(name: "{impl#0}", scope: !245)
!245 = !DINamespace(name: "mut_ptr", scope: !28)
!246 = !DISubroutineType(types: !247)
!247 = !{!231, !121}
!248 = !{!241}
!249 = !{!250, !251}
!250 = !DITemplateTypeParameter(name: "T", type: !122)
!251 = !DITemplateTypeParameter(name: "U", type: !163)
!252 = !DILocation(line: 31, column: 26, scope: !242, inlinedAt: !253)
!253 = distinct !DILocation(line: 93, column: 35, scope: !228)
!254 = !DILocation(line: 93, column: 35, scope: !228)
!255 = !DILocation(line: 93, column: 13, scope: !230)
!256 = !DILocation(line: 96, column: 22, scope: !230)
!257 = !DILocation(line: 96, column: 55, scope: !230)
!258 = !DILocation(line: 96, column: 13, scope: !233)
!259 = !DILocalVariable(name: "a", arg: 1, scope: !260, file: !173, line: 2446, type: !144)
!260 = distinct !DISubprogram(name: "eq<u8>", linkageName: "_ZN4core3ptr2eq17h82150f34785bbcb6E", scope: !28, file: !173, line: 2446, type: !261, scopeLine: 2446, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !186, retainedNodes: !264)
!261 = !DISubroutineType(types: !262)
!262 = !{!263, !144, !144}
!263 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!264 = !{!259, !265}
!265 = !DILocalVariable(name: "b", arg: 2, scope: !260, file: !173, line: 2446, type: !144)
!266 = !DILocation(line: 2446, column: 28, scope: !260, inlinedAt: !267)
!267 = distinct !DILocation(line: 97, column: 13, scope: !233)
!268 = !DILocation(line: 2446, column: 41, scope: !260, inlinedAt: !267)
!269 = !DILocation(line: 2447, column: 5, scope: !260, inlinedAt: !267)
!270 = !DILocation(line: 97, column: 13, scope: !233)
!271 = !DILocation(line: 89, column: 13, scope: !228)
!272 = !DILocation(line: 90, column: 13, scope: !228)
!273 = !DILocation(line: 102, column: 13, scope: !233)
!274 = !DILocation(line: 105, column: 25, scope: !233)
!275 = !DILocation(line: 106, column: 9, scope: !235)
!276 = !DILocation(line: 107, column: 5, scope: !233)
!277 = !DILocation(line: 108, column: 2, scope: !222)
!278 = distinct !DISubprogram(name: "size_of_val_raw<panic_unwind::imp::Exception>", linkageName: "_ZN4core3mem15size_of_val_raw17hd84cecf7ccb4d68fE", scope: !280, file: !279, line: 418, type: !281, scopeLine: 418, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, retainedNodes: !284)
!279 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "335eb38d3ee0638ae9d68820d69094ad")
!280 = !DINamespace(name: "mem", scope: !29)
!281 = !DISubroutineType(types: !282)
!282 = !{!79, !283}
!283 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const panic_unwind::imp::Exception", baseType: !163, size: 32, align: 32, dwarfAddressSpace: 0)
!284 = !{!285}
!285 = !DILocalVariable(name: "val", arg: 1, scope: !278, file: !279, line: 418, type: !283)
!286 = !DILocation(line: 418, column: 48, scope: !278)
!287 = !DILocation(line: 420, column: 14, scope: !278)
!288 = !DILocation(line: 421, column: 2, scope: !278)
!289 = distinct !DISubprogram(name: "size_of_val_raw<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3mem15size_of_val_raw17hd873f665e55e7c88E", scope: !280, file: !279, line: 418, type: !290, scopeLine: 418, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, retainedNodes: !296)
!290 = !DISubroutineType(types: !291)
!291 = !{!79, !292}
!292 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const (dyn core::any::Any + core::marker::Send)", file: !11, size: 64, align: 32, elements: !293, templateParams: !75, identifier: "413add849b70b39f2b7a0dde88785cea")
!293 = !{!294, !295}
!294 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !292, file: !11, baseType: !89, size: 32, align: 32)
!295 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !292, file: !11, baseType: !92, size: 32, align: 32, offset: 32)
!296 = !{!297}
!297 = !DILocalVariable(name: "val", arg: 1, scope: !289, file: !279, line: 418, type: !292)
!298 = !{!299}
!299 = !DITemplateTypeParameter(name: "T", type: !90)
!300 = !DILocation(line: 418, column: 48, scope: !289)
!301 = !DILocation(line: 420, column: 14, scope: !289)
!302 = !DILocation(line: 421, column: 2, scope: !289)
!303 = distinct !DISubprogram(name: "align_of_val_raw<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3mem16align_of_val_raw17h18382815f45778c9E", scope: !280, file: !279, line: 557, type: !290, scopeLine: 557, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, retainedNodes: !304)
!304 = !{!305}
!305 = !DILocalVariable(name: "val", arg: 1, scope: !303, file: !279, line: 557, type: !292)
!306 = !DILocation(line: 557, column: 49, scope: !303)
!307 = !DILocation(line: 559, column: 14, scope: !303)
!308 = !DILocation(line: 560, column: 2, scope: !303)
!309 = distinct !DISubprogram(name: "align_of_val_raw<panic_unwind::imp::Exception>", linkageName: "_ZN4core3mem16align_of_val_raw17h29d52bec7b897efdE", scope: !280, file: !279, line: 557, type: !281, scopeLine: 557, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, retainedNodes: !310)
!310 = !{!311}
!311 = !DILocalVariable(name: "val", arg: 1, scope: !309, file: !279, line: 557, type: !283)
!312 = !DILocation(line: 557, column: 49, scope: !309)
!313 = !DILocation(line: 559, column: 14, scope: !309)
!314 = !DILocation(line: 560, column: 2, scope: !309)
!315 = distinct !DISubprogram(name: "drop_in_place<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr49drop_in_place$LT$panic_unwind..imp..Exception$GT$17h3a1897ef021f70cfE", scope: !28, file: !173, line: 805, type: !316, scopeLine: 805, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, retainedNodes: !318)
!316 = !DISubroutineType(types: !317)
!317 = !{null, !231}
!318 = !{!319}
!319 = !DILocalVariable(arg: 1, scope: !315, file: !173, line: 805, type: !231)
!320 = !DILocation(line: 805, column: 1, scope: !315)
!321 = distinct !DISubprogram(name: "read<*const u8>", linkageName: "_ZN4core3ptr4read17hcd1815b95cd75f77E", scope: !28, file: !173, line: 1705, type: !322, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !353, retainedNodes: !351)
!322 = !DISubroutineType(types: !323)
!323 = !{!144, !324, !325}
!324 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !144, size: 32, align: 32, dwarfAddressSpace: 0)
!325 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::location::Location", baseType: !326, size: 32, align: 32, dwarfAddressSpace: 0)
!326 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !327, file: !11, size: 128, align: 32, flags: DIFlagPublic, elements: !329, templateParams: !75, identifier: "7c34cafe8ea1dcad4032b9360816105f")
!327 = !DINamespace(name: "location", scope: !328)
!328 = !DINamespace(name: "panic", scope: !29)
!329 = !{!330, !340, !341, !342}
!330 = !DIDerivedType(tag: DW_TAG_member, name: "filename", scope: !326, file: !11, baseType: !331, size: 64, align: 32, flags: DIFlagPrivate)
!331 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<str>", scope: !332, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !333, templateParams: !186, identifier: "88212fc410c4399fd5095990cc8304ca")
!332 = !DINamespace(name: "non_null", scope: !28)
!333 = !{!334}
!334 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !331, file: !11, baseType: !335, size: 64, align: 32, flags: DIFlagPrivate)
!335 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const str", file: !11, size: 64, align: 32, elements: !336, templateParams: !75, identifier: "238a44609877474087c05adf26cd41fa")
!336 = !{!337, !339}
!337 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !335, file: !11, baseType: !338, size: 32, align: 32)
!338 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !5, size: 32, align: 32, dwarfAddressSpace: 0)
!339 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !335, file: !11, baseType: !79, size: 32, align: 32, offset: 32)
!340 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !326, file: !11, baseType: !14, size: 32, align: 32, offset: 64, flags: DIFlagPrivate)
!341 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !326, file: !11, baseType: !14, size: 32, align: 32, offset: 96, flags: DIFlagPrivate)
!342 = !DIDerivedType(tag: DW_TAG_member, name: "_filename", scope: !326, file: !11, baseType: !343, align: 8, offset: 128, flags: DIFlagPrivate)
!343 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&str>", scope: !344, file: !11, align: 8, flags: DIFlagPublic, elements: !75, templateParams: !345, identifier: "4cfc3eea77dd95eabd59051b67bd7e66")
!344 = !DINamespace(name: "marker", scope: !29)
!345 = !{!346}
!346 = !DITemplateTypeParameter(name: "T", type: !347)
!347 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !11, size: 64, align: 32, elements: !348, templateParams: !75, identifier: "9277eecd40495f85161460476aacc992")
!348 = !{!349, !350}
!349 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !347, file: !11, baseType: !338, size: 32, align: 32)
!350 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !347, file: !11, baseType: !79, size: 32, align: 32, offset: 32)
!351 = !{!352}
!352 = !DILocalVariable(name: "src", arg: 1, scope: !321, file: !173, line: 1705, type: !324)
!353 = !{!354}
!354 = !DITemplateTypeParameter(name: "T", type: !144)
!355 = !DILocation(line: 1705, column: 29, scope: !321)
!356 = !DILocation(line: 77, column: 35, scope: !357)
!357 = !DILexicalBlockFile(scope: !321, file: !358, discriminator: 0)
!358 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ub_checks.rs", directory: "", checksumkind: CSK_MD5, checksum: "41b3943b2b7dc8c218ee37ead81b317d")
!359 = !DILocation(line: 1744, column: 9, scope: !321)
!360 = !DILocation(line: 1746, column: 2, scope: !321)
!361 = !DILocation(line: 78, column: 17, scope: !357)
!362 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core3ptr4read18precondition_check17h1c62c99544682c14E", scope: !363, file: !358, line: 68, type: !364, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !366)
!363 = !DINamespace(name: "read", scope: !28)
!364 = !DISubroutineType(types: !365)
!365 = !{null, !176, !79, !263, !325}
!366 = !{!367, !368, !369, !370}
!367 = !DILocalVariable(name: "addr", arg: 1, scope: !362, file: !358, line: 68, type: !176)
!368 = !DILocalVariable(name: "align", arg: 2, scope: !362, file: !358, line: 68, type: !79)
!369 = !DILocalVariable(name: "is_zst", arg: 3, scope: !362, file: !358, line: 68, type: !263)
!370 = !DILocalVariable(name: "msg", scope: !371, file: !358, line: 70, type: !347, align: 32)
!371 = distinct !DILexicalBlock(scope: !362, file: !358, line: 70, column: 21)
!372 = !DILocation(line: 68, column: 43, scope: !362)
!373 = !DILocation(line: 70, column: 25, scope: !371)
!374 = !DILocation(line: 1742, column: 18, scope: !375)
!375 = !DILexicalBlockFile(scope: !362, file: !173, discriminator: 0)
!376 = !DILocation(line: 73, column: 94, scope: !371)
!377 = !DILocation(line: 73, column: 59, scope: !371)
!378 = !DILocation(line: 73, column: 21, scope: !371)
!379 = !DILocation(line: 75, column: 14, scope: !362)
!380 = distinct !DISubprogram(name: "drop_in_place<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr66drop_in_place$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$17h9c94b09f3be9fe7bE", scope: !28, file: !173, line: 805, type: !381, scopeLine: 805, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, retainedNodes: !383)
!381 = !DISubroutineType(types: !382)
!382 = !{null, !106}
!383 = !{!384}
!384 = !DILocalVariable(arg: 1, scope: !380, file: !173, line: 805, type: !106)
!385 = !DILocation(line: 805, column: 1, scope: !380)
!386 = distinct !DISubprogram(name: "new_unchecked<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha82630dde1c4b48aE", scope: !388, file: !387, line: 86, type: !397, scopeLine: 86, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, declaration: !399, retainedNodes: !400)
!387 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/unique.rs", directory: "", checksumkind: CSK_MD5, checksum: "d80c6c81e1bee63c1039a27a0b137ec1")
!388 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unique<panic_unwind::imp::Exception>", scope: !389, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !390, templateParams: !210, identifier: "96b7ed634310132af33fe63ceb27d667")
!389 = !DINamespace(name: "unique", scope: !28)
!390 = !{!391, !395}
!391 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !388, file: !11, baseType: !392, size: 32, align: 32, flags: DIFlagPrivate)
!392 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<panic_unwind::imp::Exception>", scope: !332, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !393, templateParams: !210, identifier: "60719d57d80ece6c3098cfb50b3d8407")
!393 = !{!394}
!394 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !392, file: !11, baseType: !283, size: 32, align: 32, flags: DIFlagPrivate)
!395 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !388, file: !11, baseType: !396, align: 8, offset: 32, flags: DIFlagPrivate)
!396 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<panic_unwind::imp::Exception>", scope: !344, file: !11, align: 8, flags: DIFlagPublic, elements: !75, templateParams: !210, identifier: "a6d594493a0237bb568379bd065fc551")
!397 = !DISubroutineType(types: !398)
!398 = !{!388, !231}
!399 = !DISubprogram(name: "new_unchecked<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17ha82630dde1c4b48aE", scope: !388, file: !387, line: 86, type: !397, scopeLine: 86, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !210)
!400 = !{!401}
!401 = !DILocalVariable(name: "ptr", arg: 1, scope: !386, file: !387, line: 86, type: !231)
!402 = !DILocation(line: 86, column: 39, scope: !386)
!403 = !DILocation(line: 88, column: 36, scope: !386)
!404 = !DILocation(line: 89, column: 6, scope: !386)
!405 = distinct !DISubprogram(name: "new_unchecked<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hf8bbe7c74bad6648E", scope: !406, file: !387, line: 86, type: !414, scopeLine: 86, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, declaration: !416, retainedNodes: !417)
!406 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unique<(dyn core::any::Any + core::marker::Send)>", scope: !389, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !407, templateParams: !298, identifier: "9f41459e521cef518a6106b6b5add8db")
!407 = !{!408, !412}
!408 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !406, file: !11, baseType: !409, size: 64, align: 32, flags: DIFlagPrivate)
!409 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<(dyn core::any::Any + core::marker::Send)>", scope: !332, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !410, templateParams: !298, identifier: "f97311204288a27c139be7682c532b5a")
!410 = !{!411}
!411 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !409, file: !11, baseType: !292, size: 64, align: 32, flags: DIFlagPrivate)
!412 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !406, file: !11, baseType: !413, align: 8, offset: 64, flags: DIFlagPrivate)
!413 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<(dyn core::any::Any + core::marker::Send)>", scope: !344, file: !11, align: 8, flags: DIFlagPublic, elements: !75, templateParams: !298, identifier: "7985c1e550beda8eac56ba78db78b2d6")
!414 = !DISubroutineType(types: !415)
!415 = !{!406, !106}
!416 = !DISubprogram(name: "new_unchecked<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$13new_unchecked17hf8bbe7c74bad6648E", scope: !406, file: !387, line: 86, type: !414, scopeLine: 86, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !298)
!417 = !{!418}
!418 = !DILocalVariable(name: "ptr", arg: 1, scope: !405, file: !387, line: 86, type: !106)
!419 = !DILocation(line: 86, column: 39, scope: !405)
!420 = !DILocation(line: 88, column: 36, scope: !405)
!421 = !DILocation(line: 89, column: 6, scope: !405)
!422 = distinct !DISubprogram(name: "cast<(dyn core::any::Any + core::marker::Send), u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h879afa9afd8f0935E", scope: !406, file: !387, line: 150, type: !423, scopeLine: 150, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !434, declaration: !433, retainedNodes: !436)
!423 = !DISubroutineType(types: !424)
!424 = !{!425, !406}
!425 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unique<u8>", scope: !389, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !426, templateParams: !186, identifier: "15d8b1b660840b97c8698c417bf66080")
!426 = !{!427, !431}
!427 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !425, file: !11, baseType: !428, size: 32, align: 32, flags: DIFlagPrivate)
!428 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<u8>", scope: !332, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !429, templateParams: !186, identifier: "bfbed5a29c49721772982c8bebfc3819")
!429 = !{!430}
!430 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !428, file: !11, baseType: !144, size: 32, align: 32, flags: DIFlagPrivate)
!431 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !425, file: !11, baseType: !432, align: 8, offset: 32, flags: DIFlagPrivate)
!432 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<u8>", scope: !344, file: !11, align: 8, flags: DIFlagPublic, elements: !75, templateParams: !186, identifier: "86f180f8272fce39fed40a1ecf2dfbe2")
!433 = !DISubprogram(name: "cast<(dyn core::any::Any + core::marker::Send), u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h879afa9afd8f0935E", scope: !406, file: !387, line: 150, type: !423, scopeLine: 150, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !434)
!434 = !{!299, !435}
!435 = !DITemplateTypeParameter(name: "U", type: !5)
!436 = !{!437}
!437 = !DILocalVariable(name: "self", arg: 1, scope: !422, file: !387, line: 150, type: !406)
!438 = !DILocation(line: 150, column: 26, scope: !422)
!439 = !DILocation(line: 153, column: 40, scope: !422)
!440 = !DILocation(line: 154, column: 6, scope: !422)
!441 = distinct !DISubprogram(name: "cast<panic_unwind::imp::Exception, u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hf99e8766b267ebecE", scope: !388, file: !387, line: 150, type: !442, scopeLine: 150, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !445, declaration: !444, retainedNodes: !446)
!442 = !DISubroutineType(types: !443)
!443 = !{!425, !388}
!444 = !DISubprogram(name: "cast<panic_unwind::imp::Exception, u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17hf99e8766b267ebecE", scope: !388, file: !387, line: 150, type: !442, scopeLine: 150, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !445)
!445 = !{!211, !435}
!446 = !{!447}
!447 = !DILocalVariable(name: "self", arg: 1, scope: !441, file: !387, line: 150, type: !388)
!448 = !DILocation(line: 150, column: 26, scope: !441)
!449 = !DILocation(line: 153, column: 40, scope: !441)
!450 = !DILocation(line: 154, column: 6, scope: !441)
!451 = distinct !DISubprogram(name: "as_ptr<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h38473301210de45fE", scope: !388, file: !387, line: 110, type: !452, scopeLine: 110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, declaration: !454, retainedNodes: !455)
!452 = !DISubroutineType(types: !453)
!453 = !{!231, !388}
!454 = !DISubprogram(name: "as_ptr<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h38473301210de45fE", scope: !388, file: !387, line: 110, type: !452, scopeLine: 110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !210)
!455 = !{!456}
!456 = !DILocalVariable(name: "self", arg: 1, scope: !451, file: !387, line: 110, type: !388)
!457 = !DILocation(line: 110, column: 25, scope: !451)
!458 = !DILocalVariable(name: "self", arg: 1, scope: !459, file: !460, line: 401, type: !392)
!459 = distinct !DISubprogram(name: "as_ptr<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hca554739a25f2be6E", scope: !392, file: !460, line: 401, type: !461, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, declaration: !463, retainedNodes: !464)
!460 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs", directory: "", checksumkind: CSK_MD5, checksum: "6726e73c6c894eba30d90288586d0f43")
!461 = !DISubroutineType(types: !462)
!462 = !{!231, !392}
!463 = !DISubprogram(name: "as_ptr<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hca554739a25f2be6E", scope: !392, file: !460, line: 401, type: !461, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !210)
!464 = !{!458}
!465 = !DILocation(line: 401, column: 25, scope: !459, inlinedAt: !466)
!466 = distinct !DILocation(line: 111, column: 22, scope: !451)
!467 = !DILocation(line: 112, column: 6, scope: !451)
!468 = distinct !DISubprogram(name: "as_ptr<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17he3585bd48d4ed619E", scope: !406, file: !387, line: 110, type: !469, scopeLine: 110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, declaration: !471, retainedNodes: !472)
!469 = !DISubroutineType(types: !470)
!470 = !{!106, !406}
!471 = !DISubprogram(name: "as_ptr<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17he3585bd48d4ed619E", scope: !406, file: !387, line: 110, type: !469, scopeLine: 110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !298)
!472 = !{!473}
!473 = !DILocalVariable(name: "self", arg: 1, scope: !468, file: !387, line: 110, type: !406)
!474 = !DILocation(line: 110, column: 25, scope: !468)
!475 = !DILocalVariable(name: "self", arg: 1, scope: !476, file: !460, line: 401, type: !409)
!476 = distinct !DISubprogram(name: "as_ptr<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hd5aa8f1048ea8af1E", scope: !409, file: !460, line: 401, type: !477, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, declaration: !479, retainedNodes: !480)
!477 = !DISubroutineType(types: !478)
!478 = !{!106, !409}
!479 = !DISubprogram(name: "as_ptr<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hd5aa8f1048ea8af1E", scope: !409, file: !460, line: 401, type: !477, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !298)
!480 = !{!475}
!481 = !DILocation(line: 401, column: 25, scope: !476, inlinedAt: !482)
!482 = distinct !DILocation(line: 111, column: 22, scope: !468)
!483 = !DILocation(line: 408, column: 6, scope: !476, inlinedAt: !482)
!484 = !DILocation(line: 111, column: 22, scope: !468)
!485 = !DILocation(line: 112, column: 6, scope: !468)
!486 = distinct !DISubprogram(name: "drop_in_place<alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>>", linkageName: "_ZN4core3ptr74drop_in_place$LT$alloc..boxed..Box$LT$panic_unwind..imp..Exception$GT$$GT$17hd1b66519ba331a78E", scope: !28, file: !173, line: 805, type: !487, scopeLine: 805, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !492, retainedNodes: !490)
!487 = !DISubroutineType(types: !488)
!488 = !{null, !489}
!489 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>", baseType: !162, size: 32, align: 32, dwarfAddressSpace: 0)
!490 = !{!491}
!491 = !DILocalVariable(arg: 1, scope: !486, file: !173, line: 805, type: !489)
!492 = !{!493}
!493 = !DITemplateTypeParameter(name: "T", type: !162)
!494 = !DILocation(line: 805, column: 1, scope: !486)
!495 = distinct !DISubprogram(name: "new_unchecked<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h1b87b4c3f1268a7fE", scope: !392, file: !460, line: 233, type: !496, scopeLine: 233, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, declaration: !498, retainedNodes: !499)
!496 = !DISubroutineType(types: !497)
!497 = !{!392, !231, !325}
!498 = !DISubprogram(name: "new_unchecked<panic_unwind::imp::Exception>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h1b87b4c3f1268a7fE", scope: !392, file: !460, line: 233, type: !496, scopeLine: 233, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !210)
!499 = !{!500}
!500 = !DILocalVariable(name: "ptr", arg: 1, scope: !495, file: !460, line: 233, type: !231)
!501 = !DILocation(line: 233, column: 39, scope: !495)
!502 = !DILocation(line: 77, column: 35, scope: !503)
!503 = !DILexicalBlockFile(scope: !495, file: !358, discriminator: 0)
!504 = !DILocation(line: 243, column: 6, scope: !495)
!505 = !DILocation(line: 78, column: 17, scope: !503)
!506 = distinct !DISubprogram(name: "new_unchecked<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h23d448e994bed79aE", scope: !409, file: !460, line: 233, type: !507, scopeLine: 233, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, declaration: !509, retainedNodes: !510)
!507 = !DISubroutineType(types: !508)
!508 = !{!409, !106, !325}
!509 = !DISubprogram(name: "new_unchecked<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h23d448e994bed79aE", scope: !409, file: !460, line: 233, type: !507, scopeLine: 233, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !298)
!510 = !{!511}
!511 = !DILocalVariable(name: "ptr", arg: 1, scope: !506, file: !460, line: 233, type: !106)
!512 = !DILocation(line: 233, column: 39, scope: !506)
!513 = !DILocation(line: 77, column: 35, scope: !514)
!514 = !DILexicalBlockFile(scope: !506, file: !358, discriminator: 0)
!515 = !DILocation(line: 243, column: 6, scope: !506)
!516 = !DILocation(line: 78, column: 17, scope: !514)
!517 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h2ebe651468dffb0fE", scope: !518, file: !358, line: 68, type: !520, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !522)
!518 = !DINamespace(name: "new_unchecked", scope: !519)
!519 = !DINamespace(name: "{impl#3}", scope: !332)
!520 = !DISubroutineType(types: !521)
!521 = !{null, !193, !325}
!522 = !{!523, !524}
!523 = !DILocalVariable(name: "ptr", arg: 1, scope: !517, file: !358, line: 68, type: !193)
!524 = !DILocalVariable(name: "msg", scope: !525, file: !358, line: 70, type: !347, align: 32)
!525 = distinct !DILexicalBlock(scope: !517, file: !358, line: 70, column: 21)
!526 = !DILocation(line: 68, column: 43, scope: !517)
!527 = !DILocation(line: 70, column: 25, scope: !525)
!528 = !DILocation(line: 239, column: 57, scope: !529)
!529 = !DILexicalBlockFile(scope: !517, file: !460, discriminator: 0)
!530 = !DILocation(line: 239, column: 53, scope: !529)
!531 = !DILocation(line: 75, column: 14, scope: !517)
!532 = !DILocation(line: 73, column: 94, scope: !525)
!533 = !DILocation(line: 73, column: 59, scope: !525)
!534 = !DILocation(line: 73, column: 21, scope: !525)
!535 = distinct !DISubprogram(name: "cast<(dyn core::any::Any + core::marker::Send), u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h0b54b3148c34d783E", scope: !409, file: !460, line: 502, type: !536, scopeLine: 502, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !434, declaration: !538, retainedNodes: !539)
!536 = !DISubroutineType(types: !537)
!537 = !{!428, !409}
!538 = !DISubprogram(name: "cast<(dyn core::any::Any + core::marker::Send), u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h0b54b3148c34d783E", scope: !409, file: !460, line: 502, type: !536, scopeLine: 502, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !434)
!539 = !{!540}
!540 = !DILocalVariable(name: "self", arg: 1, scope: !535, file: !460, line: 502, type: !409)
!541 = !DILocation(line: 502, column: 26, scope: !535)
!542 = !DILocation(line: 401, column: 25, scope: !476, inlinedAt: !543)
!543 = distinct !DILocation(line: 504, column: 42, scope: !535)
!544 = !DILocation(line: 408, column: 6, scope: !476, inlinedAt: !543)
!545 = !DILocation(line: 504, column: 42, scope: !535)
!546 = !DILocation(line: 505, column: 6, scope: !535)
!547 = distinct !DISubprogram(name: "cast<panic_unwind::imp::Exception, u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h3d4f9fa96fc7113bE", scope: !392, file: !460, line: 502, type: !548, scopeLine: 502, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !445, declaration: !550, retainedNodes: !551)
!548 = !DISubroutineType(types: !549)
!549 = !{!428, !392}
!550 = !DISubprogram(name: "cast<panic_unwind::imp::Exception, u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h3d4f9fa96fc7113bE", scope: !392, file: !460, line: 502, type: !548, scopeLine: 502, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !445)
!551 = !{!552}
!552 = !DILocalVariable(name: "self", arg: 1, scope: !547, file: !460, line: 502, type: !392)
!553 = !DILocation(line: 502, column: 26, scope: !547)
!554 = !DILocation(line: 401, column: 25, scope: !459, inlinedAt: !555)
!555 = distinct !DILocation(line: 504, column: 42, scope: !547)
!556 = !DILocation(line: 505, column: 6, scope: !547)
!557 = distinct !DISubprogram(name: "drop_in_place<alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>>", linkageName: "_ZN4core3ptr91drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..any..Any$u2b$core..marker..Send$GT$$GT$17h4fd15b138eb3408dE", scope: !28, file: !173, line: 805, type: !558, scopeLine: 805, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !563, retainedNodes: !561)
!558 = !DISubroutineType(types: !559)
!559 = !{null, !560}
!560 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>", baseType: !86, size: 32, align: 32, dwarfAddressSpace: 0)
!561 = !{!562}
!562 = !DILocalVariable(arg: 1, scope: !557, file: !173, line: 805, type: !560)
!563 = !{!564}
!564 = !DITemplateTypeParameter(name: "T", type: !86)
!565 = !DILocation(line: 805, column: 1, scope: !557)
!566 = distinct !DISubprogram(name: "as_nonzero", linkageName: "_ZN4core3ptr9alignment9Alignment10as_nonzero17h031ac1317a479a71E", scope: !568, file: !567, line: 101, type: !571, scopeLine: 101, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, declaration: !584, retainedNodes: !585)
!567 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/alignment.rs", directory: "", checksumkind: CSK_MD5, checksum: "fe5cbe26e2468bf6e2b3ac0bfe05e41a")
!568 = !DICompositeType(tag: DW_TAG_structure_type, name: "Alignment", scope: !27, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !569, templateParams: !75, identifier: "b8055a5301a867de82116acd8d685318")
!569 = !{!570}
!570 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !568, file: !11, baseType: !26, size: 32, align: 32, flags: DIFlagPrivate)
!571 = !DISubroutineType(types: !572)
!572 = !{!573, !568}
!573 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZero<usize>", scope: !574, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !576, templateParams: !582, identifier: "a51ecee75410e49c32f50f2375440820")
!574 = !DINamespace(name: "nonzero", scope: !575)
!575 = !DINamespace(name: "num", scope: !29)
!576 = !{!577}
!577 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !573, file: !11, baseType: !578, size: 32, align: 32, flags: DIFlagPrivate)
!578 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZeroUsizeInner", scope: !579, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !580, templateParams: !75, identifier: "1788621093501d3c2e2adc902d5bca43")
!579 = !DINamespace(name: "niche_types", scope: !575)
!580 = !{!581}
!581 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !578, file: !11, baseType: !79, size: 32, align: 32, flags: DIFlagPrivate)
!582 = !{!583}
!583 = !DITemplateTypeParameter(name: "T", type: !79)
!584 = !DISubprogram(name: "as_nonzero", linkageName: "_ZN4core3ptr9alignment9Alignment10as_nonzero17h031ac1317a479a71E", scope: !568, file: !567, line: 101, type: !571, scopeLine: 101, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !75)
!585 = !{!586}
!586 = !DILocalVariable(name: "self", arg: 1, scope: !566, file: !567, line: 101, type: !568)
!587 = !DILocation(line: 101, column: 29, scope: !566)
!588 = !DILocation(line: 109, column: 6, scope: !566)
!589 = distinct !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17hb36d330e2670920aE", scope: !568, file: !567, line: 94, type: !590, scopeLine: 94, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, declaration: !592, retainedNodes: !593)
!590 = !DISubroutineType(types: !591)
!591 = !{!79, !568}
!592 = !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17hb36d330e2670920aE", scope: !568, file: !567, line: 94, type: !590, scopeLine: 94, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !75)
!593 = !{!594}
!594 = !DILocalVariable(name: "self", arg: 1, scope: !589, file: !567, line: 94, type: !568)
!595 = !DILocation(line: 94, column: 27, scope: !589)
!596 = !DILocation(line: 96, column: 6, scope: !589)
!597 = distinct !DISubprogram(name: "read<*const u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h6763137c6ca81c4dE", scope: !599, file: !598, line: 1166, type: !322, scopeLine: 1166, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !353, retainedNodes: !601)
!598 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "473e695c4e056b47688e2be1785e83b5")
!599 = !DINamespace(name: "{impl#0}", scope: !600)
!600 = !DINamespace(name: "const_ptr", scope: !28)
!601 = !{!602}
!602 = !DILocalVariable(name: "self", arg: 1, scope: !597, file: !598, line: 1166, type: !324)
!603 = !DILocation(line: 1166, column: 30, scope: !597)
!604 = !DILocation(line: 1171, column: 18, scope: !597)
!605 = !DILocation(line: 1172, column: 6, scope: !597)
!606 = distinct !DISubprogram(name: "for_value_raw<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core5alloc6layout6Layout13for_value_raw17h41fcf0e2c2b6f065E", scope: !608, file: !607, line: 220, type: !614, scopeLine: 220, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, declaration: !616, retainedNodes: !617)
!607 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/alloc/layout.rs", directory: "", checksumkind: CSK_MD5, checksum: "f4c671648a78730d8e2bc82acdc17667")
!608 = !DICompositeType(tag: DW_TAG_structure_type, name: "Layout", scope: !609, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !611, templateParams: !75, identifier: "f923cc1896f078e51d4a893c36e2e533")
!609 = !DINamespace(name: "layout", scope: !610)
!610 = !DINamespace(name: "alloc", scope: !29)
!611 = !{!612, !613}
!612 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !608, file: !11, baseType: !79, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!613 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !608, file: !11, baseType: !568, size: 32, align: 32, flags: DIFlagPrivate)
!614 = !DISubroutineType(types: !615)
!615 = !{!608, !292}
!616 = !DISubprogram(name: "for_value_raw<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN4core5alloc6layout6Layout13for_value_raw17h41fcf0e2c2b6f065E", scope: !608, file: !607, line: 220, type: !614, scopeLine: 220, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !298)
!617 = !{!618, !619, !621}
!618 = !DILocalVariable(name: "t", arg: 1, scope: !606, file: !607, line: 220, type: !292)
!619 = !DILocalVariable(name: "size", scope: !620, file: !607, line: 222, type: !79, align: 32)
!620 = distinct !DILexicalBlock(scope: !606, file: !607, line: 222, column: 9)
!621 = !DILocalVariable(name: "align", scope: !620, file: !607, line: 222, type: !79, align: 32)
!622 = !DILocation(line: 220, column: 50, scope: !606)
!623 = !DILocation(line: 222, column: 39, scope: !606)
!624 = !DILocation(line: 222, column: 64, scope: !606)
!625 = !DILocation(line: 222, column: 14, scope: !606)
!626 = !DILocation(line: 222, column: 14, scope: !620)
!627 = !DILocation(line: 222, column: 20, scope: !606)
!628 = !DILocation(line: 222, column: 20, scope: !620)
!629 = !DILocation(line: 224, column: 18, scope: !620)
!630 = !DILocation(line: 225, column: 6, scope: !606)
!631 = distinct !DISubprogram(name: "for_value_raw<panic_unwind::imp::Exception>", linkageName: "_ZN4core5alloc6layout6Layout13for_value_raw17hb269eae829dfffa3E", scope: !608, file: !607, line: 220, type: !632, scopeLine: 220, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, declaration: !634, retainedNodes: !635)
!632 = !DISubroutineType(types: !633)
!633 = !{!608, !283}
!634 = !DISubprogram(name: "for_value_raw<panic_unwind::imp::Exception>", linkageName: "_ZN4core5alloc6layout6Layout13for_value_raw17hb269eae829dfffa3E", scope: !608, file: !607, line: 220, type: !632, scopeLine: 220, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !210)
!635 = !{!636, !637, !639}
!636 = !DILocalVariable(name: "t", arg: 1, scope: !631, file: !607, line: 220, type: !283)
!637 = !DILocalVariable(name: "size", scope: !638, file: !607, line: 222, type: !79, align: 32)
!638 = distinct !DILexicalBlock(scope: !631, file: !607, line: 222, column: 9)
!639 = !DILocalVariable(name: "align", scope: !638, file: !607, line: 222, type: !79, align: 32)
!640 = !DILocation(line: 220, column: 50, scope: !631)
!641 = !DILocation(line: 222, column: 39, scope: !631)
!642 = !DILocation(line: 222, column: 64, scope: !631)
!643 = !DILocation(line: 222, column: 14, scope: !631)
!644 = !DILocation(line: 222, column: 14, scope: !638)
!645 = !DILocation(line: 222, column: 20, scope: !631)
!646 = !DILocation(line: 222, column: 20, scope: !638)
!647 = !DILocation(line: 224, column: 18, scope: !638)
!648 = !DILocation(line: 225, column: 6, scope: !631)
!649 = distinct !DISubprogram(name: "from_size_align_unchecked", linkageName: "_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h3599e2620243ac2fE", scope: !608, file: !607, line: 130, type: !650, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, declaration: !652, retainedNodes: !653)
!650 = !DISubroutineType(types: !651)
!651 = !{!608, !79, !79, !325}
!652 = !DISubprogram(name: "from_size_align_unchecked", linkageName: "_ZN4core5alloc6layout6Layout25from_size_align_unchecked17h3599e2620243ac2fE", scope: !608, file: !607, line: 130, type: !650, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !75)
!653 = !{!654, !655}
!654 = !DILocalVariable(name: "size", arg: 1, scope: !649, file: !607, line: 130, type: !79)
!655 = !DILocalVariable(name: "align", arg: 2, scope: !649, file: !607, line: 130, type: !79)
!656 = !DILocation(line: 130, column: 51, scope: !649)
!657 = !DILocation(line: 130, column: 64, scope: !649)
!658 = !DILocation(line: 77, column: 35, scope: !659)
!659 = !DILexicalBlockFile(scope: !649, file: !358, discriminator: 0)
!660 = !DILocation(line: 78, column: 17, scope: !659)
!661 = !DILocation(line: 142, column: 6, scope: !649)
!662 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17h5a8e2c1454664ab4E", scope: !663, file: !358, line: 68, type: !665, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !667)
!663 = !DINamespace(name: "from_size_align_unchecked", scope: !664)
!664 = !DINamespace(name: "{impl#0}", scope: !609)
!665 = !DISubroutineType(types: !666)
!666 = !{null, !79, !79, !325}
!667 = !{!668, !669, !670}
!668 = !DILocalVariable(name: "size", arg: 1, scope: !662, file: !358, line: 68, type: !79)
!669 = !DILocalVariable(name: "align", arg: 2, scope: !662, file: !358, line: 68, type: !79)
!670 = !DILocalVariable(name: "msg", scope: !671, file: !358, line: 70, type: !347, align: 32)
!671 = distinct !DILexicalBlock(scope: !662, file: !358, line: 70, column: 21)
!672 = !DILocation(line: 68, column: 43, scope: !662)
!673 = !DILocation(line: 70, column: 25, scope: !671)
!674 = !DILocation(line: 138, column: 18, scope: !675)
!675 = !DILexicalBlockFile(scope: !662, file: !607, discriminator: 0)
!676 = !DILocation(line: 73, column: 94, scope: !671)
!677 = !DILocation(line: 73, column: 59, scope: !671)
!678 = !DILocation(line: 73, column: 21, scope: !671)
!679 = !DILocation(line: 75, column: 14, scope: !662)
!680 = distinct !DISubprogram(name: "size", linkageName: "_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E", scope: !608, file: !607, line: 149, type: !681, scopeLine: 149, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, declaration: !684, retainedNodes: !685)
!681 = !DISubroutineType(types: !682)
!682 = !{!79, !683}
!683 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::alloc::layout::Layout", baseType: !608, size: 32, align: 32, dwarfAddressSpace: 0)
!684 = !DISubprogram(name: "size", linkageName: "_ZN4core5alloc6layout6Layout4size17h131e603cdbca7226E", scope: !608, file: !607, line: 149, type: !681, scopeLine: 149, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !75)
!685 = !{!686}
!686 = !DILocalVariable(name: "self", arg: 1, scope: !680, file: !607, line: 149, type: !683)
!687 = !DILocation(line: 149, column: 23, scope: !680)
!688 = !DILocation(line: 150, column: 9, scope: !680)
!689 = !DILocation(line: 151, column: 6, scope: !680)
!690 = distinct !DISubprogram(name: "align", linkageName: "_ZN4core5alloc6layout6Layout5align17h8d2f91b08a22bceaE", scope: !608, file: !607, line: 161, type: !681, scopeLine: 161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, declaration: !691, retainedNodes: !692)
!691 = !DISubprogram(name: "align", linkageName: "_ZN4core5alloc6layout6Layout5align17h8d2f91b08a22bceaE", scope: !608, file: !607, line: 161, type: !681, scopeLine: 161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !75)
!692 = !{!693}
!693 = !DILocalVariable(name: "self", arg: 1, scope: !690, file: !607, line: 161, type: !683)
!694 = !DILocation(line: 161, column: 24, scope: !690)
!695 = !DILocation(line: 162, column: 9, scope: !690)
!696 = !DILocation(line: 162, column: 20, scope: !690)
!697 = !DILocation(line: 163, column: 6, scope: !690)
!698 = distinct !DISubprogram(name: "dangling", linkageName: "_ZN4core5alloc6layout6Layout8dangling17h1c98afe0c3073c3dE", scope: !608, file: !607, line: 236, type: !699, scopeLine: 236, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, declaration: !701, retainedNodes: !702)
!699 = !DISubroutineType(types: !700)
!700 = !{!428, !683}
!701 = !DISubprogram(name: "dangling", linkageName: "_ZN4core5alloc6layout6Layout8dangling17h1c98afe0c3073c3dE", scope: !608, file: !607, line: 236, type: !699, scopeLine: 236, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !75)
!702 = !{!703}
!703 = !DILocalVariable(name: "self", arg: 1, scope: !698, file: !607, line: 236, type: !683)
!704 = !DILocation(line: 236, column: 27, scope: !698)
!705 = !DILocation(line: 237, column: 37, scope: !698)
!706 = !DILocation(line: 237, column: 48, scope: !698)
!707 = !DILocation(line: 237, column: 9, scope: !698)
!708 = !DILocation(line: 238, column: 6, scope: !698)
!709 = distinct !DISubprogram(name: "panic_nounwind_fmt", linkageName: "_ZN4core9panicking18panic_nounwind_fmt17h185a0c9c09e8b976E", scope: !711, file: !710, line: 95, type: !712, scopeLine: 95, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !836)
!710 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/panicking.rs", directory: "", checksumkind: CSK_MD5, checksum: "b120da646d1a09f31201b8a519374e57")
!711 = !DINamespace(name: "panicking", scope: !29)
!712 = !DISubroutineType(types: !713)
!713 = !{null, !714, !263, !325}
!714 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !715, file: !11, size: 192, align: 32, flags: DIFlagPublic, elements: !716, templateParams: !75, identifier: "d691e62b2ee4847c2af32873f04bd10")
!715 = !DINamespace(name: "fmt", scope: !29)
!716 = !{!717, !723, !765}
!717 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !714, file: !11, baseType: !718, size: 64, align: 32, flags: DIFlagPrivate)
!718 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !11, size: 64, align: 32, elements: !719, templateParams: !75, identifier: "4e66b00a376d6af5b8765440fb2839f")
!719 = !{!720, !722}
!720 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !718, file: !11, baseType: !721, size: 32, align: 32)
!721 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !347, size: 32, align: 32, dwarfAddressSpace: 0)
!722 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !718, file: !11, baseType: !79, size: 32, align: 32, offset: 32)
!723 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !714, file: !11, baseType: !724, size: 64, align: 32, offset: 128, flags: DIFlagPrivate)
!724 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !128, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !725, templateParams: !75, identifier: "a638667a460b22fe10961f9a2f3202aa")
!725 = !{!726}
!726 = !DICompositeType(tag: DW_TAG_variant_part, scope: !724, file: !11, size: 64, align: 32, elements: !727, templateParams: !75, identifier: "29af53ccc7f21f4d5671e352d673889a", discriminator: !764)
!727 = !{!728, !760}
!728 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !726, file: !11, baseType: !729, size: 64, align: 32, extraData: i32 0)
!729 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !724, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !75, templateParams: !730, identifier: "11ce4f4d10f67887bbe6bf59a521c479")
!730 = !{!731}
!731 = !DITemplateTypeParameter(name: "T", type: !732)
!732 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !11, size: 64, align: 32, elements: !733, templateParams: !75, identifier: "b0485535d7020130e949c24f3fc2aa00")
!733 = !{!734, !759}
!734 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !732, file: !11, baseType: !735, size: 32, align: 32)
!735 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !736, size: 32, align: 32, dwarfAddressSpace: 0)
!736 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !737, file: !11, size: 192, align: 32, flags: DIFlagPublic, elements: !738, templateParams: !75, identifier: "8cb06f9d78dc629c8f52fc3b5544996c")
!737 = !DINamespace(name: "rt", scope: !715)
!738 = !{!739, !740, !741, !758}
!739 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !736, file: !11, baseType: !79, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
!740 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !736, file: !11, baseType: !14, size: 32, align: 32, offset: 160, flags: DIFlagPublic)
!741 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !736, file: !11, baseType: !742, size: 64, align: 32, flags: DIFlagPublic)
!742 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !737, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !743, templateParams: !75, identifier: "2d7772037f5c744e87d41105441784d5")
!743 = !{!744}
!744 = !DICompositeType(tag: DW_TAG_variant_part, scope: !742, file: !11, size: 64, align: 32, elements: !745, templateParams: !75, identifier: "af14687975a61e1ae6bbcdaeb79a8a2", discriminator: !757)
!745 = !{!746, !751, !755}
!746 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !744, file: !11, baseType: !747, size: 64, align: 32, extraData: i16 0)
!747 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !742, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !748, templateParams: !75, identifier: "da16c9b5356522ffb015c0e99237342e")
!748 = !{!749}
!749 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !747, file: !11, baseType: !750, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!750 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!751 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !744, file: !11, baseType: !752, size: 64, align: 32, extraData: i16 1)
!752 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !742, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !753, templateParams: !75, identifier: "8d84b26eccf0f48fe70ea50c79b83fc9")
!753 = !{!754}
!754 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !752, file: !11, baseType: !79, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!755 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !744, file: !11, baseType: !756, size: 64, align: 32, extraData: i16 2)
!756 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !742, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !75, identifier: "e4d910bcc0c2da0048af65cce9b02bdf")
!757 = !DIDerivedType(tag: DW_TAG_member, scope: !742, file: !11, baseType: !750, size: 16, align: 16, flags: DIFlagArtificial)
!758 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !736, file: !11, baseType: !742, size: 64, align: 32, offset: 64, flags: DIFlagPublic)
!759 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !732, file: !11, baseType: !79, size: 32, align: 32, offset: 32)
!760 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !726, file: !11, baseType: !761, size: 64, align: 32)
!761 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !724, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !762, templateParams: !730, identifier: "b6f59188292a44db7736125146b92cb0")
!762 = !{!763}
!763 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !761, file: !11, baseType: !732, size: 64, align: 32, flags: DIFlagPublic)
!764 = !DIDerivedType(tag: DW_TAG_member, scope: !724, file: !11, baseType: !14, size: 32, align: 32, flags: DIFlagArtificial)
!765 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !714, file: !11, baseType: !766, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!766 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !11, size: 64, align: 32, elements: !767, templateParams: !75, identifier: "14634098cacc86d372c43019bc81f26f")
!767 = !{!768, !835}
!768 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !766, file: !11, baseType: !769, size: 32, align: 32)
!769 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !770, size: 32, align: 32, dwarfAddressSpace: 0)
!770 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !737, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !771, templateParams: !75, identifier: "14dca3c1b1040cd8e8db0eaa112c8216")
!771 = !{!772}
!772 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !770, file: !11, baseType: !773, size: 64, align: 32, flags: DIFlagPrivate)
!773 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !737, file: !11, size: 64, align: 32, flags: DIFlagPrivate, elements: !774, templateParams: !75, identifier: "fb1492950c21086074bab206592842dc")
!774 = !{!775}
!775 = !DICompositeType(tag: DW_TAG_variant_part, scope: !773, file: !11, size: 64, align: 32, elements: !776, templateParams: !75, identifier: "478e018ae6e38e2110d0d424641ab18", discriminator: !834)
!776 = !{!777, !830}
!777 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !775, file: !11, baseType: !778, size: 64, align: 32)
!778 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !773, file: !11, size: 64, align: 32, flags: DIFlagPrivate, elements: !779, templateParams: !75, identifier: "59bc7f5c5a99ab4be3c3f06b9190c327")
!779 = !{!780, !784, !825}
!780 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !778, file: !11, baseType: !781, size: 32, align: 32, flags: DIFlagPrivate)
!781 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !332, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !782, templateParams: !179, identifier: "d9f2bcb64deb934daba9b509aea4a83e")
!782 = !{!783}
!783 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !781, file: !11, baseType: !176, size: 32, align: 32, flags: DIFlagPrivate)
!784 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !778, file: !11, baseType: !785, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!785 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !786, size: 32, align: 32, dwarfAddressSpace: 0)
!786 = !DISubroutineType(types: !787)
!787 = !{!788, !781, !805}
!788 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !789, file: !11, size: 8, align: 8, flags: DIFlagPublic, elements: !790, templateParams: !75, identifier: "613ace46ae0c395d39c31f05d3934750")
!789 = !DINamespace(name: "result", scope: !29)
!790 = !{!791}
!791 = !DICompositeType(tag: DW_TAG_variant_part, scope: !788, file: !11, size: 8, align: 8, elements: !792, templateParams: !75, identifier: "2bd67c77928327a5a86e1d970227dbc3", discriminator: !804)
!792 = !{!793, !800}
!793 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !791, file: !11, baseType: !794, size: 8, align: 8, extraData: i8 0)
!794 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !788, file: !11, size: 8, align: 8, flags: DIFlagPublic, elements: !795, templateParams: !797, identifier: "8e1fa5ea2cd8f77479a16f216aa53a42")
!795 = !{!796}
!796 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !794, file: !11, baseType: !177, align: 8, offset: 8, flags: DIFlagPublic)
!797 = !{!180, !798}
!798 = !DITemplateTypeParameter(name: "E", type: !799)
!799 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !715, file: !11, align: 8, flags: DIFlagPublic, elements: !75, identifier: "cac4d2a6635a122844ffbe3b52a15933")
!800 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !791, file: !11, baseType: !801, size: 8, align: 8, extraData: i8 1)
!801 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !788, file: !11, size: 8, align: 8, flags: DIFlagPublic, elements: !802, templateParams: !797, identifier: "bd8eb8fbb58ca24e2467a7f35c864471")
!802 = !{!803}
!803 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !801, file: !11, baseType: !799, align: 8, offset: 8, flags: DIFlagPublic)
!804 = !DIDerivedType(tag: DW_TAG_member, scope: !788, file: !11, baseType: !5, size: 8, align: 8, flags: DIFlagArtificial)
!805 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !806, size: 32, align: 32, dwarfAddressSpace: 0)
!806 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !715, file: !11, size: 128, align: 32, flags: DIFlagPublic, elements: !807, templateParams: !75, identifier: "9c19c8ef0b5ae3cad350e741e841742c")
!807 = !{!808, !814}
!808 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !806, file: !11, baseType: !809, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!809 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !715, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !810, templateParams: !75, identifier: "8e7d20540a73fe2190308d0618721e3e")
!810 = !{!811, !812, !813}
!811 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !809, file: !11, baseType: !14, size: 32, align: 32, flags: DIFlagPrivate)
!812 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !809, file: !11, baseType: !750, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!813 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !809, file: !11, baseType: !750, size: 16, align: 16, offset: 48, flags: DIFlagPrivate)
!814 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !806, file: !11, baseType: !815, size: 64, align: 32, flags: DIFlagPrivate)
!815 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !11, size: 64, align: 32, elements: !816, templateParams: !75, identifier: "ed1fc41b72305de4afb5dbb44887680d")
!816 = !{!817, !820}
!817 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !815, file: !11, baseType: !818, size: 32, align: 32)
!818 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !819, size: 32, align: 32, dwarfAddressSpace: 0)
!819 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !11, align: 8, elements: !75, identifier: "3bd7022d6bc7a1bba9386a42dfa7db9d")
!820 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !815, file: !11, baseType: !821, size: 32, align: 32, offset: 32)
!821 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !822, size: 32, align: 32, dwarfAddressSpace: 0)
!822 = !DICompositeType(tag: DW_TAG_array_type, baseType: !79, size: 192, align: 32, elements: !823)
!823 = !{!824}
!824 = !DISubrange(count: 6, lowerBound: 0)
!825 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !778, file: !11, baseType: !826, align: 8, offset: 64, flags: DIFlagPrivate)
!826 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !344, file: !11, align: 8, flags: DIFlagPublic, elements: !75, templateParams: !827, identifier: "e71ee38df7dbfccdae82d3411c10d5bc")
!827 = !{!828}
!828 = !DITemplateTypeParameter(name: "T", type: !829)
!829 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !177, size: 32, align: 32, dwarfAddressSpace: 0)
!830 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !775, file: !11, baseType: !831, size: 64, align: 32, extraData: i32 0)
!831 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !773, file: !11, size: 64, align: 32, flags: DIFlagPrivate, elements: !832, templateParams: !75, identifier: "bcc61db69ea5777ac138ac099ea396b2")
!832 = !{!833}
!833 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !831, file: !11, baseType: !750, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!834 = !DIDerivedType(tag: DW_TAG_member, scope: !773, file: !11, baseType: !14, size: 32, align: 32, flags: DIFlagArtificial)
!835 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !766, file: !11, baseType: !79, size: 32, align: 32, offset: 32)
!836 = !{!837, !838}
!837 = !DILocalVariable(name: "fmt", arg: 1, scope: !709, file: !710, line: 95, type: !714)
!838 = !DILocalVariable(name: "force_no_backtrace", arg: 2, scope: !709, file: !710, line: 95, type: !263)
!839 = !DILocation(line: 95, column: 33, scope: !709)
!840 = !DILocation(line: 95, column: 58, scope: !709)
!841 = !DILocation(line: 2435, column: 27, scope: !842)
!842 = !DILexicalBlockFile(scope: !709, file: !843, discriminator: 0)
!843 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "5088527a679dbab229c7a43df7f388f7")
!844 = !DILocation(line: 2435, column: 9, scope: !842)
!845 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9panicking18panic_nounwind_fmt7runtime17haa28c113f74a5518E", scope: !846, file: !843, line: 2423, type: !712, scopeLine: 2423, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !847)
!846 = !DINamespace(name: "panic_nounwind_fmt", scope: !711)
!847 = !{!848, !849, !850}
!848 = !DILocalVariable(name: "fmt", arg: 1, scope: !845, file: !843, line: 2423, type: !714)
!849 = !DILocalVariable(name: "force_no_backtrace", arg: 2, scope: !845, file: !843, line: 2423, type: !263)
!850 = !DILocalVariable(name: "pi", scope: !851, file: !710, line: 114, type: !852, align: 32)
!851 = distinct !DILexicalBlock(scope: !845, file: !710, line: 114, column: 13)
!852 = !DICompositeType(tag: DW_TAG_structure_type, name: "PanicInfo", scope: !853, file: !11, size: 96, align: 32, flags: DIFlagPublic, elements: !854, templateParams: !75, identifier: "74943ad5cfeaa8d7c3439d6f603267a6")
!853 = !DINamespace(name: "panic_info", scope: !328)
!854 = !{!855, !857, !858, !859}
!855 = !DIDerivedType(tag: DW_TAG_member, name: "message", scope: !852, file: !11, baseType: !856, size: 32, align: 32, flags: DIFlagPrivate)
!856 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::Arguments", baseType: !714, size: 32, align: 32, dwarfAddressSpace: 0)
!857 = !DIDerivedType(tag: DW_TAG_member, name: "location", scope: !852, file: !11, baseType: !325, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!858 = !DIDerivedType(tag: DW_TAG_member, name: "can_unwind", scope: !852, file: !11, baseType: !263, size: 8, align: 8, offset: 64, flags: DIFlagPrivate)
!859 = !DIDerivedType(tag: DW_TAG_member, name: "force_no_backtrace", scope: !852, file: !11, baseType: !263, size: 8, align: 8, offset: 72, flags: DIFlagPrivate)
!860 = !DILocation(line: 2423, column: 40, scope: !845)
!861 = !DILocation(line: 103, column: 17, scope: !862)
!862 = !DILexicalBlockFile(scope: !845, file: !710, discriminator: 0)
!863 = distinct !DISubprogram(name: "maybe_is_aligned", linkageName: "_ZN4core9ub_checks16maybe_is_aligned17h6c66c7270555ff73E", scope: !864, file: !358, line: 135, type: !865, scopeLine: 135, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !867)
!864 = !DINamespace(name: "ub_checks", scope: !29)
!865 = !DISubroutineType(types: !866)
!866 = !{!263, !176, !79}
!867 = !{!868, !869}
!868 = !DILocalVariable(name: "ptr", arg: 1, scope: !863, file: !358, line: 135, type: !176)
!869 = !DILocalVariable(name: "align", arg: 2, scope: !863, file: !358, line: 135, type: !79)
!870 = !DILocation(line: 135, column: 38, scope: !863)
!871 = !DILocation(line: 135, column: 54, scope: !863)
!872 = !DILocation(line: 2435, column: 9, scope: !873)
!873 = !DILexicalBlockFile(scope: !863, file: !843, discriminator: 0)
!874 = !DILocation(line: 145, column: 2, scope: !863)
!875 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9ub_checks16maybe_is_aligned7runtime17hd98e47256eeccaa1E", scope: !876, file: !843, line: 2423, type: !865, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !877)
!876 = !DINamespace(name: "maybe_is_aligned", scope: !864)
!877 = !{!878, !879}
!878 = !DILocalVariable(name: "ptr", arg: 1, scope: !875, file: !843, line: 2423, type: !176)
!879 = !DILocalVariable(name: "align", arg: 2, scope: !875, file: !843, line: 2423, type: !79)
!880 = !DILocation(line: 2423, column: 40, scope: !875)
!881 = !DILocation(line: 142, column: 17, scope: !882)
!882 = !DILexicalBlockFile(scope: !875, file: !358, discriminator: 0)
!883 = !DILocation(line: 2425, column: 10, scope: !875)
!884 = distinct !DISubprogram(name: "check_language_ub", linkageName: "_ZN4core9ub_checks17check_language_ub17h7d3209090a219533E", scope: !864, file: !358, line: 96, type: !885, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75)
!885 = !DISubroutineType(types: !886)
!886 = !{!263}
!887 = !DILocation(line: 98, column: 5, scope: !884)
!888 = !DILocation(line: 2435, column: 9, scope: !889)
!889 = !DILexicalBlockFile(scope: !884, file: !843, discriminator: 0)
!890 = !DILocation(line: 109, column: 2, scope: !884)
!891 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9ub_checks17check_language_ub7runtime17h8ff65f2b3cf59f9fE", scope: !892, file: !843, line: 2423, type: !885, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75)
!892 = !DINamespace(name: "check_language_ub", scope: !864)
!893 = !DILocation(line: 2425, column: 10, scope: !891)
!894 = distinct !DISubprogram(name: "maybe_is_aligned_and_not_null", linkageName: "_ZN4core9ub_checks29maybe_is_aligned_and_not_null17ha3a1dc64a976744dE", scope: !864, file: !358, line: 119, type: !895, scopeLine: 119, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !897)
!895 = !DISubroutineType(types: !896)
!896 = !{!263, !176, !79, !263}
!897 = !{!898, !899, !900}
!898 = !DILocalVariable(name: "ptr", arg: 1, scope: !894, file: !358, line: 120, type: !176)
!899 = !DILocalVariable(name: "align", arg: 2, scope: !894, file: !358, line: 121, type: !79)
!900 = !DILocalVariable(name: "is_zst", arg: 3, scope: !894, file: !358, line: 122, type: !263)
!901 = !DILocation(line: 120, column: 5, scope: !894)
!902 = !DILocation(line: 121, column: 5, scope: !894)
!903 = !DILocation(line: 122, column: 5, scope: !894)
!904 = !DILocation(line: 125, column: 5, scope: !894)
!905 = !DILocation(line: 125, column: 38, scope: !894)
!906 = !DILocation(line: 126, column: 2, scope: !894)
!907 = !DILocation(line: 125, column: 53, scope: !894)
!908 = !DILocation(line: 125, column: 48, scope: !894)
!909 = !DILocation(line: 125, column: 37, scope: !894)
!910 = distinct !DISubprogram(name: "alloc_zeroed", linkageName: "_ZN5alloc5alloc12alloc_zeroed17h625859128e9e5166E", scope: !912, file: !911, line: 172, type: !913, scopeLine: 172, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !915)
!911 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs", directory: "", checksumkind: CSK_MD5, checksum: "be3434980e1d4fd44eed88225c1bb8c1")
!912 = !DINamespace(name: "alloc", scope: !206)
!913 = !DISubroutineType(types: !914)
!914 = !{!110, !608}
!915 = !{!916}
!916 = !DILocalVariable(name: "layout", arg: 1, scope: !910, file: !911, line: 172, type: !608)
!917 = !DILocation(line: 172, column: 28, scope: !910)
!918 = !DILocation(line: 176, column: 9, scope: !910)
!919 = !DILocation(line: 178, column: 36, scope: !910)
!920 = !DILocation(line: 178, column: 51, scope: !910)
!921 = !DILocation(line: 178, column: 9, scope: !910)
!922 = !DILocation(line: 180, column: 2, scope: !910)
!923 = distinct !DISubprogram(name: "exchange_malloc", linkageName: "_ZN5alloc5alloc15exchange_malloc17h5d053562fbeb5adbE", scope: !912, file: !911, line: 350, type: !924, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !926)
!924 = !DISubroutineType(types: !925)
!925 = !{!110, !79, !79}
!926 = !{!927, !928, !929, !931}
!927 = !DILocalVariable(name: "size", arg: 1, scope: !923, file: !911, line: 350, type: !79)
!928 = !DILocalVariable(name: "align", arg: 2, scope: !923, file: !911, line: 350, type: !79)
!929 = !DILocalVariable(name: "layout", scope: !930, file: !911, line: 351, type: !608, align: 32)
!930 = distinct !DILexicalBlock(scope: !923, file: !911, line: 351, column: 5)
!931 = !DILocalVariable(name: "ptr", scope: !932, file: !911, line: 353, type: !933, align: 32)
!932 = distinct !DILexicalBlock(scope: !930, file: !911, line: 353, column: 9)
!933 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<[u8]>", scope: !332, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !934, templateParams: !186, identifier: "63afe4dc267db9c7c31bcf5dce3dc5fa")
!934 = !{!935}
!935 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !933, file: !11, baseType: !936, size: 64, align: 32, flags: DIFlagPrivate)
!936 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const [u8]", file: !11, size: 64, align: 32, elements: !937, templateParams: !75, identifier: "a10360edaf335c418dbc95bccd0cb05d")
!937 = !{!938, !939}
!938 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !936, file: !11, baseType: !338, size: 32, align: 32)
!939 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !936, file: !11, baseType: !79, size: 32, align: 32, offset: 32)
!940 = !DILocation(line: 350, column: 27, scope: !923)
!941 = !DILocation(line: 350, column: 40, scope: !923)
!942 = !DILocation(line: 351, column: 27, scope: !923)
!943 = !DILocation(line: 351, column: 9, scope: !930)
!944 = !DILocation(line: 352, column: 18, scope: !930)
!945 = !DILocation(line: 352, column: 11, scope: !930)
!946 = !DILocation(line: 352, column: 5, scope: !930)
!947 = !DILocation(line: 354, column: 19, scope: !930)
!948 = !DILocation(line: 353, column: 12, scope: !930)
!949 = !DILocation(line: 353, column: 12, scope: !932)
!950 = !DILocation(line: 353, column: 24, scope: !932)
!951 = !DILocation(line: 356, column: 2, scope: !923)
!952 = distinct !DISubprogram(name: "alloc", linkageName: "_ZN5alloc5alloc5alloc17h8f42680c0ce0b0ceE", scope: !912, file: !911, line: 89, type: !913, scopeLine: 89, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !953)
!953 = !{!954}
!954 = !DILocalVariable(name: "layout", arg: 1, scope: !952, file: !911, line: 89, type: !608)
!955 = !DILocation(line: 89, column: 21, scope: !952)
!956 = !DILocation(line: 93, column: 9, scope: !952)
!957 = !DILocation(line: 95, column: 29, scope: !952)
!958 = !DILocation(line: 95, column: 44, scope: !952)
!959 = !DILocation(line: 95, column: 9, scope: !952)
!960 = !DILocation(line: 97, column: 2, scope: !952)
!961 = distinct !DISubprogram(name: "alloc_impl", linkageName: "_ZN5alloc5alloc6Global10alloc_impl17h2f24deec91cfa471E", scope: !962, file: !911, line: 185, type: !963, scopeLine: 185, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, declaration: !983, retainedNodes: !984)
!962 = !DICompositeType(tag: DW_TAG_structure_type, name: "Global", scope: !912, file: !11, align: 8, flags: DIFlagPublic, elements: !75, identifier: "8d3dc7eb6b91fe30566bfc073f6fd293")
!963 = !DISubroutineType(types: !964)
!964 = !{!965, !982, !608, !263}
!965 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>", scope: !789, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !966, templateParams: !75, identifier: "621f4c248d48cb85b1bb5a959d70dd40")
!966 = !{!967}
!967 = !DICompositeType(tag: DW_TAG_variant_part, scope: !965, file: !11, size: 64, align: 32, elements: !968, templateParams: !75, identifier: "ab7319af1bd471edff13bdf368ae1b34", discriminator: !981)
!968 = !{!969, !977}
!969 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !967, file: !11, baseType: !970, size: 64, align: 32)
!970 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !965, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !971, templateParams: !973, identifier: "bdd19ecc44e3eba4b2d06a28df1b5e8f")
!971 = !{!972}
!972 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !970, file: !11, baseType: !933, size: 64, align: 32, flags: DIFlagPublic)
!973 = !{!974, !975}
!974 = !DITemplateTypeParameter(name: "T", type: !933)
!975 = !DITemplateTypeParameter(name: "E", type: !976)
!976 = !DICompositeType(tag: DW_TAG_structure_type, name: "AllocError", scope: !610, file: !11, align: 8, flags: DIFlagPublic, elements: !75, identifier: "aa948b6092f32f441a138171efeb38d8")
!977 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !967, file: !11, baseType: !978, size: 64, align: 32, extraData: i32 0)
!978 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !965, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !979, templateParams: !973, identifier: "5fac8b00a5770b0ad82a49c2a1d697c9")
!979 = !{!980}
!980 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !978, file: !11, baseType: !976, align: 8, flags: DIFlagPublic)
!981 = !DIDerivedType(tag: DW_TAG_member, scope: !965, file: !11, baseType: !14, size: 32, align: 32, flags: DIFlagArtificial)
!982 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::alloc::Global", baseType: !962, size: 32, align: 32, dwarfAddressSpace: 0)
!983 = !DISubprogram(name: "alloc_impl", linkageName: "_ZN5alloc5alloc6Global10alloc_impl17h2f24deec91cfa471E", scope: !962, file: !911, line: 185, type: !963, scopeLine: 185, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !75)
!984 = !{!985, !986, !987, !988, !990, !992, !994, !1014}
!985 = !DILocalVariable(name: "self", arg: 1, scope: !961, file: !911, line: 185, type: !982)
!986 = !DILocalVariable(name: "layout", arg: 2, scope: !961, file: !911, line: 185, type: !608)
!987 = !DILocalVariable(name: "zeroed", arg: 3, scope: !961, file: !911, line: 185, type: !263)
!988 = !DILocalVariable(name: "size", scope: !989, file: !911, line: 189, type: !79, align: 32)
!989 = distinct !DILexicalBlock(scope: !961, file: !911, line: 189, column: 13)
!990 = !DILocalVariable(name: "raw_ptr", scope: !991, file: !911, line: 190, type: !110, align: 32)
!991 = distinct !DILexicalBlock(scope: !989, file: !911, line: 190, column: 17)
!992 = !DILocalVariable(name: "ptr", scope: !993, file: !911, line: 191, type: !428, align: 32)
!993 = distinct !DILexicalBlock(scope: !991, file: !911, line: 191, column: 17)
!994 = !DILocalVariable(name: "residual", scope: !995, file: !911, line: 191, type: !996, align: 8)
!995 = distinct !DILexicalBlock(scope: !991, file: !911, line: 191, column: 66)
!996 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::convert::Infallible, core::alloc::AllocError>", scope: !789, file: !11, align: 8, flags: DIFlagPublic, elements: !997, templateParams: !75, identifier: "f2b62bba2cc68ba85f1737fc67e67")
!997 = !{!998}
!998 = !DICompositeType(tag: DW_TAG_variant_part, scope: !996, file: !11, align: 8, elements: !999, templateParams: !75, identifier: "7099e72a404ce2923d6aef035e9b730d")
!999 = !{!1000, !1010}
!1000 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !998, file: !11, baseType: !1001, align: 8)
!1001 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !996, file: !11, align: 8, flags: DIFlagPublic, elements: !1002, templateParams: !1008, identifier: "85c8127fba82f59227ddd5a6dfce6ad6")
!1002 = !{!1003}
!1003 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1001, file: !11, baseType: !1004, align: 8, flags: DIFlagPublic)
!1004 = !DICompositeType(tag: DW_TAG_structure_type, name: "Infallible", scope: !1005, file: !11, align: 8, flags: DIFlagPublic, elements: !1006, templateParams: !75, identifier: "bbec56e295cb17a3c6590c058bc34564")
!1005 = !DINamespace(name: "convert", scope: !29)
!1006 = !{!1007}
!1007 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1004, file: !11, align: 8, elements: !75, identifier: "54bd9ba32f82ed48b888ad889b266af3")
!1008 = !{!1009, !975}
!1009 = !DITemplateTypeParameter(name: "T", type: !1004)
!1010 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !998, file: !11, baseType: !1011, align: 8)
!1011 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !996, file: !11, align: 8, flags: DIFlagPublic, elements: !1012, templateParams: !1008, identifier: "94f2a836cc75269a6e57ee3b2319be7e")
!1012 = !{!1013}
!1013 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1011, file: !11, baseType: !976, align: 8, flags: DIFlagPublic)
!1014 = !DILocalVariable(name: "val", scope: !1015, file: !911, line: 191, type: !428, align: 32)
!1015 = distinct !DILexicalBlock(scope: !991, file: !911, line: 191, column: 27)
!1016 = !DILocation(line: 185, column: 19, scope: !961)
!1017 = !DILocation(line: 185, column: 26, scope: !961)
!1018 = !DILocation(line: 185, column: 42, scope: !961)
!1019 = !DILocation(line: 190, column: 21, scope: !991)
!1020 = !DILocation(line: 191, column: 66, scope: !995)
!1021 = !DILocation(line: 186, column: 22, scope: !961)
!1022 = !DILocation(line: 189, column: 13, scope: !989)
!1023 = !DILocation(line: 186, column: 9, scope: !961)
!1024 = !DILocation(line: 187, column: 58, scope: !961)
!1025 = !DILocation(line: 187, column: 21, scope: !961)
!1026 = !DILocation(line: 187, column: 18, scope: !961)
!1027 = !DILocation(line: 187, column: 72, scope: !961)
!1028 = !DILocation(line: 190, column: 34, scope: !989)
!1029 = !DILocation(line: 195, column: 6, scope: !961)
!1030 = !DILocation(line: 190, column: 73, scope: !989)
!1031 = !DILocation(line: 190, column: 43, scope: !989)
!1032 = !DILocation(line: 191, column: 40, scope: !991)
!1033 = !DILocation(line: 191, column: 27, scope: !991)
!1034 = !DILocation(line: 191, column: 49, scope: !991)
!1035 = !DILocation(line: 191, column: 27, scope: !995)
!1036 = !DILocation(line: 191, column: 21, scope: !993)
!1037 = !DILocation(line: 191, column: 27, scope: !1015)
!1038 = !DILocation(line: 192, column: 20, scope: !993)
!1039 = !DILocation(line: 192, column: 17, scope: !993)
!1040 = !DILocation(line: 193, column: 13, scope: !961)
!1041 = distinct !DISubprogram(name: "dealloc", linkageName: "_ZN5alloc5alloc7dealloc17hb7357e680edfb9d7E", scope: !912, file: !911, line: 114, type: !1042, scopeLine: 114, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !1044)
!1042 = !DISubroutineType(types: !1043)
!1043 = !{null, !110, !608}
!1044 = !{!1045, !1046}
!1045 = !DILocalVariable(name: "ptr", arg: 1, scope: !1041, file: !911, line: 114, type: !110)
!1046 = !DILocalVariable(name: "layout", arg: 2, scope: !1041, file: !911, line: 114, type: !608)
!1047 = !DILocation(line: 114, column: 23, scope: !1041)
!1048 = !DILocation(line: 114, column: 37, scope: !1041)
!1049 = !DILocation(line: 115, column: 41, scope: !1041)
!1050 = !DILocation(line: 115, column: 56, scope: !1041)
!1051 = !DILocation(line: 115, column: 14, scope: !1041)
!1052 = !DILocation(line: 116, column: 2, scope: !1041)
!1053 = distinct !DISubprogram(name: "from_raw<panic_unwind::imp::Exception>", linkageName: "_ZN5alloc5boxed12Box$LT$T$GT$8from_raw17h787409b512d29d17E", scope: !1054, file: !203, line: 1063, type: !1055, scopeLine: 1063, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, retainedNodes: !1057)
!1054 = !DINamespace(name: "{impl#6}", scope: !205)
!1055 = !DISubroutineType(types: !1056)
!1056 = !{!162, !231}
!1057 = !{!1058}
!1058 = !DILocalVariable(name: "raw", arg: 1, scope: !1053, file: !203, line: 1063, type: !231)
!1059 = !DILocation(line: 1063, column: 28, scope: !1053)
!1060 = !DILocation(line: 1064, column: 18, scope: !1053)
!1061 = !DILocation(line: 1065, column: 6, scope: !1053)
!1062 = distinct !DISubprogram(name: "from_raw<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN5alloc5boxed12Box$LT$T$GT$8from_raw17hdae822c5d06d5e51E", scope: !1054, file: !203, line: 1063, type: !1063, scopeLine: 1063, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, retainedNodes: !1065)
!1063 = !DISubroutineType(types: !1064)
!1064 = !{!86, !106}
!1065 = !{!1066}
!1066 = !DILocalVariable(name: "raw", arg: 1, scope: !1062, file: !203, line: 1063, type: !106)
!1067 = !DILocation(line: 1063, column: 28, scope: !1062)
!1068 = !DILocation(line: 1064, column: 18, scope: !1062)
!1069 = !DILocation(line: 1065, column: 6, scope: !1062)
!1070 = distinct !DISubprogram(name: "into_raw<panic_unwind::imp::Exception>", linkageName: "_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17h7cdb6077d8e206e4E", scope: !1054, file: !203, line: 1171, type: !1071, scopeLine: 1171, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !210, retainedNodes: !1073)
!1071 = !DISubroutineType(types: !1072)
!1072 = !{!231, !162}
!1073 = !{!1074, !1075}
!1074 = !DILocalVariable(name: "b", arg: 1, scope: !1070, file: !203, line: 1171, type: !162)
!1075 = !DILocalVariable(name: "b", scope: !1076, file: !203, line: 1173, type: !1077, align: 32)
!1076 = distinct !DILexicalBlock(scope: !1070, file: !203, line: 1173, column: 9)
!1077 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>>", scope: !1078, file: !11, size: 32, align: 32, flags: DIFlagPublic, elements: !1079, templateParams: !492, identifier: "ed69d2297e69e6009bb0948e5b367a14")
!1078 = !DINamespace(name: "manually_drop", scope: !280)
!1079 = !{!1080}
!1080 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1077, file: !11, baseType: !162, size: 32, align: 32, flags: DIFlagPrivate)
!1081 = !DILocation(line: 1171, column: 21, scope: !1070)
!1082 = !DILocation(line: 1173, column: 13, scope: !1076)
!1083 = !DILocalVariable(name: "value", arg: 1, scope: !1084, file: !1085, line: 181, type: !162)
!1084 = distinct !DISubprogram(name: "new<alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hb2d31c6349d1f472E", scope: !1077, file: !1085, line: 181, type: !1086, scopeLine: 181, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !492, declaration: !1088, retainedNodes: !1089)
!1085 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs", directory: "", checksumkind: CSK_MD5, checksum: "cb93188e9fe8eda8268775a56e071ba3")
!1086 = !DISubroutineType(types: !1087)
!1087 = !{!1077, !162}
!1088 = !DISubprogram(name: "new<alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hb2d31c6349d1f472E", scope: !1077, file: !1085, line: 181, type: !1086, scopeLine: 181, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !492)
!1089 = !{!1083}
!1090 = !DILocation(line: 181, column: 22, scope: !1084, inlinedAt: !1091)
!1091 = distinct !DILocation(line: 1173, column: 21, scope: !1070)
!1092 = !DILocation(line: 1173, column: 21, scope: !1070)
!1093 = !DILocalVariable(name: "self", arg: 1, scope: !1094, file: !1085, line: 279, type: !1099)
!1094 = distinct !DISubprogram(name: "deref_mut<alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>>", linkageName: "_ZN94_$LT$core..mem..manually_drop..ManuallyDrop$LT$T$GT$$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h65080f4b13031015E", scope: !1095, file: !1085, line: 279, type: !1096, scopeLine: 279, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !492, retainedNodes: !1100)
!1095 = !DINamespace(name: "{impl#3}", scope: !1078)
!1096 = !DISubroutineType(types: !1097)
!1097 = !{!1098, !1099}
!1098 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>", baseType: !162, size: 32, align: 32, dwarfAddressSpace: 0)
!1099 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::manually_drop::ManuallyDrop<alloc::boxed::Box<panic_unwind::imp::Exception, alloc::alloc::Global>>", baseType: !1077, size: 32, align: 32, dwarfAddressSpace: 0)
!1100 = !{!1093}
!1101 = !DILocation(line: 279, column: 18, scope: !1094, inlinedAt: !1102)
!1102 = distinct !DILocation(line: 1176, column: 19, scope: !1076)
!1103 = !DILocation(line: 1176, column: 9, scope: !1076)
!1104 = !DILocation(line: 1177, column: 6, scope: !1070)
!1105 = distinct !DISubprogram(name: "into_raw<(dyn core::any::Any + core::marker::Send)>", linkageName: "_ZN5alloc5boxed12Box$LT$T$GT$8into_raw17hd58a19267a876565E", scope: !1054, file: !203, line: 1171, type: !1106, scopeLine: 1171, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !298, retainedNodes: !1108)
!1106 = !DISubroutineType(types: !1107)
!1107 = !{!106, !86}
!1108 = !{!1109, !1110}
!1109 = !DILocalVariable(name: "b", arg: 1, scope: !1105, file: !203, line: 1171, type: !86)
!1110 = !DILocalVariable(name: "b", scope: !1111, file: !203, line: 1173, type: !1112, align: 32)
!1111 = distinct !DILexicalBlock(scope: !1105, file: !203, line: 1173, column: 9)
!1112 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>>", scope: !1078, file: !11, size: 64, align: 32, flags: DIFlagPublic, elements: !1113, templateParams: !563, identifier: "876e39af349050b21144821d1b57311")
!1113 = !{!1114}
!1114 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1112, file: !11, baseType: !86, size: 64, align: 32, flags: DIFlagPrivate)
!1115 = !DILocation(line: 1171, column: 21, scope: !1105)
!1116 = !DILocation(line: 1173, column: 13, scope: !1111)
!1117 = !DILocalVariable(name: "value", arg: 1, scope: !1118, file: !1085, line: 181, type: !86)
!1118 = distinct !DISubprogram(name: "new<alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hf7348f98ba47fc5aE", scope: !1112, file: !1085, line: 181, type: !1119, scopeLine: 181, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !563, declaration: !1121, retainedNodes: !1122)
!1119 = !DISubroutineType(types: !1120)
!1120 = !{!1112, !86}
!1121 = !DISubprogram(name: "new<alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$3new17hf7348f98ba47fc5aE", scope: !1112, file: !1085, line: 181, type: !1119, scopeLine: 181, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !563)
!1122 = !{!1117}
!1123 = !DILocation(line: 181, column: 22, scope: !1118, inlinedAt: !1124)
!1124 = distinct !DILocation(line: 1173, column: 21, scope: !1105)
!1125 = !DILocation(line: 183, column: 6, scope: !1118, inlinedAt: !1124)
!1126 = !DILocation(line: 1173, column: 21, scope: !1105)
!1127 = !DILocalVariable(name: "self", arg: 1, scope: !1128, file: !1085, line: 279, type: !1132)
!1128 = distinct !DISubprogram(name: "deref_mut<alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>>", linkageName: "_ZN94_$LT$core..mem..manually_drop..ManuallyDrop$LT$T$GT$$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17hff98795833f29788E", scope: !1095, file: !1085, line: 279, type: !1129, scopeLine: 279, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !563, retainedNodes: !1133)
!1129 = !DISubroutineType(types: !1130)
!1130 = !{!1131, !1132}
!1131 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>", baseType: !86, size: 32, align: 32, dwarfAddressSpace: 0)
!1132 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::manually_drop::ManuallyDrop<alloc::boxed::Box<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>>", baseType: !1112, size: 32, align: 32, dwarfAddressSpace: 0)
!1133 = !{!1127}
!1134 = !DILocation(line: 279, column: 18, scope: !1128, inlinedAt: !1135)
!1135 = distinct !DILocation(line: 1176, column: 19, scope: !1111)
!1136 = !DILocation(line: 1176, column: 9, scope: !1111)
!1137 = !DILocation(line: 1177, column: 6, scope: !1105)
!1138 = distinct !DISubprogram(name: "from_raw_in<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>", linkageName: "_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h2f3a84ec7149a747E", scope: !1139, file: !203, line: 1290, type: !1140, scopeLine: 1290, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !1145, retainedNodes: !1142)
!1139 = !DINamespace(name: "{impl#7}", scope: !205)
!1140 = !DISubroutineType(types: !1141)
!1141 = !{!86, !106, !962}
!1142 = !{!1143, !1144}
!1143 = !DILocalVariable(name: "raw", arg: 1, scope: !1138, file: !203, line: 1290, type: !106)
!1144 = !DILocalVariable(name: "alloc", arg: 2, scope: !1138, file: !203, line: 1290, type: !962)
!1145 = !{!299, !1146}
!1146 = !DITemplateTypeParameter(name: "A", type: !962)
!1147 = !DILocation(line: 1290, column: 31, scope: !1138)
!1148 = !DILocation(line: 1290, column: 44, scope: !1138)
!1149 = !DILocation(line: 1291, column: 22, scope: !1138)
!1150 = !DILocation(line: 1292, column: 6, scope: !1138)
!1151 = distinct !DISubprogram(name: "from_raw_in<panic_unwind::imp::Exception, alloc::alloc::Global>", linkageName: "_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h70f6ff0daf4789b4E", scope: !1139, file: !203, line: 1290, type: !1152, scopeLine: 1290, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !1157, retainedNodes: !1154)
!1152 = !DISubroutineType(types: !1153)
!1153 = !{!162, !231, !962}
!1154 = !{!1155, !1156}
!1155 = !DILocalVariable(name: "raw", arg: 1, scope: !1151, file: !203, line: 1290, type: !231)
!1156 = !DILocalVariable(name: "alloc", arg: 2, scope: !1151, file: !203, line: 1290, type: !962)
!1157 = !{!211, !1146}
!1158 = !DILocation(line: 1290, column: 31, scope: !1151)
!1159 = !DILocation(line: 1290, column: 44, scope: !1151)
!1160 = !DILocation(line: 1291, column: 22, scope: !1151)
!1161 = !DILocation(line: 1292, column: 6, scope: !1151)
!1162 = distinct !DISubprogram(name: "deallocate", linkageName: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h4cc2aa304c89386fE", scope: !1163, file: !911, line: 262, type: !1164, scopeLine: 262, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !1166)
!1163 = !DINamespace(name: "{impl#1}", scope: !912)
!1164 = !DISubroutineType(types: !1165)
!1165 = !{null, !982, !428, !608}
!1166 = !{!1167, !1168, !1169}
!1167 = !DILocalVariable(name: "self", arg: 1, scope: !1162, file: !911, line: 262, type: !982)
!1168 = !DILocalVariable(name: "ptr", arg: 2, scope: !1162, file: !911, line: 262, type: !428)
!1169 = !DILocalVariable(name: "layout", arg: 3, scope: !1162, file: !911, line: 262, type: !608)
!1170 = !DILocation(line: 262, column: 26, scope: !1162)
!1171 = !DILocation(line: 262, column: 33, scope: !1162)
!1172 = !DILocation(line: 262, column: 51, scope: !1162)
!1173 = !DILocation(line: 263, column: 19, scope: !1162)
!1174 = !DILocation(line: 263, column: 12, scope: !1162)
!1175 = !DILocation(line: 274, column: 6, scope: !1162)
!1176 = !DILocalVariable(name: "self", arg: 1, scope: !1177, file: !460, line: 401, type: !428)
!1177 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h042f185a9d7f514dE", scope: !428, file: !460, line: 401, type: !1178, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !186, declaration: !1180, retainedNodes: !1181)
!1178 = !DISubroutineType(types: !1179)
!1179 = !{!110, !428}
!1180 = !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h042f185a9d7f514dE", scope: !428, file: !460, line: 401, type: !1178, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !186)
!1181 = !{!1176}
!1182 = !DILocation(line: 401, column: 25, scope: !1177, inlinedAt: !1183)
!1183 = distinct !DILocation(line: 272, column: 34, scope: !1162)
!1184 = !DILocation(line: 272, column: 22, scope: !1162)
!1185 = distinct !DISubprogram(name: "allocate", linkageName: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h51875d913a88c707E", scope: !1163, file: !911, line: 250, type: !1186, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !75, retainedNodes: !1188)
!1186 = !DISubroutineType(types: !1187)
!1187 = !{!965, !982, !608}
!1188 = !{!1189, !1190}
!1189 = !DILocalVariable(name: "self", arg: 1, scope: !1185, file: !911, line: 250, type: !982)
!1190 = !DILocalVariable(name: "layout", arg: 2, scope: !1185, file: !911, line: 250, type: !608)
!1191 = !DILocation(line: 250, column: 17, scope: !1185)
!1192 = !DILocation(line: 250, column: 24, scope: !1185)
!1193 = !DILocation(line: 251, column: 14, scope: !1185)
!1194 = !DILocation(line: 252, column: 6, scope: !1185)
!1195 = distinct !DISubprogram(name: "drop<panic_unwind::imp::Exception, alloc::alloc::Global>", linkageName: "_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9f990ada96c0379fE", scope: !1196, file: !203, line: 1677, type: !1197, scopeLine: 1677, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !1157, retainedNodes: !1199)
!1196 = !DINamespace(name: "{impl#8}", scope: !205)
!1197 = !DISubroutineType(types: !1198)
!1198 = !{null, !1098}
!1199 = !{!1200, !1201, !1203}
!1200 = !DILocalVariable(name: "self", arg: 1, scope: !1195, file: !203, line: 1677, type: !1098)
!1201 = !DILocalVariable(name: "ptr", scope: !1202, file: !203, line: 1680, type: !388, align: 32)
!1202 = distinct !DILexicalBlock(scope: !1195, file: !203, line: 1680, column: 9)
!1203 = !DILocalVariable(name: "layout", scope: !1204, file: !203, line: 1683, type: !608, align: 32)
!1204 = distinct !DILexicalBlock(scope: !1202, file: !203, line: 1683, column: 13)
!1205 = !DILocation(line: 1677, column: 13, scope: !1195)
!1206 = !DILocation(line: 1683, column: 17, scope: !1204)
!1207 = !DILocation(line: 1680, column: 19, scope: !1195)
!1208 = !DILocation(line: 1680, column: 13, scope: !1202)
!1209 = !DILocation(line: 1683, column: 52, scope: !1202)
!1210 = !DILocation(line: 1683, column: 26, scope: !1202)
!1211 = !DILocation(line: 1684, column: 23, scope: !1204)
!1212 = !DILocation(line: 1684, column: 16, scope: !1204)
!1213 = !DILocation(line: 1688, column: 6, scope: !1195)
!1214 = !DILocation(line: 1685, column: 17, scope: !1204)
!1215 = !DILocation(line: 1685, column: 50, scope: !1204)
!1216 = !DILocation(line: 1685, column: 35, scope: !1204)
!1217 = !DILocation(line: 1685, column: 24, scope: !1204)
!1218 = distinct !DISubprogram(name: "drop<(dyn core::any::Any + core::marker::Send), alloc::alloc::Global>", linkageName: "_ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfcf1c84a7897640aE", scope: !1196, file: !203, line: 1677, type: !1219, scopeLine: 1677, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !7, templateParams: !1145, retainedNodes: !1221)
!1219 = !DISubroutineType(types: !1220)
!1220 = !{null, !1131}
!1221 = !{!1222, !1223, !1225}
!1222 = !DILocalVariable(name: "self", arg: 1, scope: !1218, file: !203, line: 1677, type: !1131)
!1223 = !DILocalVariable(name: "ptr", scope: !1224, file: !203, line: 1680, type: !406, align: 32)
!1224 = distinct !DILexicalBlock(scope: !1218, file: !203, line: 1680, column: 9)
!1225 = !DILocalVariable(name: "layout", scope: !1226, file: !203, line: 1683, type: !608, align: 32)
!1226 = distinct !DILexicalBlock(scope: !1224, file: !203, line: 1683, column: 13)
!1227 = !DILocation(line: 1677, column: 13, scope: !1218)
!1228 = !DILocation(line: 1683, column: 17, scope: !1226)
!1229 = !DILocation(line: 1680, column: 19, scope: !1218)
!1230 = !DILocation(line: 1680, column: 13, scope: !1224)
!1231 = !DILocation(line: 1683, column: 52, scope: !1224)
!1232 = !DILocation(line: 1683, column: 26, scope: !1224)
!1233 = !DILocation(line: 1684, column: 23, scope: !1226)
!1234 = !DILocation(line: 1684, column: 16, scope: !1226)
!1235 = !DILocation(line: 1688, column: 6, scope: !1218)
!1236 = !DILocation(line: 1685, column: 17, scope: !1226)
!1237 = !DILocation(line: 1685, column: 50, scope: !1226)
!1238 = !DILocation(line: 1685, column: 35, scope: !1226)
!1239 = !DILocation(line: 1685, column: 24, scope: !1226)

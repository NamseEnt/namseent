; ModuleID = 'hello_world.b326d140966cd182-cgu.0'
source_filename = "hello_world.b326d140966cd182-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

@vtable.0 = private unnamed_addr constant <{ [12 x i8], ptr, ptr, ptr }> <{ [12 x i8] c"\00\00\00\00\04\00\00\00\04\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he48851c116189f1aE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef07beedc0148da2E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef07beedc0148da2E" }>, align 4
@anon.f52434a2809397b1abb34e52430ce470.0 = private unnamed_addr constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4
@alloc_3213114faf700a46436312d7d5d956d1 = private unnamed_addr constant [14 x i8] c"Hello, world!\0A", align 1
@alloc_9b968e9d68758268e4a8d45e405f65d0 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_3213114faf700a46436312d7d5d956d1, [4 x i8] c"\0E\00\00\00" }>, align 4

; std::rt::lang_start
; Function Attrs: nounwind
define hidden i32 @_ZN3std2rt10lang_start17h7700b2f3503e1ed4E(ptr %main, i32 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [4 x i8], align 4
  store ptr %main, ptr %_7, align 4
; call std::rt::lang_start_internal
  %_0 = call i32 @_ZN3std2rt19lang_start_internal17h9224f8262f833227E(ptr align 1 %_7, ptr align 4 @vtable.0, i32 %argc, ptr %argv, i8 %sigpipe) #4
  ret i32 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef07beedc0148da2E"(ptr align 4 %_1) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %_1, align 4
; call std::sys::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hfce07c24d14b4404E(ptr %_4) #4
; call <() as std::process::Termination>::report
  %self = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7bb77f510928e009E"() #4
  %_0 = zext i8 %self to i32
  ret i32 %_0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nounwind
define internal void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hfce07c24d14b4404E(ptr %f) unnamed_addr #2 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h8c48ca95caea87f5E(ptr %f) #4
  call void asm sideeffect "", "~{memory}"(), !srcloc !1
  ret void
}

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3a30e415830240daE"(ptr sret([24 x i8]) align 4 %_0, ptr align 4 %pieces) unnamed_addr #1 {
start:
  store ptr %pieces, ptr %_0, align 4
  %0 = getelementptr inbounds i8, ptr %_0, i32 4
  store i32 1, ptr %0, align 4
  %1 = load ptr, ptr @anon.f52434a2809397b1abb34e52430ce470.0, align 4
  %2 = load i32, ptr getelementptr inbounds (i8, ptr @anon.f52434a2809397b1abb34e52430ce470.0, i32 4), align 4
  %3 = getelementptr inbounds i8, ptr %_0, i32 16
  store ptr %1, ptr %3, align 4
  %4 = getelementptr inbounds i8, ptr %3, i32 4
  store i32 %2, ptr %4, align 4
  %5 = getelementptr inbounds i8, ptr %_0, i32 8
  store ptr inttoptr (i32 4 to ptr), ptr %5, align 4
  %6 = getelementptr inbounds i8, ptr %5, i32 4
  store i32 0, ptr %6, align 4
  ret void
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he48851c116189f1aE"(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  %0 = load ptr, ptr %_1, align 4
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h4554aeccf940ab42E(ptr %0) #4
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h4554aeccf940ab42E(ptr %0) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [4 x i8], align 4
  store ptr %0, ptr %_1, align 4
; call std::rt::lang_start::{{closure}}
  %_0 = call i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef07beedc0148da2E"(ptr align 4 %_1) #4
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ops8function6FnOnce9call_once17h8c48ca95caea87f5E(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  call void %_1() #4
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nounwind
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7bb77f510928e009E"() unnamed_addr #1 {
start:
  ret i8 0
}

; hello_world::main
; Function Attrs: nounwind
define hidden void @_ZN11hello_world4main17h64a202c57002d659E() unnamed_addr #0 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17h3a30e415830240daE"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_9b968e9d68758268e4a8d45e405f65d0) #4
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h71059b9ed4cc355dE(ptr align 4 %_2) #4
  ret void
}

; std::rt::lang_start_internal
; Function Attrs: nounwind
declare dso_local i32 @_ZN3std2rt19lang_start_internal17h9224f8262f833227E(ptr align 1, ptr align 4, i32, ptr, i8) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: nounwind
declare dso_local void @_ZN3std2io5stdio6_print17h71059b9ed4cc355dE(ptr align 4) unnamed_addr #0

define i32 @__main_void() unnamed_addr #3 {
top:
; call std::rt::lang_start
  %0 = call i32 @_ZN3std2rt10lang_start17h7700b2f3503e1ed4E(ptr @_ZN11hello_world4main17h64a202c57002d659E, i32 0, ptr null, i8 0)
  ret i32 %0
}

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { inlinehint nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { noinline nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #3 = { "target-cpu"="generic" }
attributes #4 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.90.0 (1159e78c4 2025-09-14)"}
!1 = !{i64 4458090154940025}

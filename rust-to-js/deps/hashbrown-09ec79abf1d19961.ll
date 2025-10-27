; ModuleID = 'hashbrown.c44395b665ae0048-cgu.0'
source_filename = "hashbrown.c44395b665ae0048-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

@alloc_d70fe56c8b72f00119cb15c86956890a = private unnamed_addr constant [110 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/mod.rs\00", align 1
@alloc_d3ea8a12337a4535ec15ce1eda77c8dd = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_d70fe56c8b72f00119cb15c86956890a, [12 x i8] c"m\00\00\00\DA\04\00\00\05\00\00\00" }>, align 4
@alloc_0d97bde9555861f1f32215c1f3ce127d = private unnamed_addr constant [110 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs\00", align 1
@alloc_15a52d1a884c78a5de92a2463d39823d = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_0d97bde9555861f1f32215c1f3ce127d, [12 x i8] c"m\00\00\00\0F\02\00\00\05\00\00\00" }>, align 4
@alloc_bd3468a7b96187f70c1ce98a3e7a63bf = private unnamed_addr constant [283 x i8] c"unsafe precondition(s) violated: ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null and the specified memory ranges do not overlap\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_ed8641ebea8e5515740d4eb49a916ff5 = private unnamed_addr constant [218 x i8] c"unsafe precondition(s) violated: ptr::read requires that the pointer argument is aligned and non-null\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_4fb4eca1f8b9d0ded0407faa6b2654bb = private unnamed_addr constant [214 x i8] c"unsafe precondition(s) violated: ptr::add requires that the address calculation does not overflow\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_5c1a2f972552229672fc942406cfc298 = private unnamed_addr constant [283 x i8] c"unsafe precondition(s) violated: slice::from_raw_parts_mut requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_5bd1ef6667dbdbecff436d9509a4d052 = private unnamed_addr constant [25 x i8] c"attempt to divide by zero", align 1
@alloc_2ca80fe829e7dcbb4661228c202cce92 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_5bd1ef6667dbdbecff436d9509a4d052, [4 x i8] c"\19\00\00\00" }>, align 4
@alloc_7eec19d3416e3575f65ca3f5643d5a84 = private unnamed_addr constant [28 x i8] c"attempt to add with overflow", align 1
@alloc_491fd71eacc9ac6df50464189817658a = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_7eec19d3416e3575f65ca3f5643d5a84, [4 x i8] c"\1C\00\00\00" }>, align 4
@alloc_1482d7dc779f3e37c63e31d28eda18b3 = private unnamed_addr constant [33 x i8] c"attempt to multiply with overflow", align 1
@alloc_3a541098c7af55f2d1b57c8374ee944e = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_1482d7dc779f3e37c63e31d28eda18b3, [4 x i8] c"!\00\00\00" }>, align 4
@alloc_9f8e76db6a71f2507809208910629059 = private unnamed_addr constant [35 x i8] c"attempt to shift left with overflow", align 1
@alloc_26eab6319fe0d02af8105663e6a2ea8b = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_9f8e76db6a71f2507809208910629059, [4 x i8] c"#\00\00\00" }>, align 4
@alloc_53b7c2e50e2cec3eca67f872cc62c959 = private unnamed_addr constant [36 x i8] c"attempt to shift right with overflow", align 1
@alloc_0f75c28593fb3281511a86ba9b3adf6f = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_53b7c2e50e2cec3eca67f872cc62c959, [4 x i8] c"$\00\00\00" }>, align 4
@alloc_4668a6a56031a745778990d4b3d270b1 = private unnamed_addr constant [33 x i8] c"attempt to subtract with overflow", align 1
@alloc_7daa13c2a11e2a3dbea9e2a29716d6f6 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_4668a6a56031a745778990d4b3d270b1, [4 x i8] c"!\00\00\00" }>, align 4
@alloc_504e26f1fa554fb0e7ec5c33ab8b9f26 = private unnamed_addr constant [112 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/panicking.rs\00", align 1
@alloc_55a1350f0592d90727796c17fe69030d = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_504e26f1fa554fb0e7ec5c33ab8b9f26, [12 x i8] c"o\00\00\00\E6\00\00\00\05\00\00\00" }>, align 4
@alloc_763310d78c99c2c1ad3f8a9821e942f3 = private unnamed_addr constant [61 x i8] c"is_nonoverlapping: `size_of::<T>() * count` overflows a usize", align 1
@alloc_7a037f755386427ed47af072316856f7 = private unnamed_addr constant [112 x i8] c"/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ub_checks.rs\00", align 1
@alloc_329dec4fe38a59083c3b039c87a8d615 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_7a037f755386427ed47af072316856f7, [12 x i8] c"o\00\00\00\95\00\00\006\00\00\00" }>, align 4
@alloc_3c6431c5fc85ca277eb9f2e0ebb30f52 = private unnamed_addr constant [4 x i8] c"full", align 1
@vtable.0 = private constant <{ [12 x i8], ptr }> <{ [12 x i8] c"\00\00\00\00\01\00\00\00\01\00\00\00", ptr @"_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17he1d4dd70e2b1af78E" }>, align 4, !dbg !0
@alloc_4d692e00cdd6193df669076a38c2cf3f = private unnamed_addr constant [7 x i8] c"DELETED", align 1
@alloc_5ecff085f33eeeacaf38ec6a4e5e2caf = private unnamed_addr constant [5 x i8] c"EMPTY", align 1
@alloc_51a94285e41bcfbe97ee0effb00a2208 = private unnamed_addr constant [98 x i8] c"/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5/src/raw/mod.rs\00", align 1
@alloc_56e3b117833fff2746fd8cc9c4deec48 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00\1C\10\00\00!\00\00\00" }>, align 4
@alloc_ead00003f9c8c7813d7aa60141196161 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00'\10\00\00'\00\00\00" }>, align 4
@alloc_a19464d52a327f94e09214e6632c4e75 = private unnamed_addr constant [61 x i8] c"assertion failed: index < self.bucket_mask + 1 + Group::WIDTH", align 1
@alloc_a2af0b7b77c88948576fb01caff77de4 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00'\10\00\00\11\00\00\00" }>, align 4
@alloc_bfaee7eedbdee0e37a4947f3f8a057e5 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00(\10\00\005\00\00\00" }>, align 4
@alloc_70fa19237e466d7d1e23911705586cf6 = private unnamed_addr constant [28 x i8] c"Hash table capacity overflow", align 1
@alloc_b0acc26568cfe85a22c804c4e29c0e28 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_70fa19237e466d7d1e23911705586cf6, [4 x i8] c"\1C\00\00\00" }>, align 4
@alloc_b7c0248fcba32176f5ad13cb0b6fc9ed = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00%\00\00\00(\00\00\00" }>, align 4
@alloc_33b2c073cf1eee2d5a4556da9ce40539 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00\D5\09\00\00\12\00\00\00" }>, align 4
@alloc_c2961626c79d8147cae5a8a0428ebaf1 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00\EA\09\00\00\09\00\00\00" }>, align 4
@alloc_d4ba273af71dec0b6121025ad15b0934 = private unnamed_addr constant [47 x i8] c"assertion failed: index < self.num_ctrl_bytes()", align 1
@alloc_f727df41d5c68d57374e6b6dd87eb7c6 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00\CD\09\00\00\09\00\00\00" }>, align 4
@alloc_08178e3705f6cb88f90b8c2db9b287d5 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00\CF\09\00\00\1C\00\00\00" }>, align 4
@alloc_58d775e767df59a4a584ad4e47672c04 = private unnamed_addr constant [31 x i8] c"Went past end of probe sequence", align 1
@alloc_2d3a64d43c9f7190b430f73d81e3ba6f = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_58d775e767df59a4a584ad4e47672c04, [4 x i8] c"\1F\00\00\00" }>, align 4
@alloc_8a92bc86d0cd940ed16717c77a01a2d3 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00U\00\00\00\09\00\00\00" }>, align 4
@alloc_700e6d87b4783c87b3010586a7a02ce7 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00Z\00\00\00\09\00\00\00" }>, align 4
@alloc_5407fcb00837cebb06d4c6a70ab2ff30 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_51a94285e41bcfbe97ee0effb00a2208, [12 x i8] c"a\00\00\00[\00\00\00\09\00\00\00" }>, align 4
@alloc_415d34d1a55ce06d02b8d5c94fa2ff39 = private unnamed_addr constant [35 x i8] c"assertion failed: self.is_special()", align 1
@alloc_d9a32744d9d4116f544eab2cc1631903 = private unnamed_addr constant [102 x i8] c"/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5/src/control/tag.rs\00", align 1
@alloc_9168bccc6b35eb246e9e8c9f797cb996 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_d9a32744d9d4116f544eab2cc1631903, [12 x i8] c"e\00\00\00\1D\00\00\00\09\00\00\00" }>, align 4
@alloc_623a68bf9f8ce9878e65dbae43be02d8 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_d9a32744d9d4116f544eab2cc1631903, [12 x i8] c"e\00\00\000\00\00\00\1D\00\00\00" }>, align 4
@alloc_e773017bdfe8f67da92e8b6a7810af7e = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_d9a32744d9d4116f544eab2cc1631903, [12 x i8] c"e\00\00\000\00\00\00\1C\00\00\00" }>, align 4
@alloc_a62a01c8c371fa30a0eec53cfad7d3e9 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_d9a32744d9d4116f544eab2cc1631903, [12 x i8] c"e\00\00\000\00\00\00\14\00\00\00" }>, align 4
@alloc_278634111cebbc1e4810d5f40af9ce34 = private unnamed_addr constant [112 x i8] c"/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5/src/control/group/generic.rs\00", align 1
@alloc_054d4fbbe178366ea5a2c31d76957b35 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_278634111cebbc1e4810d5f40af9ce34, [12 x i8] c"o\00\00\00K\00\00\00\0F\00\00\00" }>, align 4
@alloc_f809886b3e16c06d09f95d24f4717b77 = private unnamed_addr constant [106 x i8] c"/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5/src/control/bitmask.rs\00", align 1
@alloc_1dbe95295766279777bc500d62bb029e = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_f809886b3e16c06d09f95d24f4717b77, [12 x i8] c"i\00\00\00&\00\00\00\1A\00\00\00" }>, align 4

; core::intrinsics::cold_path
; Function Attrs: cold nounwind
define internal void @_ZN4core10intrinsics9cold_path17hf94df2e82664e0a2E() unnamed_addr #0 !dbg !65 {
start:
  ret void, !dbg !70
}

; core::fmt::num::<impl core::fmt::Debug for u8>::fmt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17he1d4dd70e2b1af78E"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #1 !dbg !71 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !119, !DIExpression(), !121)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !120, !DIExpression(), !122)
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h72b54bf2b5971ea0E(ptr align 4 %f) #9, !dbg !123
  br i1 %_3, label %bb2, label %bb3, !dbg !124

bb3:                                              ; preds = %start
; call core::fmt::Formatter::debug_upper_hex
  %_5 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hda8089ad17629515E(ptr align 4 %f) #9, !dbg !125
  br i1 %_5, label %bb5, label %bb6, !dbg !126

bb2:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for u8>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u8$GT$3fmt17hf19d67508dd58d4aE"(ptr align 1 %self, ptr align 4 %f) #9, !dbg !127
  %1 = zext i1 %0 to i8, !dbg !127
  store i8 %1, ptr %_0, align 1, !dbg !127
  br label %bb7, !dbg !127

bb6:                                              ; preds = %bb3
; call core::fmt::num::imp::<impl core::fmt::Display for u8>::fmt
  %2 = call zeroext i1 @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf2721191f040b59aE"(ptr align 1 %self, ptr align 4 %f) #9, !dbg !128
  %3 = zext i1 %2 to i8, !dbg !128
  store i8 %3, ptr %_0, align 1, !dbg !128
  br label %bb7, !dbg !128

bb5:                                              ; preds = %bb3
; call core::fmt::num::<impl core::fmt::UpperHex for u8>::fmt
  %4 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h3a28df7a448ec4e8E"(ptr align 1 %self, ptr align 4 %f) #9, !dbg !129
  %5 = zext i1 %4 to i8, !dbg !129
  store i8 %5, ptr %_0, align 1, !dbg !129
  br label %bb7, !dbg !129

bb7:                                              ; preds = %bb2, %bb5, %bb6
  %6 = load i8, ptr %_0, align 1, !dbg !130
  %7 = trunc nuw i8 %6 to i1, !dbg !130
  ret i1 %7, !dbg !130
}

; core::num::<impl i32>::unsigned_abs
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core3num21_$LT$impl$u20$i32$GT$12unsigned_abs17hddd60634a183b5cbE"(i32 %self) unnamed_addr #1 !dbg !131 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store i32 %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !139, !DIExpression(), !140)
; call core::num::<impl i32>::wrapping_abs
  %_2 = call i32 @"_ZN4core3num21_$LT$impl$u20$i32$GT$12wrapping_abs17ha537e37ea8fb8fadE"(i32 %self) #9, !dbg !141
  ret i32 %_2, !dbg !142
}

; core::num::<impl i32>::wrapping_abs
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core3num21_$LT$impl$u20$i32$GT$12wrapping_abs17ha537e37ea8fb8fadE"(i32 %self) unnamed_addr #1 !dbg !143 {
start:
  %rhs.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill.i3 = alloca [4 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 4
  store i32 %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !147, !DIExpression(), !148)
  store i32 %self, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !149, !DIExpression(), !155)
  %_0.i = icmp slt i32 %self, 0, !dbg !157
  br i1 %_0.i, label %bb2, label %bb3, !dbg !158

bb3:                                              ; preds = %start
  store i32 %self, ptr %_0, align 4, !dbg !159
  br label %bb4, !dbg !160

bb2:                                              ; preds = %start
  store i32 %self, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !161, !DIExpression(), !164)
  store i32 0, ptr %self.dbg.spill.i3, align 4
    #dbg_declare(ptr %self.dbg.spill.i3, !166, !DIExpression(), !172)
  store i32 %self, ptr %rhs.dbg.spill.i, align 4
    #dbg_declare(ptr %rhs.dbg.spill.i, !171, !DIExpression(), !174)
  %_0.i4 = sub i32 0, %self, !dbg !175
  store i32 %_0.i4, ptr %_0, align 4, !dbg !176
  br label %bb4, !dbg !176

bb4:                                              ; preds = %bb2, %bb3
  %0 = load i32, ptr %_0, align 4, !dbg !177
  ret i32 %0, !dbg !177
}

; core::num::<impl u64>::from_ne_bytes
; Function Attrs: inlinehint nounwind
define internal i64 @"_ZN4core3num21_$LT$impl$u20$u64$GT$13from_ne_bytes17ha6d065afd5223f55E"(ptr align 1 %bytes) unnamed_addr #1 !dbg !178 {
start:
    #dbg_declare(ptr %bytes, !188, !DIExpression(), !189)
  %_0 = load i64, ptr %bytes, align 1, !dbg !190
  ret i64 %_0, !dbg !191
}

; core::num::<impl usize>::checked_mul
; Function Attrs: inlinehint nounwind
define internal { i32, i32 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h489bb426cb3f2bbbE"(i32 %self, i32 %rhs) unnamed_addr #1 !dbg !192 {
start:
  %b.dbg.spill.i1 = alloca [1 x i8], align 1
  %a.dbg.spill.i = alloca [4 x i8], align 4
  %rhs.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %b.dbg.spill.i = alloca [1 x i8], align 1
  %_0.i = alloca [1 x i8], align 1
  %b.dbg.spill = alloca [1 x i8], align 1
  %a.dbg.spill = alloca [4 x i8], align 4
  %rhs.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store i32 %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !211, !DIExpression(), !216)
  store i32 %rhs, ptr %rhs.dbg.spill, align 4
    #dbg_declare(ptr %rhs.dbg.spill, !212, !DIExpression(), !217)
  store i32 %self, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !218, !DIExpression(), !231)
  store i32 %rhs, ptr %rhs.dbg.spill.i, align 4
    #dbg_declare(ptr %rhs.dbg.spill.i, !227, !DIExpression(), !233)
  %0 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %self, i32 %rhs), !dbg !234
  %_5.0.i = extractvalue { i32, i1 } %0, 0, !dbg !234
  %_5.1.i = extractvalue { i32, i1 } %0, 1, !dbg !234
  store i32 %_5.0.i, ptr %a.dbg.spill.i, align 4, !dbg !235
    #dbg_declare(ptr %a.dbg.spill.i, !228, !DIExpression(), !236)
  %1 = zext i1 %_5.1.i to i8, !dbg !237
  store i8 %1, ptr %b.dbg.spill.i1, align 1, !dbg !237
    #dbg_declare(ptr %b.dbg.spill.i1, !230, !DIExpression(), !238)
  %_5.0 = extractvalue { i32, i1 } %0, 0, !dbg !239
  %_5.1 = extractvalue { i32, i1 } %0, 1, !dbg !239
  store i32 %_5.0, ptr %a.dbg.spill, align 4, !dbg !240
    #dbg_declare(ptr %a.dbg.spill, !213, !DIExpression(), !241)
  %2 = zext i1 %_5.1 to i8, !dbg !242
  store i8 %2, ptr %b.dbg.spill, align 1, !dbg !242
    #dbg_declare(ptr %b.dbg.spill, !215, !DIExpression(), !243)
  %3 = zext i1 %_5.1 to i8
  store i8 %3, ptr %b.dbg.spill.i, align 1
    #dbg_declare(ptr %b.dbg.spill.i, !244, !DIExpression(), !249)
  br i1 %_5.1, label %bb1.i, label %bb3.i, !dbg !251

bb3.i:                                            ; preds = %start
  store i8 0, ptr %_0.i, align 1, !dbg !252
  br label %_ZN4core10intrinsics8unlikely17hbdaab305ce3910c8E.exit, !dbg !253

bb1.i:                                            ; preds = %start
  store i8 1, ptr %_0.i, align 1, !dbg !254
  br label %_ZN4core10intrinsics8unlikely17hbdaab305ce3910c8E.exit, !dbg !253

_ZN4core10intrinsics8unlikely17hbdaab305ce3910c8E.exit: ; preds = %bb3.i, %bb1.i
  %4 = load i8, ptr %_0.i, align 1, !dbg !255
  %5 = trunc nuw i8 %4 to i1, !dbg !255
  br i1 %5, label %bb3, label %bb4, !dbg !256

bb4:                                              ; preds = %_ZN4core10intrinsics8unlikely17hbdaab305ce3910c8E.exit
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !257
  store i32 %_5.0, ptr %6, align 4, !dbg !257
  store i32 1, ptr %_0, align 4, !dbg !257
  br label %bb5, !dbg !258

bb3:                                              ; preds = %_ZN4core10intrinsics8unlikely17hbdaab305ce3910c8E.exit
  store i32 0, ptr %_0, align 4, !dbg !259
  br label %bb5, !dbg !258

bb5:                                              ; preds = %bb3, %bb4
  %7 = load i32, ptr %_0, align 4, !dbg !260
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !260
  %9 = load i32, ptr %8, align 4, !dbg !260
  %10 = insertvalue { i32, i32 } poison, i32 %7, 0, !dbg !260
  %11 = insertvalue { i32, i32 } %10, i32 %9, 1, !dbg !260
  ret { i32, i32 } %11, !dbg !260
}

; core::num::<impl usize>::abs_diff
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core3num23_$LT$impl$u20$usize$GT$8abs_diff17h20dbc36ae54c693eE"(i32 %self, i32 %other) unnamed_addr #1 !dbg !261 {
start:
  %rhs.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %other.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 4
  store i32 %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !265, !DIExpression(), !267)
  store i32 %other, ptr %other.dbg.spill, align 4
    #dbg_declare(ptr %other.dbg.spill, !266, !DIExpression(), !268)
  %0 = icmp eq i32 4, 1, !dbg !269
  br i1 %0, label %bb2, label %bb5, !dbg !269

bb2:                                              ; preds = %start
  store i32 %self, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !166, !DIExpression(), !270)
  store i32 %other, ptr %rhs.dbg.spill.i, align 4
    #dbg_declare(ptr %rhs.dbg.spill.i, !171, !DIExpression(), !272)
  %_0.i = sub i32 %self, %other, !dbg !273
; call core::num::<impl i32>::unsigned_abs
  %_4 = call i32 @"_ZN4core3num21_$LT$impl$u20$i32$GT$12unsigned_abs17hddd60634a183b5cbE"(i32 %_0.i) #9, !dbg !274
  store i32 %_4, ptr %_0, align 4, !dbg !275
  br label %bb10, !dbg !276

bb5:                                              ; preds = %start
  %_8 = icmp ult i32 %self, %other, !dbg !277
  br i1 %_8, label %bb6, label %bb8, !dbg !277

bb10:                                             ; preds = %bb7, %bb9, %bb2
  %1 = load i32, ptr %_0, align 4, !dbg !278
  ret i32 %1, !dbg !278

bb8:                                              ; preds = %bb5
  %_10.0 = sub i32 %self, %other, !dbg !279
  %_10.1 = icmp ult i32 %self, %other, !dbg !279
  br i1 %_10.1, label %panic, label %bb9, !dbg !279

bb6:                                              ; preds = %bb5
  %_9.0 = sub i32 %other, %self, !dbg !280
  %_9.1 = icmp ult i32 %other, %self, !dbg !280
  br i1 %_9.1, label %panic1, label %bb7, !dbg !280

bb9:                                              ; preds = %bb8
  store i32 %_10.0, ptr %_0, align 4, !dbg !279
  br label %bb10, !dbg !281

panic:                                            ; preds = %bb8
; call core::panicking::panic_const::panic_const_sub_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_sub_overflow17h7c3bd49388308f57E(ptr align 4 @alloc_d3ea8a12337a4535ec15ce1eda77c8dd) #10, !dbg !279
  unreachable, !dbg !279

bb7:                                              ; preds = %bb6
  store i32 %_9.0, ptr %_0, align 4, !dbg !280
  br label %bb10, !dbg !281

panic1:                                           ; preds = %bb6
; call core::panicking::panic_const::panic_const_sub_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_sub_overflow17h7c3bd49388308f57E(ptr align 4 @alloc_d3ea8a12337a4535ec15ce1eda77c8dd) #10, !dbg !280
  unreachable, !dbg !280
}

; core::num::nonzero::NonZero<u64>::trailing_zeros
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core3num7nonzero18NonZero$LT$u64$GT$14trailing_zeros17h410e4a129d0958fbE"(i64 %self) unnamed_addr #1 !dbg !282 {
start:
  %0 = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [8 x i8], align 8
  store i64 %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !298, !DIExpression(), !299)
; call core::num::nonzero::NonZero<T>::get
  %_2 = call i64 @"_ZN4core3num7nonzero16NonZero$LT$T$GT$3get17he066c213f94948efE"(i64 %self) #9, !dbg !300
  %1 = call i64 @llvm.cttz.i64(i64 %_2, i1 true), !dbg !301
  %2 = trunc i64 %1 to i32, !dbg !301
  store i32 %2, ptr %0, align 4, !dbg !301
  %_0 = load i32, ptr %0, align 4, !dbg !301
  ret i32 %_0, !dbg !302
}

; core::ptr::read_unaligned
; Function Attrs: inlinehint nounwind
define dso_local i64 @_ZN4core3ptr14read_unaligned17h6d1ad2df12929142E(ptr %src, ptr align 4 %0) unnamed_addr #1 !dbg !303 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %count.dbg.spill.i = alloca [4 x i8], align 4
  %dst.dbg.spill.i = alloca [4 x i8], align 4
  %src.dbg.spill.i = alloca [4 x i8], align 4
  %self.i = alloca [8 x i8], align 8
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %src.dbg.spill = alloca [4 x i8], align 4
  %tmp = alloca [8 x i8], align 8
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !337, !DIExpression(), !350)
    #dbg_declare(ptr %tmp, !338, !DIExpression(), !351)
  store i64 undef, ptr %tmp, align 8, !dbg !352
  store ptr %tmp, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !353, !DIExpression(), !362)
  store ptr %src, ptr %src.dbg.spill.i, align 4
    #dbg_declare(ptr %src.dbg.spill.i, !364, !DIExpression(), !373)
  store ptr %tmp, ptr %dst.dbg.spill.i, align 4
    #dbg_declare(ptr %dst.dbg.spill.i, !371, !DIExpression(), !375)
  store i32 8, ptr %count.dbg.spill.i, align 4
    #dbg_declare(ptr %count.dbg.spill.i, !372, !DIExpression(), !376)
; call core::ub_checks::check_language_ub
  %_4.i = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17hc9ddcf3001b97422E() #9, !dbg !377
  br i1 %_4.i, label %bb2.i, label %_ZN4core3ptr19copy_nonoverlapping17h7addadc19defe5aeE.exit, !dbg !377

bb2.i:                                            ; preds = %start
; call core::ptr::copy_nonoverlapping::precondition_check
  call void @_ZN4core3ptr19copy_nonoverlapping18precondition_check17h5c74c2f3f7735849E(ptr %src, ptr %tmp, i32 1, i32 1, i32 8, ptr align 4 @alloc_15a52d1a884c78a5de92a2463d39823d) #9, !dbg !380
  br label %_ZN4core3ptr19copy_nonoverlapping17h7addadc19defe5aeE.exit, !dbg !380

_ZN4core3ptr19copy_nonoverlapping17h7addadc19defe5aeE.exit: ; preds = %start, %bb2.i
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %tmp, ptr align 1 %src, i32 8, i1 false), !dbg !381
  %_9 = load i64, ptr %tmp, align 8, !dbg !382
  store i64 %_9, ptr %self.i, align 8
    #dbg_declare(ptr %self.i, !383, !DIExpression(), !389)
  store ptr %self.i, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !391, !DIExpression(), !403)
; call core::ptr::const_ptr::<impl *const T>::read
  %_0.i = call i64 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h33a6069a5019fd58E"(ptr %self.i, ptr align 4 %0) #9, !dbg !405
  ret i64 %_0.i, !dbg !406
}

; core::ptr::copy_nonoverlapping::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr19copy_nonoverlapping18precondition_check17h5c74c2f3f7735849E(ptr %src, ptr %dst, i32 %size, i32 %align, i32 %count, ptr align 4 %0) unnamed_addr #1 !dbg !407 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %count.dbg.spill = alloca [4 x i8], align 4
  %align.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %dst.dbg.spill = alloca [4 x i8], align 4
  %src.dbg.spill = alloca [4 x i8], align 4
  %_17 = alloca [8 x i8], align 4
  %_15 = alloca [24 x i8], align 4
  %zero_size = alloca [1 x i8], align 1
  %_6 = alloca [1 x i8], align 1
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !413, !DIExpression(), !422)
  store ptr %dst, ptr %dst.dbg.spill, align 4
    #dbg_declare(ptr %dst.dbg.spill, !414, !DIExpression(), !422)
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !415, !DIExpression(), !422)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !416, !DIExpression(), !422)
  store i32 %count, ptr %count.dbg.spill, align 4
    #dbg_declare(ptr %count.dbg.spill, !417, !DIExpression(), !422)
    #dbg_declare(ptr %zero_size, !418, !DIExpression(), !423)
  store ptr @alloc_bd3468a7b96187f70c1ce98a3e7a63bf, ptr %msg.dbg.spill, align 4, !dbg !424
  %1 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !424
  store i32 283, ptr %1, align 4, !dbg !424
    #dbg_declare(ptr %msg.dbg.spill, !420, !DIExpression(), !424)
  %2 = icmp eq i32 %count, 0, !dbg !425
  br i1 %2, label %bb1, label %bb2, !dbg !425

bb1:                                              ; preds = %start
  store i8 1, ptr %zero_size, align 1, !dbg !425
  br label %bb3, !dbg !425

bb2:                                              ; preds = %start
  %3 = icmp eq i32 %size, 0, !dbg !427
  %4 = zext i1 %3 to i8, !dbg !427
  store i8 %4, ptr %zero_size, align 1, !dbg !427
  br label %bb3, !dbg !425

bb3:                                              ; preds = %bb2, %bb1
  %5 = load i8, ptr %zero_size, align 1, !dbg !428
  %_9 = trunc nuw i8 %5 to i1, !dbg !428
; call core::ub_checks::maybe_is_aligned_and_not_null
  %_8 = call zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hb16a941476d8d9a1E(ptr %src, i32 %align, i1 zeroext %_9) #9, !dbg !429
  br i1 %_8, label %bb5, label %bb8, !dbg !429

bb8:                                              ; preds = %bb5, %bb3
  store i8 0, ptr %_6, align 1, !dbg !429
  br label %bb9, !dbg !429

bb5:                                              ; preds = %bb3
  %6 = load i8, ptr %zero_size, align 1, !dbg !430
  %_12 = trunc nuw i8 %6 to i1, !dbg !430
; call core::ub_checks::maybe_is_aligned_and_not_null
  %_10 = call zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hb16a941476d8d9a1E(ptr %dst, i32 %align, i1 zeroext %_12) #9, !dbg !431
  br i1 %_10, label %bb7, label %bb8, !dbg !431

bb7:                                              ; preds = %bb5
; call core::ub_checks::maybe_is_nonoverlapping
  %7 = call zeroext i1 @_ZN4core9ub_checks23maybe_is_nonoverlapping17h63367108861f9f74E(ptr %src, ptr %dst, i32 %size, i32 %count) #9, !dbg !432
  %8 = zext i1 %7 to i8, !dbg !432
  store i8 %8, ptr %_6, align 1, !dbg !432
  br label %bb9, !dbg !432

bb9:                                              ; preds = %bb7, %bb8
  %9 = load i8, ptr %_6, align 1, !dbg !433
  %10 = trunc nuw i8 %9 to i1, !dbg !433
  br i1 %10, label %bb12, label %bb10, !dbg !433

bb10:                                             ; preds = %bb9
  %11 = getelementptr inbounds nuw { ptr, i32 }, ptr %_17, i32 0, !dbg !434
  store ptr @alloc_bd3468a7b96187f70c1ce98a3e7a63bf, ptr %11, align 4, !dbg !434
  %12 = getelementptr inbounds i8, ptr %11, i32 4, !dbg !434
  store i32 283, ptr %12, align 4, !dbg !434
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_15, ptr align 4 %_17) #9, !dbg !435
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h256658f36c86c48dE(ptr align 4 %_15, i1 zeroext false, ptr align 4 %0) #10, !dbg !436
  unreachable, !dbg !436

bb12:                                             ; preds = %bb9
  ret void, !dbg !437
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint nounwind
define dso_local { ptr, i32 } @_ZN4core3ptr24slice_from_raw_parts_mut17hc59c18e342d99bc7E(ptr %data, i32 %len) unnamed_addr #1 !dbg !438 {
start:
  %len.dbg.spill = alloca [4 x i8], align 4
  %data.dbg.spill = alloca [4 x i8], align 4
  store ptr %data, ptr %data.dbg.spill, align 4
    #dbg_declare(ptr %data.dbg.spill, !453, !DIExpression(), !457)
  store i32 %len, ptr %len.dbg.spill, align 4
    #dbg_declare(ptr %len.dbg.spill, !454, !DIExpression(), !458)
; call core::ptr::metadata::from_raw_parts_mut
  %0 = call { ptr, i32 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h9e627e0534e3af57E(ptr %data, i32 %len) #9, !dbg !459
  %_0.0 = extractvalue { ptr, i32 } %0, 0, !dbg !459
  %_0.1 = extractvalue { ptr, i32 } %0, 1, !dbg !459
  %1 = insertvalue { ptr, i32 } poison, ptr %_0.0, 0, !dbg !460
  %2 = insertvalue { ptr, i32 } %1, i32 %_0.1, 1, !dbg !460
  ret { ptr, i32 } %2, !dbg !460
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local i64 @_ZN4core3ptr4read17hb71da8a038dc7781E(ptr %src, ptr align 4 %0) unnamed_addr #1 !dbg !461 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !463, !DIExpression(), !464)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17hc9ddcf3001b97422E() #9, !dbg !465
  br i1 %_2, label %bb2, label %bb4, !dbg !465

bb4:                                              ; preds = %bb2, %start
  %_0 = load i64, ptr %src, align 8, !dbg !467
  ret i64 %_0, !dbg !468

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17hb802a2aeb82aab1bE(ptr %src, i32 8, i1 zeroext false, ptr align 4 %0) #9, !dbg !469
  br label %bb4, !dbg !469
}

; core::ptr::read::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr4read18precondition_check17hb802a2aeb82aab1bE(ptr %addr, i32 %align, i1 zeroext %is_zst, ptr align 4 %0) unnamed_addr #1 !dbg !470 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %is_zst.dbg.spill = alloca [1 x i8], align 1
  %align.dbg.spill = alloca [4 x i8], align 4
  %addr.dbg.spill = alloca [4 x i8], align 4
  %_8 = alloca [8 x i8], align 4
  %_6 = alloca [24 x i8], align 4
  store ptr %addr, ptr %addr.dbg.spill, align 4
    #dbg_declare(ptr %addr.dbg.spill, !475, !DIExpression(), !480)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !476, !DIExpression(), !480)
  %1 = zext i1 %is_zst to i8
  store i8 %1, ptr %is_zst.dbg.spill, align 1
    #dbg_declare(ptr %is_zst.dbg.spill, !477, !DIExpression(), !480)
  store ptr @alloc_ed8641ebea8e5515740d4eb49a916ff5, ptr %msg.dbg.spill, align 4, !dbg !481
  %2 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !481
  store i32 218, ptr %2, align 4, !dbg !481
    #dbg_declare(ptr %msg.dbg.spill, !478, !DIExpression(), !481)
; call core::ub_checks::maybe_is_aligned_and_not_null
  %_4 = call zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hb16a941476d8d9a1E(ptr %addr, i32 %align, i1 zeroext %is_zst) #9, !dbg !482
  br i1 %_4, label %bb2, label %bb3, !dbg !482

bb3:                                              ; preds = %start
  %3 = getelementptr inbounds nuw { ptr, i32 }, ptr %_8, i32 0, !dbg !484
  store ptr @alloc_ed8641ebea8e5515740d4eb49a916ff5, ptr %3, align 4, !dbg !484
  %4 = getelementptr inbounds i8, ptr %3, i32 4, !dbg !484
  store i32 218, ptr %4, align 4, !dbg !484
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_6, ptr align 4 %_8) #9, !dbg !485
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h256658f36c86c48dE(ptr align 4 %_6, i1 zeroext false, ptr align 4 %0) #10, !dbg !486
  unreachable, !dbg !486

bb2:                                              ; preds = %start
  ret void, !dbg !487
}

; core::ptr::mut_ptr::<impl *mut T>::add::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18precondition_check17h467862a1054379e6E"(ptr %this, i32 %count, i32 %size, ptr align 4 %0) unnamed_addr #1 !dbg !488 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %count.dbg.spill = alloca [4 x i8], align 4
  %this.dbg.spill = alloca [4 x i8], align 4
  %_8 = alloca [8 x i8], align 4
  %_6 = alloca [24 x i8], align 4
  store ptr %this, ptr %this.dbg.spill, align 4
    #dbg_declare(ptr %this.dbg.spill, !495, !DIExpression(), !500)
  store i32 %count, ptr %count.dbg.spill, align 4
    #dbg_declare(ptr %count.dbg.spill, !496, !DIExpression(), !500)
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !497, !DIExpression(), !500)
  store ptr @alloc_4fb4eca1f8b9d0ded0407faa6b2654bb, ptr %msg.dbg.spill, align 4, !dbg !501
  %1 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !501
  store i32 214, ptr %1, align 4, !dbg !501
    #dbg_declare(ptr %msg.dbg.spill, !498, !DIExpression(), !501)
; call core::ptr::mut_ptr::<impl *mut T>::add::runtime_add_nowrap
  %_4 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18runtime_add_nowrap17h4d84a2e818ec59acE"(ptr %this, i32 %count, i32 %size) #9, !dbg !502
  br i1 %_4, label %bb2, label %bb3, !dbg !502

bb3:                                              ; preds = %start
  %2 = getelementptr inbounds nuw { ptr, i32 }, ptr %_8, i32 0, !dbg !505
  store ptr @alloc_4fb4eca1f8b9d0ded0407faa6b2654bb, ptr %2, align 4, !dbg !505
  %3 = getelementptr inbounds i8, ptr %2, i32 4, !dbg !505
  store i32 214, ptr %3, align 4, !dbg !505
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_6, ptr align 4 %_8) #9, !dbg !506
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h256658f36c86c48dE(ptr align 4 %_6, i1 zeroext false, ptr align 4 %0) #10, !dbg !507
  unreachable, !dbg !507

bb2:                                              ; preds = %start
  ret void, !dbg !508
}

; core::ptr::mut_ptr::<impl *mut T>::add::runtime_add_nowrap
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18runtime_add_nowrap17h4d84a2e818ec59acE"(ptr %this, i32 %count, i32 %size) unnamed_addr #1 !dbg !509 {
start:
  %size.dbg.spill = alloca [4 x i8], align 4
  %count.dbg.spill = alloca [4 x i8], align 4
  %this.dbg.spill = alloca [4 x i8], align 4
  %_4 = alloca [12 x i8], align 4
  store ptr %this, ptr %this.dbg.spill, align 4
    #dbg_declare(ptr %this.dbg.spill, !513, !DIExpression(), !516)
  store i32 %count, ptr %count.dbg.spill, align 4
    #dbg_declare(ptr %count.dbg.spill, !514, !DIExpression(), !517)
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !515, !DIExpression(), !518)
  store ptr %this, ptr %_4, align 4, !dbg !519
  %0 = getelementptr inbounds i8, ptr %_4, i32 4, !dbg !519
  store i32 %count, ptr %0, align 4, !dbg !519
  %1 = getelementptr inbounds i8, ptr %_4, i32 8, !dbg !519
  store i32 %size, ptr %1, align 4, !dbg !519
  %2 = load ptr, ptr %_4, align 4, !dbg !521
  %3 = getelementptr inbounds i8, ptr %_4, i32 4, !dbg !521
  %4 = load i32, ptr %3, align 4, !dbg !521
  %5 = getelementptr inbounds i8, ptr %_4, i32 8, !dbg !521
  %6 = load i32, ptr %5, align 4, !dbg !521
; call core::ptr::mut_ptr::<impl *mut T>::add::runtime_add_nowrap::runtime
  %_0 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18runtime_add_nowrap7runtime17hea30b135eef39c51E"(ptr %2, i32 %4, i32 %6) #9, !dbg !521
  ret i1 %_0, !dbg !522
}

; core::ptr::mut_ptr::<impl *mut T>::add::runtime_add_nowrap::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18runtime_add_nowrap7runtime17hea30b135eef39c51E"(ptr %this, i32 %count, i32 %size) unnamed_addr #1 !dbg !523 {
start:
  %self.dbg.spill.i2 = alloca [4 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %b.dbg.spill.i = alloca [1 x i8], align 1
  %a.dbg.spill.i = alloca [4 x i8], align 4
  %rhs.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %overflow.dbg.spill = alloca [1 x i8], align 1
  %byte_offset.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %count.dbg.spill = alloca [4 x i8], align 4
  %this.dbg.spill = alloca [4 x i8], align 4
  %_5 = alloca [8 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %this, ptr %this.dbg.spill, align 4
    #dbg_declare(ptr %this.dbg.spill, !526, !DIExpression(), !533)
  store i32 %count, ptr %count.dbg.spill, align 4
    #dbg_declare(ptr %count.dbg.spill, !527, !DIExpression(), !533)
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !528, !DIExpression(), !533)
; call core::num::<impl usize>::checked_mul
  %0 = call { i32, i32 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h489bb426cb3f2bbbE"(i32 %count, i32 %size) #9, !dbg !534
  %1 = extractvalue { i32, i32 } %0, 0, !dbg !534
  %2 = extractvalue { i32, i32 } %0, 1, !dbg !534
  store i32 %1, ptr %_5, align 4, !dbg !534
  %3 = getelementptr inbounds i8, ptr %_5, i32 4, !dbg !534
  store i32 %2, ptr %3, align 4, !dbg !534
  %_6 = load i32, ptr %_5, align 4, !dbg !536
  %4 = getelementptr inbounds i8, ptr %_5, i32 4, !dbg !536
  %5 = load i32, ptr %4, align 4, !dbg !536
  %6 = trunc nuw i32 %_6 to i1, !dbg !537
  br i1 %6, label %bb2, label %bb3, !dbg !537

bb2:                                              ; preds = %start
  %7 = getelementptr inbounds i8, ptr %_5, i32 4, !dbg !538
  %byte_offset = load i32, ptr %7, align 4, !dbg !538
  store i32 %byte_offset, ptr %byte_offset.dbg.spill, align 4, !dbg !538
    #dbg_declare(ptr %byte_offset.dbg.spill, !529, !DIExpression(), !539)
  store ptr %this, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !540, !DIExpression(), !546)
  store ptr %this, ptr %self.dbg.spill.i2, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !548, !DIExpression(), !555)
  %_0.i = ptrtoint ptr %this to i32, !dbg !557
  store i32 %_0.i, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !558, !DIExpression(), !565)
  store i32 %byte_offset, ptr %rhs.dbg.spill.i, align 4
    #dbg_declare(ptr %rhs.dbg.spill.i, !561, !DIExpression(), !567)
  %_5.0.i = add i32 %_0.i, %byte_offset, !dbg !568
  %_5.1.i = icmp ult i32 %_5.0.i, %_0.i, !dbg !568
  store i32 %_5.0.i, ptr %a.dbg.spill.i, align 4, !dbg !569
    #dbg_declare(ptr %a.dbg.spill.i, !562, !DIExpression(), !570)
  %8 = zext i1 %_5.1.i to i8, !dbg !571
  store i8 %8, ptr %b.dbg.spill.i, align 1, !dbg !571
    #dbg_declare(ptr %b.dbg.spill.i, !564, !DIExpression(), !572)
  %9 = insertvalue { i32, i1 } poison, i32 %_5.0.i, 0, !dbg !573
  %10 = insertvalue { i32, i1 } %9, i1 %_5.1.i, 1, !dbg !573
  %_8.0 = extractvalue { i32, i1 } %10, 0, !dbg !574
  %_8.1 = extractvalue { i32, i1 } %10, 1, !dbg !574
  %11 = zext i1 %_8.1 to i8, !dbg !575
  store i8 %11, ptr %overflow.dbg.spill, align 1, !dbg !575
    #dbg_declare(ptr %overflow.dbg.spill, !531, !DIExpression(), !576)
  %_10 = icmp ule i32 %byte_offset, 2147483647, !dbg !577
  br i1 %_10, label %bb6, label %bb7, !dbg !577

bb3:                                              ; preds = %start
  store i8 0, ptr %_0, align 1, !dbg !578
  br label %bb8, !dbg !579

bb7:                                              ; preds = %bb2
  store i8 0, ptr %_0, align 1, !dbg !577
  br label %bb8, !dbg !577

bb6:                                              ; preds = %bb2
  %12 = xor i1 %_8.1, true, !dbg !580
  %13 = zext i1 %12 to i8, !dbg !580
  store i8 %13, ptr %_0, align 1, !dbg !580
  br label %bb8, !dbg !577

bb8:                                              ; preds = %bb3, %bb6, %bb7
  %14 = load i8, ptr %_0, align 1, !dbg !581
  %15 = trunc nuw i8 %14 to i1, !dbg !581
  ret i1 %15, !dbg !581

bb9:                                              ; No predecessors!
  unreachable, !dbg !582
}

; core::ptr::metadata::from_raw_parts_mut
; Function Attrs: inlinehint nounwind
define dso_local { ptr, i32 } @_ZN4core3ptr8metadata18from_raw_parts_mut17h9e627e0534e3af57E(ptr %data_pointer, i32 %metadata) unnamed_addr #1 !dbg !583 {
start:
  %metadata.dbg.spill = alloca [4 x i8], align 4
  %data_pointer.dbg.spill = alloca [4 x i8], align 4
  store ptr %data_pointer, ptr %data_pointer.dbg.spill, align 4
    #dbg_declare(ptr %data_pointer.dbg.spill, !587, !DIExpression(), !591)
  store i32 %metadata, ptr %metadata.dbg.spill, align 4
    #dbg_declare(ptr %metadata.dbg.spill, !588, !DIExpression(), !592)
  %0 = insertvalue { ptr, i32 } poison, ptr %data_pointer, 0, !dbg !593
  %1 = insertvalue { ptr, i32 } %0, i32 %metadata, 1, !dbg !593
  ret { ptr, i32 } %1, !dbg !593
}

; core::ptr::const_ptr::<impl *const T>::read
; Function Attrs: inlinehint nounwind
define dso_local i64 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h33a6069a5019fd58E"(ptr %self, ptr align 4 %0) unnamed_addr #1 !dbg !594 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !596, !DIExpression(), !597)
; call core::ptr::read
  %_0 = call i64 @_ZN4core3ptr4read17hb71da8a038dc7781E(ptr %self, ptr align 4 %0) #9, !dbg !598
  ret i64 %_0, !dbg !599
}

; core::slice::raw::from_raw_parts_mut
; Function Attrs: inlinehint nounwind
define dso_local { ptr, i32 } @_ZN4core5slice3raw18from_raw_parts_mut17heeb025ab382939bbE(ptr %data, i32 %len, ptr align 4 %0) unnamed_addr #1 !dbg !600 {
start:
  %len.dbg.spill = alloca [4 x i8], align 4
  %data.dbg.spill = alloca [4 x i8], align 4
  store ptr %data, ptr %data.dbg.spill, align 4
    #dbg_declare(ptr %data.dbg.spill, !611, !DIExpression(), !613)
  store i32 %len, ptr %len.dbg.spill, align 4
    #dbg_declare(ptr %len.dbg.spill, !612, !DIExpression(), !614)
; call core::ub_checks::check_language_ub
  %_3 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17hc9ddcf3001b97422E() #9, !dbg !615
  br i1 %_3, label %bb2, label %bb5, !dbg !615

bb5:                                              ; preds = %bb2, %start
; call core::ptr::slice_from_raw_parts_mut
  %1 = call { ptr, i32 } @_ZN4core3ptr24slice_from_raw_parts_mut17hc59c18e342d99bc7E(ptr %data, i32 %len) #9, !dbg !617
  %_8.0 = extractvalue { ptr, i32 } %1, 0, !dbg !617
  %_8.1 = extractvalue { ptr, i32 } %1, 1, !dbg !617
  %2 = insertvalue { ptr, i32 } poison, ptr %_8.0, 0, !dbg !618
  %3 = insertvalue { ptr, i32 } %2, i32 %_8.1, 1, !dbg !618
  ret { ptr, i32 } %3, !dbg !618

bb2:                                              ; preds = %start
; call core::slice::raw::from_raw_parts_mut::precondition_check
  call void @_ZN4core5slice3raw18from_raw_parts_mut18precondition_check17h33352bc35700d71fE(ptr %data, i32 1, i32 1, i32 %len, ptr align 4 %0) #9, !dbg !619
  br label %bb5, !dbg !619
}

; core::slice::raw::from_raw_parts_mut::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core5slice3raw18from_raw_parts_mut18precondition_check17h33352bc35700d71fE(ptr %data, i32 %size, i32 %align, i32 %len, ptr align 4 %0) unnamed_addr #1 !dbg !620 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %len.dbg.spill = alloca [4 x i8], align 4
  %align.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %data.dbg.spill = alloca [4 x i8], align 4
  %_11 = alloca [8 x i8], align 4
  %_9 = alloca [24 x i8], align 4
  store ptr %data, ptr %data.dbg.spill, align 4
    #dbg_declare(ptr %data.dbg.spill, !625, !DIExpression(), !631)
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !626, !DIExpression(), !631)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !627, !DIExpression(), !631)
  store i32 %len, ptr %len.dbg.spill, align 4
    #dbg_declare(ptr %len.dbg.spill, !628, !DIExpression(), !631)
  store ptr @alloc_5c1a2f972552229672fc942406cfc298, ptr %msg.dbg.spill, align 4, !dbg !632
  %1 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !632
  store i32 283, ptr %1, align 4, !dbg !632
    #dbg_declare(ptr %msg.dbg.spill, !629, !DIExpression(), !632)
; call core::ub_checks::maybe_is_aligned_and_not_null
  %_5 = call zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hb16a941476d8d9a1E(ptr %data, i32 %align, i1 zeroext false) #9, !dbg !633
  br i1 %_5, label %bb2, label %bb5, !dbg !633

bb5:                                              ; preds = %bb2, %start
  %2 = getelementptr inbounds nuw { ptr, i32 }, ptr %_11, i32 0, !dbg !635
  store ptr @alloc_5c1a2f972552229672fc942406cfc298, ptr %2, align 4, !dbg !635
  %3 = getelementptr inbounds i8, ptr %2, i32 4, !dbg !635
  store i32 283, ptr %3, align 4, !dbg !635
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_9, ptr align 4 %_11) #9, !dbg !636
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h256658f36c86c48dE(ptr align 4 %_9, i1 zeroext false, ptr align 4 %0) #10, !dbg !637
  unreachable, !dbg !637

bb2:                                              ; preds = %start
; call core::ub_checks::is_valid_allocation_size
  %_7 = call zeroext i1 @_ZN4core9ub_checks24is_valid_allocation_size17hceea5d110dce2c5eE(i32 %size, i32 %len) #9, !dbg !638
  br i1 %_7, label %bb4, label %bb5, !dbg !638

bb4:                                              ; preds = %bb2
  ret void, !dbg !639
}

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17hf49f16ed30d86844E(ptr align 4 %0) unnamed_addr #2 !dbg !640 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_2ca80fe829e7dcbb4661228c202cce92) #9, !dbg !646
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_2, ptr align 4 %0) #10, !dbg !647
  unreachable, !dbg !647
}

; core::panicking::panic_const::panic_const_add_overflow
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 %0) unnamed_addr #2 !dbg !648 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_491fd71eacc9ac6df50464189817658a) #9, !dbg !649
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_2, ptr align 4 %0) #10, !dbg !650
  unreachable, !dbg !650
}

; core::panicking::panic_const::panic_const_mul_overflow
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking11panic_const24panic_const_mul_overflow17hc3f2f5648b4a5121E(ptr align 4 %0) unnamed_addr #2 !dbg !651 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_3a541098c7af55f2d1b57c8374ee944e) #9, !dbg !652
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_2, ptr align 4 %0) #10, !dbg !653
  unreachable, !dbg !653
}

; core::panicking::panic_const::panic_const_shl_overflow
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking11panic_const24panic_const_shl_overflow17h129e566a8f606375E(ptr align 4 %0) unnamed_addr #2 !dbg !654 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_26eab6319fe0d02af8105663e6a2ea8b) #9, !dbg !655
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_2, ptr align 4 %0) #10, !dbg !656
  unreachable, !dbg !656
}

; core::panicking::panic_const::panic_const_shr_overflow
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking11panic_const24panic_const_shr_overflow17ha5f77e543b98d47cE(ptr align 4 %0) unnamed_addr #2 !dbg !657 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_0f75c28593fb3281511a86ba9b3adf6f) #9, !dbg !658
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_2, ptr align 4 %0) #10, !dbg !659
  unreachable, !dbg !659
}

; core::panicking::panic_const::panic_const_sub_overflow
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking11panic_const24panic_const_sub_overflow17h7c3bd49388308f57E(ptr align 4 %0) unnamed_addr #2 !dbg !660 {
start:
  %_2 = alloca [24 x i8], align 4
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 @alloc_7daa13c2a11e2a3dbea9e2a29716d6f6) #9, !dbg !661
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_2, ptr align 4 %0) #10, !dbg !662
  unreachable, !dbg !662
}

; core::panicking::panic_nounwind
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking14panic_nounwind17h401ade9a4c393332E(ptr align 1 %expr.0, i32 %expr.1) unnamed_addr #2 !dbg !663 {
start:
  %expr.dbg.spill = alloca [8 x i8], align 4
  %_5 = alloca [8 x i8], align 4
  %_3 = alloca [24 x i8], align 4
  store ptr %expr.0, ptr %expr.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %expr.dbg.spill, i32 4
  store i32 %expr.1, ptr %0, align 4
    #dbg_declare(ptr %expr.dbg.spill, !667, !DIExpression(), !668)
  %1 = getelementptr inbounds nuw { ptr, i32 }, ptr %_5, i32 0, !dbg !669
  store ptr %expr.0, ptr %1, align 4, !dbg !669
  %2 = getelementptr inbounds i8, ptr %1, i32 4, !dbg !669
  store i32 %expr.1, ptr %2, align 4, !dbg !669
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_3, ptr align 4 %_5) #9, !dbg !670
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h256658f36c86c48dE(ptr align 4 %_3, i1 zeroext false, ptr align 4 @alloc_55a1350f0592d90727796c17fe69030d) #10, !dbg !671
  unreachable, !dbg !671
}

; core::panicking::panic_nounwind_fmt
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking18panic_nounwind_fmt17h256658f36c86c48dE(ptr align 4 %fmt, i1 zeroext %force_no_backtrace, ptr align 4 %0) unnamed_addr #2 !dbg !672 {
start:
  %force_no_backtrace.dbg.spill = alloca [1 x i8], align 1
  %_3 = alloca [28 x i8], align 4
    #dbg_declare(ptr %fmt, !759, !DIExpression(), !761)
  %1 = zext i1 %force_no_backtrace to i8
  store i8 %1, ptr %force_no_backtrace.dbg.spill, align 1
    #dbg_declare(ptr %force_no_backtrace.dbg.spill, !760, !DIExpression(), !762)
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_3, ptr align 4 %fmt, i32 24, i1 false), !dbg !763
  %2 = getelementptr inbounds i8, ptr %_3, i32 24, !dbg !763
  %3 = zext i1 %force_no_backtrace to i8, !dbg !763
  store i8 %3, ptr %2, align 4, !dbg !763
  %4 = getelementptr inbounds i8, ptr %_3, i32 24, !dbg !765
  %5 = load i8, ptr %4, align 4, !dbg !765
  %6 = trunc nuw i8 %5 to i1, !dbg !765
; call core::panicking::panic_nounwind_fmt::runtime
  call void @_ZN4core9panicking18panic_nounwind_fmt7runtime17hb1485ab00c51cf61E(ptr align 4 %_3, i1 zeroext %6, ptr align 4 %0) #10, !dbg !765
  unreachable, !dbg !765
}

; core::panicking::panic_nounwind_fmt::runtime
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking18panic_nounwind_fmt7runtime17hb1485ab00c51cf61E(ptr align 4 %fmt, i1 zeroext %force_no_backtrace, ptr align 4 %0) unnamed_addr #2 !dbg !766 {
start:
    #dbg_declare(ptr %fmt, !769, !DIExpression(), !781)
  %force_no_backtrace.dbg.spill = alloca [1 x i8], align 1
  %1 = zext i1 %force_no_backtrace to i8
  store i8 %1, ptr %force_no_backtrace.dbg.spill, align 1
    #dbg_declare(ptr %force_no_backtrace.dbg.spill, !770, !DIExpression(), !781)
  call void @llvm.trap(), !dbg !782
  unreachable, !dbg !782
}

; core::panicking::panic
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking5panic17h2ce9e1c499078148E(ptr align 1 %expr.0, i32 %expr.1, ptr align 4 %0) unnamed_addr #2 !dbg !784 {
start:
  %expr.dbg.spill = alloca [8 x i8], align 4
  %_5 = alloca [8 x i8], align 4
  %_3 = alloca [24 x i8], align 4
  store ptr %expr.0, ptr %expr.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %expr.dbg.spill, i32 4
  store i32 %expr.1, ptr %1, align 4
    #dbg_declare(ptr %expr.dbg.spill, !788, !DIExpression(), !789)
  %2 = getelementptr inbounds nuw { ptr, i32 }, ptr %_5, i32 0, !dbg !790
  store ptr %expr.0, ptr %2, align 4, !dbg !790
  %3 = getelementptr inbounds i8, ptr %2, i32 4, !dbg !790
  store i32 %expr.1, ptr %3, align 4, !dbg !790
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_3, ptr align 4 %_5) #9, !dbg !791
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_3, ptr align 4 %0) #10, !dbg !792
  unreachable, !dbg !792
}

; core::panicking::panic_fmt
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %fmt, ptr align 4 %0) unnamed_addr #2 !dbg !793 {
start:
    #dbg_declare(ptr %fmt, !797, !DIExpression(), !800)
  call void @llvm.trap(), !dbg !801
  unreachable, !dbg !801
}

; core::ub_checks::maybe_is_aligned
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned17h89a54d7a782a9e81E(ptr %ptr, i32 %align) unnamed_addr #1 !dbg !802 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !807, !DIExpression(), !809)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !808, !DIExpression(), !810)
; call core::ub_checks::maybe_is_aligned::runtime
  %_0 = call zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned7runtime17h33a0b2aefdca9fdaE(ptr %ptr, i32 %align) #9, !dbg !811
  ret i1 %_0, !dbg !813
}

; core::ub_checks::maybe_is_aligned::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned7runtime17h33a0b2aefdca9fdaE(ptr %ptr, i32 %align) unnamed_addr #1 !dbg !814 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !817, !DIExpression(), !819)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !818, !DIExpression(), !819)
; call core::ptr::const_ptr::<impl *const T>::is_aligned_to
  %_0 = call zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13is_aligned_to17h9d68787d1567a990E"(ptr %ptr, i32 %align) #9, !dbg !820
  ret i1 %_0, !dbg !822
}

; core::ub_checks::check_language_ub
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks17check_language_ub17hc9ddcf3001b97422E() unnamed_addr #1 !dbg !823 {
start:
  %_0 = alloca [1 x i8], align 1
  br label %bb1, !dbg !826

bb1:                                              ; preds = %start
; call core::ub_checks::check_language_ub::runtime
  %0 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub7runtime17h245850c936c392bbE() #9, !dbg !827
  %1 = zext i1 %0 to i8, !dbg !827
  store i8 %1, ptr %_0, align 1, !dbg !827
  br label %bb3, !dbg !827

bb3:                                              ; preds = %bb1
  %2 = load i8, ptr %_0, align 1, !dbg !829
  %3 = trunc nuw i8 %2 to i1, !dbg !829
  ret i1 %3, !dbg !829

bb2:                                              ; No predecessors!
  unreachable
}

; core::ub_checks::check_language_ub::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks17check_language_ub7runtime17h245850c936c392bbE() unnamed_addr #1 !dbg !830 {
start:
  ret i1 true, !dbg !832
}

; core::ub_checks::maybe_is_nonoverlapping
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks23maybe_is_nonoverlapping17h63367108861f9f74E(ptr %src, ptr %dst, i32 %size, i32 %count) unnamed_addr #1 !dbg !833 {
start:
  %count.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %dst.dbg.spill = alloca [4 x i8], align 4
  %src.dbg.spill = alloca [4 x i8], align 4
  %_5 = alloca [16 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !837, !DIExpression(), !841)
  store ptr %dst, ptr %dst.dbg.spill, align 4
    #dbg_declare(ptr %dst.dbg.spill, !838, !DIExpression(), !842)
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !839, !DIExpression(), !843)
  store i32 %count, ptr %count.dbg.spill, align 4
    #dbg_declare(ptr %count.dbg.spill, !840, !DIExpression(), !844)
  store ptr %src, ptr %_5, align 4, !dbg !845
  %0 = getelementptr inbounds i8, ptr %_5, i32 4, !dbg !845
  store ptr %dst, ptr %0, align 4, !dbg !845
  %1 = getelementptr inbounds i8, ptr %_5, i32 8, !dbg !845
  store i32 %size, ptr %1, align 4, !dbg !845
  %2 = getelementptr inbounds i8, ptr %_5, i32 12, !dbg !845
  store i32 %count, ptr %2, align 4, !dbg !845
  %3 = load ptr, ptr %_5, align 4, !dbg !847
  %4 = getelementptr inbounds i8, ptr %_5, i32 4, !dbg !847
  %5 = load ptr, ptr %4, align 4, !dbg !847
  %6 = getelementptr inbounds i8, ptr %_5, i32 8, !dbg !847
  %7 = load i32, ptr %6, align 4, !dbg !847
  %8 = getelementptr inbounds i8, ptr %_5, i32 12, !dbg !847
  %9 = load i32, ptr %8, align 4, !dbg !847
; call core::ub_checks::maybe_is_nonoverlapping::runtime
  %_0 = call zeroext i1 @_ZN4core9ub_checks23maybe_is_nonoverlapping7runtime17h046557a2ba0f4e7cE(ptr %3, ptr %5, i32 %7, i32 %9) #9, !dbg !847
  ret i1 %_0, !dbg !848
}

; core::ub_checks::maybe_is_nonoverlapping::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks23maybe_is_nonoverlapping7runtime17h046557a2ba0f4e7cE(ptr %src, ptr %dst, i32 %size, i32 %count) unnamed_addr #1 !dbg !849 {
start:
  %self.dbg.spill.i7 = alloca [4 x i8], align 4
  %self.dbg.spill.i6 = alloca [4 x i8], align 4
  %self.dbg.spill.i3 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %diff.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill2 = alloca [4 x i8], align 4
  %dst_usize.dbg.spill = alloca [4 x i8], align 4
  %src_usize.dbg.spill = alloca [4 x i8], align 4
  %count.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %dst.dbg.spill = alloca [4 x i8], align 4
  %src.dbg.spill = alloca [4 x i8], align 4
  %_9 = alloca [8 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !852, !DIExpression(), !864)
  store ptr %dst, ptr %dst.dbg.spill, align 4
    #dbg_declare(ptr %dst.dbg.spill, !853, !DIExpression(), !864)
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !854, !DIExpression(), !864)
  store i32 %count, ptr %count.dbg.spill, align 4
    #dbg_declare(ptr %count.dbg.spill, !855, !DIExpression(), !864)
  store ptr %src, ptr %self.dbg.spill.i3, align 4
    #dbg_declare(ptr %self.dbg.spill.i3, !540, !DIExpression(), !865)
  store ptr %src, ptr %self.dbg.spill.i6, align 4
    #dbg_declare(ptr %self.dbg.spill.i6, !548, !DIExpression(), !868)
  %_0.i5 = ptrtoint ptr %src to i32, !dbg !870
  store i32 %_0.i5, ptr %src_usize.dbg.spill, align 4, !dbg !871
    #dbg_declare(ptr %src_usize.dbg.spill, !856, !DIExpression(), !872)
  store ptr %dst, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !540, !DIExpression(), !873)
  store ptr %dst, ptr %self.dbg.spill.i7, align 4
    #dbg_declare(ptr %self.dbg.spill.i7, !548, !DIExpression(), !875)
  %_0.i = ptrtoint ptr %dst to i32, !dbg !877
  store i32 %_0.i, ptr %dst_usize.dbg.spill, align 4, !dbg !878
    #dbg_declare(ptr %dst_usize.dbg.spill, !858, !DIExpression(), !879)
; call core::num::<impl usize>::checked_mul
  %0 = call { i32, i32 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h489bb426cb3f2bbbE"(i32 %size, i32 %count) #9, !dbg !880
  %1 = extractvalue { i32, i32 } %0, 0, !dbg !880
  %2 = extractvalue { i32, i32 } %0, 1, !dbg !880
  store i32 %1, ptr %_9, align 4, !dbg !880
  %3 = getelementptr inbounds i8, ptr %_9, i32 4, !dbg !880
  store i32 %2, ptr %3, align 4, !dbg !880
  %_10 = load i32, ptr %_9, align 4, !dbg !881
  %4 = getelementptr inbounds i8, ptr %_9, i32 4, !dbg !881
  %5 = load i32, ptr %4, align 4, !dbg !881
  %6 = trunc nuw i32 %_10 to i1, !dbg !882
  br i1 %6, label %bb4, label %bb5, !dbg !882

bb4:                                              ; preds = %start
  %7 = getelementptr inbounds i8, ptr %_9, i32 4, !dbg !883
  %size1 = load i32, ptr %7, align 4, !dbg !883
  store i32 %size1, ptr %size.dbg.spill2, align 4, !dbg !883
    #dbg_declare(ptr %size.dbg.spill2, !860, !DIExpression(), !884)
; call core::num::<impl usize>::abs_diff
  %diff = call i32 @"_ZN4core3num23_$LT$impl$u20$usize$GT$8abs_diff17h20dbc36ae54c693eE"(i32 %_0.i5, i32 %_0.i) #9, !dbg !885
  store i32 %diff, ptr %diff.dbg.spill, align 4, !dbg !885
    #dbg_declare(ptr %diff.dbg.spill, !862, !DIExpression(), !886)
  %_0 = icmp uge i32 %diff, %size1, !dbg !887
  ret i1 %_0, !dbg !888

bb5:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h401ade9a4c393332E(ptr align 1 @alloc_763310d78c99c2c1ad3f8a9821e942f3, i32 61) #10, !dbg !889
  unreachable, !dbg !889

bb7:                                              ; No predecessors!
  unreachable, !dbg !890
}

; core::ub_checks::is_valid_allocation_size
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks24is_valid_allocation_size17hceea5d110dce2c5eE(i32 %size, i32 %len) unnamed_addr #1 !dbg !891 {
start:
  %len.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [4 x i8], align 4
  %max_len = alloca [4 x i8], align 4
  store i32 %size, ptr %size.dbg.spill, align 4
    #dbg_declare(ptr %size.dbg.spill, !895, !DIExpression(), !899)
  store i32 %len, ptr %len.dbg.spill, align 4
    #dbg_declare(ptr %len.dbg.spill, !896, !DIExpression(), !900)
    #dbg_declare(ptr %max_len, !897, !DIExpression(), !901)
  %0 = icmp eq i32 %size, 0, !dbg !902
  br i1 %0, label %bb1, label %bb2, !dbg !902

bb1:                                              ; preds = %start
  store i32 -1, ptr %max_len, align 4, !dbg !903
  br label %bb4, !dbg !904

bb2:                                              ; preds = %start
  %_5 = icmp eq i32 %size, 0, !dbg !905
  br i1 %_5, label %panic, label %bb3, !dbg !905

bb4:                                              ; preds = %bb3, %bb1
  %_6 = load i32, ptr %max_len, align 4, !dbg !906
  %_0 = icmp ule i32 %len, %_6, !dbg !907
  ret i1 %_0, !dbg !908

bb3:                                              ; preds = %bb2
  %1 = udiv i32 2147483647, %size, !dbg !905
  store i32 %1, ptr %max_len, align 4, !dbg !905
  br label %bb4, !dbg !904

panic:                                            ; preds = %bb2
; call core::panicking::panic_const::panic_const_div_by_zero
  call void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17hf49f16ed30d86844E(ptr align 4 @alloc_329dec4fe38a59083c3b039c87a8d615) #10, !dbg !905
  unreachable, !dbg !905
}

; core::ub_checks::maybe_is_aligned_and_not_null
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hb16a941476d8d9a1E(ptr %ptr, i32 %align, i1 zeroext %is_zst) unnamed_addr #1 !dbg !909 {
start:
  %is_zst.dbg.spill = alloca [1 x i8], align 1
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !913, !DIExpression(), !916)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !914, !DIExpression(), !917)
  %0 = zext i1 %is_zst to i8
  store i8 %0, ptr %is_zst.dbg.spill, align 1
    #dbg_declare(ptr %is_zst.dbg.spill, !915, !DIExpression(), !918)
; call core::ub_checks::maybe_is_aligned
  %_4 = call zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned17h89a54d7a782a9e81E(ptr %ptr, i32 %align) #9, !dbg !919
  br i1 %_4, label %bb2, label %bb3, !dbg !919

bb3:                                              ; preds = %start
  store i8 0, ptr %_0, align 1, !dbg !919
  br label %bb7, !dbg !919

bb2:                                              ; preds = %start
  br i1 %is_zst, label %bb4, label %bb5, !dbg !920

bb7:                                              ; preds = %bb4, %bb5, %bb3
  %1 = load i8, ptr %_0, align 1, !dbg !921
  %2 = trunc nuw i8 %1 to i1, !dbg !921
  ret i1 %2, !dbg !921

bb5:                                              ; preds = %bb2
; call core::ptr::const_ptr::<impl *const T>::is_null
  %_5 = call zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf56eacc16313c5f5E"(ptr %ptr) #9, !dbg !922
  %3 = xor i1 %_5, true, !dbg !923
  %4 = zext i1 %3 to i8, !dbg !923
  store i8 %4, ptr %_0, align 1, !dbg !923
  br label %bb7, !dbg !924

bb4:                                              ; preds = %bb2
  store i8 1, ptr %_0, align 1, !dbg !924
  br label %bb7, !dbg !924
}

; <hashbrown::control::tag::Tag as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN65_$LT$hashbrown..control..tag..Tag$u20$as$u20$core..fmt..Debug$GT$3fmt17h9c56e725206ffa8aE"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #3 !dbg !925 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_15 = alloca [1 x i8], align 1
  %_11 = alloca [12 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !932, !DIExpression(), !934)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !933, !DIExpression(), !935)
  %_4 = load i8, ptr %self, align 1, !dbg !936
; call hashbrown::control::tag::Tag::is_special
  %_3 = call zeroext i1 @_ZN9hashbrown7control3tag3Tag10is_special17h87bbdebc7f6d281eE(i8 %_4) #9, !dbg !937
  br i1 %_3, label %bb2, label %bb6, !dbg !936

bb6:                                              ; preds = %start
; call core::fmt::Formatter::debug_tuple
  call void @_ZN4core3fmt9Formatter11debug_tuple17h6b1332a2f9e3757dE(ptr sret([12 x i8]) align 4 %_11, ptr align 4 %f, ptr align 1 @alloc_3c6431c5fc85ca277eb9f2e0ebb30f52, i32 4) #9, !dbg !938
  %_16 = load i8, ptr %self, align 1, !dbg !939
  %0 = and i8 %_16, 127, !dbg !940
  store i8 %0, ptr %_15, align 1, !dbg !940
; call core::fmt::builders::DebugTuple::field
  %_9 = call align 4 ptr @_ZN4core3fmt8builders10DebugTuple5field17h8986a07ca92d9240E(ptr align 4 %_11, ptr align 1 %_15, ptr align 4 @vtable.0) #9, !dbg !941
; call core::fmt::builders::DebugTuple::finish
  %1 = call zeroext i1 @_ZN4core3fmt8builders10DebugTuple6finish17hc4ba190d2045a9c6E(ptr align 4 %_9) #9, !dbg !942
  %2 = zext i1 %1 to i8, !dbg !942
  store i8 %2, ptr %_0, align 1, !dbg !942
  br label %bb9, !dbg !942

bb2:                                              ; preds = %start
  %_6 = load i8, ptr %self, align 1, !dbg !943
; call hashbrown::control::tag::Tag::special_is_empty
  %_5 = call zeroext i1 @_ZN9hashbrown7control3tag3Tag16special_is_empty17haace7411fda682c6E(i8 %_6) #9, !dbg !944
  br i1 %_5, label %bb4, label %bb5, !dbg !943

bb9:                                              ; preds = %bb4, %bb5, %bb6
  %3 = load i8, ptr %_0, align 1, !dbg !945
  %4 = trunc nuw i8 %3 to i1, !dbg !945
  ret i1 %4, !dbg !945

bb5:                                              ; preds = %bb2
; call core::fmt::Formatter::pad
  %5 = call zeroext i1 @_ZN4core3fmt9Formatter3pad17h9a0270e93f7a94faE(ptr align 4 %f, ptr align 1 @alloc_4d692e00cdd6193df669076a38c2cf3f, i32 7) #9, !dbg !946
  %6 = zext i1 %5 to i8, !dbg !946
  store i8 %6, ptr %_0, align 1, !dbg !946
  br label %bb9, !dbg !946

bb4:                                              ; preds = %bb2
; call core::fmt::Formatter::pad
  %7 = call zeroext i1 @_ZN4core3fmt9Formatter3pad17h9a0270e93f7a94faE(ptr align 4 %f, ptr align 1 @alloc_5ecff085f33eeeacaf38ec6a4e5e2caf, i32 5) #9, !dbg !947
  %8 = zext i1 %7 to i8, !dbg !947
  store i8 %8, ptr %_0, align 1, !dbg !947
  br label %bb9, !dbg !947
}

; <hashbrown::raw::RawIterHashInner as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nounwind
define dso_local { i32, i32 } @"_ZN91_$LT$hashbrown..raw..RawIterHashInner$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfbc9bd705ca4f856E"(ptr align 8 %self) unnamed_addr #3 !dbg !948 {
start:
  %self.dbg.spill.i8 = alloca [4 x i8], align 4
  %self.dbg.spill.i7 = alloca [4 x i8], align 4
  %count.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %b.dbg.spill.i = alloca [1 x i8], align 1
  %_0.i = alloca [1 x i8], align 1
  %group_ctrl.dbg.spill = alloca [4 x i8], align 4
  %index.dbg.spill2 = alloca [4 x i8], align 4
  %index.dbg.spill = alloca [4 x i8], align 4
  %bit.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_2 = alloca [8 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !982, !DIExpression(), !991)
  br label %bb1, !dbg !992

bb1:                                              ; preds = %"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE.exit", %start
  %_3 = getelementptr inbounds i8, ptr %self, i32 16, !dbg !993
; call <hashbrown::control::bitmask::BitMaskIter as core::iter::traits::iterator::Iterator>::next
  %0 = call { i32, i32 } @"_ZN99_$LT$hashbrown..control..bitmask..BitMaskIter$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he508eeb6e77805ebE"(ptr align 8 %_3) #9, !dbg !994
  %1 = extractvalue { i32, i32 } %0, 0, !dbg !994
  %2 = extractvalue { i32, i32 } %0, 1, !dbg !994
  store i32 %1, ptr %_2, align 4, !dbg !994
  %3 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !994
  store i32 %2, ptr %3, align 4, !dbg !994
  %_4 = load i32, ptr %_2, align 4, !dbg !993
  %4 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !993
  %5 = load i32, ptr %4, align 4, !dbg !993
  %6 = trunc nuw i32 %_4 to i1, !dbg !995
  br i1 %6, label %bb3, label %bb5, !dbg !995

bb3:                                              ; preds = %bb1
  %7 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !996
  %bit = load i32, ptr %7, align 4, !dbg !996
  store i32 %bit, ptr %bit.dbg.spill, align 4, !dbg !996
    #dbg_declare(ptr %bit.dbg.spill, !983, !DIExpression(), !996)
  %_8 = load i32, ptr %self, align 8, !dbg !997
  %_9.0 = add i32 %_8, %bit, !dbg !998
  %_9.1 = icmp ult i32 %_9.0, %_8, !dbg !998
  br i1 %_9.1, label %panic, label %bb4, !dbg !998

bb5:                                              ; preds = %bb1
  %8 = getelementptr inbounds i8, ptr %self, i32 8, !dbg !999
  %_14 = load i64, ptr %8, align 8, !dbg !999
; call hashbrown::control::group::generic::Group::match_empty
  %_13 = call i64 @_ZN9hashbrown7control5group7generic5Group11match_empty17h20c425d3959bfde7E(i64 %_14) #9, !dbg !1000
; call hashbrown::control::bitmask::BitMask::any_bit_set
  %_12 = call zeroext i1 @_ZN9hashbrown7control7bitmask7BitMask11any_bit_set17hd8ea81ce0500326fE(i64 %_13) #9, !dbg !1001
  %9 = zext i1 %_12 to i8
  store i8 %9, ptr %b.dbg.spill.i, align 1
    #dbg_declare(ptr %b.dbg.spill.i, !1002, !DIExpression(), !1005)
  br i1 %_12, label %bb1.i, label %bb2.i, !dbg !1007

bb2.i:                                            ; preds = %bb5
  store i8 0, ptr %_0.i, align 1, !dbg !1008
  br label %_ZN4core10intrinsics6likely17h48e44d0be517eb37E.exit, !dbg !1009

bb1.i:                                            ; preds = %bb5
  store i8 1, ptr %_0.i, align 1, !dbg !1010
  br label %_ZN4core10intrinsics6likely17h48e44d0be517eb37E.exit, !dbg !1009

_ZN4core10intrinsics6likely17h48e44d0be517eb37E.exit: ; preds = %bb2.i, %bb1.i
  %10 = load i8, ptr %_0.i, align 1, !dbg !1011
  %11 = trunc nuw i8 %10 to i1, !dbg !1011
  br i1 %11, label %bb9, label %bb10, !dbg !1012

bb4:                                              ; preds = %bb3
  %12 = getelementptr inbounds i8, ptr %self, i32 24, !dbg !1013
  %_10 = load i32, ptr %12, align 8, !dbg !1013
  %index = and i32 %_9.0, %_10, !dbg !998
  store i32 %index, ptr %index.dbg.spill, align 4, !dbg !998
    #dbg_declare(ptr %index.dbg.spill, !985, !DIExpression(), !1014)
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1015
  store i32 %index, ptr %13, align 4, !dbg !1015
  store i32 1, ptr %_0, align 4, !dbg !1015
  br label %bb22, !dbg !1016

panic:                                            ; preds = %bb3
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 @alloc_56e3b117833fff2746fd8cc9c4deec48) #10, !dbg !998
  unreachable, !dbg !998

bb22:                                             ; preds = %bb9, %bb4
  %14 = load i32, ptr %_0, align 4, !dbg !1019
  %15 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1019
  %16 = load i32, ptr %15, align 4, !dbg !1019
  %17 = insertvalue { i32, i32 } poison, i32 %14, 0, !dbg !1019
  %18 = insertvalue { i32, i32 } %17, i32 %16, 1, !dbg !1019
  ret { i32, i32 } %18, !dbg !1019

bb10:                                             ; preds = %_ZN4core10intrinsics6likely17h48e44d0be517eb37E.exit
  %19 = getelementptr inbounds i8, ptr %self, i32 24, !dbg !1020
  %_17 = load i32, ptr %19, align 8, !dbg !1020
; call hashbrown::raw::ProbeSeq::move_next
  call void @_ZN9hashbrown3raw8ProbeSeq9move_next17h1f9f099143cb881bE(ptr align 4 %self, i32 %_17) #9, !dbg !1021
  %index1 = load i32, ptr %self, align 8, !dbg !1022
  store i32 %index1, ptr %index.dbg.spill2, align 4, !dbg !1022
    #dbg_declare(ptr %index.dbg.spill2, !987, !DIExpression(), !1023)
  %20 = getelementptr inbounds i8, ptr %self, i32 24, !dbg !1024
  %_22 = load i32, ptr %20, align 8, !dbg !1024
  %_23.0 = add i32 %_22, 1, !dbg !1024
  %_23.1 = icmp ult i32 %_23.0, %_22, !dbg !1024
  br i1 %_23.1, label %panic3, label %bb12, !dbg !1024

bb9:                                              ; preds = %_ZN4core10intrinsics6likely17h48e44d0be517eb37E.exit
  store i32 0, ptr %_0, align 4, !dbg !1025
  br label %bb22, !dbg !1016

bb12:                                             ; preds = %bb10
  %_24.0 = add i32 %_23.0, 8, !dbg !1024
  %_24.1 = icmp ult i32 %_24.0, %_23.0, !dbg !1024
  br i1 %_24.1, label %panic4, label %bb13, !dbg !1024

panic3:                                           ; preds = %bb10
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 @alloc_ead00003f9c8c7813d7aa60141196161) #10, !dbg !1024
  unreachable, !dbg !1024

bb13:                                             ; preds = %bb12
  %_19 = icmp ult i32 %index1, %_24.0, !dbg !1026
  br i1 %_19, label %bb15, label %bb14, !dbg !1026

panic4:                                           ; preds = %bb12
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 @alloc_ead00003f9c8c7813d7aa60141196161) #10, !dbg !1024
  unreachable, !dbg !1024

bb14:                                             ; preds = %bb13
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h2ce9e1c499078148E(ptr align 1 @alloc_a19464d52a327f94e09214e6632c4e75, i32 61, ptr align 4 @alloc_a2af0b7b77c88948576fb01caff77de4) #10, !dbg !1027
  unreachable, !dbg !1027

bb15:                                             ; preds = %bb13
  %21 = getelementptr inbounds i8, ptr %self, i32 28, !dbg !1028
  %_29 = load ptr, ptr %21, align 4, !dbg !1028
  store ptr %_29, ptr %self.dbg.spill.i8, align 4
    #dbg_declare(ptr %self.dbg.spill.i8, !1029, !DIExpression(), !1036)
  store ptr %_29, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1038, !DIExpression(), !1044)
  store i32 %index1, ptr %count.dbg.spill.i, align 4
    #dbg_declare(ptr %count.dbg.spill.i, !1043, !DIExpression(), !1046)
; call core::ub_checks::check_language_ub
  %_3.i = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17hc9ddcf3001b97422E() #9, !dbg !1047
  br i1 %_3.i, label %bb2.i6, label %"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE.exit", !dbg !1047

bb2.i6:                                           ; preds = %bb15
; call core::ptr::mut_ptr::<impl *mut T>::add::precondition_check
  call void @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18precondition_check17h467862a1054379e6E"(ptr %_29, i32 %index1, i32 1, ptr align 4 @alloc_bfaee7eedbdee0e37a4947f3f8a057e5) #9, !dbg !1049
  br label %"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE.exit", !dbg !1049

"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE.exit": ; preds = %bb15, %bb2.i6
  %_0.i5 = getelementptr inbounds nuw i8, ptr %_29, i32 %index1, !dbg !1050
  store ptr %_0.i5, ptr %self.dbg.spill.i7, align 4
    #dbg_declare(ptr %self.dbg.spill.i7, !1051, !DIExpression(), !1058)
  store ptr %_0.i5, ptr %group_ctrl.dbg.spill, align 4, !dbg !1060
    #dbg_declare(ptr %group_ctrl.dbg.spill, !989, !DIExpression(), !1061)
; call hashbrown::control::group::generic::Group::load
  %_30 = call i64 @_ZN9hashbrown7control5group7generic5Group4load17ha6fcdb859ff39a02E(ptr %_0.i5) #9, !dbg !1062
  %22 = getelementptr inbounds i8, ptr %self, i32 8, !dbg !1063
  store i64 %_30, ptr %22, align 8, !dbg !1063
  %23 = getelementptr inbounds i8, ptr %self, i32 8, !dbg !1064
  %_34 = load i64, ptr %23, align 8, !dbg !1064
  %24 = getelementptr inbounds i8, ptr %self, i32 32, !dbg !1065
  %_35 = load i8, ptr %24, align 8, !dbg !1065
; call hashbrown::control::group::generic::Group::match_tag
  %_33 = call i64 @_ZN9hashbrown7control5group7generic5Group9match_tag17h7d9d885232d15550E(i64 %_34, i8 %_35) #9, !dbg !1066
; call <hashbrown::control::bitmask::BitMask as core::iter::traits::collect::IntoIterator>::into_iter
  %_32 = call i64 @"_ZN98_$LT$hashbrown..control..bitmask..BitMask$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h1a833b0d45907438E"(i64 %_33) #9, !dbg !1067
  %25 = getelementptr inbounds i8, ptr %self, i32 16, !dbg !1068
  store i64 %_32, ptr %25, align 8, !dbg !1068
  br label %bb1, !dbg !992

bb23:                                             ; No predecessors!
  unreachable, !dbg !1069
}

; <hashbrown::control::bitmask::BitMask as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: inlinehint nounwind
define internal i64 @"_ZN98_$LT$hashbrown..control..bitmask..BitMask$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h1a833b0d45907438E"(i64 %self) unnamed_addr #1 !dbg !1070 {
start:
  %self.dbg.spill = alloca [8 x i8], align 8
  store i64 %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !1076, !DIExpression(), !1077)
  %_3 = and i64 %self, -1, !dbg !1078
  ret i64 %_3, !dbg !1079
}

; <hashbrown::control::bitmask::BitMaskIter as core::iter::traits::iterator::Iterator>::next
; Function Attrs: inlinehint nounwind
define internal { i32, i32 } @"_ZN99_$LT$hashbrown..control..bitmask..BitMaskIter$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he508eeb6e77805ebE"(ptr align 8 %self) unnamed_addr #1 !dbg !1080 {
start:
  %bit.dbg.spill = alloca [4 x i8], align 4
  %residual.dbg.spill = alloca [0 x i8], align 1
  %self.dbg.spill = alloca [4 x i8], align 4
  %_2 = alloca [8 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1086, !DIExpression(), !1109)
    #dbg_declare(ptr %residual.dbg.spill, !1089, !DIExpression(), !1110)
  %_4 = load i64, ptr %self, align 8, !dbg !1111
; call hashbrown::control::bitmask::BitMask::lowest_set_bit
  %0 = call { i32, i32 } @_ZN9hashbrown7control7bitmask7BitMask14lowest_set_bit17heba4e93fefcfcf86E(i64 %_4) #9, !dbg !1112
  %_3.0 = extractvalue { i32, i32 } %0, 0, !dbg !1112
  %_3.1 = extractvalue { i32, i32 } %0, 1, !dbg !1112
; call <core::option::Option<T> as core::ops::try_trait::Try>::branch
  %1 = call { i32, i32 } @"_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h58df85d711a95350E"(i32 %_3.0, i32 %_3.1) #9, !dbg !1111
  %2 = extractvalue { i32, i32 } %1, 0, !dbg !1111
  %3 = extractvalue { i32, i32 } %1, 1, !dbg !1111
  store i32 %2, ptr %_2, align 4, !dbg !1111
  %4 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !1111
  store i32 %3, ptr %4, align 4, !dbg !1111
  %_5 = load i32, ptr %_2, align 4, !dbg !1111
  %5 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !1111
  %6 = load i32, ptr %5, align 4, !dbg !1111
  %7 = trunc nuw i32 %_5 to i1, !dbg !1111
  br i1 %7, label %bb5, label %bb4, !dbg !1111

bb5:                                              ; preds = %start
; call <core::option::Option<T> as core::ops::try_trait::FromResidual<core::option::Option<core::convert::Infallible>>>::from_residual
  %8 = call { i32, i32 } @"_ZN145_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..option..Option$LT$core..convert..Infallible$GT$$GT$$GT$13from_residual17h220f2d7a12a7e3c7E"() #9, !dbg !1113
  %9 = extractvalue { i32, i32 } %8, 0, !dbg !1113
  %10 = extractvalue { i32, i32 } %8, 1, !dbg !1113
  store i32 %9, ptr %_0, align 4, !dbg !1113
  %11 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1113
  store i32 %10, ptr %11, align 4, !dbg !1113
  br label %bb7, !dbg !1113

bb4:                                              ; preds = %start
  %12 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !1111
  %bit = load i32, ptr %12, align 4, !dbg !1111
  store i32 %bit, ptr %bit.dbg.spill, align 4, !dbg !1111
    #dbg_declare(ptr %bit.dbg.spill, !1087, !DIExpression(), !1114)
    #dbg_declare(ptr %bit.dbg.spill, !1107, !DIExpression(), !1115)
  %_8 = load i64, ptr %self, align 8, !dbg !1116
; call hashbrown::control::bitmask::BitMask::remove_lowest_bit
  %_7 = call i64 @_ZN9hashbrown7control7bitmask7BitMask17remove_lowest_bit17h74eb0823a1635631E(i64 %_8) #9, !dbg !1117
  store i64 %_7, ptr %self, align 8, !dbg !1118
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1119
  store i32 %bit, ptr %13, align 4, !dbg !1119
  store i32 1, ptr %_0, align 4, !dbg !1119
  br label %bb7, !dbg !1120

bb7:                                              ; preds = %bb5, %bb4
  %14 = load i32, ptr %_0, align 4, !dbg !1120
  %15 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1120
  %16 = load i32, ptr %15, align 4, !dbg !1120
  %17 = insertvalue { i32, i32 } poison, i32 %14, 0, !dbg !1120
  %18 = insertvalue { i32, i32 } %17, i32 %16, 1, !dbg !1120
  ret { i32, i32 } %18, !dbg !1120

bb3:                                              ; No predecessors!
  unreachable, !dbg !1111
}

; hashbrown::raw::Fallibility::capacity_overflow
; Function Attrs: nounwind
define dso_local { i32, i32 } @_ZN9hashbrown3raw11Fallibility17capacity_overflow17hda25fc3126d0db47E(i1 zeroext %self) unnamed_addr #3 !dbg !1121 {
start:
  %self.dbg.spill = alloca [1 x i8], align 1
  %_4 = alloca [24 x i8], align 4
  %0 = zext i1 %self to i8
  store i8 %0, ptr %self.dbg.spill, align 1
    #dbg_declare(ptr %self.dbg.spill, !1146, !DIExpression(), !1147)
  %_2 = zext i1 %self to i32, !dbg !1148
  %1 = trunc nuw i32 %_2 to i1, !dbg !1149
  br i1 %1, label %bb2, label %bb3, !dbg !1149

bb2:                                              ; preds = %start
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_4, ptr align 4 @alloc_b0acc26568cfe85a22c804c4e29c0e28) #9, !dbg !1150
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_4, ptr align 4 @alloc_b7c0248fcba32176f5ad13cb0b6fc9ed) #10, !dbg !1150
  unreachable, !dbg !1150

bb3:                                              ; preds = %start
  ret { i32, i32 } { i32 0, i32 undef }, !dbg !1151

bb1:                                              ; No predecessors!
  unreachable, !dbg !1148
}

; hashbrown::raw::Fallibility::alloc_err
; Function Attrs: nounwind
define dso_local { i32, i32 } @_ZN9hashbrown3raw11Fallibility9alloc_err17h85a86bfcda93906aE(i1 zeroext %self, i32 %layout.0, i32 %layout.1) unnamed_addr #3 !dbg !1152 {
start:
  %layout.dbg.spill = alloca [8 x i8], align 4
  %self.dbg.spill = alloca [1 x i8], align 1
  %0 = zext i1 %self to i8
  store i8 %0, ptr %self.dbg.spill, align 1
    #dbg_declare(ptr %self.dbg.spill, !1157, !DIExpression(), !1159)
  store i32 %layout.0, ptr %layout.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %layout.dbg.spill, i32 4
  store i32 %layout.1, ptr %1, align 4
    #dbg_declare(ptr %layout.dbg.spill, !1158, !DIExpression(), !1160)
  %_3 = zext i1 %self to i32, !dbg !1161
  %2 = trunc nuw i32 %_3 to i1, !dbg !1162
  br i1 %2, label %bb2, label %bb3, !dbg !1162

bb2:                                              ; preds = %start
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h1e5cf49dcf30c6c7E(i32 %layout.0, i32 %layout.1) #10, !dbg !1163
  unreachable, !dbg !1163

bb3:                                              ; preds = %start
  %3 = insertvalue { i32, i32 } poison, i32 %layout.0, 0, !dbg !1164
  %4 = insertvalue { i32, i32 } %3, i32 %layout.1, 1, !dbg !1164
  ret { i32, i32 } %4, !dbg !1164

bb1:                                              ; No predecessors!
  unreachable, !dbg !1161
}

; hashbrown::raw::RawTableInner::ctrl_slice
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN9hashbrown3raw13RawTableInner10ctrl_slice17h1789bac88fa3cb82E(ptr align 4 %self) unnamed_addr #3 !dbg !1165 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1177, !DIExpression(), !1178)
  %_4 = load ptr, ptr %self, align 4, !dbg !1179
  store ptr %_4, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1029, !DIExpression(), !1180)
  store ptr %_4, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1051, !DIExpression(), !1182)
; call hashbrown::raw::RawTableInner::num_ctrl_bytes
  %_5 = call i32 @_ZN9hashbrown3raw13RawTableInner14num_ctrl_bytes17h95bbfa87967f2288E(ptr align 4 %self) #9, !dbg !1184
; call core::slice::raw::from_raw_parts_mut
  %0 = call { ptr, i32 } @_ZN4core5slice3raw18from_raw_parts_mut17heeb025ab382939bbE(ptr %_4, i32 %_5, ptr align 4 @alloc_33b2c073cf1eee2d5a4556da9ce40539) #9, !dbg !1185
  %_0.0 = extractvalue { ptr, i32 } %0, 0, !dbg !1185
  %_0.1 = extractvalue { ptr, i32 } %0, 1, !dbg !1185
  %1 = insertvalue { ptr, i32 } poison, ptr %_0.0, 0, !dbg !1186
  %2 = insertvalue { ptr, i32 } %1, i32 %_0.1, 1, !dbg !1186
  ret { ptr, i32 } %2, !dbg !1186
}

; hashbrown::raw::RawTableInner::num_ctrl_bytes
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN9hashbrown3raw13RawTableInner14num_ctrl_bytes17h95bbfa87967f2288E(ptr align 4 %self) unnamed_addr #1 !dbg !1187 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1193, !DIExpression(), !1194)
  %0 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1195
  %_3 = load i32, ptr %0, align 4, !dbg !1195
  %_4.0 = add i32 %_3, 1, !dbg !1195
  %_4.1 = icmp ult i32 %_4.0, %_3, !dbg !1195
  br i1 %_4.1, label %panic, label %bb1, !dbg !1195

bb1:                                              ; preds = %start
  %_5.0 = add i32 %_4.0, 8, !dbg !1195
  %_5.1 = icmp ult i32 %_5.0, %_4.0, !dbg !1195
  br i1 %_5.1, label %panic1, label %bb2, !dbg !1195

panic:                                            ; preds = %start
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 @alloc_c2961626c79d8147cae5a8a0428ebaf1) #10, !dbg !1195
  unreachable, !dbg !1195

bb2:                                              ; preds = %bb1
  ret i32 %_5.0, !dbg !1196

panic1:                                           ; preds = %bb1
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 @alloc_c2961626c79d8147cae5a8a0428ebaf1) #10, !dbg !1195
  unreachable, !dbg !1195
}

; hashbrown::raw::RawTableInner::ctrl
; Function Attrs: inlinehint nounwind
define internal ptr @_ZN9hashbrown3raw13RawTableInner4ctrl17h5dd6d54905c726cfE(ptr align 4 %self, i32 %index) unnamed_addr #1 !dbg !1197 {
start:
  %self.dbg.spill.i2 = alloca [4 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %count.dbg.spill.i = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %index.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1202, !DIExpression(), !1204)
  store i32 %index, ptr %index.dbg.spill, align 4
    #dbg_declare(ptr %index.dbg.spill, !1203, !DIExpression(), !1205)
; call hashbrown::raw::RawTableInner::num_ctrl_bytes
  %_4 = call i32 @_ZN9hashbrown3raw13RawTableInner14num_ctrl_bytes17h95bbfa87967f2288E(ptr align 4 %self) #9, !dbg !1206
  %_3 = icmp ult i32 %index, %_4, !dbg !1207
  br i1 %_3, label %bb3, label %bb2, !dbg !1207

bb2:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h2ce9e1c499078148E(ptr align 1 @alloc_d4ba273af71dec0b6121025ad15b0934, i32 47, ptr align 4 @alloc_f727df41d5c68d57374e6b6dd87eb7c6) #10, !dbg !1208
  unreachable, !dbg !1208

bb3:                                              ; preds = %start
  %_8 = load ptr, ptr %self, align 4, !dbg !1209
  store ptr %_8, ptr %self.dbg.spill.i2, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !1029, !DIExpression(), !1210)
  store ptr %_8, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1038, !DIExpression(), !1212)
  store i32 %index, ptr %count.dbg.spill.i, align 4
    #dbg_declare(ptr %count.dbg.spill.i, !1043, !DIExpression(), !1214)
; call core::ub_checks::check_language_ub
  %_3.i = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17hc9ddcf3001b97422E() #9, !dbg !1215
  br i1 %_3.i, label %bb2.i, label %"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE.exit", !dbg !1215

bb2.i:                                            ; preds = %bb3
; call core::ptr::mut_ptr::<impl *mut T>::add::precondition_check
  call void @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18precondition_check17h467862a1054379e6E"(ptr %_8, i32 %index, i32 1, ptr align 4 @alloc_08178e3705f6cb88f90b8c2db9b287d5) #9, !dbg !1216
  br label %"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE.exit", !dbg !1216

"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE.exit": ; preds = %bb3, %bb2.i
  %_0.i = getelementptr inbounds nuw i8, ptr %_8, i32 %index, !dbg !1217
  store ptr %_0.i, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1051, !DIExpression(), !1218)
  ret ptr %_0.i, !dbg !1220
}

; hashbrown::raw::RawTableInner::probe_seq
; Function Attrs: inlinehint nounwind
define internal { i32, i32 } @_ZN9hashbrown3raw13RawTableInner9probe_seq17h4ad3f7f3d59e20f9E(ptr align 4 %self, i64 %hash) unnamed_addr #1 !dbg !1221 {
start:
  %hash.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1226, !DIExpression(), !1228)
  store i64 %hash, ptr %hash.dbg.spill, align 8
    #dbg_declare(ptr %hash.dbg.spill, !1227, !DIExpression(), !1229)
; call hashbrown::raw::h1
  %_4 = call i32 @_ZN9hashbrown3raw2h117hb6013fe87d518607E(i64 %hash) #9, !dbg !1230
  %0 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1231
  %_5 = load i32, ptr %0, align 4, !dbg !1231
  %_3 = and i32 %_4, %_5, !dbg !1230
  %1 = insertvalue { i32, i32 } poison, i32 %_3, 0, !dbg !1232
  %2 = insertvalue { i32, i32 } %1, i32 0, 1, !dbg !1232
  ret { i32, i32 } %2, !dbg !1232
}

; hashbrown::raw::RawIterHashInner::new
; Function Attrs: nounwind
define dso_local void @_ZN9hashbrown3raw16RawIterHashInner3new17h27f9036dc045e3b3E(ptr sret([40 x i8]) align 8 %_0, ptr align 4 %table, i64 %hash) unnamed_addr #3 !dbg !1233 {
start:
  %bitmask.dbg.spill = alloca [8 x i8], align 8
  %group.dbg.spill = alloca [8 x i8], align 8
  %probe_seq.dbg.spill = alloca [8 x i8], align 4
  %tag_hash.dbg.spill = alloca [1 x i8], align 1
  %hash.dbg.spill = alloca [8 x i8], align 8
  %table.dbg.spill = alloca [4 x i8], align 4
  store ptr %table, ptr %table.dbg.spill, align 4
    #dbg_declare(ptr %table.dbg.spill, !1238, !DIExpression(), !1248)
  store i64 %hash, ptr %hash.dbg.spill, align 8
    #dbg_declare(ptr %hash.dbg.spill, !1239, !DIExpression(), !1249)
; call hashbrown::control::tag::Tag::full
  %tag_hash = call i8 @_ZN9hashbrown7control3tag3Tag4full17hae36b61f1b1ec9c0E(i64 %hash) #9, !dbg !1250
  store i8 %tag_hash, ptr %tag_hash.dbg.spill, align 1, !dbg !1250
    #dbg_declare(ptr %tag_hash.dbg.spill, !1240, !DIExpression(), !1251)
; call hashbrown::raw::RawTableInner::probe_seq
  %0 = call { i32, i32 } @_ZN9hashbrown3raw13RawTableInner9probe_seq17h4ad3f7f3d59e20f9E(ptr align 4 %table, i64 %hash) #9, !dbg !1252
  %probe_seq.0 = extractvalue { i32, i32 } %0, 0, !dbg !1252
  %probe_seq.1 = extractvalue { i32, i32 } %0, 1, !dbg !1252
  store i32 %probe_seq.0, ptr %probe_seq.dbg.spill, align 4, !dbg !1252
  %1 = getelementptr inbounds i8, ptr %probe_seq.dbg.spill, i32 4, !dbg !1252
  store i32 %probe_seq.1, ptr %1, align 4, !dbg !1252
    #dbg_declare(ptr %probe_seq.dbg.spill, !1242, !DIExpression(), !1253)
; call hashbrown::raw::RawTableInner::ctrl
  %_7 = call ptr @_ZN9hashbrown3raw13RawTableInner4ctrl17h5dd6d54905c726cfE(ptr align 4 %table, i32 %probe_seq.0) #9, !dbg !1254
; call hashbrown::control::group::generic::Group::load
  %group = call i64 @_ZN9hashbrown7control5group7generic5Group4load17ha6fcdb859ff39a02E(ptr %_7) #9, !dbg !1255
  store i64 %group, ptr %group.dbg.spill, align 8, !dbg !1255
    #dbg_declare(ptr %group.dbg.spill, !1244, !DIExpression(), !1256)
; call hashbrown::control::group::generic::Group::match_tag
  %_10 = call i64 @_ZN9hashbrown7control5group7generic5Group9match_tag17h7d9d885232d15550E(i64 %group, i8 %tag_hash) #9, !dbg !1257
; call <hashbrown::control::bitmask::BitMask as core::iter::traits::collect::IntoIterator>::into_iter
  %bitmask = call i64 @"_ZN98_$LT$hashbrown..control..bitmask..BitMask$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h1a833b0d45907438E"(i64 %_10) #9, !dbg !1258
  store i64 %bitmask, ptr %bitmask.dbg.spill, align 8, !dbg !1258
    #dbg_declare(ptr %bitmask.dbg.spill, !1246, !DIExpression(), !1259)
  %2 = getelementptr inbounds i8, ptr %table, i32 4, !dbg !1260
  %_11 = load i32, ptr %2, align 4, !dbg !1260
  %_12 = load ptr, ptr %table, align 4, !dbg !1261
  %3 = getelementptr inbounds i8, ptr %_0, i32 24, !dbg !1262
  store i32 %_11, ptr %3, align 8, !dbg !1262
  %4 = getelementptr inbounds i8, ptr %_0, i32 28, !dbg !1262
  store ptr %_12, ptr %4, align 4, !dbg !1262
  %5 = getelementptr inbounds i8, ptr %_0, i32 32, !dbg !1262
  store i8 %tag_hash, ptr %5, align 8, !dbg !1262
  store i32 %probe_seq.0, ptr %_0, align 8, !dbg !1262
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1262
  store i32 %probe_seq.1, ptr %6, align 4, !dbg !1262
  %7 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !1262
  store i64 %group, ptr %7, align 8, !dbg !1262
  %8 = getelementptr inbounds i8, ptr %_0, i32 16, !dbg !1262
  store i64 %bitmask, ptr %8, align 8, !dbg !1262
  ret void, !dbg !1263
}

; hashbrown::raw::h1
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN9hashbrown3raw2h117hb6013fe87d518607E(i64 %hash) unnamed_addr #1 !dbg !1264 {
start:
  %hash.dbg.spill = alloca [8 x i8], align 8
  store i64 %hash, ptr %hash.dbg.spill, align 8
    #dbg_declare(ptr %hash.dbg.spill, !1268, !DIExpression(), !1269)
  %_0 = trunc i64 %hash to i32, !dbg !1270
  ret i32 %_0, !dbg !1271
}

; hashbrown::raw::ProbeSeq::move_next
; Function Attrs: inlinehint nounwind
define internal void @_ZN9hashbrown3raw8ProbeSeq9move_next17h1f9f099143cb881bE(ptr align 4 %self, i32 %bucket_mask) unnamed_addr #1 !dbg !1272 {
start:
  %bucket_mask.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_6 = alloca [24 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1278, !DIExpression(), !1280)
  store i32 %bucket_mask, ptr %bucket_mask.dbg.spill, align 4
    #dbg_declare(ptr %bucket_mask.dbg.spill, !1279, !DIExpression(), !1281)
  %0 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1282
  %_4 = load i32, ptr %0, align 4, !dbg !1282
  %_3 = icmp ule i32 %_4, %bucket_mask, !dbg !1282
  br i1 %_3, label %bb3, label %bb1, !dbg !1282

bb1:                                              ; preds = %start
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_6, ptr align 4 @alloc_2d3a64d43c9f7190b430f73d81e3ba6f) #9, !dbg !1283
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E(ptr align 4 %_6, ptr align 4 @alloc_8a92bc86d0cd940ed16717c77a01a2d3) #10, !dbg !1283
  unreachable, !dbg !1283

bb3:                                              ; preds = %start
  %1 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1284
  %2 = load i32, ptr %1, align 4, !dbg !1284
  %_8.0 = add i32 %2, 8, !dbg !1284
  %_8.1 = icmp ult i32 %_8.0, %2, !dbg !1284
  br i1 %_8.1, label %panic, label %bb4, !dbg !1284

bb4:                                              ; preds = %bb3
  %3 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1284
  store i32 %_8.0, ptr %3, align 4, !dbg !1284
  %4 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !1285
  %_9 = load i32, ptr %4, align 4, !dbg !1285
  %5 = load i32, ptr %self, align 4, !dbg !1286
  %_10.0 = add i32 %5, %_9, !dbg !1286
  %_10.1 = icmp ult i32 %_10.0, %5, !dbg !1286
  br i1 %_10.1, label %panic1, label %bb5, !dbg !1286

panic:                                            ; preds = %bb3
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 @alloc_700e6d87b4783c87b3010586a7a02ce7) #10, !dbg !1284
  unreachable, !dbg !1284

bb5:                                              ; preds = %bb4
  store i32 %_10.0, ptr %self, align 4, !dbg !1286
  %6 = load i32, ptr %self, align 4, !dbg !1287
  %7 = and i32 %6, %bucket_mask, !dbg !1287
  store i32 %7, ptr %self, align 4, !dbg !1287
  ret void, !dbg !1288

panic1:                                           ; preds = %bb4
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E(ptr align 4 @alloc_5407fcb00837cebb06d4c6a70ab2ff30) #10, !dbg !1286
  unreachable, !dbg !1286
}

; hashbrown::control::tag::Tag::is_special
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN9hashbrown7control3tag3Tag10is_special17h87bbdebc7f6d281eE(i8 %self) unnamed_addr #1 !dbg !1289 {
start:
  %self.dbg.spill = alloca [1 x i8], align 1
  store i8 %self, ptr %self.dbg.spill, align 1
    #dbg_declare(ptr %self.dbg.spill, !1294, !DIExpression(), !1295)
  %_2 = and i8 %self, -128, !dbg !1296
  %_0 = icmp ne i8 %_2, 0, !dbg !1296
  ret i1 %_0, !dbg !1297
}

; hashbrown::control::tag::Tag::special_is_empty
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN9hashbrown7control3tag3Tag16special_is_empty17haace7411fda682c6E(i8 %self) unnamed_addr #1 !dbg !1298 {
start:
  %self.dbg.spill = alloca [1 x i8], align 1
  store i8 %self, ptr %self.dbg.spill, align 1
    #dbg_declare(ptr %self.dbg.spill, !1301, !DIExpression(), !1302)
; call hashbrown::control::tag::Tag::is_special
  %_2 = call zeroext i1 @_ZN9hashbrown7control3tag3Tag10is_special17h87bbdebc7f6d281eE(i8 %self) #9, !dbg !1303
  br i1 %_2, label %bb3, label %bb2, !dbg !1304

bb2:                                              ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h2ce9e1c499078148E(ptr align 1 @alloc_415d34d1a55ce06d02b8d5c94fa2ff39, i32 35, ptr align 4 @alloc_9168bccc6b35eb246e9e8c9f797cb996) #10, !dbg !1305
  unreachable, !dbg !1305

bb3:                                              ; preds = %start
  %_4 = and i8 %self, 1, !dbg !1306
  %_0 = icmp ne i8 %_4, 0, !dbg !1306
  ret i1 %_0, !dbg !1307
}

; hashbrown::control::tag::Tag::full
; Function Attrs: inlinehint nounwind
define internal i8 @_ZN9hashbrown7control3tag3Tag4full17hae36b61f1b1ec9c0E(i64 %hash) unnamed_addr #1 !dbg !1308 {
start:
  %top7.dbg.spill = alloca [8 x i8], align 8
  %hash.dbg.spill = alloca [8 x i8], align 8
  store i64 %hash, ptr %hash.dbg.spill, align 8
    #dbg_declare(ptr %hash.dbg.spill, !1313, !DIExpression(), !1316)
  %0 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 4, i32 8), !dbg !1317
  %_5.0 = extractvalue { i32, i1 } %0, 0, !dbg !1317
  %_5.1 = extractvalue { i32, i1 } %0, 1, !dbg !1317
  br i1 %_5.1, label %panic, label %bb1, !dbg !1317

bb1:                                              ; preds = %start
  %_6.0 = sub i32 %_5.0, 7, !dbg !1318
  %_6.1 = icmp ult i32 %_5.0, 7, !dbg !1318
  br i1 %_6.1, label %panic1, label %bb2, !dbg !1318

panic:                                            ; preds = %start
; call core::panicking::panic_const::panic_const_mul_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_mul_overflow17hc3f2f5648b4a5121E(ptr align 4 @alloc_623a68bf9f8ce9878e65dbae43be02d8) #10, !dbg !1317
  unreachable, !dbg !1317

bb2:                                              ; preds = %bb1
  %_7 = icmp ult i32 %_6.0, 64, !dbg !1319
  br i1 %_7, label %bb3, label %panic2, !dbg !1319

panic1:                                           ; preds = %bb1
; call core::panicking::panic_const::panic_const_sub_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_sub_overflow17h7c3bd49388308f57E(ptr align 4 @alloc_e773017bdfe8f67da92e8b6a7810af7e) #10, !dbg !1318
  unreachable, !dbg !1318

bb3:                                              ; preds = %bb2
  %1 = and i32 %_6.0, 63, !dbg !1319
  %2 = zext i32 %1 to i64, !dbg !1319
  %top7 = lshr i64 %hash, %2, !dbg !1319
  store i64 %top7, ptr %top7.dbg.spill, align 8, !dbg !1319
    #dbg_declare(ptr %top7.dbg.spill, !1314, !DIExpression(), !1320)
  %_9 = and i64 %top7, 127, !dbg !1321
  %_8 = trunc i64 %_9 to i8, !dbg !1321
  ret i8 %_8, !dbg !1322

panic2:                                           ; preds = %bb2
; call core::panicking::panic_const::panic_const_shr_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_shr_overflow17ha5f77e543b98d47cE(ptr align 4 @alloc_a62a01c8c371fa30a0eec53cfad7d3e9) #10, !dbg !1319
  unreachable, !dbg !1319
}

; hashbrown::control::group::generic::Group::match_empty
; Function Attrs: inlinehint nounwind
define internal i64 @_ZN9hashbrown7control5group7generic5Group11match_empty17h20c425d3959bfde7E(i64 %self) unnamed_addr #1 !dbg !1323 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  store i64 %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !1329, !DIExpression(), !1330)
  %_6 = shl i64 %self, 1, !dbg !1331
  %_4 = and i64 %self, %_6, !dbg !1332
; call hashbrown::control::group::generic::repeat
  %_10 = call i64 @_ZN9hashbrown7control5group7generic6repeat17hb738815a69dabe76E(i8 -128) #9, !dbg !1333
  %_3 = and i64 %_4, %_10, !dbg !1334
  store i64 %_3, ptr %self.dbg.spill.i, align 8
    #dbg_declare(ptr %self.dbg.spill.i, !1335, !DIExpression(), !1340)
  ret i64 %_3, !dbg !1342
}

; hashbrown::control::group::generic::Group::load
; Function Attrs: inlinehint nounwind
define internal i64 @_ZN9hashbrown7control5group7generic5Group4load17ha6fcdb859ff39a02E(ptr %ptr) unnamed_addr #1 !dbg !1343 {
start:
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !1349, !DIExpression(), !1350)
  store ptr %ptr, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1351, !DIExpression(), !1357)
; call core::ptr::read_unaligned
  %_2 = call i64 @_ZN4core3ptr14read_unaligned17h6d1ad2df12929142E(ptr %ptr, ptr align 4 @alloc_054d4fbbe178366ea5a2c31d76957b35) #9, !dbg !1359
  ret i64 %_2, !dbg !1360
}

; hashbrown::control::group::generic::Group::match_tag
; Function Attrs: inlinehint nounwind
define internal i64 @_ZN9hashbrown7control5group7generic5Group9match_tag17h7d9d885232d15550E(i64 %self, i8 %tag) unnamed_addr #1 !dbg !1361 {
start:
  %self.dbg.spill.i1 = alloca [8 x i8], align 8
  %rhs.dbg.spill.i = alloca [8 x i8], align 8
  %self.dbg.spill.i = alloca [8 x i8], align 8
  %cmp.dbg.spill = alloca [8 x i8], align 8
  %tag.dbg.spill = alloca [1 x i8], align 1
  %self.dbg.spill = alloca [8 x i8], align 8
  store i64 %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !1366, !DIExpression(), !1370)
  store i8 %tag, ptr %tag.dbg.spill, align 1
    #dbg_declare(ptr %tag.dbg.spill, !1367, !DIExpression(), !1371)
; call hashbrown::control::group::generic::repeat
  %_5 = call i64 @_ZN9hashbrown7control5group7generic6repeat17hb738815a69dabe76E(i8 %tag) #9, !dbg !1372
  %cmp = xor i64 %self, %_5, !dbg !1373
  store i64 %cmp, ptr %cmp.dbg.spill, align 8, !dbg !1373
    #dbg_declare(ptr %cmp.dbg.spill, !1368, !DIExpression(), !1374)
; call hashbrown::control::group::generic::repeat
  %_10 = call i64 @_ZN9hashbrown7control5group7generic6repeat17hb738815a69dabe76E(i8 1) #9, !dbg !1375
  store i64 %cmp, ptr %self.dbg.spill.i, align 8
    #dbg_declare(ptr %self.dbg.spill.i, !1376, !DIExpression(), !1382)
  store i64 %_10, ptr %rhs.dbg.spill.i, align 8
    #dbg_declare(ptr %rhs.dbg.spill.i, !1381, !DIExpression(), !1384)
  %_0.i = sub i64 %cmp, %_10, !dbg !1385
  %_12 = xor i64 %cmp, -1, !dbg !1386
  %_8 = and i64 %_0.i, %_12, !dbg !1387
; call hashbrown::control::group::generic::repeat
  %_13 = call i64 @_ZN9hashbrown7control5group7generic6repeat17hb738815a69dabe76E(i8 -128) #9, !dbg !1388
  %_7 = and i64 %_8, %_13, !dbg !1389
  store i64 %_7, ptr %self.dbg.spill.i1, align 8
    #dbg_declare(ptr %self.dbg.spill.i1, !1335, !DIExpression(), !1390)
  ret i64 %_7, !dbg !1392
}

; hashbrown::control::group::generic::repeat
; Function Attrs: inlinehint nounwind
define internal i64 @_ZN9hashbrown7control5group7generic6repeat17hb738815a69dabe76E(i8 %tag) unnamed_addr #1 !dbg !1393 {
start:
  %tag.dbg.spill = alloca [1 x i8], align 1
  %_2 = alloca [8 x i8], align 1
  store i8 %tag, ptr %tag.dbg.spill, align 1
    #dbg_declare(ptr %tag.dbg.spill, !1397, !DIExpression(), !1398)
  call void @llvm.memset.p0.i32(ptr align 1 %_2, i8 %tag, i32 8, i1 false), !dbg !1399
; call core::num::<impl u64>::from_ne_bytes
  %_0 = call i64 @"_ZN4core3num21_$LT$impl$u20$u64$GT$13from_ne_bytes17ha6d065afd5223f55E"(ptr align 1 %_2) #9, !dbg !1400
  ret i64 %_0, !dbg !1401
}

; hashbrown::control::bitmask::BitMask::any_bit_set
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN9hashbrown7control7bitmask7BitMask11any_bit_set17hd8ea81ce0500326fE(i64 %self) unnamed_addr #1 !dbg !1402 {
start:
  %self.dbg.spill = alloca [8 x i8], align 8
  store i64 %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !1407, !DIExpression(), !1408)
  %_0 = icmp ne i64 %self, 0, !dbg !1409
  ret i1 %_0, !dbg !1410
}

; hashbrown::control::bitmask::BitMask::lowest_set_bit
; Function Attrs: inlinehint nounwind
define internal { i32, i32 } @_ZN9hashbrown7control7bitmask7BitMask14lowest_set_bit17heba4e93fefcfcf86E(i64 %self) unnamed_addr #1 !dbg !1411 {
start:
  %nonzero.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [8 x i8], align 8
  %_0 = alloca [8 x i8], align 4
  store i64 %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !1416, !DIExpression(), !1419)
; call core::num::nonzero::NonZero<T>::new
  %0 = call i64 @"_ZN4core3num7nonzero16NonZero$LT$T$GT$3new17heb146e1fa1755ff2E"(i64 %self) #9, !dbg !1420
  store i64 %0, ptr %_2, align 8, !dbg !1420
  %1 = load i64, ptr %_2, align 8, !dbg !1420
  %2 = icmp eq i64 %1, 0, !dbg !1420
  %_4 = select i1 %2, i32 0, i32 1, !dbg !1420
  %3 = trunc nuw i32 %_4 to i1, !dbg !1421
  br i1 %3, label %bb2, label %bb4, !dbg !1421

bb2:                                              ; preds = %start
  %nonzero = load i64, ptr %_2, align 8, !dbg !1422
  store i64 %nonzero, ptr %nonzero.dbg.spill, align 8, !dbg !1422
    #dbg_declare(ptr %nonzero.dbg.spill, !1417, !DIExpression(), !1422)
; call hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
  %_6 = call i32 @_ZN9hashbrown7control7bitmask7BitMask22nonzero_trailing_zeros17h6e7d56c1b164da0bE(i64 %nonzero) #9, !dbg !1423
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1424
  store i32 %_6, ptr %4, align 4, !dbg !1424
  store i32 1, ptr %_0, align 4, !dbg !1424
  br label %bb5, !dbg !1425

bb4:                                              ; preds = %start
  store i32 0, ptr %_0, align 4, !dbg !1426
  br label %bb5, !dbg !1425

bb5:                                              ; preds = %bb4, %bb2
  %5 = load i32, ptr %_0, align 4, !dbg !1427
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1427
  %7 = load i32, ptr %6, align 4, !dbg !1427
  %8 = insertvalue { i32, i32 } poison, i32 %5, 0, !dbg !1427
  %9 = insertvalue { i32, i32 } %8, i32 %7, 1, !dbg !1427
  ret { i32, i32 } %9, !dbg !1427

bb6:                                              ; No predecessors!
  unreachable, !dbg !1428
}

; hashbrown::control::bitmask::BitMask::remove_lowest_bit
; Function Attrs: inlinehint nounwind
define internal i64 @_ZN9hashbrown7control7bitmask7BitMask17remove_lowest_bit17h74eb0823a1635631E(i64 %self) unnamed_addr #1 !dbg !1429 {
start:
  %self.dbg.spill = alloca [8 x i8], align 8
  store i64 %self, ptr %self.dbg.spill, align 8
    #dbg_declare(ptr %self.dbg.spill, !1434, !DIExpression(), !1435)
  %_6.0 = sub i64 %self, 1, !dbg !1436
  %_6.1 = icmp ult i64 %self, 1, !dbg !1436
  br i1 %_6.1, label %panic, label %bb1, !dbg !1436

bb1:                                              ; preds = %start
  %_2 = and i64 %self, %_6.0, !dbg !1437
  ret i64 %_2, !dbg !1438

panic:                                            ; preds = %start
; call core::panicking::panic_const::panic_const_sub_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_sub_overflow17h7c3bd49388308f57E(ptr align 4 @alloc_1dbe95295766279777bc500d62bb029e) #10, !dbg !1436
  unreachable, !dbg !1436
}

; hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
; Function Attrs: inlinehint nounwind
define internal i32 @_ZN9hashbrown7control7bitmask7BitMask22nonzero_trailing_zeros17h6e7d56c1b164da0bE(i64 %nonzero) unnamed_addr #1 !dbg !1439 {
start:
  %nonzero.dbg.spill = alloca [8 x i8], align 8
  store i64 %nonzero, ptr %nonzero.dbg.spill, align 8
    #dbg_declare(ptr %nonzero.dbg.spill, !1444, !DIExpression(), !1447)
; call core::num::nonzero::NonZero<u64>::trailing_zeros
  %_4 = call i32 @"_ZN4core3num7nonzero18NonZero$LT$u64$GT$14trailing_zeros17h410e4a129d0958fbE"(i64 %nonzero) #9, !dbg !1448
  %_0 = udiv i32 %_4, 8, !dbg !1449
  ret i32 %_0, !dbg !1450
}

; core::fmt::Formatter::debug_lower_hex
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h72b54bf2b5971ea0E(ptr align 4) unnamed_addr #3

; core::fmt::Formatter::debug_upper_hex
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hda8089ad17629515E(ptr align 4) unnamed_addr #3

; core::fmt::num::imp::<impl core::fmt::Display for u8>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf2721191f040b59aE"(ptr align 1, ptr align 4) unnamed_addr #3

; core::fmt::num::<impl core::fmt::UpperHex for u8>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h3a28df7a448ec4e8E"(ptr align 1, ptr align 4) unnamed_addr #3

; core::fmt::num::<impl core::fmt::LowerHex for u8>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u8$GT$3fmt17hf19d67508dd58d4aE"(ptr align 1, ptr align 4) unnamed_addr #3

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32) #4

; core::num::nonzero::NonZero<T>::get
; Function Attrs: inlinehint nounwind
declare dso_local i64 @"_ZN4core3num7nonzero16NonZero$LT$T$GT$3get17he066c213f94948efE"(i64) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.cttz.i64(i64, i1 immarg) #4

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #5

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nounwind
declare dso_local void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4, ptr align 4) unnamed_addr #1

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #6

; core::ptr::const_ptr::<impl *const T>::is_aligned_to
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13is_aligned_to17h9d68787d1567a990E"(ptr, i32) unnamed_addr #1

; core::ptr::const_ptr::<impl *const T>::is_null
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf56eacc16313c5f5E"(ptr) unnamed_addr #1

; core::fmt::Formatter::debug_tuple
; Function Attrs: nounwind
declare dso_local void @_ZN4core3fmt9Formatter11debug_tuple17h6b1332a2f9e3757dE(ptr sret([12 x i8]) align 4, ptr align 4, ptr align 1, i32) unnamed_addr #3

; core::fmt::builders::DebugTuple::field
; Function Attrs: nounwind
declare dso_local align 4 ptr @_ZN4core3fmt8builders10DebugTuple5field17h8986a07ca92d9240E(ptr align 4, ptr align 1, ptr align 4) unnamed_addr #3

; core::fmt::builders::DebugTuple::finish
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt8builders10DebugTuple6finish17hc4ba190d2045a9c6E(ptr align 4) unnamed_addr #3

; core::fmt::Formatter::pad
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter3pad17h9a0270e93f7a94faE(ptr align 4, ptr align 1, i32) unnamed_addr #3

; <core::option::Option<T> as core::ops::try_trait::Try>::branch
; Function Attrs: inlinehint nounwind
declare dso_local { i32, i32 } @"_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h58df85d711a95350E"(i32, i32) unnamed_addr #1

; <core::option::Option<T> as core::ops::try_trait::FromResidual<core::option::Option<core::convert::Infallible>>>::from_residual
; Function Attrs: inlinehint nounwind
declare dso_local { i32, i32 } @"_ZN145_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..option..Option$LT$core..convert..Infallible$GT$$GT$$GT$13from_residual17h220f2d7a12a7e3c7E"() unnamed_addr #1

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn nounwind optsize
declare dso_local void @_ZN5alloc5alloc18handle_alloc_error17h1e5cf49dcf30c6c7E(i32, i32) unnamed_addr #7

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i32(ptr writeonly captures(none), i8, i32, i1 immarg) #8

; core::num::nonzero::NonZero<T>::new
; Function Attrs: inlinehint nounwind
declare dso_local i64 @"_ZN4core3num7nonzero16NonZero$LT$T$GT$3new17heb146e1fa1755ff2E"(i64) unnamed_addr #1

attributes #0 = { cold nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { inlinehint nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { inlinehint noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #3 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #4 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #5 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #7 = { cold minsize noreturn nounwind optsize "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #8 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #9 = { nounwind }
attributes #10 = { noreturn nounwind }

!llvm.ident = !{!14}
!llvm.dbg.cu = !{!15}
!llvm.module.flags = !{!63, !64}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<u8 as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<u8 as core::fmt::Debug>::{vtable_type}", file: !2, size: 128, align: 32, flags: DIFlagArtificial, elements: !4, vtableHolder: !12, templateParams: !13, identifier: "c612a2affee42820c5ff622205e5f9de")
!4 = !{!5, !8, !10, !11}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !3, file: !2, baseType: !6, size: 32, align: 32)
!6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !7, size: 32, align: 32, dwarfAddressSpace: 0)
!7 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !3, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!9 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !3, file: !2, baseType: !9, size: 32, align: 32, offset: 64)
!11 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !3, file: !2, baseType: !6, size: 32, align: 32, offset: 96)
!12 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!13 = !{}
!14 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!15 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !16, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !17, globals: !62, splitDebugInlining: false, nameTableKind: None)
!16 = !DIFile(filename: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5/src/lib.rs/@/hashbrown.c44395b665ae0048-cgu.0", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5")
!17 = !{!18, !24}
!18 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Fallibility", scope: !19, file: !2, baseType: !12, size: 8, align: 8, flags: DIFlagEnumClass, elements: !21)
!19 = !DINamespace(name: "raw", scope: !20)
!20 = !DINamespace(name: "hashbrown", scope: null)
!21 = !{!22, !23}
!22 = !DIEnumerator(name: "Fallible", value: 0, isUnsigned: true)
!23 = !DIEnumerator(name: "Infallible", value: 1, isUnsigned: true)
!24 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "AlignmentEnum", scope: !25, file: !2, baseType: !28, size: 32, align: 32, flags: DIFlagEnumClass, elements: !29)
!25 = !DINamespace(name: "alignment", scope: !26)
!26 = !DINamespace(name: "ptr", scope: !27)
!27 = !DINamespace(name: "core", scope: null)
!28 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!29 = !{!30, !31, !32, !33, !34, !35, !36, !37, !38, !39, !40, !41, !42, !43, !44, !45, !46, !47, !48, !49, !50, !51, !52, !53, !54, !55, !56, !57, !58, !59, !60, !61}
!30 = !DIEnumerator(name: "_Align1Shl0", value: 1, isUnsigned: true)
!31 = !DIEnumerator(name: "_Align1Shl1", value: 2, isUnsigned: true)
!32 = !DIEnumerator(name: "_Align1Shl2", value: 4, isUnsigned: true)
!33 = !DIEnumerator(name: "_Align1Shl3", value: 8, isUnsigned: true)
!34 = !DIEnumerator(name: "_Align1Shl4", value: 16, isUnsigned: true)
!35 = !DIEnumerator(name: "_Align1Shl5", value: 32, isUnsigned: true)
!36 = !DIEnumerator(name: "_Align1Shl6", value: 64, isUnsigned: true)
!37 = !DIEnumerator(name: "_Align1Shl7", value: 128, isUnsigned: true)
!38 = !DIEnumerator(name: "_Align1Shl8", value: 256, isUnsigned: true)
!39 = !DIEnumerator(name: "_Align1Shl9", value: 512, isUnsigned: true)
!40 = !DIEnumerator(name: "_Align1Shl10", value: 1024, isUnsigned: true)
!41 = !DIEnumerator(name: "_Align1Shl11", value: 2048, isUnsigned: true)
!42 = !DIEnumerator(name: "_Align1Shl12", value: 4096, isUnsigned: true)
!43 = !DIEnumerator(name: "_Align1Shl13", value: 8192, isUnsigned: true)
!44 = !DIEnumerator(name: "_Align1Shl14", value: 16384, isUnsigned: true)
!45 = !DIEnumerator(name: "_Align1Shl15", value: 32768, isUnsigned: true)
!46 = !DIEnumerator(name: "_Align1Shl16", value: 65536, isUnsigned: true)
!47 = !DIEnumerator(name: "_Align1Shl17", value: 131072, isUnsigned: true)
!48 = !DIEnumerator(name: "_Align1Shl18", value: 262144, isUnsigned: true)
!49 = !DIEnumerator(name: "_Align1Shl19", value: 524288, isUnsigned: true)
!50 = !DIEnumerator(name: "_Align1Shl20", value: 1048576, isUnsigned: true)
!51 = !DIEnumerator(name: "_Align1Shl21", value: 2097152, isUnsigned: true)
!52 = !DIEnumerator(name: "_Align1Shl22", value: 4194304, isUnsigned: true)
!53 = !DIEnumerator(name: "_Align1Shl23", value: 8388608, isUnsigned: true)
!54 = !DIEnumerator(name: "_Align1Shl24", value: 16777216, isUnsigned: true)
!55 = !DIEnumerator(name: "_Align1Shl25", value: 33554432, isUnsigned: true)
!56 = !DIEnumerator(name: "_Align1Shl26", value: 67108864, isUnsigned: true)
!57 = !DIEnumerator(name: "_Align1Shl27", value: 134217728, isUnsigned: true)
!58 = !DIEnumerator(name: "_Align1Shl28", value: 268435456, isUnsigned: true)
!59 = !DIEnumerator(name: "_Align1Shl29", value: 536870912, isUnsigned: true)
!60 = !DIEnumerator(name: "_Align1Shl30", value: 1073741824, isUnsigned: true)
!61 = !DIEnumerator(name: "_Align1Shl31", value: 2147483648, isUnsigned: true)
!62 = !{!0}
!63 = !{i32 7, !"Dwarf Version", i32 4}
!64 = !{i32 2, !"Debug Info Version", i32 3}
!65 = distinct !DISubprogram(name: "cold_path", linkageName: "_ZN4core10intrinsics9cold_path17hf94df2e82664e0a2E", scope: !67, file: !66, line: 417, type: !68, scopeLine: 417, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!66 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "5088527a679dbab229c7a43df7f388f7")
!67 = !DINamespace(name: "intrinsics", scope: !27)
!68 = !DISubroutineType(types: !69)
!69 = !{null}
!70 = !DILocation(line: 417, column: 28, scope: !65)
!71 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17he1d4dd70e2b1af78E", scope: !73, file: !72, line: 85, type: !76, scopeLine: 85, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !118)
!72 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/num.rs", directory: "", checksumkind: CSK_MD5, checksum: "14f1acdd980d957a36bf4243cc704071")
!73 = !DINamespace(name: "{impl#58}", scope: !74)
!74 = !DINamespace(name: "num", scope: !75)
!75 = !DINamespace(name: "fmt", scope: !27)
!76 = !DISubroutineType(types: !77)
!77 = !{!78, !96, !97}
!78 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !79, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !80, templateParams: !13, identifier: "613ace46ae0c395d39c31f05d3934750")
!79 = !DINamespace(name: "result", scope: !27)
!80 = !{!81}
!81 = !DICompositeType(tag: DW_TAG_variant_part, scope: !78, file: !2, size: 8, align: 8, elements: !82, templateParams: !13, identifier: "2bd67c77928327a5a86e1d970227dbc3", discriminator: !95)
!82 = !{!83, !91}
!83 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !81, file: !2, baseType: !84, size: 8, align: 8, extraData: i8 0)
!84 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !78, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !85, templateParams: !87, identifier: "8e1fa5ea2cd8f77479a16f216aa53a42")
!85 = !{!86}
!86 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !84, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!87 = !{!88, !89}
!88 = !DITemplateTypeParameter(name: "T", type: !7)
!89 = !DITemplateTypeParameter(name: "E", type: !90)
!90 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !75, file: !2, align: 8, flags: DIFlagPublic, elements: !13, identifier: "cac4d2a6635a122844ffbe3b52a15933")
!91 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !81, file: !2, baseType: !92, size: 8, align: 8, extraData: i8 1)
!92 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !78, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !93, templateParams: !87, identifier: "bd8eb8fbb58ca24e2467a7f35c864471")
!93 = !{!94}
!94 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !92, file: !2, baseType: !90, align: 8, offset: 8, flags: DIFlagPublic)
!95 = !DIDerivedType(tag: DW_TAG_member, scope: !78, file: !2, baseType: !12, size: 8, align: 8, flags: DIFlagArtificial)
!96 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&u8", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!97 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !98, size: 32, align: 32, dwarfAddressSpace: 0)
!98 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !75, file: !2, size: 128, align: 32, flags: DIFlagPublic, elements: !99, templateParams: !13, identifier: "9c19c8ef0b5ae3cad350e741e841742c")
!99 = !{!100, !107}
!100 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !98, file: !2, baseType: !101, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!101 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !75, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !102, templateParams: !13, identifier: "8e7d20540a73fe2190308d0618721e3e")
!102 = !{!103, !104, !106}
!103 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !101, file: !2, baseType: !28, size: 32, align: 32, flags: DIFlagPrivate)
!104 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !101, file: !2, baseType: !105, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!105 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!106 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !101, file: !2, baseType: !105, size: 16, align: 16, offset: 48, flags: DIFlagPrivate)
!107 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !98, file: !2, baseType: !108, size: 64, align: 32, flags: DIFlagPrivate)
!108 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 64, align: 32, elements: !109, templateParams: !13, identifier: "ed1fc41b72305de4afb5dbb44887680d")
!109 = !{!110, !113}
!110 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !108, file: !2, baseType: !111, size: 32, align: 32)
!111 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !112, size: 32, align: 32, dwarfAddressSpace: 0)
!112 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !13, identifier: "3bd7022d6bc7a1bba9386a42dfa7db9d")
!113 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !108, file: !2, baseType: !114, size: 32, align: 32, offset: 32)
!114 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !115, size: 32, align: 32, dwarfAddressSpace: 0)
!115 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 192, align: 32, elements: !116)
!116 = !{!117}
!117 = !DISubrange(count: 6, lowerBound: 0)
!118 = !{!119, !120}
!119 = !DILocalVariable(name: "self", arg: 1, scope: !71, file: !72, line: 85, type: !96)
!120 = !DILocalVariable(name: "f", arg: 2, scope: !71, file: !72, line: 85, type: !97)
!121 = !DILocation(line: 85, column: 24, scope: !71)
!122 = !DILocation(line: 85, column: 31, scope: !71)
!123 = !DILocation(line: 86, column: 26, scope: !71)
!124 = !DILocation(line: 86, column: 24, scope: !71)
!125 = !DILocation(line: 88, column: 33, scope: !71)
!126 = !DILocation(line: 88, column: 31, scope: !71)
!127 = !DILocation(line: 87, column: 25, scope: !71)
!128 = !DILocation(line: 91, column: 25, scope: !71)
!129 = !DILocation(line: 89, column: 25, scope: !71)
!130 = !DILocation(line: 93, column: 18, scope: !71)
!131 = distinct !DISubprogram(name: "unsigned_abs", linkageName: "_ZN4core3num21_$LT$impl$u20$i32$GT$12unsigned_abs17hddd60634a183b5cbE", scope: !133, file: !132, line: 2361, type: !135, scopeLine: 2361, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !138)
!132 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/int_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "f16ddcffff2d6d304c4da6339aadad04")
!133 = !DINamespace(name: "{impl#2}", scope: !134)
!134 = !DINamespace(name: "num", scope: !27)
!135 = !DISubroutineType(types: !136)
!136 = !{!28, !137}
!137 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!138 = !{!139}
!139 = !DILocalVariable(name: "self", arg: 1, scope: !131, file: !132, line: 2361, type: !137)
!140 = !DILocation(line: 2361, column: 35, scope: !131)
!141 = !DILocation(line: 2362, column: 19, scope: !131)
!142 = !DILocation(line: 2363, column: 10, scope: !131)
!143 = distinct !DISubprogram(name: "wrapping_abs", linkageName: "_ZN4core3num21_$LT$impl$u20$i32$GT$12wrapping_abs17ha537e37ea8fb8fadE", scope: !133, file: !132, line: 2337, type: !144, scopeLine: 2337, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !146)
!144 = !DISubroutineType(types: !145)
!145 = !{!137, !137}
!146 = !{!147}
!147 = !DILocalVariable(name: "self", arg: 1, scope: !143, file: !132, line: 2337, type: !137)
!148 = !DILocation(line: 2337, column: 35, scope: !143)
!149 = !DILocalVariable(name: "self", arg: 1, scope: !150, file: !132, line: 3642, type: !137)
!150 = distinct !DISubprogram(name: "is_negative", linkageName: "_ZN4core3num21_$LT$impl$u20$i32$GT$11is_negative17hab4ae24186e8eaccE", scope: !133, file: !132, line: 3642, type: !151, scopeLine: 3642, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !154)
!151 = !DISubroutineType(types: !152)
!152 = !{!153, !137}
!153 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!154 = !{!149}
!155 = !DILocation(line: 3642, column: 34, scope: !150, inlinedAt: !156)
!156 = distinct !DILocation(line: 2338, column: 22, scope: !143)
!157 = !DILocation(line: 3642, column: 50, scope: !150, inlinedAt: !156)
!158 = !DILocation(line: 2338, column: 17, scope: !143)
!159 = !DILocation(line: 2341, column: 18, scope: !143)
!160 = !DILocation(line: 2338, column: 14, scope: !143)
!161 = !DILocalVariable(name: "self", arg: 1, scope: !162, file: !132, line: 2258, type: !137)
!162 = distinct !DISubprogram(name: "wrapping_neg", linkageName: "_ZN4core3num21_$LT$impl$u20$i32$GT$12wrapping_neg17hb2cfd450b6a85de1E", scope: !133, file: !132, line: 2258, type: !144, scopeLine: 2258, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !163)
!163 = !{!161}
!164 = !DILocation(line: 2258, column: 35, scope: !162, inlinedAt: !165)
!165 = distinct !DILocation(line: 2339, column: 23, scope: !143)
!166 = !DILocalVariable(name: "self", arg: 1, scope: !167, file: !132, line: 2096, type: !137)
!167 = distinct !DISubprogram(name: "wrapping_sub", linkageName: "_ZN4core3num21_$LT$impl$u20$i32$GT$12wrapping_sub17he6f7bb4d748de2e9E", scope: !133, file: !132, line: 2096, type: !168, scopeLine: 2096, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !170)
!168 = !DISubroutineType(types: !169)
!169 = !{!137, !137, !137}
!170 = !{!166, !171}
!171 = !DILocalVariable(name: "rhs", arg: 2, scope: !167, file: !132, line: 2096, type: !137)
!172 = !DILocation(line: 2096, column: 35, scope: !167, inlinedAt: !173)
!173 = distinct !DILocation(line: 2259, column: 27, scope: !162, inlinedAt: !165)
!174 = !DILocation(line: 2096, column: 41, scope: !167, inlinedAt: !173)
!175 = !DILocation(line: 2097, column: 13, scope: !167, inlinedAt: !173)
!176 = !DILocation(line: 2339, column: 23, scope: !143)
!177 = !DILocation(line: 2343, column: 10, scope: !143)
!178 = distinct !DISubprogram(name: "from_ne_bytes", linkageName: "_ZN4core3num21_$LT$impl$u20$u64$GT$13from_ne_bytes17ha6d065afd5223f55E", scope: !180, file: !179, line: 3841, type: !181, scopeLine: 3841, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !187)
!179 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "5be88be11ad076a5d1229d10f045d3e0")
!180 = !DINamespace(name: "{impl#9}", scope: !134)
!181 = !DISubroutineType(types: !182)
!182 = !{!183, !184}
!183 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!184 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 64, align: 8, elements: !185)
!185 = !{!186}
!186 = !DISubrange(count: 8, lowerBound: 0)
!187 = !{!188}
!188 = !DILocalVariable(name: "bytes", arg: 1, scope: !178, file: !179, line: 3841, type: !184)
!189 = !DILocation(line: 3841, column: 36, scope: !178)
!190 = !DILocation(line: 3843, column: 22, scope: !178)
!191 = !DILocation(line: 3844, column: 10, scope: !178)
!192 = distinct !DISubprogram(name: "checked_mul", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h489bb426cb3f2bbbE", scope: !193, file: !179, line: 1033, type: !194, scopeLine: 1033, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !210)
!193 = !DINamespace(name: "{impl#11}", scope: !134)
!194 = !DISubroutineType(types: !195)
!195 = !{!196, !9, !9}
!196 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !197, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !198, templateParams: !13, identifier: "23b42ad4918f48bbb0d7df30a3e65f21")
!197 = !DINamespace(name: "option", scope: !27)
!198 = !{!199}
!199 = !DICompositeType(tag: DW_TAG_variant_part, scope: !196, file: !2, size: 64, align: 32, elements: !200, templateParams: !13, identifier: "fff0cc91bd07d4e2a6f41aa96659bb8", discriminator: !209)
!200 = !{!201, !205}
!201 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !199, file: !2, baseType: !202, size: 64, align: 32, extraData: i32 0)
!202 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !196, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !13, templateParams: !203, identifier: "16f3611ef7370fd7f09fc668dc1c16f8")
!203 = !{!204}
!204 = !DITemplateTypeParameter(name: "T", type: !9)
!205 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !199, file: !2, baseType: !206, size: 64, align: 32, extraData: i32 1)
!206 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !196, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !207, templateParams: !203, identifier: "9bb7e929a7e81f45f834925bbfee816")
!207 = !{!208}
!208 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !206, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!209 = !DIDerivedType(tag: DW_TAG_member, scope: !196, file: !2, baseType: !28, size: 32, align: 32, flags: DIFlagArtificial)
!210 = !{!211, !212, !213, !215}
!211 = !DILocalVariable(name: "self", arg: 1, scope: !192, file: !179, line: 1033, type: !9)
!212 = !DILocalVariable(name: "rhs", arg: 2, scope: !192, file: !179, line: 1033, type: !9)
!213 = !DILocalVariable(name: "a", scope: !214, file: !179, line: 1034, type: !9, align: 32)
!214 = distinct !DILexicalBlock(scope: !192, file: !179, line: 1034, column: 13)
!215 = !DILocalVariable(name: "b", scope: !214, file: !179, line: 1034, type: !153, align: 8)
!216 = !DILocation(line: 1033, column: 34, scope: !192)
!217 = !DILocation(line: 1033, column: 40, scope: !192)
!218 = !DILocalVariable(name: "self", arg: 1, scope: !219, file: !179, line: 2867, type: !9)
!219 = distinct !DISubprogram(name: "overflowing_mul", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_mul17hb3e76adbffddd04cE", scope: !193, file: !179, line: 2867, type: !220, scopeLine: 2867, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !226)
!220 = !DISubroutineType(types: !221)
!221 = !{!222, !9, !9}
!222 = !DICompositeType(tag: DW_TAG_structure_type, name: "(usize, bool)", file: !2, size: 64, align: 32, elements: !223, templateParams: !13, identifier: "d571287e27d8be874e95a2f698798cc6")
!223 = !{!224, !225}
!224 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !222, file: !2, baseType: !9, size: 32, align: 32)
!225 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !222, file: !2, baseType: !153, size: 8, align: 8, offset: 32)
!226 = !{!218, !227, !228, !230}
!227 = !DILocalVariable(name: "rhs", arg: 2, scope: !219, file: !179, line: 2867, type: !9)
!228 = !DILocalVariable(name: "a", scope: !229, file: !179, line: 2868, type: !28, align: 32)
!229 = distinct !DILexicalBlock(scope: !219, file: !179, line: 2868, column: 13)
!230 = !DILocalVariable(name: "b", scope: !229, file: !179, line: 2868, type: !153, align: 8)
!231 = !DILocation(line: 2867, column: 38, scope: !219, inlinedAt: !232)
!232 = distinct !DILocation(line: 1034, column: 31, scope: !192)
!233 = !DILocation(line: 2867, column: 44, scope: !219, inlinedAt: !232)
!234 = !DILocation(line: 2868, column: 26, scope: !219, inlinedAt: !232)
!235 = !DILocation(line: 2868, column: 18, scope: !219, inlinedAt: !232)
!236 = !DILocation(line: 2868, column: 18, scope: !229, inlinedAt: !232)
!237 = !DILocation(line: 2868, column: 21, scope: !219, inlinedAt: !232)
!238 = !DILocation(line: 2868, column: 21, scope: !229, inlinedAt: !232)
!239 = !DILocation(line: 1034, column: 31, scope: !192)
!240 = !DILocation(line: 1034, column: 18, scope: !192)
!241 = !DILocation(line: 1034, column: 18, scope: !214)
!242 = !DILocation(line: 1034, column: 21, scope: !192)
!243 = !DILocation(line: 1034, column: 21, scope: !214)
!244 = !DILocalVariable(name: "b", arg: 1, scope: !245, file: !66, line: 456, type: !153)
!245 = distinct !DISubprogram(name: "unlikely", linkageName: "_ZN4core10intrinsics8unlikely17hbdaab305ce3910c8E", scope: !67, file: !66, line: 456, type: !246, scopeLine: 456, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !248)
!246 = !DISubroutineType(types: !247)
!247 = !{!153, !153}
!248 = !{!244}
!249 = !DILocation(line: 456, column: 23, scope: !245, inlinedAt: !250)
!250 = distinct !DILocation(line: 1035, column: 16, scope: !214)
!251 = !DILocation(line: 457, column: 8, scope: !245, inlinedAt: !250)
!252 = !DILocation(line: 461, column: 9, scope: !245, inlinedAt: !250)
!253 = !DILocation(line: 457, column: 5, scope: !245, inlinedAt: !250)
!254 = !DILocation(line: 459, column: 9, scope: !245, inlinedAt: !250)
!255 = !DILocation(line: 463, column: 2, scope: !245, inlinedAt: !250)
!256 = !DILocation(line: 1035, column: 16, scope: !214)
!257 = !DILocation(line: 1035, column: 56, scope: !214)
!258 = !DILocation(line: 1035, column: 13, scope: !214)
!259 = !DILocation(line: 1035, column: 42, scope: !214)
!260 = !DILocation(line: 1036, column: 10, scope: !192)
!261 = distinct !DISubprogram(name: "abs_diff", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$8abs_diff17h20dbc36ae54c693eE", scope: !193, file: !179, line: 2831, type: !262, scopeLine: 2831, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !264)
!262 = !DISubroutineType(types: !263)
!263 = !{!9, !9, !9}
!264 = !{!265, !266}
!265 = !DILocalVariable(name: "self", arg: 1, scope: !261, file: !179, line: 2831, type: !9)
!266 = !DILocalVariable(name: "other", arg: 2, scope: !261, file: !179, line: 2831, type: !9)
!267 = !DILocation(line: 2831, column: 31, scope: !261)
!268 = !DILocation(line: 2831, column: 37, scope: !261)
!269 = !DILocation(line: 2832, column: 16, scope: !261)
!270 = !DILocation(line: 2096, column: 35, scope: !167, inlinedAt: !271)
!271 = distinct !DILocation(line: 2835, column: 31, scope: !261)
!272 = !DILocation(line: 2096, column: 41, scope: !167, inlinedAt: !271)
!273 = !DILocation(line: 2097, column: 13, scope: !167, inlinedAt: !271)
!274 = !DILocation(line: 2835, column: 58, scope: !261)
!275 = !DILocation(line: 2835, column: 17, scope: !261)
!276 = !DILocation(line: 2832, column: 13, scope: !261)
!277 = !DILocation(line: 2837, column: 20, scope: !261)
!278 = !DILocation(line: 2843, column: 10, scope: !261)
!279 = !DILocation(line: 2840, column: 21, scope: !261)
!280 = !DILocation(line: 2838, column: 21, scope: !261)
!281 = !DILocation(line: 2837, column: 17, scope: !261)
!282 = distinct !DISubprogram(name: "trailing_zeros", linkageName: "_ZN4core3num7nonzero18NonZero$LT$u64$GT$14trailing_zeros17h410e4a129d0958fbE", scope: !284, file: !283, line: 634, type: !294, scopeLine: 634, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !296, retainedNodes: !297)
!283 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/nonzero.rs", directory: "", checksumkind: CSK_MD5, checksum: "1885114babb84ebe4cb4d5c2eaff534f")
!284 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZero<u64>", scope: !285, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !286, templateParams: !292, identifier: "67f5f7abbc29b4b5bba17229fa1254d8")
!285 = !DINamespace(name: "nonzero", scope: !134)
!286 = !{!287}
!287 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !284, file: !2, baseType: !288, size: 64, align: 64, flags: DIFlagPrivate)
!288 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonZeroU64Inner", scope: !289, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !290, templateParams: !13, identifier: "a86a570a00b1142731a3adcef23dfbfb")
!289 = !DINamespace(name: "niche_types", scope: !134)
!290 = !{!291}
!291 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !288, file: !2, baseType: !183, size: 64, align: 64, flags: DIFlagPrivate)
!292 = !{!293}
!293 = !DITemplateTypeParameter(name: "T", type: !183)
!294 = !DISubroutineType(types: !295)
!295 = !{!28, !284}
!296 = !DISubprogram(name: "trailing_zeros", linkageName: "_ZN4core3num7nonzero18NonZero$LT$u64$GT$14trailing_zeros17h410e4a129d0958fbE", scope: !284, file: !283, line: 634, type: !294, scopeLine: 634, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!297 = !{!298}
!298 = !DILocalVariable(name: "self", arg: 1, scope: !282, file: !283, line: 634, type: !284)
!299 = !DILocation(line: 634, column: 41, scope: !282)
!300 = !DILocation(line: 637, column: 51, scope: !282)
!301 = !DILocation(line: 637, column: 21, scope: !282)
!302 = !DILocation(line: 639, column: 14, scope: !282)
!303 = distinct !DISubprogram(name: "read_unaligned<u64>", linkageName: "_ZN4core3ptr14read_unaligned17h6d1ad2df12929142E", scope: !26, file: !304, line: 1823, type: !305, scopeLine: 1823, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !292, retainedNodes: !336)
!304 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "8857c34524728cc5887872677b8e1917")
!305 = !DISubroutineType(types: !306)
!306 = !{!183, !307, !308}
!307 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u64", baseType: !183, size: 32, align: 32, dwarfAddressSpace: 0)
!308 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::location::Location", baseType: !309, size: 32, align: 32, dwarfAddressSpace: 0)
!309 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !310, file: !2, size: 128, align: 32, flags: DIFlagPublic, elements: !312, templateParams: !13, identifier: "7c34cafe8ea1dcad4032b9360816105f")
!310 = !DINamespace(name: "location", scope: !311)
!311 = !DINamespace(name: "panic", scope: !27)
!312 = !{!313, !325, !326, !327}
!313 = !DIDerivedType(tag: DW_TAG_member, name: "filename", scope: !309, file: !2, baseType: !314, size: 64, align: 32, flags: DIFlagPrivate)
!314 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<str>", scope: !315, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !316, templateParams: !323, identifier: "88212fc410c4399fd5095990cc8304ca")
!315 = !DINamespace(name: "non_null", scope: !26)
!316 = !{!317}
!317 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !314, file: !2, baseType: !318, size: 64, align: 32, flags: DIFlagPrivate)
!318 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const str", file: !2, size: 64, align: 32, elements: !319, templateParams: !13, identifier: "238a44609877474087c05adf26cd41fa")
!319 = !{!320, !322}
!320 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !318, file: !2, baseType: !321, size: 32, align: 32)
!321 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!322 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !318, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!323 = !{!324}
!324 = !DITemplateTypeParameter(name: "T", type: !12)
!325 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !309, file: !2, baseType: !28, size: 32, align: 32, offset: 64, flags: DIFlagPrivate)
!326 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !309, file: !2, baseType: !28, size: 32, align: 32, offset: 96, flags: DIFlagPrivate)
!327 = !DIDerivedType(tag: DW_TAG_member, name: "_filename", scope: !309, file: !2, baseType: !328, align: 8, offset: 128, flags: DIFlagPrivate)
!328 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&str>", scope: !329, file: !2, align: 8, flags: DIFlagPublic, elements: !13, templateParams: !330, identifier: "4cfc3eea77dd95eabd59051b67bd7e66")
!329 = !DINamespace(name: "marker", scope: !27)
!330 = !{!331}
!331 = !DITemplateTypeParameter(name: "T", type: !332)
!332 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 64, align: 32, elements: !333, templateParams: !13, identifier: "9277eecd40495f85161460476aacc992")
!333 = !{!334, !335}
!334 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !332, file: !2, baseType: !321, size: 32, align: 32)
!335 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !332, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!336 = !{!337, !338}
!337 = !DILocalVariable(name: "src", arg: 1, scope: !303, file: !304, line: 1823, type: !307)
!338 = !DILocalVariable(name: "tmp", scope: !339, file: !304, line: 1824, type: !340, align: 64)
!339 = distinct !DILexicalBlock(scope: !303, file: !304, line: 1824, column: 5)
!340 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<u64>", scope: !341, file: !2, size: 64, align: 64, elements: !343, templateParams: !292, identifier: "ad853c86c35c91bbf7836739e76f6a44")
!341 = !DINamespace(name: "maybe_uninit", scope: !342)
!342 = !DINamespace(name: "mem", scope: !27)
!343 = !{!344, !345}
!344 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !340, file: !2, baseType: !7, align: 8)
!345 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !340, file: !2, baseType: !346, size: 64, align: 64)
!346 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<u64>", scope: !347, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !348, templateParams: !292, identifier: "c5ae4d98ac6d63094ddf1e8ac6cd22d2")
!347 = !DINamespace(name: "manually_drop", scope: !342)
!348 = !{!349}
!349 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !346, file: !2, baseType: !183, size: 64, align: 64, flags: DIFlagPrivate)
!350 = !DILocation(line: 1823, column: 39, scope: !303)
!351 = !DILocation(line: 1824, column: 9, scope: !339)
!352 = !DILocation(line: 1824, column: 19, scope: !303)
!353 = !DILocalVariable(name: "self", arg: 1, scope: !354, file: !355, line: 560, type: !359)
!354 = distinct !DISubprogram(name: "as_mut_ptr<u64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h9ad1c30d921758b7E", scope: !340, file: !355, line: 560, type: !356, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !292, declaration: !360, retainedNodes: !361)
!355 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs", directory: "", checksumkind: CSK_MD5, checksum: "6de2d108794a3cb7d570256a1615f222")
!356 = !DISubroutineType(types: !357)
!357 = !{!358, !359}
!358 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u64", baseType: !183, size: 32, align: 32, dwarfAddressSpace: 0)
!359 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<u64>", baseType: !340, size: 32, align: 32, dwarfAddressSpace: 0)
!360 = !DISubprogram(name: "as_mut_ptr<u64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h9ad1c30d921758b7E", scope: !340, file: !355, line: 560, type: !356, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !292)
!361 = !{!353}
!362 = !DILocation(line: 560, column: 29, scope: !354, inlinedAt: !363)
!363 = distinct !DILocation(line: 1832, column: 51, scope: !339)
!364 = !DILocalVariable(name: "src", arg: 1, scope: !365, file: !304, line: 526, type: !368)
!365 = distinct !DISubprogram(name: "copy_nonoverlapping<u8>", linkageName: "_ZN4core3ptr19copy_nonoverlapping17h7addadc19defe5aeE", scope: !26, file: !304, line: 526, type: !366, scopeLine: 526, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !323, retainedNodes: !370)
!366 = !DISubroutineType(types: !367)
!367 = !{null, !368, !369, !9}
!368 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!369 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!370 = !{!364, !371, !372}
!371 = !DILocalVariable(name: "dst", arg: 2, scope: !365, file: !304, line: 526, type: !369)
!372 = !DILocalVariable(name: "count", arg: 3, scope: !365, file: !304, line: 526, type: !9)
!373 = !DILocation(line: 526, column: 44, scope: !365, inlinedAt: !374)
!374 = distinct !DILocation(line: 1832, column: 9, scope: !339)
!375 = !DILocation(line: 526, column: 59, scope: !365, inlinedAt: !374)
!376 = !DILocation(line: 526, column: 72, scope: !365, inlinedAt: !374)
!377 = !DILocation(line: 77, column: 35, scope: !378, inlinedAt: !374)
!378 = !DILexicalBlockFile(scope: !365, file: !379, discriminator: 0)
!379 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ub_checks.rs", directory: "", checksumkind: CSK_MD5, checksum: "41b3943b2b7dc8c218ee37ead81b317d")
!380 = !DILocation(line: 78, column: 17, scope: !378, inlinedAt: !374)
!381 = !DILocation(line: 547, column: 14, scope: !365, inlinedAt: !374)
!382 = !DILocation(line: 1833, column: 9, scope: !339)
!383 = !DILocalVariable(name: "self", arg: 1, scope: !384, file: !355, line: 615, type: !340)
!384 = distinct !DISubprogram(name: "assume_init<u64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17hec418d15c4a50ea0E", scope: !340, file: !355, line: 615, type: !385, scopeLine: 615, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !292, declaration: !387, retainedNodes: !388)
!385 = !DISubroutineType(types: !386)
!386 = !{!183, !340, !308}
!387 = !DISubprogram(name: "assume_init<u64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17hec418d15c4a50ea0E", scope: !340, file: !355, line: 615, type: !385, scopeLine: 615, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !292)
!388 = !{!383}
!389 = !DILocation(line: 615, column: 37, scope: !384, inlinedAt: !390)
!390 = distinct !DILocation(line: 1833, column: 13, scope: !339)
!391 = !DILocalVariable(name: "self", arg: 1, scope: !392, file: !393, line: 48, type: !398)
!392 = distinct !DISubprogram(name: "cast<core::mem::manually_drop::ManuallyDrop<u64>, u64>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4cast17hacc900992aa62d30E", scope: !394, file: !393, line: 48, type: !396, scopeLine: 48, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !400, retainedNodes: !399)
!393 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "473e695c4e056b47688e2be1785e83b5")
!394 = !DINamespace(name: "{impl#0}", scope: !395)
!395 = !DINamespace(name: "const_ptr", scope: !26)
!396 = !DISubroutineType(types: !397)
!397 = !{!307, !398}
!398 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::mem::manually_drop::ManuallyDrop<u64>", baseType: !346, size: 32, align: 32, dwarfAddressSpace: 0)
!399 = !{!391}
!400 = !{!401, !402}
!401 = !DITemplateTypeParameter(name: "T", type: !346)
!402 = !DITemplateTypeParameter(name: "U", type: !183)
!403 = !DILocation(line: 48, column: 26, scope: !392, inlinedAt: !404)
!404 = distinct !DILocation(line: 622, column: 37, scope: !384, inlinedAt: !390)
!405 = !DILocation(line: 622, column: 49, scope: !384, inlinedAt: !390)
!406 = !DILocation(line: 1835, column: 2, scope: !303)
!407 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core3ptr19copy_nonoverlapping18precondition_check17h5c74c2f3f7735849E", scope: !408, file: !379, line: 68, type: !409, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !412)
!408 = !DINamespace(name: "copy_nonoverlapping", scope: !26)
!409 = !DISubroutineType(types: !410)
!410 = !{null, !6, !411, !9, !9, !9, !308}
!411 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut ()", baseType: !7, size: 32, align: 32, dwarfAddressSpace: 0)
!412 = !{!413, !414, !415, !416, !417, !418, !420}
!413 = !DILocalVariable(name: "src", arg: 1, scope: !407, file: !379, line: 68, type: !6)
!414 = !DILocalVariable(name: "dst", arg: 2, scope: !407, file: !379, line: 68, type: !411)
!415 = !DILocalVariable(name: "size", arg: 3, scope: !407, file: !379, line: 68, type: !9)
!416 = !DILocalVariable(name: "align", arg: 4, scope: !407, file: !379, line: 68, type: !9)
!417 = !DILocalVariable(name: "count", arg: 5, scope: !407, file: !379, line: 68, type: !9)
!418 = !DILocalVariable(name: "zero_size", scope: !419, file: !304, line: 538, type: !153, align: 8)
!419 = distinct !DILexicalBlock(scope: !407, file: !304, line: 538, column: 13)
!420 = !DILocalVariable(name: "msg", scope: !421, file: !379, line: 70, type: !332, align: 32)
!421 = distinct !DILexicalBlock(scope: !407, file: !379, line: 70, column: 21)
!422 = !DILocation(line: 68, column: 43, scope: !407)
!423 = !DILocation(line: 538, column: 17, scope: !419)
!424 = !DILocation(line: 70, column: 25, scope: !421)
!425 = !DILocation(line: 538, column: 29, scope: !426)
!426 = !DILexicalBlockFile(scope: !407, file: !304, discriminator: 0)
!427 = !DILocation(line: 538, column: 43, scope: !426)
!428 = !DILocation(line: 539, column: 66, scope: !419)
!429 = !DILocation(line: 539, column: 13, scope: !419)
!430 = !DILocation(line: 540, column: 73, scope: !419)
!431 = !DILocation(line: 540, column: 20, scope: !419)
!432 = !DILocation(line: 541, column: 20, scope: !419)
!433 = !DILocation(line: 537, column: 14, scope: !426)
!434 = !DILocation(line: 73, column: 94, scope: !421)
!435 = !DILocation(line: 73, column: 59, scope: !421)
!436 = !DILocation(line: 73, column: 21, scope: !421)
!437 = !DILocation(line: 75, column: 14, scope: !407)
!438 = distinct !DISubprogram(name: "slice_from_raw_parts_mut<hashbrown::control::tag::Tag>", linkageName: "_ZN4core3ptr24slice_from_raw_parts_mut17hc59c18e342d99bc7E", scope: !26, file: !304, line: 1218, type: !439, scopeLine: 1218, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !455, retainedNodes: !452)
!439 = !DISubroutineType(types: !440)
!440 = !{!441, !451, !9}
!441 = !DICompositeType(tag: DW_TAG_structure_type, name: "*mut [hashbrown::control::tag::Tag]", file: !2, size: 64, align: 32, elements: !442, templateParams: !13, identifier: "19a9933a2a15a6affb82b3fd78dbcfc")
!442 = !{!443, !450}
!443 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !441, file: !2, baseType: !444, size: 32, align: 32)
!444 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !445, size: 32, align: 32, dwarfAddressSpace: 0)
!445 = !DICompositeType(tag: DW_TAG_structure_type, name: "Tag", scope: !446, file: !2, size: 8, align: 8, flags: DIFlagProtected, elements: !448, templateParams: !13, identifier: "6803943742004346136402c414d490a1")
!446 = !DINamespace(name: "tag", scope: !447)
!447 = !DINamespace(name: "control", scope: !20)
!448 = !{!449}
!449 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !445, file: !2, baseType: !12, size: 8, align: 8, flags: DIFlagProtected)
!450 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !441, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!451 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut hashbrown::control::tag::Tag", baseType: !445, size: 32, align: 32, dwarfAddressSpace: 0)
!452 = !{!453, !454}
!453 = !DILocalVariable(name: "data", arg: 1, scope: !438, file: !304, line: 1218, type: !451)
!454 = !DILocalVariable(name: "len", arg: 2, scope: !438, file: !304, line: 1218, type: !9)
!455 = !{!456}
!456 = !DITemplateTypeParameter(name: "T", type: !445)
!457 = !DILocation(line: 1218, column: 42, scope: !438)
!458 = !DILocation(line: 1218, column: 56, scope: !438)
!459 = !DILocation(line: 1219, column: 5, scope: !438)
!460 = !DILocation(line: 1220, column: 2, scope: !438)
!461 = distinct !DISubprogram(name: "read<u64>", linkageName: "_ZN4core3ptr4read17hb71da8a038dc7781E", scope: !26, file: !304, line: 1705, type: !305, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !292, retainedNodes: !462)
!462 = !{!463}
!463 = !DILocalVariable(name: "src", arg: 1, scope: !461, file: !304, line: 1705, type: !307)
!464 = !DILocation(line: 1705, column: 29, scope: !461)
!465 = !DILocation(line: 77, column: 35, scope: !466)
!466 = !DILexicalBlockFile(scope: !461, file: !379, discriminator: 0)
!467 = !DILocation(line: 1744, column: 9, scope: !461)
!468 = !DILocation(line: 1746, column: 2, scope: !461)
!469 = !DILocation(line: 78, column: 17, scope: !466)
!470 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core3ptr4read18precondition_check17hb802a2aeb82aab1bE", scope: !471, file: !379, line: 68, type: !472, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !474)
!471 = !DINamespace(name: "read", scope: !26)
!472 = !DISubroutineType(types: !473)
!473 = !{null, !6, !9, !153, !308}
!474 = !{!475, !476, !477, !478}
!475 = !DILocalVariable(name: "addr", arg: 1, scope: !470, file: !379, line: 68, type: !6)
!476 = !DILocalVariable(name: "align", arg: 2, scope: !470, file: !379, line: 68, type: !9)
!477 = !DILocalVariable(name: "is_zst", arg: 3, scope: !470, file: !379, line: 68, type: !153)
!478 = !DILocalVariable(name: "msg", scope: !479, file: !379, line: 70, type: !332, align: 32)
!479 = distinct !DILexicalBlock(scope: !470, file: !379, line: 70, column: 21)
!480 = !DILocation(line: 68, column: 43, scope: !470)
!481 = !DILocation(line: 70, column: 25, scope: !479)
!482 = !DILocation(line: 1742, column: 18, scope: !483)
!483 = !DILexicalBlockFile(scope: !470, file: !304, discriminator: 0)
!484 = !DILocation(line: 73, column: 94, scope: !479)
!485 = !DILocation(line: 73, column: 59, scope: !479)
!486 = !DILocation(line: 73, column: 21, scope: !479)
!487 = !DILocation(line: 75, column: 14, scope: !470)
!488 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18precondition_check17h467862a1054379e6E", scope: !489, file: !379, line: 68, type: !492, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !494)
!489 = !DINamespace(name: "add", scope: !490)
!490 = !DINamespace(name: "{impl#0}", scope: !491)
!491 = !DINamespace(name: "mut_ptr", scope: !26)
!492 = !DISubroutineType(types: !493)
!493 = !{null, !6, !9, !9, !308}
!494 = !{!495, !496, !497, !498}
!495 = !DILocalVariable(name: "this", arg: 1, scope: !488, file: !379, line: 68, type: !6)
!496 = !DILocalVariable(name: "count", arg: 2, scope: !488, file: !379, line: 68, type: !9)
!497 = !DILocalVariable(name: "size", arg: 3, scope: !488, file: !379, line: 68, type: !9)
!498 = !DILocalVariable(name: "msg", scope: !499, file: !379, line: 70, type: !332, align: 32)
!499 = distinct !DILexicalBlock(scope: !488, file: !379, line: 70, column: 21)
!500 = !DILocation(line: 68, column: 43, scope: !488)
!501 = !DILocation(line: 70, column: 25, scope: !499)
!502 = !DILocation(line: 957, column: 18, scope: !503)
!503 = !DILexicalBlockFile(scope: !488, file: !504, discriminator: 0)
!504 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "b0bbe11126e084b85a45fba4c5663912")
!505 = !DILocation(line: 73, column: 94, scope: !499)
!506 = !DILocation(line: 73, column: 59, scope: !499)
!507 = !DILocation(line: 73, column: 21, scope: !499)
!508 = !DILocation(line: 75, column: 14, scope: !488)
!509 = distinct !DISubprogram(name: "runtime_add_nowrap", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18runtime_add_nowrap17h4d84a2e818ec59acE", scope: !489, file: !504, line: 934, type: !510, scopeLine: 934, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !512)
!510 = !DISubroutineType(types: !511)
!511 = !{!153, !6, !9, !9}
!512 = !{!513, !514, !515}
!513 = !DILocalVariable(name: "this", arg: 1, scope: !509, file: !504, line: 934, type: !6)
!514 = !DILocalVariable(name: "count", arg: 2, scope: !509, file: !504, line: 934, type: !9)
!515 = !DILocalVariable(name: "size", arg: 3, scope: !509, file: !504, line: 934, type: !9)
!516 = !DILocation(line: 934, column: 37, scope: !509)
!517 = !DILocation(line: 934, column: 54, scope: !509)
!518 = !DILocation(line: 934, column: 68, scope: !509)
!519 = !DILocation(line: 2435, column: 27, scope: !520)
!520 = !DILexicalBlockFile(scope: !509, file: !66, discriminator: 0)
!521 = !DILocation(line: 2435, column: 9, scope: !520)
!522 = !DILocation(line: 947, column: 10, scope: !509)
!523 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add18runtime_add_nowrap7runtime17hea30b135eef39c51E", scope: !524, file: !66, line: 2423, type: !510, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !525)
!524 = !DINamespace(name: "runtime_add_nowrap", scope: !489)
!525 = !{!526, !527, !528, !529, !531}
!526 = !DILocalVariable(name: "this", arg: 1, scope: !523, file: !66, line: 2423, type: !6)
!527 = !DILocalVariable(name: "count", arg: 2, scope: !523, file: !66, line: 2423, type: !9)
!528 = !DILocalVariable(name: "size", arg: 3, scope: !523, file: !66, line: 2423, type: !9)
!529 = !DILocalVariable(name: "byte_offset", scope: !530, file: !504, line: 940, type: !9, align: 32)
!530 = distinct !DILexicalBlock(scope: !523, file: !504, line: 940, column: 21)
!531 = !DILocalVariable(name: "overflow", scope: !532, file: !504, line: 943, type: !153, align: 8)
!532 = distinct !DILexicalBlock(scope: !530, file: !504, line: 943, column: 21)
!533 = !DILocation(line: 2423, column: 40, scope: !523)
!534 = !DILocation(line: 940, column: 51, scope: !535)
!535 = !DILexicalBlockFile(scope: !523, file: !504, discriminator: 0)
!536 = !DILocation(line: 940, column: 45, scope: !535)
!537 = !DILocation(line: 940, column: 25, scope: !535)
!538 = !DILocation(line: 940, column: 30, scope: !535)
!539 = !DILocation(line: 940, column: 30, scope: !530)
!540 = !DILocalVariable(name: "self", arg: 1, scope: !541, file: !393, line: 153, type: !6)
!541 = distinct !DISubprogram(name: "addr<()>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4addr17hc35208c358293368E", scope: !394, file: !393, line: 153, type: !542, scopeLine: 153, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !545, retainedNodes: !544)
!542 = !DISubroutineType(types: !543)
!543 = !{!9, !6}
!544 = !{!540}
!545 = !{!88}
!546 = !DILocation(line: 153, column: 17, scope: !541, inlinedAt: !547)
!547 = distinct !DILocation(line: 943, column: 46, scope: !530)
!548 = !DILocalVariable(name: "self", arg: 1, scope: !549, file: !393, line: 48, type: !6)
!549 = distinct !DISubprogram(name: "cast<(), ()>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4cast17h78d613b6d9843c2eE", scope: !394, file: !393, line: 48, type: !550, scopeLine: 48, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !553, retainedNodes: !552)
!550 = !DISubroutineType(types: !551)
!551 = !{!6, !6}
!552 = !{!548}
!553 = !{!88, !554}
!554 = !DITemplateTypeParameter(name: "U", type: !7)
!555 = !DILocation(line: 48, column: 26, scope: !549, inlinedAt: !556)
!556 = distinct !DILocation(line: 159, column: 38, scope: !541, inlinedAt: !547)
!557 = !DILocation(line: 159, column: 18, scope: !541, inlinedAt: !547)
!558 = !DILocalVariable(name: "self", arg: 1, scope: !559, file: !179, line: 2645, type: !9)
!559 = distinct !DISubprogram(name: "overflowing_add", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_add17h5ed3665df2a5b632E", scope: !193, file: !179, line: 2645, type: !220, scopeLine: 2645, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !560)
!560 = !{!558, !561, !562, !564}
!561 = !DILocalVariable(name: "rhs", arg: 2, scope: !559, file: !179, line: 2645, type: !9)
!562 = !DILocalVariable(name: "a", scope: !563, file: !179, line: 2646, type: !28, align: 32)
!563 = distinct !DILexicalBlock(scope: !559, file: !179, line: 2646, column: 13)
!564 = !DILocalVariable(name: "b", scope: !563, file: !179, line: 2646, type: !153, align: 8)
!565 = !DILocation(line: 2645, column: 38, scope: !559, inlinedAt: !566)
!566 = distinct !DILocation(line: 943, column: 53, scope: !530)
!567 = !DILocation(line: 2645, column: 44, scope: !559, inlinedAt: !566)
!568 = !DILocation(line: 2646, column: 26, scope: !559, inlinedAt: !566)
!569 = !DILocation(line: 2646, column: 18, scope: !559, inlinedAt: !566)
!570 = !DILocation(line: 2646, column: 18, scope: !563, inlinedAt: !566)
!571 = !DILocation(line: 2646, column: 21, scope: !559, inlinedAt: !566)
!572 = !DILocation(line: 2646, column: 21, scope: !563, inlinedAt: !566)
!573 = !DILocation(line: 2648, column: 10, scope: !559, inlinedAt: !566)
!574 = !DILocation(line: 943, column: 53, scope: !530)
!575 = !DILocation(line: 943, column: 29, scope: !530)
!576 = !DILocation(line: 943, column: 29, scope: !532)
!577 = !DILocation(line: 944, column: 21, scope: !532)
!578 = !DILocation(line: 941, column: 32, scope: !535)
!579 = !DILocation(line: 941, column: 25, scope: !535)
!580 = !DILocation(line: 944, column: 61, scope: !532)
!581 = !DILocation(line: 2425, column: 10, scope: !523)
!582 = !DILocation(line: 2423, column: 9, scope: !523)
!583 = distinct !DISubprogram(name: "from_raw_parts_mut<[hashbrown::control::tag::Tag], hashbrown::control::tag::Tag>", linkageName: "_ZN4core3ptr8metadata18from_raw_parts_mut17h9e627e0534e3af57E", scope: !585, file: !584, line: 128, type: !439, scopeLine: 128, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !589, retainedNodes: !586)
!584 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/metadata.rs", directory: "", checksumkind: CSK_MD5, checksum: "88d1c59ea4b69b6dc0e553c0ee1c4c73")
!585 = !DINamespace(name: "metadata", scope: !26)
!586 = !{!587, !588}
!587 = !DILocalVariable(name: "data_pointer", arg: 1, scope: !583, file: !584, line: 129, type: !451)
!588 = !DILocalVariable(name: "metadata", arg: 2, scope: !583, file: !584, line: 130, type: !9)
!589 = !{!456, !590}
!590 = !DITemplateTypeParameter(name: "impl Thin", type: !445)
!591 = !DILocation(line: 129, column: 5, scope: !583)
!592 = !DILocation(line: 130, column: 5, scope: !583)
!593 = !DILocation(line: 133, column: 2, scope: !583)
!594 = distinct !DISubprogram(name: "read<u64>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4read17h33a6069a5019fd58E", scope: !394, file: !393, line: 1166, type: !305, scopeLine: 1166, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !292, retainedNodes: !595)
!595 = !{!596}
!596 = !DILocalVariable(name: "self", arg: 1, scope: !594, file: !393, line: 1166, type: !307)
!597 = !DILocation(line: 1166, column: 30, scope: !594)
!598 = !DILocation(line: 1171, column: 18, scope: !594)
!599 = !DILocation(line: 1172, column: 6, scope: !594)
!600 = distinct !DISubprogram(name: "from_raw_parts_mut<hashbrown::control::tag::Tag>", linkageName: "_ZN4core5slice3raw18from_raw_parts_mut17heeb025ab382939bbE", scope: !602, file: !601, line: 179, type: !604, scopeLine: 179, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !455, retainedNodes: !610)
!601 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/slice/raw.rs", directory: "", checksumkind: CSK_MD5, checksum: "1c257c0bb74a1862c3fb776eeea63ad9")
!602 = !DINamespace(name: "raw", scope: !603)
!603 = !DINamespace(name: "slice", scope: !27)
!604 = !DISubroutineType(types: !605)
!605 = !{!606, !451, !9, !308}
!606 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut [hashbrown::control::tag::Tag]", file: !2, size: 64, align: 32, elements: !607, templateParams: !13, identifier: "c9d9d212fe91bf7f789d15d56bb867f3")
!607 = !{!608, !609}
!608 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !606, file: !2, baseType: !444, size: 32, align: 32)
!609 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !606, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!610 = !{!611, !612}
!611 = !DILocalVariable(name: "data", arg: 1, scope: !600, file: !601, line: 179, type: !451)
!612 = !DILocalVariable(name: "len", arg: 2, scope: !600, file: !601, line: 179, type: !9)
!613 = !DILocation(line: 179, column: 47, scope: !600)
!614 = !DILocation(line: 179, column: 61, scope: !600)
!615 = !DILocation(line: 77, column: 35, scope: !616)
!616 = !DILexicalBlockFile(scope: !600, file: !379, discriminator: 0)
!617 = !DILocation(line: 194, column: 15, scope: !600)
!618 = !DILocation(line: 196, column: 2, scope: !600)
!619 = !DILocation(line: 78, column: 17, scope: !616)
!620 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core5slice3raw18from_raw_parts_mut18precondition_check17h33352bc35700d71fE", scope: !621, file: !379, line: 68, type: !622, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !624)
!621 = !DINamespace(name: "from_raw_parts_mut", scope: !602)
!622 = !DISubroutineType(types: !623)
!623 = !{null, !411, !9, !9, !9, !308}
!624 = !{!625, !626, !627, !628, !629}
!625 = !DILocalVariable(name: "data", arg: 1, scope: !620, file: !379, line: 68, type: !411)
!626 = !DILocalVariable(name: "size", arg: 2, scope: !620, file: !379, line: 68, type: !9)
!627 = !DILocalVariable(name: "align", arg: 3, scope: !620, file: !379, line: 68, type: !9)
!628 = !DILocalVariable(name: "len", arg: 4, scope: !620, file: !379, line: 68, type: !9)
!629 = !DILocalVariable(name: "msg", scope: !630, file: !379, line: 70, type: !332, align: 32)
!630 = distinct !DILexicalBlock(scope: !620, file: !379, line: 70, column: 21)
!631 = !DILocation(line: 68, column: 43, scope: !620)
!632 = !DILocation(line: 70, column: 25, scope: !630)
!633 = !DILocation(line: 191, column: 13, scope: !634)
!634 = !DILexicalBlockFile(scope: !620, file: !601, discriminator: 0)
!635 = !DILocation(line: 73, column: 94, scope: !630)
!636 = !DILocation(line: 73, column: 59, scope: !630)
!637 = !DILocation(line: 73, column: 21, scope: !630)
!638 = !DILocation(line: 192, column: 20, scope: !634)
!639 = !DILocation(line: 75, column: 14, scope: !620)
!640 = distinct !DISubprogram(name: "panic_const_div_by_zero", linkageName: "_ZN4core9panicking11panic_const23panic_const_div_by_zero17hf49f16ed30d86844E", scope: !642, file: !641, line: 173, type: !644, scopeLine: 173, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!641 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/panicking.rs", directory: "", checksumkind: CSK_MD5, checksum: "b120da646d1a09f31201b8a519374e57")
!642 = !DINamespace(name: "panic_const", scope: !643)
!643 = !DINamespace(name: "panicking", scope: !27)
!644 = !DISubroutineType(types: !645)
!645 = !{null, !308}
!646 = !DILocation(line: 180, column: 27, scope: !640)
!647 = !DILocation(line: 180, column: 17, scope: !640)
!648 = distinct !DISubprogram(name: "panic_const_add_overflow", linkageName: "_ZN4core9panicking11panic_const24panic_const_add_overflow17h37f2b215d7298529E", scope: !642, file: !641, line: 173, type: !644, scopeLine: 173, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!649 = !DILocation(line: 180, column: 27, scope: !648)
!650 = !DILocation(line: 180, column: 17, scope: !648)
!651 = distinct !DISubprogram(name: "panic_const_mul_overflow", linkageName: "_ZN4core9panicking11panic_const24panic_const_mul_overflow17hc3f2f5648b4a5121E", scope: !642, file: !641, line: 173, type: !644, scopeLine: 173, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!652 = !DILocation(line: 180, column: 27, scope: !651)
!653 = !DILocation(line: 180, column: 17, scope: !651)
!654 = distinct !DISubprogram(name: "panic_const_shl_overflow", linkageName: "_ZN4core9panicking11panic_const24panic_const_shl_overflow17h129e566a8f606375E", scope: !642, file: !641, line: 173, type: !644, scopeLine: 173, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!655 = !DILocation(line: 180, column: 27, scope: !654)
!656 = !DILocation(line: 180, column: 17, scope: !654)
!657 = distinct !DISubprogram(name: "panic_const_shr_overflow", linkageName: "_ZN4core9panicking11panic_const24panic_const_shr_overflow17ha5f77e543b98d47cE", scope: !642, file: !641, line: 173, type: !644, scopeLine: 173, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!658 = !DILocation(line: 180, column: 27, scope: !657)
!659 = !DILocation(line: 180, column: 17, scope: !657)
!660 = distinct !DISubprogram(name: "panic_const_sub_overflow", linkageName: "_ZN4core9panicking11panic_const24panic_const_sub_overflow17h7c3bd49388308f57E", scope: !642, file: !641, line: 173, type: !644, scopeLine: 173, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!661 = !DILocation(line: 180, column: 27, scope: !660)
!662 = !DILocation(line: 180, column: 17, scope: !660)
!663 = distinct !DISubprogram(name: "panic_nounwind", linkageName: "_ZN4core9panicking14panic_nounwind17h401ade9a4c393332E", scope: !643, file: !641, line: 229, type: !664, scopeLine: 229, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !666)
!664 = !DISubroutineType(types: !665)
!665 = !{null, !332}
!666 = !{!667}
!667 = !DILocalVariable(name: "expr", arg: 1, scope: !663, file: !641, line: 229, type: !332)
!668 = !DILocation(line: 229, column: 29, scope: !663)
!669 = !DILocation(line: 230, column: 51, scope: !663)
!670 = !DILocation(line: 230, column: 24, scope: !663)
!671 = !DILocation(line: 230, column: 5, scope: !663)
!672 = distinct !DISubprogram(name: "panic_nounwind_fmt", linkageName: "_ZN4core9panicking18panic_nounwind_fmt17h256658f36c86c48dE", scope: !643, file: !641, line: 95, type: !673, scopeLine: 95, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !758)
!673 = !DISubroutineType(types: !674)
!674 = !{null, !675, !153, !308}
!675 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !75, file: !2, size: 192, align: 32, flags: DIFlagPublic, elements: !676, templateParams: !13, identifier: "d691e62b2ee4847c2af32873f04bd10")
!676 = !{!677, !683, !724}
!677 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !675, file: !2, baseType: !678, size: 64, align: 32, flags: DIFlagPrivate)
!678 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 64, align: 32, elements: !679, templateParams: !13, identifier: "4e66b00a376d6af5b8765440fb2839f")
!679 = !{!680, !682}
!680 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !678, file: !2, baseType: !681, size: 32, align: 32)
!681 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !332, size: 32, align: 32, dwarfAddressSpace: 0)
!682 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !678, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!683 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !675, file: !2, baseType: !684, size: 64, align: 32, offset: 128, flags: DIFlagPrivate)
!684 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !197, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !685, templateParams: !13, identifier: "a638667a460b22fe10961f9a2f3202aa")
!685 = !{!686}
!686 = !DICompositeType(tag: DW_TAG_variant_part, scope: !684, file: !2, size: 64, align: 32, elements: !687, templateParams: !13, identifier: "29af53ccc7f21f4d5671e352d673889a", discriminator: !723)
!687 = !{!688, !719}
!688 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !686, file: !2, baseType: !689, size: 64, align: 32, extraData: i32 0)
!689 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !684, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !13, templateParams: !690, identifier: "11ce4f4d10f67887bbe6bf59a521c479")
!690 = !{!691}
!691 = !DITemplateTypeParameter(name: "T", type: !692)
!692 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !2, size: 64, align: 32, elements: !693, templateParams: !13, identifier: "b0485535d7020130e949c24f3fc2aa00")
!693 = !{!694, !718}
!694 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !692, file: !2, baseType: !695, size: 32, align: 32)
!695 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !696, size: 32, align: 32, dwarfAddressSpace: 0)
!696 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !697, file: !2, size: 192, align: 32, flags: DIFlagPublic, elements: !698, templateParams: !13, identifier: "8cb06f9d78dc629c8f52fc3b5544996c")
!697 = !DINamespace(name: "rt", scope: !75)
!698 = !{!699, !700, !701, !717}
!699 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !696, file: !2, baseType: !9, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
!700 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !696, file: !2, baseType: !28, size: 32, align: 32, offset: 160, flags: DIFlagPublic)
!701 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !696, file: !2, baseType: !702, size: 64, align: 32, flags: DIFlagPublic)
!702 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !697, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !703, templateParams: !13, identifier: "2d7772037f5c744e87d41105441784d5")
!703 = !{!704}
!704 = !DICompositeType(tag: DW_TAG_variant_part, scope: !702, file: !2, size: 64, align: 32, elements: !705, templateParams: !13, identifier: "af14687975a61e1ae6bbcdaeb79a8a2", discriminator: !716)
!705 = !{!706, !710, !714}
!706 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !704, file: !2, baseType: !707, size: 64, align: 32, extraData: i16 0)
!707 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !702, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !708, templateParams: !13, identifier: "da16c9b5356522ffb015c0e99237342e")
!708 = !{!709}
!709 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !707, file: !2, baseType: !105, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!710 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !704, file: !2, baseType: !711, size: 64, align: 32, extraData: i16 1)
!711 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !702, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !712, templateParams: !13, identifier: "8d84b26eccf0f48fe70ea50c79b83fc9")
!712 = !{!713}
!713 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !711, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!714 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !704, file: !2, baseType: !715, size: 64, align: 32, extraData: i16 2)
!715 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !702, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !13, identifier: "e4d910bcc0c2da0048af65cce9b02bdf")
!716 = !DIDerivedType(tag: DW_TAG_member, scope: !702, file: !2, baseType: !105, size: 16, align: 16, flags: DIFlagArtificial)
!717 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !696, file: !2, baseType: !702, size: 64, align: 32, offset: 64, flags: DIFlagPublic)
!718 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !692, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!719 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !686, file: !2, baseType: !720, size: 64, align: 32)
!720 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !684, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !721, templateParams: !690, identifier: "b6f59188292a44db7736125146b92cb0")
!721 = !{!722}
!722 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !720, file: !2, baseType: !692, size: 64, align: 32, flags: DIFlagPublic)
!723 = !DIDerivedType(tag: DW_TAG_member, scope: !684, file: !2, baseType: !28, size: 32, align: 32, flags: DIFlagArtificial)
!724 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !675, file: !2, baseType: !725, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!725 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !2, size: 64, align: 32, elements: !726, templateParams: !13, identifier: "14634098cacc86d372c43019bc81f26f")
!726 = !{!727, !757}
!727 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !725, file: !2, baseType: !728, size: 32, align: 32)
!728 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !729, size: 32, align: 32, dwarfAddressSpace: 0)
!729 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !697, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !730, templateParams: !13, identifier: "14dca3c1b1040cd8e8db0eaa112c8216")
!730 = !{!731}
!731 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !729, file: !2, baseType: !732, size: 64, align: 32, flags: DIFlagPrivate)
!732 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !697, file: !2, size: 64, align: 32, flags: DIFlagPrivate, elements: !733, templateParams: !13, identifier: "fb1492950c21086074bab206592842dc")
!733 = !{!734}
!734 = !DICompositeType(tag: DW_TAG_variant_part, scope: !732, file: !2, size: 64, align: 32, elements: !735, templateParams: !13, identifier: "478e018ae6e38e2110d0d424641ab18", discriminator: !756)
!735 = !{!736, !752}
!736 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !734, file: !2, baseType: !737, size: 64, align: 32)
!737 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !732, file: !2, size: 64, align: 32, flags: DIFlagPrivate, elements: !738, templateParams: !13, identifier: "59bc7f5c5a99ab4be3c3f06b9190c327")
!738 = !{!739, !743, !747}
!739 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !737, file: !2, baseType: !740, size: 32, align: 32, flags: DIFlagPrivate)
!740 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !315, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !741, templateParams: !545, identifier: "d9f2bcb64deb934daba9b509aea4a83e")
!741 = !{!742}
!742 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !740, file: !2, baseType: !6, size: 32, align: 32, flags: DIFlagPrivate)
!743 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !737, file: !2, baseType: !744, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!744 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !745, size: 32, align: 32, dwarfAddressSpace: 0)
!745 = !DISubroutineType(types: !746)
!746 = !{!78, !740, !97}
!747 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !737, file: !2, baseType: !748, align: 8, offset: 64, flags: DIFlagPrivate)
!748 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !329, file: !2, align: 8, flags: DIFlagPublic, elements: !13, templateParams: !749, identifier: "e71ee38df7dbfccdae82d3411c10d5bc")
!749 = !{!750}
!750 = !DITemplateTypeParameter(name: "T", type: !751)
!751 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !7, size: 32, align: 32, dwarfAddressSpace: 0)
!752 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !734, file: !2, baseType: !753, size: 64, align: 32, extraData: i32 0)
!753 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !732, file: !2, size: 64, align: 32, flags: DIFlagPrivate, elements: !754, templateParams: !13, identifier: "bcc61db69ea5777ac138ac099ea396b2")
!754 = !{!755}
!755 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !753, file: !2, baseType: !105, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!756 = !DIDerivedType(tag: DW_TAG_member, scope: !732, file: !2, baseType: !28, size: 32, align: 32, flags: DIFlagArtificial)
!757 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !725, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!758 = !{!759, !760}
!759 = !DILocalVariable(name: "fmt", arg: 1, scope: !672, file: !641, line: 95, type: !675)
!760 = !DILocalVariable(name: "force_no_backtrace", arg: 2, scope: !672, file: !641, line: 95, type: !153)
!761 = !DILocation(line: 95, column: 33, scope: !672)
!762 = !DILocation(line: 95, column: 58, scope: !672)
!763 = !DILocation(line: 2435, column: 27, scope: !764)
!764 = !DILexicalBlockFile(scope: !672, file: !66, discriminator: 0)
!765 = !DILocation(line: 2435, column: 9, scope: !764)
!766 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9panicking18panic_nounwind_fmt7runtime17hb1485ab00c51cf61E", scope: !767, file: !66, line: 2423, type: !673, scopeLine: 2423, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !768)
!767 = !DINamespace(name: "panic_nounwind_fmt", scope: !643)
!768 = !{!769, !770, !771}
!769 = !DILocalVariable(name: "fmt", arg: 1, scope: !766, file: !66, line: 2423, type: !675)
!770 = !DILocalVariable(name: "force_no_backtrace", arg: 2, scope: !766, file: !66, line: 2423, type: !153)
!771 = !DILocalVariable(name: "pi", scope: !772, file: !641, line: 114, type: !773, align: 32)
!772 = distinct !DILexicalBlock(scope: !766, file: !641, line: 114, column: 13)
!773 = !DICompositeType(tag: DW_TAG_structure_type, name: "PanicInfo", scope: !774, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !775, templateParams: !13, identifier: "74943ad5cfeaa8d7c3439d6f603267a6")
!774 = !DINamespace(name: "panic_info", scope: !311)
!775 = !{!776, !778, !779, !780}
!776 = !DIDerivedType(tag: DW_TAG_member, name: "message", scope: !773, file: !2, baseType: !777, size: 32, align: 32, flags: DIFlagPrivate)
!777 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::Arguments", baseType: !675, size: 32, align: 32, dwarfAddressSpace: 0)
!778 = !DIDerivedType(tag: DW_TAG_member, name: "location", scope: !773, file: !2, baseType: !308, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!779 = !DIDerivedType(tag: DW_TAG_member, name: "can_unwind", scope: !773, file: !2, baseType: !153, size: 8, align: 8, offset: 64, flags: DIFlagPrivate)
!780 = !DIDerivedType(tag: DW_TAG_member, name: "force_no_backtrace", scope: !773, file: !2, baseType: !153, size: 8, align: 8, offset: 72, flags: DIFlagPrivate)
!781 = !DILocation(line: 2423, column: 40, scope: !766)
!782 = !DILocation(line: 103, column: 17, scope: !783)
!783 = !DILexicalBlockFile(scope: !766, file: !641, discriminator: 0)
!784 = distinct !DISubprogram(name: "panic", linkageName: "_ZN4core9panicking5panic17h2ce9e1c499078148E", scope: !643, file: !641, line: 138, type: !785, scopeLine: 138, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !787)
!785 = !DISubroutineType(types: !786)
!786 = !{null, !332, !308}
!787 = !{!788}
!788 = !DILocalVariable(name: "expr", arg: 1, scope: !784, file: !641, line: 138, type: !332)
!789 = !DILocation(line: 138, column: 20, scope: !784)
!790 = !DILocation(line: 150, column: 42, scope: !784)
!791 = !DILocation(line: 150, column: 15, scope: !784)
!792 = !DILocation(line: 150, column: 5, scope: !784)
!793 = distinct !DISubprogram(name: "panic_fmt", linkageName: "_ZN4core9panicking9panic_fmt17hd5d6a6d9a54ae566E", scope: !643, file: !641, line: 60, type: !794, scopeLine: 60, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !796)
!794 = !DISubroutineType(types: !795)
!795 = !{null, !675, !308}
!796 = !{!797, !798}
!797 = !DILocalVariable(name: "fmt", arg: 1, scope: !793, file: !641, line: 60, type: !675)
!798 = !DILocalVariable(name: "pi", scope: !799, file: !641, line: 72, type: !773, align: 32)
!799 = distinct !DILexicalBlock(scope: !793, file: !641, line: 72, column: 5)
!800 = !DILocation(line: 60, column: 24, scope: !793)
!801 = !DILocation(line: 62, column: 9, scope: !793)
!802 = distinct !DISubprogram(name: "maybe_is_aligned", linkageName: "_ZN4core9ub_checks16maybe_is_aligned17h89a54d7a782a9e81E", scope: !803, file: !379, line: 135, type: !804, scopeLine: 135, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !806)
!803 = !DINamespace(name: "ub_checks", scope: !27)
!804 = !DISubroutineType(types: !805)
!805 = !{!153, !6, !9}
!806 = !{!807, !808}
!807 = !DILocalVariable(name: "ptr", arg: 1, scope: !802, file: !379, line: 135, type: !6)
!808 = !DILocalVariable(name: "align", arg: 2, scope: !802, file: !379, line: 135, type: !9)
!809 = !DILocation(line: 135, column: 38, scope: !802)
!810 = !DILocation(line: 135, column: 54, scope: !802)
!811 = !DILocation(line: 2435, column: 9, scope: !812)
!812 = !DILexicalBlockFile(scope: !802, file: !66, discriminator: 0)
!813 = !DILocation(line: 145, column: 2, scope: !802)
!814 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9ub_checks16maybe_is_aligned7runtime17h33a0b2aefdca9fdaE", scope: !815, file: !66, line: 2423, type: !804, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !816)
!815 = !DINamespace(name: "maybe_is_aligned", scope: !803)
!816 = !{!817, !818}
!817 = !DILocalVariable(name: "ptr", arg: 1, scope: !814, file: !66, line: 2423, type: !6)
!818 = !DILocalVariable(name: "align", arg: 2, scope: !814, file: !66, line: 2423, type: !9)
!819 = !DILocation(line: 2423, column: 40, scope: !814)
!820 = !DILocation(line: 142, column: 17, scope: !821)
!821 = !DILexicalBlockFile(scope: !814, file: !379, discriminator: 0)
!822 = !DILocation(line: 2425, column: 10, scope: !814)
!823 = distinct !DISubprogram(name: "check_language_ub", linkageName: "_ZN4core9ub_checks17check_language_ub17hc9ddcf3001b97422E", scope: !803, file: !379, line: 96, type: !824, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!824 = !DISubroutineType(types: !825)
!825 = !{!153}
!826 = !DILocation(line: 98, column: 5, scope: !823)
!827 = !DILocation(line: 2435, column: 9, scope: !828)
!828 = !DILexicalBlockFile(scope: !823, file: !66, discriminator: 0)
!829 = !DILocation(line: 109, column: 2, scope: !823)
!830 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9ub_checks17check_language_ub7runtime17h245850c936c392bbE", scope: !831, file: !66, line: 2423, type: !824, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13)
!831 = !DINamespace(name: "check_language_ub", scope: !803)
!832 = !DILocation(line: 2425, column: 10, scope: !830)
!833 = distinct !DISubprogram(name: "maybe_is_nonoverlapping", linkageName: "_ZN4core9ub_checks23maybe_is_nonoverlapping17h63367108861f9f74E", scope: !803, file: !379, line: 160, type: !834, scopeLine: 160, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !836)
!834 = !DISubroutineType(types: !835)
!835 = !{!153, !6, !6, !9, !9}
!836 = !{!837, !838, !839, !840}
!837 = !DILocalVariable(name: "src", arg: 1, scope: !833, file: !379, line: 161, type: !6)
!838 = !DILocalVariable(name: "dst", arg: 2, scope: !833, file: !379, line: 162, type: !6)
!839 = !DILocalVariable(name: "size", arg: 3, scope: !833, file: !379, line: 163, type: !9)
!840 = !DILocalVariable(name: "count", arg: 4, scope: !833, file: !379, line: 164, type: !9)
!841 = !DILocation(line: 161, column: 5, scope: !833)
!842 = !DILocation(line: 162, column: 5, scope: !833)
!843 = !DILocation(line: 163, column: 5, scope: !833)
!844 = !DILocation(line: 164, column: 5, scope: !833)
!845 = !DILocation(line: 2435, column: 27, scope: !846)
!846 = !DILexicalBlockFile(scope: !833, file: !66, discriminator: 0)
!847 = !DILocation(line: 2435, column: 9, scope: !846)
!848 = !DILocation(line: 185, column: 2, scope: !833)
!849 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9ub_checks23maybe_is_nonoverlapping7runtime17h046557a2ba0f4e7cE", scope: !850, file: !66, line: 2423, type: !834, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !851)
!850 = !DINamespace(name: "maybe_is_nonoverlapping", scope: !803)
!851 = !{!852, !853, !854, !855, !856, !858, !860, !862}
!852 = !DILocalVariable(name: "src", arg: 1, scope: !849, file: !66, line: 2423, type: !6)
!853 = !DILocalVariable(name: "dst", arg: 2, scope: !849, file: !66, line: 2423, type: !6)
!854 = !DILocalVariable(name: "size", arg: 3, scope: !849, file: !66, line: 2423, type: !9)
!855 = !DILocalVariable(name: "count", arg: 4, scope: !849, file: !66, line: 2423, type: !9)
!856 = !DILocalVariable(name: "src_usize", scope: !857, file: !379, line: 172, type: !9, align: 32)
!857 = distinct !DILexicalBlock(scope: !849, file: !379, line: 172, column: 13)
!858 = !DILocalVariable(name: "dst_usize", scope: !859, file: !379, line: 173, type: !9, align: 32)
!859 = distinct !DILexicalBlock(scope: !857, file: !379, line: 173, column: 13)
!860 = !DILocalVariable(name: "size", scope: !861, file: !379, line: 174, type: !9, align: 32)
!861 = distinct !DILexicalBlock(scope: !859, file: !379, line: 174, column: 13)
!862 = !DILocalVariable(name: "diff", scope: !863, file: !379, line: 179, type: !9, align: 32)
!863 = distinct !DILexicalBlock(scope: !861, file: !379, line: 179, column: 13)
!864 = !DILocation(line: 2423, column: 40, scope: !849)
!865 = !DILocation(line: 153, column: 17, scope: !541, inlinedAt: !866)
!866 = distinct !DILocation(line: 172, column: 33, scope: !867)
!867 = !DILexicalBlockFile(scope: !849, file: !379, discriminator: 0)
!868 = !DILocation(line: 48, column: 26, scope: !549, inlinedAt: !869)
!869 = distinct !DILocation(line: 159, column: 38, scope: !541, inlinedAt: !866)
!870 = !DILocation(line: 159, column: 18, scope: !541, inlinedAt: !866)
!871 = !DILocation(line: 172, column: 33, scope: !867)
!872 = !DILocation(line: 172, column: 17, scope: !857)
!873 = !DILocation(line: 153, column: 17, scope: !541, inlinedAt: !874)
!874 = distinct !DILocation(line: 173, column: 33, scope: !857)
!875 = !DILocation(line: 48, column: 26, scope: !549, inlinedAt: !876)
!876 = distinct !DILocation(line: 159, column: 38, scope: !541, inlinedAt: !874)
!877 = !DILocation(line: 159, column: 18, scope: !541, inlinedAt: !874)
!878 = !DILocation(line: 173, column: 33, scope: !857)
!879 = !DILocation(line: 173, column: 17, scope: !859)
!880 = !DILocation(line: 174, column: 35, scope: !859)
!881 = !DILocation(line: 174, column: 30, scope: !859)
!882 = !DILocation(line: 174, column: 17, scope: !859)
!883 = !DILocation(line: 174, column: 22, scope: !859)
!884 = !DILocation(line: 174, column: 22, scope: !861)
!885 = !DILocation(line: 179, column: 34, scope: !861)
!886 = !DILocation(line: 179, column: 17, scope: !863)
!887 = !DILocation(line: 182, column: 13, scope: !863)
!888 = !DILocation(line: 2425, column: 10, scope: !849)
!889 = !DILocation(line: 175, column: 17, scope: !859)
!890 = !DILocation(line: 2423, column: 9, scope: !849)
!891 = distinct !DISubprogram(name: "is_valid_allocation_size", linkageName: "_ZN4core9ub_checks24is_valid_allocation_size17hceea5d110dce2c5eE", scope: !803, file: !379, line: 148, type: !892, scopeLine: 148, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !894)
!892 = !DISubroutineType(types: !893)
!893 = !{!153, !9, !9}
!894 = !{!895, !896, !897}
!895 = !DILocalVariable(name: "size", arg: 1, scope: !891, file: !379, line: 148, type: !9)
!896 = !DILocalVariable(name: "len", arg: 2, scope: !891, file: !379, line: 148, type: !9)
!897 = !DILocalVariable(name: "max_len", scope: !898, file: !379, line: 149, type: !9, align: 32)
!898 = distinct !DILexicalBlock(scope: !891, file: !379, line: 149, column: 5)
!899 = !DILocation(line: 148, column: 46, scope: !891)
!900 = !DILocation(line: 148, column: 59, scope: !891)
!901 = !DILocation(line: 149, column: 9, scope: !898)
!902 = !DILocation(line: 149, column: 22, scope: !891)
!903 = !DILocation(line: 149, column: 34, scope: !891)
!904 = !DILocation(line: 149, column: 19, scope: !891)
!905 = !DILocation(line: 149, column: 54, scope: !891)
!906 = !DILocation(line: 150, column: 12, scope: !898)
!907 = !DILocation(line: 150, column: 5, scope: !898)
!908 = !DILocation(line: 151, column: 2, scope: !891)
!909 = distinct !DISubprogram(name: "maybe_is_aligned_and_not_null", linkageName: "_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hb16a941476d8d9a1E", scope: !803, file: !379, line: 119, type: !910, scopeLine: 119, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !912)
!910 = !DISubroutineType(types: !911)
!911 = !{!153, !6, !9, !153}
!912 = !{!913, !914, !915}
!913 = !DILocalVariable(name: "ptr", arg: 1, scope: !909, file: !379, line: 120, type: !6)
!914 = !DILocalVariable(name: "align", arg: 2, scope: !909, file: !379, line: 121, type: !9)
!915 = !DILocalVariable(name: "is_zst", arg: 3, scope: !909, file: !379, line: 122, type: !153)
!916 = !DILocation(line: 120, column: 5, scope: !909)
!917 = !DILocation(line: 121, column: 5, scope: !909)
!918 = !DILocation(line: 122, column: 5, scope: !909)
!919 = !DILocation(line: 125, column: 5, scope: !909)
!920 = !DILocation(line: 125, column: 38, scope: !909)
!921 = !DILocation(line: 126, column: 2, scope: !909)
!922 = !DILocation(line: 125, column: 53, scope: !909)
!923 = !DILocation(line: 125, column: 48, scope: !909)
!924 = !DILocation(line: 125, column: 37, scope: !909)
!925 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN65_$LT$hashbrown..control..tag..Tag$u20$as$u20$core..fmt..Debug$GT$3fmt17h9c56e725206ffa8aE", scope: !927, file: !926, line: 53, type: !928, scopeLine: 53, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !931)
!926 = !DIFile(filename: "src/control/tag.rs", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5", checksumkind: CSK_MD5, checksum: "8d1dbc376b4c6e81b128daf60eb1eaa3")
!927 = !DINamespace(name: "{impl#1}", scope: !446)
!928 = !DISubroutineType(types: !929)
!929 = !{!78, !930, !97}
!930 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&hashbrown::control::tag::Tag", baseType: !445, size: 32, align: 32, dwarfAddressSpace: 0)
!931 = !{!932, !933}
!932 = !DILocalVariable(name: "self", arg: 1, scope: !925, file: !926, line: 53, type: !930)
!933 = !DILocalVariable(name: "f", arg: 2, scope: !925, file: !926, line: 53, type: !97)
!934 = !DILocation(line: 53, column: 12, scope: !925)
!935 = !DILocation(line: 53, column: 19, scope: !925)
!936 = !DILocation(line: 54, column: 12, scope: !925)
!937 = !DILocation(line: 54, column: 17, scope: !925)
!938 = !DILocation(line: 61, column: 15, scope: !925)
!939 = !DILocation(line: 61, column: 43, scope: !925)
!940 = !DILocation(line: 61, column: 42, scope: !925)
!941 = !DILocation(line: 61, column: 35, scope: !925)
!942 = !DILocation(line: 61, column: 59, scope: !925)
!943 = !DILocation(line: 55, column: 16, scope: !925)
!944 = !DILocation(line: 55, column: 21, scope: !925)
!945 = !DILocation(line: 63, column: 6, scope: !925)
!946 = !DILocation(line: 58, column: 19, scope: !925)
!947 = !DILocation(line: 56, column: 19, scope: !925)
!948 = distinct !DISubprogram(name: "next", linkageName: "_ZN91_$LT$hashbrown..raw..RawIterHashInner$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfbc9bd705ca4f856E", scope: !950, file: !949, line: 4120, type: !951, scopeLine: 4120, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !981)
!949 = !DIFile(filename: "src/raw/mod.rs", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5", checksumkind: CSK_MD5, checksum: "5d3b64719ee64b7578bc74be56a5ef35")
!950 = !DINamespace(name: "{impl#56}", scope: !19)
!951 = !DISubroutineType(types: !952)
!952 = !{!196, !953}
!953 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut hashbrown::raw::RawIterHashInner", baseType: !954, size: 32, align: 32, dwarfAddressSpace: 0)
!954 = !DICompositeType(tag: DW_TAG_structure_type, name: "RawIterHashInner", scope: !19, file: !2, size: 320, align: 64, flags: DIFlagPrivate, elements: !955, templateParams: !13, identifier: "f022cf0990b10a707cb598fe3db8a6dc")
!955 = !{!956, !957, !961, !962, !967, !973}
!956 = !DIDerivedType(tag: DW_TAG_member, name: "bucket_mask", scope: !954, file: !2, baseType: !9, size: 32, align: 32, offset: 192, flags: DIFlagPrivate)
!957 = !DIDerivedType(tag: DW_TAG_member, name: "ctrl", scope: !954, file: !2, baseType: !958, size: 32, align: 32, offset: 224, flags: DIFlagPrivate)
!958 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<u8>", scope: !315, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !959, templateParams: !323, identifier: "bfbed5a29c49721772982c8bebfc3819")
!959 = !{!960}
!960 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !958, file: !2, baseType: !368, size: 32, align: 32, flags: DIFlagPrivate)
!961 = !DIDerivedType(tag: DW_TAG_member, name: "tag_hash", scope: !954, file: !2, baseType: !445, size: 8, align: 8, offset: 256, flags: DIFlagPrivate)
!962 = !DIDerivedType(tag: DW_TAG_member, name: "probe_seq", scope: !954, file: !2, baseType: !963, size: 64, align: 32, flags: DIFlagPrivate)
!963 = !DICompositeType(tag: DW_TAG_structure_type, name: "ProbeSeq", scope: !19, file: !2, size: 64, align: 32, flags: DIFlagPrivate, elements: !964, templateParams: !13, identifier: "342b85286dfa0726b0f97165b1230ff8")
!964 = !{!965, !966}
!965 = !DIDerivedType(tag: DW_TAG_member, name: "pos", scope: !963, file: !2, baseType: !9, size: 32, align: 32, flags: DIFlagPrivate)
!966 = !DIDerivedType(tag: DW_TAG_member, name: "stride", scope: !963, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!967 = !DIDerivedType(tag: DW_TAG_member, name: "group", scope: !954, file: !2, baseType: !968, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!968 = !DICompositeType(tag: DW_TAG_structure_type, name: "Group", scope: !969, file: !2, size: 64, align: 64, flags: DIFlagProtected, elements: !971, templateParams: !13, identifier: "97de3c78f795ee25abf27a63fe17a735")
!969 = !DINamespace(name: "generic", scope: !970)
!970 = !DINamespace(name: "group", scope: !447)
!971 = !{!972}
!972 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !968, file: !2, baseType: !183, size: 64, align: 64, flags: DIFlagPrivate)
!973 = !DIDerivedType(tag: DW_TAG_member, name: "bitmask", scope: !954, file: !2, baseType: !974, size: 64, align: 64, offset: 128, flags: DIFlagPrivate)
!974 = !DICompositeType(tag: DW_TAG_structure_type, name: "BitMaskIter", scope: !975, file: !2, size: 64, align: 64, flags: DIFlagProtected, elements: !976, templateParams: !13, identifier: "da38e1804ebd49db364966dbd76650cd")
!975 = !DINamespace(name: "bitmask", scope: !447)
!976 = !{!977}
!977 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !974, file: !2, baseType: !978, size: 64, align: 64, flags: DIFlagProtected)
!978 = !DICompositeType(tag: DW_TAG_structure_type, name: "BitMask", scope: !975, file: !2, size: 64, align: 64, flags: DIFlagProtected, elements: !979, templateParams: !13, identifier: "12a60fc4469a30652a9c7c91f3fd3de")
!979 = !{!980}
!980 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !978, file: !2, baseType: !183, size: 64, align: 64, flags: DIFlagProtected)
!981 = !{!982, !983, !985, !987, !989}
!982 = !DILocalVariable(name: "self", arg: 1, scope: !948, file: !949, line: 4120, type: !953)
!983 = !DILocalVariable(name: "bit", scope: !984, file: !949, line: 4123, type: !9, align: 32)
!984 = distinct !DILexicalBlock(scope: !948, file: !949, line: 4123, column: 56)
!985 = !DILocalVariable(name: "index", scope: !986, file: !949, line: 4124, type: !9, align: 32)
!986 = distinct !DILexicalBlock(scope: !984, file: !949, line: 4124, column: 21)
!987 = !DILocalVariable(name: "index", scope: !988, file: !949, line: 4134, type: !9, align: 32)
!988 = distinct !DILexicalBlock(scope: !948, file: !949, line: 4134, column: 17)
!989 = !DILocalVariable(name: "group_ctrl", scope: !990, file: !949, line: 4136, type: !451, align: 32)
!990 = distinct !DILexicalBlock(scope: !988, file: !949, line: 4136, column: 17)
!991 = !DILocation(line: 4120, column: 13, scope: !948)
!992 = !DILocation(line: 4122, column: 13, scope: !948)
!993 = !DILocation(line: 4123, column: 36, scope: !984)
!994 = !DILocation(line: 4123, column: 49, scope: !984)
!995 = !DILocation(line: 4123, column: 24, scope: !984)
!996 = !DILocation(line: 4123, column: 29, scope: !984)
!997 = !DILocation(line: 4124, column: 34, scope: !984)
!998 = !DILocation(line: 4124, column: 33, scope: !984)
!999 = !DILocation(line: 4127, column: 27, scope: !948)
!1000 = !DILocation(line: 4127, column: 38, scope: !948)
!1001 = !DILocation(line: 4127, column: 52, scope: !948)
!1002 = !DILocalVariable(name: "b", arg: 1, scope: !1003, file: !66, line: 433, type: !153)
!1003 = distinct !DISubprogram(name: "likely", linkageName: "_ZN4core10intrinsics6likely17h48e44d0be517eb37E", scope: !67, file: !66, line: 433, type: !246, scopeLine: 433, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !1004)
!1004 = !{!1002}
!1005 = !DILocation(line: 433, column: 21, scope: !1003, inlinedAt: !1006)
!1006 = distinct !DILocation(line: 4127, column: 20, scope: !948)
!1007 = !DILocation(line: 434, column: 8, scope: !1003, inlinedAt: !1006)
!1008 = !DILocation(line: 438, column: 9, scope: !1003, inlinedAt: !1006)
!1009 = !DILocation(line: 434, column: 5, scope: !1003, inlinedAt: !1006)
!1010 = !DILocation(line: 435, column: 9, scope: !1003, inlinedAt: !1006)
!1011 = !DILocation(line: 440, column: 2, scope: !1003, inlinedAt: !1006)
!1012 = !DILocation(line: 4127, column: 20, scope: !948)
!1013 = !DILocation(line: 4124, column: 62, scope: !984)
!1014 = !DILocation(line: 4124, column: 25, scope: !986)
!1015 = !DILocation(line: 4125, column: 28, scope: !986)
!1016 = !DILocation(line: 0, scope: !1017)
!1017 = !DILexicalBlockFile(scope: !948, file: !1018, discriminator: 0)
!1018 = !DIFile(filename: "src/lib.rs", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5", checksumkind: CSK_MD5, checksum: "858460b20e056d803025b9ecb564b761")
!1019 = !DILocation(line: 4142, column: 6, scope: !948)
!1020 = !DILocation(line: 4130, column: 42, scope: !948)
!1021 = !DILocation(line: 4130, column: 32, scope: !948)
!1022 = !DILocation(line: 4134, column: 29, scope: !948)
!1023 = !DILocation(line: 4134, column: 21, scope: !988)
!1024 = !DILocation(line: 4135, column: 39, scope: !988)
!1025 = !DILocation(line: 4128, column: 28, scope: !948)
!1026 = !DILocation(line: 4135, column: 31, scope: !988)
!1027 = !DILocation(line: 4135, column: 17, scope: !988)
!1028 = !DILocation(line: 4136, column: 34, scope: !988)
!1029 = !DILocalVariable(name: "self", arg: 1, scope: !1030, file: !1031, line: 401, type: !958)
!1030 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hc931bdbf206410afE", scope: !958, file: !1031, line: 401, type: !1032, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !323, declaration: !1034, retainedNodes: !1035)
!1031 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs", directory: "", checksumkind: CSK_MD5, checksum: "6726e73c6c894eba30d90288586d0f43")
!1032 = !DISubroutineType(types: !1033)
!1033 = !{!369, !958}
!1034 = !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hc931bdbf206410afE", scope: !958, file: !1031, line: 401, type: !1032, scopeLine: 401, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !323)
!1035 = !{!1029}
!1036 = !DILocation(line: 401, column: 25, scope: !1030, inlinedAt: !1037)
!1037 = distinct !DILocation(line: 4136, column: 44, scope: !988)
!1038 = !DILocalVariable(name: "self", arg: 1, scope: !1039, file: !504, line: 927, type: !369)
!1039 = distinct !DISubprogram(name: "add<u8>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h1aafe67359b89d4eE", scope: !490, file: !504, line: 927, type: !1040, scopeLine: 927, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !323, retainedNodes: !1042)
!1040 = !DISubroutineType(types: !1041)
!1041 = !{!369, !369, !9, !308}
!1042 = !{!1038, !1043}
!1043 = !DILocalVariable(name: "count", arg: 2, scope: !1039, file: !504, line: 927, type: !9)
!1044 = !DILocation(line: 927, column: 29, scope: !1039, inlinedAt: !1045)
!1045 = distinct !DILocation(line: 4136, column: 53, scope: !988)
!1046 = !DILocation(line: 927, column: 35, scope: !1039, inlinedAt: !1045)
!1047 = !DILocation(line: 77, column: 35, scope: !1048, inlinedAt: !1045)
!1048 = !DILexicalBlockFile(scope: !1039, file: !379, discriminator: 0)
!1049 = !DILocation(line: 78, column: 17, scope: !1048, inlinedAt: !1045)
!1050 = !DILocation(line: 961, column: 18, scope: !1039, inlinedAt: !1045)
!1051 = !DILocalVariable(name: "self", arg: 1, scope: !1052, file: !504, line: 31, type: !369)
!1052 = distinct !DISubprogram(name: "cast<u8, hashbrown::control::tag::Tag>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$4cast17hcf32ef9ad39781adE", scope: !490, file: !504, line: 31, type: !1053, scopeLine: 31, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !1056, retainedNodes: !1055)
!1053 = !DISubroutineType(types: !1054)
!1054 = !{!451, !369}
!1055 = !{!1051}
!1056 = !{!324, !1057}
!1057 = !DITemplateTypeParameter(name: "U", type: !445)
!1058 = !DILocation(line: 31, column: 26, scope: !1052, inlinedAt: !1059)
!1059 = distinct !DILocation(line: 4136, column: 64, scope: !988)
!1060 = !DILocation(line: 4136, column: 64, scope: !988)
!1061 = !DILocation(line: 4136, column: 21, scope: !990)
!1062 = !DILocation(line: 4138, column: 30, scope: !990)
!1063 = !DILocation(line: 4138, column: 17, scope: !990)
!1064 = !DILocation(line: 4139, column: 32, scope: !990)
!1065 = !DILocation(line: 4139, column: 53, scope: !990)
!1066 = !DILocation(line: 4139, column: 43, scope: !990)
!1067 = !DILocation(line: 4139, column: 68, scope: !990)
!1068 = !DILocation(line: 4139, column: 17, scope: !990)
!1069 = !DILocation(line: 4120, column: 5, scope: !948)
!1070 = distinct !DISubprogram(name: "into_iter", linkageName: "_ZN98_$LT$hashbrown..control..bitmask..BitMask$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h1a833b0d45907438E", scope: !1072, file: !1071, line: 96, type: !1073, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !1075)
!1071 = !DIFile(filename: "src/control/bitmask.rs", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5", checksumkind: CSK_MD5, checksum: "ff7f3e6a3e2daa5cd0d49465a9feef11")
!1072 = !DINamespace(name: "{impl#1}", scope: !975)
!1073 = !DISubroutineType(types: !1074)
!1074 = !{!974, !978}
!1075 = !{!1076}
!1076 = !DILocalVariable(name: "self", arg: 1, scope: !1070, file: !1071, line: 96, type: !978)
!1077 = !DILocation(line: 96, column: 18, scope: !1070)
!1078 = !DILocation(line: 99, column: 29, scope: !1070)
!1079 = !DILocation(line: 100, column: 6, scope: !1070)
!1080 = distinct !DISubprogram(name: "next", linkageName: "_ZN99_$LT$hashbrown..control..bitmask..BitMaskIter$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he508eeb6e77805ebE", scope: !1081, file: !1071, line: 112, type: !1082, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !1085)
!1081 = !DINamespace(name: "{impl#2}", scope: !975)
!1082 = !DISubroutineType(types: !1083)
!1083 = !{!196, !1084}
!1084 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut hashbrown::control::bitmask::BitMaskIter", baseType: !974, size: 32, align: 32, dwarfAddressSpace: 0)
!1085 = !{!1086, !1087, !1089, !1107}
!1086 = !DILocalVariable(name: "self", arg: 1, scope: !1080, file: !1071, line: 112, type: !1084)
!1087 = !DILocalVariable(name: "bit", scope: !1088, file: !1071, line: 113, type: !9, align: 32)
!1088 = distinct !DILexicalBlock(scope: !1080, file: !1071, line: 113, column: 9)
!1089 = !DILocalVariable(name: "residual", scope: !1090, file: !1071, line: 113, type: !1091, align: 8)
!1090 = distinct !DILexicalBlock(scope: !1080, file: !1071, line: 113, column: 42)
!1091 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<core::convert::Infallible>", scope: !197, file: !2, align: 8, flags: DIFlagPublic, elements: !1092, templateParams: !13, identifier: "9b6c9073db236d9ff1d56f0661f0ec14")
!1092 = !{!1093}
!1093 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1091, file: !2, align: 8, elements: !1094, templateParams: !13, identifier: "a233f30efe3bb8b1e75bd84589e6176e")
!1094 = !{!1095, !1103}
!1095 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1093, file: !2, baseType: !1096, align: 8)
!1096 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1091, file: !2, align: 8, flags: DIFlagPublic, elements: !13, templateParams: !1097, identifier: "6cefa6fa2abaa24ef975313467073b05")
!1097 = !{!1098}
!1098 = !DITemplateTypeParameter(name: "T", type: !1099)
!1099 = !DICompositeType(tag: DW_TAG_structure_type, name: "Infallible", scope: !1100, file: !2, align: 8, flags: DIFlagPublic, elements: !1101, templateParams: !13, identifier: "bbec56e295cb17a3c6590c058bc34564")
!1100 = !DINamespace(name: "convert", scope: !27)
!1101 = !{!1102}
!1102 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1099, file: !2, align: 8, elements: !13, identifier: "54bd9ba32f82ed48b888ad889b266af3")
!1103 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1093, file: !2, baseType: !1104, align: 8)
!1104 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1091, file: !2, align: 8, flags: DIFlagPublic, elements: !1105, templateParams: !1097, identifier: "7799c6cbd728f2f9bec7debffe6d8503")
!1105 = !{!1106}
!1106 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1104, file: !2, baseType: !1099, align: 8, flags: DIFlagPublic)
!1107 = !DILocalVariable(name: "val", scope: !1108, file: !1071, line: 113, type: !9, align: 32)
!1108 = distinct !DILexicalBlock(scope: !1080, file: !1071, line: 113, column: 19)
!1109 = !DILocation(line: 112, column: 13, scope: !1080)
!1110 = !DILocation(line: 113, column: 42, scope: !1090)
!1111 = !DILocation(line: 113, column: 19, scope: !1080)
!1112 = !DILocation(line: 113, column: 26, scope: !1080)
!1113 = !DILocation(line: 113, column: 19, scope: !1090)
!1114 = !DILocation(line: 113, column: 13, scope: !1088)
!1115 = !DILocation(line: 113, column: 19, scope: !1108)
!1116 = !DILocation(line: 114, column: 18, scope: !1088)
!1117 = !DILocation(line: 114, column: 25, scope: !1088)
!1118 = !DILocation(line: 114, column: 9, scope: !1088)
!1119 = !DILocation(line: 115, column: 9, scope: !1088)
!1120 = !DILocation(line: 116, column: 6, scope: !1080)
!1121 = distinct !DISubprogram(name: "capacity_overflow", linkageName: "_ZN9hashbrown3raw11Fallibility17capacity_overflow17hda25fc3126d0db47E", scope: !18, file: !949, line: 34, type: !1122, scopeLine: 34, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1144, retainedNodes: !1145)
!1122 = !DISubroutineType(types: !1123)
!1123 = !{!1124, !18}
!1124 = !DICompositeType(tag: DW_TAG_structure_type, name: "TryReserveError", scope: !20, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1125, templateParams: !13, identifier: "c7db6892fde2fe85849ada29143f492c")
!1125 = !{!1126}
!1126 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1124, file: !2, size: 64, align: 32, elements: !1127, templateParams: !13, identifier: "5b0d7e6dfa852d792ce11f4e52f744cb", discriminator: !1143)
!1127 = !{!1128, !1130}
!1128 = !DIDerivedType(tag: DW_TAG_member, name: "CapacityOverflow", scope: !1126, file: !2, baseType: !1129, size: 64, align: 32, extraData: i32 0)
!1129 = !DICompositeType(tag: DW_TAG_structure_type, name: "CapacityOverflow", scope: !1124, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !13, identifier: "33feb620220f89531acf9f7823849277")
!1130 = !DIDerivedType(tag: DW_TAG_member, name: "AllocError", scope: !1126, file: !2, baseType: !1131, size: 64, align: 32)
!1131 = !DICompositeType(tag: DW_TAG_structure_type, name: "AllocError", scope: !1124, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1132, templateParams: !13, identifier: "2fdf5966f473746e7937de4ca6ed19bd")
!1132 = !{!1133}
!1133 = !DIDerivedType(tag: DW_TAG_member, name: "layout", scope: !1131, file: !2, baseType: !1134, size: 64, align: 32, flags: DIFlagPublic)
!1134 = !DICompositeType(tag: DW_TAG_structure_type, name: "Layout", scope: !1135, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1137, templateParams: !13, identifier: "f923cc1896f078e51d4a893c36e2e533")
!1135 = !DINamespace(name: "layout", scope: !1136)
!1136 = !DINamespace(name: "alloc", scope: !27)
!1137 = !{!1138, !1139}
!1138 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !1134, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!1139 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !1134, file: !2, baseType: !1140, size: 32, align: 32, flags: DIFlagPrivate)
!1140 = !DICompositeType(tag: DW_TAG_structure_type, name: "Alignment", scope: !25, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !1141, templateParams: !13, identifier: "b8055a5301a867de82116acd8d685318")
!1141 = !{!1142}
!1142 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1140, file: !2, baseType: !24, size: 32, align: 32, flags: DIFlagPrivate)
!1143 = !DIDerivedType(tag: DW_TAG_member, scope: !1124, file: !2, baseType: !28, size: 32, align: 32, flags: DIFlagArtificial)
!1144 = !DISubprogram(name: "capacity_overflow", linkageName: "_ZN9hashbrown3raw11Fallibility17capacity_overflow17hda25fc3126d0db47E", scope: !18, file: !949, line: 34, type: !1122, scopeLine: 34, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1145 = !{!1146}
!1146 = !DILocalVariable(name: "self", arg: 1, scope: !1121, file: !949, line: 34, type: !18)
!1147 = !DILocation(line: 34, column: 26, scope: !1121)
!1148 = !DILocation(line: 35, column: 15, scope: !1121)
!1149 = !DILocation(line: 35, column: 9, scope: !1121)
!1150 = !DILocation(line: 37, column: 40, scope: !1121)
!1151 = !DILocation(line: 39, column: 6, scope: !1121)
!1152 = distinct !DISubprogram(name: "alloc_err", linkageName: "_ZN9hashbrown3raw11Fallibility9alloc_err17h85a86bfcda93906aE", scope: !18, file: !949, line: 43, type: !1153, scopeLine: 43, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1155, retainedNodes: !1156)
!1153 = !DISubroutineType(types: !1154)
!1154 = !{!1124, !18, !1134}
!1155 = !DISubprogram(name: "alloc_err", linkageName: "_ZN9hashbrown3raw11Fallibility9alloc_err17h85a86bfcda93906aE", scope: !18, file: !949, line: 43, type: !1153, scopeLine: 43, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1156 = !{!1157, !1158}
!1157 = !DILocalVariable(name: "self", arg: 1, scope: !1152, file: !949, line: 43, type: !18)
!1158 = !DILocalVariable(name: "layout", arg: 2, scope: !1152, file: !949, line: 43, type: !1134)
!1159 = !DILocation(line: 43, column: 18, scope: !1152)
!1160 = !DILocation(line: 43, column: 24, scope: !1152)
!1161 = !DILocation(line: 44, column: 15, scope: !1152)
!1162 = !DILocation(line: 44, column: 9, scope: !1152)
!1163 = !DILocation(line: 46, column: 40, scope: !1152)
!1164 = !DILocation(line: 48, column: 6, scope: !1152)
!1165 = distinct !DISubprogram(name: "ctrl_slice", linkageName: "_ZN9hashbrown3raw13RawTableInner10ctrl_slice17h1789bac88fa3cb82E", scope: !1166, file: !949, line: 2515, type: !1172, scopeLine: 2515, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1175, retainedNodes: !1176)
!1166 = !DICompositeType(tag: DW_TAG_structure_type, name: "RawTableInner", scope: !19, file: !2, size: 128, align: 32, flags: DIFlagPrivate, elements: !1167, templateParams: !13, identifier: "37db1a21273931fc7fad72ba481a2ebb")
!1167 = !{!1168, !1169, !1170, !1171}
!1168 = !DIDerivedType(tag: DW_TAG_member, name: "bucket_mask", scope: !1166, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!1169 = !DIDerivedType(tag: DW_TAG_member, name: "ctrl", scope: !1166, file: !2, baseType: !958, size: 32, align: 32, flags: DIFlagPrivate)
!1170 = !DIDerivedType(tag: DW_TAG_member, name: "growth_left", scope: !1166, file: !2, baseType: !9, size: 32, align: 32, offset: 64, flags: DIFlagPrivate)
!1171 = !DIDerivedType(tag: DW_TAG_member, name: "items", scope: !1166, file: !2, baseType: !9, size: 32, align: 32, offset: 96, flags: DIFlagPrivate)
!1172 = !DISubroutineType(types: !1173)
!1173 = !{!606, !1174}
!1174 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut hashbrown::raw::RawTableInner", baseType: !1166, size: 32, align: 32, dwarfAddressSpace: 0)
!1175 = !DISubprogram(name: "ctrl_slice", linkageName: "_ZN9hashbrown3raw13RawTableInner10ctrl_slice17h1789bac88fa3cb82E", scope: !1166, file: !949, line: 2515, type: !1172, scopeLine: 2515, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1176 = !{!1177}
!1177 = !DILocalVariable(name: "self", arg: 1, scope: !1165, file: !949, line: 2515, type: !1174)
!1178 = !DILocation(line: 2515, column: 19, scope: !1165)
!1179 = !DILocation(line: 2517, column: 44, scope: !1165)
!1180 = !DILocation(line: 401, column: 25, scope: !1030, inlinedAt: !1181)
!1181 = distinct !DILocation(line: 2517, column: 54, scope: !1165)
!1182 = !DILocation(line: 31, column: 26, scope: !1052, inlinedAt: !1183)
!1183 = distinct !DILocation(line: 2517, column: 63, scope: !1165)
!1184 = !DILocation(line: 2517, column: 76, scope: !1165)
!1185 = !DILocation(line: 2517, column: 18, scope: !1165)
!1186 = !DILocation(line: 2518, column: 6, scope: !1165)
!1187 = distinct !DISubprogram(name: "num_ctrl_bytes", linkageName: "_ZN9hashbrown3raw13RawTableInner14num_ctrl_bytes17h95bbfa87967f2288E", scope: !1166, file: !949, line: 2537, type: !1188, scopeLine: 2537, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1191, retainedNodes: !1192)
!1188 = !DISubroutineType(types: !1189)
!1189 = !{!9, !1190}
!1190 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&hashbrown::raw::RawTableInner", baseType: !1166, size: 32, align: 32, dwarfAddressSpace: 0)
!1191 = !DISubprogram(name: "num_ctrl_bytes", linkageName: "_ZN9hashbrown3raw13RawTableInner14num_ctrl_bytes17h95bbfa87967f2288E", scope: !1166, file: !949, line: 2537, type: !1188, scopeLine: 2537, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1192 = !{!1193}
!1193 = !DILocalVariable(name: "self", arg: 1, scope: !1187, file: !949, line: 2537, type: !1190)
!1194 = !DILocation(line: 2537, column: 23, scope: !1187)
!1195 = !DILocation(line: 2538, column: 9, scope: !1187)
!1196 = !DILocation(line: 2539, column: 6, scope: !1187)
!1197 = distinct !DISubprogram(name: "ctrl", linkageName: "_ZN9hashbrown3raw13RawTableInner4ctrl17h5dd6d54905c726cfE", scope: !1166, file: !949, line: 2508, type: !1198, scopeLine: 2508, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1200, retainedNodes: !1201)
!1198 = !DISubroutineType(types: !1199)
!1199 = !{!451, !1190, !9}
!1200 = !DISubprogram(name: "ctrl", linkageName: "_ZN9hashbrown3raw13RawTableInner4ctrl17h5dd6d54905c726cfE", scope: !1166, file: !949, line: 2508, type: !1198, scopeLine: 2508, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1201 = !{!1202, !1203}
!1202 = !DILocalVariable(name: "self", arg: 1, scope: !1197, file: !949, line: 2508, type: !1190)
!1203 = !DILocalVariable(name: "index", arg: 2, scope: !1197, file: !949, line: 2508, type: !9)
!1204 = !DILocation(line: 2508, column: 20, scope: !1197)
!1205 = !DILocation(line: 2508, column: 27, scope: !1197)
!1206 = !DILocation(line: 2509, column: 36, scope: !1197)
!1207 = !DILocation(line: 2509, column: 23, scope: !1197)
!1208 = !DILocation(line: 2509, column: 9, scope: !1197)
!1209 = !DILocation(line: 2511, column: 9, scope: !1197)
!1210 = !DILocation(line: 401, column: 25, scope: !1030, inlinedAt: !1211)
!1211 = distinct !DILocation(line: 2511, column: 19, scope: !1197)
!1212 = !DILocation(line: 927, column: 29, scope: !1039, inlinedAt: !1213)
!1213 = distinct !DILocation(line: 2511, column: 28, scope: !1197)
!1214 = !DILocation(line: 927, column: 35, scope: !1039, inlinedAt: !1213)
!1215 = !DILocation(line: 77, column: 35, scope: !1048, inlinedAt: !1213)
!1216 = !DILocation(line: 78, column: 17, scope: !1048, inlinedAt: !1213)
!1217 = !DILocation(line: 961, column: 18, scope: !1039, inlinedAt: !1213)
!1218 = !DILocation(line: 31, column: 26, scope: !1052, inlinedAt: !1219)
!1219 = distinct !DILocation(line: 2511, column: 39, scope: !1197)
!1220 = !DILocation(line: 2512, column: 6, scope: !1197)
!1221 = distinct !DISubprogram(name: "probe_seq", linkageName: "_ZN9hashbrown3raw13RawTableInner9probe_seq17h4ad3f7f3d59e20f9E", scope: !1166, file: !949, line: 2334, type: !1222, scopeLine: 2334, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1224, retainedNodes: !1225)
!1222 = !DISubroutineType(types: !1223)
!1223 = !{!963, !1190, !183}
!1224 = !DISubprogram(name: "probe_seq", linkageName: "_ZN9hashbrown3raw13RawTableInner9probe_seq17h4ad3f7f3d59e20f9E", scope: !1166, file: !949, line: 2334, type: !1222, scopeLine: 2334, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1225 = !{!1226, !1227}
!1226 = !DILocalVariable(name: "self", arg: 1, scope: !1221, file: !949, line: 2334, type: !1190)
!1227 = !DILocalVariable(name: "hash", arg: 2, scope: !1221, file: !949, line: 2334, type: !183)
!1228 = !DILocation(line: 2334, column: 18, scope: !1221)
!1229 = !DILocation(line: 2334, column: 25, scope: !1221)
!1230 = !DILocation(line: 2338, column: 18, scope: !1221)
!1231 = !DILocation(line: 2338, column: 29, scope: !1221)
!1232 = !DILocation(line: 2341, column: 6, scope: !1221)
!1233 = distinct !DISubprogram(name: "new", linkageName: "_ZN9hashbrown3raw16RawIterHashInner3new17h27f9036dc045e3b3E", scope: !954, file: !949, line: 4081, type: !1234, scopeLine: 4081, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1236, retainedNodes: !1237)
!1234 = !DISubroutineType(types: !1235)
!1235 = !{!954, !1190, !183}
!1236 = !DISubprogram(name: "new", linkageName: "_ZN9hashbrown3raw16RawIterHashInner3new17h27f9036dc045e3b3E", scope: !954, file: !949, line: 4081, type: !1234, scopeLine: 4081, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1237 = !{!1238, !1239, !1240, !1242, !1244, !1246}
!1238 = !DILocalVariable(name: "table", arg: 1, scope: !1233, file: !949, line: 4081, type: !1190)
!1239 = !DILocalVariable(name: "hash", arg: 2, scope: !1233, file: !949, line: 4081, type: !183)
!1240 = !DILocalVariable(name: "tag_hash", scope: !1241, file: !949, line: 4082, type: !445, align: 8)
!1241 = distinct !DILexicalBlock(scope: !1233, file: !949, line: 4082, column: 9)
!1242 = !DILocalVariable(name: "probe_seq", scope: !1243, file: !949, line: 4083, type: !963, align: 32)
!1243 = distinct !DILexicalBlock(scope: !1241, file: !949, line: 4083, column: 9)
!1244 = !DILocalVariable(name: "group", scope: !1245, file: !949, line: 4084, type: !968, align: 64)
!1245 = distinct !DILexicalBlock(scope: !1243, file: !949, line: 4084, column: 9)
!1246 = !DILocalVariable(name: "bitmask", scope: !1247, file: !949, line: 4085, type: !974, align: 64)
!1247 = distinct !DILexicalBlock(scope: !1245, file: !949, line: 4085, column: 9)
!1248 = !DILocation(line: 4081, column: 19, scope: !1233)
!1249 = !DILocation(line: 4081, column: 42, scope: !1233)
!1250 = !DILocation(line: 4082, column: 24, scope: !1233)
!1251 = !DILocation(line: 4082, column: 13, scope: !1241)
!1252 = !DILocation(line: 4083, column: 31, scope: !1241)
!1253 = !DILocation(line: 4083, column: 13, scope: !1243)
!1254 = !DILocation(line: 4084, column: 39, scope: !1243)
!1255 = !DILocation(line: 4084, column: 21, scope: !1243)
!1256 = !DILocation(line: 4084, column: 13, scope: !1245)
!1257 = !DILocation(line: 4085, column: 29, scope: !1245)
!1258 = !DILocation(line: 4085, column: 49, scope: !1245)
!1259 = !DILocation(line: 4085, column: 13, scope: !1247)
!1260 = !DILocation(line: 4088, column: 26, scope: !1247)
!1261 = !DILocation(line: 4089, column: 19, scope: !1247)
!1262 = !DILocation(line: 4087, column: 9, scope: !1247)
!1263 = !DILocation(line: 4095, column: 6, scope: !1233)
!1264 = distinct !DISubprogram(name: "h1", linkageName: "_ZN9hashbrown3raw2h117hb6013fe87d518607E", scope: !19, file: !949, line: 61, type: !1265, scopeLine: 61, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !1267)
!1265 = !DISubroutineType(types: !1266)
!1266 = !{!9, !183}
!1267 = !{!1268}
!1268 = !DILocalVariable(name: "hash", arg: 1, scope: !1264, file: !949, line: 61, type: !183)
!1269 = !DILocation(line: 61, column: 7, scope: !1264)
!1270 = !DILocation(line: 63, column: 5, scope: !1264)
!1271 = !DILocation(line: 64, column: 2, scope: !1264)
!1272 = distinct !DISubprogram(name: "move_next", linkageName: "_ZN9hashbrown3raw8ProbeSeq9move_next17h1f9f099143cb881bE", scope: !963, file: !949, line: 83, type: !1273, scopeLine: 83, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1276, retainedNodes: !1277)
!1273 = !DISubroutineType(types: !1274)
!1274 = !{null, !1275, !9}
!1275 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut hashbrown::raw::ProbeSeq", baseType: !963, size: 32, align: 32, dwarfAddressSpace: 0)
!1276 = !DISubprogram(name: "move_next", linkageName: "_ZN9hashbrown3raw8ProbeSeq9move_next17h1f9f099143cb881bE", scope: !963, file: !949, line: 83, type: !1273, scopeLine: 83, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1277 = !{!1278, !1279}
!1278 = !DILocalVariable(name: "self", arg: 1, scope: !1272, file: !949, line: 83, type: !1275)
!1279 = !DILocalVariable(name: "bucket_mask", arg: 2, scope: !1272, file: !949, line: 83, type: !9)
!1280 = !DILocation(line: 83, column: 18, scope: !1272)
!1281 = !DILocation(line: 83, column: 29, scope: !1272)
!1282 = !DILocation(line: 86, column: 13, scope: !1272)
!1283 = !DILocation(line: 85, column: 9, scope: !1272)
!1284 = !DILocation(line: 90, column: 9, scope: !1272)
!1285 = !DILocation(line: 91, column: 21, scope: !1272)
!1286 = !DILocation(line: 91, column: 9, scope: !1272)
!1287 = !DILocation(line: 92, column: 9, scope: !1272)
!1288 = !DILocation(line: 93, column: 6, scope: !1272)
!1289 = distinct !DISubprogram(name: "is_special", linkageName: "_ZN9hashbrown7control3tag3Tag10is_special17h87bbdebc7f6d281eE", scope: !445, file: !926, line: 22, type: !1290, scopeLine: 22, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1292, retainedNodes: !1293)
!1290 = !DISubroutineType(types: !1291)
!1291 = !{!153, !445}
!1292 = !DISubprogram(name: "is_special", linkageName: "_ZN9hashbrown7control3tag3Tag10is_special17h87bbdebc7f6d281eE", scope: !445, file: !926, line: 22, type: !1290, scopeLine: 22, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1293 = !{!1294}
!1294 = !DILocalVariable(name: "self", arg: 1, scope: !1289, file: !926, line: 22, type: !445)
!1295 = !DILocation(line: 22, column: 36, scope: !1289)
!1296 = !DILocation(line: 23, column: 9, scope: !1289)
!1297 = !DILocation(line: 24, column: 6, scope: !1289)
!1298 = distinct !DISubprogram(name: "special_is_empty", linkageName: "_ZN9hashbrown7control3tag3Tag16special_is_empty17haace7411fda682c6E", scope: !445, file: !926, line: 28, type: !1290, scopeLine: 28, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1299, retainedNodes: !1300)
!1299 = !DISubprogram(name: "special_is_empty", linkageName: "_ZN9hashbrown7control3tag3Tag16special_is_empty17haace7411fda682c6E", scope: !445, file: !926, line: 28, type: !1290, scopeLine: 28, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1300 = !{!1301}
!1301 = !DILocalVariable(name: "self", arg: 1, scope: !1298, file: !926, line: 28, type: !445)
!1302 = !DILocation(line: 28, column: 42, scope: !1298)
!1303 = !DILocation(line: 29, column: 28, scope: !1298)
!1304 = !DILocation(line: 29, column: 23, scope: !1298)
!1305 = !DILocation(line: 29, column: 9, scope: !1298)
!1306 = !DILocation(line: 30, column: 9, scope: !1298)
!1307 = !DILocation(line: 31, column: 6, scope: !1298)
!1308 = distinct !DISubprogram(name: "full", linkageName: "_ZN9hashbrown7control3tag3Tag4full17hae36b61f1b1ec9c0E", scope: !445, file: !926, line: 36, type: !1309, scopeLine: 36, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1311, retainedNodes: !1312)
!1309 = !DISubroutineType(types: !1310)
!1310 = !{!445, !183}
!1311 = !DISubprogram(name: "full", linkageName: "_ZN9hashbrown7control3tag3Tag4full17hae36b61f1b1ec9c0E", scope: !445, file: !926, line: 36, type: !1309, scopeLine: 36, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1312 = !{!1313, !1314}
!1313 = !DILocalVariable(name: "hash", arg: 1, scope: !1308, file: !926, line: 36, type: !183)
!1314 = !DILocalVariable(name: "top7", scope: !1315, file: !926, line: 48, type: !183, align: 64)
!1315 = distinct !DILexicalBlock(scope: !1308, file: !926, line: 48, column: 9)
!1316 = !DILocation(line: 36, column: 30, scope: !1308)
!1317 = !DILocation(line: 48, column: 29, scope: !1308)
!1318 = !DILocation(line: 48, column: 28, scope: !1308)
!1319 = !DILocation(line: 48, column: 20, scope: !1308)
!1320 = !DILocation(line: 48, column: 13, scope: !1315)
!1321 = !DILocation(line: 49, column: 13, scope: !1315)
!1322 = !DILocation(line: 50, column: 6, scope: !1308)
!1323 = distinct !DISubprogram(name: "match_empty", linkageName: "_ZN9hashbrown7control5group7generic5Group11match_empty17h20c425d3959bfde7E", scope: !968, file: !1324, line: 117, type: !1325, scopeLine: 117, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1327, retainedNodes: !1328)
!1324 = !DIFile(filename: "src/control/group/generic.rs", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.5", checksumkind: CSK_MD5, checksum: "d5e7c5fd768a592ce589c0c27339fe9d")
!1325 = !DISubroutineType(types: !1326)
!1326 = !{!978, !968}
!1327 = !DISubprogram(name: "match_empty", linkageName: "_ZN9hashbrown7control5group7generic5Group11match_empty17h20c425d3959bfde7E", scope: !968, file: !1324, line: 117, type: !1325, scopeLine: 117, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1328 = !{!1329}
!1329 = !DILocalVariable(name: "self", arg: 1, scope: !1323, file: !1324, line: 117, type: !968)
!1330 = !DILocation(line: 117, column: 31, scope: !1323)
!1331 = !DILocation(line: 121, column: 27, scope: !1323)
!1332 = !DILocation(line: 121, column: 18, scope: !1323)
!1333 = !DILocation(line: 121, column: 43, scope: !1323)
!1334 = !DILocation(line: 121, column: 17, scope: !1323)
!1335 = !DILocalVariable(name: "self", arg: 1, scope: !1336, file: !179, line: 606, type: !183)
!1336 = distinct !DISubprogram(name: "to_le", linkageName: "_ZN4core3num21_$LT$impl$u20$u64$GT$5to_le17h3e0d954071b5bbc7E", scope: !180, file: !179, line: 606, type: !1337, scopeLine: 606, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !1339)
!1337 = !DISubroutineType(types: !1338)
!1338 = !{!183, !183}
!1339 = !{!1335}
!1340 = !DILocation(line: 606, column: 28, scope: !1336, inlinedAt: !1341)
!1341 = distinct !DILocation(line: 121, column: 65, scope: !1323)
!1342 = !DILocation(line: 122, column: 6, scope: !1323)
!1343 = distinct !DISubprogram(name: "load", linkageName: "_ZN9hashbrown7control5group7generic5Group4load17ha6fcdb859ff39a02E", scope: !968, file: !1324, line: 74, type: !1344, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1347, retainedNodes: !1348)
!1344 = !DISubroutineType(types: !1345)
!1345 = !{!968, !1346}
!1346 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const hashbrown::control::tag::Tag", baseType: !445, size: 32, align: 32, dwarfAddressSpace: 0)
!1347 = !DISubprogram(name: "load", linkageName: "_ZN9hashbrown7control5group7generic5Group4load17ha6fcdb859ff39a02E", scope: !968, file: !1324, line: 74, type: !1344, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1348 = !{!1349}
!1349 = !DILocalVariable(name: "ptr", arg: 1, scope: !1343, file: !1324, line: 74, type: !1346)
!1350 = !DILocation(line: 74, column: 31, scope: !1343)
!1351 = !DILocalVariable(name: "self", arg: 1, scope: !1352, file: !393, line: 48, type: !1346)
!1352 = distinct !DISubprogram(name: "cast<hashbrown::control::tag::Tag, u64>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4cast17he7df39e7173d82b8E", scope: !394, file: !393, line: 48, type: !1353, scopeLine: 48, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !1356, retainedNodes: !1355)
!1353 = !DISubroutineType(types: !1354)
!1354 = !{!307, !1346}
!1355 = !{!1351}
!1356 = !{!456, !402}
!1357 = !DILocation(line: 48, column: 26, scope: !1352, inlinedAt: !1358)
!1358 = distinct !DILocation(line: 75, column: 39, scope: !1343)
!1359 = !DILocation(line: 75, column: 15, scope: !1343)
!1360 = !DILocation(line: 76, column: 6, scope: !1343)
!1361 = distinct !DISubprogram(name: "match_tag", linkageName: "_ZN9hashbrown7control5group7generic5Group9match_tag17h7d9d885232d15550E", scope: !968, file: !1324, line: 107, type: !1362, scopeLine: 107, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1364, retainedNodes: !1365)
!1362 = !DISubroutineType(types: !1363)
!1363 = !{!978, !968, !445}
!1364 = !DISubprogram(name: "match_tag", linkageName: "_ZN9hashbrown7control5group7generic5Group9match_tag17h7d9d885232d15550E", scope: !968, file: !1324, line: 107, type: !1362, scopeLine: 107, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1365 = !{!1366, !1367, !1368}
!1366 = !DILocalVariable(name: "self", arg: 1, scope: !1361, file: !1324, line: 107, type: !968)
!1367 = !DILocalVariable(name: "tag", arg: 2, scope: !1361, file: !1324, line: 107, type: !445)
!1368 = !DILocalVariable(name: "cmp", scope: !1369, file: !1324, line: 110, type: !183, align: 64)
!1369 = distinct !DILexicalBlock(scope: !1361, file: !1324, line: 110, column: 9)
!1370 = !DILocation(line: 107, column: 29, scope: !1361)
!1371 = !DILocation(line: 107, column: 35, scope: !1361)
!1372 = !DILocation(line: 110, column: 28, scope: !1361)
!1373 = !DILocation(line: 110, column: 19, scope: !1361)
!1374 = !DILocation(line: 110, column: 13, scope: !1369)
!1375 = !DILocation(line: 111, column: 35, scope: !1369)
!1376 = !DILocalVariable(name: "self", arg: 1, scope: !1377, file: !179, line: 2339, type: !183)
!1377 = distinct !DISubprogram(name: "wrapping_sub", linkageName: "_ZN4core3num21_$LT$impl$u20$u64$GT$12wrapping_sub17he22cc0c70fbc2595E", scope: !180, file: !179, line: 2339, type: !1378, scopeLine: 2339, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !1380)
!1378 = !DISubroutineType(types: !1379)
!1379 = !{!183, !183, !183}
!1380 = !{!1376, !1381}
!1381 = !DILocalVariable(name: "rhs", arg: 2, scope: !1377, file: !179, line: 2339, type: !183)
!1382 = !DILocation(line: 2339, column: 35, scope: !1377, inlinedAt: !1383)
!1383 = distinct !DILocation(line: 111, column: 22, scope: !1369)
!1384 = !DILocation(line: 2339, column: 41, scope: !1377, inlinedAt: !1383)
!1385 = !DILocation(line: 2340, column: 13, scope: !1377, inlinedAt: !1383)
!1386 = !DILocation(line: 111, column: 56, scope: !1369)
!1387 = !DILocation(line: 111, column: 18, scope: !1369)
!1388 = !DILocation(line: 111, column: 63, scope: !1369)
!1389 = !DILocation(line: 111, column: 17, scope: !1369)
!1390 = !DILocation(line: 606, column: 28, scope: !1336, inlinedAt: !1391)
!1391 = distinct !DILocation(line: 111, column: 85, scope: !1369)
!1392 = !DILocation(line: 112, column: 6, scope: !1361)
!1393 = distinct !DISubprogram(name: "repeat", linkageName: "_ZN9hashbrown7control5group7generic6repeat17hb738815a69dabe76E", scope: !969, file: !1324, line: 33, type: !1394, scopeLine: 33, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, retainedNodes: !1396)
!1394 = !DISubroutineType(types: !1395)
!1395 = !{!183, !445}
!1396 = !{!1397}
!1397 = !DILocalVariable(name: "tag", arg: 1, scope: !1393, file: !1324, line: 33, type: !445)
!1398 = !DILocation(line: 33, column: 11, scope: !1393)
!1399 = !DILocation(line: 34, column: 30, scope: !1393)
!1400 = !DILocation(line: 34, column: 5, scope: !1393)
!1401 = !DILocation(line: 35, column: 2, scope: !1393)
!1402 = distinct !DISubprogram(name: "any_bit_set", linkageName: "_ZN9hashbrown7control7bitmask7BitMask11any_bit_set17hd8ea81ce0500326fE", scope: !978, file: !1071, line: 43, type: !1403, scopeLine: 43, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1405, retainedNodes: !1406)
!1403 = !DISubroutineType(types: !1404)
!1404 = !{!153, !978}
!1405 = !DISubprogram(name: "any_bit_set", linkageName: "_ZN9hashbrown7control7bitmask7BitMask11any_bit_set17hd8ea81ce0500326fE", scope: !978, file: !1071, line: 43, type: !1403, scopeLine: 43, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1406 = !{!1407}
!1407 = !DILocalVariable(name: "self", arg: 1, scope: !1402, file: !1071, line: 43, type: !978)
!1408 = !DILocation(line: 43, column: 31, scope: !1402)
!1409 = !DILocation(line: 44, column: 9, scope: !1402)
!1410 = !DILocation(line: 45, column: 6, scope: !1402)
!1411 = distinct !DISubprogram(name: "lowest_set_bit", linkageName: "_ZN9hashbrown7control7bitmask7BitMask14lowest_set_bit17heba4e93fefcfcf86E", scope: !978, file: !1071, line: 49, type: !1412, scopeLine: 49, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1414, retainedNodes: !1415)
!1412 = !DISubroutineType(types: !1413)
!1413 = !{!196, !978}
!1414 = !DISubprogram(name: "lowest_set_bit", linkageName: "_ZN9hashbrown7control7bitmask7BitMask14lowest_set_bit17heba4e93fefcfcf86E", scope: !978, file: !1071, line: 49, type: !1412, scopeLine: 49, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1415 = !{!1416, !1417}
!1416 = !DILocalVariable(name: "self", arg: 1, scope: !1411, file: !1071, line: 49, type: !978)
!1417 = !DILocalVariable(name: "nonzero", scope: !1418, file: !1071, line: 50, type: !284, align: 64)
!1418 = distinct !DILexicalBlock(scope: !1411, file: !1071, line: 50, column: 64)
!1419 = !DILocation(line: 49, column: 34, scope: !1411)
!1420 = !DILocation(line: 50, column: 32, scope: !1418)
!1421 = !DILocation(line: 50, column: 16, scope: !1418)
!1422 = !DILocation(line: 50, column: 21, scope: !1418)
!1423 = !DILocation(line: 51, column: 18, scope: !1418)
!1424 = !DILocation(line: 51, column: 13, scope: !1418)
!1425 = !DILocation(line: 50, column: 9, scope: !1411)
!1426 = !DILocation(line: 53, column: 13, scope: !1411)
!1427 = !DILocation(line: 55, column: 6, scope: !1411)
!1428 = !DILocation(line: 49, column: 5, scope: !1411)
!1429 = distinct !DISubprogram(name: "remove_lowest_bit", linkageName: "_ZN9hashbrown7control7bitmask7BitMask17remove_lowest_bit17h74eb0823a1635631E", scope: !978, file: !1071, line: 37, type: !1430, scopeLine: 37, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1432, retainedNodes: !1433)
!1430 = !DISubroutineType(types: !1431)
!1431 = !{!978, !978}
!1432 = !DISubprogram(name: "remove_lowest_bit", linkageName: "_ZN9hashbrown7control7bitmask7BitMask17remove_lowest_bit17h74eb0823a1635631E", scope: !978, file: !1071, line: 37, type: !1430, scopeLine: 37, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1433 = !{!1434}
!1434 = !DILocalVariable(name: "self", arg: 1, scope: !1429, file: !1071, line: 37, type: !978)
!1435 = !DILocation(line: 37, column: 26, scope: !1429)
!1436 = !DILocation(line: 38, column: 26, scope: !1429)
!1437 = !DILocation(line: 38, column: 17, scope: !1429)
!1438 = !DILocation(line: 39, column: 6, scope: !1429)
!1439 = distinct !DISubprogram(name: "nonzero_trailing_zeros", linkageName: "_ZN9hashbrown7control7bitmask7BitMask22nonzero_trailing_zeros17h6e7d56c1b164da0bE", scope: !978, file: !1071, line: 74, type: !1440, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !15, templateParams: !13, declaration: !1442, retainedNodes: !1443)
!1440 = !DISubroutineType(types: !1441)
!1441 = !{!9, !284}
!1442 = !DISubprogram(name: "nonzero_trailing_zeros", linkageName: "_ZN9hashbrown7control7bitmask7BitMask22nonzero_trailing_zeros17h6e7d56c1b164da0bE", scope: !978, file: !1071, line: 74, type: !1440, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!1443 = !{!1444, !1445}
!1444 = !DILocalVariable(name: "nonzero", arg: 1, scope: !1439, file: !1071, line: 74, type: !284)
!1445 = !DILocalVariable(name: "swapped", scope: !1446, file: !1071, line: 77, type: !284, align: 64)
!1446 = distinct !DILexicalBlock(scope: !1439, file: !1071, line: 77, column: 13)
!1447 = !DILocation(line: 74, column: 31, scope: !1439)
!1448 = !DILocation(line: 80, column: 21, scope: !1439)
!1449 = !DILocation(line: 80, column: 13, scope: !1439)
!1450 = !DILocation(line: 82, column: 6, scope: !1439)

import {
  core__fmt__rt__Argument__new_display,
  core__fmt__rt__Arguments__new_v1,
  core__fmt__rt__Arguments__new_const,
  std__io__print,
  std__iter__IntoIterator__into_iter,
  std__iter__Iterator__next,
  discriminant,
  switchInt,
  alloc__alloc__exchange_malloc,
  std__slice__impl__T__into_vec,
  Var,
  Enum,
  std__ptr__Unique,
  std__ptr__NonNull,
  AllocatedMemory,
  Vec,
  Array,
  Tuple,
  _add,
  _sub,
  _mul,
  _div,
  _rem,
  _xor,
  _and,
  _or,
  _shl,
  _shr,
  _eq,
  _lt,
  _le,
  _ne,
  _ge,
  _gt,
  _not,
  LocalVar,
  StackMemory,
  RefVar,
  NoRefVar,
  MutRefVar,
  memory,
  _ref,
  Int8,
  Int16,
  Int32,
  Int64,
  Uint8,
  Uint16,
  Uint32,
  Uint64,
} from "./impl.ts";

// ====
function main() {
const _0 = new NoRefVar(sizeof(0));
const _1 = new ClosureVar(main____closure__0___ty_i8___fn_ptr___ty_tuple);const _2 = new NoRefVar(sizeof(0));
const _3 = new RefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new MutRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(0));
const _8 = new MutRefVar(sizeof(4));
const _9 = new NoRefVar(sizeof(4));
const _10 = new NoRefVar(sizeof(0));
const _11 = new RefVar(sizeof(4));
function bb0() {
_3.assign(_ref(_1));
_2=std__ops__Fn__call(_3, new Tuple([]));
return bb1();
}
function bb1() {
_4.assign(new Int32(5));
_6.assign(_ref(_4));
_5.assign(main____closure__1___ty_i16___fn_ptr___ty_tuple__ref__ty_i32);
_8.assign(_ref(_5));
_7=std__ops__FnMut__call_mutmain____closure__1___ty_i16___fn_ptr___ty_tuple__ref__ty_i32___ty_tuple(_8, new Tuple([]));
return bb2();
}
function bb2() {
_9.assign(main____closure__2___ty_i8___fn_ptr___ty_tuple__ty_i32);
_11.assign(_ref(_9));
_10=std__ops__Fn__call(_11, new Tuple([]));
return bb3();
}
function bb3() {
return;
}
bb0();
return _0;
}
function main____closure__0___ty_i8___fn_ptr___ty_tuple() {
const _0 = new NoRefVar(sizeof(0));
const _1 = new RefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(0));
const _3 = new NoRefVar(sizeof(24));
const _4 = new RefVar(sizeof(4));
function bb0() {
_4.assign(main____closure__0___ty_i8___fn_ptr___ty_tuple__promoted_0);
_3=core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize(_4);
return bb1();
}
function bb1() {
_2=std__io___print(_3);
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
const main____closure__0___ty_i8___fn_ptr___ty_tuple__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(8));
function bb0() {
_1.assign(new Array([new Uint8Array([104, 101, 108, 108, 111, 10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function main____closure__2___ty_i8___fn_ptr___ty_tuple__ty_i32() {
const _0 = new NoRefVar(sizeof(0));
const _1 = new RefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(0));
const _5 = new NoRefVar(sizeof(24));
const _6 = new NoRefVar(sizeof(4));
const _7 = new RefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(8));
const _9 = new NoRefVar(sizeof(8));
const _10 = new RefVar(sizeof(4));
const _11 = new RefVar(sizeof(4));
const _12 = new RefVar(sizeof(4));
function bb0() {
_2.assign(_1.deref().field(0));
_3.assign(_add(_2, new Int32(1)));
if (_eq(_3.field(1), false)) {
return bb1();
} else {
throw new Error('assert failed: Overflow(Add, copy _2, const 1_i32)');
}
}
function bb1() {
_2.assign(_3.field(0));
_7.assign(_ref(_2));
_6.assign(new Tuple([_7]));
_12.assign(_6.field(0));
_9=core__fmt__rt__Argument___________new_display_ty_i32(_12);
return bb2();
}
function bb2() {
_8.assign(new Array([_9]));
_10.assign(main____closure__2___ty_i8___fn_ptr___ty_tuple__ty_i32__promoted_0);
_11.assign(_ref(_8));
_5=core__fmt__rt____impl__std__fmt__Arguments____a______new_v12_usize__1_usize(_10, _11);
return bb3();
}
function bb3() {
_4=std__io___print(_5);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
const main____closure__2___ty_i8___fn_ptr___ty_tuple__ty_i32__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(16));
function bb0() {
_1.assign(new Array([new Uint8Array([]), new Uint8Array([10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_v12_usize__1_usize() {
const _0 = new NoRefVar(sizeof(24));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(4));
const _3 = new RefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(8));
const _5 = new RefVar(sizeof(8));
function bb0() {
_3.assign(_1);
_4.assign(new Enum(undefined, 0));
_5.assign(_2);
_0.assign([_3, _4, _5]);
return;
}
bb0();
return _0;
}
function main____closure__1___ty_i16___fn_ptr___ty_tuple__ref__ty_i32() {
const _0 = new NoRefVar(sizeof(0));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(0));
const _4 = new NoRefVar(sizeof(24));
const _5 = new NoRefVar(sizeof(4));
const _6 = new RefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(8));
const _8 = new NoRefVar(sizeof(8));
const _9 = new RefVar(sizeof(4));
const _10 = new RefVar(sizeof(4));
const _11 = new MutRefVar(sizeof(4));
const _12 = new MutRefVar(sizeof(4));
const _13 = new MutRefVar(sizeof(4));
const _14 = new MutRefVar(sizeof(4));
const _15 = new RefVar(sizeof(4));
function bb0() {
_11.assign(_1.deref().field(0));
_2.assign(_add(_11.deref(), new Int32(1)));
_12.assign(_1.deref().field(0));
if (_eq(_2.field(1), false)) {
return bb1();
} else {
throw new Error('assert failed: Overflow(Add, copy (*_12), const 1_i32)');
}
}
function bb1() {
_13.assign(_1.deref().field(0));
_13.deref().assign(_2.field(0));
_14.assign(_1.deref().field(0));
_6.assign(_ref(_14.deref()));
_5.assign(new Tuple([_6]));
_15.assign(_5.field(0));
_8=core__fmt__rt__Argument___________new_display_ty_i32(_15);
return bb2();
}
function bb2() {
_7.assign(new Array([_8]));
_9.assign(main____closure__1___ty_i16___fn_ptr___ty_tuple__ref__ty_i32__promoted_0);
_10.assign(_ref(_7));
_4=core__fmt__rt____impl__std__fmt__Arguments____a______new_v12_usize__1_usize(_9, _10);
return bb3();
}
function bb3() {
_3=std__io___print(_4);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
const main____closure__1___ty_i16___fn_ptr___ty_tuple__ref__ty_i32__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(16));
function bb0() {
_1.assign(new Array([new Uint8Array([]), new Uint8Array([10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_v12_usize__1_usize() {
const _0 = new NoRefVar(sizeof(24));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(4));
const _3 = new RefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(8));
const _5 = new RefVar(sizeof(8));
function bb0() {
_3.assign(_1);
_4.assign(new Enum(undefined, 0));
_5.assign(_2);
_0.assign([_3, _4, _5]);
return;
}
bb0();
return _0;
}
function core__fmt__rt__Argument___________new_display_ty_i32() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new RefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
function bb0() {
_4=std__ptr__NonNull____T____from_ref_ty_i32(_1);
return bb1();
}
function bb1() {
_3=std__ptr__NonNull____T____cast_ty_i32___ty_tuple(_4);
return bb2();
}
function bb2() {
_6.assign(std__fmt__Display__fmt_ty_i32);
_5.assign(_6);
_2.assign([_3, _5, std__marker__PhantomData_ref__ty_tuple]);
_0.assign([_2]);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____cast_ty_i32___ty_tuple() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
function bb0() {
_4=std__ptr__NonNull____T____as_ptr_ty_i32(_1);
return bb1();
}
function bb1() {
_3.assign(_4);
_2.assign(_3);
_0.assign([_2]);
return;
}
bb0();
return _0;
}
function core__fmt__num__imp____impl__std__fmt__Display__for__i32____fmt() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new RefVar(sizeof(4));
const _2 = new MutRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(10));
const _4 = new NoRefVar(sizeof(1));
const _5 = new NoRefVar(sizeof(1));
const _6 = new NoRefVar(sizeof(4));
const _7 = new RefVar(sizeof(8));
const _8 = new RefVar(sizeof(8));
const _9 = new NoRefVar(sizeof(4));
const _10 = new NoRefVar(sizeof(4));
const _11 = new MutRefVar(sizeof(8));
const _12 = new MutRefVar(sizeof(4));
function bb0() {
_4=std__mem__MaybeUninit____T____uninit_ty_u8();
return bb1();
}
function bb1() {
_3.assign(_repeat(_4, 10));
_6.assign(_1.deref());
_5.assign(_ge(_6, new Int32(0)));
_7.assign(new Uint8Array([]));
_10.assign(_1.deref());
_9=core__num____impl__i32____unsigned_abs(_10);
return bb2();
}
function bb2() {
_12.assign(_ref(_3));
_11.assign(_12);
_8=core__fmt__num__imp____impl__u32_____fmt(_9, _11);
return bb3();
}
function bb3() {
_0=std__fmt__Formatter______a____pad_integral(_2, _5, _7, _8);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function core__fmt__num__imp____impl__u32_____fmt() {
const _0 = new RefVar(sizeof(8));
const _1 = new NoRefVar(sizeof(4));
const _2 = new MutRefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(4));
const _4 = new RefVar(sizeof(8));
function bb0() {
_3=core__fmt__num__imp____impl__u32_____fmt_inner(_1, _2);
return bb1();
}
function bb1() {
_4.assign(_ref(_2.deref()));
_0=core__fmt__num__slice_buffer_to_str(_4, _3);
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
function std__fmt__Formatter______a____pad_integral() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(1));
const _3 = new RefVar(sizeof(8));
const _4 = new RefVar(sizeof(8));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(8));
const _9 = new NoRefVar(sizeof(1));
const _10 = new RefVar(sizeof(4));
const _11 = new NoRefVar(sizeof(4));
const _12 = new NoRefVar(sizeof(8));
const _13 = new NoRefVar(sizeof(8));
const _14 = new NoRefVar(sizeof(1));
const _15 = new RefVar(sizeof(4));
const _16 = new NoRefVar(sizeof(4));
const _17 = new NoRefVar(sizeof(8));
const _18 = new NoRefVar(sizeof(8));
const _19 = new NoRefVar(sizeof(2));
const _20 = new NoRefVar(sizeof(1));
const _21 = new NoRefVar(sizeof(4));
const _22 = new NoRefVar(sizeof(4));
const _23 = new NoRefVar(sizeof(1));
const _24 = new NoRefVar(sizeof(1));
const _25 = new NoRefVar(sizeof(4));
const _26 = new NoRefVar(sizeof(8));
const _27 = new NoRefVar(sizeof(4));
const _28 = new NoRefVar(sizeof(1));
const _29 = new RefVar(sizeof(4));
const _30 = new NoRefVar(sizeof(8));
const _31 = new MutRefVar(sizeof(4));
const _32 = new MutRefVar(sizeof(4));
const _33 = new MutRefVar(sizeof(4));
const _34 = new NoRefVar(sizeof(1));
const _35 = new NoRefVar(sizeof(1));
const _36 = new NoRefVar(sizeof(1));
const _37 = new NoRefVar(sizeof(1));
const _38 = new NoRefVar(sizeof(4));
const _39 = new NoRefVar(sizeof(8));
const _40 = new NoRefVar(sizeof(4));
const _41 = new NoRefVar(sizeof(8));
const _42 = new NoRefVar(sizeof(8));
const _43 = new NoRefVar(sizeof(2));
const _44 = new NoRefVar(sizeof(2));
const _45 = new NoRefVar(sizeof(4));
const _46 = new NoRefVar(sizeof(4));
const _47 = new NoRefVar(sizeof(1));
const _48 = new NoRefVar(sizeof(4));
const _49 = new NoRefVar(sizeof(8));
const _50 = new NoRefVar(sizeof(1));
const _51 = new NoRefVar(sizeof(1));
const _52 = new NoRefVar(sizeof(4));
const _53 = new NoRefVar(sizeof(1));
const _54 = new NoRefVar(sizeof(1));
const _55 = new NoRefVar(sizeof(4));
const _56 = new NoRefVar(sizeof(8));
const _57 = new NoRefVar(sizeof(8));
const _58 = new NoRefVar(sizeof(2));
const _59 = new NoRefVar(sizeof(2));
const _60 = new NoRefVar(sizeof(4));
const _61 = new NoRefVar(sizeof(4));
const _62 = new NoRefVar(sizeof(1));
const _63 = new NoRefVar(sizeof(4));
const _64 = new NoRefVar(sizeof(8));
const _65 = new NoRefVar(sizeof(1));
const _66 = new NoRefVar(sizeof(1));
const _67 = new NoRefVar(sizeof(4));
const _68 = new NoRefVar(sizeof(8));
const _69 = new NoRefVar(sizeof(4));
const _70 = new NoRefVar(sizeof(1));
const _71 = new NoRefVar(sizeof(1));
const _72 = new NoRefVar(sizeof(4));
const _73 = new MutRefVar(sizeof(8));
const _74 = new MutRefVar(sizeof(8));
const _75 = new MutRefVar(sizeof(8));
function bb0() {
_5=core__str____impl__str____len(_4);
return bb1();
}
function bb1() {
_6.assign(new Enum(undefined, 0));
switch (switchInt(_2)) {
case 0:return bb2();
default: return bb4();
}
}
function bb2() {
_7.assign(new Enum(new Char('-'), 1));
_6.assign(_7);
_8.assign(_add(_5, new Uint32(1)));
if (_eq(_8.field(1), false)) {
return bb3();
} else {
throw new Error('assert failed: Overflow(Add, copy _5, const 1_usize)');
}
}
function bb3() {
_5.assign(_8.field(0));
return bb8();
}
function bb4() {
_10.assign(_ref(_1.deref()));
_9=std__fmt__Formatter______a____sign_plus(_10);
return bb5();
}
function bb5() {
switch (switchInt(_9)) {
case 0:return bb8();
default: return bb6();
}
}
function bb6() {
_11.assign(new Enum(new Char('+'), 1));
_6.assign(_11);
_12.assign(_add(_5, new Uint32(1)));
if (_eq(_12.field(1), false)) {
return bb7();
} else {
throw new Error('assert failed: Overflow(Add, copy _5, const 1_usize)');
}
}
function bb7() {
_5.assign(_12.field(0));
return bb8();
}
function bb8() {
_15.assign(_ref(_1.deref()));
_14=std__fmt__Formatter______a____alternate(_15);
return bb9();
}
function bb9() {
switch (switchInt(_14)) {
case 0:return bb14();
default: return bb10();
}
}
function bb10() {
_17=core__str____impl__str____chars(_3);
return bb11();
}
function bb11() {
_16=std__iter__Iterator__countstd__str__Chars(_17);
return bb12();
}
function bb12() {
_18.assign(_add(_5, _16));
if (_eq(_18.field(1), false)) {
return bb13();
} else {
throw new Error('assert failed: Overflow(Add, copy _5, move _16)');
}
}
function bb13() {
_5.assign(_18.field(0));
_13.assign(new Enum(_3, 1));
return bb15();
}
function bb14() {
_13.assign(new Enum(undefined, 0));
return bb15();
}
function bb15() {
_19.assign(_1.deref().field(0).field(1));
_21.assign(_5);
_22=std__convert__From__from_ty_usize___ty_u16(_19);
return bb16();
}
function bb16() {
_20.assign(_ge(_21, _22));
switch (switchInt(_20)) {
case 0:return bb23();
default: return bb17();
}
}
function bb17() {
_25.assign(_6);
_26.assign(_13);
_24=std__fmt__Formatter______a____pad_integral__write_prefix(_1, _25, _26);
return bb18();
}
function bb18() {
_23=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_24);
return bb19();
}
function bb19() {
_27.assign(discriminant(_23));
switch (switchInt(_27)) {
case 0:return bb21();
case 1:return bb22();
default: return bb20();
}
}
function bb20() {
throw new Error('unreachable');
}
function bb21() {
_73.assign(_1.deref().field(1));
_0=_fn_call(_73, _4);
return bb59();
}
function bb22() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb23() {
_29.assign(_ref(_1.deref()));
_28=std__fmt__Formatter______a____sign_aware_zero_pad(_29);
return bb24();
}
function bb24() {
switch (switchInt(_28)) {
case 0:return bb45();
default: return bb25();
}
}
function bb25() {
_30.assign(_1.deref().field(0));
_33.assign(_ref(_1.deref().field(0)));
_32=std__fmt__FormattingOptions__fill(_33, new Char('0'));
return bb26();
}
function bb26() {
_35.assign([]);
_34.assign(new Enum(_35, 1));
_31=std__fmt__FormattingOptions__align(_32, _34);
return bb27();
}
function bb27() {
_38.assign(_6);
_39.assign(_13);
_37=std__fmt__Formatter______a____pad_integral__write_prefix(_1, _38, _39);
return bb28();
}
function bb28() {
_36=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_37);
return bb29();
}
function bb29() {
_40.assign(discriminant(_36));
switch (switchInt(_40)) {
case 0:return bb30();
case 1:return bb31();
default: return bb20();
}
}
function bb30() {
_45.assign(_5);
_44.assign(_45);
_46.assign(_sub(_19, _44));
if (_eq(_46.field(1), false)) {
return bb32();
} else {
throw new Error('assert failed: Overflow(Sub, copy _19, move _44)');
}
}
function bb31() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb32() {
_43.assign(_46.field(0));
_47.assign([]);
_42=std__fmt__Formatter______a____padding(_1, _43, _47);
return bb33();
}
function bb33() {
_41=std__ops__Try__branchstd__result__Resultcore__fmt__PostPadding__std__fmt__Error(_42);
return bb34();
}
function bb34() {
_48.assign(discriminant(_41));
switch (switchInt(_48)) {
case 0:return bb35();
case 1:return bb36();
default: return bb20();
}
}
function bb35() {
_49.assign(_41.downcast(Continue, 0).field(0));
_74.assign(_1.deref().field(1));
_51=_fn_call(_74, _4);
return bb37();
}
function bb36() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb37() {
_50=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_51);
return bb38();
}
function bb38() {
_52.assign(discriminant(_50));
switch (switchInt(_52)) {
case 0:return bb39();
case 1:return bb40();
default: return bb20();
}
}
function bb39() {
_54=core__fmt__PostPadding__write(_49, _1);
return bb41();
}
function bb40() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb41() {
_53=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_54);
return bb42();
}
function bb42() {
_55.assign(discriminant(_53));
switch (switchInt(_55)) {
case 0:return bb43();
case 1:return bb44();
default: return bb20();
}
}
function bb43() {
_1.deref().field(0).assign(_30);
_0.assign([new Tuple([])]);
return bb59();
}
function bb44() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb45() {
_60.assign(_5);
_59.assign(_60);
_61.assign(_sub(_19, _59));
if (_eq(_61.field(1), false)) {
return bb46();
} else {
throw new Error('assert failed: Overflow(Sub, copy _19, move _59)');
}
}
function bb46() {
_58.assign(_61.field(0));
_62.assign([]);
_57=std__fmt__Formatter______a____padding(_1, _58, _62);
return bb47();
}
function bb47() {
_56=std__ops__Try__branchstd__result__Resultcore__fmt__PostPadding__std__fmt__Error(_57);
return bb48();
}
function bb48() {
_63.assign(discriminant(_56));
switch (switchInt(_63)) {
case 0:return bb49();
case 1:return bb50();
default: return bb20();
}
}
function bb49() {
_64.assign(_56.downcast(Continue, 0).field(0));
_67.assign(_6);
_68.assign(_13);
_66=std__fmt__Formatter______a____pad_integral__write_prefix(_1, _67, _68);
return bb51();
}
function bb50() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb51() {
_65=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_66);
return bb52();
}
function bb52() {
_69.assign(discriminant(_65));
switch (switchInt(_69)) {
case 0:return bb53();
case 1:return bb54();
default: return bb20();
}
}
function bb53() {
_75.assign(_1.deref().field(1));
_71=_fn_call(_75, _4);
return bb55();
}
function bb54() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb55() {
_70=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_71);
return bb56();
}
function bb56() {
_72.assign(discriminant(_70));
switch (switchInt(_72)) {
case 0:return bb57();
case 1:return bb58();
default: return bb20();
}
}
function bb57() {
_0=core__fmt__PostPadding__write(_64, _1);
return bb59();
}
function bb58() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb59();
}
function bb59() {
return;
}
bb0();
return _0;
}
function core__fmt__PostPadding__write() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(8));
const _2 = new MutRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(2));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(4));
const _8 = new MutRefVar(sizeof(4));
const _9 = new NoRefVar(sizeof(4));
const _10 = new NoRefVar(sizeof(1));
const _11 = new NoRefVar(sizeof(1));
const _12 = new NoRefVar(sizeof(4));
const _13 = new NoRefVar(sizeof(4));
const _14 = new MutRefVar(sizeof(8));
function bb0() {
_5.assign(_1.field(1));
_4.assign([new Uint16(0), _5]);
_3=std__iter__IntoIterator__into_iterstd__ops__Range_ty_u16(_4);
return bb1();
}
function bb1() {
_6.assign(_3);
return bb2();
}
function bb2() {
_8.assign(_ref(_6));
_7=std__iter__Iterator__nextstd__ops__Range_ty_u16(_8);
return bb3();
}
function bb3() {
_9.assign(discriminant(_7));
switch (switchInt(_9)) {
case 0:return bb6();
case 1:return bb5();
default: return bb4();
}
}
function bb4() {
throw new Error('unreachable');
}
function bb5() {
_14.assign(_2.deref().field(1));
_12.assign(_1.field(0));
_11=_fn_call(_14, _12);
return bb7();
}
function bb6() {
_0.assign([new Tuple([])]);
return bb10();
}
function bb7() {
_10=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_11);
return bb8();
}
function bb8() {
_13.assign(discriminant(_10));
switch (switchInt(_13)) {
case 0:return bb2();
case 1:return bb9();
default: return bb4();
}
}
function bb9() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb10();
}
function bb10() {
return;
}
bb0();
return _0;
}
function std__mem__MaybeUninit____T____uninit_ty_u8() {
const _0 = new NoRefVar(sizeof(1));
function bb0() {
_0.assign([new Tuple([])]);
return;
}
bb0();
return _0;
}
function core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize() {
const _0 = new NoRefVar(sizeof(24));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(8));
const _4 = new RefVar(sizeof(8));
const _5 = new RefVar(sizeof(4));
function bb0() {
_2.assign(_1);
_3.assign(new Enum(undefined, 0));
_5.assign(core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0);
_4.assign(_5);
_0.assign([_2, _3, _4]);
return;
}
bb0();
return _0;
}
const core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(0));
function bb0() {
_1.assign(new Array([]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__num____impl__i32____unsigned_abs() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_2=core__num____impl__i32____wrapping_abs(_1);
return bb1();
}
function bb1() {
_0.assign(_2);
return;
}
bb0();
return _0;
}
function core__num____impl__i32____wrapping_abs() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(1));
function bb0() {
_2=core__num____impl__i32____is_negative(_1);
return bb1();
}
function bb1() {
switch (switchInt(_2)) {
case 0:return bb3();
default: return bb2();
}
}
function bb2() {
_0=core__num____impl__i32____wrapping_neg(_1);
return bb4();
}
function bb3() {
_0.assign(_1);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function core__num____impl__i32____is_negative() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_lt(_1, new Int32(0)));
return;
}
bb0();
return _0;
}
function core__num____impl__i32____wrapping_neg() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0=core__num____impl__i32____wrapping_sub(new Int32(0), _1);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function core__num____impl__i32____wrapping_sub() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_sub(_1, _2));
return;
}
bb0();
return _0;
}
function __std__result__Result__T,__E____as__std__ops__Try____branchcore__fmt__PostPadding__std__fmt__Error() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new NoRefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(0));
const _5 = new NoRefVar(sizeof(0));
function bb0() {
_2.assign(discriminant(_1));
switch (switchInt(_2)) {
case 0:return bb3();
case 1:return bb2();
default: return bb1();
}
}
function bb1() {
throw new Error('unreachable');
}
function bb2() {
_4.assign(_1.downcast(Err, 1).field(0));
_5.assign([_4]);
_0.assign([_5]);
return bb4();
}
function bb3() {
_3.assign(_1.downcast(Ok, 0).field(0));
_0.assign([_3]);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function std__convert__num____impl__std__convert__From__u16____for__usize____from() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(2));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function core__fmt__num__slice_buffer_to_str() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
const _3 = new RefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(4));
const _5 = new RefVar(sizeof(8));
function bb0() {
_4.assign([_2]);
_3=core__slice____impl____T______get_uncheckedstd__mem__MaybeUninit_ty_u8__std__ops__RangeFrom_ty_usize(_1, _4);
return bb1();
}
function bb1() {
_5=____std__mem__MaybeUninit__T________assume_init_ref_ty_u8(_3);
return bb2();
}
function bb2() {
_0=std__str__from_utf8_unchecked(_5);
return bb3();
}
function bb3() {
return;
}
bb0();
return _0;
}
function std__str__from_utf8_unchecked() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function __std__result__Result__T,__E____as__std__ops__Try____branch_ty_tuple__std__fmt__Error() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(1));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(0));
const _4 = new NoRefVar(sizeof(0));
const _5 = new NoRefVar(sizeof(0));
function bb0() {
_2.assign(discriminant(_1));
switch (switchInt(_2)) {
case 0:return bb3();
case 1:return bb2();
default: return bb1();
}
}
function bb1() {
throw new Error('unreachable');
}
function bb2() {
_4.assign(_1.downcast(Err, 1).field(0));
_5.assign([_4]);
_0.assign([_5]);
return bb4();
}
function bb3() {
_3.assign(_1.downcast(Ok, 0).field(0));
_0.assign([_3]);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function std__iter__range____impl__std__iter__Iterator__for__std__ops__Range__A______next_ty_u16() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
function bb0() {
_0=std__iter__range__RangeIteratorImpl__spec_nextstd__ops__Range_ty_u16(_1);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function __std__ops__Range__T____as__std__iter__range__RangeIteratorImpl____spec_next_ty_u16() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(1));
const _3 = new RefVar(sizeof(4));
const _4 = new RefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(2));
const _6 = new NoRefVar(sizeof(2));
function bb0() {
_3.assign(_ref(_1.deref().field(0)));
_4.assign(_ref(_1.deref().field(1)));
_2=std__cmp__PartialOrd__lt_ty_u16___ty_u16(_3, _4);
return bb1();
}
function bb1() {
switch (switchInt(_2)) {
case 0:return bb4();
default: return bb2();
}
}
function bb2() {
_5.assign(_1.deref().field(0));
_6=std__iter__Step__forward_unchecked_ty_u16(_5, new Uint32(1));
return bb3();
}
function bb3() {
_1.deref().field(0).assign(_6);
_0.assign(new Enum(_5, 1));
return bb5();
}
function bb4() {
_0.assign(new Enum(undefined, 0));
return bb5();
}
function bb5() {
return;
}
bb0();
return _0;
}
function __std__str__Chars____a____as__std__iter__Iterator____count() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(8));
const _2 = new RefVar(sizeof(8));
const _3 = new RefVar(sizeof(4));
function bb0() {
_3.assign(_ref(_1));
_2=std__str__Chars______a____as_str(_3);
return bb1();
}
function bb1() {
_0=core__str__count__count_chars(_2);
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
function std__str__Chars______a____as_str() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(8));
const _3 = new RefVar(sizeof(4));
function bb0() {
_3.assign(_ref(_1.deref().field(0)));
_2=std__slice__Iter______a,__T____as_slice_ty_u8(_3);
return bb1();
}
function bb1() {
_0=std__str__from_utf8_unchecked(_2);
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
function std__str__from_utf8_unchecked() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function core__str__count__count_chars() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(1));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(8));
const _6 = new RefVar(sizeof(8));
function bb0() {
_3=core__str____impl__str____len(_1);
return bb1();
}
function bb1() {
_5.assign(_mul(core__str__count__USIZE_SIZE, core__str__count__UNROLL_INNER));
if (_eq(_5.field(1), false)) {
return bb2();
} else {
throw new Error('assert failed: Overflow(Mul, const core::str::count::USIZE_SIZE, const core::str::count::UNROLL_INNER)');
}
}
function bb2() {
_4.assign(_5.field(0));
_2.assign(_lt(_3, _4));
switch (switchInt(_2)) {
case 0:return bb5();
default: return bb3();
}
}
function bb3() {
_6=core__str____impl__str____as_bytes(_1);
return bb4();
}
function bb4() {
_0=core__str__count__char_count_general_case(_6);
return bb6();
}
function bb5() {
_0=core__str__count__do_count_chars(_1);
return bb6();
}
function bb6() {
return;
}
bb0();
return _0;
}
function core__str__count__UNROLL_INNER() {
const _0 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(new Uint32(4));
return;
}
bb0();
return _0;
}
function core__str__count__do_count_chars() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new RefVar(sizeof(8));
const _2 = new RefVar(sizeof(8));
const _3 = new RefVar(sizeof(8));
const _4 = new RefVar(sizeof(8));
const _5 = new NoRefVar(sizeof(24));
const _6 = new RefVar(sizeof(8));
const _7 = new NoRefVar(sizeof(1));
const _8 = new NoRefVar(sizeof(1));
const _9 = new NoRefVar(sizeof(1));
const _10 = new NoRefVar(sizeof(1));
const _11 = new NoRefVar(sizeof(4));
const _12 = new NoRefVar(sizeof(4));
const _13 = new RefVar(sizeof(8));
const _14 = new NoRefVar(sizeof(4));
const _15 = new NoRefVar(sizeof(4));
const _16 = new NoRefVar(sizeof(4));
const _17 = new NoRefVar(sizeof(8));
const _18 = new NoRefVar(sizeof(12));
const _19 = new NoRefVar(sizeof(12));
const _20 = new NoRefVar(sizeof(12));
const _21 = new NoRefVar(sizeof(8));
const _22 = new MutRefVar(sizeof(4));
const _23 = new NoRefVar(sizeof(4));
const _24 = new RefVar(sizeof(8));
const _25 = new NoRefVar(sizeof(4));
const _26 = new RefVar(sizeof(8));
const _27 = new RefVar(sizeof(8));
const _28 = new NoRefVar(sizeof(16));
const _29 = new NoRefVar(sizeof(8));
const _30 = new NoRefVar(sizeof(8));
const _31 = new NoRefVar(sizeof(4));
const _32 = new MutRefVar(sizeof(4));
const _33 = new NoRefVar(sizeof(4));
const _34 = new RefVar(sizeof(4));
const _35 = new NoRefVar(sizeof(8));
const _36 = new NoRefVar(sizeof(8));
const _37 = new NoRefVar(sizeof(4));
const _38 = new MutRefVar(sizeof(4));
const _39 = new NoRefVar(sizeof(4));
const _40 = new NoRefVar(sizeof(4));
const _41 = new NoRefVar(sizeof(4));
const _42 = new NoRefVar(sizeof(8));
const _43 = new NoRefVar(sizeof(4));
const _44 = new NoRefVar(sizeof(4));
const _45 = new NoRefVar(sizeof(8));
const _46 = new NoRefVar(sizeof(1));
const _47 = new NoRefVar(sizeof(4));
const _48 = new NoRefVar(sizeof(8));
const _49 = new NoRefVar(sizeof(8));
const _50 = new NoRefVar(sizeof(4));
const _51 = new MutRefVar(sizeof(4));
const _52 = new NoRefVar(sizeof(4));
const _53 = new NoRefVar(sizeof(4));
const _54 = new NoRefVar(sizeof(4));
const _55 = new NoRefVar(sizeof(8));
const _56 = new NoRefVar(sizeof(4));
const _57 = new NoRefVar(sizeof(4));
const _58 = new NoRefVar(sizeof(8));
const _59 = new RefVar(sizeof(4));
const _60 = new RefVar(sizeof(4));
function bb0() {
_6=core__str____impl__str____as_bytes(_1);
return bb1();
}
function bb1() {
_5=core__slice____impl____T______align_to_ty_u8___ty_usize(_6);
return bb2();
}
function bb2() {
_2.assign(_5.field(0));
_3.assign(_5.field(1));
_4.assign(_5.field(2));
_9=core__slice____impl____T______is_empty_ty_usize(_3);
return bb3();
}
function bb3() {
switch (switchInt(_9)) {
case 0:return bb4();
default: return bb5();
}
}
function bb4() {
_11.assign(_ptr_metadata(_2));
_10.assign(_gt(_11, core__str__count__USIZE_SIZE));
switch (switchInt(_10)) {
case 0:return bb6();
default: return bb5();
}
}
function bb5() {
_8.assign(new Bool(true));
return bb7();
}
function bb6() {
_12.assign(_ptr_metadata(_4));
_8.assign(_gt(_12, core__str__count__USIZE_SIZE));
return bb7();
}
function bb7() {
_7=std__intrinsics__unlikely(_8);
return bb8();
}
function bb8() {
switch (switchInt(_7)) {
case 0:return bb11();
default: return bb9();
}
}
function bb9() {
_13=core__str____impl__str____as_bytes(_1);
return bb10();
}
function bb10() {
_0=core__str__count__char_count_general_case(_13);
return bb47();
}
function bb11() {
_15=core__str__count__char_count_general_case(_2);
return bb12();
}
function bb12() {
_16=core__str__count__char_count_general_case(_4);
return bb13();
}
function bb13() {
_17.assign(_add(_15, _16));
if (_eq(_17.field(1), false)) {
return bb14();
} else {
throw new Error('assert failed: Overflow(Add, move _15, move _16)');
}
}
function bb14() {
_14.assign(_17.field(0));
_19=core__slice____impl____T______chunks_ty_usize(_3, core__str__count__do_count_chars__CHUNK_SIZE);
return bb15();
}
function bb15() {
_18=std__iter__IntoIterator__into_iterstd__slice__Chunks_ty_usize(_19);
return bb16();
}
function bb16() {
_20.assign(_18);
return bb17();
}
function bb17() {
_22.assign(_ref(_20));
_21=std__iter__Iterator__nextstd__slice__Chunks_ty_usize(_22);
return bb18();
}
function bb18() {
_23.assign(discriminant(_21));
switch (switchInt(_23)) {
case 0:return bb46();
case 1:return bb20();
default: return bb19();
}
}
function bb19() {
throw new Error('unreachable');
}
function bb20() {
_24.assign(_21.downcast(Some, 1).field(0));
_25.assign(new Uint32(0));
_28=core__slice____impl____T______as_chunks_ty_usize__4_usize(_24);
return bb21();
}
function bb21() {
_26.assign(_28.field(0));
_27.assign(_28.field(1));
_29=std__iter__IntoIterator__into_iter_ref__ty_slice__ty_array__ty_usize_4(_26);
return bb22();
}
function bb22() {
_30.assign(_29);
return bb23();
}
function bb23() {
_32.assign(_ref(_30));
_31=std__iter__Iterator__nextstd__slice__Iter_ty_array__ty_usize_4(_32);
return bb24();
}
function bb24() {
_33.assign(discriminant(_31));
switch (switchInt(_33)) {
case 0:return bb26();
case 1:return bb25();
default: return bb19();
}
}
function bb25() {
_34.assign(_31.downcast(Some, 1).field(0));
_35=std__iter__IntoIterator__into_iter_ref__ty_array__ty_usize_4(_34);
return bb27();
}
function bb26() {
_44.assign(_25);
_43=core__str__count__sum_bytes_in_usize(_44);
return bb33();
}
function bb27() {
_36.assign(_35);
return bb28();
}
function bb28() {
_38.assign(_ref(_36));
_37=std__iter__Iterator__nextstd__slice__Iter_ty_usize(_38);
return bb29();
}
function bb29() {
_39.assign(discriminant(_37));
switch (switchInt(_39)) {
case 0:return bb23();
case 1:return bb30();
default: return bb19();
}
}
function bb30() {
_59.assign(_37.downcast(Some, 1).field(0));
_40.assign(_59.deref());
_41=core__str__count__contains_non_continuation_byte(_40);
return bb31();
}
function bb31() {
_42.assign(_add(_25, _41));
if (_eq(_42.field(1), false)) {
return bb32();
} else {
throw new Error('assert failed: Overflow(Add, copy _25, move _41)');
}
}
function bb32() {
_25.assign(_42.field(0));
return bb28();
}
function bb33() {
_45.assign(_add(_14, _43));
if (_eq(_45.field(1), false)) {
return bb34();
} else {
throw new Error('assert failed: Overflow(Add, copy _14, move _43)');
}
}
function bb34() {
_14.assign(_45.field(0));
_46=core__slice____impl____T______is_empty_ty_usize(_27);
return bb35();
}
function bb35() {
switch (switchInt(_46)) {
case 0:return bb36();
default: return bb17();
}
}
function bb36() {
_47.assign(new Uint32(0));
_48=std__iter__IntoIterator__into_iter_ref__ty_slice__ty_usize(_27);
return bb37();
}
function bb37() {
_49.assign(_48);
return bb38();
}
function bb38() {
_51.assign(_ref(_49));
_50=std__iter__Iterator__nextstd__slice__Iter_ty_usize(_51);
return bb39();
}
function bb39() {
_52.assign(discriminant(_50));
switch (switchInt(_52)) {
case 0:return bb41();
case 1:return bb40();
default: return bb19();
}
}
function bb40() {
_60.assign(_50.downcast(Some, 1).field(0));
_53.assign(_60.deref());
_54=core__str__count__contains_non_continuation_byte(_53);
return bb42();
}
function bb41() {
_57.assign(_47);
_56=core__str__count__sum_bytes_in_usize(_57);
return bb44();
}
function bb42() {
_55.assign(_add(_47, _54));
if (_eq(_55.field(1), false)) {
return bb43();
} else {
throw new Error('assert failed: Overflow(Add, copy _47, move _54)');
}
}
function bb43() {
_47.assign(_55.field(0));
return bb38();
}
function bb44() {
_58.assign(_add(_14, _56));
if (_eq(_58.field(1), false)) {
return bb45();
} else {
throw new Error('assert failed: Overflow(Add, copy _14, move _56)');
}
}
function bb45() {
_14.assign(_58.field(0));
return bb46();
}
function bb46() {
_0.assign(_14);
return bb47();
}
function bb47() {
return;
}
bb0();
return _0;
}
function __I__as__std__iter__IntoIterator____into_iterstd__slice__Chunks_ty_usize() {
const _0 = new NoRefVar(sizeof(12));
const _1 = new NoRefVar(sizeof(12));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function __std__slice__Iter____a,__T____as__std__iter__Iterator____next_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(1));
const _8 = new RefVar(sizeof(4));
const _9 = new RefVar(sizeof(4));
const _10 = new NoRefVar(sizeof(4));
const _11 = new NoRefVar(sizeof(4));
const _12 = new RefVar(sizeof(4));
const _13 = new RefVar(sizeof(4));
const _14 = new NoRefVar(sizeof(4));
function bb0() {
_2.assign(_1.deref().field(0));
_3.assign(_1.deref().field(1));
switch (switchInt(std__mem__SizedTypeProperties__IS_ZST_ty_usize)) {
case 0:return bb7();
default: return bb1();
}
}
function bb1() {
_4=std__ptr__const_ptr____impl__*const__T____addr_ty_usize(_3);
return bb2();
}
function bb2() {
switch (switchInt(_4)) {
case 0:return bb3();
default: return bb4();
}
}
function bb3() {
_0.assign(new Enum(undefined, 0));
return bb14();
}
function bb4() {
_6=core__num____impl__usize____unchecked_sub(_4, new Uint32(1));
return bb5();
}
function bb5() {
_5=std__ptr__without_provenance_mut_ty_usize(_6);
return bb6();
}
function bb6() {
_1.deref().field(1).assign(_5);
return bb12();
}
function bb7() {
_8.assign(_ref(_2));
_10.assign(_3);
_9.assign(_ref(_10));
_7=std__cmp__PartialEq__eqstd__ptr__NonNull_ty_usize__std__ptr__NonNull_ty_usize(_8, _9);
return bb8();
}
function bb8() {
switch (switchInt(_7)) {
case 0:return bb10();
default: return bb9();
}
}
function bb9() {
_0.assign(new Enum(undefined, 0));
return bb14();
}
function bb10() {
_11=std__ptr__NonNull____T____add_ty_usize(_2, new Uint32(1));
return bb11();
}
function bb11() {
_1.deref().field(0).assign(_11);
return bb12();
}
function bb12() {
_14.assign(_2);
_13.assign(_ref(_14));
_12=std__ptr__NonNull____T____as_ref_ty_usize(_13);
return bb13();
}
function bb13() {
_0.assign(new Enum(_12, 1));
return bb14();
}
function bb14() {
return;
}
bb0();
return _0;
}
function core__num____impl__usize____unchecked_sub() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(1));
const _4 = new NoRefVar(sizeof(0));
function bb0() {
_3=core__ub_checks__check_language_ub();
return bb1();
}
function bb1() {
switch (switchInt(_3)) {
case 0:return bb3();
default: return bb2();
}
}
function bb2() {
_4=core__num____impl__usize____unchecked_sub__precondition_check(_1, _2);
return bb3();
}
function bb3() {
_0.assign(_sub(_1, _2));
return;
}
bb0();
return _0;
}
function std__mem__SizedTypeProperties__IS_ZST_ty_usize() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_1=std__mem__size_of_ty_usize();
return bb1();
}
function bb1() {
_0.assign(_eq(_1, new Uint32(0)));
return;
}
function bb2() {
// UnwindResume
}
bb0();
return _0;
}
function core__slice____impl____T______chunks_ty_usize() {
const _0 = new NoRefVar(sizeof(12));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(0));
const _4 = new NoRefVar(sizeof(24));
const _5 = new RefVar(sizeof(4));
function bb0() {
switch (switchInt(_2)) {
case 0:return bb2();
default: return bb1();
}
}
function bb1() {
_0=std__slice__Chunks______a,__T____new_ty_usize(_1, _2);
return bb4();
}
function bb2() {
_5.assign(core__slice____impl____T______chunks_ty_usize__promoted_0);
_4=core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize(_5);
return bb3();
}
function bb3() {
_3=std__rt__panic_fmt(_4);
}
function bb4() {
return;
}
bb0();
return _0;
}
const core__slice____impl____T______chunks_ty_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(8));
function bb0() {
_1.assign(new Array([new Uint8Array([99, 104, 117, 110, 107, 32, 115, 105, 122, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 111, 110, 45, 122, 101, 114, 111])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize() {
const _0 = new NoRefVar(sizeof(24));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(8));
const _4 = new RefVar(sizeof(8));
const _5 = new RefVar(sizeof(4));
function bb0() {
_2.assign(_1);
_3.assign(new Enum(undefined, 0));
_5.assign(core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0);
_4.assign(_5);
_0.assign([_2, _3, _4]);
return;
}
bb0();
return _0;
}
const core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(0));
function bb0() {
_1.assign(new Array([]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function std__slice__Chunks______a,__T____new_ty_usize() {
const _0 = new NoRefVar(sizeof(12));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign([_1, _2]);
return;
}
bb0();
return _0;
}
function __std__slice__Chunks____a,__T____as__std__iter__Iterator____next_ty_usize() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(1));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new RefVar(sizeof(8));
const _7 = new RefVar(sizeof(8));
const _8 = new NoRefVar(sizeof(16));
const _9 = new RefVar(sizeof(8));
const _10 = new RefVar(sizeof(8));
const _11 = new RefVar(sizeof(8));
function bb0() {
_9.assign(_1.deref().field(0));
_2=core__slice____impl____T______is_empty_ty_usize(_9);
return bb1();
}
function bb1() {
switch (switchInt(_2)) {
case 0:return bb3();
default: return bb2();
}
}
function bb2() {
_0.assign(new Enum(undefined, 0));
return bb6();
}
function bb3() {
_10.assign(_1.deref().field(0));
_4.assign(_ptr_metadata(_10));
_5.assign(_1.deref().field(1));
_3=std__cmp__min_ty_usize(_4, _5);
return bb4();
}
function bb4() {
_11.assign(_1.deref().field(0));
_8=core__slice____impl____T______split_at_ty_usize(_11, _3);
return bb5();
}
function bb5() {
_6.assign(_8.field(0));
_7.assign(_8.field(1));
_1.deref().field(0).assign(_7);
_0.assign(new Enum(_6, 1));
return bb6();
}
function bb6() {
return;
}
bb0();
return _0;
}
function core__slice____impl____T______split_at_ty_usize() {
const _0 = new NoRefVar(sizeof(16));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(16));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(0));
const _6 = new NoRefVar(sizeof(24));
const _7 = new RefVar(sizeof(4));
function bb0() {
_3=core__slice____impl____T______split_at_checked_ty_usize(_1, _2);
return bb1();
}
function bb1() {
_4.assign(discriminant(_3));
switch (switchInt(_4)) {
case 0:return bb3();
case 1:return bb4();
default: return bb2();
}
}
function bb2() {
throw new Error('unreachable');
}
function bb3() {
_7.assign(core__slice____impl____T______split_at_ty_usize__promoted_0);
_6=core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize(_7);
return bb5();
}
function bb4() {
_0.assign(_3.downcast(Some, 1).field(0));
return;
}
function bb5() {
_5=std__rt__panic_fmt(_6);
}
bb0();
return _0;
}
const core__slice____impl____T______split_at_ty_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(8));
function bb0() {
_1.assign(new Array([new Uint8Array([109, 105, 100, 32, 62, 32, 108, 101, 110])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize() {
const _0 = new NoRefVar(sizeof(24));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(8));
const _4 = new RefVar(sizeof(8));
const _5 = new RefVar(sizeof(4));
function bb0() {
_2.assign(_1);
_3.assign(new Enum(undefined, 0));
_5.assign(core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0);
_4.assign(_5);
_0.assign([_2, _3, _4]);
return;
}
bb0();
return _0;
}
const core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(0));
function bb0() {
_1.assign(new Array([]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__slice____impl____T______as_chunks_ty_usize__4_usize() {
const _0 = new NoRefVar(sizeof(16));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(1));
const _3 = new NoRefVar(sizeof(0));
const _4 = new NoRefVar(sizeof(24));
const _5 = new RefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(4));
const _9 = new NoRefVar(sizeof(1));
const _10 = new NoRefVar(sizeof(8));
const _11 = new RefVar(sizeof(8));
const _12 = new RefVar(sizeof(8));
const _13 = new NoRefVar(sizeof(16));
const _14 = new RefVar(sizeof(8));
function bb0() {
_2.assign(_ne(4_usize, new Uint32(0)));
switch (switchInt(_2)) {
case 0:return bb1();
default: return bb3();
}
}
function bb1() {
_5.assign(core__slice____impl____T______as_chunks_ty_usize__4_usize__promoted_0);
_4=core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize(_5);
return bb2();
}
function bb2() {
_3=std__rt__panic_fmt(_4);
}
function bb3() {
_8.assign(_ptr_metadata(_1));
_9.assign(_eq(4_usize, new Uint32(0)));
if (_eq(_9, false)) {
return bb4();
} else {
throw new Error('assert failed: DivisionByZero(copy _8)');
}
}
function bb4() {
_7.assign(_div(_8, 4_usize));
_10.assign(_mul(_7, 4_usize));
if (_eq(_10.field(1), false)) {
return bb5();
} else {
throw new Error('assert failed: Overflow(Mul, move _7, const 4_usize)');
}
}
function bb5() {
_6.assign(_10.field(0));
_13=core__slice____impl____T______split_at_unchecked_ty_usize(_1, _6);
return bb6();
}
function bb6() {
_11.assign(_13.field(0));
_12.assign(_13.field(1));
_14=core__slice____impl____T______as_chunks_unchecked_ty_usize__4_usize(_11);
return bb7();
}
function bb7() {
_0.assign(new Tuple([_14, _12]));
return;
}
bb0();
return _0;
}
const core__slice____impl____T______as_chunks_ty_usize__4_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(8));
function bb0() {
_1.assign(new Array([new Uint8Array([99, 104, 117, 110, 107, 32, 115, 105, 122, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 111, 110, 45, 122, 101, 114, 111])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize() {
const _0 = new NoRefVar(sizeof(24));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(8));
const _4 = new RefVar(sizeof(8));
const _5 = new RefVar(sizeof(4));
function bb0() {
_2.assign(_1);
_3.assign(new Enum(undefined, 0));
_5.assign(core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0);
_4.assign(_5);
_0.assign([_2, _3, _4]);
return;
}
bb0();
return _0;
}
const core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(0));
function bb0() {
_1.assign(new Array([]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function std__array____impl__std__iter__IntoIterator__for__&__a____T____N______into_iter_ty_usize__4_usize() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(8));
function bb0() {
_2.assign(_1);
_0=core__slice____impl____T______iter_ty_usize(_2);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function core__slice____impl____T______iter_ty_usize() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
function bb0() {
_0=std__slice__Iter______a,__T____new_ty_usize(_1);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function std__slice__Iter______a,__T____new_ty_usize() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(8));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(4));
function bb0() {
_2.assign(_ptr_metadata(_1));
_4=std__ptr__NonNull____T____from_ref_ty_slice__ty_usize(_1);
return bb1();
}
function bb1() {
_3=std__ptr__NonNull____T____cast_ty_slice__ty_usize___ty_usize(_4);
return bb2();
}
function bb2() {
switch (switchInt(std__mem__SizedTypeProperties__IS_ZST_ty_usize)) {
case 0:return bb4();
default: return bb3();
}
}
function bb3() {
_5=std__ptr__without_provenance_ty_usize(_2);
return bb7();
}
function bb4() {
_7=std__ptr__NonNull____T____as_ptr_ty_usize(_3);
return bb5();
}
function bb5() {
_6=std__ptr__mut_ptr____impl__*mut__T____add_ty_usize(_7, _2);
return bb6();
}
function bb6() {
_5.assign(_6);
return bb7();
}
function bb7() {
_8.assign(_5);
_0.assign([_3, _8, std__marker__PhantomData_ref__ty_usize]);
return;
}
bb0();
return _0;
}
function std__ptr__mut_ptr____impl__*mut__T____add_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(1));
const _4 = new NoRefVar(sizeof(0));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
function bb0() {
_3=core__ub_checks__check_language_ub();
return bb1();
}
function bb1() {
switch (switchInt(_3)) {
case 0:return bb4();
default: return bb2();
}
}
function bb2() {
_5.assign(_1);
_6=std__mem__size_of_ty_usize();
return bb3();
}
function bb3() {
_4=std__ptr__mut_ptr____impl__*mut__T____add__precondition_check(_5, _2, _6);
return bb4();
}
function bb4() {
_0.assign(_offset(_1, _2));
return;
}
bb0();
return _0;
}
function std__ptr__mut_ptr____impl__*mut__T____add__precondition_check() {
const _0 = new NoRefVar(sizeof(0));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(1));
const _5 = new NoRefVar(sizeof(0));
const _6 = new NoRefVar(sizeof(24));
const _7 = new RefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(8));
const _9 = new RefVar(sizeof(8));
function bb0() {
_4=std__ptr__mut_ptr____impl__*mut__T____add__runtime_add_nowrap(_1, _2, _3);
return bb1();
}
function bb1() {
switch (switchInt(_4)) {
case 0:return bb3();
default: return bb2();
}
}
function bb2() {
return;
}
function bb3() {
_9.assign(new Uint8Array([117, 110, 115, 97, 102, 101, 32, 112, 114, 101, 99, 111, 110, 100, 105, 116, 105, 111, 110, 40, 115, 41, 32, 118, 105, 111, 108, 97, 116, 101, 100, 58, 32, 112, 116, 114, 58, 58, 97, 100, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 97, 116, 32, 116, 104, 101, 32, 97, 100, 100, 114, 101, 115, 115, 32, 99, 97, 108, 99, 117, 108, 97, 116, 105, 111, 110, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 111, 118, 101, 114, 102, 108, 111, 119, 10, 10, 84, 104, 105, 115, 32, 105, 110, 100, 105, 99, 97, 116, 101, 115, 32, 97, 32, 98, 117, 103, 32, 105, 110, 32, 116, 104, 101, 32, 112, 114, 111, 103, 114, 97, 109, 46, 32, 84, 104, 105, 115, 32, 85, 110, 100, 101, 102, 105, 110, 101, 100, 32, 66, 101, 104, 97, 118, 105, 111, 114, 32, 99, 104, 101, 99, 107, 32, 105, 115, 32, 111, 112, 116, 105, 111, 110, 97, 108, 44, 32, 97, 110, 100, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 114, 101, 108, 105, 101, 100, 32, 111, 110, 32, 102, 111, 114, 32, 115, 97, 102, 101, 116, 121, 46]));
_8.assign(new Array([_9]));
_7.assign(_ref(_8));
_6=core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize(_7);
return bb4();
}
function bb4() {
_5=core__panicking__panic_nounwind_fmt(_6, new Bool(false));
}
bb0();
return _0;
}
function std__ptr__without_provenance_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_2=std__ptr__without_provenance_mut_ty_usize(_1);
return bb1();
}
function bb1() {
_0.assign(_2);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____as_ptr_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function std__fmt__Formatter______a____sign_plus() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new RefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
function bb0() {
_3.assign(_1.deref().field(0).field(0));
_2.assign(_and(_3, core__fmt__flags__SIGN_PLUS_FLAG));
_0.assign(_ne(_2, new Uint32(0)));
return;
}
bb0();
return _0;
}
function std__fmt__Formatter______a____pad_integral__write_prefix() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(1));
const _7 = new NoRefVar(sizeof(1));
const _8 = new NoRefVar(sizeof(4));
const _9 = new NoRefVar(sizeof(4));
const _10 = new RefVar(sizeof(8));
const _11 = new MutRefVar(sizeof(8));
const _12 = new MutRefVar(sizeof(8));
function bb0() {
_4.assign(discriminant(_2));
switch (switchInt(_4)) {
case 1:return bb1();
case 0:return bb6();
default: return bb4();
}
}
function bb1() {
_5.assign(_2.downcast(Some, 1).field(0));
_11.assign(_1.deref().field(1));
_7=_fn_call(_11, _5);
return bb2();
}
function bb2() {
_6=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_7);
return bb3();
}
function bb3() {
_8.assign(discriminant(_6));
switch (switchInt(_8)) {
case 0:return bb6();
case 1:return bb5();
default: return bb4();
}
}
function bb4() {
throw new Error('unreachable');
}
function bb5() {
_0=std__ops__FromResidual__from_residualstd__result__Result_ty_tuple__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb9();
}
function bb6() {
_9.assign(discriminant(_3));
switch (switchInt(_9)) {
case 1:return bb7();
case 0:return bb8();
default: return bb4();
}
}
function bb7() {
_10.assign(_3.downcast(Some, 1).field(0));
_12.assign(_1.deref().field(1));
_0=_fn_call(_12, _10);
return bb9();
}
function bb8() {
_0.assign([new Tuple([])]);
return bb9();
}
function bb9() {
return;
}
bb0();
return _0;
}
function __std__result__Result__T,__E____as__std__ops__Try____branch_ty_tuple__std__fmt__Error() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(1));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(0));
const _4 = new NoRefVar(sizeof(0));
const _5 = new NoRefVar(sizeof(0));
function bb0() {
_2.assign(discriminant(_1));
switch (switchInt(_2)) {
case 0:return bb3();
case 1:return bb2();
default: return bb1();
}
}
function bb1() {
throw new Error('unreachable');
}
function bb2() {
_4.assign(_1.downcast(Err, 1).field(0));
_5.assign([_4]);
_0.assign([_5]);
return bb4();
}
function bb3() {
_3.assign(_1.downcast(Ok, 0).field(0));
_0.assign([_3]);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function std__fmt__Formatter______a____padding() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(2));
const _3 = new NoRefVar(sizeof(1));
const _4 = new NoRefVar(sizeof(1));
const _5 = new NoRefVar(sizeof(1));
const _6 = new RefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(4));
const _8 = new RefVar(sizeof(4));
const _9 = new NoRefVar(sizeof(2));
const _10 = new NoRefVar(sizeof(4));
const _11 = new NoRefVar(sizeof(1));
const _12 = new NoRefVar(sizeof(4));
const _13 = new NoRefVar(sizeof(4));
const _14 = new NoRefVar(sizeof(2));
const _15 = new NoRefVar(sizeof(4));
const _16 = new NoRefVar(sizeof(4));
const _17 = new MutRefVar(sizeof(4));
const _18 = new NoRefVar(sizeof(4));
const _19 = new NoRefVar(sizeof(1));
const _20 = new NoRefVar(sizeof(1));
const _21 = new NoRefVar(sizeof(4));
const _22 = new NoRefVar(sizeof(8));
const _23 = new NoRefVar(sizeof(2));
const _24 = new NoRefVar(sizeof(2));
const _25 = new NoRefVar(sizeof(4));
const _26 = new MutRefVar(sizeof(8));
function bb0() {
_6.assign(_ref(_1.deref().field(0)));
_5=std__fmt__FormattingOptions__get_align(_6);
return bb1();
}
function bb1() {
_4=std__option__Option____T____unwrap_orstd__fmt__Alignment(_5, _3);
return bb2();
}
function bb2() {
_8.assign(_ref(_1.deref().field(0)));
_7=std__fmt__FormattingOptions__get_fill(_8);
return bb3();
}
function bb3() {
_10.assign(discriminant(_4));
switch (switchInt(_10)) {
case 0:return bb7();
case 1:return bb6();
case 2:return bb5();
default: return bb4();
}
}
function bb4() {
throw new Error('unreachable');
}
function bb5() {
_11.assign(_eq(new Uint16(2), new Uint16(0)));
if (_eq(_11, false)) {
return bb8();
} else {
throw new Error('assert failed: DivisionByZero(copy _2)');
}
}
function bb6() {
_9.assign(_2);
return bb9();
}
function bb7() {
_9.assign(new Uint16(0));
return bb9();
}
function bb8() {
_9.assign(_div(_2, new Uint16(2)));
return bb9();
}
function bb9() {
_14.assign(_9);
_13.assign([new Uint16(0), _14]);
_12=std__iter__IntoIterator__into_iterstd__ops__Range_ty_u16(_13);
return bb10();
}
function bb10() {
_15.assign(_12);
return bb11();
}
function bb11() {
_17.assign(_ref(_15));
_16=std__iter__Iterator__nextstd__ops__Range_ty_u16(_17);
return bb12();
}
function bb12() {
_18.assign(discriminant(_16));
switch (switchInt(_18)) {
case 0:return bb14();
case 1:return bb13();
default: return bb4();
}
}
function bb13() {
_26.assign(_1.deref().field(1));
_20=_fn_call(_26, _7);
return bb15();
}
function bb14() {
_24.assign(_9);
_25.assign(_sub(_2, _24));
if (_eq(_25.field(1), false)) {
return bb18();
} else {
throw new Error('assert failed: Overflow(Sub, copy _2, move _24)');
}
}
function bb15() {
_19=std__ops__Try__branchstd__result__Result_ty_tuple__std__fmt__Error(_20);
return bb16();
}
function bb16() {
_21.assign(discriminant(_19));
switch (switchInt(_21)) {
case 0:return bb11();
case 1:return bb17();
default: return bb4();
}
}
function bb17() {
_0=std__ops__FromResidual__from_residualstd__result__Resultcore__fmt__PostPadding__std__fmt__Error__std__result__Resultstd__convert__Infallible__std__fmt__Error(std__result__Resultstd__convert__Infallible__std__fmt__Error);
return bb20();
}
function bb18() {
_23.assign(_25.field(0));
_22=core__fmt__PostPadding__new(_7, _23);
return bb19();
}
function bb19() {
_0.assign([_22]);
return bb20();
}
function bb20() {
return;
}
bb0();
return _0;
}
function core__fmt__PostPadding__new() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(2));
function bb0() {
_0.assign([_1, _2]);
return;
}
bb0();
return _0;
}
function __std__result__Result__T,__E____as__std__ops__Try____branch_ty_tuple__std__fmt__Error() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(1));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(0));
const _4 = new NoRefVar(sizeof(0));
const _5 = new NoRefVar(sizeof(0));
function bb0() {
_2.assign(discriminant(_1));
switch (switchInt(_2)) {
case 0:return bb3();
case 1:return bb2();
default: return bb1();
}
}
function bb1() {
throw new Error('unreachable');
}
function bb2() {
_4.assign(_1.downcast(Err, 1).field(0));
_5.assign([_4]);
_0.assign([_5]);
return bb4();
}
function bb3() {
_3.assign(_1.downcast(Ok, 0).field(0));
_0.assign([_3]);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function core__slice____impl____T______get_uncheckedstd__mem__MaybeUninit_ty_u8__std__ops__RangeFrom_ty_usize() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(8));
function bb0() {
_4.assign(_raw_ptr(_1.deref()));
_3=std__slice__SliceIndex__get_uncheckedstd__ops__RangeFrom_ty_usize___ty_slice_std__mem__MaybeUninit_ty_u8(_2, _4);
return bb1();
}
function bb1() {
_0.assign(_ref(_3.deref()));
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____cast_ty_slice__ty_usize___ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(8));
function bb0() {
_4=std__ptr__NonNull____T____as_ptr_ty_slice__ty_usize(_1);
return bb1();
}
function bb1() {
_3.assign(_4);
_2.assign(_3);
_0.assign([_2]);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____as_ptr_ty_slice__ty_usize() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new NoRefVar(sizeof(8));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function std__option__Option____T____unwrap_orstd__fmt__Alignment() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(1));
const _2 = new NoRefVar(sizeof(1));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(1));
const _5 = new NoRefVar(sizeof(1));
function bb0() {
_5.assign(new Bool(false));
_5.assign(new Bool(true));
_3.assign(discriminant(_1));
switch (switchInt(_3)) {
case 0:return bb2();
case 1:return bb3();
default: return bb1();
}
}
function bb1() {
throw new Error('unreachable');
}
function bb2() {
_5.assign(new Bool(false));
_0.assign(_2);
return bb6();
}
function bb3() {
_4.assign(_1.downcast(Some, 1).field(0));
_0.assign(_4);
return bb6();
}
function bb4() {
return;
}
function bb5() {
return bb4();
}
function bb6() {
switch (switchInt(_5)) {
case 0:return bb4();
default: return bb5();
}
}
bb0();
return _0;
}
function std__ptr__without_provenance_mut_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function core__fmt__num__imp____impl__u32_____fmt_inner() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new MutRefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(4));
const _4 = new RefVar(sizeof(8));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(1));
const _7 = new NoRefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(1));
const _9 = new NoRefVar(sizeof(4));
const _10 = new NoRefVar(sizeof(4));
const _11 = new NoRefVar(sizeof(8));
const _12 = new RefVar(sizeof(8));
const _13 = new NoRefVar(sizeof(0));
const _14 = new NoRefVar(sizeof(1));
const _15 = new NoRefVar(sizeof(4));
const _16 = new NoRefVar(sizeof(0));
const _17 = new NoRefVar(sizeof(1));
const _18 = new NoRefVar(sizeof(4));
const _19 = new NoRefVar(sizeof(4));
const _20 = new RefVar(sizeof(8));
const _21 = new NoRefVar(sizeof(8));
const _22 = new NoRefVar(sizeof(4));
const _23 = new NoRefVar(sizeof(8));
const _24 = new RefVar(sizeof(8));
const _25 = new NoRefVar(sizeof(4));
const _26 = new NoRefVar(sizeof(4));
const _27 = new NoRefVar(sizeof(1));
const _28 = new NoRefVar(sizeof(1));
const _29 = new NoRefVar(sizeof(4));
const _30 = new NoRefVar(sizeof(4));
const _31 = new NoRefVar(sizeof(1));
const _32 = new NoRefVar(sizeof(4));
const _33 = new NoRefVar(sizeof(4));
const _34 = new NoRefVar(sizeof(1));
const _35 = new MutRefVar(sizeof(4));
const _36 = new MutRefVar(sizeof(4));
const _37 = new NoRefVar(sizeof(4));
const _38 = new NoRefVar(sizeof(4));
const _39 = new NoRefVar(sizeof(8));
const _40 = new NoRefVar(sizeof(8));
const _41 = new NoRefVar(sizeof(4));
const _42 = new NoRefVar(sizeof(1));
const _43 = new NoRefVar(sizeof(1));
const _44 = new RefVar(sizeof(4));
const _45 = new NoRefVar(sizeof(4));
const _46 = new NoRefVar(sizeof(4));
const _47 = new NoRefVar(sizeof(8));
const _48 = new NoRefVar(sizeof(8));
const _49 = new NoRefVar(sizeof(1));
const _50 = new MutRefVar(sizeof(4));
const _51 = new MutRefVar(sizeof(4));
const _52 = new NoRefVar(sizeof(4));
const _53 = new NoRefVar(sizeof(4));
const _54 = new NoRefVar(sizeof(8));
const _55 = new NoRefVar(sizeof(8));
const _56 = new NoRefVar(sizeof(4));
const _57 = new NoRefVar(sizeof(1));
const _58 = new NoRefVar(sizeof(1));
const _59 = new RefVar(sizeof(4));
const _60 = new NoRefVar(sizeof(4));
const _61 = new NoRefVar(sizeof(4));
const _62 = new NoRefVar(sizeof(8));
const _63 = new NoRefVar(sizeof(8));
const _64 = new NoRefVar(sizeof(1));
const _65 = new MutRefVar(sizeof(4));
const _66 = new MutRefVar(sizeof(4));
const _67 = new NoRefVar(sizeof(4));
const _68 = new NoRefVar(sizeof(4));
const _69 = new NoRefVar(sizeof(8));
const _70 = new NoRefVar(sizeof(8));
const _71 = new NoRefVar(sizeof(4));
const _72 = new NoRefVar(sizeof(1));
const _73 = new NoRefVar(sizeof(1));
const _74 = new RefVar(sizeof(4));
const _75 = new NoRefVar(sizeof(4));
const _76 = new NoRefVar(sizeof(4));
const _77 = new NoRefVar(sizeof(8));
const _78 = new NoRefVar(sizeof(8));
const _79 = new NoRefVar(sizeof(1));
const _80 = new MutRefVar(sizeof(4));
const _81 = new MutRefVar(sizeof(4));
const _82 = new NoRefVar(sizeof(4));
const _83 = new NoRefVar(sizeof(4));
const _84 = new NoRefVar(sizeof(8));
const _85 = new NoRefVar(sizeof(8));
const _86 = new NoRefVar(sizeof(4));
const _87 = new NoRefVar(sizeof(1));
const _88 = new NoRefVar(sizeof(1));
const _89 = new RefVar(sizeof(4));
const _90 = new NoRefVar(sizeof(4));
const _91 = new NoRefVar(sizeof(4));
const _92 = new NoRefVar(sizeof(8));
const _93 = new NoRefVar(sizeof(8));
const _94 = new NoRefVar(sizeof(1));
const _95 = new NoRefVar(sizeof(1));
const _96 = new NoRefVar(sizeof(4));
const _97 = new NoRefVar(sizeof(0));
const _98 = new NoRefVar(sizeof(1));
const _99 = new NoRefVar(sizeof(4));
const _100 = new NoRefVar(sizeof(0));
const _101 = new NoRefVar(sizeof(1));
const _102 = new NoRefVar(sizeof(4));
const _103 = new NoRefVar(sizeof(4));
const _104 = new RefVar(sizeof(8));
const _105 = new NoRefVar(sizeof(8));
const _106 = new NoRefVar(sizeof(4));
const _107 = new NoRefVar(sizeof(4));
const _108 = new NoRefVar(sizeof(4));
const _109 = new NoRefVar(sizeof(1));
const _110 = new NoRefVar(sizeof(1));
const _111 = new MutRefVar(sizeof(4));
const _112 = new MutRefVar(sizeof(4));
const _113 = new NoRefVar(sizeof(4));
const _114 = new NoRefVar(sizeof(4));
const _115 = new NoRefVar(sizeof(8));
const _116 = new NoRefVar(sizeof(8));
const _117 = new NoRefVar(sizeof(4));
const _118 = new NoRefVar(sizeof(1));
const _119 = new NoRefVar(sizeof(1));
const _120 = new RefVar(sizeof(4));
const _121 = new NoRefVar(sizeof(4));
const _122 = new NoRefVar(sizeof(4));
const _123 = new NoRefVar(sizeof(8));
const _124 = new NoRefVar(sizeof(8));
const _125 = new NoRefVar(sizeof(1));
const _126 = new MutRefVar(sizeof(4));
const _127 = new MutRefVar(sizeof(4));
const _128 = new NoRefVar(sizeof(4));
const _129 = new NoRefVar(sizeof(4));
const _130 = new NoRefVar(sizeof(8));
const _131 = new NoRefVar(sizeof(8));
const _132 = new NoRefVar(sizeof(4));
const _133 = new NoRefVar(sizeof(1));
const _134 = new NoRefVar(sizeof(1));
const _135 = new RefVar(sizeof(4));
const _136 = new NoRefVar(sizeof(4));
const _137 = new NoRefVar(sizeof(4));
const _138 = new NoRefVar(sizeof(8));
const _139 = new NoRefVar(sizeof(8));
const _140 = new NoRefVar(sizeof(1));
const _141 = new NoRefVar(sizeof(4));
const _142 = new NoRefVar(sizeof(0));
const _143 = new NoRefVar(sizeof(1));
const _144 = new NoRefVar(sizeof(4));
const _145 = new NoRefVar(sizeof(0));
const _146 = new NoRefVar(sizeof(1));
const _147 = new NoRefVar(sizeof(4));
const _148 = new NoRefVar(sizeof(4));
const _149 = new RefVar(sizeof(8));
const _150 = new NoRefVar(sizeof(8));
const _151 = new NoRefVar(sizeof(4));
const _152 = new NoRefVar(sizeof(4));
const _153 = new NoRefVar(sizeof(4));
const _154 = new MutRefVar(sizeof(4));
const _155 = new MutRefVar(sizeof(4));
const _156 = new NoRefVar(sizeof(4));
const _157 = new NoRefVar(sizeof(8));
const _158 = new NoRefVar(sizeof(4));
const _159 = new NoRefVar(sizeof(1));
const _160 = new NoRefVar(sizeof(1));
const _161 = new RefVar(sizeof(4));
const _162 = new NoRefVar(sizeof(4));
const _163 = new NoRefVar(sizeof(4));
const _164 = new NoRefVar(sizeof(8));
const _165 = new NoRefVar(sizeof(8));
const _166 = new NoRefVar(sizeof(1));
const _167 = new RefVar(sizeof(4));
const _168 = new RefVar(sizeof(4));
const _169 = new RefVar(sizeof(4));
const _170 = new RefVar(sizeof(4));
const _171 = new RefVar(sizeof(4));
const _172 = new RefVar(sizeof(4));
const _173 = new RefVar(sizeof(4));
function bb0() {
_4.assign(_ref(_2.deref()));
_3.assign(_ptr_metadata(_4));
_5.assign(_1);
return bb1();
}
function bb1() {
_7=std__mem__size_of_ty_u32();
return bb2();
}
function bb2() {
_6.assign(_gt(_7, new Uint32(1)));
switch (switchInt(_6)) {
case 0:return bb39();
default: return bb3();
}
}
function bb3() {
_9.assign(_5);
_11=std__convert__TryInto__try_into_ty_i32___ty_u32(new Int32(999));
return bb4();
}
function bb4() {
_12.assign(new Uint8Array([98, 114, 97, 110, 99, 104, 32, 105, 115, 32, 110, 111, 116, 32, 104, 105, 116, 32, 102, 111, 114, 32, 116, 121, 112, 101, 115, 32, 116, 104, 97, 116, 32, 99, 97, 110, 110, 111, 116, 32, 102, 105, 116, 32, 57, 57, 57, 32, 40, 117, 56, 41]));
_10=std__result__Result____T,__E____expect_ty_u32__std__num__TryFromIntError(_11, _12);
return bb5();
}
function bb5() {
_8.assign(_gt(_9, _10));
switch (switchInt(_8)) {
case 0:return bb39();
default: return bb6();
}
}
function bb6() {
_15.assign(_3);
_14.assign(_ge(_15, new Uint32(4)));
_13=std__hint__assert_unchecked(_14);
return bb7();
}
function bb7() {
_18.assign(_3);
_20.assign(_ref(_2.deref()));
_19.assign(_ptr_metadata(_20));
_17.assign(_le(_18, _19));
_16=std__hint__assert_unchecked(_17);
return bb8();
}
function bb8() {
_21.assign(_sub(_3, new Uint32(4)));
if (_eq(_21.field(1), false)) {
return bb9();
} else {
throw new Error('assert failed: Overflow(Sub, copy _3, const 4_usize)');
}
}
function bb9() {
_3.assign(_21.field(0));
_23=std__convert__TryInto__try_into_ty_i32___ty_u32(new Int32(10000));
return bb10();
}
function bb10() {
_24.assign(new Uint8Array([98, 114, 97, 110, 99, 104, 32, 105, 115, 32, 110, 111, 116, 32, 104, 105, 116, 32, 102, 111, 114, 32, 116, 121, 112, 101, 115, 32, 116, 104, 97, 116, 32, 99, 97, 110, 110, 111, 116, 32, 102, 105, 116, 32, 49, 69, 52, 32, 40, 117, 56, 41]));
_22=std__result__Result____T,__E____expect_ty_u32__std__num__TryFromIntError(_23, _24);
return bb11();
}
function bb11() {
_26.assign(_5);
_27.assign(_eq(_22, new Uint32(0)));
if (_eq(_27, false)) {
return bb12();
} else {
throw new Error('assert failed: RemainderByZero(copy _26)');
}
}
function bb12() {
_25.assign(_rem(_26, _22));
_28.assign(_eq(_22, new Uint32(0)));
if (_eq(_28, false)) {
return bb13();
} else {
throw new Error('assert failed: DivisionByZero(copy _5)');
}
}
function bb13() {
_5.assign(_div(_5, _22));
_31.assign(_eq(new Uint32(100), new Uint32(0)));
if (_eq(_31, false)) {
return bb14();
} else {
throw new Error('assert failed: DivisionByZero(copy _25)');
}
}
function bb14() {
_30.assign(_div(_25, new Uint32(100)));
_29.assign(_30);
_34.assign(_eq(new Uint32(100), new Uint32(0)));
if (_eq(_34, false)) {
return bb15();
} else {
throw new Error('assert failed: RemainderByZero(copy _25)');
}
}
function bb15() {
_33.assign(_rem(_25, new Uint32(100)));
_32.assign(_33);
_38.assign(_3);
_39.assign(_add(_38, new Uint32(0)));
if (_eq(_39.field(1), false)) {
return bb16();
} else {
throw new Error('assert failed: Overflow(Add, move _38, const 0_usize)');
}
}
function bb16() {
_37.assign(_39.field(0));
_40.assign(_raw_ptr(_2.deref()));
_41.assign(_ptr_metadata(_40));
_42.assign(_lt(_37, _41));
if (_eq(_42, true)) {
return bb17();
} else {
throw new Error('assert failed: BoundsCheck { len: move _41, index: copy _37 }');
}
}
function bb17() {
_36.assign(_ref(_2.deref().index(_37)));
_44.assign(_ref(pointer to alloc10));
_47.assign(_mul(_29, new Uint32(2)));
if (_eq(_47.field(1), false)) {
return bb18();
} else {
throw new Error('assert failed: Overflow(Mul, copy _29, const 2_usize)');
}
}
function bb18() {
_46.assign(_47.field(0));
_48.assign(_add(_46, new Uint32(0)));
if (_eq(_48.field(1), false)) {
return bb19();
} else {
throw new Error('assert failed: Overflow(Add, move _46, const 0_usize)');
}
}
function bb19() {
_45.assign(_48.field(0));
_49.assign(_lt(_45, 200_usize));
if (_eq(_49, true)) {
return bb20();
} else {
throw new Error('assert failed: BoundsCheck { len: const 200_usize, index: copy _45 }');
}
}
function bb20() {
_167.assign(_44.deref());
_43.assign(_167.deref().index(_45));
_35=std__mem__MaybeUninit____T____write_ty_u8(_36, _43);
return bb21();
}
function bb21() {
_53.assign(_3);
_54.assign(_add(_53, new Uint32(1)));
if (_eq(_54.field(1), false)) {
return bb22();
} else {
throw new Error('assert failed: Overflow(Add, move _53, const 1_usize)');
}
}
function bb22() {
_52.assign(_54.field(0));
_55.assign(_raw_ptr(_2.deref()));
_56.assign(_ptr_metadata(_55));
_57.assign(_lt(_52, _56));
if (_eq(_57, true)) {
return bb23();
} else {
throw new Error('assert failed: BoundsCheck { len: move _56, index: copy _52 }');
}
}
function bb23() {
_51.assign(_ref(_2.deref().index(_52)));
_59.assign(_ref(pointer to alloc10));
_62.assign(_mul(_29, new Uint32(2)));
if (_eq(_62.field(1), false)) {
return bb24();
} else {
throw new Error('assert failed: Overflow(Mul, copy _29, const 2_usize)');
}
}
function bb24() {
_61.assign(_62.field(0));
_63.assign(_add(_61, new Uint32(1)));
if (_eq(_63.field(1), false)) {
return bb25();
} else {
throw new Error('assert failed: Overflow(Add, move _61, const 1_usize)');
}
}
function bb25() {
_60.assign(_63.field(0));
_64.assign(_lt(_60, 200_usize));
if (_eq(_64, true)) {
return bb26();
} else {
throw new Error('assert failed: BoundsCheck { len: const 200_usize, index: copy _60 }');
}
}
function bb26() {
_168.assign(_59.deref());
_58.assign(_168.deref().index(_60));
_50=std__mem__MaybeUninit____T____write_ty_u8(_51, _58);
return bb27();
}
function bb27() {
_68.assign(_3);
_69.assign(_add(_68, new Uint32(2)));
if (_eq(_69.field(1), false)) {
return bb28();
} else {
throw new Error('assert failed: Overflow(Add, move _68, const 2_usize)');
}
}
function bb28() {
_67.assign(_69.field(0));
_70.assign(_raw_ptr(_2.deref()));
_71.assign(_ptr_metadata(_70));
_72.assign(_lt(_67, _71));
if (_eq(_72, true)) {
return bb29();
} else {
throw new Error('assert failed: BoundsCheck { len: move _71, index: copy _67 }');
}
}
function bb29() {
_66.assign(_ref(_2.deref().index(_67)));
_74.assign(_ref(pointer to alloc10));
_77.assign(_mul(_32, new Uint32(2)));
if (_eq(_77.field(1), false)) {
return bb30();
} else {
throw new Error('assert failed: Overflow(Mul, copy _32, const 2_usize)');
}
}
function bb30() {
_76.assign(_77.field(0));
_78.assign(_add(_76, new Uint32(0)));
if (_eq(_78.field(1), false)) {
return bb31();
} else {
throw new Error('assert failed: Overflow(Add, move _76, const 0_usize)');
}
}
function bb31() {
_75.assign(_78.field(0));
_79.assign(_lt(_75, 200_usize));
if (_eq(_79, true)) {
return bb32();
} else {
throw new Error('assert failed: BoundsCheck { len: const 200_usize, index: copy _75 }');
}
}
function bb32() {
_169.assign(_74.deref());
_73.assign(_169.deref().index(_75));
_65=std__mem__MaybeUninit____T____write_ty_u8(_66, _73);
return bb33();
}
function bb33() {
_83.assign(_3);
_84.assign(_add(_83, new Uint32(3)));
if (_eq(_84.field(1), false)) {
return bb34();
} else {
throw new Error('assert failed: Overflow(Add, move _83, const 3_usize)');
}
}
function bb34() {
_82.assign(_84.field(0));
_85.assign(_raw_ptr(_2.deref()));
_86.assign(_ptr_metadata(_85));
_87.assign(_lt(_82, _86));
if (_eq(_87, true)) {
return bb35();
} else {
throw new Error('assert failed: BoundsCheck { len: move _86, index: copy _82 }');
}
}
function bb35() {
_81.assign(_ref(_2.deref().index(_82)));
_89.assign(_ref(pointer to alloc10));
_92.assign(_mul(_32, new Uint32(2)));
if (_eq(_92.field(1), false)) {
return bb36();
} else {
throw new Error('assert failed: Overflow(Mul, copy _32, const 2_usize)');
}
}
function bb36() {
_91.assign(_92.field(0));
_93.assign(_add(_91, new Uint32(1)));
if (_eq(_93.field(1), false)) {
return bb37();
} else {
throw new Error('assert failed: Overflow(Add, move _91, const 1_usize)');
}
}
function bb37() {
_90.assign(_93.field(0));
_94.assign(_lt(_90, 200_usize));
if (_eq(_94, true)) {
return bb38();
} else {
throw new Error('assert failed: BoundsCheck { len: const 200_usize, index: copy _90 }');
}
}
function bb38() {
_170.assign(_89.deref());
_88.assign(_170.deref().index(_90));
_80=std__mem__MaybeUninit____T____write_ty_u8(_81, _88);
return bb1();
}
function bb39() {
_96.assign(_5);
_95.assign(_gt(_96, new Uint32(9)));
switch (switchInt(_95)) {
case 0:return bb57();
default: return bb40();
}
}
function bb40() {
_99.assign(_3);
_98.assign(_ge(_99, new Uint32(2)));
_97=std__hint__assert_unchecked(_98);
return bb41();
}
function bb41() {
_102.assign(_3);
_104.assign(_ref(_2.deref()));
_103.assign(_ptr_metadata(_104));
_101.assign(_le(_102, _103));
_100=std__hint__assert_unchecked(_101);
return bb42();
}
function bb42() {
_105.assign(_sub(_3, new Uint32(2)));
if (_eq(_105.field(1), false)) {
return bb43();
} else {
throw new Error('assert failed: Overflow(Sub, copy _3, const 2_usize)');
}
}
function bb43() {
_3.assign(_105.field(0));
_108.assign(_5);
_109.assign(_eq(new Uint32(100), new Uint32(0)));
if (_eq(_109, false)) {
return bb44();
} else {
throw new Error('assert failed: RemainderByZero(copy _108)');
}
}
function bb44() {
_107.assign(_rem(_108, new Uint32(100)));
_106.assign(_107);
_110.assign(_eq(new Uint32(100), new Uint32(0)));
if (_eq(_110, false)) {
return bb45();
} else {
throw new Error('assert failed: DivisionByZero(copy _5)');
}
}
function bb45() {
_5.assign(_div(_5, new Uint32(100)));
_114.assign(_3);
_115.assign(_add(_114, new Uint32(0)));
if (_eq(_115.field(1), false)) {
return bb46();
} else {
throw new Error('assert failed: Overflow(Add, move _114, const 0_usize)');
}
}
function bb46() {
_113.assign(_115.field(0));
_116.assign(_raw_ptr(_2.deref()));
_117.assign(_ptr_metadata(_116));
_118.assign(_lt(_113, _117));
if (_eq(_118, true)) {
return bb47();
} else {
throw new Error('assert failed: BoundsCheck { len: move _117, index: copy _113 }');
}
}
function bb47() {
_112.assign(_ref(_2.deref().index(_113)));
_120.assign(_ref(pointer to alloc10));
_123.assign(_mul(_106, new Uint32(2)));
if (_eq(_123.field(1), false)) {
return bb48();
} else {
throw new Error('assert failed: Overflow(Mul, copy _106, const 2_usize)');
}
}
function bb48() {
_122.assign(_123.field(0));
_124.assign(_add(_122, new Uint32(0)));
if (_eq(_124.field(1), false)) {
return bb49();
} else {
throw new Error('assert failed: Overflow(Add, move _122, const 0_usize)');
}
}
function bb49() {
_121.assign(_124.field(0));
_125.assign(_lt(_121, 200_usize));
if (_eq(_125, true)) {
return bb50();
} else {
throw new Error('assert failed: BoundsCheck { len: const 200_usize, index: copy _121 }');
}
}
function bb50() {
_171.assign(_120.deref());
_119.assign(_171.deref().index(_121));
_111=std__mem__MaybeUninit____T____write_ty_u8(_112, _119);
return bb51();
}
function bb51() {
_129.assign(_3);
_130.assign(_add(_129, new Uint32(1)));
if (_eq(_130.field(1), false)) {
return bb52();
} else {
throw new Error('assert failed: Overflow(Add, move _129, const 1_usize)');
}
}
function bb52() {
_128.assign(_130.field(0));
_131.assign(_raw_ptr(_2.deref()));
_132.assign(_ptr_metadata(_131));
_133.assign(_lt(_128, _132));
if (_eq(_133, true)) {
return bb53();
} else {
throw new Error('assert failed: BoundsCheck { len: move _132, index: copy _128 }');
}
}
function bb53() {
_127.assign(_ref(_2.deref().index(_128)));
_135.assign(_ref(pointer to alloc10));
_138.assign(_mul(_106, new Uint32(2)));
if (_eq(_138.field(1), false)) {
return bb54();
} else {
throw new Error('assert failed: Overflow(Mul, copy _106, const 2_usize)');
}
}
function bb54() {
_137.assign(_138.field(0));
_139.assign(_add(_137, new Uint32(1)));
if (_eq(_139.field(1), false)) {
return bb55();
} else {
throw new Error('assert failed: Overflow(Add, move _137, const 1_usize)');
}
}
function bb55() {
_136.assign(_139.field(0));
_140.assign(_lt(_136, 200_usize));
if (_eq(_140, true)) {
return bb56();
} else {
throw new Error('assert failed: BoundsCheck { len: const 200_usize, index: copy _136 }');
}
}
function bb56() {
_172.assign(_135.deref());
_134.assign(_172.deref().index(_136));
_126=std__mem__MaybeUninit____T____write_ty_u8(_127, _134);
return bb57();
}
function bb57() {
_141.assign(_5);
switch (switchInt(_141)) {
case 0:return bb58();
default: return bb59();
}
}
function bb58() {
switch (switchInt(_1)) {
case 0:return bb59();
default: return bb67();
}
}
function bb59() {
_144.assign(_3);
_143.assign(_ge(_144, new Uint32(1)));
_142=std__hint__assert_unchecked(_143);
return bb60();
}
function bb60() {
_147.assign(_3);
_149.assign(_ref(_2.deref()));
_148.assign(_ptr_metadata(_149));
_146.assign(_le(_147, _148));
_145=std__hint__assert_unchecked(_146);
return bb61();
}
function bb61() {
_150.assign(_sub(_3, new Uint32(1)));
if (_eq(_150.field(1), false)) {
return bb62();
} else {
throw new Error('assert failed: Overflow(Sub, copy _3, const 1_usize)');
}
}
function bb62() {
_3.assign(_150.field(0));
_153.assign(_5);
_152.assign(_and(_153, new Uint32(15)));
_151.assign(_152);
_156.assign(_3);
_157.assign(_raw_ptr(_2.deref()));
_158.assign(_ptr_metadata(_157));
_159.assign(_lt(_156, _158));
if (_eq(_159, true)) {
return bb63();
} else {
throw new Error('assert failed: BoundsCheck { len: move _158, index: copy _156 }');
}
}
function bb63() {
_155.assign(_ref(_2.deref().index(_156)));
_161.assign(_ref(pointer to alloc10));
_164.assign(_mul(_151, new Uint32(2)));
if (_eq(_164.field(1), false)) {
return bb64();
} else {
throw new Error('assert failed: Overflow(Mul, copy _151, const 2_usize)');
}
}
function bb64() {
_163.assign(_164.field(0));
_165.assign(_add(_163, new Uint32(1)));
if (_eq(_165.field(1), false)) {
return bb65();
} else {
throw new Error('assert failed: Overflow(Add, move _163, const 1_usize)');
}
}
function bb65() {
_162.assign(_165.field(0));
_166.assign(_lt(_162, 200_usize));
if (_eq(_166, true)) {
return bb66();
} else {
throw new Error('assert failed: BoundsCheck { len: const 200_usize, index: copy _162 }');
}
}
function bb66() {
_173.assign(_161.deref());
_160.assign(_173.deref().index(_162));
_154=std__mem__MaybeUninit____T____write_ty_u8(_155, _160);
return bb67();
}
function bb67() {
_0.assign(_3);
return;
}
bb0();
return _0;
}
function std__mem__MaybeUninit____T____write_ty_u8() {
const _0 = new MutRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(1));
const _3 = new NoRefVar(sizeof(1));
function bb0() {
_3=std__mem__MaybeUninit____T____new_ty_u8(_2);
return bb1();
}
function bb1() {
_1.deref().assign(_3);
_0=std__mem__MaybeUninit____T____assume_init_mut_ty_u8(_1);
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
function std__mem__MaybeUninit____T____assume_init_mut_ty_u8() {
const _0 = new MutRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(0));
const _3 = new NoRefVar(sizeof(4));
function bb0() {
_2=std__intrinsics__assert_inhabited_ty_u8();
return bb1();
}
function bb1() {
_3=std__mem__MaybeUninit____T____as_mut_ptr_ty_u8(_1);
return bb2();
}
function bb2() {
_0.assign(_ref(_3.deref()));
return;
}
bb0();
return _0;
}
function std__mem__size_of_ty_u32() {
const _0 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(4);
return;
}
bb0();
return _0;
}
function std__mem__MaybeUninit____T____as_mut_ptr_ty_u8() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_2.assign(_raw_ptr(_1.deref()));
_0.assign(_2);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____from_ref_ty_i32() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new RefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_2.assign(_raw_ptr(_1.deref()));
_0.assign([_2]);
return;
}
bb0();
return _0;
}
function core__slice__iter____impl__std__iter__IntoIterator__for__&__a____T______into_iter_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
function bb0() {
_0=core__slice____impl____T______iter_ty_array__ty_usize_4(_1);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function std__rt__panic_fmt() {
const _0 = new NoRefVar(sizeof(0));
const _1 = new NoRefVar(sizeof(24));
const _2 = new NoRefVar(sizeof(0));
const _3 = new NoRefVar(sizeof(12));
function bb0() {
_2=std__intrinsics__abort();
}
bb0();
return _0;
}
function std__slice__Iter______a,__T____as_slice_ty_u8() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(4));
function bb0() {
_0=std__slice__Iter______a,__T____make_slice_ty_u8(_1);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function std__fmt__FormattingOptions__get_fill() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new RefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
function bb0() {
_3.assign(_1.deref().field(0));
_2.assign(_and(_3, new Uint32(2097151)));
_0=std__char__methods____impl__char____from_u32_unchecked(_2);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function core__str____impl__str____as_bytes() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function __std__ops__RangeFrom__usize____as__std__slice__SliceIndex____T________get_uncheckedstd__mem__MaybeUninit_ty_u8() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(8));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
function bb0() {
_4.assign(_1.field(0));
_5=std__ptr__const_ptr____impl__*const____T______lenstd__mem__MaybeUninit_ty_u8(_2);
return bb1();
}
function bb1() {
_3.assign([_4, _5]);
_0=std__slice__SliceIndex__get_uncheckedstd__ops__Range_ty_usize___ty_slice_std__mem__MaybeUninit_ty_u8(_3, _2);
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
function std__iter__range____impl__std__iter__Iterator__for__std__ops__Range__A______next_ty_u16() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
function bb0() {
_0=std__iter__range__RangeIteratorImpl__spec_nextstd__ops__Range_ty_u16(_1);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function __std__ops__Range__T____as__std__iter__range__RangeIteratorImpl____spec_next_ty_u16() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(1));
const _3 = new RefVar(sizeof(4));
const _4 = new RefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(2));
const _6 = new NoRefVar(sizeof(2));
function bb0() {
_3.assign(_ref(_1.deref().field(0)));
_4.assign(_ref(_1.deref().field(1)));
_2=std__cmp__PartialOrd__lt_ty_u16___ty_u16(_3, _4);
return bb1();
}
function bb1() {
switch (switchInt(_2)) {
case 0:return bb4();
default: return bb2();
}
}
function bb2() {
_5.assign(_1.deref().field(0));
_6=std__iter__Step__forward_unchecked_ty_u16(_5, new Uint32(1));
return bb3();
}
function bb3() {
_1.deref().field(0).assign(_6);
_0.assign(new Enum(_5, 1));
return bb5();
}
function bb4() {
_0.assign(new Enum(undefined, 0));
return bb5();
}
function bb5() {
return;
}
bb0();
return _0;
}
function std__mem__size_of_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(4);
return;
}
bb0();
return _0;
}
function std__cmp__impls____impl__std__cmp__PartialOrd__for__u16____lt() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(2));
const _4 = new NoRefVar(sizeof(2));
function bb0() {
_3.assign(_1.deref());
_4.assign(_2.deref());
_0.assign(_lt(_3, _4));
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____add_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
function bb0() {
_5=std__ptr__NonNull____T____as_ptr_ty_usize(_1);
return bb1();
}
function bb1() {
_4.assign(_5);
_3.assign(_offset(_4, _2));
_0.assign([_3]);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____as_ptr_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function __std__slice__Iter____a,__T____as__std__iter__Iterator____next_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(1));
const _8 = new RefVar(sizeof(4));
const _9 = new RefVar(sizeof(4));
const _10 = new NoRefVar(sizeof(4));
const _11 = new NoRefVar(sizeof(4));
const _12 = new RefVar(sizeof(4));
const _13 = new RefVar(sizeof(4));
const _14 = new NoRefVar(sizeof(4));
function bb0() {
_2.assign(_1.deref().field(0));
_3.assign(_1.deref().field(1));
switch (switchInt(std__mem__SizedTypeProperties__IS_ZST_ty_array__ty_usize_4)) {
case 0:return bb7();
default: return bb1();
}
}
function bb1() {
_4=std__ptr__const_ptr____impl__*const__T____addr_ty_array__ty_usize_4(_3);
return bb2();
}
function bb2() {
switch (switchInt(_4)) {
case 0:return bb3();
default: return bb4();
}
}
function bb3() {
_0.assign(new Enum(undefined, 0));
return bb14();
}
function bb4() {
_6=core__num____impl__usize____unchecked_sub(_4, new Uint32(1));
return bb5();
}
function bb5() {
_5=std__ptr__without_provenance_mut_ty_array__ty_usize_4(_6);
return bb6();
}
function bb6() {
_1.deref().field(1).assign(_5);
return bb12();
}
function bb7() {
_8.assign(_ref(_2));
_10.assign(_3);
_9.assign(_ref(_10));
_7=std__cmp__PartialEq__eqstd__ptr__NonNull_ty_array__ty_usize_4__std__ptr__NonNull_ty_array__ty_usize_4(_8, _9);
return bb8();
}
function bb8() {
switch (switchInt(_7)) {
case 0:return bb10();
default: return bb9();
}
}
function bb9() {
_0.assign(new Enum(undefined, 0));
return bb14();
}
function bb10() {
_11=std__ptr__NonNull____T____add_ty_array__ty_usize_4(_2, new Uint32(1));
return bb11();
}
function bb11() {
_1.deref().field(0).assign(_11);
return bb12();
}
function bb12() {
_14.assign(_2);
_13.assign(_ref(_14));
_12=std__ptr__NonNull____T____as_ref_ty_array__ty_usize_4(_13);
return bb13();
}
function bb13() {
_0.assign(new Enum(_12, 1));
return bb14();
}
function bb14() {
return;
}
bb0();
return _0;
}
function std__mem__SizedTypeProperties__IS_ZST_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_1=std__mem__size_of_ty_array__ty_usize_4();
return bb1();
}
function bb1() {
_0.assign(_eq(_1, new Uint32(0)));
return;
}
function bb2() {
// UnwindResume
}
bb0();
return _0;
}
function std__ptr__const_ptr____impl__*const__T____addr_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_2=std__ptr__const_ptr____impl__*const__T____cast_ty_array__ty_usize_4___ty_tuple(_1);
return bb1();
}
function bb1() {
_0.assign(_2);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____add_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
function bb0() {
_5=std__ptr__NonNull____T____as_ptr_ty_array__ty_usize_4(_1);
return bb1();
}
function bb1() {
_4.assign(_5);
_3.assign(_offset(_4, _2));
_0.assign([_3]);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____as_ptr_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function std__fmt__FormattingOptions__fill() {
const _0 = new MutRefVar(sizeof(4));
const _1 = new MutRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(1));
const _8 = new NoRefVar(sizeof(4));
function bb0() {
_4.assign(_1.deref().field(0));
_6.assign(new Int32(21));
_7.assign(_lt(_6, new Uint32(32)));
if (_eq(_7, true)) {
return bb1();
} else {
throw new Error('assert failed: Overflow(Shl, const core::num::<impl u32>::MAX, const 21_i32)');
}
}
function bb1() {
_5.assign(_shl(core__num____impl__u32____MAX, new Int32(21)));
_3.assign(_and(_4, _5));
_8.assign(_2);
_1.deref().field(0).assign(_or(_3, _8));
_0.assign(_1);
return;
}
bb0();
return _0;
}
function std__cmp__min_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
function bb0() {
_0=std__cmp__Ord__min_ty_usize(_1, _2);
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function std__cmp__Ord__min_ty_usize() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(1));
const _4 = new RefVar(sizeof(4));
const _5 = new RefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(1));
const _7 = new NoRefVar(sizeof(1));
function bb0() {
_7.assign(new Bool(false));
_6.assign(new Bool(false));
_7.assign(new Bool(true));
_6.assign(new Bool(true));
_4.assign(_ref(_2));
_5.assign(_ref(_1));
_3=std__cmp__PartialOrd__lt_ty_usize___ty_usize(_4, _5);
return bb1();
}
function bb1() {
switch (switchInt(_3)) {
case 0:return bb3();
default: return bb2();
}
}
function bb2() {
_6.assign(new Bool(false));
_0.assign(_2);
return bb4();
}
function bb3() {
_7.assign(new Bool(false));
_0.assign(_1);
return bb4();
}
function bb4() {
switch (switchInt(_6)) {
case 0:return bb5();
default: return bb7();
}
}
function bb5() {
switch (switchInt(_7)) {
case 0:return bb6();
default: return bb8();
}
}
function bb6() {
return;
}
function bb7() {
return bb5();
}
function bb8() {
return bb6();
}
bb0();
return _0;
}
function std__cmp__impls____impl__std__cmp__PartialOrd__for__usize____lt() {
const _0 = new NoRefVar(sizeof(1));
const _1 = new RefVar(sizeof(4));
const _2 = new RefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
function bb0() {
_3.assign(_1.deref());
_4.assign(_2.deref());
_0.assign(_lt(_3, _4));
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____as_ref_ty_array__ty_usize_4() {
const _0 = new RefVar(sizeof(4));
const _1 = new RefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
function bb0() {
_4.assign(_1.deref());
_3=std__ptr__NonNull____T____as_ptr_ty_array__ty_usize_4(_4);
return bb1();
}
function bb1() {
_2=std__ptr__mut_ptr____impl__*mut__T____cast_const_ty_array__ty_usize_4(_3);
return bb2();
}
function bb2() {
_0.assign(_ref(_2.deref()));
return;
}
bb0();
return _0;
}
function std__ptr__mut_ptr____impl__*mut__T____cast_const_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function std__ptr__NonNull____T____as_ptr_ty_array__ty_usize_4() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function core__str__count__contains_non_continuation_byte() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new NoRefVar(sizeof(4));
const _2 = new NoRefVar(sizeof(4));
const _3 = new NoRefVar(sizeof(4));
const _4 = new NoRefVar(sizeof(4));
const _5 = new NoRefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(1));
const _7 = new NoRefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(4));
const _9 = new NoRefVar(sizeof(1));
function bb0() {
_4.assign(_not(_1));
_5.assign(new Int32(7));
_6.assign(_lt(_5, new Uint32(32)));
if (_eq(_6, true)) {
return bb1();
} else {
throw new Error('assert failed: Overflow(Shr, copy _4, const 7_i32)');
}
}
function bb1() {
_3.assign(_shr(_4, new Int32(7)));
_8.assign(new Int32(6));
_9.assign(_lt(_8, new Uint32(32)));
if (_eq(_9, true)) {
return bb2();
} else {
throw new Error('assert failed: Overflow(Shr, copy _1, const 6_i32)');
}
}
function bb2() {
_7.assign(_shr(_1, new Int32(6)));
_2.assign(_or(_3, _7));
_0.assign(_and(_2, core__str__count__contains_non_continuation_byte__LSB));
return;
}
bb0();
return _0;
}
function ____std__mem__MaybeUninit__T________assume_init_ref_ty_u8() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(8));
const _3 = new NoRefVar(sizeof(8));
function bb0() {
_3.assign(_raw_ptr(_1.deref()));
_2.assign(_3);
_0.assign(_ref(_2.deref()));
return;
}
bb0();
return _0;
}
function core__str____impl__str____len() {
const _0 = new NoRefVar(sizeof(4));
const _1 = new RefVar(sizeof(8));
const _2 = new RefVar(sizeof(8));
function bb0() {
_2=core__str____impl__str____as_bytes(_1);
return bb1();
}
function bb1() {
_0.assign(_ptr_metadata(_2));
return;
}
bb0();
return _0;
}
function core__str____impl__str____as_bytes() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function core__str____impl__str____chars() {
const _0 = new NoRefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
const _2 = new NoRefVar(sizeof(8));
const _3 = new RefVar(sizeof(8));
function bb0() {
_3=core__str____impl__str____as_bytes(_1);
return bb1();
}
function bb1() {
_2=core__slice____impl____T______iter_ty_u8(_3);
return bb2();
}
function bb2() {
_0.assign([_2]);
return;
}
bb0();
return _0;
}
function core__str____impl__str____as_bytes() {
const _0 = new RefVar(sizeof(8));
const _1 = new RefVar(sizeof(8));
function bb0() {
_0.assign(_1);
return;
}
bb0();
return _0;
}
function core__slice____impl____T______align_to_ty_u8___ty_usize() {
const _0 = new NoRefVar(sizeof(24));
const _1 = new RefVar(sizeof(8));
const _2 = new RefVar(sizeof(8));
const _3 = new RefVar(sizeof(4));
const _4 = new RefVar(sizeof(8));
const _5 = new RefVar(sizeof(4));
const _6 = new NoRefVar(sizeof(4));
const _7 = new NoRefVar(sizeof(4));
const _8 = new NoRefVar(sizeof(4));
const _9 = new NoRefVar(sizeof(1));
const _10 = new NoRefVar(sizeof(4));
const _11 = new RefVar(sizeof(8));
const _12 = new RefVar(sizeof(4));
const _13 = new RefVar(sizeof(8));
const _14 = new RefVar(sizeof(4));
const _15 = new RefVar(sizeof(8));
const _16 = new RefVar(sizeof(8));
const _17 = new NoRefVar(sizeof(16));
const _18 = new NoRefVar(sizeof(4));
const _19 = new NoRefVar(sizeof(4));
const _20 = new NoRefVar(sizeof(8));
const _21 = new RefVar(sizeof(8));
const _22 = new NoRefVar(sizeof(4));
const _23 = new NoRefVar(sizeof(4));
const _24 = new RefVar(sizeof(8));
const _25 = new NoRefVar(sizeof(4));
const _26 = new NoRefVar(sizeof(4));
const _27 = new NoRefVar(sizeof(4));
const _28 = new NoRefVar(sizeof(4));
const _29 = new NoRefVar(sizeof(8));
function bb0() {
switch (switchInt(std__mem__SizedTypeProperties__IS_ZST_ty_usize)) {
case 0:return bb1();
default: return bb2();
}
}
function bb1() {
switch (switchInt(std__mem__SizedTypeProperties__IS_ZST_ty_u8)) {
case 0:return bb3();
default: return bb2();
}
}
function bb2() {
_3.assign(core__slice____impl____T______align_to_ty_u8___ty_usize__promoted_1);
_2.assign(_3);
_5.assign(core__slice____impl____T______align_to_ty_u8___ty_usize__promoted_0);
_4.assign(_5);
_0.assign(new Tuple([_1, _2, _4]));
return bb17();
}
function bb3() {
_6=core__slice____impl____T______as_ptr_ty_u8(_1);
return bb4();
}
function bb4() {
_8=std__mem__align_of_ty_usize();
return bb5();
}
function bb5() {
_7=std__ptr__align_offset_ty_u8(_6, _8);
return bb6();
}
function bb6() {
_10.assign(_ptr_metadata(_1));
_9.assign(_gt(_7, _10));
switch (switchInt(_9)) {
case 0:return bb8();
default: return bb7();
}
}
function bb7() {
_12.assign(core__slice____impl____T______align_to_ty_u8___ty_usize__promoted_3);
_11.assign(_12);
_14.assign(core__slice____impl____T______align_to_ty_u8___ty_usize__promoted_2);
_13.assign(_14);
_0.assign(new Tuple([_1, _11, _13]));
return bb17();
}
function bb8() {
_17=core__slice____impl____T______split_at_ty_u8(_1, _7);
return bb9();
}
function bb9() {
_15.assign(_17.field(0));
_16.assign(_17.field(1));
_20=core__slice____impl____T______align_to_offsets_ty_u8___ty_usize(_16);
return bb10();
}
function bb10() {
_18.assign(_20.field(0));
_19.assign(_20.field(1));
_23=core__slice____impl____T______as_ptr_ty_u8(_16);
return bb11();
}
function bb11() {
_22.assign(_23);
_21=std__slice__from_raw_parts_ty_usize(_22, _18);
return bb12();
}
function bb12() {
_26=core__slice____impl____T______as_ptr_ty_u8(_16);
return bb13();
}
function bb13() {
_28.assign(_ptr_metadata(_16));
_29.assign(_sub(_28, _19));
if (_eq(_29.field(1), false)) {
return bb14();
} else {
throw new Error('assert failed: Overflow(Sub, move _28, copy _19)');
}
}
function bb14() {
_27.assign(_29.field(0));
_25=std__ptr__const_ptr____impl__*const__T____add_ty_u8(_26, _27);
return bb15();
}
function bb15() {
_24=std__slice__from_raw_parts_ty_u8(_25, _19);
return bb16();
}
function bb16() {
_0.assign(new Tuple([_15, _21, _24]));
return bb17();
}
function bb17() {
return;
}
bb0();
return _0;
}
const core__slice____impl____T______align_to_ty_u8___ty_usize__promoted_0 = (() => {
const _0 = new RefVar(sizeof(4));
const _1 = new NoRefVar(
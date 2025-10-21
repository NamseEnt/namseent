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
function __A__as__std__iter__Iterator____next() {
let _0 = new NoRefVar(memory.stackAlloc(8), 8);let _1 = new MutRefVar(memory.stackAlloc(8), 8);let _2 = new NoRefVar(memory.stackAlloc(1), 1);let _3 = new NoRefVar(memory.stackAlloc(4), 4);let _4 = new NoRefVar(memory.stackAlloc(4), 4);let _5 = new NoRefVar(memory.stackAlloc(8), 8);function bb0() {
_3.assign(_1.deref().field(0));
_2.assign(_gt(_3, new Int32(3)));
switch (switchInt(_2)) {
case 0:return bb2();
default: return bb1();
}
}
function bb1() {
_0.assign(new Enum(undefined, 0));
return bb4();
}
function bb2() {
_4.assign(_1.deref().field(0));
_5.assign(_add(_1.deref().field(0), new Int32(1)));
if (_eq(_5.field(1), false)) {
return bb3();
} else {
throw new Error('assert failed: Overflow(Add, copy ((*_1).0: i32), const 1_i32)');
}
}
function bb3() {
_1.deref().field(0).assign(_5.field(0));
_0.assign(new Enum(_4, 1));
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function main() {
let _0 = new NoRefVar(memory.stackAlloc(0), 0);let _1 = new NoRefVar(memory.stackAlloc(12), 12);let _2 = new NoRefVar(memory.stackAlloc(0), 0);let _3 = new NoRefVar(memory.stackAlloc(48), 48);let _4 = new RefVar(memory.stackAlloc(8), 8);let _5 = new NoRefVar(memory.stackAlloc(32), 32);let _6 = new NoRefVar(memory.stackAlloc(32), 32);let _7 = new NoRefVar(memory.stackAlloc(8), 8);let _8 = new MutRefVar(memory.stackAlloc(8), 8);let _9 = new NoRefVar(memory.stackAlloc(8), 8);let _10 = new NoRefVar(memory.stackAlloc(4), 4);let _11 = new NoRefVar(memory.stackAlloc(0), 0);let _12 = new NoRefVar(memory.stackAlloc(48), 48);let _13 = new NoRefVar(memory.stackAlloc(8), 8);let _14 = new RefVar(memory.stackAlloc(8), 8);let _15 = new NoRefVar(memory.stackAlloc(16), 16);let _16 = new NoRefVar(memory.stackAlloc(16), 16);let _17 = new RefVar(memory.stackAlloc(8), 8);let _18 = new RefVar(memory.stackAlloc(8), 8);let _19 = new NoRefVar(memory.stackAlloc(0), 0);let _20 = new NoRefVar(memory.stackAlloc(48), 48);let _21 = new RefVar(memory.stackAlloc(8), 8);let _22 = new NoRefVar(memory.stackAlloc(24), 24);let _23 = new NoRefVar(memory.stackAlloc(16), 16);let _24 = new NoRefVar(memory.stackAlloc(8), 8);let _25 = new NoRefVar(memory.stackAlloc(8), 8);let _26 = new NoRefVar(memory.stackAlloc(8), 8);let _27 = new NoRefVar(memory.stackAlloc(8), 8);let _28 = new NoRefVar(memory.stackAlloc(32), 32);let _29 = new NoRefVar(memory.stackAlloc(32), 32);let _30 = new NoRefVar(memory.stackAlloc(8), 8);let _31 = new MutRefVar(memory.stackAlloc(8), 8);let _32 = new NoRefVar(memory.stackAlloc(8), 8);let _33 = new NoRefVar(memory.stackAlloc(4), 4);let _34 = new NoRefVar(memory.stackAlloc(0), 0);let _35 = new NoRefVar(memory.stackAlloc(48), 48);let _36 = new NoRefVar(memory.stackAlloc(8), 8);let _37 = new RefVar(memory.stackAlloc(8), 8);let _38 = new NoRefVar(memory.stackAlloc(16), 16);let _39 = new NoRefVar(memory.stackAlloc(16), 16);let _40 = new RefVar(memory.stackAlloc(8), 8);let _41 = new RefVar(memory.stackAlloc(8), 8);let _42 = new NoRefVar(memory.stackAlloc(0), 0);let _43 = new NoRefVar(memory.stackAlloc(48), 48);let _44 = new RefVar(memory.stackAlloc(8), 8);let _45 = new NoRefVar(memory.stackAlloc(4), 4);let _46 = new NoRefVar(memory.stackAlloc(4), 4);let _47 = new NoRefVar(memory.stackAlloc(4), 4);let _48 = new NoRefVar(memory.stackAlloc(8), 8);let _49 = new MutRefVar(memory.stackAlloc(8), 8);let _50 = new NoRefVar(memory.stackAlloc(8), 8);let _51 = new NoRefVar(memory.stackAlloc(4), 4);let _52 = new NoRefVar(memory.stackAlloc(0), 0);let _53 = new NoRefVar(memory.stackAlloc(48), 48);let _54 = new NoRefVar(memory.stackAlloc(8), 8);let _55 = new RefVar(memory.stackAlloc(8), 8);let _56 = new NoRefVar(memory.stackAlloc(16), 16);let _57 = new NoRefVar(memory.stackAlloc(16), 16);let _58 = new RefVar(memory.stackAlloc(8), 8);let _59 = new RefVar(memory.stackAlloc(8), 8);let _60 = new RefVar(memory.stackAlloc(8), 8);let _61 = new RefVar(memory.stackAlloc(8), 8);let _62 = new RefVar(memory.stackAlloc(8), 8);let _63 = new NoRefVar(memory.stackAlloc(8), 8);let _64 = new NoRefVar(memory.stackAlloc(8), 8);let _65 = new NoRefVar(memory.stackAlloc(8), 8);let _66 = new NoRefVar(memory.stackAlloc(8), 8);let _67 = new NoRefVar(memory.stackAlloc(8), 8);let _68 = new NoRefVar(memory.stackAlloc(8), 8);let _69 = new NoRefVar(memory.stackAlloc(1), 1);let _70 = new NoRefVar(memory.stackAlloc(8), 8);let _71 = new NoRefVar(memory.stackAlloc(8), 8);let _72 = new NoRefVar(memory.stackAlloc(8), 8);let _73 = new NoRefVar(memory.stackAlloc(1), 1);let _74 = new NoRefVar(memory.stackAlloc(1), 1);let _75 = new NoRefVar(memory.stackAlloc(1), 1);let _76 = new NoRefVar(memory.stackAlloc(1), 1);function bb0() {
_1.assign(new Array([new Int32(1), new Int32(2), new Int32(3)]));
_4.assign(main__promoted_5);
_3=core__fmt__rt__Arguments__new_const(_4);
return bb1();
}
function bb1() {
_2=std__io__print(_3);
return bb2();
}
function bb2() {
_5=std__iter__IntoIterator__into_iter__[i32; 3](_1);
return bb3();
}
function bb3() {
_6.assign(_5);
return bb4();
}
function bb4() {
_8.assign(_ref(_6));
_7=std__iter__Iterator__next(_8);
return bb5();
}
function bb5() {
_9.assign(discriminant(_7));
switch (switchInt(_9)) {
case 0:return bb8();
case 1:return bb7();
default: return bb6();
}
}
function bb6() {
throw new Error('unreachable');
}
function bb7() {
_10.assign(_7.field(0));
_14.assign(_ref(_10));
_13.assign(new Tuple([_14]));
_60.assign(_13.field(0));
_16=core__fmt__rt__Argument__new_display(_60);
return bb9();
}
function bb8() {
return bb11();
}
function bb9() {
_15.assign(new Array([_16]));
_17.assign(main__promoted_0);
_18.assign(_ref(_15));
_12=core__fmt__rt__Arguments__new_v1(_17, _18);
return bb10();
}
function bb10() {
_11=std__io__print(_12);
return bb38();
}
function bb11() {
_21.assign(main__promoted_4);
_20=core__fmt__rt__Arguments__new_const(_21);
return bb12();
}
function bb12() {
_19=std__io__print(_20);
return bb13();
}
function bb13() {
_24.assign(12);
_25.assign(12);
_26=alloc__alloc__exchange_malloc(_24, _25);
return bb14();
}
function bb14() {
_27.assign(_26);
_63.assign(_27.field(0).field(0));
_64.assign(_63);
_65.assign(_64);
_66.assign(12);
_67.assign(_sub(_66, new Uint64(1)));
_68.assign(_and(_65, _67));
_69.assign(_eq(_68, new Uint64(0)));
if (_eq(_69, true)) {
return bb36();
} else {
throw new Error('assert failed: MisalignedPointerDereference { required: copy _66, found: copy _65 }');
}
}
function bb15() {
_28=std__iter__IntoIterator__into_iter__std::vec::Vec<i32>(_22);
return bb16();
}
function bb16() {
_29.assign(_28);
return bb17();
}
function bb17() {
_31.assign(_ref(_29));
_30=std__iter__Iterator__next(_31);
return bb18();
}
function bb18() {
_32.assign(discriminant(_30));
switch (switchInt(_32)) {
case 0:return bb20();
case 1:return bb19();
default: return bb6();
}
}
function bb19() {
_33.assign(_30.field(0));
_37.assign(_ref(_33));
_36.assign(new Tuple([_37]));
_61.assign(_36.field(0));
_39=core__fmt__rt__Argument__new_display(_61);
return bb21();
}
function bb20() {
return bb23();
}
function bb21() {
_38.assign(new Array([_39]));
_40.assign(main__promoted_1);
_41.assign(_ref(_38));
_35=core__fmt__rt__Arguments__new_v1(_40, _41);
return bb22();
}
function bb22() {
_34=std__io__print(_35);
return bb39();
}
function bb23() {
_44.assign(main__promoted_3);
_43=core__fmt__rt__Arguments__new_const(_44);
return bb24();
}
function bb24() {
_42=std__io__print(_43);
return bb25();
}
function bb25() {
_45.assign([new Int32(0)]);
_46=std__iter__IntoIterator__into_iter__A(_45);
return bb26();
}
function bb26() {
_47.assign(_46);
return bb27();
}
function bb27() {
_49.assign(_ref(_47));
_48=std__iter__Iterator__next(_49);
return bb28();
}
function bb28() {
_50.assign(discriminant(_48));
switch (switchInt(_50)) {
case 0:return bb30();
case 1:return bb29();
default: return bb6();
}
}
function bb29() {
_51.assign(_48.field(0));
_55.assign(_ref(_51));
_54.assign(new Tuple([_55]));
_62.assign(_54.field(0));
_57=core__fmt__rt__Argument__new_display(_62);
return bb31();
}
function bb30() {
return;
}
function bb31() {
_56.assign(new Array([_57]));
_58.assign(main__promoted_2);
_59.assign(_ref(_56));
_53=core__fmt__rt__Arguments__new_v1(_58, _59);
return bb32();
}
function bb32() {
_52=std__io__print(_53);
return bb27();
}
function bb33() {
return bb35();
}
function bb34() {
return bb35();
}
function bb35() {
// UnwindResume
}
function bb36() {
_70.assign(_63);
_71.assign(_70);
_72.assign(12);
_73.assign(_ne(_72, new Uint64(0)));
_74.assign(_eq(_71, new Uint64(0)));
_75.assign(_and(_74, _73));
_76.assign(_not(_75));
if (_eq(_76, true)) {
return bb37();
} else {
throw new Error('assert failed: NullPointerDereference');
}
}
function bb37() {
_63.deref().assign(new Array([new Int32(1), new Int32(2), new Int32(3)]));
_23.assign(_27);
_22=std__slice__impl__T__into_vec(_23);
return bb15();
}
function bb38() {
return bb4();
}
function bb39() {
return bb17();
}
bb0();
return _0;
}
const main__promoted_0 = (() => {
let _0 = new RefVar(memory.stackAlloc(8), 8);let _1 = new NoRefVar(memory.stackAlloc(32), 32);function bb0() {
_1.assign(new Array([new Uint8Array([]), new Uint8Array([10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
const main__promoted_1 = (() => {
let _0 = new RefVar(memory.stackAlloc(8), 8);let _1 = new NoRefVar(memory.stackAlloc(32), 32);function bb0() {
_1.assign(new Array([new Uint8Array([]), new Uint8Array([10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
const main__promoted_2 = (() => {
let _0 = new RefVar(memory.stackAlloc(8), 8);let _1 = new NoRefVar(memory.stackAlloc(32), 32);function bb0() {
_1.assign(new Array([new Uint8Array([]), new Uint8Array([10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
const main__promoted_3 = (() => {
let _0 = new RefVar(memory.stackAlloc(8), 8);let _1 = new NoRefVar(memory.stackAlloc(16), 16);function bb0() {
_1.assign(new Array([new Uint8Array([105, 116, 101, 114, 97, 116, 111, 114, 10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
const main__promoted_4 = (() => {
let _0 = new RefVar(memory.stackAlloc(8), 8);let _1 = new NoRefVar(memory.stackAlloc(16), 16);function bb0() {
_1.assign(new Array([new Uint8Array([118, 101, 99, 10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
const main__promoted_5 = (() => {
let _0 = new RefVar(memory.stackAlloc(8), 8);let _1 = new NoRefVar(memory.stackAlloc(16), 16);function bb0() {
_1.assign(new Array([new Uint8Array([97, 114, 114, 97, 121, 10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
main();

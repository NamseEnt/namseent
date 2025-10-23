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
let _0 = new NoRefVar(sizeof(0));
let _1 = new NoRefVar(sizeof(0));
let _2 = new RefVar(sizeof(8));
let _3 = new RefVar(sizeof(4));
let _4 = new NoRefVar(sizeof(0));
let _5 = new RefVar(sizeof(8));
let _6 = new RefVar(sizeof(4));
function bb0() {
_3.assign(main__promoted_1);
_2.assign(_3);
_1=foo_ty_u8(_2);
return bb1();
}
function bb1() {
_6.assign(main__promoted_0);
_5.assign(_6);
_4=foo_ty_u16(_5);
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
const main__promoted_0 = (() => {
let _0 = new RefVar(sizeof(4));
let _1 = new NoRefVar(sizeof(6));
function bb0() {
_1.assign(new Array([new Uint16(1), new Uint16(2), new Uint16(3)]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
const main__promoted_1 = (() => {
let _0 = new RefVar(sizeof(4));
let _1 = new NoRefVar(sizeof(3));
function bb0() {
_1.assign(new Array([new Uint8(1), new Uint8(2), new Uint8(3)]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function foo_ty_u16() {
let _0 = new NoRefVar(sizeof(0));
let _1 = new RefVar(sizeof(8));
let _2 = new NoRefVar(sizeof(6));
let _3 = new NoRefVar(sizeof(2));
let _4 = new NoRefVar(sizeof(4));
let _5 = new NoRefVar(sizeof(4));
let _6 = new NoRefVar(sizeof(1));
let _7 = new NoRefVar(sizeof(4));
let _8 = new NoRefVar(sizeof(2));
let _9 = new NoRefVar(sizeof(4));
let _10 = new NoRefVar(sizeof(4));
let _11 = new NoRefVar(sizeof(1));
let _12 = new NoRefVar(sizeof(2));
let _13 = new NoRefVar(sizeof(4));
let _14 = new NoRefVar(sizeof(4));
let _15 = new NoRefVar(sizeof(1));
function bb0() {
_4.assign(new Uint32(0));
_5.assign(_ptr_metadata(_1));
_6.assign(_lt(_4, _5));
if (_eq(_6, true)) {
return bb1();
} else {
throw new Error('assert failed: BoundsCheck { len: move _5, index: copy _4 }');
}
}
function bb1() {
_3.assign(_1.deref().index(_4));
_9.assign(new Uint32(1));
_10.assign(_ptr_metadata(_1));
_11.assign(_lt(_9, _10));
if (_eq(_11, true)) {
return bb2();
} else {
throw new Error('assert failed: BoundsCheck { len: move _10, index: copy _9 }');
}
}
function bb2() {
_8.assign(_1.deref().index(_9));
_13.assign(new Uint32(2));
_14.assign(_ptr_metadata(_1));
_15.assign(_lt(_13, _14));
if (_eq(_15, true)) {
return bb3();
} else {
throw new Error('assert failed: BoundsCheck { len: move _14, index: copy _13 }');
}
}
function bb3() {
_12.assign(_1.deref().index(_13));
_7.assign(new Tuple([_8, _12]));
_2.assign([_3, _7]);
return;
}
bb0();
return _0;
}

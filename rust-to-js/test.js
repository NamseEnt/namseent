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
let _2 = new NoRefVar(sizeof(0));
let _3 = new RefVar(sizeof(4));
function bb0() {
_3.assign(_ref(_1));
_2=main____closure__0___ty_i8___fn_ptr___ty_tuple(_3, new Tuple([]));
return bb1();
}
function bb1() {
return;
}
bb0();
return _0;
}
function main____closure__0___ty_i8___fn_ptr___ty_tuple() {
let _0 = new NoRefVar(sizeof(0));
let _1 = new RefVar(sizeof(4));
let _2 = new NoRefVar(sizeof(0));
let _3 = new NoRefVar(sizeof(24));
let _4 = new RefVar(sizeof(4));
function bb0() {
_4.assign(main__promoted_0);
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
let _0 = new RefVar(sizeof(4));
let _1 = new NoRefVar(sizeof(8));
function bb0() {
_1.assign(new Array([new Uint8Array([104, 101, 108, 108, 111, 10])]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize() {
let _0 = new NoRefVar(sizeof(24));
let _1 = new RefVar(sizeof(4));
let _2 = new RefVar(sizeof(8));
let _3 = new NoRefVar(sizeof(8));
let _4 = new RefVar(sizeof(8));
let _5 = new RefVar(sizeof(4));
function bb0() {
_2.assign(_1);
_3.assign(new Enum(undefined, 0));
_5.assign(main__promoted_0);
_4.assign(_5);
_0.assign([_2, _3, _4]);
return;
}
bb0();
return _0;
}
const core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0 = (() => {
let _0 = new RefVar(sizeof(4));
let _1 = new NoRefVar(sizeof(0));
function bb0() {
_1.assign(new Array([]));
_0.assign(_ref(_1));
return;
}
bb0();
return _0;
})();
main();

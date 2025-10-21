class Ptr {
  constructor(value) {
    this.value = value;
  }
}

class UninitPtr {
  assign(value) {
    return new Ptr(value);
  }
}

class Adt {
  constructor(value) {
    this.value = value;
  }
}

class UninitAdt {
  assign(value) {
    return new Adt(value);
  }
}

function core__fmt__rt__Argument__new_display(arg) {
  return arg.toString();
}
function core__fmt__rt__Arguments__new_v1(format_strings, args) {
  return { format_strings, args };
}
function core__fmt__rt__Arguments__new_const(format_strings) {
  return { format_strings, args: [] };
}
function std__io__print(args) {
  let output = "";
  for (let i = 0; i < args.format_strings.length; i++) {
    output += args.format_strings[i];
    if (i < args.args.length) {
      output += args.args[i];
    }
  }
  console.log(output);
}
class Enum {
  constructor(value, discriminant) {
    this[0] = value;
    this.discriminant = discriminant;
  }
}
function std__iter__IntoIterator__into_iter(iter) {
  if (Array.isArray(iter)) {
    let i = 0;
    return {
      next() {
        if (i < iter.length) {
          return new Enum(iter[i++], 1);
        }
        return new Enum(undefined, 0);
      },
    };
  }
  return iter.intoIter();
}
function std__iter__Iterator__next(arg) {
  return arg.next();
}
function discriminant(arg) {
  if (!(arg instanceof Enum)) {
    throw new Error("arg is not enum");
  }
  return arg.discriminant;
}
function switchInt(value) {
  if (typeof value === "number") {
    return value;
  }
  throw new Error();
}
let nextPtr = 0;
class AllocatedMemory {
  constructor(ptr) {
    this[0] = ptr;
  }
}
function alloc__alloc__exchange_malloc(size, align) {
  const ptr = nextPtr + (nextPtr % align);
  nextPtr = ptr + size;
  return new AllocatedMemory(ptr);
}

class Vec {
  constructor(slice) {
    this.len = slice.length;
    this.values = slice;
  }
  intoIter() {
    let i = 0;
    return {
      next() {
        if (i < this.len) {
          return new Enum(this.values[i++], 1);
        }
        return new Enum(undefined, 0);
      },
    };
  }
}

function std__slice__impl__T__into_vec(slice) {
  return new Vec(slice);
}

// ====
function __A__as__std__iter__Iterator____next() {
let _0 = new UninitAdt();
let _1 = new UninitPtr();
let _2 = false;
let _3 = 0;
let _4 = 0;
let _5 = [];
function bb0() {
_3 = _1[0];
_2 = _3>3;
switch (switchInt(_2)) {
case 0:return bb2();
default: return bb1();
}
}
function bb1() {
_0 = new Enum(undefined, 0);
return bb4();
}
function bb2() {
_4 = _1[0];
_5 = _1[0]+1;
if (_5[1]=== false) {
return bb3();
} else {
throw new Error('assert failed: Overflow(Add, copy ((*_1).0: i32), const 1_i32)');
}
}
function bb3() {
_1.deref()[0] = _5[0];
_0 = new Enum(_4, 1);
return bb4();
}
function bb4() {
return;
}
bb0();
return _0;
}
function main() {
let _0 = [];
let _1 = [];
let _2 = [];
let _3 = new UninitAdt();
let _4 = new UninitPtr();
let _5 = new UninitAdt();
let _6 = new UninitAdt();
let _7 = new UninitAdt();
let _8 = new UninitPtr();
let _9 = 0;
let _10 = 0;
let _11 = [];
let _12 = new UninitAdt();
let _13 = [];
let _14 = new UninitPtr();
let _15 = [];
let _16 = new UninitAdt();
let _17 = new UninitPtr();
let _18 = new UninitPtr();
let _19 = [];
let _20 = new UninitAdt();
let _21 = new UninitPtr();
let _22 = new UninitAdt();
let _23 = new UninitAdt();
let _24 = 0;
let _25 = 0;
let _26 = new UninitPtr();
let _27 = new UninitAdt();
let _28 = new UninitAdt();
let _29 = new UninitAdt();
let _30 = new UninitAdt();
let _31 = new UninitPtr();
let _32 = 0;
let _33 = 0;
let _34 = [];
let _35 = new UninitAdt();
let _36 = [];
let _37 = new UninitPtr();
let _38 = [];
let _39 = new UninitAdt();
let _40 = new UninitPtr();
let _41 = new UninitPtr();
let _42 = [];
let _43 = new UninitAdt();
let _44 = new UninitPtr();
let _45 = new UninitAdt();
let _46 = new UninitAdt();
let _47 = new UninitAdt();
let _48 = new UninitAdt();
let _49 = new UninitPtr();
let _50 = 0;
let _51 = 0;
let _52 = [];
let _53 = new UninitAdt();
let _54 = [];
let _55 = new UninitPtr();
let _56 = [];
let _57 = new UninitAdt();
let _58 = new UninitPtr();
let _59 = new UninitPtr();
let _60 = new UninitPtr();
let _61 = new UninitPtr();
let _62 = new UninitPtr();
let _63 = new UninitPtr();
let _64 = new UninitPtr();
let _65 = 0;
let _66 = 0;
let _67 = 0;
let _68 = 0;
let _69 = false;
let _70 = new UninitPtr();
let _71 = 0;
let _72 = 0;
let _73 = false;
let _74 = false;
let _75 = false;
let _76 = false;
function bb0() {
_1 = [1, 2, 3];
_4 = main__promoted_5;
_3 = core__fmt__rt__Arguments__new_const(_4);
return bb1();
}
function bb1() {
_2 = std__io__print(_3);
return bb2();
}
function bb2() {
_5 = std__iter__IntoIterator__into_iter(_1);
return bb3();
}
function bb3() {
_6 = _5;
return bb4();
}
function bb4() {
_8 = _6;
_7 = std__iter__Iterator__next(_8);
return bb5();
}
function bb5() {
_9 = discriminant(_7);
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
_10 = _7[0];
_14 = _10;
_13 = [_14];
_60 = _13[0];
_16 = core__fmt__rt__Argument__new_display(_60);
return bb9();
}
function bb8() {
return bb11();
}
function bb9() {
_15 = [_16];
_17 = main__promoted_0;
_18 = _15;
_12 = core__fmt__rt__Arguments__new_v1(_17, _18);
return bb10();
}
function bb10() {
_11 = std__io__print(_12);
return bb38();
}
function bb11() {
_21 = main__promoted_4;
_20 = core__fmt__rt__Arguments__new_const(_21);
return bb12();
}
function bb12() {
_19 = std__io__print(_20);
return bb13();
}
function bb13() {
_24 = 12;
_25 = 12;
_26 = alloc__alloc__exchange_malloc(_24, _25);
return bb14();
}
function bb14() {
_27 = _26;
_63 = _27[0][0];
_64 = _63;
_65 = _64;
_66 = 12;
_67 = _66-1;
_68 = _65&_67;
_69 = _68==0;
if (_69=== true) {
return bb36();
} else {
throw new Error('assert failed: MisalignedPointerDereference { required: copy _66, found: copy _65 }');
}
}
function bb15() {
_28 = std__iter__IntoIterator__into_iter(_22);
return bb16();
}
function bb16() {
_29 = _28;
return bb17();
}
function bb17() {
_31 = _29;
_30 = std__iter__Iterator__next(_31);
return bb18();
}
function bb18() {
_32 = discriminant(_30);
switch (switchInt(_32)) {
case 0:return bb20();
case 1:return bb19();
default: return bb6();
}
}
function bb19() {
_33 = _30[0];
_37 = _33;
_36 = [_37];
_61 = _36[0];
_39 = core__fmt__rt__Argument__new_display(_61);
return bb21();
}
function bb20() {
return bb23();
}
function bb21() {
_38 = [_39];
_40 = main__promoted_1;
_41 = _38;
_35 = core__fmt__rt__Arguments__new_v1(_40, _41);
return bb22();
}
function bb22() {
_34 = std__io__print(_35);
return bb39();
}
function bb23() {
_44 = main__promoted_3;
_43 = core__fmt__rt__Arguments__new_const(_44);
return bb24();
}
function bb24() {
_42 = std__io__print(_43);
return bb25();
}
function bb25() {
_45 = [0];
_46 = std__iter__IntoIterator__into_iter(_45);
return bb26();
}
function bb26() {
_47 = _46;
return bb27();
}
function bb27() {
_49 = _47;
_48 = std__iter__Iterator__next(_49);
return bb28();
}
function bb28() {
_50 = discriminant(_48);
switch (switchInt(_50)) {
case 0:return bb30();
case 1:return bb29();
default: return bb6();
}
}
function bb29() {
_51 = _48[0];
_55 = _51;
_54 = [_55];
_62 = _54[0];
_57 = core__fmt__rt__Argument__new_display(_62);
return bb31();
}
function bb30() {
return;
}
function bb31() {
_56 = [_57];
_58 = main__promoted_2;
_59 = _56;
_53 = core__fmt__rt__Arguments__new_v1(_58, _59);
return bb32();
}
function bb32() {
_52 = std__io__print(_53);
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
_70 = _63;
_71 = _70;
_72 = 12;
_73 = _72!=0;
_74 = _71==0;
_75 = _74&_73;
_76 = !_75;
if (_76=== true) {
return bb37();
} else {
throw new Error('assert failed: NullPointerDereference');
}
}
function bb37() {
_63.derefAssign([1, 2, 3]);
_23 = _27;
_22 = std__slice__impl__T__into_vec(_23);
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
let _0 = new UninitPtr();
let _1 = [];
function bb0() {
_1 = ["", "\n"];
_0 = _1;
return;
}
bb0();
return _0;
})();
const main__promoted_1 = (() => {
let _0 = new UninitPtr();
let _1 = [];
function bb0() {
_1 = ["", "\n"];
_0 = _1;
return;
}
bb0();
return _0;
})();
const main__promoted_2 = (() => {
let _0 = new UninitPtr();
let _1 = [];
function bb0() {
_1 = ["", "\n"];
_0 = _1;
return;
}
bb0();
return _0;
})();
const main__promoted_3 = (() => {
let _0 = new UninitPtr();
let _1 = [];
function bb0() {
_1 = ["iterator\n"];
_0 = _1;
return;
}
bb0();
return _0;
})();
const main__promoted_4 = (() => {
let _0 = new UninitPtr();
let _1 = [];
function bb0() {
_1 = ["vec\n"];
_0 = _1;
return;
}
bb0();
return _0;
})();
const main__promoted_5 = (() => {
let _0 = new UninitPtr();
let _1 = [];
function bb0() {
_1 = ["array\n"];
_0 = _1;
return;
}
bb0();
return _0;
})();
main();

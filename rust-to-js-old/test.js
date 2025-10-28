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
    const _0 = stackAlloc(0);
    const _1 = stackAlloc(0);
    const _2 = stackAlloc(0);
    const _3 = stackAlloc(4);
    function bb0() {
        _3.assign(_1.ptr);
        assign(_2, __StructA__as__First____print(_3));
        bb1();
    }
    function bb1() {
        // Return
    }
    bb0();
    stackDealloc(4);
    return _0;
}
function __StructA__as__First____print() {
    const _0 = stackAlloc(0);
    const _1 = stackAlloc(4);
    const _2 = stackAlloc(0);
    const _3 = stackAlloc(24);
    const _4 = stackAlloc(4);
    function bb0() {
        _4.assign(__StructA__as__First____print__promoted_0);
        assign(_3, core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize(_4));
        bb1();
    }
    function bb1() {
        assign(_2, std__io___print(_3));
        bb2();
    }
    function bb2() {
        // Return
    }
    bb0();
    stackDealloc(32);
    return _0;
}
const __StructA__as__First____print__promoted_0 = (() => {
    const _0 = stackAlloc(4);
    const _1 = stackAlloc(8);
    function bb0() {
        _1.assign(new Array([new Uint8Array([83, 116, 114, 117, 99, 116, 65, 10])]));
        _0.assign(_1.ptr);
        // Return
    }
    bb0();
    stackDealloc(12);
    return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize() {
    const _0 = stackAlloc(24);
    const _1 = stackAlloc(4);
    const _2 = stackAlloc(8);
    const _3 = stackAlloc(8);
    function bb0() {
        _2.assign(_1);
        _3.assign(core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0);
        _0.assign([_2, _indirect(4, 0), _3]);
        // Return
    }
    bb0();
    stackDealloc(44);
    return _0;
}
const core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0 = (() => {
    const _0 = stackAlloc(4);
    const _1 = stackAlloc(0);
    function bb0() {
        _1.assign(new Array([]));
        _0.assign(_1.ptr);
        // Return
    }
    bb0();
    stackDealloc(4);
    return _0;
})();
main();

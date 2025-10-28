import {
    stackPush,
    stackAlloc,
    assign,
    stackAllocArray,
    stackAllocBytes,
    stackPop,
    stackAllocInt8,
    stackAllocInt16,
    stackAllocInt32,
    stackAllocUint8,
    stackAllocUint16,
    stackAllocUint32,
    std__io___print,
} from "./impl.ts";

// ====
function main() {
    stackPush();
    const _0 = stackAlloc(0);
    const _1 = stackAlloc(4);
    const _2 = stackAlloc(12);
    const _3 = stackAlloc(4);
    const _4 = stackAlloc(0);
    const _5 = stackAlloc(0);
    const _6 = stackAlloc(4);
    const _7 = stackAlloc(0);
    const _8 = stackAlloc(4);
    function bb0() {
        assign(_3, stackAllocInt32(4));
        assign(_2, stackAllocArray([_3, stackAllocInt32(2), stackAllocInt32(3)]));
        assign(_1, _2.ptr);
        assign(_6, _4.ptr);
        assign(_5, __StructA__as__First____print(_6));
        bb1();
    }
    function bb1() {
        assign(_8, _4.ptr);
        assign(_7, __StructA__as__FirstB____print(_8));
        bb2();
    }
    function bb2() {
        // Return
    }
    bb0();
    stackPop();
    return _0;
}
function __StructA__as__FirstB____print(arg0) {
    stackPush();
    const _0 = stackAlloc(0);
    const _1 = stackAlloc(4);
    assign(_1, arg0);
    const _2 = stackAlloc(0);
    const _3 = stackAlloc(24);
    const _4 = stackAlloc(4);
    function bb0() {
        assign(_4, __StructA__as__FirstB____print__promoted_0);
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
    stackPop();
    return _0;
}
const __StructA__as__FirstB____print__promoted_0 = (() => {
    stackPush();
    const _0 = stackAlloc(4);
    const _1 = stackAlloc(8);
    function bb0() {
        assign(_1, stackAllocArray([stackAllocBytes(new Uint8Array([83, 116, 114, 117, 99, 116, 65, 10]))]));
        assign(_0, _1.ptr);
        // Return
    }
    bb0();
    stackPop();
    return _0;
})();
function core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize(arg0) {
    stackPush();
    const _0 = stackAlloc(24);
    const _1 = stackAlloc(4);
    assign(_1, arg0);
    const _2 = stackAlloc(8);
    const _3 = stackAlloc(8);
    function bb0() {
        assign(_2, _1);
        assign(_3, core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0);
        assign(_0, [_2, stackAllocBytes(new Uint8Array([0, 0, 0, 0, 0, 0, 0, 0])), _3]);
        // Return
    }
    bb0();
    stackPop();
    return _0;
}
const core__fmt__rt____impl__std__fmt__Arguments____a______new_const1_usize__promoted_0 = (() => {
    stackPush();
    const _0 = stackAlloc(4);
    const _1 = stackAlloc(0);
    function bb0() {
        assign(_1, stackAllocArray([]));
        assign(_0, _1.ptr);
        // Return
    }
    bb0();
    stackPop();
    return _0;
})();
function __StructA__as__First____print(arg0) {
    stackPush();
    const _0 = stackAlloc(0);
    const _1 = stackAlloc(4);
    assign(_1, arg0);
    const _2 = stackAlloc(0);
    const _3 = stackAlloc(24);
    const _4 = stackAlloc(4);
    function bb0() {
        assign(_4, __StructA__as__First____print__promoted_0);
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
    stackPop();
    return _0;
}
const __StructA__as__First____print__promoted_0 = (() => {
    stackPush();
    const _0 = stackAlloc(4);
    const _1 = stackAlloc(8);
    function bb0() {
        assign(_1, stackAllocArray([stackAllocBytes(new Uint8Array([83, 116, 114, 117, 99, 116, 65, 10]))]));
        assign(_0, _1.ptr);
        // Return
    }
    bb0();
    stackPop();
    return _0;
})();
main();

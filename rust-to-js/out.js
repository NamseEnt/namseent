const memory = new Uint8Array(1024 * 1024);
let stackLocalPtr = 0;

function ops_alloca(size) {
    const out = stackLocalPtr;
    stackLocalPtr += size;
    return out;
}

/**
 *
 * @param {Uint8Array} value
 * @param {number} ptr
 */
function ops_store(value, ptr) {
    memory.set(value, ptr);
}

function _ZN3std2rt10lang_start17h7700b2f3503e1ed4E(main, argc, argv, sigpipe) {
    function start() {
        _7 = ops_alloca(4);
        ops_store(new Uint32Array([main]), _7);
        _0 = _ZN3std2rt19lang_start_internal17h9224f8262f833227E(
            _7,
            vtable_0,
            argc,
            argv,
            sigpipe,
        );
        return _0;
    }
    let _0;
    let _7;
    start();
}
function _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef07beedc0148da2E(
    _1,
) {
    function start() {
        _4 = ops_load(_1, 4);
        _ZN3std3sys9backtrace28__rust_begin_short_backtrace17hfce07c24d14b4404E(
            _4,
        );
        self =
            _ZN54_$LT$$LP$$RP$$u20$as$u20$std__process__Termination$GT$6report17h7bb77f510928e009E();
        _0 = ops_zext(self, 8, 32);
        return _0;
    }
    let _0;
    let _4;
    let self;
    start();
}
function _ZN3std3sys9backtrace28__rust_begin_short_backtrace17hfce07c24d14b4404E(
    f,
) {
    function start() {
        _ZN4core3ops8function6FnOnce9call_once17h8c48ca95caea87f5E(f);
        l0();
        return;
    }
    start();
}
function _ZN4core3fmt2rt38_$LT$impl$u20$core__fmt__Arguments$GT$9new_const17h3a30e415830240daE(
    _0,
    pieces,
) {
    function start() {
        ops_store(new Uint32Array([pieces]), _0);
        ops_store(new Uint32Array([1]), l0);
        l1 = ops_load(anon_f52434a2809397b1abb34e52430ce470_0, 4);
        l2 = ops_load(l3, 4);
        ops_store(new Uint32Array([ll1]), l4);
        ops_store(new Uint32Array([ll2]), l5);
        ops_store(new Uint32Array([l6]), l7);
        ops_store(new Uint32Array([0]), l8);
        return;
    }
    let l1;
    let l2;
    start();
}
function _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable_shim$u7d$$u7d$17he48851c116189f1aE(
    _1,
) {
    function start() {
        _2 = ops_alloca(0);
        l0 = ops_load(_1, 4);
        _0 = _ZN4core3ops8function6FnOnce9call_once17h4554aeccf940ab42E(ll0);
        return _0;
    }
    let _0;
    let _2;
    let l0;
    start();
}
function _ZN4core3ops8function6FnOnce9call_once17h4554aeccf940ab42E(l0) {
    function start() {
        _2 = ops_alloca(0);
        _1 = ops_alloca(4);
        ops_store(new Uint32Array([ll0]), _1);
        _0 =
            _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hef07beedc0148da2E(
                _1,
            );
        return _0;
    }
    let _0;
    let _1;
    let _2;
    start();
}
function _ZN4core3ops8function6FnOnce9call_once17h8c48ca95caea87f5E(_1) {
    function start() {
        _2 = ops_alloca(0);
        _1();
        return;
    }
    let _2;
    start();
}
function _ZN54_$LT$$LP$$RP$$u20$as$u20$std__process__Termination$GT$6report17h7bb77f510928e009E() {
    function start() {
        return 0;
    }
    start();
}
function _ZN11hello_world4main17h64a202c57002d659E() {
    function start() {
        _2 = ops_alloca(24);
        _ZN4core3fmt2rt38_$LT$impl$u20$core__fmt__Arguments$GT$9new_const17h3a30e415830240daE(
            _2,
            alloc_9b968e9d68758268e4a8d45e405f65d0,
        );
        _ZN3std2io5stdio6_print17h71059b9ed4cc355dE(_2);
        return;
    }
    let _2;
    start();
}
function __main_void() {
    function top() {
        l0 = _ZN3std2rt10lang_start17h7700b2f3503e1ed4E(
            _ZN11hello_world4main17h64a202c57002d659E,
            0,
            null,
            0,
        );
        return ll0;
    }
    let l0;
    top();
}
__main_void();

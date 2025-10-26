function main() {
  let _0;
  let _1;
  let _2;
  let _3;
  function bb0() {
    _3 = main__promoted_0;
    _2 = core__fmt__rt__Arguments__new_const(_3);
    return bb1();
  }
  function bb1() {
    _1 = std__io__print(_2);
    return bb2();
  }
  function bb2() {
    return;
  }
  bb0();
  return _0;
}
const main__promoted_0 = (() => {
  let _0;
  let _1;
  function bb0() {
    _1 = ["hello world!\n"];
    _0 = _1;
    return;
  }
  bb0();
  return _0;
})();
main();

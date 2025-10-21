function main() {
  let _0;
  let _1;
  let _2;
  let _3;
  let _4;
  let _5;
  let _6;
  let _7;
  let _8;
  let _9;
  let _10;
  let _11;
  let _12;
  let _13;
  let _14;
  let _15;
  let _16;
  let _17;
  let _18;
  function bb0() {
    _1 = 5;
    _2 = "abc";
    _3 = true;
    _7 = _1;
    _8 = _2;
    _9 = _3;
    _6 = [_7, _8, _9];
    _16 = _6[0];
    _11 = core__fmt__rt__Argument__new_display(_16);
    return bb1();
  }
  function bb1() {
    _17 = _6[1];
    _12 = core__fmt__rt__Argument__new_display(_17);
    return bb2();
  }
  function bb2() {
    _18 = _6[2];
    _13 = core__fmt__rt__Argument__new_display(_18);
    return bb3();
  }
  function bb3() {
    _10 = [_11, _12, _13];
    _14 = main__promoted_0;
    _15 = _10;
    _5 = core__fmt__rt__Arguments__new_v1(_14, _15);
    return bb4();
  }
  function bb4() {
    _4 = std__io__print(_5);
    return bb5();
  }
  function bb5() {
    return;
  }
  bb0();
  return _0;
}
const main__promoted_0 = (() => {
  let _0;
  let _1;
  function bb0() {
    _1 = ["a: ", " b: ", " c: ", "\n"];
    _0 = _1;
    return;
  }
  bb0();
  return _0;
})();
main();

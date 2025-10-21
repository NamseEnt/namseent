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
  function bb0() {
  _2 = ("hello");
  return bb1();
  }
  function bb1() {
  _1 = [1, _2, 2.2999999999999998];
  _6 = _1[0];
  _7 = _1[1];
  _8 = _1[2];
  _5 = [_6, _7, _8];
  _15 = _5[0];
  _10 = core__fmt__rt__Argument__new_display(_15);
  return bb2();
  }
  function bb2() {
  _16 = _5[1];
  _11 = core__fmt__rt__Argument__new_display(_16);
  return bb3();
  }
  function bb3() {
  _17 = _5[2];
  _12 = core__fmt__rt__Argument__new_display(_17);
  return bb4();
  }
  function bb4() {
  _9 = [_10, _11, _12];
  _13 = main__promoted_0;
  _14 = _9;
  _4 = core__fmt__rt__Arguments__new_v1(_13, _14);
  return bb5();
  }
  function bb5() {
  _3 = std__io__print(_4);
  return bb6();
  }
  function bb6() {
  return bb7();
  }
  function bb7() {
  return;
  }
  function bb8() {
  return bb9();
  }
  function bb9() {
  // UnwindResume
  }
  bb0();
  return _0;
  }
  const main__promoted_0 = (() => {
  let _0;
  let _1;
  function bb0() {
  _1 = ["", " ", " ", "\n"];
  _0 = _1;
  return;
  }
  bb0();
  return _0;
  })();
  main();
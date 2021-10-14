import {
  ColorUtil,
  Language,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  TextInput,
  Selection,
  Rect,
  AfterDraw,
  Translate,
} from "namui";

type State = {
  text: string;
  focus: boolean;
  selection?: Selection;
};

export function render(state: State): RenderingTree {
  return [
    Text({
      x: 100,
      y: 100,
      align: TextAlign.left,
      baseline: TextBaseline.bottom,
      fontType: {
        language: Language.ko,
        serif: false,
        size: 20,
      },
      style: {
        color: ColorUtil.Black,
      },
      text: "test 123 abc",
    }),
    Rect({
      x: 100,
      y: 100,
      width: 100,
      height: 100,
      style: {
        stroke: {
          color: ColorUtil.Blue,
          width: 1,
        },
      },
    }),
    TextInput({
      text: state.text,
      focus: state.focus,
      selection: state.selection,
      x: 200,
      y: 100,
      width: 200,
      height: 50,
      onClick: () => {
        state.focus = true; // TODO : Out focus = onOutClick
      },
      onClickOut: () => {
        state.focus = false;
        state.selection = undefined;
      },
      onChange: ({ text, selection }) => {
        console.warn("onChange", selection);
        state.text = text;
        state.selection = selection;
      },
      align: TextAlign.left,
      baseline: TextBaseline.top,
      fontType: {
        serif: false,
        size: 32,
        language: Language.ko,
      },
      rectStyle: {
        stroke: {
          color: ColorUtil.Black,
          width: 1,
        },
      },
      style: {
        color: ColorUtil.Black,
      },
    }),
    testAfterDraw(),
  ];
}

function testAfterDraw() {
  const x = 100;
  const y = 50;
  return Translate(
    {
      x,
      y,
    },
    [
      AfterDraw(({ translated }) => {
        if (translated.x !== x || translated.y !== y) {
          throw new Error(
            `translated is not same! expected: ${JSON.stringify({
              x,
              y,
            })} but ${translated.toString()}`,
          );
        }
      }),
    ],
  );
}

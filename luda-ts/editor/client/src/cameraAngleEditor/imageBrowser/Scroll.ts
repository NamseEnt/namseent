import {
  BorderPosition,
  Clip,
  ColorUtil,
  engine,
  Mathu,
  Rect,
  Render,
  RenderingTree,
  Translate,
} from "namui";

export type ScrollState = {
  scrollY: number;
};

export type ScrollProps = {
  layout: {
    x: number;
    y: number;
    innerWidth: number;
    scrollBarWidth: number;
    height: number;
    innerHeight: number;
  };
  innerRenderingTree: RenderingTree;
};

export const Scroll: Render<ScrollState, ScrollProps> = (state, props) => {
  state.scrollY = Mathu.clamp(
    state.scrollY,
    0,
    Math.max(0, props.layout.innerHeight - props.layout.height),
  );

  const inner = Clip(
    {
      path: new CanvasKit.Path().addRect(
        CanvasKit.XYWHRect(0, 0, props.layout.innerWidth, props.layout.height),
      ),
      clipOp: CanvasKit.ClipOp.Intersect,
    },
    [
      Translate(
        {
          x: 0,
          y: -state.scrollY,
        },
        [props.innerRenderingTree as RenderingTree],
      ),
    ],
  );

  const scrollBarHandleHeight =
    props.layout.height ** 2 / props.layout.innerHeight;
  const scrollBar = props.layout.innerHeight > props.layout.height && [
    Rect({
      x: props.layout.innerWidth,
      y: 0,
      width: props.layout.scrollBarWidth,
      height: props.layout.height,
      style: {
        stroke: {
          width: 1,
          borderPosition: BorderPosition.inside,
          color: ColorUtil.Black,
        },
        fill: {
          color: ColorUtil.White,
        },
      },
    }),
    Rect({
      x: props.layout.innerWidth,
      y: state.scrollY,
      width: props.layout.scrollBarWidth,
      height: scrollBarHandleHeight,
      style: {
        fill: {
          color: ColorUtil.Grayscale01(0.5),
        },
      },
    }),
  ];
  const wholeRect = Rect({
    x: 0,
    y: 0,
    width: props.layout.innerWidth + props.layout.scrollBarWidth,
    height: props.layout.height,
    style: {
      stroke: {
        borderPosition: BorderPosition.middle,
        color: ColorUtil.Black,
        width: 1,
      },
    },
    onAfterDraw(id) {
      engine.wheel.onWheel((event) => {
        if (
          engine.render.isGlobalVectorOutOfRenderingData(
            engine.mousePosition.mousePosition,
            id,
          )
        ) {
          return;
        }
        const nextScrollY = state.scrollY + event.deltaY;
        state.scrollY = Mathu.clamp(
          nextScrollY,
          0,
          props.layout.innerHeight - props.layout.height,
        );
      });
    },
  });

  return [
    Translate(
      {
        ...props.layout,
      },
      [wholeRect, inner, scrollBar],
    ),
  ];
};

import { Rect, RenderingTree, Translate, XywhRect } from "namui";

export function renderSlider(props: {
  layout: XywhRect;
  value: number;
  min: number;
  max: number;
  style: {
    thumb: {
      stroke: {
        width: number;
        color: Float32Array;
      };
      fill: {
        color: Float32Array;
      };
    };
    background: {
      fill: {
        color: Float32Array;
      };
    };
  };
  onChange: (value: number) => void;
}): RenderingTree {
  const thumbPosition = Math.max(
    0,
    Math.min((props.value - props.min) / (props.max - props.min), 1),
  );
  return Translate(props.layout, [
    Rect({
      ...props.layout,
      x: 0,
      y: 0,
      style: props.style.background,
      onMouseMoveIn: (event) => {
        if (!event.isLeftButtonDown) {
          return;
        }
        const newThumbPosition = Math.max(
          0,
          Math.min(event.translated.x / props.layout.width, 1),
        );
        props.onChange((props.max - props.min) * newThumbPosition + props.min);
      },
    }),
    Rect({
      x: thumbPosition * props.layout.width - 2,
      y: 0,
      width: 4,
      height: props.layout.height,
      style: props.style.thumb,
    }),
  ]);
}

import { ColorUtil, RenderingTree, Selection, Rect, Translate } from "namui";

export function renderWithMouseEvent(): RenderingTree {
  return [
    renderRect(),
    Translate({ x: 100, y: 100 }, [renderRect()]),
    renderRect(renderRect()),
    renderRect(Translate({ x: 100, y: 100 }, [renderRect()])),
  ];
}

function renderRect(children?: RenderingTree): RenderingTree {
  return [
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
      onMouseIn: (event) => {
        console.log("hi");
      },
    }),
    children,
  ];
}

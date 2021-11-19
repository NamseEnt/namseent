import { RenderingTree, Translate } from "namui";

type Row = {
  renderingData: RenderingTree;
  height: number;
};

export function renderRows(rows: Row[], spacing: number = 4): RenderingTree {
  let y = 0;
  return rows.map((row) => {
    const translated = row.renderingData
      ? Translate(
          {
            x: 0,
            y,
          },
          row.renderingData,
        )
      : undefined;
    y += row.height ? row.height + spacing : 0;
    return translated;
  });
}

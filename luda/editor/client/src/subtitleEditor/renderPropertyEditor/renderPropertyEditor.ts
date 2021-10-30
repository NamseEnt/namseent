import {
  ColorUtil,
  FontWeight,
  Language,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
} from "namui";
import { SubtitleEditorState } from "../type";
import { renderColorInput } from "./renderColorInput/renderColorInput";
import { renderNumberInput } from "./renderNumberInput";
import { renderRows } from "./renderRows";
import { renderTextInput } from "./renderTextInput";

export function renderPropertyEditor(
  props: {
    layout: {
      x: number;
      y: number;
      width: number;
    };
  },
  state: SubtitleEditorState,
): RenderingTree {
  return Translate(
    {
      x: props.layout.x,
      y: props.layout.y,
    },
    renderRows([
      {
        height: 20,
        renderingData: renderTextInput(
          {
            width: props.layout.width,
            label: "text",
            onChange: (value) => (state.subtitle.text = value),
            value: state.subtitle.text,
            textInputId: "text",
          },
          state.textInput,
        ),
      },
      {
        height: 20,
        renderingData: undefined,
      },
      {
        height: 20,
        renderingData: Text({
          x: 0,
          y: 0,
          align: TextAlign.left,
          baseline: TextBaseline.top,
          fontType: {
            language: Language.ko,
            serif: false,
            fontWeight: FontWeight.regular,
            size: 12,
          },
          style: {
            color: ColorUtil.Black,
          },
          text: "style",
        }),
      },
      {
        height: 20,
        renderingData: renderNumberInput(
          {
            width: props.layout.width,
            label: "fontSize",
            onChange: (value) => (state.subtitle.style.fontSize = value),
            value: state.subtitle.style.fontSize,
            textInputId: "fontSize",
          },
          state.textInput,
        ),
      },
      {
        height: state.colorInput.targetId === "fontColor" ? 120 : 20,
        renderingData: renderColorInput(
          {
            width: props.layout.width,
            label: "fontColor",
            colorInputId: "fontColor",
            value: state.subtitle.style.fontColor,
            onChange: (color) => (state.subtitle.style.fontColor = color),
          },
          state.colorInput,
        ),
      },
      {
        height: state.colorInput.targetId === "backgroundColor" ? 120 : 20,
        renderingData: renderColorInput(
          {
            width: props.layout.width,
            label: "backgroundColor",
            colorInputId: "backgroundColor",
            value: state.subtitle.style.backgroundColor,
            onChange: (color) => (state.subtitle.style.backgroundColor = color),
          },
          state.colorInput,
        ),
      },
      {
        height: 20,
        renderingData: undefined,
      },
    ]),
  );
}

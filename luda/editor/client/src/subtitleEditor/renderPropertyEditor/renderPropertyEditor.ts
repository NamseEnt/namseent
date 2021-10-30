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
import { renderCheckboxInput } from "./renderCheckboxInput";
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
          text: "Font",
        }),
      },
      {
        height: 20,
        renderingData: renderTextInput(
          {
            width: props.layout.width,
            label: "text",
            onChange: (value) => (state.subtitle.text = value),
            value: state.subtitle.text,
            textInputId: "subtitleEditorState.subtitle.text",
          },
          state.textInput,
        ),
      },
      {
        height: 20,
        renderingData: renderNumberInput(
          {
            width: props.layout.width,
            label: "size",
            onChange: (value) => (state.subtitle.fontType.size = value),
            value: state.subtitle.fontType.size,
            textInputId: "subtitleEditorState.subtitle.fontType.size",
          },
          state.textInput,
        ),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "serif",
          onChange: (value) => (state.subtitle.fontType.serif = value),
          value: state.subtitle.fontType.serif,
        }),
      },
      {
        height:
          state.colorInput.targetId ===
          "subtitleEditorState.subtitle.style.color"
            ? 120
            : 20,
        renderingData: renderColorInput(
          {
            width: props.layout.width,
            label: "color",
            colorInputId: "subtitleEditorState.subtitle.style.color",
            value: state.subtitle.style.color,
            onChange: (color) => (state.subtitle.style.color = color),
          },
          state.colorInput,
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
          text: "Background",
        }),
      },
      {
        height:
          state.colorInput.targetId ===
          "subtitleEditorState.subtitle.style.background.color"
            ? 120
            : 20,
        renderingData: renderColorInput(
          {
            width: props.layout.width,
            label: "color",
            colorInputId: "subtitleEditorState.subtitle.style.background.color",
            value: state.subtitle.style.background.color,
            onChange: (color) =>
              (state.subtitle.style.background.color = color),
          },
          state.colorInput,
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
          text: "Border",
        }),
      },
      {
        height:
          state.colorInput.targetId ===
          "subtitleEditorState.subtitle.style.border.color"
            ? 120
            : 20,
        renderingData: renderColorInput(
          {
            width: props.layout.width,
            label: "color",
            colorInputId: "subtitleEditorState.subtitle.style.border.color",
            value: state.subtitle.style.border.color,
            onChange: (color) => (state.subtitle.style.border.color = color),
          },
          state.colorInput,
        ),
      },
      {
        height: 20,
        renderingData: renderNumberInput(
          {
            width: props.layout.width,
            label: "width",
            onChange: (value) => (state.subtitle.style.border.width = value),
            value: state.subtitle.style.border.width,
            textInputId: "subtitleEditorState.subtitle.style.border.width",
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
          text: "Drop shadow",
        }),
      },
      {
        height:
          state.colorInput.targetId ===
          "subtitleEditorState.subtitle.style.dropShadow.color"
            ? 120
            : 20,
        renderingData: renderColorInput(
          {
            width: props.layout.width,
            label: "color",
            colorInputId: "subtitleEditorState.subtitle.style.dropShadow.color",
            value: state.subtitle.style.dropShadow.color,
            onChange: (color) =>
              (state.subtitle.style.dropShadow.color = color),
          },
          state.colorInput,
        ),
      },
      {
        height: 20,
        renderingData: renderNumberInput(
          {
            width: props.layout.width,
            label: "x",
            onChange: (value) => (state.subtitle.style.dropShadow.x = value),
            value: state.subtitle.style.dropShadow.x,
            textInputId: "subtitleEditorState.subtitle.style.dropShadow.x",
          },
          state.textInput,
        ),
      },
      {
        height: 20,
        renderingData: renderNumberInput(
          {
            width: props.layout.width,
            label: "y",
            onChange: (value) => (state.subtitle.style.dropShadow.y = value),
            value: state.subtitle.style.dropShadow.y,
            textInputId: "subtitleEditorState.subtitle.style.dropShadow.y",
          },
          state.textInput,
        ),
      },
    ]),
  );
}

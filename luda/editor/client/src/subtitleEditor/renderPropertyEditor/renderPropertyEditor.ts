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
import { SubtitleFontSize } from "../../type";
import { SubtitleEditorState } from "../type";
import { renderCheckboxInput } from "./renderCheckboxInput";
import { renderColorInput } from "./renderColorInput/renderColorInput";
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
        renderingData: renderCheckboxInput({
          label: "serif",
          onChange: (value) => (state.subtitle.fontType.serif = value),
          value: state.subtitle.fontType.serif,
        }),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "small",
          onChange: () => {
            state.subtitle.fontType.size = SubtitleFontSize.small;
            state.subtitle.style.border.width = SubtitleFontSize.small / 24;
            state.subtitle.style.dropShadow.x = SubtitleFontSize.small / 24;
            state.subtitle.style.dropShadow.y = SubtitleFontSize.small / 24;
          },
          value: state.subtitle.fontType.size === SubtitleFontSize.small,
        }),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "regular",
          onChange: () => {
            state.subtitle.fontType.size = SubtitleFontSize.regular;
            state.subtitle.style.border.width = SubtitleFontSize.regular / 24;
            state.subtitle.style.dropShadow.x = SubtitleFontSize.regular / 24;
            state.subtitle.style.dropShadow.y = SubtitleFontSize.regular / 24;
          },
          value: state.subtitle.fontType.size === SubtitleFontSize.regular,
        }),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "large",
          onChange: () => {
            state.subtitle.fontType.size = SubtitleFontSize.large;
            state.subtitle.style.border.width = SubtitleFontSize.large / 24;
            state.subtitle.style.dropShadow.x = SubtitleFontSize.large / 24;
            state.subtitle.style.dropShadow.y = SubtitleFontSize.large / 24;
          },
          value: state.subtitle.fontType.size === SubtitleFontSize.large,
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
    ]),
  );
}

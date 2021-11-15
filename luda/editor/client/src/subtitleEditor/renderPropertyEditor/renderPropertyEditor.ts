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
import { renderRows } from "../../common/renderRows";
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
  const { subtitle } = state;
  if (!subtitle) {
    return;
  }

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
            onChange: (value) => (subtitle.text = value),
            value: subtitle.text,
            textInputId: "subtitleEditorState.subtitle.text",
          },
          state.textInput,
        ),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "serif",
          onChange: (value) => (subtitle.fontType.serif = value),
          value: subtitle.fontType.serif,
        }),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "small",
          onChange: () => {
            subtitle.fontType.size = SubtitleFontSize.small;
            subtitle.style.border.width = SubtitleFontSize.small / 24;
            subtitle.style.dropShadow.x = SubtitleFontSize.small / 24;
            subtitle.style.dropShadow.y = SubtitleFontSize.small / 24;
          },
          value: subtitle.fontType.size === SubtitleFontSize.small,
        }),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "regular",
          onChange: () => {
            subtitle.fontType.size = SubtitleFontSize.regular;
            subtitle.style.border.width = SubtitleFontSize.regular / 24;
            subtitle.style.dropShadow.x = SubtitleFontSize.regular / 24;
            subtitle.style.dropShadow.y = SubtitleFontSize.regular / 24;
          },
          value: subtitle.fontType.size === SubtitleFontSize.regular,
        }),
      },
      {
        height: 20,
        renderingData: renderCheckboxInput({
          label: "large",
          onChange: () => {
            subtitle.fontType.size = SubtitleFontSize.large;
            subtitle.style.border.width = SubtitleFontSize.large / 24;
            subtitle.style.dropShadow.x = SubtitleFontSize.large / 24;
            subtitle.style.dropShadow.y = SubtitleFontSize.large / 24;
          },
          value: subtitle.fontType.size === SubtitleFontSize.large,
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
            value: subtitle.style.color,
            onChange: (color) => (subtitle.style.color = color),
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
            value: subtitle.style.background.color,
            onChange: (color) => (subtitle.style.background.color = color),
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
            value: subtitle.style.border.color,
            onChange: (color) => (subtitle.style.border.color = color),
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
            value: subtitle.style.dropShadow.color,
            onChange: (color) => (subtitle.style.dropShadow.color = color),
          },
          state.colorInput,
        ),
      },
    ]),
  );
}

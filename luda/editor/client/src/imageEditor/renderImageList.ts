import {
  ColorUtil,
  Language,
  Button,
  Text,
  RenderingTree,
  TextAlign,
  TextBaseline,
  Image,
  Translate,
  ImageFit,
  FontWeight,
} from "namui";
import fileSystem from "../fileSystem/fileSystem";
import { ImageEditorState } from "./type";

export function renderImageList(
  imageEditorState: ImageEditorState,
): RenderingTree {
  const imageListSize = {
    width: 600,
    height: 1000,
  };
  const headerSize = {
    width: imageListSize.width,
    height: 50,
  };
  const bodySize = {
    width: imageListSize.width,
    height: imageListSize.height - headerSize.height,
  };
  return [
    renderImageListHeader({
      ...headerSize,
      imageEditorState,
    }),
    Translate(
      {
        x: 0,
        y: headerSize.height,
      },
      renderingImageListBody({
        ...bodySize,
        imageEditorState,
      }),
    ),
  ];
}

function renderImageListHeader({
  width,
  height,
  imageEditorState,
}: {
  width: number;
  height: number;
  imageEditorState: ImageEditorState;
}): RenderingTree {
  return [
    Button({
      x: 0,
      y: 0,
      width,
      height,
      onClick: async () => {
        const directoryPath = "images";
        const dirents = await fileSystem.list(directoryPath);
        const files = dirents.filter((dirent) => dirent.type === "file");
        const imageUrls = files.map((dirent) => {
          const filePath = `resources/${directoryPath}/${dirent.name}`;
          return filePath;
        });
        imageEditorState.imageUrls = imageUrls;
      },
      style: {
        fill: {
          color: ColorUtil.Black,
        },
      },
      content: Text({
        x: width / 2,
        y: height / 2,
        baseline: TextBaseline.middle,
        align: TextAlign.center,
        text: "+ Sync List",
        fontType: {
          serif: false,
          language: Language.ko,
          size: 20,
          fontWeight: FontWeight.regular,
        },
        style: {
          color: ColorUtil.White,
        },
      }),
    }),
  ];
}

function renderingImageListBody({
  width,
  height,
  imageEditorState,
}: {
  width: number;
  height: number;
  imageEditorState: ImageEditorState;
}): RenderingTree {
  if (!imageEditorState.imageUrls.length) {
    return;
  }
  const imageWidth = width / 2;
  const imageHeight = height / Math.ceil(imageEditorState.imageUrls.length / 2);
  return imageEditorState.imageUrls.map((imageUrl, index) => {
    const position = {
      x: index % 2 === 0 ? 0 : imageWidth,
      y: Math.floor(index / 2) * imageHeight,
    };
    return Image({
      position,
      url: imageUrl,
      size: {
        width: imageWidth,
        height: imageHeight,
      },
      style: {
        fit: ImageFit.contain,
      },
    });
  });
}

function renderingImageListItem(): RenderingTree {
  return undefined;
}

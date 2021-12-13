import { Render, Translate, WhSize, XywhRect } from "namui";
import { BackButton } from "./BackButton";
import { BrowserItem } from "./BrowserItem";
import { CurrentDirectoryLabel } from "./CurrentDirectoryLabel";
import { convertImageFilenameObjectToUrl } from "./ImageFilenameObject";
import { Scroll } from "./Scroll";
import { SyncBrowserItems } from "./SyncBrowserItems";
import { ImageBrowserState } from "./type";

export const ImageBrowser: Render<
  ImageBrowserState,
  {
    chooseImage: (url: string) => void;
  }
> = (state, props) => {
  const isRoot = state.directoryKey === "";
  const itemMargin = 10;

  const itemWidth = state.layout.width / 2 - itemMargin;
  const itemSize: WhSize = {
    width: itemWidth,
    height: itemWidth,
  };
  const thumbnailRect: XywhRect = {
    x: 10,
    y: 5,
    width: itemSize.width - 20,
    height: itemSize.height - 20,
  };

  function getBrowserItemY(index: number): number {
    return itemMargin + Math.floor(index / 2) * (itemSize.height + itemMargin);
  }
  const browserItems = [
    ...(isRoot ? [] : [BackButton(state, { itemSize, thumbnailRect })]),
    ...getBrowserItemProps({ state, chooseImage: props.chooseImage }).map(
      (props) => {
        return BrowserItem(state, {
          itemSize,
          thumbnailRect,
          ...props,
        });
      },
    ),
  ].map((browserItem, index) => {
    return Translate(
      {
        x: (index % 2) * (itemSize.width + itemMargin),
        y: getBrowserItemY(index),
      },
      [browserItem],
    );
  });

  const browserItemScrollHeight =
    getBrowserItemY(browserItems.length - 1) + itemSize.height + itemMargin;

  const scrollBarWidth = 10;

  return Translate(state.layout, [
    CurrentDirectoryLabel(state, {}),
    Translate(
      {
        x: 0,
        y:
          state.layout.currentDirectoryLabel.y +
          state.layout.currentDirectoryLabel.height,
      },
      [
        Scroll(state.scrollState, {
          layout: {
            x: 0,
            y: 0,
            innerWidth: state.layout.width - scrollBarWidth,
            innerHeight: browserItemScrollHeight,
            scrollBarWidth,
            height:
              state.layout.height -
              (state.layout.currentDirectoryLabel.y +
                state.layout.currentDirectoryLabel.height),
          },
          innerRenderingTree: [browserItems],
        }),
      ],
    ),
    SyncBrowserItems(
      {
        imageBrowser: state,
        syncBrowserItems: state.syncBrowserItems,
      },
      {},
    ),
  ]);
};

function getBrowserItemProps({
  state,
  chooseImage,
}: {
  state: ImageBrowserState;
  chooseImage: (url: string) => void;
}): {
  name: string;
  thumbnailUrl: string;
  onSelect: () => void;
  isSelected: boolean;
}[] {
  const [character, pose] = state.directoryKey.split("-");
  if (!character) {
    const characters = new Set<string>();
    state.imageFilenameObjects.forEach((filenameObject) => {
      characters.add(filenameObject.character);
    });
    return Array.from(characters).map((character) => {
      const filenameObject = state.imageFilenameObjects.find(
        (filenameObject) => {
          return filenameObject.character === character;
        },
      )!;
      const key = `${character}`;
      return {
        name: character,
        thumbnailUrl: convertImageFilenameObjectToUrl(filenameObject),
        onSelect() {
          state.directoryKey = key;
          state.selectedKey = "back";
        },
        isSelected: state.selectedKey === key,
      };
    });
  }

  if (!pose) {
    const poses = new Set<string>();
    state.imageFilenameObjects
      .filter((filenameObject) => filenameObject.character === character)
      .forEach((filenameObject) => {
        poses.add(filenameObject.pose);
      });

    return Array.from(poses).map((pose) => {
      const filenameObject = state.imageFilenameObjects.find(
        (filenameObject) => {
          return (
            filenameObject.character === character &&
            filenameObject.pose === pose
          );
        },
      )!;
      const key = `${character}-${pose}`;
      const imageUrl = convertImageFilenameObjectToUrl(filenameObject);
      return {
        name: pose,
        thumbnailUrl: imageUrl,
        onSelect() {
          state.directoryKey = key;
          state.selectedKey = "back";
        },
        isSelected: state.selectedKey === key,
      };
    });
  }

  const emotions = new Set<string>();
  state.imageFilenameObjects
    .filter(
      (filenameObject) =>
        filenameObject.character === character && filenameObject.pose === pose,
    )
    .forEach((filenameObject) => {
      emotions.add(filenameObject.emotion);
    });

  return Array.from(emotions).map((emotion) => {
    const filenameObject = state.imageFilenameObjects.find((filenameObject) => {
      return (
        filenameObject.character === character &&
        filenameObject.pose === pose &&
        filenameObject.emotion === emotion
      );
    })!;
    const key = `${character}-${pose}-${emotion}`;
    const imageUrl = convertImageFilenameObjectToUrl(filenameObject);
    return {
      name: emotion,
      thumbnailUrl: imageUrl,
      onSelect() {
        chooseImage(imageUrl);
        state.selectedKey = key;
      },
      isSelected: state.selectedKey === key,
    };
  });
}

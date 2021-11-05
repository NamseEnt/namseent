import { Render, Translate, WhSize, XywhRect } from "namui";
import { BackButton } from "./BackButton";
import { BrowserItem } from "./BrowserItem";
import { CurrentDirectoryLabel } from "./CurrentDirectoryLabel";
import { convertImageFileKeyObjectToUrl } from "./ImageFileKeyObject";
import { SyncBrowserItems } from "./SyncBrowserItems";
import { ImageBrowserState } from "./type";

export const ImageBrowser: Render<ImageBrowserState, {}> = (state, props) => {
  const isRoot = state.key === "";
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

  const browserItems = [
    ...(isRoot ? [] : [BackButton(state, { itemSize, thumbnailRect })]),
    ...getBrowserItemProps({ state }).map((props) => {
      return BrowserItem(state, {
        itemSize,
        thumbnailRect,
        ...props,
      });
    }),
  ].map((browserItem, index) => {
    return Translate(
      {
        x: (index % 2) * (itemSize.width + itemMargin),
        y: Math.floor(index / 2) * (itemSize.height + itemMargin),
      },
      [browserItem],
    );
  });

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
        // Scroll({},
        [browserItems],
        // ),
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

function getBrowserItemProps({ state }: { state: ImageBrowserState }): {
  name: string;
  thumbnailUrl: string;
  onSelect: () => void;
}[] {
  const [character, pose] = state.key.split("-");
  if (!character) {
    const characters = new Set<string>();
    state.imageFileKeyObjects.forEach((keyObject) => {
      characters.add(keyObject.character);
    });
    return Array.from(characters).map((character) => {
      const keyObject = state.imageFileKeyObjects.find((keyObject) => {
        return keyObject.character === character;
      })!;
      const key = `${character}`;
      return {
        name: key,
        thumbnailUrl: convertImageFileKeyObjectToUrl(keyObject),
        onSelect() {
          state.key = key;
        },
      };
    });
  }

  if (!pose) {
    const poses = new Set<string>();
    state.imageFileKeyObjects
      .filter((keyObject) => keyObject.character === character)
      .forEach((keyObject) => {
        poses.add(keyObject.pose);
      });

    return Array.from(poses).map((pose) => {
      const keyObject = state.imageFileKeyObjects.find((keyObject) => {
        return keyObject.character === character && keyObject.pose === pose;
      })!;
      const key = `${character}-${pose}`;
      return {
        name: key,
        thumbnailUrl: convertImageFileKeyObjectToUrl(keyObject),
        onSelect() {
          state.key = key;
        },
      };
    });
  }

  const emotions = new Set<string>();
  state.imageFileKeyObjects
    .filter(
      (keyObject) =>
        keyObject.character === character && keyObject.pose === pose,
    )
    .forEach((keyObject) => {
      emotions.add(keyObject.emotion);
    });

  return Array.from(emotions).map((emotion) => {
    const keyObject = state.imageFileKeyObjects.find((keyObject) => {
      return (
        keyObject.character === character &&
        keyObject.pose === pose &&
        keyObject.emotion === emotion
      );
    })!;
    const key = `${character}-${pose}-${emotion}`;
    return {
      name: key,
      thumbnailUrl: convertImageFileKeyObjectToUrl(keyObject),
      onSelect() {
        state.key = key;
      },
    };
  });
}

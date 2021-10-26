import { InputRect } from "canvaskit-wasm";
import { ImageFit, XywhRect } from "../../type";

export function getRectsInFit({
  fit,
  imageSize,
  commandRect,
}: {
  fit: ImageFit;
  imageSize: { width: number; height: number };
  commandRect: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
}): {
  srcRect: InputRect;
  destRect: InputRect;
} {
  const { srcRect, destRect } = getXywhRectsInFit({
    fit,
    imageSize,
    commandRect,
  });
  return {
    srcRect: CanvasKit.XYWHRect(
      srcRect.x,
      srcRect.y,
      srcRect.width,
      srcRect.height,
    ),
    destRect: CanvasKit.XYWHRect(
      destRect.x,
      destRect.y,
      destRect.width,
      destRect.height,
    ),
  };
}

function getXywhRectsInFit({
  fit,
  imageSize,
  commandRect,
}: {
  fit: ImageFit;
  imageSize: { width: number; height: number };
  commandRect: XywhRect;
}): {
  srcRect: XywhRect;
  destRect: XywhRect;
} {
  switch (fit) {
    case ImageFit.fill: {
      return {
        srcRect: {
          x: 0,
          y: 0,
          width: imageSize.width,
          height: imageSize.height,
        },
        destRect: {
          x: commandRect.x,
          y: commandRect.y,
          width: commandRect.width,
          height: commandRect.height,
        },
      };
    }
    case ImageFit.contain: {
      return {
        srcRect: {
          x: 0,
          y: 0,
          width: imageSize.width,
          height: imageSize.height,
        },
        destRect: calculateContainFitDestRect({ imageSize, commandRect }),
      };
    }
    case ImageFit.cover: {
      return {
        srcRect: calculateCoverFitSrcRect({ imageSize, commandRect }),
        destRect: {
          x: commandRect.x,
          y: commandRect.y,
          width: commandRect.width,
          height: commandRect.height,
        },
      };
    }
    case ImageFit.none: {
      return calculateNoneFitRects({ imageSize, commandRect });
    }
    case ImageFit.scaleDown: {
      const containFitSrcDest = getXywhRectsInFit({
        fit: ImageFit.contain,
        imageSize,
        commandRect,
      });
      const noneFitSrcDest = getXywhRectsInFit({
        fit: ImageFit.contain,
        imageSize,
        commandRect,
      });
      return containFitSrcDest.destRect.width < noneFitSrcDest.destRect.width ||
        containFitSrcDest.destRect.height < noneFitSrcDest.destRect.height
        ? containFitSrcDest
        : noneFitSrcDest;
    }
  }
}

function calculateNoneFitRects({
  imageSize,
  commandRect,
}: {
  imageSize: { width: number; height: number };
  commandRect: { x: number; y: number; width: number; height: number };
}): {
  srcRect: XywhRect;
  destRect: XywhRect;
} {
  const srcX =
    imageSize.width <= commandRect.width
      ? 0
      : (imageSize.width - commandRect.width) / 2;
  const srcY =
    imageSize.height <= commandRect.height
      ? 0
      : (imageSize.height - commandRect.height) / 2;
  const srcWidth =
    imageSize.width <= commandRect.width ? imageSize.width : commandRect.width;
  const srcHeight =
    imageSize.height <= commandRect.height
      ? imageSize.height
      : commandRect.height;
  const srcRect = {
    x: srcX,
    y: srcY,
    width: srcWidth,
    height: srcHeight,
  };

  const destCenterX = commandRect.x + commandRect.width / 2;
  const destCenterY = commandRect.y + commandRect.height / 2;
  const destX = destCenterX - srcWidth / 2;
  const destY = destCenterY - srcHeight / 2;
  const destRect = {
    x: destX,
    y: destY,
    width: srcWidth,
    height: srcHeight,
  };

  return { srcRect, destRect };
}

function calculateContainFitDestRect({
  imageSize,
  commandRect,
}: {
  imageSize: { width: number; height: number };
  commandRect: { x: number; y: number; width: number; height: number };
}): XywhRect {
  if (
    imageSize.width / imageSize.height ===
    commandRect.width / commandRect.height
  ) {
    return {
      x: commandRect.x,
      y: commandRect.y,
      width: commandRect.width,
      height: commandRect.height,
    };
  }

  if (
    imageSize.width / imageSize.height >
    commandRect.width / commandRect.height
  ) {
    const k = commandRect.width / imageSize.width;
    const deltaY = (commandRect.height - k * imageSize.height) / 2;
    return {
      x: commandRect.x,
      y: commandRect.y + deltaY,
      width: commandRect.width,
      height: k * imageSize.height,
    };
  }

  const k = commandRect.height / imageSize.height;
  const deltaX = (commandRect.width - k * imageSize.width) / 2;
  return {
    x: commandRect.x + deltaX,
    y: commandRect.y,
    width: k * imageSize.width,
    height: commandRect.height,
  };
}

function calculateCoverFitSrcRect({
  imageSize,
  commandRect,
}: {
  imageSize: { width: number; height: number };
  commandRect: { x: number; y: number; width: number; height: number };
}): XywhRect {
  if (
    imageSize.width / imageSize.height ===
    commandRect.width / commandRect.height
  ) {
    return {
      x: 0,
      y: 0,
      width: imageSize.width,
      height: imageSize.height,
    };
  }
  if (
    imageSize.width / imageSize.height >
    commandRect.width / commandRect.height
  ) {
    const k = commandRect.height / imageSize.height;
    const deltaX = (k * imageSize.width - commandRect.width) / (2 * k);
    return {
      x: deltaX,
      y: 0,
      width: imageSize.width - 2 * deltaX,
      height: imageSize.height,
    };
  }

  const k = commandRect.width / imageSize.width;
  const deltaY = (k * imageSize.height - commandRect.height) / (2 * k);
  return {
    x: 0,
    y: deltaY,
    width: imageSize.width,
    height: imageSize.height - 2 * deltaY,
  };
}

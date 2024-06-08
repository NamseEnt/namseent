///https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D
export function getSizePerPixel(
  webgl: WebGL2RenderingContext,
  format: number,
  type: number
) {
  function assertType(expectedType: number) {
    if (expectedType !== type) {
      throw new Error(`Invalid type: ${type}, expected: ${expectedType}`);
    }
  }

  switch (format) {
    case webgl.RGBA: // 0x1908
      switch (type) {
        case webgl.UNSIGNED_BYTE: // 0x1401
          return 4;
        case webgl.UNSIGNED_SHORT_4_4_4_4: // 0x8033
          return 2;
        case webgl.UNSIGNED_SHORT_5_5_5_1: // 0x8034
          return 2;
        default:
          throw new Error("Invalid type");
      }
    case webgl.RGB: // 0x1907
      switch (type) {
        case webgl.UNSIGNED_BYTE: // 0x1401
          return 3;
        case webgl.UNSIGNED_SHORT_5_6_5: // 0x8363
          return 2;
        default:
          throw new Error("Invalid type");
      }
    case webgl.LUMINANCE_ALPHA: // 0x190A
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 2;
    case webgl.LUMINANCE: // 0x1909
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 1;
    case webgl.ALPHA: // 0x1906
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 1;
    case webgl.RED: // 0x1903
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 1;
    case webgl.R8: // 0x8229
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 1;
    case webgl.R8_SNORM: // 0x8F94
      assertType(webgl.BYTE); // 0x1400
      return 1;
    case webgl.RG8: // 0x822B
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 2;
    case webgl.RG8_SNORM: // 0x8F95
      assertType(webgl.BYTE); // 0x1400
      return 2;
    case webgl.RGB8: // 0x8051
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 3;
    case webgl.RGB8_SNORM: // 0x8F96
      assertType(webgl.BYTE); // 0x1400
      return 3;
    case webgl.RGB565: // 0x8D62
      assertType(webgl.UNSIGNED_SHORT_5_6_5); // 0x8363
      return 2;
    case webgl.RGBA4: // 0x8056
      assertType(webgl.UNSIGNED_SHORT_4_4_4_4); // 0x8033
      return 2;
    case webgl.RGB5_A1: // 0x8057
      assertType(webgl.UNSIGNED_SHORT_5_5_5_1); // 0x8034
      return 2;
    case webgl.RGBA8: // 0x8058
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 4;
    case webgl.RGBA8_SNORM: // 0x8F97
      assertType(webgl.BYTE); // 0x1400
      return 4;
    case webgl.RGB10_A2: // 0x8059
      assertType(webgl.UNSIGNED_INT_2_10_10_10_REV); // 0x8368
      return 4;
    case webgl.RGB10_A2UI: // 0x906F
      assertType(webgl.UNSIGNED_INT_2_10_10_10_REV); // 0x8368
      return 4;
    case webgl.SRGB8: // 0x8C41
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 3;
    case webgl.SRGB8_ALPHA8: // 0x8C43
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 4;
    case webgl.R16F: // 0x822D
      assertType(webgl.HALF_FLOAT); // 0x140B
      return 2;
    case webgl.RG16F: // 0x822F
      assertType(webgl.HALF_FLOAT); // 0x140B
      return 4;
    case webgl.RGB16F: // 0x881B
      assertType(webgl.HALF_FLOAT); // 0x140B
      return 6;
    case webgl.RGBA16F: // 0x881A
      assertType(webgl.HALF_FLOAT); // 0x140B
      return 8;
    case webgl.R32F: // 0x822E
      assertType(webgl.FLOAT); // 0x1406
      return 4;
    case webgl.RG32F: // 0x8230
      assertType(webgl.FLOAT); // 0x1406
      return 8;
    case webgl.RGB32F: // 0x8815
      assertType(webgl.FLOAT); // 0x1406
      return 12;
    case webgl.RGBA32F: // 0x8814
      assertType(webgl.FLOAT); // 0x1406
      return 16;
    case webgl.R11F_G11F_B10F: // 0x8C3A
      assertType(webgl.UNSIGNED_INT_10F_11F_11F_REV); // 0x8C3B
      return 4;
    case webgl.RGB9_E5: // 0x8C3D
      assertType(webgl.UNSIGNED_INT_5_9_9_9_REV); // 0x8C3E
      return 4;
    case webgl.R8I: // 0x8231
      assertType(webgl.BYTE); // 0x1400
      return 1;
    case webgl.R8UI: // 0x8232
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 1;
    case webgl.R16I: // 0x8233
      assertType(webgl.SHORT); // 0x1402
      return 2;
    case webgl.R16UI: // 0x8234
      assertType(webgl.UNSIGNED_SHORT); // 0x1403
      return 2;
    case webgl.R32I: // 0x8235
      assertType(webgl.INT); // 0x1404
      return 4;
    case webgl.R32UI: // 0x8236
      assertType(webgl.UNSIGNED_INT); // 0x1405
      return 4;
    case webgl.RG8I: // 0x8237
      assertType(webgl.BYTE); // 0x1400
      return 2;
    case webgl.RG8UI: // 0x8238
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 2;
    case webgl.RG16I: // 0x8239
      assertType(webgl.SHORT); // 0x1402
      return 4;
    case webgl.RG16UI: // 0x823A
      assertType(webgl.UNSIGNED_SHORT); // 0x1403
      return 4;
    case webgl.RG32I: // 0x823B
      assertType(webgl.INT); // 0x1404
      return 8;
    case webgl.RG32UI: // 0x823C
      assertType(webgl.UNSIGNED_INT); // 0x1405
      return 8;
    case webgl.RGB8I: // 0x8D8F
      assertType(webgl.BYTE); // 0x1400
      return 3;
    case webgl.RGB8UI: // 0x8D7D
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 3;
    case webgl.RGB16I: // 0x8D89
      assertType(webgl.SHORT); // 0x1402
      return 6;
    case webgl.RGB16UI: // 0x8D77
      assertType(webgl.UNSIGNED_SHORT); // 0x1403
      return 6;
    case webgl.RGB32I: // 0x8D83
      assertType(webgl.INT); // 0x1404
      return 12;
    case webgl.RGB32UI: // 0x8D71
      assertType(webgl.UNSIGNED_INT); // 0x1405
      return 12;
    case webgl.RGBA8I: // 0x8D8E
      assertType(webgl.BYTE); // 0x1400
      return 4;
    case webgl.RGBA8UI: // 0x8D7C
      assertType(webgl.UNSIGNED_BYTE); // 0x1401
      return 4;
    case webgl.RGBA16I: // 0x8D88
      assertType(webgl.SHORT); // 0x1402
      return 8;
    case webgl.RGBA16UI: // 0x8D76
      assertType(webgl.UNSIGNED_SHORT); // 0x1403
      return 8;
    case webgl.RGBA32I: // 0x8D82
      assertType(webgl.INT); // 0x1404
      return 16;
    case webgl.RGBA32UI: // 0x8D70
      assertType(webgl.UNSIGNED_INT); // 0x1405
      return 16;
    default:
      throw new Error("Invalid format");
  }
}

/// https://github.com/emscripten-core/emscripten/blob/cb99414efed02dc61d04315d3e3cf5ad3180e56f/src/library_webgl.js#L1554C4-L1564C5
export function computeUnpackAlignedImageSize(
  webgl: WebGL2RenderingContext,
  width: number,
  height: number,
  sizePerPixel: number
) {
  function roundedToNextMultipleOf(x: number, y: number) {
    if ((y & (y - 1)) !== 0) {
      throw new Error(
        `Unpack alignment must be a power of 2! ${y} is not power of 2. (Allowed values per WebGL spec are 1, 2, 4 or 8)`
      );
    }
    return (x + y - 1) & -y;
  }
  const plainRowSize =
    (webgl.getParameter(webgl.UNPACK_ROW_LENGTH) || width) * sizePerPixel;
  const alignedRowSize = roundedToNextMultipleOf(
    plainRowSize,
    webgl.getParameter(webgl.UNPACK_ALIGNMENT)
  );
  return height * alignedRowSize;
}

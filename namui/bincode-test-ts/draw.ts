import { BincodeReader } from "./reader";

// Canvas interface placeholder (나중에 실제 구현으로 대체)
export interface Canvas {
  save(): void;
  restore(): void;
  translate(x: number, y: number): void;
  rotate(angle: number): void;
  scale(x: number, y: number): void;
  setMatrix(matrix: [[number, number, number], [number, number, number]]): void;
  getMatrix(): [[number, number, number], [number, number, number]];
  clipPath(clipOp: number): void;
  // draw 메서드들은 나중에 추가
}

interface OnTopContext {
  matrix: [[number, number, number], [number, number, number]];
  readerOffset: number;
}

export function visitRenderingTree(
  reader: BincodeReader,
  canvas: Canvas,
  onTopNodes: OnTopContext[]
): void {
  const variant = reader.readVarintU32();

  switch (variant) {
    case 0: // Empty
      break;

    case 1: // Node
      visitDrawCommand(reader, canvas);
      break;

    case 2: // Children
      {
        const len = reader.readVarintU64();
        // Children are drawn in reverse order
        const offsets: number[] = [];
        for (let i = 0; i < len; i++) {
          offsets.push(reader.getOffset());
          skipRenderingTree(reader);
        }
        for (let i = offsets.length - 1; i >= 0; i--) {
          const buffer = (reader as any).view.buffer as ArrayBuffer;
          const childReader = new BincodeReader(buffer.slice(offsets[i]));
          visitRenderingTree(childReader, canvas, onTopNodes);
        }
      }
      break;

    case 3: // Special
      visitSpecialRenderingNode(reader, canvas, onTopNodes);
      break;

    case 4: // Boxed
      visitRenderingTree(reader, canvas, onTopNodes);
      break;

    case 5: // BoxedChildren
      {
        const len = reader.readVarintU64();
        const offsets: number[] = [];
        for (let i = 0; i < len; i++) {
          offsets.push(reader.getOffset());
          skipRenderingTree(reader);
        }
        for (let i = offsets.length - 1; i >= 0; i--) {
          const buffer = (reader as any).view.buffer as ArrayBuffer;
          const childReader = new BincodeReader(buffer.slice(offsets[i]));
          visitRenderingTree(childReader, canvas, onTopNodes);
        }
      }
      break;

    default:
      throw new Error(`Unknown RenderingTree variant: ${variant}`);
  }
}

function visitSpecialRenderingNode(
  reader: BincodeReader,
  canvas: Canvas,
  onTopNodes: OnTopContext[]
): void {
  const variant = reader.readVarintU32();

  switch (variant) {
    case 0: // Translate
      {
        const x = reader.readF32();
        const y = reader.readF32();
        canvas.save();
        canvas.translate(x, y);
        visitRenderingTree(reader, canvas, onTopNodes);
        canvas.restore();
      }
      break;

    case 1: // Clip
      {
        skipPath(reader);
        const clipOp = reader.readVarintU32();
        canvas.save();
        canvas.clipPath(clipOp);
        visitRenderingTree(reader, canvas, onTopNodes);
        canvas.restore();
      }
      break;

    case 2: // WithId
      {
        reader.readString(); // id
        visitRenderingTree(reader, canvas, onTopNodes);
      }
      break;

    case 3: // Absolute
      {
        const x = reader.readF32();
        const y = reader.readF32();
        canvas.save();
        canvas.setMatrix([
          [1.0, 0.0, x],
          [0.0, 1.0, y],
        ]);
        visitRenderingTree(reader, canvas, onTopNodes);
        canvas.restore();
      }
      break;

    case 4: // Rotate
      {
        const angle = reader.readF32();
        canvas.save();
        canvas.rotate(angle);
        visitRenderingTree(reader, canvas, onTopNodes);
        canvas.restore();
      }
      break;

    case 5: // Scale
      {
        const x = reader.readF32();
        const y = reader.readF32();
        canvas.save();
        canvas.scale(x, y);
        visitRenderingTree(reader, canvas, onTopNodes);
        canvas.restore();
      }
      break;

    case 6: // Transform
      {
        const matrix: [[number, number, number], [number, number, number]] = [
          [reader.readF32(), reader.readF32(), reader.readF32()],
          [reader.readF32(), reader.readF32(), reader.readF32()],
        ];
        canvas.save();
        canvas.setMatrix(matrix);
        visitRenderingTree(reader, canvas, onTopNodes);
        canvas.restore();
      }
      break;

    case 7: // OnTop
      {
        const matrix = canvas.getMatrix();
        const offset = reader.getOffset();
        onTopNodes.push({ matrix, readerOffset: offset });
        skipRenderingTree(reader);
      }
      break;

    case 8: // MouseCursor
      {
        skipMouseCursor(reader);
        visitRenderingTree(reader, canvas, onTopNodes);
      }
      break;

    default:
      throw new Error(`Unknown SpecialRenderingNode variant: ${variant}`);
  }
}

function visitDrawCommand(reader: BincodeReader, canvas: Canvas): void {
  const variant = reader.readVarintU32();

  switch (variant) {
    case 0: // Path
      visitPathDrawCommand(reader, canvas);
      break;

    case 1: // Text
      visitTextDrawCommand(reader, canvas);
      break;

    case 2: // Image
      visitImageDrawCommand(reader, canvas);
      break;

    default:
      throw new Error(`Unknown DrawCommand variant: ${variant}`);
  }
}

function visitPathDrawCommand(reader: BincodeReader, canvas: Canvas): void {
  // TODO: 실제 구현
  skipPath(reader);
  skipPaint(reader);
}

function visitTextDrawCommand(reader: BincodeReader, canvas: Canvas): void {
  // TODO: 실제 구현
  const text = reader.readString();
  skipFont(reader);
  const x = reader.readF32();
  const y = reader.readF32();
  skipPaint(reader);
  const align = reader.readVarintU32();
  const baseline = reader.readVarintU32();
  const hasMaxWidth = reader.readU8();
  if (hasMaxWidth) {
    reader.readF32();
  }
  const lineHeightPercent = reader.readF32();
  const hasUnderline = reader.readU8();
  if (hasUnderline) {
    skipPaint(reader);
  }
}

function visitImageDrawCommand(reader: BincodeReader, canvas: Canvas): void {
  // TODO: 실제 구현
  skipRect(reader);
  skipImage(reader);
  const fit = reader.readVarintU32();
  const hasPaint = reader.readU8();
  if (hasPaint) {
    skipPaint(reader);
  }
}

// Skip functions (구조를 읽지만 사용하지 않고 넘어감)
function skipRenderingTree(reader: BincodeReader): void {
  const variant = reader.readVarintU32();
  switch (variant) {
    case 0: // Empty
      break;
    case 1: // Node
      skipDrawCommand(reader);
      break;
    case 2: // Children
      {
        const len = reader.readVarintU64();
        for (let i = 0; i < len; i++) {
          skipRenderingTree(reader);
        }
      }
      break;
    case 3: // Special
      skipSpecialRenderingNode(reader);
      break;
    case 4: // Boxed
      skipRenderingTree(reader);
      break;
    case 5: // BoxedChildren
      {
        const len = reader.readVarintU64();
        for (let i = 0; i < len; i++) {
          skipRenderingTree(reader);
        }
      }
      break;
  }
}

function skipSpecialRenderingNode(reader: BincodeReader): void {
  const variant = reader.readVarintU32();
  switch (variant) {
    case 0: // Translate
      reader.readF32();
      reader.readF32();
      skipRenderingTree(reader);
      break;
    case 1: // Clip
      skipPath(reader);
      reader.readVarintU32();
      skipRenderingTree(reader);
      break;
    case 2: // WithId
      reader.readString();
      skipRenderingTree(reader);
      break;
    case 3: // Absolute
      reader.readF32();
      reader.readF32();
      skipRenderingTree(reader);
      break;
    case 4: // Rotate
      reader.readF32();
      skipRenderingTree(reader);
      break;
    case 5: // Scale
      reader.readF32();
      reader.readF32();
      skipRenderingTree(reader);
      break;
    case 6: // Transform
      for (let i = 0; i < 6; i++) reader.readF32();
      skipRenderingTree(reader);
      break;
    case 7: // OnTop
      skipRenderingTree(reader);
      break;
    case 8: // MouseCursor
      skipMouseCursor(reader);
      skipRenderingTree(reader);
      break;
  }
}

function skipDrawCommand(reader: BincodeReader): void {
  const variant = reader.readVarintU32();
  switch (variant) {
    case 0:
      skipPath(reader);
      skipPaint(reader);
      break;
    case 1:
      reader.readString();
      skipFont(reader);
      reader.readF32();
      reader.readF32();
      skipPaint(reader);
      reader.readVarintU32();
      reader.readVarintU32();
      if (reader.readU8()) reader.readF32();
      reader.readF32();
      if (reader.readU8()) skipPaint(reader);
      break;
    case 2:
      skipRect(reader);
      skipImage(reader);
      reader.readVarintU32();
      if (reader.readU8()) skipPaint(reader);
      break;
  }
}

function skipPath(reader: BincodeReader): void {
  // TODO: 실제 Path 구조에 맞게 구현
}

function skipPaint(reader: BincodeReader): void {
  // TODO: 실제 Paint 구조에 맞게 구현
}

function skipFont(reader: BincodeReader): void {
  // TODO: 실제 Font 구조에 맞게 구현
}

function skipRect(reader: BincodeReader): void {
  reader.readF32(); // left
  reader.readF32(); // top
  reader.readF32(); // right
  reader.readF32(); // bottom
}

function skipImage(reader: BincodeReader): void {
  // TODO: 실제 Image 구조에 맞게 구현
}

function skipMouseCursor(reader: BincodeReader): void {
  // TODO: 실제 MouseCursor 구조에 맞게 구현
}

import { BincodeReader } from "./reader";

// Global canvas, image map, and onTopNodes for rendering
let globalCanvas: Canvas = null as any;
let globalImageMap: Map<bigint, HTMLImageElement | ImageBitmap> | undefined;
let globalOnTopNodes: OnTopContext[] = [];

export function setCanvas(canvas: Canvas): void {
    globalCanvas = canvas;
}

export function getCanvas(): Canvas {
    return globalCanvas;
}

export function setImageMap(
    imageMap: Map<bigint, HTMLImageElement | ImageBitmap>,
): void {
    globalImageMap = imageMap;
}

export function getImageMap():
    | Map<bigint, HTMLImageElement | ImageBitmap>
    | undefined {
    return globalImageMap;
}

export function getOnTopNodes(): OnTopContext[] {
    return globalOnTopNodes;
}

export function clearOnTopNodes(): void {
    globalOnTopNodes = [];
}

// Canvas wrapper for web canvas API
export class Canvas {
    private ctx: CanvasRenderingContext2D;

    constructor(ctx: CanvasRenderingContext2D) {
        this.ctx = ctx;
    }

    save(): void {
        this.ctx.save();
    }

    restore(): void {
        this.ctx.restore();
    }

    translate(x: number, y: number): void {
        this.ctx.translate(x, y);
    }

    rotate(angle: number): void {
        this.ctx.rotate(angle);
    }

    scale(x: number, y: number): void {
        this.ctx.scale(x, y);
    }

    setMatrix(
        matrix: [[number, number, number], [number, number, number]],
    ): void {
        // matrix = [[a, c, tx], [b, d, ty]]
        // setTransform(a, b, c, d, tx, ty)
        this.ctx.setTransform(
            matrix[0][0],
            matrix[1][0],
            matrix[0][1],
            matrix[1][1],
            matrix[0][2],
            matrix[1][2],
        );
    }

    getMatrix(): [[number, number, number], [number, number, number]] {
        const transform = this.ctx.getTransform();
        return [
            [transform.a, transform.c, transform.e],
            [transform.b, transform.d, transform.f],
        ];
    }

    clipPath(path: Path2D, clipOp: number): void {
        // clipOp: 0 = Intersect, 1 = Difference
        // Web Canvas API는 Intersect만 지원 (clip 메서드는 항상 intersect처럼 동작)
        if (clipOp === 0) {
            this.ctx.clip(path);
        } else {
            // Difference는 지원되지 않음
            console.warn(
                "ClipOp Difference is not supported in web canvas API",
            );
            // 일단 무시하고 진행
        }
    }

    getContext(): CanvasRenderingContext2D {
        return this.ctx;
    }
}

interface OnTopContext {
    matrix: [[number, number, number], [number, number, number]];
    readerOffset: number;
}

export function visitRenderingTree(reader: BincodeReader): void {
    const variant = reader.readVarintU32();

    switch (variant) {
        case 0: // Empty
            break;

        case 1: // Node
            visitDrawCommand(reader);
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
                    const childReader = new BincodeReader(
                        buffer.slice(offsets[i]),
                    );
                    visitRenderingTree(childReader);
                }
            }
            break;

        case 3: // Special
            visitSpecialRenderingNode(reader);
            break;

        case 4: // Boxed
            visitRenderingTree(reader);
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
                    const childReader = new BincodeReader(
                        buffer.slice(offsets[i]),
                    );
                    visitRenderingTree(childReader);
                }
            }
            break;

        default:
            throw new Error(`Unknown RenderingTree variant: ${variant}`);
    }
}

function visitSpecialRenderingNode(reader: BincodeReader): void {
    const variant = reader.readVarintU32();

    switch (variant) {
        case 0: // Translate
            {
                const x = reader.readF32();
                const y = reader.readF32();
                globalCanvas.save();
                globalCanvas.translate(x, y);
                visitRenderingTree(reader);
                globalCanvas.restore();
            }
            break;

        case 1: // Clip
            {
                const path = readPath(reader);
                const clipOp = reader.readVarintU32();
                globalCanvas.save();
                globalCanvas.clipPath(path, clipOp);
                visitRenderingTree(reader);
                globalCanvas.restore();
            }
            break;

        case 2: // WithId
            {
                reader.readString(); // id
                visitRenderingTree(reader);
            }
            break;

        case 3: // Absolute
            {
                const x = reader.readF32();
                const y = reader.readF32();
                globalCanvas.save();
                globalCanvas.setMatrix([
                    [1.0, 0.0, x],
                    [0.0, 1.0, y],
                ]);
                visitRenderingTree(reader);
                globalCanvas.restore();
            }
            break;

        case 4: // Rotate
            {
                const angle = reader.readF32();
                globalCanvas.save();
                globalCanvas.rotate(angle);
                visitRenderingTree(reader);
                globalCanvas.restore();
            }
            break;

        case 5: // Scale
            {
                const x = reader.readF32();
                const y = reader.readF32();
                globalCanvas.save();
                globalCanvas.scale(x, y);
                visitRenderingTree(reader);
                globalCanvas.restore();
            }
            break;

        case 6: // Transform
            {
                const matrix: [
                    [number, number, number],
                    [number, number, number],
                ] = [
                    [reader.readF32(), reader.readF32(), reader.readF32()],
                    [reader.readF32(), reader.readF32(), reader.readF32()],
                ];
                globalCanvas.save();
                globalCanvas.setMatrix(matrix);
                visitRenderingTree(reader);
                globalCanvas.restore();
            }
            break;

        case 7: // OnTop
            {
                const matrix = globalCanvas.getMatrix();
                const offset = reader.getOffset();
                globalOnTopNodes.push({ matrix, readerOffset: offset });
                skipRenderingTree(reader);
            }
            break;

        case 8: // MouseCursor
            {
                skipMouseCursor(reader);
                visitRenderingTree(reader);
            }
            break;

        default:
            throw new Error(`Unknown SpecialRenderingNode variant: ${variant}`);
    }
}

function visitDrawCommand(reader: BincodeReader): void {
    const variant = reader.readVarintU32();

    switch (variant) {
        case 0: // Path
            visitPathDrawCommand(reader);
            break;

        case 1: // Text
            visitTextDrawCommand(reader);
            break;

        case 2: // Image
            visitImageDrawCommand(reader);
            break;

        default:
            throw new Error(`Unknown DrawCommand variant: ${variant}`);
    }
}

function visitPathDrawCommand(reader: BincodeReader): void {
    const path = readPath(reader);
    const paint = readPaint(reader);
    const ctx = globalCanvas.getContext();

    applyPaint(ctx, paint);

    if (paint.style === "fill") {
        ctx.fill(path);
    } else if (paint.style === "stroke") {
        ctx.stroke(path);
    } else {
        // default is fill
        ctx.fill(path);
    }
}

interface Paint {
    color: { r: number; g: number; b: number; a: number };
    style?: "fill" | "stroke";
    antiAlias?: boolean;
    strokeWidth?: number;
    strokeCap?: "butt" | "round" | "square";
    strokeJoin?: "bevel" | "miter" | "round";
    strokeMiter?: number;
    blendMode?: string;
}

function readPaint(reader: BincodeReader): Paint {
    const r = reader.readU8();
    const g = reader.readU8();
    const b = reader.readU8();
    const a = reader.readU8();

    const paint: Paint = {
        color: { r, g, b, a },
    };

    // paint_style: Option<PaintStyle>
    if (reader.readU8()) {
        const style = reader.readVarintU32();
        paint.style = style === 0 ? "fill" : "stroke";
    }

    // anti_alias: Option<bool>
    if (reader.readU8()) {
        paint.antiAlias = reader.readU8() !== 0;
    }

    // stroke_width: Px
    paint.strokeWidth = reader.readF32();

    // stroke_cap: Option<StrokeCap>
    if (reader.readU8()) {
        const cap = reader.readVarintU32();
        paint.strokeCap = cap === 0 ? "butt" : cap === 1 ? "round" : "square";
    }

    // stroke_join: Option<StrokeJoin>
    if (reader.readU8()) {
        const join = reader.readVarintU32();
        paint.strokeJoin =
            join === 0 ? "bevel" : join === 1 ? "miter" : "round";
    }

    // stroke_miter: Px
    paint.strokeMiter = reader.readF32();

    // color_filter: Option<ColorFilter>
    if (reader.readU8()) skipColorFilter(reader);

    // blend_mode: Option<BlendMode>
    if (reader.readU8()) {
        const mode = reader.readVarintU32();
        paint.blendMode = getBlendMode(mode);
    }

    // shader: Option<Box<Shader>>
    if (reader.readU8()) skipShader(reader);

    // mask_filter: Option<MaskFilter>
    if (reader.readU8()) skipMaskFilter(reader);

    // image_filter: Option<Box<ImageFilter>>
    if (reader.readU8()) skipImageFilter(reader);

    return paint;
}

function applyPaint(ctx: CanvasRenderingContext2D, paint: Paint): void {
    const { r, g, b, a } = paint.color;
    const color = `rgba(${r}, ${g}, ${b}, ${a / 255})`;

    if (paint.style === "fill") {
        ctx.fillStyle = color;
    } else if (paint.style === "stroke") {
        ctx.strokeStyle = color;
        if (paint.strokeWidth !== undefined) {
            ctx.lineWidth = paint.strokeWidth;
        }
        if (paint.strokeCap) {
            ctx.lineCap = paint.strokeCap;
        }
        if (paint.strokeJoin) {
            ctx.lineJoin = paint.strokeJoin;
        }
        if (paint.strokeMiter !== undefined) {
            ctx.miterLimit = paint.strokeMiter;
        }
    } else {
        ctx.fillStyle = color;
    }

    if (paint.blendMode) {
        ctx.globalCompositeOperation =
            paint.blendMode as GlobalCompositeOperation;
    }
}

function getBlendMode(mode: number): string {
    const modes = [
        "clear",
        "source-over", // Src
        "destination-over", // Dst
        "source-over", // SrcOver
        "destination-over", // DstOver
        "source-in", // SrcIn
        "destination-in", // DstIn
        "source-out", // SrcOut
        "destination-out", // DstOut
        "source-atop", // SrcATop
        "destination-atop", // DstATop
        "xor", // Xor
        "lighter", // Plus
        "multiply", // Modulate
        "screen", // Screen
        "overlay", // Overlay
        "darken", // Darken
        "lighten", // Lighten
        "color-dodge", // ColorDodge
        "color-burn", // ColorBurn
        "hard-light", // HardLight
        "soft-light", // SoftLight
        "difference", // Difference
        "exclusion", // Exclusion
        "multiply", // Multiply
        "hue", // Hue
        "saturation", // Saturation
        "color", // Color
        "luminosity", // Luminosity
    ];
    return modes[mode] || "source-over";
}

function visitTextDrawCommand(reader: BincodeReader): void {
    const text = reader.readString();
    const font = readFont(reader);
    const x = reader.readF32();
    const y = reader.readF32();
    const paint = readPaint(reader);
    const align = reader.readVarintU32(); // 0: Left, 1: Center, 2: Right
    const baseline = reader.readVarintU32(); // 0: Top, 1: Middle, 2: Bottom
    const hasMaxWidth = reader.readU8();
    let maxWidth: number | undefined;
    if (hasMaxWidth) {
        maxWidth = reader.readF32();
    }
    const _lineHeightPercent = reader.readF32(); // TODO: multiline text 지원 시 사용
    const hasUnderline = reader.readU8();
    let underlinePaint: Paint | undefined;
    if (hasUnderline) {
        underlinePaint = readPaint(reader);
    }

    const ctx = globalCanvas.getContext();

    // Set font
    // ctx.font = `${font.size}px ${font.name}`;
    ctx.font = "16px Arial";

    // Set text align
    ctx.textAlign = align === 0 ? "left" : align === 1 ? "center" : "right";

    // Set text baseline
    ctx.textBaseline =
        baseline === 0 ? "top" : baseline === 1 ? "middle" : "bottom";

    // Apply paint
    applyPaint(ctx, paint);

    // Draw text
    if (paint.style === "stroke") {
        ctx.strokeText(text, x, y, maxWidth);
    } else {
        ctx.fillText(text, x, y, maxWidth);
    }

    // Draw underline if exists
    if (underlinePaint) {
        const metrics = ctx.measureText(text);
        const underlineY = y + 2; // 텍스트 아래 2px
        const textWidth =
            maxWidth && metrics.width > maxWidth ? maxWidth : metrics.width;

        ctx.save();
        applyPaint(ctx, underlinePaint);
        ctx.beginPath();
        ctx.moveTo(x, underlineY);
        ctx.lineTo(x + textWidth, underlineY);
        ctx.stroke();
        ctx.restore();
    }
}

interface Font {
    size: number;
    name: string;
}

function readFont(reader: BincodeReader): Font {
    const size = reader.readVarintI32();
    const name = reader.readString();
    return { size, name };
}

function visitImageDrawCommand(reader: BincodeReader): void {
    const rect = readRect(reader);
    const image = readImage(reader);
    const fit = reader.readVarintU32(); // 0: Fill, 1: Contain, 2: Cover, 3: ScaleDown, 4: None
    const hasPaint = reader.readU8();
    let paint: Paint | undefined;
    if (hasPaint) {
        paint = readPaint(reader);
    }

    const ctx = globalCanvas.getContext();

    // Get image from map
    const imageElement = globalImageMap?.get(image.id);
    if (!imageElement) {
        console.warn(`Image not found: ${image.id}`);
        return;
    }

    // Calculate source and destination rectangles based on fit
    const imageSize = { width: image.info.width, height: image.info.height };
    const { srcRect, destRect } = getSrcDestRectsInFit(fit, imageSize, rect);

    ctx.save();

    if (paint) {
        applyPaint(ctx, paint);
    }

    ctx.drawImage(
        imageElement,
        srcRect.x,
        srcRect.y,
        srcRect.width,
        srcRect.height,
        destRect.x,
        destRect.y,
        destRect.width,
        destRect.height,
    );

    ctx.restore();
}

interface ImageInfo {
    alphaType: number;
    colorType: number;
    height: number;
    width: number;
}

interface Image {
    info: ImageInfo;
    id: bigint;
}

function readImage(reader: BincodeReader): Image {
    const alphaType = reader.readVarintU32();
    const colorType = reader.readVarintU32();
    const height = reader.readF32();
    const width = reader.readF32();
    const id = reader.readU64();

    return {
        info: { alphaType, colorType, height, width },
        id,
    };
}

function getSrcDestRectsInFit(
    fit: number,
    imageSize: { width: number; height: number },
    commandRect: { x: number; y: number; width: number; height: number },
): {
    srcRect: { x: number; y: number; width: number; height: number };
    destRect: { x: number; y: number; width: number; height: number };
} {
    switch (fit) {
        case 0: // Fill
            return {
                srcRect: {
                    x: 0,
                    y: 0,
                    width: imageSize.width,
                    height: imageSize.height,
                },
                destRect: commandRect,
            };

        case 1: {
            // Contain
            const destRect = calculateContainFitDestRect(
                imageSize,
                commandRect,
            );
            return {
                srcRect: {
                    x: 0,
                    y: 0,
                    width: imageSize.width,
                    height: imageSize.height,
                },
                destRect,
            };
        }

        case 2: {
            // Cover
            const srcRect = calculateCoverFitSrcRect(imageSize, commandRect);
            return { srcRect, destRect: commandRect };
        }

        case 3: {
            // ScaleDown
            const containResult = getSrcDestRectsInFit(
                1,
                imageSize,
                commandRect,
            );
            const noneResult = getSrcDestRectsInFit(4, imageSize, commandRect);

            if (
                containResult.destRect.width < noneResult.destRect.width ||
                containResult.destRect.height < noneResult.destRect.height
            ) {
                return containResult;
            } else {
                return noneResult;
            }
        }

        case 4: // None
            return calculateNoneFitRects(imageSize, commandRect);

        default:
            return {
                srcRect: {
                    x: 0,
                    y: 0,
                    width: imageSize.width,
                    height: imageSize.height,
                },
                destRect: commandRect,
            };
    }
}

function calculateContainFitDestRect(
    imageSize: { width: number; height: number },
    commandRect: { x: number; y: number; width: number; height: number },
): { x: number; y: number; width: number; height: number } {
    if (
        imageSize.width / imageSize.height ===
        commandRect.width / commandRect.height
    ) {
        return commandRect;
    }

    if (
        imageSize.width / imageSize.height >
        commandRect.width / commandRect.height
    ) {
        const k = commandRect.width / imageSize.width;
        const deltaY = (commandRect.height - imageSize.height * k) / 2;
        return {
            x: commandRect.x,
            y: commandRect.y + deltaY,
            width: commandRect.width,
            height: imageSize.height * k,
        };
    }

    const k = commandRect.height / imageSize.height;
    const deltaX = (commandRect.width - imageSize.width * k) / 2;
    return {
        x: commandRect.x + deltaX,
        y: commandRect.y,
        width: imageSize.width * k,
        height: commandRect.height,
    };
}

function calculateCoverFitSrcRect(
    imageSize: { width: number; height: number },
    commandRect: { x: number; y: number; width: number; height: number },
): { x: number; y: number; width: number; height: number } {
    // width fit case
    let width = imageSize.width;
    let height = width * (commandRect.height / commandRect.width);

    if (height <= imageSize.height) {
        return {
            x: 0,
            y: (imageSize.height - height) / 2,
            width,
            height,
        };
    }

    // height fit case
    height = imageSize.height;
    width = height * (commandRect.width / commandRect.height);

    return {
        x: (imageSize.width - width) / 2,
        y: 0,
        width,
        height,
    };
}

function calculateNoneFitRects(
    imageSize: { width: number; height: number },
    commandRect: { x: number; y: number; width: number; height: number },
): {
    srcRect: { x: number; y: number; width: number; height: number };
    destRect: { x: number; y: number; width: number; height: number };
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
        imageSize.width <= commandRect.width
            ? imageSize.width
            : commandRect.width;
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
    const len = reader.readVarintU64();
    for (let i = 0; i < len; i++) {
        skipPathCommand(reader);
    }
}

function skipPathCommand(reader: BincodeReader): void {
    const variant = reader.readVarintU32();
    switch (variant) {
        case 0: // AddRect
            skipRect(reader);
            break;
        case 1: // AddRrect
            skipRect(reader);
            reader.readF32(); // rx
            reader.readF32(); // ry
            break;
        case 2: // Stroke
            skipStrokeOptions(reader);
            break;
        case 3: // MoveTo
            reader.readF32(); // x
            reader.readF32(); // y
            break;
        case 4: // LineTo
            reader.readF32(); // x
            reader.readF32(); // y
            break;
        case 5: // CubicTo
            reader.readF32(); // first_x
            reader.readF32(); // first_y
            reader.readF32(); // second_x
            reader.readF32(); // second_y
            reader.readF32(); // end_x
            reader.readF32(); // end_y
            break;
        case 6: // ArcTo
            skipRect(reader); // oval
            skipAngle(reader); // start_angle
            skipAngle(reader); // delta_angle
            break;
        case 7: // Scale
            reader.readF32(); // x
            reader.readF32(); // y
            break;
        case 8: // Translate
            reader.readF32(); // x
            reader.readF32(); // y
            break;
        case 9: // Transform
            for (let i = 0; i < 6; i++) reader.readF32(); // matrix
            break;
        case 10: // AddOval
            skipRect(reader);
            break;
        case 11: // AddArc
            skipRect(reader); // oval
            skipAngle(reader); // start_angle
            skipAngle(reader); // delta_angle
            break;
        case 12: // AddPoly
            {
                const len = reader.readVarintU64();
                for (let i = 0; i < len; i++) {
                    reader.readF32(); // x
                    reader.readF32(); // y
                }
                reader.readU8(); // close
            }
            break;
        case 13: // Close
            break;
        default:
            throw new Error(`Unknown PathCommand variant: ${variant}`);
    }
}

function skipStrokeOptions(reader: BincodeReader): void {
    // width: Option<Px>
    if (reader.readU8()) reader.readF32();
    // miter_limit: Option<Px>
    if (reader.readU8()) reader.readF32();
    // precision: Option<OrderedFloat>
    if (reader.readU8()) reader.readF32();
    // join: Option<StrokeJoin>
    if (reader.readU8()) reader.readVarintU32();
    // cap: Option<StrokeCap>
    if (reader.readU8()) reader.readVarintU32();
}

function skipAngle(reader: BincodeReader): void {
    reader.readF32(); // radians
}

function readPath(reader: BincodeReader): Path2D {
    const path = new Path2D();
    const len = reader.readVarintU64();

    for (let i = 0; i < len; i++) {
        const variant = reader.readVarintU32();

        switch (variant) {
            case 0: // AddRect
                {
                    const rect = readRect(reader);
                    path.rect(rect.x, rect.y, rect.width, rect.height);
                }
                break;
            case 1: // AddRrect
                {
                    const rect = readRect(reader);
                    const rx = reader.readF32();
                    const ry = reader.readF32();
                    path.roundRect(rect.x, rect.y, rect.width, rect.height, [
                        rx,
                        ry,
                    ]);
                }
                break;
            case 2: // Stroke
                skipStrokeOptions(reader);
                // Stroke는 path 자체가 아니라 paint 설정이므로 무시
                break;
            case 3: // MoveTo
                {
                    const x = reader.readF32();
                    const y = reader.readF32();
                    path.moveTo(x, y);
                }
                break;
            case 4: // LineTo
                {
                    const x = reader.readF32();
                    const y = reader.readF32();
                    path.lineTo(x, y);
                }
                break;
            case 5: // CubicTo
                {
                    const x1 = reader.readF32();
                    const y1 = reader.readF32();
                    const x2 = reader.readF32();
                    const y2 = reader.readF32();
                    const x = reader.readF32();
                    const y = reader.readF32();
                    path.bezierCurveTo(x1, y1, x2, y2, x, y);
                }
                break;
            case 6: // ArcTo
                {
                    const oval = readRect(reader);
                    const startAngle = reader.readF32();
                    const deltaAngle = reader.readF32();
                    const centerX = oval.x + oval.width / 2;
                    const centerY = oval.y + oval.height / 2;
                    const radiusX = oval.width / 2;
                    const radiusY = oval.height / 2;

                    if (radiusX === radiusY) {
                        path.arc(
                            centerX,
                            centerY,
                            radiusX,
                            startAngle,
                            startAngle + deltaAngle,
                            deltaAngle < 0,
                        );
                    } else {
                        path.ellipse(
                            centerX,
                            centerY,
                            radiusX,
                            radiusY,
                            0,
                            startAngle,
                            startAngle + deltaAngle,
                            deltaAngle < 0,
                        );
                    }
                }
                break;
            case 7: // Scale
            case 8: // Translate
            case 9: // Transform
                // Path transform은 나중에 처리
                if (variant === 7) {
                    reader.readF32();
                    reader.readF32();
                } else if (variant === 8) {
                    reader.readF32();
                    reader.readF32();
                } else {
                    for (let i = 0; i < 6; i++) reader.readF32();
                }
                break;
            case 10: // AddOval
                {
                    const rect = readRect(reader);
                    const centerX = rect.x + rect.width / 2;
                    const centerY = rect.y + rect.height / 2;
                    const radiusX = rect.width / 2;
                    const radiusY = rect.height / 2;

                    if (radiusX === radiusY) {
                        path.arc(centerX, centerY, radiusX, 0, Math.PI * 2);
                    } else {
                        path.ellipse(
                            centerX,
                            centerY,
                            radiusX,
                            radiusY,
                            0,
                            0,
                            Math.PI * 2,
                        );
                    }
                }
                break;
            case 11: // AddArc
                {
                    const oval = readRect(reader);
                    const startAngle = reader.readF32();
                    const deltaAngle = reader.readF32();
                    const centerX = oval.x + oval.width / 2;
                    const centerY = oval.y + oval.height / 2;
                    const radiusX = oval.width / 2;
                    const radiusY = oval.height / 2;

                    if (radiusX === radiusY) {
                        path.arc(
                            centerX,
                            centerY,
                            radiusX,
                            startAngle,
                            startAngle + deltaAngle,
                            deltaAngle < 0,
                        );
                    } else {
                        path.ellipse(
                            centerX,
                            centerY,
                            radiusX,
                            radiusY,
                            0,
                            startAngle,
                            startAngle + deltaAngle,
                            deltaAngle < 0,
                        );
                    }
                }
                break;
            case 12: // AddPoly
                {
                    const len = reader.readVarintU64();
                    for (let i = 0; i < len; i++) {
                        const x = reader.readF32();
                        const y = reader.readF32();
                        if (i === 0) {
                            path.moveTo(x, y);
                        } else {
                            path.lineTo(x, y);
                        }
                    }
                    const close = reader.readU8();
                    if (close) {
                        path.closePath();
                    }
                }
                break;
            case 13: // Close
                path.closePath();
                break;
            default:
                throw new Error(`Unknown PathCommand variant: ${variant}`);
        }
    }

    return path;
}

function readRect(reader: BincodeReader): {
    x: number;
    y: number;
    width: number;
    height: number;
} {
    const left = reader.readF32();
    const top = reader.readF32();
    const right = reader.readF32();
    const bottom = reader.readF32();
    return {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    };
}

function skipPaint(reader: BincodeReader): void {
    // color: Color (r, g, b, a)
    reader.readU8(); // r
    reader.readU8(); // g
    reader.readU8(); // b
    reader.readU8(); // a

    // paint_style: Option<PaintStyle>
    if (reader.readU8()) reader.readVarintU32();

    // anti_alias: Option<bool>
    if (reader.readU8()) reader.readU8();

    // stroke_width: Px
    reader.readF32();

    // stroke_cap: Option<StrokeCap>
    if (reader.readU8()) reader.readVarintU32();

    // stroke_join: Option<StrokeJoin>
    if (reader.readU8()) reader.readVarintU32();

    // stroke_miter: Px
    reader.readF32();

    // color_filter: Option<ColorFilter>
    if (reader.readU8()) skipColorFilter(reader);

    // blend_mode: Option<BlendMode>
    if (reader.readU8()) reader.readVarintU32();

    // shader: Option<Box<Shader>>
    if (reader.readU8()) skipShader(reader);

    // mask_filter: Option<MaskFilter>
    if (reader.readU8()) skipMaskFilter(reader);

    // image_filter: Option<Box<ImageFilter>>
    if (reader.readU8()) skipImageFilter(reader);
}

function skipFont(reader: BincodeReader): void {
    // size: IntPx (varint i32)
    reader.readVarintI32();
    // name: String
    reader.readString();
}

function skipRect(reader: BincodeReader): void {
    reader.readF32(); // left
    reader.readF32(); // top
    reader.readF32(); // right
    reader.readF32(); // bottom
}

function skipImage(reader: BincodeReader): void {
    // info: ImageInfo
    reader.readVarintU32(); // alpha_type
    reader.readVarintU32(); // color_type
    reader.readF32(); // height
    reader.readF32(); // width

    // image_id: Arc<SkiaImageId>
    reader.readU64(); // id
}

function skipMouseCursor(reader: BincodeReader): void {
    reader.readVarintU32(); // variant
    // MouseCursor enum variants (예상)
    // 대부분 데이터가 없거나 적음
}

function skipColorFilter(reader: BincodeReader): void {
    const variant = reader.readVarintU32();
    switch (variant) {
        case 0: // Matrix
            for (let i = 0; i < 20; i++) reader.readF32();
            break;
        case 1: // Blend
            reader.readU8(); // r
            reader.readU8(); // g
            reader.readU8(); // b
            reader.readU8(); // a
            reader.readVarintU32(); // blend_mode
            break;
        default:
            break;
    }
}

function skipShader(reader: BincodeReader): void {
    const variant = reader.readVarintU32();
    switch (variant) {
        case 0: // Color
            reader.readU8(); // r
            reader.readU8(); // g
            reader.readU8(); // b
            reader.readU8(); // a
            break;
        case 1: // LinearGradient
            {
                // start_xy
                reader.readF32();
                reader.readF32();
                // end_xy
                reader.readF32();
                reader.readF32();
                // colors
                const colorLen = reader.readVarintU64();
                for (let i = 0; i < colorLen; i++) {
                    reader.readU8();
                    reader.readU8();
                    reader.readU8();
                    reader.readU8();
                }
                // positions
                const posLen = reader.readVarintU64();
                for (let i = 0; i < posLen; i++) {
                    reader.readF32();
                }
                // tile_mode
                reader.readVarintU32();
            }
            break;
        case 2: // RadialGradient
            {
                // center
                reader.readF32();
                reader.readF32();
                // radius
                reader.readF32();
                // colors
                const colorLen = reader.readVarintU64();
                for (let i = 0; i < colorLen; i++) {
                    reader.readU8();
                    reader.readU8();
                    reader.readU8();
                    reader.readU8();
                }
                // positions
                const posLen = reader.readVarintU64();
                for (let i = 0; i < posLen; i++) {
                    reader.readF32();
                }
                // tile_mode
                reader.readVarintU32();
            }
            break;
        case 3: // Image
            skipImage(reader);
            // tile_mode_xy
            reader.readVarintU32();
            reader.readVarintU32();
            break;
        default:
            break;
    }
}

function skipMaskFilter(reader: BincodeReader): void {
    const variant = reader.readVarintU32();
    switch (variant) {
        case 0: // Blur
            reader.readVarintU32(); // blur_style
            reader.readF32(); // sigma
            if (reader.readU8()) {
                // respect_ctm
                reader.readU8();
            }
            break;
        default:
            break;
    }
}

function skipImageFilter(reader: BincodeReader): void {
    const variant = reader.readVarintU32();
    switch (variant) {
        case 0: // Blur
            reader.readF32(); // sigma_x
            reader.readF32(); // sigma_y
            reader.readVarintU32(); // tile_mode
            if (reader.readU8()) {
                skipImageFilter(reader);
            }
            break;
        case 1: // DropShadow
            reader.readF32(); // dx
            reader.readF32(); // dy
            reader.readF32(); // sigma_x
            reader.readF32(); // sigma_y
            reader.readU8(); // r
            reader.readU8(); // g
            reader.readU8(); // b
            reader.readU8(); // a
            if (reader.readU8()) {
                skipImageFilter(reader);
            }
            break;
        default:
            if (reader.readU8()) {
                skipImageFilter(reader);
            }
            break;
    }
}

use super::*;

#[wasm_bindgen]
extern "C" {
    pub type CanvasKitCanvas;
    // /**
    //     /// Fills the current clip with the given color using Src BlendMode.
    //     /// This has the effect of replacing all pixels contained by clip with color.
    //     /// @param color
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn clear(this: &CanvasKitCanvas, color: InputColor);

    ///
    /// Replaces clip with the intersection or difference of the current clip and path,
    /// with an aliased or anti-aliased clip edge.
    /// @param path
    /// @param op
    /// @param doAntiAlias
    ///
    #[wasm_bindgen(method)]
    pub fn clipPath(
        this: &CanvasKitCanvas,
        path: &CanvasKitPath,
        op: CanvasKitClipOp,
        doAntiAlias: bool,
    );

    //     ///
    //     /// Replaces clip with the intersection or difference of the current clip and rect,
    //     /// with an aliased or anti-aliased clip edge.
    //     /// @param rect
    //     /// @param op
    //     /// @param doAntiAlias
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn clipRect(this: &CanvasKitCanvas, rect: InputRect, op: ClipOp, doAntiAlias: boolean);

    //     ///
    //     /// Replaces clip with the intersection or difference of the current clip and rrect,
    //     /// with an aliased or anti-aliased clip edge.
    //     /// @param rrect
    //     /// @param op
    //     /// @param doAntiAlias
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn clipRRect(this: &CanvasKitCanvas, rrect: InputRRect, op: ClipOp, doAntiAlias: boolean);

    //     ///
    //     /// Replaces current matrix with m premultiplied with the existing matrix.
    //     /// @param m
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn concat(this: &CanvasKitCanvas, m: InputMatrix);

    //     ///
    //     /// Draws arc using clip, Matrix, and Paint paint.
    //      *
    //     /// Arc is part of oval bounded by oval, sweeping from startAngle to startAngle plus
    //     /// sweepAngle. startAngle and sweepAngle are in degrees.
    //     /// @param oval - bounds of oval containing arc to draw
    //     /// @param startAngle - angle in degrees where arc begins
    //     /// @param sweepAngle - sweep angle in degrees; positive is clockwise
    //     /// @param useCenter - if true, include the center of the oval
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawArc(this: &CanvasKitCanvas, oval: InputRect, startAngle: AngleInDegrees, sweepAngle: AngleInDegrees,
    //             useCenter: boolean, paint: Paint);

    //     ///
    //     /// Draws a set of sprites from atlas, using clip, Matrix, and optional Paint paint.
    //     /// @param atlas - Image containing sprites
    //     /// @param srcRects - Rect locations of sprites in atlas
    //     /// @param dstXforms - RSXform mappings for sprites in atlas
    //     /// @param paint
    //     /// @param blendMode - BlendMode combining colors and sprites
    //     /// @param colors - If provided, will be blended with sprite using blendMode.
    //     /// @param sampling - Specifies sampling options. If null, bilinear is used.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawAtlas(this: &CanvasKitCanvas, atlas: Image, srcRects: InputFlattenedRectangleArray,
    //               dstXforms: InputFlattenedRSXFormArray, paint: Paint,
    //               blendMode?: BlendMode | null, colors?: ColorIntArray | null,
    //               sampling?: CubicResampler | FilterOptions);

    //     ///
    //     /// Draws a circle at (cx, cy) with the given radius.
    //     /// @param cx
    //     /// @param cy
    //     /// @param radius
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawCircle(this: &CanvasKitCanvas, cx: number, cy: number, radius: number, paint: Paint);

    //     ///
    //     /// Fills clip with the given color.
    //     /// @param color
    //     /// @param blendMode - defaults to SrcOver.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawColor(this: &CanvasKitCanvas, color: InputColor, blendMode?: BlendMode);

    //     ///
    //     /// Fills clip with the given color.
    //     /// @param r - red value (typically from 0 to 1.0).
    //     /// @param g - green value (typically from 0 to 1.0).
    //     /// @param b - blue value (typically from 0 to 1.0).
    //     /// @param a - alpha value, range 0 to 1.0 (1.0 is opaque).
    //     /// @param blendMode - defaults to SrcOver.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawColorComponents(this: &CanvasKitCanvas, r: number, g: number, b: number, a: number, blendMode?: BlendMode);

    //     ///
    //     /// Fills clip with the given color.
    //     /// @param color
    //     /// @param blendMode - defaults to SrcOver.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawColorInt(this: &CanvasKitCanvas, color: ColorInt, blendMode?: BlendMode);

    //     ///
    //     /// Draws RRect outer and inner using clip, Matrix, and Paint paint.
    //     /// outer must contain inner or the drawing is undefined.
    //     /// @param outer
    //     /// @param inner
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawDRRect(this: &CanvasKitCanvas, outer: InputRRect, inner: InputRRect, paint: Paint);

    //     ///
    //     /// Draws a run of glyphs, at corresponding positions, in a given font.
    //     /// @param glyphs the array of glyph IDs (Uint16TypedArray)
    //     /// @param positions the array of x,y floats to position each glyph
    //     /// @param x x-coordinate of the origin of the entire run
    //     /// @param y y-coordinate of the origin of the entire run
    //     /// @param font the font that contains the glyphs
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawGlyphs(this: &CanvasKitCanvas, glyphs: InputGlyphIDArray,
    //                positions: InputFlattenedPointArray,
    //                x: number, y: number,
    //                font: Font, paint: Paint);

    //     ///
    //     /// Draws the given image with its top-left corner at (left, top) using the current clip,
    //     /// the current matrix, and optionally-provided paint.
    //     /// @param img
    //     /// @param left
    //     /// @param top
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawImage(this: &CanvasKitCanvas, img: Image, left: number, top: number, paint?: Paint | null);

    //     ///
    //     /// Draws the given image with its top-left corner at (left, top) using the current clip,
    //     /// the current matrix. It will use the cubic sampling options B and C if necessary.
    //     /// @param img
    //     /// @param left
    //     /// @param top
    //     /// @param B - See CubicResampler in SkSamplingOptions.h for more information
    //     /// @param C - See CubicResampler in SkSamplingOptions.h for more information
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawImageCubic(this: &CanvasKitCanvas, img: Image, left: number, top: number, B: number, C: number,
    //                    paint?: Paint | null);

    //     ///
    //     /// Draws the given image with its top-left corner at (left, top) using the current clip,
    //     /// the current matrix. It will use the provided sampling options if necessary.
    //     /// @param img
    //     /// @param left
    //     /// @param top
    //     /// @param fm - The filter mode.
    //     /// @param mm - The mipmap mode. Note: for settings other than None, the image must have mipmaps
    //     ///             calculated with makeCopyWithDefaultMipmaps;
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawImageOptions(this: &CanvasKitCanvas, img: Image, left: number, top: number, fm: FilterMode,
    //                      mm: MipmapMode, paint?: Paint | null);

    //     ///
    //     ///  Draws the provided image stretched proportionally to fit into dst rectangle.
    //     ///  The center rectangle divides the image into nine sections: four sides, four corners, and
    //     ///  the center.
    //     /// @param img
    //     /// @param center
    //     /// @param dest
    //     /// @param filter - what technique to use when sampling the image
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawImageNine(this: &CanvasKitCanvas, img: Image, center: InputIRect, dest: InputRect, filter: FilterMode,
    //                   paint?: Paint | null);

    //     ///
    //     /// Draws sub-rectangle src from provided image, scaled and translated to fill dst rectangle.
    //     /// @param img
    //     /// @param src
    //     /// @param dest
    //     /// @param paint
    //     /// @param fastSample - if false, will filter strictly within src.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawImageRect(this: &CanvasKitCanvas, img: Image, src: InputRect, dest: InputRect, paint: Paint,
    //                   fastSample?: boolean);

    //     ///
    //     /// Draws sub-rectangle src from provided image, scaled and translated to fill dst rectangle.
    //     /// It will use the cubic sampling options B and C if necessary.
    //     /// @param img
    //     /// @param src
    //     /// @param dest
    //     /// @param B - See CubicResampler in SkSamplingOptions.h for more information
    //     /// @param C - See CubicResampler in SkSamplingOptions.h for more information
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawImageRectCubic(this: &CanvasKitCanvas, img: Image, src: InputRect, dest: InputRect,
    //                        B: number, C: number, paint?: Paint | null);

    //     ///
    //     /// Draws sub-rectangle src from provided image, scaled and translated to fill dst rectangle.
    //     /// It will use the provided sampling options if necessary.
    //     /// @param img
    //     /// @param src
    //     /// @param dest
    //     /// @param fm - The filter mode.
    //     /// @param mm - The mipmap mode. Note: for settings other than None, the image must have mipmaps
    //     ///             calculated with makeCopyWithDefaultMipmaps;
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawImageRectOptions(this: &CanvasKitCanvas, img: Image, src: InputRect, dest: InputRect, fm: FilterMode,
    //                          mm: MipmapMode, paint?: Paint | null);

    //     ///
    //     /// Draws line segment from (x0, y0) to (x1, y1) using the current clip, current matrix,
    //     /// and the provided paint.
    //     /// @param x0
    //     /// @param y0
    //     /// @param x1
    //     /// @param y1
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawLine(this: &CanvasKitCanvas, x0: number, y0: number, x1: number, y1: number, paint: Paint);

    //     ///
    //     /// Draws an oval bounded by the given rectangle using the current clip, current matrix,
    //     /// and the provided paint.
    //     /// @param oval
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawOval(this: &CanvasKitCanvas, oval: InputRect, paint: Paint);

    //     ///
    //     /// Fills clip with the given paint.
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawPaint(this: &CanvasKitCanvas, paint: Paint);

    //     ///
    //     /// Draws the given Paragraph at the provided coordinates.
    //     /// Requires the Paragraph code to be compiled in.
    //     /// @param p
    //     /// @param x
    //     /// @param y
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawParagraph(this: &CanvasKitCanvas, p: Paragraph, x: number, y: number);

    ///
    /// Draws the given path using the current clip, current matrix, and the provided paint.
    /// @param path
    /// @param paint
    ///
    #[wasm_bindgen(method)]
    pub fn drawPath(this: &CanvasKitCanvas, path: &CanvasKitPath, paint: &CanvasKitPaint);

    //     ///
    //     /// Draws a cubic patch defined by 12 control points [top, right, bottom, left] with optional
    //     /// colors and shader-coordinates [4] specifed for each corner [top-left, top-right, bottom-right, bottom-left]
    //     /// @param cubics 12 points : 4 connected cubics specifying the boundary of the patch
    //     /// @param colors optional colors interpolated across the patch
    //     /// @param texs optional shader coordinates interpolated across the patch
    //     /// @param mode Specifies how shader and colors blend (if both are specified)
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawPatch(this: &CanvasKitCanvas, cubics: InputFlattenedPointArray,
    //               colors?: ColorIntArray | Color[] | null,
    //               texs?: InputFlattenedPointArray | null,
    //               mode?: BlendMode | null,
    //               paint?: Paint);

    //     ///
    //     /// Draws the given picture using the current clip, current matrix, and the provided paint.
    //     /// @param skp
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawPicture(this: &CanvasKitCanvas, skp: SkPicture);

    //     ///
    //     /// Draws the given points using the current clip, current matrix, and the provided paint.
    //      *
    //     /// See Canvas.h for more on the mode and its interaction with paint.
    //     /// @param mode
    //     /// @param points
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawPoints(this: &CanvasKitCanvas, mode: PointMode, points: InputFlattenedPointArray, paint: Paint);

    //     ///
    //     /// Draws the given rectangle using the current clip, current matrix, and the provided paint.
    //     /// @param rect
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawRect(this: &CanvasKitCanvas, rect: InputRect, paint: Paint);

    //     ///
    //     /// Draws the given rectangle using the current clip, current matrix, and the provided paint.
    //     /// @param left
    //     /// @param top
    //     /// @param right
    //     /// @param bottom
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawRect4f(this: &CanvasKitCanvas, left: number, top: number, right: number, bottom: number, paint: Paint);

    //     ///
    //     /// Draws the given rectangle with rounded corners using the current clip, current matrix,
    //     /// and the provided paint.
    //     /// @param rrect
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawRRect(this: &CanvasKitCanvas, rrect: InputRRect, paint: Paint);

    //     ///
    //     /// Draw an offset spot shadow and outlining ambient shadow for the given path using a disc
    //     /// light. See SkShadowUtils.h for more details
    //     /// @param path - The occluder used to generate the shadows.
    //     /// @param zPlaneParams - Values for the plane function which returns the Z offset of the
    //     ///                       occluder from the canvas based on local x and y values (the current
    //     ///                       matrix is not applied).
    //     /// @param lightPos - The 3D position of the light relative to the canvas plane. This is
    //     ///                   independent of the canvas's current matrix.
    //     /// @param lightRadius - The radius of the disc light.
    //     /// @param ambientColor - The color of the ambient shadow.
    //     /// @param spotColor -  The color of the spot shadow.
    //     /// @param flags - See SkShadowFlags.h; 0 means use default options.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawShadow(this: &CanvasKitCanvas, path: Path, zPlaneParams: InputVector3, lightPos: InputVector3, lightRadius: number,
    //                ambientColor: InputColor, spotColor: InputColor, flags: number);

    //     ///
    //     /// Draw the given text at the location (x, y) using the provided paint and font. The text will
    //     /// be drawn as is; no shaping, left-to-right, etc.
    //     /// @param str
    //     /// @param x
    //     /// @param y
    //     /// @param paint
    //     /// @param font
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawText(this: &CanvasKitCanvas, str: string, x: number, y: number, paint: Paint, font: Font);

    ///
    /// Draws the given TextBlob at (x, y) using the current clip, current matrix, and the
    /// provided paint. Reminder that the fonts used to draw TextBlob are part of the blob.
    /// @param blob
    /// @param x
    /// @param y
    /// @param paint
    ///
    #[wasm_bindgen(method)]
    pub fn drawTextBlob(
        this: &CanvasKitCanvas,
        blob: &CanvasKitTextBlob,
        x: f32,
        y: f32,
        paint: &CanvasKitPaint,
    );

    //     ///
    //     /// Draws the given vertices (a triangle mesh) using the current clip, current matrix, and the
    //     /// provided paint.
    //     ///  If paint contains an Shader and vertices does not contain texCoords, the shader
    //     ///  is mapped using the vertices' positions.
    //     ///  If vertices colors are defined in vertices, and Paint paint contains Shader,
    //     ///  BlendMode mode combines vertices colors with Shader.
    //     /// @param verts
    //     /// @param mode
    //     /// @param paint
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn drawVertices(this: &CanvasKitCanvas, verts: Vertices, mode: BlendMode, paint: Paint);

    //     ///
    //     /// Returns the 4x4 matrix matching the given marker or null if there was none.
    //     /// See also markCTM.
    //     /// @param marker
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn findMarkedCTM(this: &CanvasKitCanvas, marker: string) -> Matrix4x4 | null;

    //     ///
    //     /// Returns the current transform from local coordinates to the 'device', which for most
    //     /// purposes means pixels.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn getLocalToDevice(this: &CanvasKitCanvas) -> Matrix4x4;

    //     ///
    //     /// Returns the number of saved states, each containing: Matrix and clip.
    //     /// Equals the number of save() calls less the number of restore() calls plus one.
    //     /// The save count of a new canvas is one.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn getSaveCount(this: &CanvasKitCanvas) -> number;

    //     ///
    //     /// Legacy version of getLocalToDevice(), which strips away any Z information, and
    //     /// just returns a 3x3 version.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn getTotalMatrix(this: &CanvasKitCanvas) -> number[];

    //     ///
    //     /// Creates Surface matching info and props, and associates it with Canvas.
    //     /// Returns null if no match found.
    //     /// @param info
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn makeSurface(this: &CanvasKitCanvas, info: ImageInfo) -> Surface | null;

    //     ///
    //     /// Record a marker (provided by caller) for the current CTM. This does not change anything
    //     /// about the ctm or clip, but does "name" this matrix value, so it can be referenced by
    //     /// custom effects (who access it by specifying the same name).
    //     /// See also findMarkedCTM.
    //     /// @param marker
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn markCTM(this: &CanvasKitCanvas, marker: string);

    //     ///
    //     /// Returns a TypedArray containing the pixels reading starting at (srcX, srcY) and does not
    //     /// exceed the size indicated by imageInfo. See SkCanvas.h for more on the caveats.
    //      *
    //     /// If dest is not provided, we allocate memory equal to the provided height * the provided
    //     /// bytesPerRow to fill the data with.
    //      *
    //     /// This is generally a very expensive call for the GPU backend.
    //      *
    //     /// @param srcX
    //     /// @param srcY
    //     /// @param imageInfo - describes the destination format of the pixels.
    //     /// @param dest - If provided, the pixels will be copied into the allocated buffer allowing
    //     ///        access to the pixels without allocating a new TypedArray.
    //     /// @param bytesPerRow - number of bytes per row. Must be provided if dest is set. This
    //     ///        depends on destination ColorType. For example, it must be at least 4 * width for
    //     ///        the 8888 color type.
    //     /// @returns a TypedArray appropriate for the specified ColorType. Note that 16 bit floats are
    //     ///          not supported in JS, so that colorType corresponds to raw bytes Uint8Array.
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn readPixels(this: &CanvasKitCanvas, srcX: number, srcY: number, imageInfo: ImageInfo, dest?: MallocObj,
    //                bytesPerRow?: number) -> Uint8Array | Float32Array | null;

    ///
    /// Removes changes to the current matrix and clip since Canvas state was
    /// last saved. The state is removed from the stack.
    /// Does nothing if the stack is empty.
    ///
    #[wasm_bindgen(method)]
    pub fn restore(this: &CanvasKitCanvas);

    //     ///
    //     /// Restores state to a previous stack value.
    //     /// @param saveCount
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn restoreToCount(this: &CanvasKitCanvas, saveCount: number);

    //     ///
    //     /// Rotates the current matrix by the number of degrees.
    //     /// @param rot - angle of rotation in degrees.
    //     /// @param rx
    //     /// @param ry
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn rotate(this: &CanvasKitCanvas, rot: AngleInDegrees, rx: number, ry: number);

    ///
    /// Saves the current matrix and clip and returns current height of the stack.
    ///
    #[wasm_bindgen(method)]
    pub fn save(this: &CanvasKitCanvas) -> usize;

    //     ///
    //     /// Saves Matrix and clip, and allocates a SkBitmap for subsequent drawing.
    //     /// Calling restore() discards changes to Matrix and clip, and draws the SkBitmap.
    //     /// It returns the height of the stack.
    //     /// See Canvas.h for more.
    //     /// @param paint
    //     /// @param bounds
    //     /// @param backdrop
    //     /// @param flags
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn saveLayer(this: &CanvasKitCanvas, paint?: Paint, bounds?: InputRect | null, backdrop?: ImageFilter | null,
    //               flags?: SaveLayerFlag) -> number;

    //     ///
    //     /// Scales the current matrix by sx on the x-axis and sy on the y-axis.
    //     /// @param sx
    //     /// @param sy
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn scale(this: &CanvasKitCanvas, sx: number, sy: number);

    //     ///
    //     ///  Skews Matrix by sx on the x-axis and sy on the y-axis. A positive value of sx
    //     ///  skews the drawing right as y-axis values increase; a positive value of sy skews
    //     ///  the drawing down as x-axis values increase.
    //     /// @param sx
    //     /// @param sy
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn skew(this: &CanvasKitCanvas, sx: number, sy: number);

    ///
    /// Translates Matrix by dx along the x-axis and dy along the y-axis.
    /// @param dx
    /// @param dy
    ///
    #[wasm_bindgen(method)]
    pub fn translate(this: &CanvasKitCanvas, dx: f32, dy: f32);

    //     ///
    //     /// Writes the given rectangle of pixels to the provided coordinates. The source pixels
    //     /// will be converted to the canvas's alphaType and colorType if they do not match.
    //     /// @param pixels
    //     /// @param srcWidth
    //     /// @param srcHeight
    //     /// @param destX
    //     /// @param destY
    //     /// @param alphaType - defaults to Unpremul
    //     /// @param colorType - defaults to RGBA_8888
    //     /// @param colorSpace - defaults to SRGB
    //     ///
    // #[wasm_bindgen(method)]
    // pub fn writePixels(this: &CanvasKitCanvas, pixels: Uint8Array | number[], srcWidth: number, srcHeight: number,
    //                 destX: number, destY: number, alphaType?: AlphaType, colorType?: ColorType,
    //                 colorSpace?: ColorSpace) -> boolean;

}

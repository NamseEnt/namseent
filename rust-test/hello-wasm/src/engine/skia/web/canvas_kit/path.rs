use super::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Path")]
    pub type CanvasKitPath;

    #[wasm_bindgen(constructor, js_class="Path", js_namespace = ["globalThis", "CanvasKit"])]
    pub fn new() -> CanvasKitPath;

    // ///
    // /// Appends arc to Path, as the start of new contour. Arc added is part of ellipse
    // /// bounded by oval, from startAngle through sweepAngle. Both startAngle and
    // /// sweepAngle are measured in degrees, where zero degrees is aligned with the
    // /// positive x-axis, and positive sweeps extends arc clockwise.
    // /// Returns the modified path for easier chaining.
    // /// @param oval
    // /// @param startAngle
    // /// @param sweepAngle
    // ///
    // #[wasm_bindgen(method)]
    // pub fn addArc(this: &CanvasKitPath, oval: js_sys::Float32Array, startAngle: AngleInDegrees, sweepAngle: AngleInDegrees) -> CanvasKitPath;

    // ///
    // /// Adds oval to Path, appending kMove_Verb, four kConic_Verb, and kClose_Verb.
    // /// Oval is upright ellipse bounded by Rect oval with radii equal to half oval width
    // /// and half oval height. Oval begins at start and continues clockwise by default.
    // /// Returns the modified path for easier chaining.
    // /// @param oval
    // /// @param isCCW - if the path should be drawn counter-clockwise or not
    // /// @param startIndex - index of initial point of ellipse
    // ///
    // #[wasm_bindgen(method)]
    // pub fn addOval(this: &CanvasKitPath, oval: js_sys::Float32Array, isCCW: Option<bool, startIndex: Option<number) -> CanvasKitPath;

    // ///
    // /// Takes 1, 2, 7, or 10 required args, where the first arg is always the path.
    // /// The last arg is an optional bool and chooses between add or extend mode.
    // /// The options for the remaining args are:
    // ///   - an array of 6 or 9 parameters (perspective is optional)
    // ///   - the 9 parameters of a full matrix or
    // ///     the 6 non-perspective params of a matrix.
    // /// Returns the modified path for easier chaining (or null if params were incorrect).
    // /// @param args
    // ///
    // #[wasm_bindgen(method)]
    // pub fn addPath(this: &CanvasKitPath, (...args: any[]) -> CanvasKitPath | null;

    // ///
    // /// Adds contour created from array of n points, adding (count - 1) line segments.
    // /// Contour added starts at pts[0], then adds a line for every additional point
    // /// in pts array. If close is true, appends kClose_Verb to Path, connecting
    // /// pts[count - 1] and pts[0].
    // /// Returns the modified path for easier chaining.
    // /// @param points
    // /// @param close - if true, will add a line connecting last point to the first point.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn addPoly(this: &CanvasKitPath, points: InputFlattenedPointArray, close: bool) -> CanvasKitPath;

    ///
    /// Adds Rect to Path, appending kMove_Verb, three kLine_Verb, and kClose_Verb,
    /// starting with top-left corner of Rect; followed by top-right, bottom-right,
    /// and bottom-left if isCCW is false; or followed by bottom-left,
    /// bottom-right, and top-right if isCCW is true.
    /// Returns the modified path for easier chaining.
    /// @param rect
    /// @param isCCW
    ///
    #[wasm_bindgen(method)]
    pub fn addRect(
        this: &CanvasKitPath,
        rect: js_sys::Float32Array,
        isCCW: Option<bool>,
    ) -> CanvasKitPath;

    ///
    /// Adds rrect to Path, creating a new closed contour.
    /// Returns the modified path for easier chaining.
    /// @param rrect
    /// @param isCCW
    ///
    #[wasm_bindgen(method)]
    pub fn addRRect(
        this: &CanvasKitPath,
        rrect: js_sys::Float32Array,
        isCCW: Option<bool>,
    ) -> CanvasKitPath;

    // ///
    // /// Adds the given verbs and associated points/weights to the path. The process
    // /// reads the first verb from verbs and then the appropriate number of points from the
    // /// FlattenedPointArray (e.g. 2 points for moveTo, 4 points for quadTo, etc). If the verb is
    // /// a conic, a weight will be read from the WeightList.
    // /// Returns the modified path for easier chaining
    // /// @param verbs - the verbs that create this path, in the order of being drawn.
    // /// @param points - represents n points with 2n floats.
    // /// @param weights - used if any of the verbs are conics, can be omitted otherwise.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn addVerbsPointsWeights(this: &CanvasKitPath, verbs: VerbList, points: InputFlattenedPointArray,
    //                       weights: Option<WeightList) -> CanvasKitPath;

    // ///
    // /// Adds an arc to this path, emulating the Canvas2D behavior.
    // /// Returns the modified path for easier chaining.
    // /// @param x
    // /// @param y
    // /// @param radius
    // /// @param startAngle
    // /// @param endAngle
    // /// @param isCCW
    // ///
    // #[wasm_bindgen(method)]
    // pub fn arc(this: &CanvasKitPath, x: number, y: number, radius: number, startAngle: AngleInRadians, endAngle: AngleInRadians,
    //     isCCW: Option<bool) -> CanvasKitPath;

    // ///
    // /// Appends arc to Path. Arc added is part of ellipse
    // /// bounded by oval, from startAngle through sweepAngle. Both startAngle and
    // /// sweepAngle are measured in degrees, where zero degrees is aligned with the
    // /// positive x-axis, and positive sweeps extends arc clockwise.
    // /// Returns the modified path for easier chaining.
    // /// @param oval
    // /// @param startAngle
    // /// @param endAngle
    // /// @param forceMoveTo
    // ///
    // #[wasm_bindgen(method)]
    // pub fn arcToOval(this: &CanvasKitPath, oval: js_sys::Float32Array, startAngle: AngleInDegrees, endAngle: AngleInDegrees,
    //           forceMoveTo: bool) -> CanvasKitPath;

    // ///
    // /// Appends arc to Path. Arc is implemented by one or more conics weighted to
    // /// describe part of oval with radii (rx, ry) rotated by xAxisRotate degrees. Arc
    // /// curves from last Path Point to (x, y), choosing one of four possible routes:
    // /// clockwise or counterclockwise, and smaller or larger. See SkPath.h for more details.
    // /// Returns the modified path for easier chaining.
    // /// @param rx
    // /// @param ry
    // /// @param xAxisRotate
    // /// @param useSmallArc
    // /// @param isCCW
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn arcToRotated(this: &CanvasKitPath, rx: number, ry: number, xAxisRotate: AngleInDegrees, useSmallArc: bool,
    //              isCCW: bool, x: number, y: number) -> CanvasKitPath;

    // ///
    // /// Appends arc to Path, after appending line if needed. Arc is implemented by conic
    // /// weighted to describe part of circle. Arc is contained by tangent from
    // /// last Path point to (x1, y1), and tangent from (x1, y1) to (x2, y2). Arc
    // /// is part of circle sized to radius, positioned so it touches both tangent lines.
    // /// Returns the modified path for easier chaining.
    // /// @param x1
    // /// @param y1
    // /// @param x2
    // /// @param y2
    // /// @param radius
    // ///
    // #[wasm_bindgen(method)]
    // pub fn arcToTangent(this: &CanvasKitPath, x1: number, y1: number, x2: number, y2: number, radius: number) -> CanvasKitPath;

    // ///
    // /// Appends CLOSE_VERB to Path. A closed contour connects the first and last point
    // /// with a line, forming a continuous loop.
    // /// Returns the modified path for easier chaining.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn close(this: &CanvasKitPath) -> CanvasKitPath;

    // ///
    // /// Returns minimum and maximum axes values of the lines and curves in Path.
    // /// Returns (0, 0, 0, 0) if Path contains no points.
    // /// Returned bounds width and height may be larger or smaller than area affected
    // /// when Path is drawn.
    //  *
    // /// Behaves identically to getBounds() when Path contains
    // /// only lines. If Path contains curves, computed bounds includes
    // /// the maximum extent of the quad, conic, or cubic; is slower than getBounds();
    // /// and unlike getBounds(), does not cache the result.
    // /// @param outputArray - if provided, the bounding box will be copied into this array instead of
    // ///                      allocating a new one.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn computeTightBounds(this: &CanvasKitPath, outputArray: Option<Rect) -> Rect;

    // ///
    // /// Adds conic from last point towards (x1, y1), to (x2, y2), weighted by w.
    // /// If Path is empty, or path is closed, the last point is set to (0, 0)
    // /// before adding conic.
    // /// Returns the modified path for easier chaining.
    // /// @param x1
    // /// @param y1
    // /// @param x2
    // /// @param y2
    // /// @param w
    // ///
    // #[wasm_bindgen(method)]
    // pub fn conicTo(this: &CanvasKitPath, x1: number, y1: number, x2: number, y2: number, w: number) -> CanvasKitPath;

    // ///
    // /// Returns true if the point (x, y) is contained by Path, taking into
    // /// account FillType.
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn contains(this: &CanvasKitPath, x: number, y: number) -> bool;

    ///
    /// Returns a copy of this Path.
    ///
    #[wasm_bindgen(method)]
    pub fn copy(this: &CanvasKitPath) -> CanvasKitPath;

    // ///
    // /// Returns the number of points in this path. Initially zero.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn countPoints(this: &CanvasKitPath) -> number;

    // ///
    // ///  Adds cubic from last point towards (x1, y1), then towards (x2, y2), ending at
    // /// (x3, y3). If Path is empty, or path is closed, the last point is set to
    // /// (0, 0) before adding cubic.
    // /// @param cpx1
    // /// @param cpy1
    // /// @param cpx2
    // /// @param cpy2
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn cubicTo(this: &CanvasKitPath, cpx1: number, cpy1: number, cpx2: number, cpy2: number, x: number, y: number) -> CanvasKitPath;

    // ///
    // /// Changes this path to be the dashed version of itself. This is the same effect as creating
    // /// a DashPathEffect and calling filterPath on this path.
    // /// @param on
    // /// @param off
    // /// @param phase
    // ///
    // #[wasm_bindgen(method)]
    // pub fn dash(this: &CanvasKitPath, on: number, off: number, phase: number) -> bool;

    // ///
    // /// Returns true if other path is equal to this path.
    // /// @param other
    // ///
    // #[wasm_bindgen(method)]
    // pub fn equals(this: &CanvasKitPath, other: Path) -> bool;

    // ///
    // /// Returns minimum and maximum axes values of Point array.
    // /// Returns (0, 0, 0, 0) if Path contains no points. Returned bounds width and height may
    // /// be larger or smaller than area affected when Path is drawn.
    // /// @param outputArray - if provided, the bounding box will be copied into this array instead of
    // ///                      allocating a new one.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn getBounds(this: &CanvasKitPath, outputArray: Option<Rect) -> Rect;

    // ///
    // /// Return the FillType for this path.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn getFillType(this: &CanvasKitPath) -> FillType;

    // ///
    // /// Returns the Point at index in Point array. Valid range for index is
    // /// 0 to countPoints() - 1.
    // /// @param index
    // /// @param outputArray - if provided, the point will be copied into this array instead of
    // ///                      allocating a new one.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn getPoint(this: &CanvasKitPath, index: number, outputArray: Option<Point) -> Point;

    // ///
    // /// Returns true if there are no verbs in the path.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn isEmpty(this: &CanvasKitPath) -> bool;

    // ///
    // /// Returns true if the path is volatile; it will not be altered or discarded
    // /// by the caller after it is drawn. Path by default have volatile set false, allowing
    // /// Surface to attach a cache of data which speeds repeated drawing. If true, Surface
    // /// may not speed repeated drawing.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn isVolatile(this: &CanvasKitPath) -> bool;

    // ///
    // /// Adds line from last point to (x, y). If Path is empty, or last path is closed,
    // /// last point is set to (0, 0) before adding line.
    // /// Returns the modified path for easier chaining.
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn lineTo(this: &CanvasKitPath, x: number, y: number) -> CanvasKitPath;

    // ///
    // /// Returns a new path that covers the same area as the original path, but with the
    // /// Winding FillType. This may re-draw some contours in the path as counter-clockwise
    // /// instead of clockwise to achieve that effect. If such a transformation cannot
    // /// be done, null is returned.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn makeAsWinding(this: &CanvasKitPath) -> CanvasKitPath | null;

    // ///
    // /// Adds beginning of contour at the given point.
    // /// Returns the modified path for easier chaining.
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn moveTo(this: &CanvasKitPath, x: number, y: number) -> CanvasKitPath;

    // ///
    // /// Translates all the points in the path by dx, dy.
    // /// Returns the modified path for easier chaining.
    // /// @param dx
    // /// @param dy
    // ///
    // #[wasm_bindgen(method)]
    // pub fn offset(this: &CanvasKitPath, dx: number, dy: number) -> CanvasKitPath;

    // ///
    // /// Combines this path with the other path using the given PathOp. Returns false if the operation
    // /// fails.
    // /// @param other
    // /// @param op
    // ///
    // #[wasm_bindgen(method)]
    // pub fn op(this: &CanvasKitPath, other: Path, op: PathOp) -> bool;

    // ///
    // /// Adds quad from last point towards (x1, y1), to (x2, y2).
    // /// If Path is empty, or path is closed, last point is set to (0, 0) before adding quad.
    // /// Returns the modified path for easier chaining.
    // /// @param x1
    // /// @param y1
    // /// @param x2
    // /// @param y2
    // ///
    // #[wasm_bindgen(method)]
    // pub fn quadTo(this: &CanvasKitPath, x1: number, y1: number, x2: number, y2: number) -> CanvasKitPath;

    // ///
    // /// Relative version of arcToRotated.
    // /// @param rx
    // /// @param ry
    // /// @param xAxisRotate
    // /// @param useSmallArc
    // /// @param isCCW
    // /// @param dx
    // /// @param dy
    // ///
    // #[wasm_bindgen(method)]
    // pub fn rArcTo(this: &CanvasKitPath, rx: number, ry: number, xAxisRotate: AngleInDegrees, useSmallArc: bool,
    //        isCCW: bool, dx: number, dy: number) -> CanvasKitPath;

    // ///
    // /// Relative version of conicTo.
    // /// @param dx1
    // /// @param dy1
    // /// @param dx2
    // /// @param dy2
    // /// @param w
    // ///
    // #[wasm_bindgen(method)]
    // pub fn rConicTo(this: &CanvasKitPath, dx1: number, dy1: number, dx2: number, dy2: number, w: number) -> CanvasKitPath;

    // ///
    // /// Relative version of cubicTo.
    // /// @param cpx1
    // /// @param cpy1
    // /// @param cpx2
    // /// @param cpy2
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn rCubicTo(this: &CanvasKitPath, cpx1: number, cpy1: number, cpx2: number, cpy2: number, x: number, y: number) -> CanvasKitPath;

    // ///
    // /// Sets Path to its initial state.
    // /// Removes verb array, point array, and weights, and sets FillType to Winding.
    // /// Internal storage associated with Path is released
    // ///
    // #[wasm_bindgen(method)]
    // pub fn reset(this: &CanvasKitPath) -> void;

    // ///
    // /// Sets Path to its initial state.
    // /// Removes verb array, point array, and weights, and sets FillType to Winding.
    // /// Internal storage associated with Path is *not* released.
    // /// Use rewind() instead of reset() if Path storage will be reused and performance
    // /// is critical.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn rewind(this: &CanvasKitPath) -> void;

    // ///
    // /// Relative version of lineTo.
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn rLineTo(this: &CanvasKitPath, x: number, y: number) -> CanvasKitPath;

    // ///
    // /// Relative version of moveTo.
    // /// @param x
    // /// @param y
    // ///
    // #[wasm_bindgen(method)]
    // pub fn rMoveTo(this: &CanvasKitPath, x: number, y: number) -> CanvasKitPath;

    // ///
    // /// Relative version of quadTo.
    // /// @param x1
    // /// @param y1
    // /// @param x2
    // /// @param y2
    // ///
    // #[wasm_bindgen(method)]
    // pub fn rQuadTo(this: &CanvasKitPath, x1: number, y1: number, x2: number, y2: number) -> CanvasKitPath;

    // ///
    // /// Sets FillType, the rule used to fill Path.
    // /// @param fill
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setFillType(this: &CanvasKitPath, fill: FillType) -> void;

    // ///
    // /// Specifies whether Path is volatile; whether it will be altered or discarded
    // /// by the caller after it is drawn. Path by default have volatile set false.
    //  *
    // /// Mark animating or temporary paths as volatile to improve performance.
    // /// Mark unchanging Path non-volatile to improve repeated rendering.
    // /// @param volatile
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setIsVolatile(this: &CanvasKitPath, volatile: bool) -> void;

    // ///
    // /// Set this path to a set of non-overlapping contours that describe the
    // /// same area as the original path.
    // /// The curve order is reduced where possible so that cubics may
    // /// be turned into quadratics, and quadratics maybe turned into lines.
    //  *
    // /// Returns true if operation was able to produce a result.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn simplify(this: &CanvasKitPath) -> bool;

    // ///
    // /// Turns this path into the filled equivalent of the stroked path. Returns null if the operation
    // /// fails (e.g. the path is a hairline).
    // /// @param opts - describe how stroked path should look.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn stroke(this: &CanvasKitPath, opts: Option<StrokeOpts) -> CanvasKitPath | null;

    // ///
    // /// Serializes the contents of this path as a series of commands.
    // /// The first item will be a verb, followed by any number of arguments needed. Then it will
    // /// be followed by another verb, more arguments and so on.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn toCmds(this: &CanvasKitPath) -> Float32Array;

    // ///
    // /// Returns this path as an SVG string.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn toSVGString(this: &CanvasKitPath) -> string;

    // ///
    // /// Takes a 3x3 matrix as either an array or as 9 individual params.
    // /// @param args
    // ///
    // #[wasm_bindgen(method)]
    // pub fn transform(this: &CanvasKitPath, (...args: any[]) -> CanvasKitPath;

    // ///
    // /// Take start and stop "t" values (values between 0...1), and modify this path such that
    // /// it is a subset of the original path.
    // /// The trim values apply to the entire path, so if it contains several contours, all of them
    // /// are including in the calculation.
    // /// Null is returned if either input value is NaN.
    // /// @param startT - a value in the range [0.0, 1.0]. 0.0 is the beginning of the path.
    // /// @param stopT  - a value in the range [0.0, 1.0]. 1.0 is the end of the path.
    // /// @param isComplement
    // ///
    // #[wasm_bindgen(method)]
    // pub fn trim(this: &CanvasKitPath, startT: number, stopT: number, isComplement: bool) -> CanvasKitPath | null;
}

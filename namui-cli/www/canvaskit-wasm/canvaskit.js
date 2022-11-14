var CanvasKitInit = (() => {
    var _scriptDir =
        typeof document !== "undefined" && document.currentScript
            ? document.currentScript.src
            : undefined;
    if (typeof __filename !== "undefined")
        _scriptDir = _scriptDir || __filename;
    return function (CanvasKitInit) {
        CanvasKitInit = CanvasKitInit || {};

        null;
        var w;
        w || (w = typeof CanvasKitInit !== "undefined" ? CanvasKitInit : {});
        var aa, ea;
        w.ready = new Promise(function (a, b) {
            aa = a;
            ea = b;
        });
        (function (a) {
            a.Sd = a.Sd || [];
            a.Sd.push(function () {
                a.MakeSWCanvasSurface = function (b) {
                    var e = b;
                    if (
                        "CANVAS" !== e.tagName &&
                        ((e = document.getElementById(b)), !e)
                    )
                        throw "Canvas with id " + b + " was not found";
                    if ((b = a.MakeSurface(e.width, e.height))) b.Od = e;
                    return b;
                };
                a.MakeCanvasSurface ||
                    (a.MakeCanvasSurface = a.MakeSWCanvasSurface);
                a.MakeSurface = function (b, e) {
                    var f = {
                            width: b,
                            height: e,
                            colorType: a.ColorType.RGBA_8888,
                            alphaType: a.AlphaType.Unpremul,
                            colorSpace: a.ColorSpace.SRGB,
                        },
                        h = b * e * 4,
                        m = a._malloc(h);
                    if ((f = a.Surface._makeRasterDirect(f, m, 4 * b)))
                        (f.Od = null),
                            (f.Ff = b),
                            (f.Bf = e),
                            (f.Df = h),
                            (f.af = m),
                            f.getCanvas().clear(a.TRANSPARENT);
                    return f;
                };
                a.MakeRasterDirectSurface = function (b, e, f) {
                    return a.Surface._makeRasterDirect(b, e.byteOffset, f);
                };
                a.Surface.prototype.flush = function (b) {
                    a.Pd(this.Nd);
                    this._flush();
                    if (this.Od) {
                        var e = new Uint8ClampedArray(
                            a.HEAPU8.buffer,
                            this.af,
                            this.Df,
                        );
                        e = new ImageData(e, this.Ff, this.Bf);
                        b
                            ? this.Od.getContext("2d").putImageData(
                                  e,
                                  0,
                                  0,
                                  b[0],
                                  b[1],
                                  b[2] - b[0],
                                  b[3] - b[1],
                              )
                            : this.Od.getContext("2d").putImageData(e, 0, 0);
                    }
                };
                a.Surface.prototype.dispose = function () {
                    this.af && a._free(this.af);
                    this.delete();
                };
                a.Pd = a.Pd || function () {};
                a.ef =
                    a.ef ||
                    function () {
                        return null;
                    };
            });
        })(w);
        (function (a) {
            a.Sd = a.Sd || [];
            a.Sd.push(function () {
                function b(n, r, v) {
                    return n && n.hasOwnProperty(r) ? n[r] : v;
                }
                function e(n) {
                    var r = fa(ja);
                    ja[r] = n;
                    return r;
                }
                function f(n) {
                    return (
                        n.naturalHeight ||
                        n.videoHeight ||
                        n.displayHeight ||
                        n.height
                    );
                }
                function h(n) {
                    return (
                        n.naturalWidth ||
                        n.videoWidth ||
                        n.displayWidth ||
                        n.width
                    );
                }
                function m(n, r, v, D) {
                    n.bindTexture(n.TEXTURE_2D, r);
                    D ||
                        v.alphaType !== a.AlphaType.Premul ||
                        n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL, !0);
                    return r;
                }
                function u(n, r, v) {
                    v ||
                        r.alphaType !== a.AlphaType.Premul ||
                        n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL, !1);
                    n.bindTexture(n.TEXTURE_2D, null);
                }
                a.GetWebGLContext = function (n, r) {
                    if (!n) throw "null canvas passed into makeWebGLContext";
                    var v = {
                        alpha: b(r, "alpha", 1),
                        depth: b(r, "depth", 1),
                        stencil: b(r, "stencil", 8),
                        antialias: b(r, "antialias", 0),
                        premultipliedAlpha: b(r, "premultipliedAlpha", 1),
                        preserveDrawingBuffer: b(r, "preserveDrawingBuffer", 0),
                        preferLowPowerToHighPerformance: b(
                            r,
                            "preferLowPowerToHighPerformance",
                            0,
                        ),
                        failIfMajorPerformanceCaveat: b(
                            r,
                            "failIfMajorPerformanceCaveat",
                            0,
                        ),
                        enableExtensionsByDefault: b(
                            r,
                            "enableExtensionsByDefault",
                            1,
                        ),
                        explicitSwapControl: b(r, "explicitSwapControl", 0),
                        renderViaOffscreenBackBuffer: b(
                            r,
                            "renderViaOffscreenBackBuffer",
                            0,
                        ),
                    };
                    v.majorVersion =
                        r && r.majorVersion
                            ? r.majorVersion
                            : "undefined" !== typeof WebGL2RenderingContext
                            ? 2
                            : 1;
                    if (v.explicitSwapControl)
                        throw "explicitSwapControl is not supported";
                    n = ka(n, v);
                    if (!n) return 0;
                    ma(n);
                    x.he.getExtension("WEBGL_debug_renderer_info");
                    return n;
                };
                a.deleteContext = function (n) {
                    x === na[n] && (x = null);
                    "object" == typeof JSEvents && JSEvents.vg(na[n].he.canvas);
                    na[n] && na[n].he.canvas && (na[n].he.canvas.zf = void 0);
                    na[n] = null;
                };
                a._setTextureCleanup({
                    deleteTexture: function (n, r) {
                        var v = ja[r];
                        v && na[n].he.deleteTexture(v);
                        ja[r] = null;
                    },
                });
                a.MakeWebGLContext = function (n) {
                    if (!this.Pd(n)) return null;
                    var r = this._MakeGrContext();
                    if (!r) return null;
                    r.Nd = n;
                    return (x.ff = r);
                };
                a.MakeGrContext = a.MakeWebGLContext;
                a.MakeOnScreenGLSurface = function (n, r, v, D) {
                    if (!this.Pd(n.Nd)) return null;
                    r = this._MakeOnScreenGLSurface(n, r, v, D);
                    if (!r) return null;
                    r.Nd = n.Nd;
                    return r;
                };
                a.MakeRenderTarget = function () {
                    var n = arguments[0];
                    if (!this.Pd(n.Nd)) return null;
                    if (3 === arguments.length) {
                        var r = this._MakeRenderTargetWH(
                            n,
                            arguments[1],
                            arguments[2],
                        );
                        if (!r) return null;
                    } else if (2 === arguments.length) {
                        if (
                            ((r = this._MakeRenderTargetII(n, arguments[1])),
                            !r)
                        )
                            return null;
                    } else return null;
                    r.Nd = n.Nd;
                    return r;
                };
                a.MakeWebGLCanvasSurface = function (n, r, v) {
                    r = r || null;
                    var D = n,
                        I =
                            "undefined" !== typeof OffscreenCanvas &&
                            D instanceof OffscreenCanvas;
                    if (
                        !(
                            ("undefined" !== typeof HTMLCanvasElement &&
                                D instanceof HTMLCanvasElement) ||
                            I ||
                            ((D = document.getElementById(n)), D)
                        )
                    )
                        throw "Canvas with id " + n + " was not found";
                    n = this.GetWebGLContext(D, v);
                    if (!n || 0 > n)
                        throw "failed to create webgl context: err " + n;
                    n = this.MakeWebGLContext(n);
                    r = this.MakeOnScreenGLSurface(n, D.width, D.height, r);
                    return r
                        ? r
                        : ((r = D.cloneNode(!0)),
                          D.parentNode.replaceChild(r, D),
                          r.classList.add("ck-replaced"),
                          a.MakeSWCanvasSurface(r));
                };
                a.MakeCanvasSurface = a.MakeWebGLCanvasSurface;
                a.Surface.prototype.makeImageFromTexture = function (n, r) {
                    a.Pd(this.Nd);
                    n = e(n);
                    if ((r = this._makeImageFromTexture(this.Nd, n, r)))
                        r.Me = n;
                    return r;
                };
                a.Surface.prototype.makeImageFromTextureSource = function (
                    n,
                    r,
                    v,
                ) {
                    r ||
                        (r = {
                            height: f(n),
                            width: h(n),
                            colorType: a.ColorType.RGBA_8888,
                            alphaType: v
                                ? a.AlphaType.Premul
                                : a.AlphaType.Unpremul,
                        });
                    r.colorSpace || (r.colorSpace = a.ColorSpace.SRGB);
                    a.Pd(this.Nd);
                    var D = x.he;
                    v = m(D, D.createTexture(), r, v);
                    2 === x.version
                        ? D.texImage2D(
                              D.TEXTURE_2D,
                              0,
                              D.RGBA,
                              r.width,
                              r.height,
                              0,
                              D.RGBA,
                              D.UNSIGNED_BYTE,
                              n,
                          )
                        : D.texImage2D(
                              D.TEXTURE_2D,
                              0,
                              D.RGBA,
                              D.RGBA,
                              D.UNSIGNED_BYTE,
                              n,
                          );
                    u(D, r);
                    this._resetContext();
                    return this.makeImageFromTexture(v, r);
                };
                a.Surface.prototype.updateTextureFromSource = function (
                    n,
                    r,
                    v,
                ) {
                    if (n.Me) {
                        a.Pd(this.Nd);
                        var D = n.getImageInfo(),
                            I = x.he,
                            O = m(I, ja[n.Me], D, v);
                        2 === x.version
                            ? I.texImage2D(
                                  I.TEXTURE_2D,
                                  0,
                                  I.RGBA,
                                  h(r),
                                  f(r),
                                  0,
                                  I.RGBA,
                                  I.UNSIGNED_BYTE,
                                  r,
                              )
                            : I.texImage2D(
                                  I.TEXTURE_2D,
                                  0,
                                  I.RGBA,
                                  I.RGBA,
                                  I.UNSIGNED_BYTE,
                                  r,
                              );
                        u(I, D, v);
                        this._resetContext();
                        ja[n.Me] = null;
                        n.Me = e(O);
                        D.colorSpace = n.getColorSpace();
                        r = this._makeImageFromTexture(this.Nd, n.Me, D);
                        v = n.Md.Vd;
                        I = n.Md.ae;
                        n.Md.Vd = r.Md.Vd;
                        n.Md.ae = r.Md.ae;
                        r.Md.Vd = v;
                        r.Md.ae = I;
                        r.delete();
                        D.colorSpace.delete();
                    }
                };
                a.MakeLazyImageFromTextureSource = function (n, r, v) {
                    r ||
                        (r = {
                            height: f(n),
                            width: h(n),
                            colorType: a.ColorType.RGBA_8888,
                            alphaType: v
                                ? a.AlphaType.Premul
                                : a.AlphaType.Unpremul,
                        });
                    r.colorSpace || (r.colorSpace = a.ColorSpace.SRGB);
                    var D = {
                        makeTexture: function () {
                            var I = x,
                                O = I.he,
                                z = m(O, O.createTexture(), r, v);
                            2 === I.version
                                ? O.texImage2D(
                                      O.TEXTURE_2D,
                                      0,
                                      O.RGBA,
                                      r.width,
                                      r.height,
                                      0,
                                      O.RGBA,
                                      O.UNSIGNED_BYTE,
                                      n,
                                  )
                                : O.texImage2D(
                                      O.TEXTURE_2D,
                                      0,
                                      O.RGBA,
                                      O.RGBA,
                                      O.UNSIGNED_BYTE,
                                      n,
                                  );
                            u(O, r, v);
                            return e(z);
                        },
                        freeSrc: function () {},
                    };
                    "VideoFrame" === n.constructor.name &&
                        (D.freeSrc = function () {
                            n.close();
                        });
                    return a.Image._makeFromGenerator(r, D);
                };
                a.Pd = function (n) {
                    return n ? ma(n) : !1;
                };
                a.ef = function () {
                    return x && x.ff && !x.ff.isDeleted() ? x.ff : null;
                };
            });
        })(w);
        (function (a) {
            function b(d, c, g, l, q) {
                for (var y = 0; y < d.length; y++)
                    c[y * g + ((y * q + l + g) % g)] = d[y];
                return c;
            }
            function e(d) {
                for (var c = d * d, g = Array(c); c--; )
                    g[c] = 0 === c % (d + 1) ? 1 : 0;
                return g;
            }
            function f(d) {
                return d
                    ? d.constructor === Float32Array && 4 === d.length
                    : !1;
            }
            function h(d) {
                return (
                    ((n(255 * d[3]) << 24) |
                        (n(255 * d[0]) << 16) |
                        (n(255 * d[1]) << 8) |
                        (n(255 * d[2]) << 0)) >>>
                    0
                );
            }
            function m(d) {
                if (d && d._ck) return d;
                if (d instanceof Float32Array) {
                    for (
                        var c = Math.floor(d.length / 4),
                            g = new Uint32Array(c),
                            l = 0;
                        l < c;
                        l++
                    )
                        g[l] = h(d.slice(4 * l, 4 * (l + 1)));
                    return g;
                }
                if (d instanceof Uint32Array) return d;
                if (d instanceof Array && d[0] instanceof Float32Array)
                    return d.map(h);
            }
            function u(d) {
                if (void 0 === d) return 1;
                var c = parseFloat(d);
                return d && -1 !== d.indexOf("%") ? c / 100 : c;
            }
            function n(d) {
                return Math.round(Math.max(0, Math.min(d || 0, 255)));
            }
            function r(d, c) {
                (c && c._ck) || a._free(d);
            }
            function v(d, c, g) {
                if (!d || !d.length) return U;
                if (d && d._ck) return d.byteOffset;
                var l = a[c].BYTES_PER_ELEMENT;
                g || (g = a._malloc(d.length * l));
                a[c].set(d, g / l);
                return g;
            }
            function D(d) {
                var c = {
                    de: U,
                    count: d.length,
                    colorType: a.ColorType.RGBA_F32,
                };
                if (d instanceof Float32Array)
                    (c.de = v(d, "HEAPF32")), (c.count = d.length / 4);
                else if (d instanceof Uint32Array)
                    (c.de = v(d, "HEAPU32")),
                        (c.colorType = a.ColorType.RGBA_8888);
                else if (d instanceof Array) {
                    if (d && d.length) {
                        for (
                            var g = a._malloc(16 * d.length),
                                l = 0,
                                q = g / 4,
                                y = 0;
                            y < d.length;
                            y++
                        )
                            for (var B = 0; 4 > B; B++)
                                (a.HEAPF32[q + l] = d[y][B]), l++;
                        d = g;
                    } else d = U;
                    c.de = d;
                } else
                    throw (
                        "Invalid argument to copyFlexibleColorArray, Not a color array " +
                        typeof d
                    );
                return c;
            }
            function I(d) {
                if (!d) return U;
                var c = Ub.toTypedArray();
                if (d.length) {
                    if (6 === d.length || 9 === d.length)
                        return (
                            v(d, "HEAPF32", Oa),
                            6 === d.length && a.HEAPF32.set(yd, 6 + Oa / 4),
                            Oa
                        );
                    if (16 === d.length)
                        return (
                            (c[0] = d[0]),
                            (c[1] = d[1]),
                            (c[2] = d[3]),
                            (c[3] = d[4]),
                            (c[4] = d[5]),
                            (c[5] = d[7]),
                            (c[6] = d[12]),
                            (c[7] = d[13]),
                            (c[8] = d[15]),
                            Oa
                        );
                    throw "invalid matrix size";
                }
                if (void 0 === d.m11) throw "invalid matrix argument";
                c[0] = d.m11;
                c[1] = d.m21;
                c[2] = d.m41;
                c[3] = d.m12;
                c[4] = d.m22;
                c[5] = d.m42;
                c[6] = d.m14;
                c[7] = d.m24;
                c[8] = d.m44;
                return Oa;
            }
            function O(d) {
                if (!d) return U;
                var c = Vb.toTypedArray();
                if (d.length) {
                    if (16 !== d.length && 6 !== d.length && 9 !== d.length)
                        throw "invalid matrix size";
                    if (16 === d.length) return v(d, "HEAPF32", ab);
                    c.fill(0);
                    c[0] = d[0];
                    c[1] = d[1];
                    c[3] = d[2];
                    c[4] = d[3];
                    c[5] = d[4];
                    c[7] = d[5];
                    c[10] = 1;
                    c[12] = d[6];
                    c[13] = d[7];
                    c[15] = d[8];
                    6 === d.length && ((c[12] = 0), (c[13] = 0), (c[15] = 1));
                    return ab;
                }
                if (void 0 === d.m11) throw "invalid matrix argument";
                c[0] = d.m11;
                c[1] = d.m21;
                c[2] = d.m31;
                c[3] = d.m41;
                c[4] = d.m12;
                c[5] = d.m22;
                c[6] = d.m32;
                c[7] = d.m42;
                c[8] = d.m13;
                c[9] = d.m23;
                c[10] = d.m33;
                c[11] = d.m43;
                c[12] = d.m14;
                c[13] = d.m24;
                c[14] = d.m34;
                c[15] = d.m44;
                return ab;
            }
            function z(d, c) {
                return v(d, "HEAPF32", c || Ua);
            }
            function P(d, c, g, l) {
                var q = Wb.toTypedArray();
                q[0] = d;
                q[1] = c;
                q[2] = g;
                q[3] = l;
                return Ua;
            }
            function V(d) {
                for (var c = new Float32Array(4), g = 0; 4 > g; g++)
                    c[g] = a.HEAPF32[d / 4 + g];
                return c;
            }
            function S(d, c) {
                return v(d, "HEAPF32", c || ha);
            }
            function ra(d, c) {
                return v(d, "HEAPF32", c || Xb);
            }
            function ya() {
                for (var d = 0, c = 0; c < arguments.length - 1; c += 2)
                    d += arguments[c] * arguments[c + 1];
                return d;
            }
            function gb(d, c, g) {
                for (var l = Array(d.length), q = 0; q < g; q++)
                    for (var y = 0; y < g; y++) {
                        for (var B = 0, J = 0; J < g; J++)
                            B += d[g * q + J] * c[g * J + y];
                        l[q * g + y] = B;
                    }
                return l;
            }
            function hb(d, c) {
                for (var g = gb(c[0], c[1], d), l = 2; l < c.length; )
                    (g = gb(g, c[l], d)), l++;
                return g;
            }
            a.Color = function (d, c, g, l) {
                void 0 === l && (l = 1);
                return a.Color4f(n(d) / 255, n(c) / 255, n(g) / 255, l);
            };
            a.ColorAsInt = function (d, c, g, l) {
                void 0 === l && (l = 255);
                return (
                    ((n(l) << 24) |
                        (n(d) << 16) |
                        (n(c) << 8) |
                        ((n(g) << 0) & 268435455)) >>>
                    0
                );
            };
            a.Color4f = function (d, c, g, l) {
                void 0 === l && (l = 1);
                return Float32Array.of(d, c, g, l);
            };
            Object.defineProperty(a, "TRANSPARENT", {
                get: function () {
                    return a.Color4f(0, 0, 0, 0);
                },
            });
            Object.defineProperty(a, "BLACK", {
                get: function () {
                    return a.Color4f(0, 0, 0, 1);
                },
            });
            Object.defineProperty(a, "WHITE", {
                get: function () {
                    return a.Color4f(1, 1, 1, 1);
                },
            });
            Object.defineProperty(a, "RED", {
                get: function () {
                    return a.Color4f(1, 0, 0, 1);
                },
            });
            Object.defineProperty(a, "GREEN", {
                get: function () {
                    return a.Color4f(0, 1, 0, 1);
                },
            });
            Object.defineProperty(a, "BLUE", {
                get: function () {
                    return a.Color4f(0, 0, 1, 1);
                },
            });
            Object.defineProperty(a, "YELLOW", {
                get: function () {
                    return a.Color4f(1, 1, 0, 1);
                },
            });
            Object.defineProperty(a, "CYAN", {
                get: function () {
                    return a.Color4f(0, 1, 1, 1);
                },
            });
            Object.defineProperty(a, "MAGENTA", {
                get: function () {
                    return a.Color4f(1, 0, 1, 1);
                },
            });
            a.getColorComponents = function (d) {
                return [
                    Math.floor(255 * d[0]),
                    Math.floor(255 * d[1]),
                    Math.floor(255 * d[2]),
                    d[3],
                ];
            };
            a.parseColorString = function (d, c) {
                d = d.toLowerCase();
                if (d.startsWith("#")) {
                    c = 255;
                    switch (d.length) {
                        case 9:
                            c = parseInt(d.slice(7, 9), 16);
                        case 7:
                            var g = parseInt(d.slice(1, 3), 16);
                            var l = parseInt(d.slice(3, 5), 16);
                            var q = parseInt(d.slice(5, 7), 16);
                            break;
                        case 5:
                            c = 17 * parseInt(d.slice(4, 5), 16);
                        case 4:
                            (g = 17 * parseInt(d.slice(1, 2), 16)),
                                (l = 17 * parseInt(d.slice(2, 3), 16)),
                                (q = 17 * parseInt(d.slice(3, 4), 16));
                    }
                    return a.Color(g, l, q, c / 255);
                }
                return d.startsWith("rgba")
                    ? ((d = d.slice(5, -1)),
                      (d = d.split(",")),
                      a.Color(+d[0], +d[1], +d[2], u(d[3])))
                    : d.startsWith("rgb")
                    ? ((d = d.slice(4, -1)),
                      (d = d.split(",")),
                      a.Color(+d[0], +d[1], +d[2], u(d[3])))
                    : d.startsWith("gray(") ||
                      d.startsWith("hsl") ||
                      !c ||
                      ((d = c[d]), void 0 === d)
                    ? a.BLACK
                    : d;
            };
            a.multiplyByAlpha = function (d, c) {
                d = d.slice();
                d[3] = Math.max(0, Math.min(d[3] * c, 1));
                return d;
            };
            a.Malloc = function (d, c) {
                var g = a._malloc(c * d.BYTES_PER_ELEMENT);
                return {
                    _ck: !0,
                    length: c,
                    byteOffset: g,
                    se: null,
                    subarray: function (l, q) {
                        l = this.toTypedArray().subarray(l, q);
                        l._ck = !0;
                        return l;
                    },
                    toTypedArray: function () {
                        if (this.se && this.se.length) return this.se;
                        this.se = new d(a.HEAPU8.buffer, g, c);
                        this.se._ck = !0;
                        return this.se;
                    },
                };
            };
            a.Free = function (d) {
                a._free(d.byteOffset);
                d.byteOffset = U;
                d.toTypedArray = null;
                d.se = null;
            };
            var Oa = U,
                Ub,
                ab = U,
                Vb,
                Ua = U,
                Wb,
                sa,
                ha = U,
                Cc,
                Pa = U,
                Dc,
                Yb = U,
                Ec,
                Zb = U,
                $b,
                xb = U,
                Fc,
                Xb = U,
                Gc,
                Hc = U,
                yd = Float32Array.of(0, 0, 1),
                U = 0;
            a.onRuntimeInitialized = function () {
                function d(c, g, l, q, y, B, J) {
                    B ||
                        ((B = 4 * q.width),
                        q.colorType === a.ColorType.RGBA_F16
                            ? (B *= 2)
                            : q.colorType === a.ColorType.RGBA_F32 && (B *= 4));
                    var N = B * q.height;
                    var L = y ? y.byteOffset : a._malloc(N);
                    if (
                        J
                            ? !c._readPixels(q, L, B, g, l, J)
                            : !c._readPixels(q, L, B, g, l)
                    )
                        return y || a._free(L), null;
                    if (y) return y.toTypedArray();
                    switch (q.colorType) {
                        case a.ColorType.RGBA_8888:
                        case a.ColorType.RGBA_F16:
                            c = new Uint8Array(a.HEAPU8.buffer, L, N).slice();
                            break;
                        case a.ColorType.RGBA_F32:
                            c = new Float32Array(a.HEAPU8.buffer, L, N).slice();
                            break;
                        default:
                            return null;
                    }
                    a._free(L);
                    return c;
                }
                Wb = a.Malloc(Float32Array, 4);
                Ua = Wb.byteOffset;
                Vb = a.Malloc(Float32Array, 16);
                ab = Vb.byteOffset;
                Ub = a.Malloc(Float32Array, 9);
                Oa = Ub.byteOffset;
                Fc = a.Malloc(Float32Array, 12);
                Xb = Fc.byteOffset;
                Gc = a.Malloc(Float32Array, 12);
                Hc = Gc.byteOffset;
                sa = a.Malloc(Float32Array, 4);
                ha = sa.byteOffset;
                Cc = a.Malloc(Float32Array, 4);
                Pa = Cc.byteOffset;
                Dc = a.Malloc(Float32Array, 3);
                Yb = Dc.byteOffset;
                Ec = a.Malloc(Float32Array, 3);
                Zb = Ec.byteOffset;
                $b = a.Malloc(Int32Array, 4);
                xb = $b.byteOffset;
                a.ColorSpace.SRGB = a.ColorSpace._MakeSRGB();
                a.ColorSpace.DISPLAY_P3 = a.ColorSpace._MakeDisplayP3();
                a.ColorSpace.ADOBE_RGB = a.ColorSpace._MakeAdobeRGB();
                a.GlyphRunFlags = {
                    IsWhiteSpace: a._GlyphRunFlags_isWhiteSpace,
                };
                a.Path.MakeFromCmds = function (c) {
                    var g = v(c, "HEAPF32"),
                        l = a.Path._MakeFromCmds(g, c.length);
                    r(g, c);
                    return l;
                };
                a.Path.MakeFromVerbsPointsWeights = function (c, g, l) {
                    var q = v(c, "HEAPU8"),
                        y = v(g, "HEAPF32"),
                        B = v(l, "HEAPF32"),
                        J = a.Path._MakeFromVerbsPointsWeights(
                            q,
                            c.length,
                            y,
                            g.length,
                            B,
                            (l && l.length) || 0,
                        );
                    r(q, c);
                    r(y, g);
                    r(B, l);
                    return J;
                };
                a.Path.prototype.addArc = function (c, g, l) {
                    c = S(c);
                    this._addArc(c, g, l);
                    return this;
                };
                a.Path.prototype.addCircle = function (c, g, l, q) {
                    this._addCircle(c, g, l, !!q);
                    return this;
                };
                a.Path.prototype.addOval = function (c, g, l) {
                    void 0 === l && (l = 1);
                    c = S(c);
                    this._addOval(c, !!g, l);
                    return this;
                };
                a.Path.prototype.addPath = function () {
                    var c = Array.prototype.slice.call(arguments),
                        g = c[0],
                        l = !1;
                    "boolean" === typeof c[c.length - 1] && (l = c.pop());
                    if (1 === c.length)
                        this._addPath(g, 1, 0, 0, 0, 1, 0, 0, 0, 1, l);
                    else if (2 === c.length)
                        (c = c[1]),
                            this._addPath(
                                g,
                                c[0],
                                c[1],
                                c[2],
                                c[3],
                                c[4],
                                c[5],
                                c[6] || 0,
                                c[7] || 0,
                                c[8] || 1,
                                l,
                            );
                    else if (7 === c.length || 10 === c.length)
                        this._addPath(
                            g,
                            c[1],
                            c[2],
                            c[3],
                            c[4],
                            c[5],
                            c[6],
                            c[7] || 0,
                            c[8] || 0,
                            c[9] || 1,
                            l,
                        );
                    else return null;
                    return this;
                };
                a.Path.prototype.addPoly = function (c, g) {
                    var l = v(c, "HEAPF32");
                    this._addPoly(l, c.length / 2, g);
                    r(l, c);
                    return this;
                };
                a.Path.prototype.addRect = function (c, g) {
                    c = S(c);
                    this._addRect(c, !!g);
                    return this;
                };
                a.Path.prototype.addRRect = function (c, g) {
                    c = ra(c);
                    this._addRRect(c, !!g);
                    return this;
                };
                a.Path.prototype.addVerbsPointsWeights = function (c, g, l) {
                    var q = v(c, "HEAPU8"),
                        y = v(g, "HEAPF32"),
                        B = v(l, "HEAPF32");
                    this._addVerbsPointsWeights(
                        q,
                        c.length,
                        y,
                        g.length,
                        B,
                        (l && l.length) || 0,
                    );
                    r(q, c);
                    r(y, g);
                    r(B, l);
                };
                a.Path.prototype.arc = function (c, g, l, q, y, B) {
                    c = a.LTRBRect(c - l, g - l, c + l, g + l);
                    y = ((y - q) / Math.PI) * 180 - 360 * !!B;
                    B = new a.Path();
                    B.addArc(c, (q / Math.PI) * 180, y);
                    this.addPath(B, !0);
                    B.delete();
                    return this;
                };
                a.Path.prototype.arcToOval = function (c, g, l, q) {
                    c = S(c);
                    this._arcToOval(c, g, l, q);
                    return this;
                };
                a.Path.prototype.arcToRotated = function (c, g, l, q, y, B, J) {
                    this._arcToRotated(c, g, l, !!q, !!y, B, J);
                    return this;
                };
                a.Path.prototype.arcToTangent = function (c, g, l, q, y) {
                    this._arcToTangent(c, g, l, q, y);
                    return this;
                };
                a.Path.prototype.close = function () {
                    this._close();
                    return this;
                };
                a.Path.prototype.conicTo = function (c, g, l, q, y) {
                    this._conicTo(c, g, l, q, y);
                    return this;
                };
                a.Path.prototype.computeTightBounds = function (c) {
                    this._computeTightBounds(ha);
                    var g = sa.toTypedArray();
                    return c ? (c.set(g), c) : g.slice();
                };
                a.Path.prototype.cubicTo = function (c, g, l, q, y, B) {
                    this._cubicTo(c, g, l, q, y, B);
                    return this;
                };
                a.Path.prototype.dash = function (c, g, l) {
                    return this._dash(c, g, l) ? this : null;
                };
                a.Path.prototype.getBounds = function (c) {
                    this._getBounds(ha);
                    var g = sa.toTypedArray();
                    return c ? (c.set(g), c) : g.slice();
                };
                a.Path.prototype.lineTo = function (c, g) {
                    this._lineTo(c, g);
                    return this;
                };
                a.Path.prototype.moveTo = function (c, g) {
                    this._moveTo(c, g);
                    return this;
                };
                a.Path.prototype.offset = function (c, g) {
                    this._transform(1, 0, c, 0, 1, g, 0, 0, 1);
                    return this;
                };
                a.Path.prototype.quadTo = function (c, g, l, q) {
                    this._quadTo(c, g, l, q);
                    return this;
                };
                a.Path.prototype.rArcTo = function (c, g, l, q, y, B, J) {
                    this._rArcTo(c, g, l, q, y, B, J);
                    return this;
                };
                a.Path.prototype.rConicTo = function (c, g, l, q, y) {
                    this._rConicTo(c, g, l, q, y);
                    return this;
                };
                a.Path.prototype.rCubicTo = function (c, g, l, q, y, B) {
                    this._rCubicTo(c, g, l, q, y, B);
                    return this;
                };
                a.Path.prototype.rLineTo = function (c, g) {
                    this._rLineTo(c, g);
                    return this;
                };
                a.Path.prototype.rMoveTo = function (c, g) {
                    this._rMoveTo(c, g);
                    return this;
                };
                a.Path.prototype.rQuadTo = function (c, g, l, q) {
                    this._rQuadTo(c, g, l, q);
                    return this;
                };
                a.Path.prototype.stroke = function (c) {
                    c = c || {};
                    c.width = c.width || 1;
                    c.miter_limit = c.miter_limit || 4;
                    c.cap = c.cap || a.StrokeCap.Butt;
                    c.join = c.join || a.StrokeJoin.Miter;
                    c.precision = c.precision || 1;
                    return this._stroke(c) ? this : null;
                };
                a.Path.prototype.transform = function () {
                    if (1 === arguments.length) {
                        var c = arguments[0];
                        this._transform(
                            c[0],
                            c[1],
                            c[2],
                            c[3],
                            c[4],
                            c[5],
                            c[6] || 0,
                            c[7] || 0,
                            c[8] || 1,
                        );
                    } else if (6 === arguments.length || 9 === arguments.length)
                        (c = arguments),
                            this._transform(
                                c[0],
                                c[1],
                                c[2],
                                c[3],
                                c[4],
                                c[5],
                                c[6] || 0,
                                c[7] || 0,
                                c[8] || 1,
                            );
                    else
                        throw (
                            "transform expected to take 1 or 9 arguments. Got " +
                            arguments.length
                        );
                    return this;
                };
                a.Path.prototype.trim = function (c, g, l) {
                    return this._trim(c, g, !!l) ? this : null;
                };
                a.Image.prototype.makeShaderCubic = function (c, g, l, q, y) {
                    y = I(y);
                    return this._makeShaderCubic(c, g, l, q, y);
                };
                a.Image.prototype.makeShaderOptions = function (c, g, l, q, y) {
                    y = I(y);
                    return this._makeShaderOptions(c, g, l, q, y);
                };
                a.Image.prototype.readPixels = function (c, g, l, q, y) {
                    var B = a.ef();
                    return d(this, c, g, l, q, y, B);
                };
                a.Canvas.prototype.clear = function (c) {
                    a.Pd(this.Nd);
                    c = z(c);
                    this._clear(c);
                };
                a.Canvas.prototype.clipRRect = function (c, g, l) {
                    a.Pd(this.Nd);
                    c = ra(c);
                    this._clipRRect(c, g, l);
                };
                a.Canvas.prototype.clipRect = function (c, g, l) {
                    a.Pd(this.Nd);
                    c = S(c);
                    this._clipRect(c, g, l);
                };
                a.Canvas.prototype.concat = function (c) {
                    a.Pd(this.Nd);
                    c = O(c);
                    this._concat(c);
                };
                a.Canvas.prototype.drawArc = function (c, g, l, q, y) {
                    a.Pd(this.Nd);
                    c = S(c);
                    this._drawArc(c, g, l, q, y);
                };
                a.Canvas.prototype.drawAtlas = function (c, g, l, q, y, B, J) {
                    if (c && q && g && l && g.length === l.length) {
                        a.Pd(this.Nd);
                        y || (y = a.BlendMode.SrcOver);
                        var N = v(g, "HEAPF32"),
                            L = v(l, "HEAPF32"),
                            W = l.length / 4,
                            t = v(m(B), "HEAPU32");
                        if (J && "B" in J && "C" in J)
                            this._drawAtlasCubic(c, L, N, t, W, y, J.B, J.C, q);
                        else {
                            let F = a.FilterMode.Linear,
                                Q = a.MipmapMode.None;
                            J &&
                                ((F = J.filter),
                                "mipmap" in J && (Q = J.mipmap));
                            this._drawAtlasOptions(c, L, N, t, W, y, F, Q, q);
                        }
                        r(N, g);
                        r(L, l);
                        r(t, B);
                    }
                };
                a.Canvas.prototype.drawCircle = function (c, g, l, q) {
                    a.Pd(this.Nd);
                    this._drawCircle(c, g, l, q);
                };
                a.Canvas.prototype.drawColor = function (c, g) {
                    a.Pd(this.Nd);
                    c = z(c);
                    void 0 !== g ? this._drawColor(c, g) : this._drawColor(c);
                };
                a.Canvas.prototype.drawColorInt = function (c, g) {
                    a.Pd(this.Nd);
                    this._drawColorInt(c, g || a.BlendMode.SrcOver);
                };
                a.Canvas.prototype.drawColorComponents = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                ) {
                    a.Pd(this.Nd);
                    c = P(c, g, l, q);
                    void 0 !== y ? this._drawColor(c, y) : this._drawColor(c);
                };
                a.Canvas.prototype.drawDRRect = function (c, g, l) {
                    a.Pd(this.Nd);
                    c = ra(c, Xb);
                    g = ra(g, Hc);
                    this._drawDRRect(c, g, l);
                };
                a.Canvas.prototype.drawImage = function (c, g, l, q) {
                    a.Pd(this.Nd);
                    this._drawImage(c, g, l, q || null);
                };
                a.Canvas.prototype.drawImageCubic = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                ) {
                    a.Pd(this.Nd);
                    this._drawImageCubic(c, g, l, q, y, B || null);
                };
                a.Canvas.prototype.drawImageOptions = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                ) {
                    a.Pd(this.Nd);
                    this._drawImageOptions(c, g, l, q, y, B || null);
                };
                a.Canvas.prototype.drawImageNine = function (c, g, l, q, y) {
                    a.Pd(this.Nd);
                    g = v(g, "HEAP32", xb);
                    l = S(l);
                    this._drawImageNine(c, g, l, q, y || null);
                };
                a.Canvas.prototype.drawImageRect = function (c, g, l, q, y) {
                    a.Pd(this.Nd);
                    S(g, ha);
                    S(l, Pa);
                    this._drawImageRect(c, ha, Pa, q, !!y);
                };
                a.Canvas.prototype.drawImageRectCubic = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                ) {
                    a.Pd(this.Nd);
                    S(g, ha);
                    S(l, Pa);
                    this._drawImageRectCubic(c, ha, Pa, q, y, B || null);
                };
                a.Canvas.prototype.drawImageRectOptions = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                ) {
                    a.Pd(this.Nd);
                    S(g, ha);
                    S(l, Pa);
                    this._drawImageRectOptions(c, ha, Pa, q, y, B || null);
                };
                a.Canvas.prototype.drawLine = function (c, g, l, q, y) {
                    a.Pd(this.Nd);
                    this._drawLine(c, g, l, q, y);
                };
                a.Canvas.prototype.drawOval = function (c, g) {
                    a.Pd(this.Nd);
                    c = S(c);
                    this._drawOval(c, g);
                };
                a.Canvas.prototype.drawPaint = function (c) {
                    a.Pd(this.Nd);
                    this._drawPaint(c);
                };
                a.Canvas.prototype.drawParagraph = function (c, g, l) {
                    a.Pd(this.Nd);
                    this._drawParagraph(c, g, l);
                };
                a.Canvas.prototype.drawPatch = function (c, g, l, q, y) {
                    if (24 > c.length) throw "Need 12 cubic points";
                    if (g && 4 > g.length) throw "Need 4 colors";
                    if (l && 8 > l.length) throw "Need 4 shader coordinates";
                    a.Pd(this.Nd);
                    const B = v(c, "HEAPF32"),
                        J = g ? v(m(g), "HEAPU32") : U,
                        N = l ? v(l, "HEAPF32") : U;
                    q || (q = a.BlendMode.Modulate);
                    this._drawPatch(B, J, N, q, y);
                    r(N, l);
                    r(J, g);
                    r(B, c);
                };
                a.Canvas.prototype.drawPath = function (c, g) {
                    a.Pd(this.Nd);
                    this._drawPath(c, g);
                };
                a.Canvas.prototype.drawPicture = function (c) {
                    a.Pd(this.Nd);
                    this._drawPicture(c);
                };
                a.Canvas.prototype.drawPoints = function (c, g, l) {
                    a.Pd(this.Nd);
                    var q = v(g, "HEAPF32");
                    this._drawPoints(c, q, g.length / 2, l);
                    r(q, g);
                };
                a.Canvas.prototype.drawRRect = function (c, g) {
                    a.Pd(this.Nd);
                    c = ra(c);
                    this._drawRRect(c, g);
                };
                a.Canvas.prototype.drawRect = function (c, g) {
                    a.Pd(this.Nd);
                    c = S(c);
                    this._drawRect(c, g);
                };
                a.Canvas.prototype.drawRect4f = function (c, g, l, q, y) {
                    a.Pd(this.Nd);
                    this._drawRect4f(c, g, l, q, y);
                };
                a.Canvas.prototype.drawShadow = function (c, g, l, q, y, B, J) {
                    a.Pd(this.Nd);
                    var N = v(y, "HEAPF32"),
                        L = v(B, "HEAPF32");
                    g = v(g, "HEAPF32", Yb);
                    l = v(l, "HEAPF32", Zb);
                    this._drawShadow(c, g, l, q, N, L, J);
                    r(N, y);
                    r(L, B);
                };
                a.getShadowLocalBounds = function (c, g, l, q, y, B, J) {
                    c = I(c);
                    l = v(l, "HEAPF32", Yb);
                    q = v(q, "HEAPF32", Zb);
                    if (!this._getShadowLocalBounds(c, g, l, q, y, B, ha))
                        return null;
                    g = sa.toTypedArray();
                    return J ? (J.set(g), J) : g.slice();
                };
                a.Canvas.prototype.drawTextBlob = function (c, g, l, q) {
                    a.Pd(this.Nd);
                    this._drawTextBlob(c, g, l, q);
                };
                a.Canvas.prototype.drawVertices = function (c, g, l) {
                    a.Pd(this.Nd);
                    this._drawVertices(c, g, l);
                };
                a.Canvas.prototype.getDeviceClipBounds = function (c) {
                    this._getDeviceClipBounds(xb);
                    var g = $b.toTypedArray();
                    c ? c.set(g) : (c = g.slice());
                    return c;
                };
                a.Canvas.prototype.getLocalToDevice = function () {
                    this._getLocalToDevice(ab);
                    for (var c = ab, g = Array(16), l = 0; 16 > l; l++)
                        g[l] = a.HEAPF32[c / 4 + l];
                    return g;
                };
                a.Canvas.prototype.getTotalMatrix = function () {
                    this._getTotalMatrix(Oa);
                    for (var c = Array(9), g = 0; 9 > g; g++)
                        c[g] = a.HEAPF32[Oa / 4 + g];
                    return c;
                };
                a.Canvas.prototype.makeSurface = function (c) {
                    c = this._makeSurface(c);
                    c.Nd = this.Nd;
                    return c;
                };
                a.Canvas.prototype.readPixels = function (c, g, l, q, y) {
                    a.Pd(this.Nd);
                    return d(this, c, g, l, q, y);
                };
                a.Canvas.prototype.saveLayer = function (c, g, l, q) {
                    g = S(g);
                    return this._saveLayer(c || null, g, l || null, q || 0);
                };
                a.Canvas.prototype.writePixels = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                    J,
                    N,
                ) {
                    if (c.byteLength % (g * l))
                        throw "pixels length must be a multiple of the srcWidth * srcHeight";
                    a.Pd(this.Nd);
                    var L = c.byteLength / (g * l);
                    B = B || a.AlphaType.Unpremul;
                    J = J || a.ColorType.RGBA_8888;
                    N = N || a.ColorSpace.SRGB;
                    var W = L * g;
                    L = v(c, "HEAPU8");
                    g = this._writePixels(
                        {
                            width: g,
                            height: l,
                            colorType: J,
                            alphaType: B,
                            colorSpace: N,
                        },
                        L,
                        W,
                        q,
                        y,
                    );
                    r(L, c);
                    return g;
                };
                a.ColorFilter.MakeBlend = function (c, g, l) {
                    c = z(c);
                    l = l || a.ColorSpace.SRGB;
                    return a.ColorFilter._MakeBlend(c, g, l);
                };
                a.ColorFilter.MakeMatrix = function (c) {
                    if (!c || 20 !== c.length) throw "invalid color matrix";
                    var g = v(c, "HEAPF32"),
                        l = a.ColorFilter._makeMatrix(g);
                    r(g, c);
                    return l;
                };
                a.ContourMeasure.prototype.getPosTan = function (c, g) {
                    this._getPosTan(c, ha);
                    c = sa.toTypedArray();
                    return g ? (g.set(c), g) : c.slice();
                };
                a.ImageFilter.MakeDropShadow = function (c, g, l, q, y, B) {
                    y = z(y, Ua);
                    return a.ImageFilter._MakeDropShadow(c, g, l, q, y, B);
                };
                a.ImageFilter.MakeDropShadowOnly = function (c, g, l, q, y, B) {
                    y = z(y, Ua);
                    return a.ImageFilter._MakeDropShadowOnly(c, g, l, q, y, B);
                };
                a.ImageFilter.MakeImage = function (c, g, l, q) {
                    l = S(l, ha);
                    q = S(q, Pa);
                    if ("B" in g && "C" in g)
                        return a.ImageFilter._MakeImageCubic(c, g.B, g.C, l, q);
                    const y = g.filter;
                    let B = a.MipmapMode.None;
                    "mipmap" in g && (B = g.mipmap);
                    return a.ImageFilter._MakeImageOptions(c, y, B, l, q);
                };
                a.ImageFilter.MakeMatrixTransform = function (c, g, l) {
                    c = I(c);
                    if ("B" in g && "C" in g)
                        return a.ImageFilter._MakeMatrixTransformCubic(
                            c,
                            g.B,
                            g.C,
                            l,
                        );
                    const q = g.filter;
                    let y = a.MipmapMode.None;
                    "mipmap" in g && (y = g.mipmap);
                    return a.ImageFilter._MakeMatrixTransformOptions(
                        c,
                        q,
                        y,
                        l,
                    );
                };
                a.Paint.prototype.getColor = function () {
                    this._getColor(Ua);
                    return V(Ua);
                };
                a.Paint.prototype.setColor = function (c, g) {
                    g = g || null;
                    c = z(c);
                    this._setColor(c, g);
                };
                a.Paint.prototype.setColorComponents = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                ) {
                    y = y || null;
                    c = P(c, g, l, q);
                    this._setColor(c, y);
                };
                a.Path.prototype.getPoint = function (c, g) {
                    this._getPoint(c, ha);
                    c = sa.toTypedArray();
                    return g
                        ? ((g[0] = c[0]), (g[1] = c[1]), g)
                        : c.slice(0, 2);
                };
                a.Picture.prototype.makeShader = function (c, g, l, q, y) {
                    q = I(q);
                    y = S(y);
                    return this._makeShader(c, g, l, q, y);
                };
                a.PictureRecorder.prototype.beginRecording = function (c) {
                    c = S(c);
                    return this._beginRecording(c);
                };
                a.Surface.prototype.getCanvas = function () {
                    var c = this._getCanvas();
                    c.Nd = this.Nd;
                    return c;
                };
                a.Surface.prototype.makeImageSnapshot = function (c) {
                    a.Pd(this.Nd);
                    c = v(c, "HEAP32", xb);
                    return this._makeImageSnapshot(c);
                };
                a.Surface.prototype.makeSurface = function (c) {
                    a.Pd(this.Nd);
                    c = this._makeSurface(c);
                    c.Nd = this.Nd;
                    return c;
                };
                a.Surface.prototype.Ef = function (c, g) {
                    this.Ie || (this.Ie = this.getCanvas());
                    requestAnimationFrame(
                        function () {
                            a.Pd(this.Nd);
                            c(this.Ie);
                            this.flush(g);
                        }.bind(this),
                    );
                };
                a.Surface.prototype.requestAnimationFrame ||
                    (a.Surface.prototype.requestAnimationFrame =
                        a.Surface.prototype.Ef);
                a.Surface.prototype.Af = function (c, g) {
                    this.Ie || (this.Ie = this.getCanvas());
                    requestAnimationFrame(
                        function () {
                            a.Pd(this.Nd);
                            c(this.Ie);
                            this.flush(g);
                            this.dispose();
                        }.bind(this),
                    );
                };
                a.Surface.prototype.drawOnce ||
                    (a.Surface.prototype.drawOnce = a.Surface.prototype.Af);
                a.PathEffect.MakeDash = function (c, g) {
                    g || (g = 0);
                    if (!c.length || 1 === c.length % 2)
                        throw "Intervals array must have even length";
                    var l = v(c, "HEAPF32");
                    g = a.PathEffect._MakeDash(l, c.length, g);
                    r(l, c);
                    return g;
                };
                a.PathEffect.MakeLine2D = function (c, g) {
                    g = I(g);
                    return a.PathEffect._MakeLine2D(c, g);
                };
                a.PathEffect.MakePath2D = function (c, g) {
                    c = I(c);
                    return a.PathEffect._MakePath2D(c, g);
                };
                a.Shader.MakeColor = function (c, g) {
                    g = g || null;
                    c = z(c);
                    return a.Shader._MakeColor(c, g);
                };
                a.Shader.Blend = a.Shader.MakeBlend;
                a.Shader.Color = a.Shader.MakeColor;
                a.Shader.MakeLinearGradient = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                    J,
                    N,
                ) {
                    N = N || null;
                    var L = D(l),
                        W = v(q, "HEAPF32");
                    J = J || 0;
                    B = I(B);
                    var t = sa.toTypedArray();
                    t.set(c);
                    t.set(g, 2);
                    c = a.Shader._MakeLinearGradient(
                        ha,
                        L.de,
                        L.colorType,
                        W,
                        L.count,
                        y,
                        J,
                        B,
                        N,
                    );
                    r(L.de, l);
                    q && r(W, q);
                    return c;
                };
                a.Shader.MakeRadialGradient = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                    J,
                    N,
                ) {
                    N = N || null;
                    var L = D(l),
                        W = v(q, "HEAPF32");
                    J = J || 0;
                    B = I(B);
                    c = a.Shader._MakeRadialGradient(
                        c[0],
                        c[1],
                        g,
                        L.de,
                        L.colorType,
                        W,
                        L.count,
                        y,
                        J,
                        B,
                        N,
                    );
                    r(L.de, l);
                    q && r(W, q);
                    return c;
                };
                a.Shader.MakeSweepGradient = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                    J,
                    N,
                    L,
                    W,
                ) {
                    W = W || null;
                    var t = D(l),
                        F = v(q, "HEAPF32");
                    J = J || 0;
                    N = N || 0;
                    L = L || 360;
                    B = I(B);
                    c = a.Shader._MakeSweepGradient(
                        c,
                        g,
                        t.de,
                        t.colorType,
                        F,
                        t.count,
                        y,
                        N,
                        L,
                        J,
                        B,
                        W,
                    );
                    r(t.de, l);
                    q && r(F, q);
                    return c;
                };
                a.Shader.MakeTwoPointConicalGradient = function (
                    c,
                    g,
                    l,
                    q,
                    y,
                    B,
                    J,
                    N,
                    L,
                    W,
                ) {
                    W = W || null;
                    var t = D(y),
                        F = v(B, "HEAPF32");
                    L = L || 0;
                    N = I(N);
                    var Q = sa.toTypedArray();
                    Q.set(c);
                    Q.set(l, 2);
                    c = a.Shader._MakeTwoPointConicalGradient(
                        ha,
                        g,
                        q,
                        t.de,
                        t.colorType,
                        F,
                        t.count,
                        J,
                        L,
                        N,
                        W,
                    );
                    r(t.de, y);
                    B && r(F, B);
                    return c;
                };
                a.Vertices.prototype.bounds = function (c) {
                    this._bounds(ha);
                    var g = sa.toTypedArray();
                    return c ? (c.set(g), c) : g.slice();
                };
                a.Sd &&
                    a.Sd.forEach(function (c) {
                        c();
                    });
            };
            a.computeTonalColors = function (d) {
                var c = v(d.ambient, "HEAPF32"),
                    g = v(d.spot, "HEAPF32");
                this._computeTonalColors(c, g);
                var l = { ambient: V(c), spot: V(g) };
                r(c, d.ambient);
                r(g, d.spot);
                return l;
            };
            a.LTRBRect = function (d, c, g, l) {
                return Float32Array.of(d, c, g, l);
            };
            a.XYWHRect = function (d, c, g, l) {
                return Float32Array.of(d, c, d + g, c + l);
            };
            a.LTRBiRect = function (d, c, g, l) {
                return Int32Array.of(d, c, g, l);
            };
            a.XYWHiRect = function (d, c, g, l) {
                return Int32Array.of(d, c, d + g, c + l);
            };
            a.RRectXY = function (d, c, g) {
                return Float32Array.of(
                    d[0],
                    d[1],
                    d[2],
                    d[3],
                    c,
                    g,
                    c,
                    g,
                    c,
                    g,
                    c,
                    g,
                );
            };
            a.MakeAnimatedImageFromEncoded = function (d) {
                d = new Uint8Array(d);
                var c = a._malloc(d.byteLength);
                a.HEAPU8.set(d, c);
                return (d = a._decodeAnimatedImage(c, d.byteLength)) ? d : null;
            };
            a.MakeImageFromEncoded = function (d) {
                d = new Uint8Array(d);
                var c = a._malloc(d.byteLength);
                a.HEAPU8.set(d, c);
                return (d = a._decodeImage(c, d.byteLength)) ? d : null;
            };
            var ib = null;
            a.MakeImageFromCanvasImageSource = function (d) {
                var c = d.width,
                    g = d.height;
                ib || (ib = document.createElement("canvas"));
                ib.width = c;
                ib.height = g;
                var l = ib.getContext("2d", { xg: !0 });
                l.drawImage(d, 0, 0);
                d = l.getImageData(0, 0, c, g);
                return a.MakeImage(
                    {
                        width: c,
                        height: g,
                        alphaType: a.AlphaType.Unpremul,
                        colorType: a.ColorType.RGBA_8888,
                        colorSpace: a.ColorSpace.SRGB,
                    },
                    d.data,
                    4 * c,
                );
            };
            a.MakeImage = function (d, c, g) {
                var l = a._malloc(c.length);
                a.HEAPU8.set(c, l);
                return a._MakeImage(d, l, c.length, g);
            };
            a.MakeVertices = function (d, c, g, l, q, y) {
                var B = (q && q.length) || 0,
                    J = 0;
                g && g.length && (J |= 1);
                l && l.length && (J |= 2);
                void 0 === y || y || (J |= 4);
                d = new a._VerticesBuilder(d, c.length / 2, B, J);
                v(c, "HEAPF32", d.positions());
                d.texCoords() && v(g, "HEAPF32", d.texCoords());
                d.colors() && v(m(l), "HEAPU32", d.colors());
                d.indices() && v(q, "HEAPU16", d.indices());
                return d.detach();
            };
            a.Matrix = {};
            a.Matrix.identity = function () {
                return e(3);
            };
            a.Matrix.invert = function (d) {
                var c =
                    d[0] * d[4] * d[8] +
                    d[1] * d[5] * d[6] +
                    d[2] * d[3] * d[7] -
                    d[2] * d[4] * d[6] -
                    d[1] * d[3] * d[8] -
                    d[0] * d[5] * d[7];
                return c
                    ? [
                          (d[4] * d[8] - d[5] * d[7]) / c,
                          (d[2] * d[7] - d[1] * d[8]) / c,
                          (d[1] * d[5] - d[2] * d[4]) / c,
                          (d[5] * d[6] - d[3] * d[8]) / c,
                          (d[0] * d[8] - d[2] * d[6]) / c,
                          (d[2] * d[3] - d[0] * d[5]) / c,
                          (d[3] * d[7] - d[4] * d[6]) / c,
                          (d[1] * d[6] - d[0] * d[7]) / c,
                          (d[0] * d[4] - d[1] * d[3]) / c,
                      ]
                    : null;
            };
            a.Matrix.mapPoints = function (d, c) {
                for (var g = 0; g < c.length; g += 2) {
                    var l = c[g],
                        q = c[g + 1],
                        y = d[6] * l + d[7] * q + d[8],
                        B = d[3] * l + d[4] * q + d[5];
                    c[g] = (d[0] * l + d[1] * q + d[2]) / y;
                    c[g + 1] = B / y;
                }
                return c;
            };
            a.Matrix.multiply = function () {
                return hb(3, arguments);
            };
            a.Matrix.rotated = function (d, c, g) {
                c = c || 0;
                g = g || 0;
                var l = Math.sin(d);
                d = Math.cos(d);
                return [
                    d,
                    -l,
                    ya(l, g, 1 - d, c),
                    l,
                    d,
                    ya(-l, c, 1 - d, g),
                    0,
                    0,
                    1,
                ];
            };
            a.Matrix.scaled = function (d, c, g, l) {
                g = g || 0;
                l = l || 0;
                var q = b([d, c], e(3), 3, 0, 1);
                return b([g - d * g, l - c * l], q, 3, 2, 0);
            };
            a.Matrix.skewed = function (d, c, g, l) {
                g = g || 0;
                l = l || 0;
                var q = b([d, c], e(3), 3, 1, -1);
                return b([-d * g, -c * l], q, 3, 2, 0);
            };
            a.Matrix.translated = function (d, c) {
                return b(arguments, e(3), 3, 2, 0);
            };
            a.Vector = {};
            a.Vector.dot = function (d, c) {
                return d
                    .map(function (g, l) {
                        return g * c[l];
                    })
                    .reduce(function (g, l) {
                        return g + l;
                    });
            };
            a.Vector.lengthSquared = function (d) {
                return a.Vector.dot(d, d);
            };
            a.Vector.length = function (d) {
                return Math.sqrt(a.Vector.lengthSquared(d));
            };
            a.Vector.mulScalar = function (d, c) {
                return d.map(function (g) {
                    return g * c;
                });
            };
            a.Vector.add = function (d, c) {
                return d.map(function (g, l) {
                    return g + c[l];
                });
            };
            a.Vector.sub = function (d, c) {
                return d.map(function (g, l) {
                    return g - c[l];
                });
            };
            a.Vector.dist = function (d, c) {
                return a.Vector.length(a.Vector.sub(d, c));
            };
            a.Vector.normalize = function (d) {
                return a.Vector.mulScalar(d, 1 / a.Vector.length(d));
            };
            a.Vector.cross = function (d, c) {
                return [
                    d[1] * c[2] - d[2] * c[1],
                    d[2] * c[0] - d[0] * c[2],
                    d[0] * c[1] - d[1] * c[0],
                ];
            };
            a.M44 = {};
            a.M44.identity = function () {
                return e(4);
            };
            a.M44.translated = function (d) {
                return b(d, e(4), 4, 3, 0);
            };
            a.M44.scaled = function (d) {
                return b(d, e(4), 4, 0, 1);
            };
            a.M44.rotated = function (d, c) {
                return a.M44.rotatedUnitSinCos(
                    a.Vector.normalize(d),
                    Math.sin(c),
                    Math.cos(c),
                );
            };
            a.M44.rotatedUnitSinCos = function (d, c, g) {
                var l = d[0],
                    q = d[1];
                d = d[2];
                var y = 1 - g;
                return [
                    y * l * l + g,
                    y * l * q - c * d,
                    y * l * d + c * q,
                    0,
                    y * l * q + c * d,
                    y * q * q + g,
                    y * q * d - c * l,
                    0,
                    y * l * d - c * q,
                    y * q * d + c * l,
                    y * d * d + g,
                    0,
                    0,
                    0,
                    0,
                    1,
                ];
            };
            a.M44.lookat = function (d, c, g) {
                c = a.Vector.normalize(a.Vector.sub(c, d));
                g = a.Vector.normalize(g);
                g = a.Vector.normalize(a.Vector.cross(c, g));
                var l = a.M44.identity();
                b(g, l, 4, 0, 0);
                b(a.Vector.cross(g, c), l, 4, 1, 0);
                b(a.Vector.mulScalar(c, -1), l, 4, 2, 0);
                b(d, l, 4, 3, 0);
                d = a.M44.invert(l);
                return null === d ? a.M44.identity() : d;
            };
            a.M44.perspective = function (d, c, g) {
                var l = 1 / (c - d);
                g /= 2;
                g = Math.cos(g) / Math.sin(g);
                return [
                    g,
                    0,
                    0,
                    0,
                    0,
                    g,
                    0,
                    0,
                    0,
                    0,
                    (c + d) * l,
                    2 * c * d * l,
                    0,
                    0,
                    -1,
                    1,
                ];
            };
            a.M44.rc = function (d, c, g) {
                return d[4 * c + g];
            };
            a.M44.multiply = function () {
                return hb(4, arguments);
            };
            a.M44.invert = function (d) {
                var c = d[0],
                    g = d[4],
                    l = d[8],
                    q = d[12],
                    y = d[1],
                    B = d[5],
                    J = d[9],
                    N = d[13],
                    L = d[2],
                    W = d[6],
                    t = d[10],
                    F = d[14],
                    Q = d[3],
                    Y = d[7],
                    ia = d[11];
                d = d[15];
                var la = c * B - g * y,
                    qa = c * J - l * y,
                    ta = c * N - q * y,
                    ba = g * J - l * B,
                    H = g * N - q * B,
                    k = l * N - q * J,
                    p = L * Y - W * Q,
                    A = L * ia - t * Q,
                    C = L * d - F * Q,
                    E = W * ia - t * Y,
                    G = W * d - F * Y,
                    M = t * d - F * ia,
                    ca = la * M - qa * G + ta * E + ba * C - H * A + k * p,
                    da = 1 / ca;
                if (0 === ca || Infinity === da) return null;
                la *= da;
                qa *= da;
                ta *= da;
                ba *= da;
                H *= da;
                k *= da;
                p *= da;
                A *= da;
                C *= da;
                E *= da;
                G *= da;
                M *= da;
                c = [
                    B * M - J * G + N * E,
                    J * C - y * M - N * A,
                    y * G - B * C + N * p,
                    B * A - y * E - J * p,
                    l * G - g * M - q * E,
                    c * M - l * C + q * A,
                    g * C - c * G - q * p,
                    c * E - g * A + l * p,
                    Y * k - ia * H + d * ba,
                    ia * ta - Q * k - d * qa,
                    Q * H - Y * ta + d * la,
                    Y * qa - Q * ba - ia * la,
                    t * H - W * k - F * ba,
                    L * k - t * ta + F * qa,
                    W * ta - L * H - F * la,
                    L * ba - W * qa + t * la,
                ];
                return c.every(function (Ia) {
                    return !isNaN(Ia) && Infinity !== Ia && -Infinity !== Ia;
                })
                    ? c
                    : null;
            };
            a.M44.transpose = function (d) {
                return [
                    d[0],
                    d[4],
                    d[8],
                    d[12],
                    d[1],
                    d[5],
                    d[9],
                    d[13],
                    d[2],
                    d[6],
                    d[10],
                    d[14],
                    d[3],
                    d[7],
                    d[11],
                    d[15],
                ];
            };
            a.M44.mustInvert = function (d) {
                d = a.M44.invert(d);
                if (null === d) throw "Matrix not invertible";
                return d;
            };
            a.M44.setupCamera = function (d, c, g) {
                var l = a.M44.lookat(g.eye, g.coa, g.up);
                g = a.M44.perspective(g.near, g.far, g.angle);
                c = [(d[2] - d[0]) / 2, (d[3] - d[1]) / 2, c];
                d = a.M44.multiply(
                    a.M44.translated([(d[0] + d[2]) / 2, (d[1] + d[3]) / 2, 0]),
                    a.M44.scaled(c),
                );
                return a.M44.multiply(d, g, l, a.M44.mustInvert(d));
            };
            a.ColorMatrix = {};
            a.ColorMatrix.identity = function () {
                var d = new Float32Array(20);
                d[0] = 1;
                d[6] = 1;
                d[12] = 1;
                d[18] = 1;
                return d;
            };
            a.ColorMatrix.scaled = function (d, c, g, l) {
                var q = new Float32Array(20);
                q[0] = d;
                q[6] = c;
                q[12] = g;
                q[18] = l;
                return q;
            };
            var zd = [
                [6, 7, 11, 12],
                [0, 10, 2, 12],
                [0, 1, 5, 6],
            ];
            a.ColorMatrix.rotated = function (d, c, g) {
                var l = a.ColorMatrix.identity();
                d = zd[d];
                l[d[0]] = g;
                l[d[1]] = c;
                l[d[2]] = -c;
                l[d[3]] = g;
                return l;
            };
            a.ColorMatrix.postTranslate = function (d, c, g, l, q) {
                d[4] += c;
                d[9] += g;
                d[14] += l;
                d[19] += q;
                return d;
            };
            a.ColorMatrix.concat = function (d, c) {
                for (
                    var g = new Float32Array(20), l = 0, q = 0;
                    20 > q;
                    q += 5
                ) {
                    for (var y = 0; 4 > y; y++)
                        g[l++] =
                            d[q] * c[y] +
                            d[q + 1] * c[y + 5] +
                            d[q + 2] * c[y + 10] +
                            d[q + 3] * c[y + 15];
                    g[l++] =
                        d[q] * c[4] +
                        d[q + 1] * c[9] +
                        d[q + 2] * c[14] +
                        d[q + 3] * c[19] +
                        d[q + 4];
                }
                return g;
            };
            (function (d) {
                d.Sd = d.Sd || [];
                d.Sd.push(function () {
                    function c(t) {
                        if (!t || !t.length) return [];
                        for (var F = [], Q = 0; Q < t.length; Q += 5) {
                            var Y = d.LTRBRect(
                                t[Q],
                                t[Q + 1],
                                t[Q + 2],
                                t[Q + 3],
                            );
                            Y.direction =
                                0 === t[Q + 4]
                                    ? d.TextDirection.RTL
                                    : d.TextDirection.LTR;
                            F.push(Y);
                        }
                        d._free(t.byteOffset);
                        return F;
                    }
                    function g(t) {
                        t = t || {};
                        void 0 === t.weight && (t.weight = d.FontWeight.Normal);
                        t.width = t.width || d.FontWidth.Normal;
                        t.slant = t.slant || d.FontSlant.Upright;
                        return t;
                    }
                    function l(t) {
                        if (!t || !t.length) return U;
                        for (var F = [], Q = 0; Q < t.length; Q++) {
                            var Y = q(t[Q]);
                            F.push(Y);
                        }
                        return v(F, "HEAPU32");
                    }
                    function q(t) {
                        if (J[t]) return J[t];
                        var F = oa(t) + 1,
                            Q = d._malloc(F);
                        pa(t, K, Q, F);
                        return (J[t] = Q);
                    }
                    function y(t) {
                        t._colorPtr = z(t.color);
                        t._foregroundColorPtr = U;
                        t._backgroundColorPtr = U;
                        t._decorationColorPtr = U;
                        t.foregroundColor &&
                            (t._foregroundColorPtr = z(t.foregroundColor, N));
                        t.backgroundColor &&
                            (t._backgroundColorPtr = z(t.backgroundColor, L));
                        t.decorationColor &&
                            (t._decorationColorPtr = z(t.decorationColor, W));
                        Array.isArray(t.fontFamilies) && t.fontFamilies.length
                            ? ((t._fontFamiliesPtr = l(t.fontFamilies)),
                              (t._fontFamiliesLen = t.fontFamilies.length))
                            : ((t._fontFamiliesPtr = U),
                              (t._fontFamiliesLen = 0));
                        if (t.locale) {
                            var F = t.locale;
                            t._localePtr = q(F);
                            t._localeLen = oa(F) + 1;
                        } else (t._localePtr = U), (t._localeLen = 0);
                        if (Array.isArray(t.shadows) && t.shadows.length) {
                            F = t.shadows;
                            var Q = F.map(function (ba) {
                                    return ba.color || d.BLACK;
                                }),
                                Y = F.map(function (ba) {
                                    return ba.blurRadius || 0;
                                });
                            t._shadowLen = F.length;
                            for (
                                var ia = d._malloc(8 * F.length),
                                    la = ia / 4,
                                    qa = 0;
                                qa < F.length;
                                qa++
                            ) {
                                var ta = F[qa].offset || [0, 0];
                                d.HEAPF32[la] = ta[0];
                                d.HEAPF32[la + 1] = ta[1];
                                la += 2;
                            }
                            t._shadowColorsPtr = D(Q).de;
                            t._shadowOffsetsPtr = ia;
                            t._shadowBlurRadiiPtr = v(Y, "HEAPF32");
                        } else (t._shadowLen = 0), (t._shadowColorsPtr = U), (t._shadowOffsetsPtr = U), (t._shadowBlurRadiiPtr = U);
                        Array.isArray(t.fontFeatures) && t.fontFeatures.length
                            ? ((F = t.fontFeatures),
                              (Q = F.map(function (ba) {
                                  return ba.name;
                              })),
                              (Y = F.map(function (ba) {
                                  return ba.value;
                              })),
                              (t._fontFeatureLen = F.length),
                              (t._fontFeatureNamesPtr = l(Q)),
                              (t._fontFeatureValuesPtr = v(Y, "HEAPU32")))
                            : ((t._fontFeatureLen = 0),
                              (t._fontFeatureNamesPtr = U),
                              (t._fontFeatureValuesPtr = U));
                        Array.isArray(t.fontVariations) &&
                        t.fontVariations.length
                            ? ((F = t.fontVariations),
                              (Q = F.map(function (ba) {
                                  return ba.axis;
                              })),
                              (Y = F.map(function (ba) {
                                  return ba.value;
                              })),
                              (t._fontVariationLen = F.length),
                              (t._fontVariationAxesPtr = l(Q)),
                              (t._fontVariationValuesPtr = v(Y, "HEAPF32")))
                            : ((t._fontVariationLen = 0),
                              (t._fontVariationAxesPtr = U),
                              (t._fontVariationValuesPtr = U));
                    }
                    function B(t) {
                        d._free(t._fontFamiliesPtr);
                        d._free(t._shadowColorsPtr);
                        d._free(t._shadowOffsetsPtr);
                        d._free(t._shadowBlurRadiiPtr);
                        d._free(t._fontFeatureNamesPtr);
                        d._free(t._fontFeatureValuesPtr);
                    }
                    d.Paragraph.prototype.getRectsForRange = function (
                        t,
                        F,
                        Q,
                        Y,
                    ) {
                        t = this._getRectsForRange(t, F, Q, Y);
                        return c(t);
                    };
                    d.Paragraph.prototype.getRectsForPlaceholders =
                        function () {
                            var t = this._getRectsForPlaceholders();
                            return c(t);
                        };
                    d.TypefaceFontProvider.prototype.registerFont = function (
                        t,
                        F,
                    ) {
                        t = d.Typeface.MakeFreeTypeFaceFromData(t);
                        if (!t) return null;
                        F = q(F);
                        this._registerFont(t, F);
                    };
                    d.ParagraphStyle = function (t) {
                        t.disableHinting = t.disableHinting || !1;
                        if (t.ellipsis) {
                            var F = t.ellipsis;
                            t._ellipsisPtr = q(F);
                            t._ellipsisLen = oa(F) + 1;
                        } else (t._ellipsisPtr = U), (t._ellipsisLen = 0);
                        t.heightMultiplier = t.heightMultiplier || 0;
                        t.maxLines = t.maxLines || 0;
                        t.replaceTabCharacters = t.replaceTabCharacters || !1;
                        F = (F = t.strutStyle) || {};
                        F.strutEnabled = F.strutEnabled || !1;
                        F.strutEnabled &&
                        Array.isArray(F.fontFamilies) &&
                        F.fontFamilies.length
                            ? ((F._fontFamiliesPtr = l(F.fontFamilies)),
                              (F._fontFamiliesLen = F.fontFamilies.length))
                            : ((F._fontFamiliesPtr = U),
                              (F._fontFamiliesLen = 0));
                        F.fontStyle = g(F.fontStyle);
                        F.fontSize = F.fontSize || 0;
                        F.heightMultiplier = F.heightMultiplier || 0;
                        F.halfLeading = F.halfLeading || !1;
                        F.leading = F.leading || 0;
                        F.forceStrutHeight = F.forceStrutHeight || !1;
                        t.strutStyle = F;
                        t.textAlign = t.textAlign || d.TextAlign.Start;
                        t.textDirection =
                            t.textDirection || d.TextDirection.LTR;
                        t.textHeightBehavior =
                            t.textHeightBehavior || d.TextHeightBehavior.All;
                        t.textStyle = d.TextStyle(t.textStyle);
                        return t;
                    };
                    d.TextStyle = function (t) {
                        t.color || (t.color = d.BLACK);
                        t.decoration = t.decoration || 0;
                        t.decorationThickness = t.decorationThickness || 0;
                        t.decorationStyle =
                            t.decorationStyle || d.DecorationStyle.Solid;
                        t.textBaseline =
                            t.textBaseline || d.TextBaseline.Alphabetic;
                        t.fontSize = t.fontSize || 0;
                        t.letterSpacing = t.letterSpacing || 0;
                        t.wordSpacing = t.wordSpacing || 0;
                        t.heightMultiplier = t.heightMultiplier || 0;
                        t.halfLeading = t.halfLeading || !1;
                        t.fontStyle = g(t.fontStyle);
                        return t;
                    };
                    var J = {},
                        N = d._malloc(16),
                        L = d._malloc(16),
                        W = d._malloc(16);
                    d.ParagraphBuilder.Make = function (t, F) {
                        y(t.textStyle);
                        F = d.ParagraphBuilder._Make(t, F);
                        B(t.textStyle);
                        return F;
                    };
                    d.ParagraphBuilder.MakeFromFontProvider = function (t, F) {
                        y(t.textStyle);
                        F = d.ParagraphBuilder._MakeFromFontProvider(t, F);
                        B(t.textStyle);
                        return F;
                    };
                    d.ParagraphBuilder.ShapeText = function (t, F, Q) {
                        let Y = 0;
                        for (const ia of F) Y += ia.length;
                        if (Y !== t.length)
                            throw "Accumulated block lengths must equal text.length";
                        return d.ParagraphBuilder._ShapeText(t, F, Q);
                    };
                    d.ParagraphBuilder.prototype.pushStyle = function (t) {
                        y(t);
                        this._pushStyle(t);
                        B(t);
                    };
                    d.ParagraphBuilder.prototype.pushPaintStyle = function (
                        t,
                        F,
                        Q,
                    ) {
                        y(t);
                        this._pushPaintStyle(t, F, Q);
                        B(t);
                    };
                    d.ParagraphBuilder.prototype.addPlaceholder = function (
                        t,
                        F,
                        Q,
                        Y,
                        ia,
                    ) {
                        Q = Q || d.PlaceholderAlignment.Baseline;
                        Y = Y || d.TextBaseline.Alphabetic;
                        this._addPlaceholder(t || 0, F || 0, Q, Y, ia || 0);
                    };
                    d.ParagraphBuilder.prototype.buildWithClientInfo =
                        function (t, F, Q, Y, ia) {
                            var la = v(t, "HEAPU32"),
                                qa = v(F, "HEAPU32"),
                                ta = v(Q, "HEAPU32"),
                                ba = v(Y, "HEAPU32"),
                                H = v(ia, "HEAPU32"),
                                k = this._buildWithClientInfo(
                                    la,
                                    (t && t.length) || 0,
                                    qa,
                                    (F && F.length) || 0,
                                    ta,
                                    (Q && Q.length) || 0,
                                    ba,
                                    (Y && Y.length) || 0,
                                    H,
                                    (ia && ia.length) || 0,
                                );
                            r(la, t);
                            r(qa, F);
                            r(ta, Q);
                            r(ba, Y);
                            r(H, ia);
                            return k;
                        };
                });
            })(w);
            a.MakeManagedAnimation = function (d, c, g, l, q) {
                if (!a._MakeManagedAnimation)
                    throw "Not compiled with MakeManagedAnimation";
                g || (g = "");
                if (!c) return a._MakeManagedAnimation(d, 0, U, U, U, g, l, q);
                for (
                    var y = [], B = [], J = [], N = Object.keys(c || {}), L = 0;
                    L < N.length;
                    L++
                ) {
                    var W = N[L],
                        t = new Uint8Array(c[W]),
                        F = a._malloc(t.byteLength);
                    a.HEAPU8.set(t, F);
                    B.push(F);
                    J.push(t.byteLength);
                    t = oa(W) + 1;
                    F = a._malloc(t);
                    pa(W, K, F, t);
                    y.push(F);
                }
                c = v(y, "HEAPU32");
                B = v(B, "HEAPU32");
                J = v(J, "HEAPU32");
                d = a._MakeManagedAnimation(d, N.length, c, B, J, g, l, q);
                a._free(c);
                a._free(B);
                a._free(J);
                return d;
            };
            (function (d) {
                d.Sd = d.Sd || [];
                d.Sd.push(function () {
                    d.Animation.prototype.render = function (c, g) {
                        S(g, ha);
                        this._render(c, ha);
                    };
                    d.Animation.prototype.size = function (c) {
                        this._size(ha);
                        var g = sa.toTypedArray();
                        return c
                            ? ((c[0] = g[0]), (c[1] = g[1]), c)
                            : g.slice(0, 2);
                    };
                    d.ManagedAnimation &&
                        ((d.ManagedAnimation.prototype.render = function (
                            c,
                            g,
                        ) {
                            S(g, ha);
                            this._render(c, ha);
                        }),
                        (d.ManagedAnimation.prototype.seek = function (c, g) {
                            this._seek(c, ha);
                            c = sa.toTypedArray();
                            return g ? (g.set(c), g) : c.slice();
                        }),
                        (d.ManagedAnimation.prototype.seekFrame = function (
                            c,
                            g,
                        ) {
                            this._seekFrame(c, ha);
                            c = sa.toTypedArray();
                            return g ? (g.set(c), g) : c.slice();
                        }),
                        (d.ManagedAnimation.prototype.setColor = function (
                            c,
                            g,
                        ) {
                            g = z(g);
                            return this._setColor(c, g);
                        }),
                        (d.ManagedAnimation.prototype.size = function (c) {
                            this._size(ha);
                            var g = sa.toTypedArray();
                            return c
                                ? ((c[0] = g[0]), (c[1] = g[1]), c)
                                : g.slice(0, 2);
                        }));
                });
            })(w);
            a.MakeParticles = function (d, c) {
                if (!a._MakeParticles) throw "Not compiled with MakeParticles";
                if (!c) return a._MakeParticles(d, 0, U, U, U);
                for (
                    var g = [], l = [], q = [], y = Object.keys(c || {}), B = 0;
                    B < y.length;
                    B++
                ) {
                    var J = y[B],
                        N = new Uint8Array(c[J]),
                        L = a._malloc(N.byteLength);
                    a.HEAPU8.set(N, L);
                    l.push(L);
                    q.push(N.byteLength);
                    N = oa(J) + 1;
                    L = a._malloc(N);
                    pa(J, K, L, N);
                    g.push(L);
                }
                c = v(g, "HEAPU32");
                l = v(l, "HEAPU32");
                q = v(q, "HEAPU32");
                d = a._MakeParticles(d, y.length, c, l, q);
                a._free(c);
                a._free(l);
                a._free(q);
                return d;
            };
            a.Sd = a.Sd || [];
            a.Sd.push(function () {
                a.ParticleEffect.prototype.uniforms = function () {
                    var d = this._uniformPtr(),
                        c = this.getUniformFloatCount();
                    return !d || 0 >= c
                        ? new Float32Array()
                        : new Float32Array(a.HEAPU8.buffer, d, c);
                };
                a.ParticleEffect.prototype.setPosition = function (d) {
                    this._setPosition(d[0], d[1]);
                };
            });
            a.Sd = a.Sd || [];
            a.Sd.push(function () {
                a.Path.prototype.op = function (d, c) {
                    return this._op(d, c) ? this : null;
                };
                a.Path.prototype.simplify = function () {
                    return this._simplify() ? this : null;
                };
            });
            a.Sd = a.Sd || [];
            a.Sd.push(function () {
                a.Canvas.prototype.drawText = function (d, c, g, l, q) {
                    var y = oa(d),
                        B = a._malloc(y + 1);
                    pa(d, K, B, y + 1);
                    this._drawSimpleText(B, y, c, g, q, l);
                    a._free(B);
                };
                a.Canvas.prototype.drawGlyphs = function (d, c, g, l, q, y) {
                    if (!(2 * d.length <= c.length))
                        throw "Not enough positions for the array of gyphs";
                    a.Pd(this.Nd);
                    const B = v(d, "HEAPU16"),
                        J = v(c, "HEAPF32");
                    this._drawGlyphs(d.length, B, J, g, l, q, y);
                    r(J, c);
                    r(B, d);
                };
                a.Font.prototype.getGlyphBounds = function (d, c, g) {
                    var l = v(d, "HEAPU16"),
                        q = a._malloc(16 * d.length);
                    this._getGlyphWidthBounds(l, d.length, U, q, c || null);
                    c = new Float32Array(a.HEAPU8.buffer, q, 4 * d.length);
                    r(l, d);
                    if (g) return g.set(c), a._free(q), g;
                    d = Float32Array.from(c);
                    a._free(q);
                    return d;
                };
                a.Font.prototype.getGlyphIDs = function (d, c, g) {
                    c || (c = d.length);
                    var l = oa(d) + 1,
                        q = a._malloc(l);
                    pa(d, K, q, l);
                    d = a._malloc(2 * c);
                    c = this._getGlyphIDs(q, l - 1, c, d);
                    a._free(q);
                    if (0 > c) return a._free(d), null;
                    q = new Uint16Array(a.HEAPU8.buffer, d, c);
                    if (g) return g.set(q), a._free(d), g;
                    g = Uint16Array.from(q);
                    a._free(d);
                    return g;
                };
                a.Font.prototype.getGlyphIntercepts = function (d, c, g, l) {
                    var q = v(d, "HEAPU16"),
                        y = v(c, "HEAPF32");
                    return this._getGlyphIntercepts(
                        q,
                        d.length,
                        !(d && d._ck),
                        y,
                        c.length,
                        !(c && c._ck),
                        g,
                        l,
                    );
                };
                a.Font.prototype.getGlyphWidths = function (d, c, g) {
                    var l = v(d, "HEAPU16"),
                        q = a._malloc(4 * d.length);
                    this._getGlyphWidthBounds(l, d.length, q, U, c || null);
                    c = new Float32Array(a.HEAPU8.buffer, q, d.length);
                    r(l, d);
                    if (g) return g.set(c), a._free(q), g;
                    d = Float32Array.from(c);
                    a._free(q);
                    return d;
                };
                a.FontMgr.FromData = function () {
                    if (!arguments.length) return null;
                    var d = arguments;
                    1 === d.length && Array.isArray(d[0]) && (d = arguments[0]);
                    if (!d.length) return null;
                    for (var c = [], g = [], l = 0; l < d.length; l++) {
                        var q = new Uint8Array(d[l]),
                            y = v(q, "HEAPU8");
                        c.push(y);
                        g.push(q.byteLength);
                    }
                    c = v(c, "HEAPU32");
                    g = v(g, "HEAPU32");
                    d = a.FontMgr._fromData(c, g, d.length);
                    a._free(c);
                    a._free(g);
                    return d;
                };
                a.Typeface.MakeFreeTypeFaceFromData = function (d) {
                    d = new Uint8Array(d);
                    var c = v(d, "HEAPU8");
                    return (d = a.Typeface._MakeFreeTypeFaceFromData(
                        c,
                        d.byteLength,
                    ))
                        ? d
                        : null;
                };
                a.Typeface.prototype.getGlyphIDs = function (d, c, g) {
                    c || (c = d.length);
                    var l = oa(d) + 1,
                        q = a._malloc(l);
                    pa(d, K, q, l);
                    d = a._malloc(2 * c);
                    c = this._getGlyphIDs(q, l - 1, c, d);
                    a._free(q);
                    if (0 > c) return a._free(d), null;
                    q = new Uint16Array(a.HEAPU8.buffer, d, c);
                    if (g) return g.set(q), a._free(d), g;
                    g = Uint16Array.from(q);
                    a._free(d);
                    return g;
                };
                a.TextBlob.MakeOnPath = function (d, c, g, l) {
                    if (d && d.length && c && c.countPoints()) {
                        if (1 === c.countPoints())
                            return this.MakeFromText(d, g);
                        l || (l = 0);
                        var q = g.getGlyphIDs(d);
                        q = g.getGlyphWidths(q);
                        var y = [];
                        c = new a.ContourMeasureIter(c, !1, 1);
                        for (
                            var B = c.next(), J = new Float32Array(4), N = 0;
                            N < d.length && B;
                            N++
                        ) {
                            var L = q[N];
                            l += L / 2;
                            if (l > B.length()) {
                                B.delete();
                                B = c.next();
                                if (!B) {
                                    d = d.substring(0, N);
                                    break;
                                }
                                l = L / 2;
                            }
                            B.getPosTan(l, J);
                            var W = J[2],
                                t = J[3];
                            y.push(
                                W,
                                t,
                                J[0] - (L / 2) * W,
                                J[1] - (L / 2) * t,
                            );
                            l += L / 2;
                        }
                        d = this.MakeFromRSXform(d, y, g);
                        B && B.delete();
                        c.delete();
                        return d;
                    }
                };
                a.TextBlob.MakeFromRSXform = function (d, c, g) {
                    var l = oa(d) + 1,
                        q = a._malloc(l);
                    pa(d, K, q, l);
                    d = v(c, "HEAPF32");
                    g = a.TextBlob._MakeFromRSXform(q, l - 1, d, g);
                    a._free(q);
                    return g ? g : null;
                };
                a.TextBlob.MakeFromRSXformGlyphs = function (d, c, g) {
                    var l = v(d, "HEAPU16");
                    c = v(c, "HEAPF32");
                    g = a.TextBlob._MakeFromRSXformGlyphs(
                        l,
                        2 * d.length,
                        c,
                        g,
                    );
                    r(l, d);
                    return g ? g : null;
                };
                a.TextBlob.MakeFromGlyphs = function (d, c) {
                    var g = v(d, "HEAPU16");
                    c = a.TextBlob._MakeFromGlyphs(g, 2 * d.length, c);
                    r(g, d);
                    return c ? c : null;
                };
                a.TextBlob.MakeFromText = function (d, c) {
                    var g = oa(d) + 1,
                        l = a._malloc(g);
                    pa(d, K, l, g);
                    d = a.TextBlob._MakeFromText(l, g - 1, c);
                    a._free(l);
                    return d ? d : null;
                };
                a.MallocGlyphIDs = function (d) {
                    return a.Malloc(Uint16Array, d);
                };
            });
            a.Sd = a.Sd || [];
            a.Sd.push(function () {
                a.MakePicture = function (d) {
                    d = new Uint8Array(d);
                    var c = a._malloc(d.byteLength);
                    a.HEAPU8.set(d, c);
                    return (d = a._MakePicture(c, d.byteLength)) ? d : null;
                };
            });
            a.Sd = a.Sd || [];
            a.Sd.push(function () {
                a.RuntimeEffect.Make = function (d, c) {
                    return a.RuntimeEffect._Make(d, {
                        onError:
                            c ||
                            function (g) {
                                console.log("RuntimeEffect error", g);
                            },
                    });
                };
                a.RuntimeEffect.prototype.makeShader = function (d, c) {
                    var g = !d._ck,
                        l = v(d, "HEAPF32");
                    c = I(c);
                    return this._makeShader(l, 4 * d.length, g, c);
                };
                a.RuntimeEffect.prototype.makeShaderWithChildren = function (
                    d,
                    c,
                    g,
                ) {
                    var l = !d._ck,
                        q = v(d, "HEAPF32");
                    g = I(g);
                    for (var y = [], B = 0; B < c.length; B++)
                        y.push(c[B].Md.Vd);
                    c = v(y, "HEAPU32");
                    return this._makeShaderWithChildren(
                        q,
                        4 * d.length,
                        l,
                        c,
                        y.length,
                        g,
                    );
                };
            });
            (function () {
                function d(H) {
                    for (var k = 0; k < H.length; k++)
                        if (void 0 !== H[k] && !Number.isFinite(H[k]))
                            return !1;
                    return !0;
                }
                function c(H) {
                    var k = a.getColorComponents(H);
                    H = k[0];
                    var p = k[1],
                        A = k[2];
                    k = k[3];
                    if (1 === k)
                        return (
                            (H = H.toString(16).toLowerCase()),
                            (p = p.toString(16).toLowerCase()),
                            (A = A.toString(16).toLowerCase()),
                            (H = 1 === H.length ? "0" + H : H),
                            (p = 1 === p.length ? "0" + p : p),
                            (A = 1 === A.length ? "0" + A : A),
                            "#" + H + p + A
                        );
                    k = 0 === k || 1 === k ? k : k.toFixed(8);
                    return "rgba(" + H + ", " + p + ", " + A + ", " + k + ")";
                }
                function g(H) {
                    return a.parseColorString(H, qa);
                }
                function l(H) {
                    H = ta.exec(H);
                    if (!H) return null;
                    var k = parseFloat(H[4]),
                        p = 16;
                    switch (H[5]) {
                        case "em":
                        case "rem":
                            p = 16 * k;
                            break;
                        case "pt":
                            p = (4 * k) / 3;
                            break;
                        case "px":
                            p = k;
                            break;
                        case "pc":
                            p = 16 * k;
                            break;
                        case "in":
                            p = 96 * k;
                            break;
                        case "cm":
                            p = (96 * k) / 2.54;
                            break;
                        case "mm":
                            p = (96 / 25.4) * k;
                            break;
                        case "q":
                            p = (96 / 25.4 / 4) * k;
                            break;
                        case "%":
                            p = (16 / 75) * k;
                    }
                    return {
                        style: H[1],
                        variant: H[2],
                        weight: H[3],
                        sizePx: p,
                        family: H[6].trim(),
                    };
                }
                function q(H) {
                    this.Od = H;
                    this.Rd = new a.Paint();
                    this.Rd.setAntiAlias(!0);
                    this.Rd.setStrokeMiter(10);
                    this.Rd.setStrokeCap(a.StrokeCap.Butt);
                    this.Rd.setStrokeJoin(a.StrokeJoin.Miter);
                    this.Re = "10px monospace";
                    this.oe = new a.Font(null, 10);
                    this.oe.setSubpixel(!0);
                    this.ce = this.ie = a.BLACK;
                    this.xe = 0;
                    this.Ke = a.TRANSPARENT;
                    this.ze = this.ye = 0;
                    this.Le = this.le = 1;
                    this.Je = 0;
                    this.we = [];
                    this.Qd = a.BlendMode.SrcOver;
                    this.Rd.setStrokeWidth(this.Le);
                    this.Rd.setBlendMode(this.Qd);
                    this.Ud = new a.Path();
                    this.Wd = a.Matrix.identity();
                    this.nf = [];
                    this.De = [];
                    this.ne = function () {
                        this.Ud.delete();
                        this.Rd.delete();
                        this.oe.delete();
                        this.De.forEach(function (k) {
                            k.ne();
                        });
                    };
                    Object.defineProperty(this, "currentTransform", {
                        enumerable: !0,
                        get: function () {
                            return {
                                a: this.Wd[0],
                                c: this.Wd[1],
                                e: this.Wd[2],
                                b: this.Wd[3],
                                d: this.Wd[4],
                                f: this.Wd[5],
                            };
                        },
                        set: function (k) {
                            k.a &&
                                this.setTransform(k.a, k.b, k.c, k.d, k.e, k.f);
                        },
                    });
                    Object.defineProperty(this, "fillStyle", {
                        enumerable: !0,
                        get: function () {
                            return f(this.ce) ? c(this.ce) : this.ce;
                        },
                        set: function (k) {
                            "string" === typeof k
                                ? (this.ce = g(k))
                                : k.ve && (this.ce = k);
                        },
                    });
                    Object.defineProperty(this, "font", {
                        enumerable: !0,
                        get: function () {
                            return this.Re;
                        },
                        set: function (k) {
                            var p = l(k),
                                A = p.family;
                            p.typeface = ba[A]
                                ? ba[A][
                                      (p.style || "normal") +
                                          "|" +
                                          (p.variant || "normal") +
                                          "|" +
                                          (p.weight || "normal")
                                  ] || ba[A]["*"]
                                : null;
                            p &&
                                (this.oe.setSize(p.sizePx),
                                this.oe.setTypeface(p.typeface),
                                (this.Re = k));
                        },
                    });
                    Object.defineProperty(this, "globalAlpha", {
                        enumerable: !0,
                        get: function () {
                            return this.le;
                        },
                        set: function (k) {
                            !isFinite(k) || 0 > k || 1 < k || (this.le = k);
                        },
                    });
                    Object.defineProperty(this, "globalCompositeOperation", {
                        enumerable: !0,
                        get: function () {
                            switch (this.Qd) {
                                case a.BlendMode.SrcOver:
                                    return "source-over";
                                case a.BlendMode.DstOver:
                                    return "destination-over";
                                case a.BlendMode.Src:
                                    return "copy";
                                case a.BlendMode.Dst:
                                    return "destination";
                                case a.BlendMode.Clear:
                                    return "clear";
                                case a.BlendMode.SrcIn:
                                    return "source-in";
                                case a.BlendMode.DstIn:
                                    return "destination-in";
                                case a.BlendMode.SrcOut:
                                    return "source-out";
                                case a.BlendMode.DstOut:
                                    return "destination-out";
                                case a.BlendMode.SrcATop:
                                    return "source-atop";
                                case a.BlendMode.DstATop:
                                    return "destination-atop";
                                case a.BlendMode.Xor:
                                    return "xor";
                                case a.BlendMode.Plus:
                                    return "lighter";
                                case a.BlendMode.Multiply:
                                    return "multiply";
                                case a.BlendMode.Screen:
                                    return "screen";
                                case a.BlendMode.Overlay:
                                    return "overlay";
                                case a.BlendMode.Darken:
                                    return "darken";
                                case a.BlendMode.Lighten:
                                    return "lighten";
                                case a.BlendMode.ColorDodge:
                                    return "color-dodge";
                                case a.BlendMode.ColorBurn:
                                    return "color-burn";
                                case a.BlendMode.HardLight:
                                    return "hard-light";
                                case a.BlendMode.SoftLight:
                                    return "soft-light";
                                case a.BlendMode.Difference:
                                    return "difference";
                                case a.BlendMode.Exclusion:
                                    return "exclusion";
                                case a.BlendMode.Hue:
                                    return "hue";
                                case a.BlendMode.Saturation:
                                    return "saturation";
                                case a.BlendMode.Color:
                                    return "color";
                                case a.BlendMode.Luminosity:
                                    return "luminosity";
                            }
                        },
                        set: function (k) {
                            switch (k) {
                                case "source-over":
                                    this.Qd = a.BlendMode.SrcOver;
                                    break;
                                case "destination-over":
                                    this.Qd = a.BlendMode.DstOver;
                                    break;
                                case "copy":
                                    this.Qd = a.BlendMode.Src;
                                    break;
                                case "destination":
                                    this.Qd = a.BlendMode.Dst;
                                    break;
                                case "clear":
                                    this.Qd = a.BlendMode.Clear;
                                    break;
                                case "source-in":
                                    this.Qd = a.BlendMode.SrcIn;
                                    break;
                                case "destination-in":
                                    this.Qd = a.BlendMode.DstIn;
                                    break;
                                case "source-out":
                                    this.Qd = a.BlendMode.SrcOut;
                                    break;
                                case "destination-out":
                                    this.Qd = a.BlendMode.DstOut;
                                    break;
                                case "source-atop":
                                    this.Qd = a.BlendMode.SrcATop;
                                    break;
                                case "destination-atop":
                                    this.Qd = a.BlendMode.DstATop;
                                    break;
                                case "xor":
                                    this.Qd = a.BlendMode.Xor;
                                    break;
                                case "lighter":
                                    this.Qd = a.BlendMode.Plus;
                                    break;
                                case "plus-lighter":
                                    this.Qd = a.BlendMode.Plus;
                                    break;
                                case "plus-darker":
                                    throw "plus-darker is not supported";
                                case "multiply":
                                    this.Qd = a.BlendMode.Multiply;
                                    break;
                                case "screen":
                                    this.Qd = a.BlendMode.Screen;
                                    break;
                                case "overlay":
                                    this.Qd = a.BlendMode.Overlay;
                                    break;
                                case "darken":
                                    this.Qd = a.BlendMode.Darken;
                                    break;
                                case "lighten":
                                    this.Qd = a.BlendMode.Lighten;
                                    break;
                                case "color-dodge":
                                    this.Qd = a.BlendMode.ColorDodge;
                                    break;
                                case "color-burn":
                                    this.Qd = a.BlendMode.ColorBurn;
                                    break;
                                case "hard-light":
                                    this.Qd = a.BlendMode.HardLight;
                                    break;
                                case "soft-light":
                                    this.Qd = a.BlendMode.SoftLight;
                                    break;
                                case "difference":
                                    this.Qd = a.BlendMode.Difference;
                                    break;
                                case "exclusion":
                                    this.Qd = a.BlendMode.Exclusion;
                                    break;
                                case "hue":
                                    this.Qd = a.BlendMode.Hue;
                                    break;
                                case "saturation":
                                    this.Qd = a.BlendMode.Saturation;
                                    break;
                                case "color":
                                    this.Qd = a.BlendMode.Color;
                                    break;
                                case "luminosity":
                                    this.Qd = a.BlendMode.Luminosity;
                                    break;
                                default:
                                    return;
                            }
                            this.Rd.setBlendMode(this.Qd);
                        },
                    });
                    Object.defineProperty(this, "imageSmoothingEnabled", {
                        enumerable: !0,
                        get: function () {
                            return !0;
                        },
                        set: function () {},
                    });
                    Object.defineProperty(this, "imageSmoothingQuality", {
                        enumerable: !0,
                        get: function () {
                            return "high";
                        },
                        set: function () {},
                    });
                    Object.defineProperty(this, "lineCap", {
                        enumerable: !0,
                        get: function () {
                            switch (this.Rd.getStrokeCap()) {
                                case a.StrokeCap.Butt:
                                    return "butt";
                                case a.StrokeCap.Round:
                                    return "round";
                                case a.StrokeCap.Square:
                                    return "square";
                            }
                        },
                        set: function (k) {
                            switch (k) {
                                case "butt":
                                    this.Rd.setStrokeCap(a.StrokeCap.Butt);
                                    break;
                                case "round":
                                    this.Rd.setStrokeCap(a.StrokeCap.Round);
                                    break;
                                case "square":
                                    this.Rd.setStrokeCap(a.StrokeCap.Square);
                            }
                        },
                    });
                    Object.defineProperty(this, "lineDashOffset", {
                        enumerable: !0,
                        get: function () {
                            return this.Je;
                        },
                        set: function (k) {
                            isFinite(k) && (this.Je = k);
                        },
                    });
                    Object.defineProperty(this, "lineJoin", {
                        enumerable: !0,
                        get: function () {
                            switch (this.Rd.getStrokeJoin()) {
                                case a.StrokeJoin.Miter:
                                    return "miter";
                                case a.StrokeJoin.Round:
                                    return "round";
                                case a.StrokeJoin.Bevel:
                                    return "bevel";
                            }
                        },
                        set: function (k) {
                            switch (k) {
                                case "miter":
                                    this.Rd.setStrokeJoin(a.StrokeJoin.Miter);
                                    break;
                                case "round":
                                    this.Rd.setStrokeJoin(a.StrokeJoin.Round);
                                    break;
                                case "bevel":
                                    this.Rd.setStrokeJoin(a.StrokeJoin.Bevel);
                            }
                        },
                    });
                    Object.defineProperty(this, "lineWidth", {
                        enumerable: !0,
                        get: function () {
                            return this.Rd.getStrokeWidth();
                        },
                        set: function (k) {
                            0 >= k ||
                                !k ||
                                ((this.Le = k), this.Rd.setStrokeWidth(k));
                        },
                    });
                    Object.defineProperty(this, "miterLimit", {
                        enumerable: !0,
                        get: function () {
                            return this.Rd.getStrokeMiter();
                        },
                        set: function (k) {
                            0 >= k || !k || this.Rd.setStrokeMiter(k);
                        },
                    });
                    Object.defineProperty(this, "shadowBlur", {
                        enumerable: !0,
                        get: function () {
                            return this.xe;
                        },
                        set: function (k) {
                            0 > k || !isFinite(k) || (this.xe = k);
                        },
                    });
                    Object.defineProperty(this, "shadowColor", {
                        enumerable: !0,
                        get: function () {
                            return c(this.Ke);
                        },
                        set: function (k) {
                            this.Ke = g(k);
                        },
                    });
                    Object.defineProperty(this, "shadowOffsetX", {
                        enumerable: !0,
                        get: function () {
                            return this.ye;
                        },
                        set: function (k) {
                            isFinite(k) && (this.ye = k);
                        },
                    });
                    Object.defineProperty(this, "shadowOffsetY", {
                        enumerable: !0,
                        get: function () {
                            return this.ze;
                        },
                        set: function (k) {
                            isFinite(k) && (this.ze = k);
                        },
                    });
                    Object.defineProperty(this, "strokeStyle", {
                        enumerable: !0,
                        get: function () {
                            return c(this.ie);
                        },
                        set: function (k) {
                            "string" === typeof k
                                ? (this.ie = g(k))
                                : k.ve && (this.ie = k);
                        },
                    });
                    this.arc = function (k, p, A, C, E, G) {
                        F(this.Ud, k, p, A, A, 0, C, E, G);
                    };
                    this.arcTo = function (k, p, A, C, E) {
                        L(this.Ud, k, p, A, C, E);
                    };
                    this.beginPath = function () {
                        this.Ud.delete();
                        this.Ud = new a.Path();
                    };
                    this.bezierCurveTo = function (k, p, A, C, E, G) {
                        var M = this.Ud;
                        d([k, p, A, C, E, G]) &&
                            (M.isEmpty() && M.moveTo(k, p),
                            M.cubicTo(k, p, A, C, E, G));
                    };
                    this.clearRect = function (k, p, A, C) {
                        this.Rd.setStyle(a.PaintStyle.Fill);
                        this.Rd.setBlendMode(a.BlendMode.Clear);
                        this.Od.drawRect(a.XYWHRect(k, p, A, C), this.Rd);
                        this.Rd.setBlendMode(this.Qd);
                    };
                    this.clip = function (k, p) {
                        "string" === typeof k
                            ? ((p = k), (k = this.Ud))
                            : k && k.$e && (k = k.Xd);
                        k || (k = this.Ud);
                        k = k.copy();
                        p && "evenodd" === p.toLowerCase()
                            ? k.setFillType(a.FillType.EvenOdd)
                            : k.setFillType(a.FillType.Winding);
                        this.Od.clipPath(k, a.ClipOp.Intersect, !0);
                        k.delete();
                    };
                    this.closePath = function () {
                        W(this.Ud);
                    };
                    this.createImageData = function () {
                        if (1 === arguments.length) {
                            var k = arguments[0];
                            return new J(
                                new Uint8ClampedArray(4 * k.width * k.height),
                                k.width,
                                k.height,
                            );
                        }
                        if (2 === arguments.length) {
                            k = arguments[0];
                            var p = arguments[1];
                            return new J(
                                new Uint8ClampedArray(4 * k * p),
                                k,
                                p,
                            );
                        }
                        throw (
                            "createImageData expects 1 or 2 arguments, got " +
                            arguments.length
                        );
                    };
                    this.createLinearGradient = function (k, p, A, C) {
                        if (d(arguments)) {
                            var E = new N(k, p, A, C);
                            this.De.push(E);
                            return E;
                        }
                    };
                    this.createPattern = function (k, p) {
                        k = new ia(k, p);
                        this.De.push(k);
                        return k;
                    };
                    this.createRadialGradient = function (k, p, A, C, E, G) {
                        if (d(arguments)) {
                            var M = new la(k, p, A, C, E, G);
                            this.De.push(M);
                            return M;
                        }
                    };
                    this.drawImage = function (k) {
                        k instanceof B && (k = k.uf());
                        var p = this.Qe();
                        if (3 === arguments.length || 5 === arguments.length)
                            var A = a.XYWHRect(
                                    arguments[1],
                                    arguments[2],
                                    arguments[3] || k.width(),
                                    arguments[4] || k.height(),
                                ),
                                C = a.XYWHRect(0, 0, k.width(), k.height());
                        else if (9 === arguments.length)
                            (A = a.XYWHRect(
                                arguments[5],
                                arguments[6],
                                arguments[7],
                                arguments[8],
                            )),
                                (C = a.XYWHRect(
                                    arguments[1],
                                    arguments[2],
                                    arguments[3],
                                    arguments[4],
                                ));
                        else
                            throw (
                                "invalid number of args for drawImage, need 3, 5, or 9; got " +
                                arguments.length
                            );
                        this.Od.drawImageRect(k, C, A, p, !1);
                        p.dispose();
                    };
                    this.ellipse = function (k, p, A, C, E, G, M, ca) {
                        F(this.Ud, k, p, A, C, E, G, M, ca);
                    };
                    this.Qe = function () {
                        var k = this.Rd.copy();
                        k.setStyle(a.PaintStyle.Fill);
                        if (f(this.ce)) {
                            var p = a.multiplyByAlpha(this.ce, this.le);
                            k.setColor(p);
                        } else
                            (p = this.ce.ve(this.Wd)),
                                k.setColor(a.Color(0, 0, 0, this.le)),
                                k.setShader(p);
                        k.dispose = function () {
                            this.delete();
                        };
                        return k;
                    };
                    this.fill = function (k, p) {
                        "string" === typeof k
                            ? ((p = k), (k = this.Ud))
                            : k && k.$e && (k = k.Xd);
                        if ("evenodd" === p)
                            this.Ud.setFillType(a.FillType.EvenOdd);
                        else {
                            if ("nonzero" !== p && p) throw "invalid fill rule";
                            this.Ud.setFillType(a.FillType.Winding);
                        }
                        k || (k = this.Ud);
                        p = this.Qe();
                        var A = this.Ae(p);
                        A &&
                            (this.Od.save(),
                            this.te(),
                            this.Od.drawPath(k, A),
                            this.Od.restore(),
                            A.dispose());
                        this.Od.drawPath(k, p);
                        p.dispose();
                    };
                    this.fillRect = function (k, p, A, C) {
                        var E = this.Qe(),
                            G = this.Ae(E);
                        G &&
                            (this.Od.save(),
                            this.te(),
                            this.Od.drawRect(a.XYWHRect(k, p, A, C), G),
                            this.Od.restore(),
                            G.dispose());
                        this.Od.drawRect(a.XYWHRect(k, p, A, C), E);
                        E.dispose();
                    };
                    this.fillText = function (k, p, A) {
                        var C = this.Qe();
                        k = a.TextBlob.MakeFromText(k, this.oe);
                        var E = this.Ae(C);
                        E &&
                            (this.Od.save(),
                            this.te(),
                            this.Od.drawTextBlob(k, p, A, E),
                            this.Od.restore(),
                            E.dispose());
                        this.Od.drawTextBlob(k, p, A, C);
                        k.delete();
                        C.dispose();
                    };
                    this.getImageData = function (k, p, A, C) {
                        return (k = this.Od.readPixels(k, p, {
                            width: A,
                            height: C,
                            colorType: a.ColorType.RGBA_8888,
                            alphaType: a.AlphaType.Unpremul,
                            colorSpace: a.ColorSpace.SRGB,
                        }))
                            ? new J(new Uint8ClampedArray(k.buffer), A, C)
                            : null;
                    };
                    this.getLineDash = function () {
                        return this.we.slice();
                    };
                    this.pf = function (k) {
                        var p = a.Matrix.invert(this.Wd);
                        a.Matrix.mapPoints(p, k);
                        return k;
                    };
                    this.isPointInPath = function (k, p, A) {
                        var C = arguments;
                        if (3 === C.length) var E = this.Ud;
                        else if (4 === C.length)
                            (E = C[0]), (k = C[1]), (p = C[2]), (A = C[3]);
                        else
                            throw (
                                "invalid arg count, need 3 or 4, got " +
                                C.length
                            );
                        if (!isFinite(k) || !isFinite(p)) return !1;
                        A = A || "nonzero";
                        if ("nonzero" !== A && "evenodd" !== A) return !1;
                        C = this.pf([k, p]);
                        k = C[0];
                        p = C[1];
                        E.setFillType(
                            "nonzero" === A
                                ? a.FillType.Winding
                                : a.FillType.EvenOdd,
                        );
                        return E.contains(k, p);
                    };
                    this.isPointInStroke = function (k, p) {
                        var A = arguments;
                        if (2 === A.length) var C = this.Ud;
                        else if (3 === A.length)
                            (C = A[0]), (k = A[1]), (p = A[2]);
                        else
                            throw (
                                "invalid arg count, need 2 or 3, got " +
                                A.length
                            );
                        if (!isFinite(k) || !isFinite(p)) return !1;
                        A = this.pf([k, p]);
                        k = A[0];
                        p = A[1];
                        C = C.copy();
                        C.setFillType(a.FillType.Winding);
                        C.stroke({
                            width: this.lineWidth,
                            miter_limit: this.miterLimit,
                            cap: this.Rd.getStrokeCap(),
                            join: this.Rd.getStrokeJoin(),
                            precision: 0.3,
                        });
                        A = C.contains(k, p);
                        C.delete();
                        return A;
                    };
                    this.lineTo = function (k, p) {
                        Q(this.Ud, k, p);
                    };
                    this.measureText = function (k) {
                        k = this.oe.getGlyphIDs(k);
                        k = this.oe.getGlyphWidths(k);
                        let p = 0;
                        for (const A of k) p += A;
                        return { width: p };
                    };
                    this.moveTo = function (k, p) {
                        var A = this.Ud;
                        d([k, p]) && A.moveTo(k, p);
                    };
                    this.putImageData = function (k, p, A, C, E, G, M) {
                        if (d([p, A, C, E, G, M]))
                            if (void 0 === C)
                                this.Od.writePixels(
                                    k.data,
                                    k.width,
                                    k.height,
                                    p,
                                    A,
                                );
                            else if (
                                ((C = C || 0),
                                (E = E || 0),
                                (G = G || k.width),
                                (M = M || k.height),
                                0 > G && ((C += G), (G = Math.abs(G))),
                                0 > M && ((E += M), (M = Math.abs(M))),
                                0 > C && ((G += C), (C = 0)),
                                0 > E && ((M += E), (E = 0)),
                                !(0 >= G || 0 >= M))
                            ) {
                                k = a.MakeImage(
                                    {
                                        width: k.width,
                                        height: k.height,
                                        alphaType: a.AlphaType.Unpremul,
                                        colorType: a.ColorType.RGBA_8888,
                                        colorSpace: a.ColorSpace.SRGB,
                                    },
                                    k.data,
                                    4 * k.width,
                                );
                                var ca = a.XYWHRect(C, E, G, M);
                                p = a.XYWHRect(p + C, A + E, G, M);
                                A = a.Matrix.invert(this.Wd);
                                this.Od.save();
                                this.Od.concat(A);
                                this.Od.drawImageRect(k, ca, p, null, !1);
                                this.Od.restore();
                                k.delete();
                            }
                    };
                    this.quadraticCurveTo = function (k, p, A, C) {
                        var E = this.Ud;
                        d([k, p, A, C]) &&
                            (E.isEmpty() && E.moveTo(k, p),
                            E.quadTo(k, p, A, C));
                    };
                    this.rect = function (k, p, A, C) {
                        var E = this.Ud;
                        k = a.XYWHRect(k, p, A, C);
                        d(k) && E.addRect(k);
                    };
                    this.resetTransform = function () {
                        this.Ud.transform(this.Wd);
                        var k = a.Matrix.invert(this.Wd);
                        this.Od.concat(k);
                        this.Wd = this.Od.getTotalMatrix();
                    };
                    this.restore = function () {
                        var k = this.nf.pop();
                        if (k) {
                            var p = a.Matrix.multiply(
                                this.Wd,
                                a.Matrix.invert(k.Hf),
                            );
                            this.Ud.transform(p);
                            this.Rd.delete();
                            this.Rd = k.ag;
                            this.we = k.Zf;
                            this.Le = k.ng;
                            this.ie = k.mg;
                            this.ce = k.fs;
                            this.ye = k.kg;
                            this.ze = k.lg;
                            this.xe = k.eg;
                            this.Ke = k.jg;
                            this.le = k.Of;
                            this.Qd = k.Pf;
                            this.Je = k.$f;
                            this.Re = k.Nf;
                            this.Od.restore();
                            this.Wd = this.Od.getTotalMatrix();
                        }
                    };
                    this.rotate = function (k) {
                        if (isFinite(k)) {
                            var p = a.Matrix.rotated(-k);
                            this.Ud.transform(p);
                            this.Od.rotate((k / Math.PI) * 180, 0, 0);
                            this.Wd = this.Od.getTotalMatrix();
                        }
                    };
                    this.save = function () {
                        if (this.ce.ue) {
                            var k = this.ce.ue();
                            this.De.push(k);
                        } else k = this.ce;
                        if (this.ie.ue) {
                            var p = this.ie.ue();
                            this.De.push(p);
                        } else p = this.ie;
                        this.nf.push({
                            Hf: this.Wd.slice(),
                            Zf: this.we.slice(),
                            ng: this.Le,
                            mg: p,
                            fs: k,
                            kg: this.ye,
                            lg: this.ze,
                            eg: this.xe,
                            jg: this.Ke,
                            Of: this.le,
                            $f: this.Je,
                            Pf: this.Qd,
                            ag: this.Rd.copy(),
                            Nf: this.Re,
                        });
                        this.Od.save();
                    };
                    this.scale = function (k, p) {
                        if (d(arguments)) {
                            var A = a.Matrix.scaled(1 / k, 1 / p);
                            this.Ud.transform(A);
                            this.Od.scale(k, p);
                            this.Wd = this.Od.getTotalMatrix();
                        }
                    };
                    this.setLineDash = function (k) {
                        for (var p = 0; p < k.length; p++)
                            if (!isFinite(k[p]) || 0 > k[p]) return;
                        1 === k.length % 2 && Array.prototype.push.apply(k, k);
                        this.we = k;
                    };
                    this.setTransform = function (k, p, A, C, E, G) {
                        d(arguments) &&
                            (this.resetTransform(),
                            this.transform(k, p, A, C, E, G));
                    };
                    this.te = function () {
                        var k = a.Matrix.invert(this.Wd);
                        this.Od.concat(k);
                        this.Od.concat(a.Matrix.translated(this.ye, this.ze));
                        this.Od.concat(this.Wd);
                    };
                    this.Ae = function (k) {
                        var p = a.multiplyByAlpha(this.Ke, this.le);
                        if (
                            !a.getColorComponents(p)[3] ||
                            !(this.xe || this.ze || this.ye)
                        )
                            return null;
                        k = k.copy();
                        k.setColor(p);
                        var A = a.MaskFilter.MakeBlur(
                            a.BlurStyle.Normal,
                            this.xe / 2,
                            !1,
                        );
                        k.setMaskFilter(A);
                        k.dispose = function () {
                            A.delete();
                            this.delete();
                        };
                        return k;
                    };
                    this.bf = function () {
                        var k = this.Rd.copy();
                        k.setStyle(a.PaintStyle.Stroke);
                        if (f(this.ie)) {
                            var p = a.multiplyByAlpha(this.ie, this.le);
                            k.setColor(p);
                        } else
                            (p = this.ie.ve(this.Wd)),
                                k.setColor(a.Color(0, 0, 0, this.le)),
                                k.setShader(p);
                        k.setStrokeWidth(this.Le);
                        if (this.we.length) {
                            var A = a.PathEffect.MakeDash(this.we, this.Je);
                            k.setPathEffect(A);
                        }
                        k.dispose = function () {
                            A && A.delete();
                            this.delete();
                        };
                        return k;
                    };
                    this.stroke = function (k) {
                        k = k ? k.Xd : this.Ud;
                        var p = this.bf(),
                            A = this.Ae(p);
                        A &&
                            (this.Od.save(),
                            this.te(),
                            this.Od.drawPath(k, A),
                            this.Od.restore(),
                            A.dispose());
                        this.Od.drawPath(k, p);
                        p.dispose();
                    };
                    this.strokeRect = function (k, p, A, C) {
                        var E = this.bf(),
                            G = this.Ae(E);
                        G &&
                            (this.Od.save(),
                            this.te(),
                            this.Od.drawRect(a.XYWHRect(k, p, A, C), G),
                            this.Od.restore(),
                            G.dispose());
                        this.Od.drawRect(a.XYWHRect(k, p, A, C), E);
                        E.dispose();
                    };
                    this.strokeText = function (k, p, A) {
                        var C = this.bf();
                        k = a.TextBlob.MakeFromText(k, this.oe);
                        var E = this.Ae(C);
                        E &&
                            (this.Od.save(),
                            this.te(),
                            this.Od.drawTextBlob(k, p, A, E),
                            this.Od.restore(),
                            E.dispose());
                        this.Od.drawTextBlob(k, p, A, C);
                        k.delete();
                        C.dispose();
                    };
                    this.translate = function (k, p) {
                        if (d(arguments)) {
                            var A = a.Matrix.translated(-k, -p);
                            this.Ud.transform(A);
                            this.Od.translate(k, p);
                            this.Wd = this.Od.getTotalMatrix();
                        }
                    };
                    this.transform = function (k, p, A, C, E, G) {
                        k = [k, A, E, p, C, G, 0, 0, 1];
                        p = a.Matrix.invert(k);
                        this.Ud.transform(p);
                        this.Od.concat(k);
                        this.Wd = this.Od.getTotalMatrix();
                    };
                    this.addHitRegion = function () {};
                    this.clearHitRegions = function () {};
                    this.drawFocusIfNeeded = function () {};
                    this.removeHitRegion = function () {};
                    this.scrollPathIntoView = function () {};
                    Object.defineProperty(this, "canvas", {
                        value: null,
                        writable: !1,
                    });
                }
                function y(H) {
                    this.cf = H;
                    this.Nd = new q(H.getCanvas());
                    this.Se = [];
                    this.decodeImage = function (k) {
                        k = a.MakeImageFromEncoded(k);
                        if (!k) throw "Invalid input";
                        this.Se.push(k);
                        return new B(k);
                    };
                    this.loadFont = function (k, p) {
                        k = a.Typeface.MakeFreeTypeFaceFromData(k);
                        if (!k) return null;
                        this.Se.push(k);
                        var A =
                            (p.style || "normal") +
                            "|" +
                            (p.variant || "normal") +
                            "|" +
                            (p.weight || "normal");
                        p = p.family;
                        ba[p] || (ba[p] = { "*": k });
                        ba[p][A] = k;
                    };
                    this.makePath2D = function (k) {
                        k = new Y(k);
                        this.Se.push(k.Xd);
                        return k;
                    };
                    this.getContext = function (k) {
                        return "2d" === k ? this.Nd : null;
                    };
                    this.toDataURL = function (k, p) {
                        this.cf.flush();
                        var A = this.cf.makeImageSnapshot();
                        if (A) {
                            k = k || "image/png";
                            var C = a.ImageFormat.PNG;
                            "image/jpeg" === k && (C = a.ImageFormat.JPEG);
                            if ((p = A.encodeToBytes(C, p || 0.92))) {
                                A.delete();
                                k = "data:" + k + ";base64,";
                                if ("undefined" !== typeof Buffer)
                                    p = Buffer.from(p).toString("base64");
                                else {
                                    A = 0;
                                    C = p.length;
                                    for (var E = "", G; A < C; )
                                        (G = p.slice(
                                            A,
                                            Math.min(A + 32768, C),
                                        )),
                                            (E += String.fromCharCode.apply(
                                                null,
                                                G,
                                            )),
                                            (A += 32768);
                                    p = btoa(E);
                                }
                                return k + p;
                            }
                        }
                    };
                    this.dispose = function () {
                        this.Nd.ne();
                        this.Se.forEach(function (k) {
                            k.delete();
                        });
                        this.cf.dispose();
                    };
                }
                function B(H) {
                    this.width = H.width();
                    this.height = H.height();
                    this.naturalWidth = this.width;
                    this.naturalHeight = this.height;
                    this.uf = function () {
                        return H;
                    };
                }
                function J(H, k, p) {
                    if (!k || 0 === p)
                        throw "invalid dimensions, width and height must be non-zero";
                    if (H.length % 4) throw "arr must be a multiple of 4";
                    p = p || H.length / (4 * k);
                    Object.defineProperty(this, "data", {
                        value: H,
                        writable: !1,
                    });
                    Object.defineProperty(this, "height", {
                        value: p,
                        writable: !1,
                    });
                    Object.defineProperty(this, "width", {
                        value: k,
                        writable: !1,
                    });
                }
                function N(H, k, p, A) {
                    this.Zd = null;
                    this.ee = [];
                    this.be = [];
                    this.addColorStop = function (C, E) {
                        if (0 > C || 1 < C || !isFinite(C))
                            throw "offset must be between 0 and 1 inclusively";
                        E = g(E);
                        var G = this.be.indexOf(C);
                        if (-1 !== G) this.ee[G] = E;
                        else {
                            for (
                                G = 0;
                                G < this.be.length && !(this.be[G] > C);
                                G++
                            );
                            this.be.splice(G, 0, C);
                            this.ee.splice(G, 0, E);
                        }
                    };
                    this.ue = function () {
                        var C = new N(H, k, p, A);
                        C.ee = this.ee.slice();
                        C.be = this.be.slice();
                        return C;
                    };
                    this.ne = function () {
                        this.Zd && (this.Zd.delete(), (this.Zd = null));
                    };
                    this.ve = function (C) {
                        var E = [H, k, p, A];
                        a.Matrix.mapPoints(C, E);
                        C = E[0];
                        var G = E[1],
                            M = E[2];
                        E = E[3];
                        this.ne();
                        return (this.Zd = a.Shader.MakeLinearGradient(
                            [C, G],
                            [M, E],
                            this.ee,
                            this.be,
                            a.TileMode.Clamp,
                        ));
                    };
                }
                function L(H, k, p, A, C, E) {
                    if (d([k, p, A, C, E])) {
                        if (0 > E) throw "radii cannot be negative";
                        H.isEmpty() && H.moveTo(k, p);
                        H.arcToTangent(k, p, A, C, E);
                    }
                }
                function W(H) {
                    if (!H.isEmpty()) {
                        var k = H.getBounds();
                        (k[3] - k[1] || k[2] - k[0]) && H.close();
                    }
                }
                function t(H, k, p, A, C, E, G) {
                    G = ((G - E) / Math.PI) * 180;
                    E = (E / Math.PI) * 180;
                    k = a.LTRBRect(k - A, p - C, k + A, p + C);
                    1e-5 > Math.abs(Math.abs(G) - 360)
                        ? ((p = G / 2),
                          H.arcToOval(k, E, p, !1),
                          H.arcToOval(k, E + p, p, !1))
                        : H.arcToOval(k, E, G, !1);
                }
                function F(H, k, p, A, C, E, G, M, ca) {
                    if (d([k, p, A, C, E, G, M])) {
                        if (0 > A || 0 > C) throw "radii cannot be negative";
                        var da = 2 * Math.PI,
                            Ia = G % da;
                        0 > Ia && (Ia += da);
                        var bb = Ia - G;
                        G = Ia;
                        M += bb;
                        !ca && M - G >= da
                            ? (M = G + da)
                            : ca && G - M >= da
                            ? (M = G - da)
                            : !ca && G > M
                            ? (M = G + (da - ((G - M) % da)))
                            : ca && G < M && (M = G - (da - ((M - G) % da)));
                        E
                            ? ((ca = a.Matrix.rotated(E, k, p)),
                              (E = a.Matrix.rotated(-E, k, p)),
                              H.transform(E),
                              t(H, k, p, A, C, G, M),
                              H.transform(ca))
                            : t(H, k, p, A, C, G, M);
                    }
                }
                function Q(H, k, p) {
                    d([k, p]) &&
                        (H.isEmpty() && H.moveTo(k, p), H.lineTo(k, p));
                }
                function Y(H) {
                    this.Xd = null;
                    this.Xd =
                        "string" === typeof H
                            ? a.Path.MakeFromSVGString(H)
                            : H && H.$e
                            ? H.Xd.copy()
                            : new a.Path();
                    this.$e = function () {
                        return this.Xd;
                    };
                    this.addPath = function (k, p) {
                        p || (p = { a: 1, c: 0, e: 0, b: 0, d: 1, f: 0 });
                        this.Xd.addPath(k.Xd, [p.a, p.c, p.e, p.b, p.d, p.f]);
                    };
                    this.arc = function (k, p, A, C, E, G) {
                        F(this.Xd, k, p, A, A, 0, C, E, G);
                    };
                    this.arcTo = function (k, p, A, C, E) {
                        L(this.Xd, k, p, A, C, E);
                    };
                    this.bezierCurveTo = function (k, p, A, C, E, G) {
                        var M = this.Xd;
                        d([k, p, A, C, E, G]) &&
                            (M.isEmpty() && M.moveTo(k, p),
                            M.cubicTo(k, p, A, C, E, G));
                    };
                    this.closePath = function () {
                        W(this.Xd);
                    };
                    this.ellipse = function (k, p, A, C, E, G, M, ca) {
                        F(this.Xd, k, p, A, C, E, G, M, ca);
                    };
                    this.lineTo = function (k, p) {
                        Q(this.Xd, k, p);
                    };
                    this.moveTo = function (k, p) {
                        var A = this.Xd;
                        d([k, p]) && A.moveTo(k, p);
                    };
                    this.quadraticCurveTo = function (k, p, A, C) {
                        var E = this.Xd;
                        d([k, p, A, C]) &&
                            (E.isEmpty() && E.moveTo(k, p),
                            E.quadTo(k, p, A, C));
                    };
                    this.rect = function (k, p, A, C) {
                        var E = this.Xd;
                        k = a.XYWHRect(k, p, A, C);
                        d(k) && E.addRect(k);
                    };
                }
                function ia(H, k) {
                    this.Zd = null;
                    H instanceof B && (H = H.uf());
                    this.Cf = H;
                    this._transform = a.Matrix.identity();
                    "" === k && (k = "repeat");
                    switch (k) {
                        case "repeat-x":
                            this.Be = a.TileMode.Repeat;
                            this.Ce = a.TileMode.Decal;
                            break;
                        case "repeat-y":
                            this.Be = a.TileMode.Decal;
                            this.Ce = a.TileMode.Repeat;
                            break;
                        case "repeat":
                            this.Ce = this.Be = a.TileMode.Repeat;
                            break;
                        case "no-repeat":
                            this.Ce = this.Be = a.TileMode.Decal;
                            break;
                        default:
                            throw "invalid repetition mode " + k;
                    }
                    this.setTransform = function (p) {
                        p = [p.a, p.c, p.e, p.b, p.d, p.f, 0, 0, 1];
                        d(p) && (this._transform = p);
                    };
                    this.ue = function () {
                        var p = new ia();
                        p.Be = this.Be;
                        p.Ce = this.Ce;
                        return p;
                    };
                    this.ne = function () {
                        this.Zd && (this.Zd.delete(), (this.Zd = null));
                    };
                    this.ve = function () {
                        this.ne();
                        return (this.Zd = this.Cf.makeShaderCubic(
                            this.Be,
                            this.Ce,
                            1 / 3,
                            1 / 3,
                            this._transform,
                        ));
                    };
                }
                function la(H, k, p, A, C, E) {
                    this.Zd = null;
                    this.ee = [];
                    this.be = [];
                    this.addColorStop = function (G, M) {
                        if (0 > G || 1 < G || !isFinite(G))
                            throw "offset must be between 0 and 1 inclusively";
                        M = g(M);
                        var ca = this.be.indexOf(G);
                        if (-1 !== ca) this.ee[ca] = M;
                        else {
                            for (
                                ca = 0;
                                ca < this.be.length && !(this.be[ca] > G);
                                ca++
                            );
                            this.be.splice(ca, 0, G);
                            this.ee.splice(ca, 0, M);
                        }
                    };
                    this.ue = function () {
                        var G = new la(H, k, p, A, C, E);
                        G.ee = this.ee.slice();
                        G.be = this.be.slice();
                        return G;
                    };
                    this.ne = function () {
                        this.Zd && (this.Zd.delete(), (this.Zd = null));
                    };
                    this.ve = function (G) {
                        var M = [H, k, A, C];
                        a.Matrix.mapPoints(G, M);
                        var ca = M[0],
                            da = M[1],
                            Ia = M[2];
                        M = M[3];
                        var bb = (Math.abs(G[0]) + Math.abs(G[4])) / 2;
                        G = p * bb;
                        bb *= E;
                        this.ne();
                        return (this.Zd = a.Shader.MakeTwoPointConicalGradient(
                            [ca, da],
                            G,
                            [Ia, M],
                            bb,
                            this.ee,
                            this.be,
                            a.TileMode.Clamp,
                        ));
                    };
                }
                a._testing = {};
                var qa = {
                    aliceblue: Float32Array.of(0.941, 0.973, 1, 1),
                    antiquewhite: Float32Array.of(0.98, 0.922, 0.843, 1),
                    aqua: Float32Array.of(0, 1, 1, 1),
                    aquamarine: Float32Array.of(0.498, 1, 0.831, 1),
                    azure: Float32Array.of(0.941, 1, 1, 1),
                    beige: Float32Array.of(0.961, 0.961, 0.863, 1),
                    bisque: Float32Array.of(1, 0.894, 0.769, 1),
                    black: Float32Array.of(0, 0, 0, 1),
                    blanchedalmond: Float32Array.of(1, 0.922, 0.804, 1),
                    blue: Float32Array.of(0, 0, 1, 1),
                    blueviolet: Float32Array.of(0.541, 0.169, 0.886, 1),
                    brown: Float32Array.of(0.647, 0.165, 0.165, 1),
                    burlywood: Float32Array.of(0.871, 0.722, 0.529, 1),
                    cadetblue: Float32Array.of(0.373, 0.62, 0.627, 1),
                    chartreuse: Float32Array.of(0.498, 1, 0, 1),
                    chocolate: Float32Array.of(0.824, 0.412, 0.118, 1),
                    coral: Float32Array.of(1, 0.498, 0.314, 1),
                    cornflowerblue: Float32Array.of(0.392, 0.584, 0.929, 1),
                    cornsilk: Float32Array.of(1, 0.973, 0.863, 1),
                    crimson: Float32Array.of(0.863, 0.078, 0.235, 1),
                    cyan: Float32Array.of(0, 1, 1, 1),
                    darkblue: Float32Array.of(0, 0, 0.545, 1),
                    darkcyan: Float32Array.of(0, 0.545, 0.545, 1),
                    darkgoldenrod: Float32Array.of(0.722, 0.525, 0.043, 1),
                    darkgray: Float32Array.of(0.663, 0.663, 0.663, 1),
                    darkgreen: Float32Array.of(0, 0.392, 0, 1),
                    darkgrey: Float32Array.of(0.663, 0.663, 0.663, 1),
                    darkkhaki: Float32Array.of(0.741, 0.718, 0.42, 1),
                    darkmagenta: Float32Array.of(0.545, 0, 0.545, 1),
                    darkolivegreen: Float32Array.of(0.333, 0.42, 0.184, 1),
                    darkorange: Float32Array.of(1, 0.549, 0, 1),
                    darkorchid: Float32Array.of(0.6, 0.196, 0.8, 1),
                    darkred: Float32Array.of(0.545, 0, 0, 1),
                    darksalmon: Float32Array.of(0.914, 0.588, 0.478, 1),
                    darkseagreen: Float32Array.of(0.561, 0.737, 0.561, 1),
                    darkslateblue: Float32Array.of(0.282, 0.239, 0.545, 1),
                    darkslategray: Float32Array.of(0.184, 0.31, 0.31, 1),
                    darkslategrey: Float32Array.of(0.184, 0.31, 0.31, 1),
                    darkturquoise: Float32Array.of(0, 0.808, 0.82, 1),
                    darkviolet: Float32Array.of(0.58, 0, 0.827, 1),
                    deeppink: Float32Array.of(1, 0.078, 0.576, 1),
                    deepskyblue: Float32Array.of(0, 0.749, 1, 1),
                    dimgray: Float32Array.of(0.412, 0.412, 0.412, 1),
                    dimgrey: Float32Array.of(0.412, 0.412, 0.412, 1),
                    dodgerblue: Float32Array.of(0.118, 0.565, 1, 1),
                    firebrick: Float32Array.of(0.698, 0.133, 0.133, 1),
                    floralwhite: Float32Array.of(1, 0.98, 0.941, 1),
                    forestgreen: Float32Array.of(0.133, 0.545, 0.133, 1),
                    fuchsia: Float32Array.of(1, 0, 1, 1),
                    gainsboro: Float32Array.of(0.863, 0.863, 0.863, 1),
                    ghostwhite: Float32Array.of(0.973, 0.973, 1, 1),
                    gold: Float32Array.of(1, 0.843, 0, 1),
                    goldenrod: Float32Array.of(0.855, 0.647, 0.125, 1),
                    gray: Float32Array.of(0.502, 0.502, 0.502, 1),
                    green: Float32Array.of(0, 0.502, 0, 1),
                    greenyellow: Float32Array.of(0.678, 1, 0.184, 1),
                    grey: Float32Array.of(0.502, 0.502, 0.502, 1),
                    honeydew: Float32Array.of(0.941, 1, 0.941, 1),
                    hotpink: Float32Array.of(1, 0.412, 0.706, 1),
                    indianred: Float32Array.of(0.804, 0.361, 0.361, 1),
                    indigo: Float32Array.of(0.294, 0, 0.51, 1),
                    ivory: Float32Array.of(1, 1, 0.941, 1),
                    khaki: Float32Array.of(0.941, 0.902, 0.549, 1),
                    lavender: Float32Array.of(0.902, 0.902, 0.98, 1),
                    lavenderblush: Float32Array.of(1, 0.941, 0.961, 1),
                    lawngreen: Float32Array.of(0.486, 0.988, 0, 1),
                    lemonchiffon: Float32Array.of(1, 0.98, 0.804, 1),
                    lightblue: Float32Array.of(0.678, 0.847, 0.902, 1),
                    lightcoral: Float32Array.of(0.941, 0.502, 0.502, 1),
                    lightcyan: Float32Array.of(0.878, 1, 1, 1),
                    lightgoldenrodyellow: Float32Array.of(0.98, 0.98, 0.824, 1),
                    lightgray: Float32Array.of(0.827, 0.827, 0.827, 1),
                    lightgreen: Float32Array.of(0.565, 0.933, 0.565, 1),
                    lightgrey: Float32Array.of(0.827, 0.827, 0.827, 1),
                    lightpink: Float32Array.of(1, 0.714, 0.757, 1),
                    lightsalmon: Float32Array.of(1, 0.627, 0.478, 1),
                    lightseagreen: Float32Array.of(0.125, 0.698, 0.667, 1),
                    lightskyblue: Float32Array.of(0.529, 0.808, 0.98, 1),
                    lightslategray: Float32Array.of(0.467, 0.533, 0.6, 1),
                    lightslategrey: Float32Array.of(0.467, 0.533, 0.6, 1),
                    lightsteelblue: Float32Array.of(0.69, 0.769, 0.871, 1),
                    lightyellow: Float32Array.of(1, 1, 0.878, 1),
                    lime: Float32Array.of(0, 1, 0, 1),
                    limegreen: Float32Array.of(0.196, 0.804, 0.196, 1),
                    linen: Float32Array.of(0.98, 0.941, 0.902, 1),
                    magenta: Float32Array.of(1, 0, 1, 1),
                    maroon: Float32Array.of(0.502, 0, 0, 1),
                    mediumaquamarine: Float32Array.of(0.4, 0.804, 0.667, 1),
                    mediumblue: Float32Array.of(0, 0, 0.804, 1),
                    mediumorchid: Float32Array.of(0.729, 0.333, 0.827, 1),
                    mediumpurple: Float32Array.of(0.576, 0.439, 0.859, 1),
                    mediumseagreen: Float32Array.of(0.235, 0.702, 0.443, 1),
                    mediumslateblue: Float32Array.of(0.482, 0.408, 0.933, 1),
                    mediumspringgreen: Float32Array.of(0, 0.98, 0.604, 1),
                    mediumturquoise: Float32Array.of(0.282, 0.82, 0.8, 1),
                    mediumvioletred: Float32Array.of(0.78, 0.082, 0.522, 1),
                    midnightblue: Float32Array.of(0.098, 0.098, 0.439, 1),
                    mintcream: Float32Array.of(0.961, 1, 0.98, 1),
                    mistyrose: Float32Array.of(1, 0.894, 0.882, 1),
                    moccasin: Float32Array.of(1, 0.894, 0.71, 1),
                    navajowhite: Float32Array.of(1, 0.871, 0.678, 1),
                    navy: Float32Array.of(0, 0, 0.502, 1),
                    oldlace: Float32Array.of(0.992, 0.961, 0.902, 1),
                    olive: Float32Array.of(0.502, 0.502, 0, 1),
                    olivedrab: Float32Array.of(0.42, 0.557, 0.137, 1),
                    orange: Float32Array.of(1, 0.647, 0, 1),
                    orangered: Float32Array.of(1, 0.271, 0, 1),
                    orchid: Float32Array.of(0.855, 0.439, 0.839, 1),
                    palegoldenrod: Float32Array.of(0.933, 0.91, 0.667, 1),
                    palegreen: Float32Array.of(0.596, 0.984, 0.596, 1),
                    paleturquoise: Float32Array.of(0.686, 0.933, 0.933, 1),
                    palevioletred: Float32Array.of(0.859, 0.439, 0.576, 1),
                    papayawhip: Float32Array.of(1, 0.937, 0.835, 1),
                    peachpuff: Float32Array.of(1, 0.855, 0.725, 1),
                    peru: Float32Array.of(0.804, 0.522, 0.247, 1),
                    pink: Float32Array.of(1, 0.753, 0.796, 1),
                    plum: Float32Array.of(0.867, 0.627, 0.867, 1),
                    powderblue: Float32Array.of(0.69, 0.878, 0.902, 1),
                    purple: Float32Array.of(0.502, 0, 0.502, 1),
                    rebeccapurple: Float32Array.of(0.4, 0.2, 0.6, 1),
                    red: Float32Array.of(1, 0, 0, 1),
                    rosybrown: Float32Array.of(0.737, 0.561, 0.561, 1),
                    royalblue: Float32Array.of(0.255, 0.412, 0.882, 1),
                    saddlebrown: Float32Array.of(0.545, 0.271, 0.075, 1),
                    salmon: Float32Array.of(0.98, 0.502, 0.447, 1),
                    sandybrown: Float32Array.of(0.957, 0.643, 0.376, 1),
                    seagreen: Float32Array.of(0.18, 0.545, 0.341, 1),
                    seashell: Float32Array.of(1, 0.961, 0.933, 1),
                    sienna: Float32Array.of(0.627, 0.322, 0.176, 1),
                    silver: Float32Array.of(0.753, 0.753, 0.753, 1),
                    skyblue: Float32Array.of(0.529, 0.808, 0.922, 1),
                    slateblue: Float32Array.of(0.416, 0.353, 0.804, 1),
                    slategray: Float32Array.of(0.439, 0.502, 0.565, 1),
                    slategrey: Float32Array.of(0.439, 0.502, 0.565, 1),
                    snow: Float32Array.of(1, 0.98, 0.98, 1),
                    springgreen: Float32Array.of(0, 1, 0.498, 1),
                    steelblue: Float32Array.of(0.275, 0.51, 0.706, 1),
                    tan: Float32Array.of(0.824, 0.706, 0.549, 1),
                    teal: Float32Array.of(0, 0.502, 0.502, 1),
                    thistle: Float32Array.of(0.847, 0.749, 0.847, 1),
                    tomato: Float32Array.of(1, 0.388, 0.278, 1),
                    transparent: Float32Array.of(0, 0, 0, 0),
                    turquoise: Float32Array.of(0.251, 0.878, 0.816, 1),
                    violet: Float32Array.of(0.933, 0.51, 0.933, 1),
                    wheat: Float32Array.of(0.961, 0.871, 0.702, 1),
                    white: Float32Array.of(1, 1, 1, 1),
                    whitesmoke: Float32Array.of(0.961, 0.961, 0.961, 1),
                    yellow: Float32Array.of(1, 1, 0, 1),
                    yellowgreen: Float32Array.of(0.604, 0.804, 0.196, 1),
                };
                a._testing.parseColor = g;
                a._testing.colorToString = c;
                var ta = RegExp(
                        "(italic|oblique|normal|)\\s*(small-caps|normal|)\\s*(bold|bolder|lighter|[1-9]00|normal|)\\s*([\\d\\.]+)(px|pt|pc|in|cm|mm|%|em|ex|ch|rem|q)(.+)",
                    ),
                    ba = {
                        "Noto Mono": { "*": null },
                        monospace: { "*": null },
                    };
                a._testing.parseFontString = l;
                a.MakeCanvas = function (H, k) {
                    return (H = a.MakeSurface(H, k)) ? new y(H) : null;
                };
                a.ImageData = function () {
                    if (2 === arguments.length) {
                        var H = arguments[0],
                            k = arguments[1];
                        return new J(new Uint8ClampedArray(4 * H * k), H, k);
                    }
                    if (3 === arguments.length) {
                        var p = arguments[0];
                        if (p.prototype.constructor !== Uint8ClampedArray)
                            throw "bytes must be given as a Uint8ClampedArray";
                        H = arguments[1];
                        k = arguments[2];
                        if (p % 4)
                            throw "bytes must be given in a multiple of 4";
                        if (p % H) throw "bytes must divide evenly by width";
                        if (k && k !== p / (4 * H))
                            throw "invalid height given";
                        return new J(p, H, p / (4 * H));
                    }
                    throw (
                        "invalid number of arguments - takes 2 or 3, saw " +
                        arguments.length
                    );
                };
            })();
        })(w);
        var ua = Object.assign({}, w),
            va = "./this.program",
            wa = (a, b) => {
                throw b;
            },
            xa = "object" == typeof window,
            za = "function" == typeof importScripts,
            Aa =
                "object" == typeof process &&
                "object" == typeof process.versions &&
                "string" == typeof process.versions.node,
            Ba = "",
            Ca,
            Da,
            Ea,
            fs,
            Fa,
            Ga;
        if (Aa)
            (Ba = za ? require("path").dirname(Ba) + "/" : __dirname + "/"),
                (Ga = () => {
                    Fa || ((fs = require("fs")), (Fa = require("path")));
                }),
                (Ca = function (a, b) {
                    Ga();
                    a = Fa.normalize(a);
                    return fs.readFileSync(a, b ? void 0 : "utf8");
                }),
                (Ea = (a) => {
                    a = Ca(a, !0);
                    a.buffer || (a = new Uint8Array(a));
                    return a;
                }),
                (Da = (a, b, e) => {
                    Ga();
                    a = Fa.normalize(a);
                    fs.readFile(a, function (f, h) {
                        f ? e(f) : b(h.buffer);
                    });
                }),
                1 < process.argv.length &&
                    (va = process.argv[1].replace(/\\/g, "/")),
                process.argv.slice(2),
                process.on("unhandledRejection", function (a) {
                    throw a;
                }),
                (wa = (a, b) => {
                    if (noExitRuntime) throw ((process.exitCode = a), b);
                    b instanceof Ha || Ja("exiting due to exception: " + b);
                    process.exit(a);
                }),
                (w.inspect = function () {
                    return "[Emscripten Module object]";
                });
        else if (xa || za)
            za
                ? (Ba = self.location.href)
                : "undefined" != typeof document &&
                  document.currentScript &&
                  (Ba = document.currentScript.src),
                _scriptDir && (Ba = _scriptDir),
                0 !== Ba.indexOf("blob:")
                    ? (Ba = Ba.substr(
                          0,
                          Ba.replace(/[?#].*/, "").lastIndexOf("/") + 1,
                      ))
                    : (Ba = ""),
                (Ca = (a) => {
                    var b = new XMLHttpRequest();
                    b.open("GET", a, !1);
                    b.send(null);
                    return b.responseText;
                }),
                za &&
                    (Ea = (a) => {
                        var b = new XMLHttpRequest();
                        b.open("GET", a, !1);
                        b.responseType = "arraybuffer";
                        b.send(null);
                        return new Uint8Array(b.response);
                    }),
                (Da = (a, b, e) => {
                    var f = new XMLHttpRequest();
                    f.open("GET", a, !0);
                    f.responseType = "arraybuffer";
                    f.onload = () => {
                        200 == f.status || (0 == f.status && f.response)
                            ? b(f.response)
                            : e();
                    };
                    f.onerror = e;
                    f.send(null);
                });
        var Ka = w.print || console.log.bind(console),
            Ja = w.printErr || console.warn.bind(console);
        Object.assign(w, ua);
        ua = null;
        w.thisProgram && (va = w.thisProgram);
        w.quit && (wa = w.quit);
        var La = 0,
            Ma;
        w.wasmBinary && (Ma = w.wasmBinary);
        var noExitRuntime = w.noExitRuntime || !0;
        "object" != typeof WebAssembly && Na("no native wasm support detected");
        var Qa,
            Ra = !1,
            Sa =
                "undefined" != typeof TextDecoder
                    ? new TextDecoder("utf8")
                    : void 0;
        function Ta(a, b, e) {
            var f = b + e;
            for (e = b; a[e] && !(e >= f); ) ++e;
            if (16 < e - b && a.buffer && Sa)
                return Sa.decode(a.subarray(b, e));
            for (f = ""; b < e; ) {
                var h = a[b++];
                if (h & 128) {
                    var m = a[b++] & 63;
                    if (192 == (h & 224))
                        f += String.fromCharCode(((h & 31) << 6) | m);
                    else {
                        var u = a[b++] & 63;
                        h =
                            224 == (h & 240)
                                ? ((h & 15) << 12) | (m << 6) | u
                                : ((h & 7) << 18) |
                                  (m << 12) |
                                  (u << 6) |
                                  (a[b++] & 63);
                        65536 > h
                            ? (f += String.fromCharCode(h))
                            : ((h -= 65536),
                              (f += String.fromCharCode(
                                  55296 | (h >> 10),
                                  56320 | (h & 1023),
                              )));
                    }
                } else f += String.fromCharCode(h);
            }
            return f;
        }
        function Va(a, b) {
            return a ? Ta(K, a, b) : "";
        }
        function pa(a, b, e, f) {
            if (!(0 < f)) return 0;
            var h = e;
            f = e + f - 1;
            for (var m = 0; m < a.length; ++m) {
                var u = a.charCodeAt(m);
                if (55296 <= u && 57343 >= u) {
                    var n = a.charCodeAt(++m);
                    u = (65536 + ((u & 1023) << 10)) | (n & 1023);
                }
                if (127 >= u) {
                    if (e >= f) break;
                    b[e++] = u;
                } else {
                    if (2047 >= u) {
                        if (e + 1 >= f) break;
                        b[e++] = 192 | (u >> 6);
                    } else {
                        if (65535 >= u) {
                            if (e + 2 >= f) break;
                            b[e++] = 224 | (u >> 12);
                        } else {
                            if (e + 3 >= f) break;
                            b[e++] = 240 | (u >> 18);
                            b[e++] = 128 | ((u >> 12) & 63);
                        }
                        b[e++] = 128 | ((u >> 6) & 63);
                    }
                    b[e++] = 128 | (u & 63);
                }
            }
            b[e] = 0;
            return e - h;
        }
        function oa(a) {
            for (var b = 0, e = 0; e < a.length; ++e) {
                var f = a.charCodeAt(e);
                55296 <= f &&
                    57343 >= f &&
                    (f =
                        (65536 + ((f & 1023) << 10)) |
                        (a.charCodeAt(++e) & 1023));
                127 >= f
                    ? ++b
                    : (b = 2047 >= f ? b + 2 : 65535 >= f ? b + 3 : b + 4);
            }
            return b;
        }
        var Wa =
            "undefined" != typeof TextDecoder
                ? new TextDecoder("utf-16le")
                : void 0;
        function Xa(a, b) {
            var e = a >> 1;
            for (var f = e + b / 2; !(e >= f) && Ya[e]; ) ++e;
            e <<= 1;
            if (32 < e - a && Wa) return Wa.decode(K.subarray(a, e));
            e = "";
            for (f = 0; !(f >= b / 2); ++f) {
                var h = Za[(a + 2 * f) >> 1];
                if (0 == h) break;
                e += String.fromCharCode(h);
            }
            return e;
        }
        function $a(a, b, e) {
            void 0 === e && (e = 2147483647);
            if (2 > e) return 0;
            e -= 2;
            var f = b;
            e = e < 2 * a.length ? e / 2 : a.length;
            for (var h = 0; h < e; ++h)
                (Za[b >> 1] = a.charCodeAt(h)), (b += 2);
            Za[b >> 1] = 0;
            return b - f;
        }
        function cb(a) {
            return 2 * a.length;
        }
        function db(a, b) {
            for (var e = 0, f = ""; !(e >= b / 4); ) {
                var h = R[(a + 4 * e) >> 2];
                if (0 == h) break;
                ++e;
                65536 <= h
                    ? ((h -= 65536),
                      (f += String.fromCharCode(
                          55296 | (h >> 10),
                          56320 | (h & 1023),
                      )))
                    : (f += String.fromCharCode(h));
            }
            return f;
        }
        function eb(a, b, e) {
            void 0 === e && (e = 2147483647);
            if (4 > e) return 0;
            var f = b;
            e = f + e - 4;
            for (var h = 0; h < a.length; ++h) {
                var m = a.charCodeAt(h);
                if (55296 <= m && 57343 >= m) {
                    var u = a.charCodeAt(++h);
                    m = (65536 + ((m & 1023) << 10)) | (u & 1023);
                }
                R[b >> 2] = m;
                b += 4;
                if (b + 4 > e) break;
            }
            R[b >> 2] = 0;
            return b - f;
        }
        function fb(a) {
            for (var b = 0, e = 0; e < a.length; ++e) {
                var f = a.charCodeAt(e);
                55296 <= f && 57343 >= f && ++e;
                b += 4;
            }
            return b;
        }
        var jb, kb, K, Za, Ya, R, lb, T, mb;
        function nb() {
            var a = Qa.buffer;
            jb = a;
            w.HEAP8 = kb = new Int8Array(a);
            w.HEAP16 = Za = new Int16Array(a);
            w.HEAP32 = R = new Int32Array(a);
            w.HEAPU8 = K = new Uint8Array(a);
            w.HEAPU16 = Ya = new Uint16Array(a);
            w.HEAPU32 = lb = new Uint32Array(a);
            w.HEAPF32 = T = new Float32Array(a);
            w.HEAPF64 = mb = new Float64Array(a);
        }
        var ob,
            pb = [],
            qb = [],
            rb = [];
        function sb() {
            var a = w.preRun.shift();
            pb.unshift(a);
        }
        var tb = 0,
            ub = null,
            vb = null;
        function Na(a) {
            if (w.onAbort) w.onAbort(a);
            a = "Aborted(" + a + ")";
            Ja(a);
            Ra = !0;
            a = new WebAssembly.RuntimeError(
                a + ". Build with -sASSERTIONS for more info.",
            );
            ea(a);
            throw a;
        }
        function wb() {
            return yb.startsWith("data:application/octet-stream;base64,");
        }
        var yb;
        yb = "canvaskit.wasm";
        if (!wb()) {
            var zb = yb;
            yb = w.locateFile ? w.locateFile(zb, Ba) : Ba + zb;
        }
        function Ab() {
            var a = yb;
            try {
                if (a == yb && Ma) return new Uint8Array(Ma);
                if (Ea) return Ea(a);
                throw "both async and sync fetching of the wasm failed";
            } catch (b) {
                Na(b);
            }
        }
        function Bb() {
            if (!Ma && (xa || za)) {
                if ("function" == typeof fetch && !yb.startsWith("file://"))
                    return fetch(yb, { credentials: "same-origin" })
                        .then(function (a) {
                            if (!a.ok)
                                throw (
                                    "failed to load wasm binary file at '" +
                                    yb +
                                    "'"
                                );
                            return a.arrayBuffer();
                        })
                        .catch(function () {
                            return Ab();
                        });
                if (Da)
                    return new Promise(function (a, b) {
                        Da(
                            yb,
                            function (e) {
                                a(new Uint8Array(e));
                            },
                            b,
                        );
                    });
            }
            return Promise.resolve().then(function () {
                return Ab();
            });
        }
        function Cb(a) {
            for (; 0 < a.length; ) a.shift()(w);
        }
        function Db(a) {
            return ob.get(a);
        }
        var Eb = {};
        function Fb(a) {
            for (; a.length; ) {
                var b = a.pop();
                a.pop()(b);
            }
        }
        function Gb(a) {
            return this.fromWireType(R[a >> 2]);
        }
        var Hb = {},
            Ib = {},
            Jb = {};
        function Kb(a) {
            if (void 0 === a) return "_unknown";
            a = a.replace(/[^a-zA-Z0-9_]/g, "$");
            var b = a.charCodeAt(0);
            return 48 <= b && 57 >= b ? "_" + a : a;
        }
        function Lb(a, b) {
            a = Kb(a);
            return function () {
                null;
                return b.apply(this, arguments);
            };
        }
        function Mb(a) {
            var b = Error,
                e = Lb(a, function (f) {
                    this.name = a;
                    this.message = f;
                    f = Error(f).stack;
                    void 0 !== f &&
                        (this.stack =
                            this.toString() +
                            "\n" +
                            f.replace(/^Error(:[^\n]*)?\n/, ""));
                });
            e.prototype = Object.create(b.prototype);
            e.prototype.constructor = e;
            e.prototype.toString = function () {
                return void 0 === this.message
                    ? this.name
                    : this.name + ": " + this.message;
            };
            return e;
        }
        var Nb = void 0;
        function Ob(a) {
            throw new Nb(a);
        }
        function Pb(a, b, e) {
            function f(n) {
                n = e(n);
                n.length !== a.length && Ob("Mismatched type converter count");
                for (var r = 0; r < a.length; ++r) Qb(a[r], n[r]);
            }
            a.forEach(function (n) {
                Jb[n] = b;
            });
            var h = Array(b.length),
                m = [],
                u = 0;
            b.forEach((n, r) => {
                Ib.hasOwnProperty(n)
                    ? (h[r] = Ib[n])
                    : (m.push(n),
                      Hb.hasOwnProperty(n) || (Hb[n] = []),
                      Hb[n].push(() => {
                          h[r] = Ib[n];
                          ++u;
                          u === m.length && f(h);
                      }));
            });
            0 === m.length && f(h);
        }
        function Rb(a) {
            switch (a) {
                case 1:
                    return 0;
                case 2:
                    return 1;
                case 4:
                    return 2;
                case 8:
                    return 3;
                default:
                    throw new TypeError("Unknown type size: " + a);
            }
        }
        var Sb = void 0;
        function Tb(a) {
            for (var b = ""; K[a]; ) b += Sb[K[a++]];
            return b;
        }
        var ac = void 0;
        function X(a) {
            throw new ac(a);
        }
        function Qb(a, b, e = {}) {
            if (!("argPackAdvance" in b))
                throw new TypeError(
                    "registerType registeredInstance requires argPackAdvance",
                );
            var f = b.name;
            a ||
                X(
                    'type "' +
                        f +
                        '" must have a positive integer typeid pointer',
                );
            if (Ib.hasOwnProperty(a)) {
                if (e.Wf) return;
                X("Cannot register type '" + f + "' twice");
            }
            Ib[a] = b;
            delete Jb[a];
            Hb.hasOwnProperty(a) &&
                ((b = Hb[a]), delete Hb[a], b.forEach((h) => h()));
        }
        function bc(a) {
            X(a.Md.Yd.Td.name + " instance already deleted");
        }
        var cc = !1;
        function dc() {}
        function ec(a) {
            --a.count.value;
            0 === a.count.value && (a.ae ? a.ge.me(a.ae) : a.Yd.Td.me(a.Vd));
        }
        function fc(a, b, e) {
            if (b === e) return a;
            if (void 0 === e.je) return null;
            a = fc(a, b, e.je);
            return null === a ? null : e.Kf(a);
        }
        var gc = {},
            hc = [];
        function ic() {
            for (; hc.length; ) {
                var a = hc.pop();
                a.Md.Ge = !1;
                a["delete"]();
            }
        }
        var jc = void 0,
            kc = {};
        function lc(a, b) {
            for (void 0 === b && X("ptr should not be undefined"); a.je; )
                (b = a.Pe(b)), (a = a.je);
            return kc[b];
        }
        function mc(a, b) {
            (b.Yd && b.Vd) || Ob("makeClassHandle requires ptr and ptrType");
            !!b.ge !== !!b.ae &&
                Ob("Both smartPtrType and smartPtr must be specified");
            b.count = { value: 1 };
            return nc(Object.create(a, { Md: { value: b } }));
        }
        function nc(a) {
            if ("undefined" === typeof FinalizationRegistry)
                return (nc = (b) => b), a;
            cc = new FinalizationRegistry((b) => {
                ec(b.Md);
            });
            nc = (b) => {
                var e = b.Md;
                e.ae && cc.register(b, { Md: e }, b);
                return b;
            };
            dc = (b) => {
                cc.unregister(b);
            };
            return nc(a);
        }
        function oc() {}
        function pc(a, b, e) {
            if (void 0 === a[b].$d) {
                var f = a[b];
                a[b] = function () {
                    a[b].$d.hasOwnProperty(arguments.length) ||
                        X(
                            "Function '" +
                                e +
                                "' called with an invalid number of arguments (" +
                                arguments.length +
                                ") - expects one of (" +
                                a[b].$d +
                                ")!",
                        );
                    return a[b].$d[arguments.length].apply(this, arguments);
                };
                a[b].$d = [];
                a[b].$d[f.Ee] = f;
            }
        }
        function qc(a, b, e) {
            w.hasOwnProperty(a)
                ? ((void 0 === e ||
                      (void 0 !== w[a].$d && void 0 !== w[a].$d[e])) &&
                      X("Cannot register public name '" + a + "' twice"),
                  pc(w, a, a),
                  w.hasOwnProperty(e) &&
                      X(
                          "Cannot register multiple overloads of a function with the same number of arguments (" +
                              e +
                              ")!",
                      ),
                  (w[a].$d[e] = b))
                : ((w[a] = b), void 0 !== e && (w[a].ug = e));
        }
        function rc(a, b, e, f, h, m, u, n) {
            this.name = a;
            this.constructor = b;
            this.He = e;
            this.me = f;
            this.je = h;
            this.Qf = m;
            this.Pe = u;
            this.Kf = n;
            this.cg = [];
        }
        function sc(a, b, e) {
            for (; b !== e; )
                b.Pe ||
                    X(
                        "Expected null or instance of " +
                            e.name +
                            ", got an instance of " +
                            b.name,
                    ),
                    (a = b.Pe(a)),
                    (b = b.je);
            return a;
        }
        function tc(a, b) {
            if (null === b)
                return this.gf && X("null is not a valid " + this.name), 0;
            b.Md || X('Cannot pass "' + uc(b) + '" as a ' + this.name);
            b.Md.Vd ||
                X(
                    "Cannot pass deleted object as a pointer of type " +
                        this.name,
                );
            return sc(b.Md.Vd, b.Md.Yd.Td, this.Td);
        }
        function vc(a, b) {
            if (null === b) {
                this.gf && X("null is not a valid " + this.name);
                if (this.Ue) {
                    var e = this.hf();
                    null !== a && a.push(this.me, e);
                    return e;
                }
                return 0;
            }
            b.Md || X('Cannot pass "' + uc(b) + '" as a ' + this.name);
            b.Md.Vd ||
                X(
                    "Cannot pass deleted object as a pointer of type " +
                        this.name,
                );
            !this.Te &&
                b.Md.Yd.Te &&
                X(
                    "Cannot convert argument of type " +
                        (b.Md.ge ? b.Md.ge.name : b.Md.Yd.name) +
                        " to parameter type " +
                        this.name,
                );
            e = sc(b.Md.Vd, b.Md.Yd.Td, this.Td);
            if (this.Ue)
                switch (
                    (void 0 === b.Md.ae &&
                        X("Passing raw pointer to smart pointer is illegal"),
                    this.ig)
                ) {
                    case 0:
                        b.Md.ge === this
                            ? (e = b.Md.ae)
                            : X(
                                  "Cannot convert argument of type " +
                                      (b.Md.ge ? b.Md.ge.name : b.Md.Yd.name) +
                                      " to parameter type " +
                                      this.name,
                              );
                        break;
                    case 1:
                        e = b.Md.ae;
                        break;
                    case 2:
                        if (b.Md.ge === this) e = b.Md.ae;
                        else {
                            var f = b.clone();
                            e = this.dg(
                                e,
                                wc(function () {
                                    f["delete"]();
                                }),
                            );
                            null !== a && a.push(this.me, e);
                        }
                        break;
                    default:
                        X("Unsupporting sharing policy");
                }
            return e;
        }
        function xc(a, b) {
            if (null === b)
                return this.gf && X("null is not a valid " + this.name), 0;
            b.Md || X('Cannot pass "' + uc(b) + '" as a ' + this.name);
            b.Md.Vd ||
                X(
                    "Cannot pass deleted object as a pointer of type " +
                        this.name,
                );
            b.Md.Yd.Te &&
                X(
                    "Cannot convert argument of type " +
                        b.Md.Yd.name +
                        " to parameter type " +
                        this.name,
                );
            return sc(b.Md.Vd, b.Md.Yd.Td, this.Td);
        }
        function yc(a, b, e, f, h, m, u, n, r, v, D) {
            this.name = a;
            this.Td = b;
            this.gf = e;
            this.Te = f;
            this.Ue = h;
            this.bg = m;
            this.ig = u;
            this.wf = n;
            this.hf = r;
            this.dg = v;
            this.me = D;
            h || void 0 !== b.je
                ? (this.toWireType = vc)
                : ((this.toWireType = f ? tc : xc), (this.fe = null));
        }
        function zc(a, b, e) {
            w.hasOwnProperty(a) || Ob("Replacing nonexistant public symbol");
            void 0 !== w[a].$d && void 0 !== e
                ? (w[a].$d[e] = b)
                : ((w[a] = b), (w[a].Ee = e));
        }
        function Ac(a, b) {
            var e = [];
            return function () {
                e.length = 0;
                Object.assign(e, arguments);
                if (a.includes("j")) {
                    var f = w["dynCall_" + a];
                    f =
                        e && e.length
                            ? f.apply(null, [b].concat(e))
                            : f.call(null, b);
                } else f = Db(b).apply(null, e);
                return f;
            };
        }
        function Bc(a, b) {
            a = Tb(a);
            var e = a.includes("j") ? Ac(a, b) : Db(b);
            "function" != typeof e &&
                X("unknown function pointer with signature " + a + ": " + b);
            return e;
        }
        var Ic = void 0;
        function Jc(a) {
            a = Kc(a);
            var b = Tb(a);
            Lc(a);
            return b;
        }
        function Mc(a, b) {
            function e(m) {
                h[m] ||
                    Ib[m] ||
                    (Jb[m] ? Jb[m].forEach(e) : (f.push(m), (h[m] = !0)));
            }
            var f = [],
                h = {};
            b.forEach(e);
            throw new Ic(a + ": " + f.map(Jc).join([", "]));
        }
        function Nc(a, b, e, f, h) {
            var m = b.length;
            2 > m &&
                X(
                    "argTypes array size mismatch! Must at least get return value and 'this' types!",
                );
            var u = null !== b[1] && null !== e,
                n = !1;
            for (e = 1; e < b.length; ++e)
                if (null !== b[e] && void 0 === b[e].fe) {
                    n = !0;
                    break;
                }
            var r = "void" !== b[0].name,
                v = m - 2,
                D = Array(v),
                I = [],
                O = [];
            return function () {
                arguments.length !== v &&
                    X(
                        "function " +
                            a +
                            " called with " +
                            arguments.length +
                            " arguments, expected " +
                            v +
                            " args!",
                    );
                O.length = 0;
                I.length = u ? 2 : 1;
                I[0] = h;
                if (u) {
                    var z = b[1].toWireType(O, this);
                    I[1] = z;
                }
                for (var P = 0; P < v; ++P)
                    (D[P] = b[P + 2].toWireType(O, arguments[P])), I.push(D[P]);
                P = f.apply(null, I);
                if (n) Fb(O);
                else
                    for (var V = u ? 1 : 2; V < b.length; V++) {
                        var S = 1 === V ? z : D[V - 2];
                        null !== b[V].fe && b[V].fe(S);
                    }
                z = r ? b[0].fromWireType(P) : void 0;
                return z;
            };
        }
        function Oc(a, b) {
            for (var e = [], f = 0; f < a; f++) e.push(lb[(b + 4 * f) >> 2]);
            return e;
        }
        var Pc = [],
            Qc = [
                {},
                { value: void 0 },
                { value: null },
                { value: !0 },
                { value: !1 },
            ];
        function Rc(a) {
            4 < a && 0 === --Qc[a].jf && ((Qc[a] = void 0), Pc.push(a));
        }
        var Sc = (a) => {
                a || X("Cannot use deleted val. handle = " + a);
                return Qc[a].value;
            },
            wc = (a) => {
                switch (a) {
                    case void 0:
                        return 1;
                    case null:
                        return 2;
                    case !0:
                        return 3;
                    case !1:
                        return 4;
                    default:
                        var b = Pc.length ? Pc.pop() : Qc.length;
                        Qc[b] = { jf: 1, value: a };
                        return b;
                }
            };
        function Tc(a, b, e) {
            switch (b) {
                case 0:
                    return function (f) {
                        return this.fromWireType((e ? kb : K)[f]);
                    };
                case 1:
                    return function (f) {
                        return this.fromWireType((e ? Za : Ya)[f >> 1]);
                    };
                case 2:
                    return function (f) {
                        return this.fromWireType((e ? R : lb)[f >> 2]);
                    };
                default:
                    throw new TypeError("Unknown integer type: " + a);
            }
        }
        function Uc(a, b) {
            var e = Ib[a];
            void 0 === e && X(b + " has unknown type " + Jc(a));
            return e;
        }
        function uc(a) {
            if (null === a) return "null";
            var b = typeof a;
            return "object" === b || "array" === b || "function" === b
                ? a.toString()
                : "" + a;
        }
        function Vc(a, b) {
            switch (b) {
                case 2:
                    return function (e) {
                        return this.fromWireType(T[e >> 2]);
                    };
                case 3:
                    return function (e) {
                        return this.fromWireType(mb[e >> 3]);
                    };
                default:
                    throw new TypeError("Unknown float type: " + a);
            }
        }
        function Wc(a, b, e) {
            switch (b) {
                case 0:
                    return e
                        ? function (f) {
                              return kb[f];
                          }
                        : function (f) {
                              return K[f];
                          };
                case 1:
                    return e
                        ? function (f) {
                              return Za[f >> 1];
                          }
                        : function (f) {
                              return Ya[f >> 1];
                          };
                case 2:
                    return e
                        ? function (f) {
                              return R[f >> 2];
                          }
                        : function (f) {
                              return lb[f >> 2];
                          };
                default:
                    throw new TypeError("Unknown integer type: " + a);
            }
        }
        var Xc = {};
        function Yc(a) {
            var b = Xc[a];
            return void 0 === b ? Tb(a) : b;
        }
        var Zc = [];
        function $c() {
            function a(b) {
                b.$$$embind_global$$$ = b;
                var e =
                    "object" == typeof $$$embind_global$$$ &&
                    b.$$$embind_global$$$ == b;
                e || delete b.$$$embind_global$$$;
                return e;
            }
            if ("object" == typeof globalThis) return globalThis;
            if ("object" == typeof $$$embind_global$$$)
                return $$$embind_global$$$;
            "object" == typeof global && a(global)
                ? ($$$embind_global$$$ = global)
                : "object" == typeof self &&
                  a(self) &&
                  ($$$embind_global$$$ = self);
            if ("object" == typeof $$$embind_global$$$)
                return $$$embind_global$$$;
            throw Error("unable to get global object.");
        }
        function ad(a) {
            var b = Zc.length;
            Zc.push(a);
            return b;
        }
        function bd(a, b) {
            for (var e = Array(a), f = 0; f < a; ++f)
                e[f] = Uc(lb[(b + 4 * f) >> 2], "parameter " + f);
            return e;
        }
        var cd = [];
        function dd(a) {
            var b = Array(a + 1);
            return function (e, f, h) {
                b[0] = e;
                for (var m = 0; m < a; ++m) {
                    var u = Uc(lb[(f + 4 * m) >> 2], "parameter " + m);
                    b[m + 1] = u.readValueFromPointer(h);
                    h += u.argPackAdvance;
                }
                e = new (e.bind.apply(e, b))();
                return wc(e);
            };
        }
        var ed = {},
            fd;
        fd = Aa
            ? () => {
                  var a = process.hrtime();
                  return 1e3 * a[0] + a[1] / 1e6;
              }
            : () => performance.now();
        function gd(a) {
            var b = a.getExtension("ANGLE_instanced_arrays");
            b &&
                ((a.vertexAttribDivisor = function (e, f) {
                    b.vertexAttribDivisorANGLE(e, f);
                }),
                (a.drawArraysInstanced = function (e, f, h, m) {
                    b.drawArraysInstancedANGLE(e, f, h, m);
                }),
                (a.drawElementsInstanced = function (e, f, h, m, u) {
                    b.drawElementsInstancedANGLE(e, f, h, m, u);
                }));
        }
        function hd(a) {
            var b = a.getExtension("OES_vertex_array_object");
            b &&
                ((a.createVertexArray = function () {
                    return b.createVertexArrayOES();
                }),
                (a.deleteVertexArray = function (e) {
                    b.deleteVertexArrayOES(e);
                }),
                (a.bindVertexArray = function (e) {
                    b.bindVertexArrayOES(e);
                }),
                (a.isVertexArray = function (e) {
                    return b.isVertexArrayOES(e);
                }));
        }
        function jd(a) {
            var b = a.getExtension("WEBGL_draw_buffers");
            b &&
                (a.drawBuffers = function (e, f) {
                    b.drawBuffersWEBGL(e, f);
                });
        }
        var kd = 1,
            ld = [],
            md = [],
            nd = [],
            od = [],
            ja = [],
            pd = [],
            qd = [],
            na = [],
            rd = [],
            sd = [],
            td = {},
            ud = {},
            vd = 4;
        function wd(a) {
            xd || (xd = a);
        }
        function fa(a) {
            for (var b = kd++, e = a.length; e < b; e++) a[e] = null;
            return b;
        }
        function ka(a, b) {
            a.mf ||
                ((a.mf = a.getContext),
                (a.getContext = function (f, h) {
                    h = a.mf(f, h);
                    return ("webgl" == f) == h instanceof WebGLRenderingContext
                        ? h
                        : null;
                }));
            var e =
                1 < b.majorVersion
                    ? a.getContext("webgl2", b)
                    : a.getContext("webgl", b);
            return e ? Ad(e, b) : 0;
        }
        function Ad(a, b) {
            var e = fa(na),
                f = { Vf: e, attributes: b, version: b.majorVersion, he: a };
            a.canvas && (a.canvas.zf = f);
            na[e] = f;
            ("undefined" == typeof b.Lf || b.Lf) && Bd(f);
            return e;
        }
        function ma(a) {
            x = na[a];
            w.sg = Z = x && x.he;
            return !(a && !Z);
        }
        function Bd(a) {
            a || (a = x);
            if (!a.Xf) {
                a.Xf = !0;
                var b = a.he;
                gd(b);
                hd(b);
                jd(b);
                b.rf = b.getExtension(
                    "WEBGL_draw_instanced_base_vertex_base_instance",
                );
                b.vf = b.getExtension(
                    "WEBGL_multi_draw_instanced_base_vertex_base_instance",
                );
                2 <= a.version &&
                    (b.sf = b.getExtension("EXT_disjoint_timer_query_webgl2"));
                if (2 > a.version || !b.sf)
                    b.sf = b.getExtension("EXT_disjoint_timer_query");
                b.tg = b.getExtension("WEBGL_multi_draw");
                (b.getSupportedExtensions() || []).forEach(function (e) {
                    e.includes("lose_context") ||
                        e.includes("debug") ||
                        b.getExtension(e);
                });
            }
        }
        var x,
            xd,
            Cd = [];
        function Dd(a, b, e, f) {
            for (var h = 0; h < a; h++) {
                var m = Z[e](),
                    u = m && fa(f);
                m ? ((m.name = u), (f[u] = m)) : wd(1282);
                R[(b + 4 * h) >> 2] = u;
            }
        }
        function Ed(a, b, e) {
            if (b) {
                var f = void 0;
                switch (a) {
                    case 36346:
                        f = 1;
                        break;
                    case 36344:
                        0 != e && 1 != e && wd(1280);
                        return;
                    case 34814:
                    case 36345:
                        f = 0;
                        break;
                    case 34466:
                        var h = Z.getParameter(34467);
                        f = h ? h.length : 0;
                        break;
                    case 33309:
                        if (2 > x.version) {
                            wd(1282);
                            return;
                        }
                        f = 2 * (Z.getSupportedExtensions() || []).length;
                        break;
                    case 33307:
                    case 33308:
                        if (2 > x.version) {
                            wd(1280);
                            return;
                        }
                        f = 33307 == a ? 3 : 0;
                }
                if (void 0 === f)
                    switch (((h = Z.getParameter(a)), typeof h)) {
                        case "number":
                            f = h;
                            break;
                        case "boolean":
                            f = h ? 1 : 0;
                            break;
                        case "string":
                            wd(1280);
                            return;
                        case "object":
                            if (null === h)
                                switch (a) {
                                    case 34964:
                                    case 35725:
                                    case 34965:
                                    case 36006:
                                    case 36007:
                                    case 32873:
                                    case 34229:
                                    case 36662:
                                    case 36663:
                                    case 35053:
                                    case 35055:
                                    case 36010:
                                    case 35097:
                                    case 35869:
                                    case 32874:
                                    case 36389:
                                    case 35983:
                                    case 35368:
                                    case 34068:
                                        f = 0;
                                        break;
                                    default:
                                        wd(1280);
                                        return;
                                }
                            else {
                                if (
                                    h instanceof Float32Array ||
                                    h instanceof Uint32Array ||
                                    h instanceof Int32Array ||
                                    h instanceof Array
                                ) {
                                    for (a = 0; a < h.length; ++a)
                                        switch (e) {
                                            case 0:
                                                R[(b + 4 * a) >> 2] = h[a];
                                                break;
                                            case 2:
                                                T[(b + 4 * a) >> 2] = h[a];
                                                break;
                                            case 4:
                                                kb[(b + a) >> 0] = h[a] ? 1 : 0;
                                        }
                                    return;
                                }
                                try {
                                    f = h.name | 0;
                                } catch (m) {
                                    wd(1280);
                                    Ja(
                                        "GL_INVALID_ENUM in glGet" +
                                            e +
                                            "v: Unknown object returned from WebGL getParameter(" +
                                            a +
                                            ")! (error: " +
                                            m +
                                            ")",
                                    );
                                    return;
                                }
                            }
                            break;
                        default:
                            wd(1280);
                            Ja(
                                "GL_INVALID_ENUM in glGet" +
                                    e +
                                    "v: Native code calling glGet" +
                                    e +
                                    "v(" +
                                    a +
                                    ") and it returns " +
                                    h +
                                    " of type " +
                                    typeof h +
                                    "!",
                            );
                            return;
                    }
                switch (e) {
                    case 1:
                        e = f;
                        lb[b >> 2] = e;
                        lb[(b + 4) >> 2] = (e - lb[b >> 2]) / 4294967296;
                        break;
                    case 0:
                        R[b >> 2] = f;
                        break;
                    case 2:
                        T[b >> 2] = f;
                        break;
                    case 4:
                        kb[b >> 0] = f ? 1 : 0;
                }
            } else wd(1281);
        }
        function Fd(a) {
            var b = oa(a) + 1,
                e = Gd(b);
            pa(a, K, e, b);
            return e;
        }
        function Hd(a) {
            return "]" == a.slice(-1) && a.lastIndexOf("[");
        }
        function Id(a) {
            a -= 5120;
            return 0 == a
                ? kb
                : 1 == a
                ? K
                : 2 == a
                ? Za
                : 4 == a
                ? R
                : 6 == a
                ? T
                : 5 == a || 28922 == a || 28520 == a || 30779 == a || 30782 == a
                ? lb
                : Ya;
        }
        function Jd(a, b, e, f, h) {
            a = Id(a);
            var m = 31 - Math.clz32(a.BYTES_PER_ELEMENT),
                u = vd;
            return a.subarray(
                h >> m,
                (h +
                    f *
                        ((e *
                            ({
                                5: 3,
                                6: 4,
                                8: 2,
                                29502: 3,
                                29504: 4,
                                26917: 2,
                                26918: 2,
                                29846: 3,
                                29847: 4,
                            }[b - 6402] || 1) *
                            (1 << m) +
                            u -
                            1) &
                            -u)) >>
                    m,
            );
        }
        function Kd(a) {
            var b = Z.If;
            if (b) {
                var e = b.Oe[a];
                "number" == typeof e &&
                    (b.Oe[a] = e =
                        Z.getUniformLocation(
                            b,
                            b.xf[a] + (0 < e ? "[" + e + "]" : ""),
                        ));
                return e;
            }
            wd(1282);
        }
        var Ld = [],
            Md = [],
            Nd = {};
        function Od() {
            if (!Pd) {
                var a = {
                        USER: "web_user",
                        LOGNAME: "web_user",
                        PATH: "/",
                        PWD: "/",
                        HOME: "/home/web_user",
                        LANG:
                            (
                                ("object" == typeof navigator &&
                                    navigator.languages &&
                                    navigator.languages[0]) ||
                                "C"
                            ).replace("-", "_") + ".UTF-8",
                        _: va || "./this.program",
                    },
                    b;
                for (b in Nd) void 0 === Nd[b] ? delete a[b] : (a[b] = Nd[b]);
                var e = [];
                for (b in a) e.push(b + "=" + a[b]);
                Pd = e;
            }
            return Pd;
        }
        var Pd,
            Qd = [null, [], []];
        function Rd(a) {
            return 0 === a % 4 && (0 !== a % 100 || 0 === a % 400);
        }
        var Sd = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
            Td = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        function Ud(a, b, e, f) {
            function h(z, P, V) {
                for (
                    z = "number" == typeof z ? z.toString() : z || "";
                    z.length < P;

                )
                    z = V[0] + z;
                return z;
            }
            function m(z, P) {
                return h(z, P, "0");
            }
            function u(z, P) {
                function V(ra) {
                    return 0 > ra ? -1 : 0 < ra ? 1 : 0;
                }
                var S;
                0 === (S = V(z.getFullYear() - P.getFullYear())) &&
                    0 === (S = V(z.getMonth() - P.getMonth())) &&
                    (S = V(z.getDate() - P.getDate()));
                return S;
            }
            function n(z) {
                switch (z.getDay()) {
                    case 0:
                        return new Date(z.getFullYear() - 1, 11, 29);
                    case 1:
                        return z;
                    case 2:
                        return new Date(z.getFullYear(), 0, 3);
                    case 3:
                        return new Date(z.getFullYear(), 0, 2);
                    case 4:
                        return new Date(z.getFullYear(), 0, 1);
                    case 5:
                        return new Date(z.getFullYear() - 1, 11, 31);
                    case 6:
                        return new Date(z.getFullYear() - 1, 11, 30);
                }
            }
            function r(z) {
                var P = z.qe;
                for (
                    z = new Date(new Date(z.re + 1900, 0, 1).getTime());
                    0 < P;

                ) {
                    var V = z.getMonth(),
                        S = (Rd(z.getFullYear()) ? Sd : Td)[V];
                    if (P > S - z.getDate())
                        (P -= S - z.getDate() + 1),
                            z.setDate(1),
                            11 > V
                                ? z.setMonth(V + 1)
                                : (z.setMonth(0),
                                  z.setFullYear(z.getFullYear() + 1));
                    else {
                        z.setDate(z.getDate() + P);
                        break;
                    }
                }
                V = new Date(z.getFullYear() + 1, 0, 4);
                P = n(new Date(z.getFullYear(), 0, 4));
                V = n(V);
                return 0 >= u(P, z)
                    ? 0 >= u(V, z)
                        ? z.getFullYear() + 1
                        : z.getFullYear()
                    : z.getFullYear() - 1;
            }
            var v = R[(f + 40) >> 2];
            f = {
                qg: R[f >> 2],
                pg: R[(f + 4) >> 2],
                Ye: R[(f + 8) >> 2],
                kf: R[(f + 12) >> 2],
                Ze: R[(f + 16) >> 2],
                re: R[(f + 20) >> 2],
                ke: R[(f + 24) >> 2],
                qe: R[(f + 28) >> 2],
                wg: R[(f + 32) >> 2],
                og: R[(f + 36) >> 2],
                rg: v ? Va(v) : "",
            };
            e = Va(e);
            v = {
                "%c": "%a %b %d %H:%M:%S %Y",
                "%D": "%m/%d/%y",
                "%F": "%Y-%m-%d",
                "%h": "%b",
                "%r": "%I:%M:%S %p",
                "%R": "%H:%M",
                "%T": "%H:%M:%S",
                "%x": "%m/%d/%y",
                "%X": "%H:%M:%S",
                "%Ec": "%c",
                "%EC": "%C",
                "%Ex": "%m/%d/%y",
                "%EX": "%H:%M:%S",
                "%Ey": "%y",
                "%EY": "%Y",
                "%Od": "%d",
                "%Oe": "%e",
                "%OH": "%H",
                "%OI": "%I",
                "%Om": "%m",
                "%OM": "%M",
                "%OS": "%S",
                "%Ou": "%u",
                "%OU": "%U",
                "%OV": "%V",
                "%Ow": "%w",
                "%OW": "%W",
                "%Oy": "%y",
            };
            for (var D in v) e = e.replace(new RegExp(D, "g"), v[D]);
            var I =
                    "Sunday Monday Tuesday Wednesday Thursday Friday Saturday".split(
                        " ",
                    ),
                O =
                    "January February March April May June July August September October November December".split(
                        " ",
                    );
            v = {
                "%a": function (z) {
                    return I[z.ke].substring(0, 3);
                },
                "%A": function (z) {
                    return I[z.ke];
                },
                "%b": function (z) {
                    return O[z.Ze].substring(0, 3);
                },
                "%B": function (z) {
                    return O[z.Ze];
                },
                "%C": function (z) {
                    return m(((z.re + 1900) / 100) | 0, 2);
                },
                "%d": function (z) {
                    return m(z.kf, 2);
                },
                "%e": function (z) {
                    return h(z.kf, 2, " ");
                },
                "%g": function (z) {
                    return r(z).toString().substring(2);
                },
                "%G": function (z) {
                    return r(z);
                },
                "%H": function (z) {
                    return m(z.Ye, 2);
                },
                "%I": function (z) {
                    z = z.Ye;
                    0 == z ? (z = 12) : 12 < z && (z -= 12);
                    return m(z, 2);
                },
                "%j": function (z) {
                    for (
                        var P = 0, V = 0;
                        V <= z.Ze - 1;
                        P += (Rd(z.re + 1900) ? Sd : Td)[V++]
                    );
                    return m(z.kf + P, 3);
                },
                "%m": function (z) {
                    return m(z.Ze + 1, 2);
                },
                "%M": function (z) {
                    return m(z.pg, 2);
                },
                "%n": function () {
                    return "\n";
                },
                "%p": function (z) {
                    return 0 <= z.Ye && 12 > z.Ye ? "AM" : "PM";
                },
                "%S": function (z) {
                    return m(z.qg, 2);
                },
                "%t": function () {
                    return "\t";
                },
                "%u": function (z) {
                    return z.ke || 7;
                },
                "%U": function (z) {
                    return m(Math.floor((z.qe + 7 - z.ke) / 7), 2);
                },
                "%V": function (z) {
                    var P = Math.floor((z.qe + 7 - ((z.ke + 6) % 7)) / 7);
                    2 >= (z.ke + 371 - z.qe - 2) % 7 && P++;
                    if (P)
                        53 == P &&
                            ((V = (z.ke + 371 - z.qe) % 7),
                            4 == V || (3 == V && Rd(z.re)) || (P = 1));
                    else {
                        P = 52;
                        var V = (z.ke + 7 - z.qe - 1) % 7;
                        (4 == V || (5 == V && Rd((z.re % 400) - 1))) && P++;
                    }
                    return m(P, 2);
                },
                "%w": function (z) {
                    return z.ke;
                },
                "%W": function (z) {
                    return m(Math.floor((z.qe + 7 - ((z.ke + 6) % 7)) / 7), 2);
                },
                "%y": function (z) {
                    return (z.re + 1900).toString().substring(2);
                },
                "%Y": function (z) {
                    return z.re + 1900;
                },
                "%z": function (z) {
                    z = z.og;
                    var P = 0 <= z;
                    z = Math.abs(z) / 60;
                    return (
                        (P ? "+" : "-") +
                        String("0000" + ((z / 60) * 100 + (z % 60))).slice(-4)
                    );
                },
                "%Z": function (z) {
                    return z.rg;
                },
                "%%": function () {
                    return "%";
                },
            };
            e = e.replace(/%%/g, "\x00\x00");
            for (D in v)
                e.includes(D) && (e = e.replace(new RegExp(D, "g"), v[D](f)));
            e = e.replace(/\0\0/g, "%");
            D = Vd(e);
            if (D.length > b) return 0;
            kb.set(D, a);
            return D.length - 1;
        }
        Nb = w.InternalError = Mb("InternalError");
        for (var Wd = Array(256), Xd = 0; 256 > Xd; ++Xd)
            Wd[Xd] = String.fromCharCode(Xd);
        Sb = Wd;
        ac = w.BindingError = Mb("BindingError");
        oc.prototype.isAliasOf = function (a) {
            if (!(this instanceof oc && a instanceof oc)) return !1;
            var b = this.Md.Yd.Td,
                e = this.Md.Vd,
                f = a.Md.Yd.Td;
            for (a = a.Md.Vd; b.je; ) (e = b.Pe(e)), (b = b.je);
            for (; f.je; ) (a = f.Pe(a)), (f = f.je);
            return b === f && e === a;
        };
        oc.prototype.clone = function () {
            this.Md.Vd || bc(this);
            if (this.Md.Ne) return (this.Md.count.value += 1), this;
            var a = nc,
                b = Object,
                e = b.create,
                f = Object.getPrototypeOf(this),
                h = this.Md;
            a = a(
                e.call(b, f, {
                    Md: {
                        value: {
                            count: h.count,
                            Ge: h.Ge,
                            Ne: h.Ne,
                            Vd: h.Vd,
                            Yd: h.Yd,
                            ae: h.ae,
                            ge: h.ge,
                        },
                    },
                }),
            );
            a.Md.count.value += 1;
            a.Md.Ge = !1;
            return a;
        };
        oc.prototype["delete"] = function () {
            this.Md.Vd || bc(this);
            this.Md.Ge &&
                !this.Md.Ne &&
                X("Object already scheduled for deletion");
            dc(this);
            ec(this.Md);
            this.Md.Ne || ((this.Md.ae = void 0), (this.Md.Vd = void 0));
        };
        oc.prototype.isDeleted = function () {
            return !this.Md.Vd;
        };
        oc.prototype.deleteLater = function () {
            this.Md.Vd || bc(this);
            this.Md.Ge &&
                !this.Md.Ne &&
                X("Object already scheduled for deletion");
            hc.push(this);
            1 === hc.length && jc && jc(ic);
            this.Md.Ge = !0;
            return this;
        };
        w.getInheritedInstanceCount = function () {
            return Object.keys(kc).length;
        };
        w.getLiveInheritedInstances = function () {
            var a = [],
                b;
            for (b in kc) kc.hasOwnProperty(b) && a.push(kc[b]);
            return a;
        };
        w.flushPendingDeletes = ic;
        w.setDelayFunction = function (a) {
            jc = a;
            hc.length && jc && jc(ic);
        };
        yc.prototype.Rf = function (a) {
            this.wf && (a = this.wf(a));
            return a;
        };
        yc.prototype.qf = function (a) {
            this.me && this.me(a);
        };
        yc.prototype.argPackAdvance = 8;
        yc.prototype.readValueFromPointer = Gb;
        yc.prototype.deleteObject = function (a) {
            if (null !== a) a["delete"]();
        };
        yc.prototype.fromWireType = function (a) {
            function b() {
                return this.Ue
                    ? mc(this.Td.He, { Yd: this.bg, Vd: e, ge: this, ae: a })
                    : mc(this.Td.He, { Yd: this, Vd: a });
            }
            var e = this.Rf(a);
            if (!e) return this.qf(a), null;
            var f = lc(this.Td, e);
            if (void 0 !== f) {
                if (0 === f.Md.count.value)
                    return (f.Md.Vd = e), (f.Md.ae = a), f.clone();
                f = f.clone();
                this.qf(a);
                return f;
            }
            f = this.Td.Qf(e);
            f = gc[f];
            if (!f) return b.call(this);
            f = this.Te ? f.Gf : f.pointerType;
            var h = fc(e, this.Td, f.Td);
            return null === h
                ? b.call(this)
                : this.Ue
                ? mc(f.Td.He, { Yd: f, Vd: h, ge: this, ae: a })
                : mc(f.Td.He, { Yd: f, Vd: h });
        };
        Ic = w.UnboundTypeError = Mb("UnboundTypeError");
        w.count_emval_handles = function () {
            for (var a = 0, b = 5; b < Qc.length; ++b) void 0 !== Qc[b] && ++a;
            return a;
        };
        w.get_first_emval = function () {
            for (var a = 5; a < Qc.length; ++a)
                if (void 0 !== Qc[a]) return Qc[a];
            return null;
        };
        for (var Z, Yd = 0; 32 > Yd; ++Yd) Cd.push(Array(Yd));
        var Zd = new Float32Array(288);
        for (Yd = 0; 288 > Yd; ++Yd) Ld[Yd] = Zd.subarray(0, Yd + 1);
        var $d = new Int32Array(288);
        for (Yd = 0; 288 > Yd; ++Yd) Md[Yd] = $d.subarray(0, Yd + 1);
        function Vd(a) {
            var b = Array(oa(a) + 1);
            pa(a, b, 0, b.length);
            return b;
        }
        var pe = {
            V: function () {
                return 0;
            },
            Bb: function () {},
            Db: function () {
                return 0;
            },
            yb: function () {},
            zb: function () {},
            W: function () {},
            Ab: function () {},
            D: function (a) {
                var b = Eb[a];
                delete Eb[a];
                var e = b.hf,
                    f = b.me,
                    h = b.tf,
                    m = h.map((u) => u.Uf).concat(h.map((u) => u.gg));
                Pb([a], m, (u) => {
                    var n = {};
                    h.forEach((r, v) => {
                        var D = u[v],
                            I = r.Sf,
                            O = r.Tf,
                            z = u[v + h.length],
                            P = r.fg,
                            V = r.hg;
                        n[r.Mf] = {
                            read: (S) => D.fromWireType(I(O, S)),
                            write: (S, ra) => {
                                var ya = [];
                                P(V, S, z.toWireType(ya, ra));
                                Fb(ya);
                            },
                        };
                    });
                    return [
                        {
                            name: b.name,
                            fromWireType: function (r) {
                                var v = {},
                                    D;
                                for (D in n) v[D] = n[D].read(r);
                                f(r);
                                return v;
                            },
                            toWireType: function (r, v) {
                                for (var D in n)
                                    if (!(D in v))
                                        throw new TypeError(
                                            'Missing field:  "' + D + '"',
                                        );
                                var I = e();
                                for (D in n) n[D].write(I, v[D]);
                                null !== r && r.push(f, I);
                                return I;
                            },
                            argPackAdvance: 8,
                            readValueFromPointer: Gb,
                            fe: f,
                        },
                    ];
                });
            },
            qb: function () {},
            Hb: function (a, b, e, f, h) {
                var m = Rb(e);
                b = Tb(b);
                Qb(a, {
                    name: b,
                    fromWireType: function (u) {
                        return !!u;
                    },
                    toWireType: function (u, n) {
                        return n ? f : h;
                    },
                    argPackAdvance: 8,
                    readValueFromPointer: function (u) {
                        if (1 === e) var n = kb;
                        else if (2 === e) n = Za;
                        else if (4 === e) n = R;
                        else
                            throw new TypeError(
                                "Unknown boolean type size: " + b,
                            );
                        return this.fromWireType(n[u >> m]);
                    },
                    fe: null,
                });
            },
            p: function (a, b, e, f, h, m, u, n, r, v, D, I, O) {
                D = Tb(D);
                m = Bc(h, m);
                n && (n = Bc(u, n));
                v && (v = Bc(r, v));
                O = Bc(I, O);
                var z = Kb(D);
                qc(z, function () {
                    Mc("Cannot construct " + D + " due to unbound types", [f]);
                });
                Pb([a, b, e], f ? [f] : [], function (P) {
                    P = P[0];
                    if (f) {
                        var V = P.Td;
                        var S = V.He;
                    } else S = oc.prototype;
                    P = Lb(z, function () {
                        if (Object.getPrototypeOf(this) !== ra)
                            throw new ac("Use 'new' to construct " + D);
                        if (void 0 === ya.pe)
                            throw new ac(D + " has no accessible constructor");
                        var hb = ya.pe[arguments.length];
                        if (void 0 === hb)
                            throw new ac(
                                "Tried to invoke ctor of " +
                                    D +
                                    " with invalid number of parameters (" +
                                    arguments.length +
                                    ") - expected (" +
                                    Object.keys(ya.pe).toString() +
                                    ") parameters instead!",
                            );
                        return hb.apply(this, arguments);
                    });
                    var ra = Object.create(S, { constructor: { value: P } });
                    P.prototype = ra;
                    var ya = new rc(D, P, ra, O, V, m, n, v);
                    V = new yc(D, ya, !0, !1, !1);
                    S = new yc(D + "*", ya, !1, !1, !1);
                    var gb = new yc(D + " const*", ya, !1, !0, !1);
                    gc[a] = { pointerType: S, Gf: gb };
                    zc(z, P);
                    return [V, S, gb];
                });
            },
            i: function (a, b, e, f, h, m, u) {
                var n = Oc(e, f);
                b = Tb(b);
                m = Bc(h, m);
                Pb([], [a], function (r) {
                    function v() {
                        Mc("Cannot call " + D + " due to unbound types", n);
                    }
                    r = r[0];
                    var D = r.name + "." + b;
                    b.startsWith("@@") && (b = Symbol[b.substring(2)]);
                    var I = r.Td.constructor;
                    void 0 === I[b]
                        ? ((v.Ee = e - 1), (I[b] = v))
                        : (pc(I, b, D), (I[b].$d[e - 1] = v));
                    Pb([], n, function (O) {
                        O = [O[0], null].concat(O.slice(1));
                        O = Nc(D, O, null, m, u);
                        void 0 === I[b].$d
                            ? ((O.Ee = e - 1), (I[b] = O))
                            : (I[b].$d[e - 1] = O);
                        return [];
                    });
                    return [];
                });
            },
            F: function (a, b, e, f, h, m) {
                0 < b || Na();
                var u = Oc(b, e);
                h = Bc(f, h);
                Pb([], [a], function (n) {
                    n = n[0];
                    var r = "constructor " + n.name;
                    void 0 === n.Td.pe && (n.Td.pe = []);
                    if (void 0 !== n.Td.pe[b - 1])
                        throw new ac(
                            "Cannot register multiple constructors with identical number of parameters (" +
                                (b - 1) +
                                ") for class '" +
                                n.name +
                                "'! Overload resolution is currently only performed using the parameter count, not actual type info!",
                        );
                    n.Td.pe[b - 1] = () => {
                        Mc(
                            "Cannot construct " +
                                n.name +
                                " due to unbound types",
                            u,
                        );
                    };
                    Pb([], u, function (v) {
                        v.splice(1, 0, null);
                        n.Td.pe[b - 1] = Nc(r, v, null, h, m);
                        return [];
                    });
                    return [];
                });
            },
            b: function (a, b, e, f, h, m, u, n) {
                var r = Oc(e, f);
                b = Tb(b);
                m = Bc(h, m);
                Pb([], [a], function (v) {
                    function D() {
                        Mc("Cannot call " + I + " due to unbound types", r);
                    }
                    v = v[0];
                    var I = v.name + "." + b;
                    b.startsWith("@@") && (b = Symbol[b.substring(2)]);
                    n && v.Td.cg.push(b);
                    var O = v.Td.He,
                        z = O[b];
                    void 0 === z ||
                    (void 0 === z.$d &&
                        z.className !== v.name &&
                        z.Ee === e - 2)
                        ? ((D.Ee = e - 2), (D.className = v.name), (O[b] = D))
                        : (pc(O, b, I), (O[b].$d[e - 2] = D));
                    Pb([], r, function (P) {
                        P = Nc(I, P, v, m, u);
                        void 0 === O[b].$d
                            ? ((P.Ee = e - 2), (O[b] = P))
                            : (O[b].$d[e - 2] = P);
                        return [];
                    });
                    return [];
                });
            },
            u: function (a, b, e) {
                a = Tb(a);
                Pb([], [b], function (f) {
                    f = f[0];
                    w[a] = f.fromWireType(e);
                    return [];
                });
            },
            Gb: function (a, b) {
                b = Tb(b);
                Qb(a, {
                    name: b,
                    fromWireType: function (e) {
                        var f = Sc(e);
                        Rc(e);
                        return f;
                    },
                    toWireType: function (e, f) {
                        return wc(f);
                    },
                    argPackAdvance: 8,
                    readValueFromPointer: Gb,
                    fe: null,
                });
            },
            o: function (a, b, e, f) {
                function h() {}
                e = Rb(e);
                b = Tb(b);
                h.values = {};
                Qb(a, {
                    name: b,
                    constructor: h,
                    fromWireType: function (m) {
                        return this.constructor.values[m];
                    },
                    toWireType: function (m, u) {
                        return u.value;
                    },
                    argPackAdvance: 8,
                    readValueFromPointer: Tc(b, e, f),
                    fe: null,
                });
                qc(b, h);
            },
            f: function (a, b, e) {
                var f = Uc(a, "enum");
                b = Tb(b);
                a = f.constructor;
                f = Object.create(f.constructor.prototype, {
                    value: { value: e },
                    constructor: {
                        value: Lb(f.name + "_" + b, function () {}),
                    },
                });
                a.values[e] = f;
                a[b] = f;
            },
            Z: function (a, b, e) {
                e = Rb(e);
                b = Tb(b);
                Qb(a, {
                    name: b,
                    fromWireType: function (f) {
                        return f;
                    },
                    toWireType: function (f, h) {
                        return h;
                    },
                    argPackAdvance: 8,
                    readValueFromPointer: Vc(b, e),
                    fe: null,
                });
            },
            y: function (a, b, e, f, h, m) {
                var u = Oc(b, e);
                a = Tb(a);
                h = Bc(f, h);
                qc(
                    a,
                    function () {
                        Mc("Cannot call " + a + " due to unbound types", u);
                    },
                    b - 1,
                );
                Pb([], u, function (n) {
                    n = [n[0], null].concat(n.slice(1));
                    zc(a, Nc(a, n, null, h, m), b - 1);
                    return [];
                });
            },
            H: function (a, b, e, f, h) {
                b = Tb(b);
                -1 === h && (h = 4294967295);
                h = Rb(e);
                var m = (n) => n;
                if (0 === f) {
                    var u = 32 - 8 * e;
                    m = (n) => (n << u) >>> u;
                }
                e = b.includes("unsigned")
                    ? function (n, r) {
                          return r >>> 0;
                      }
                    : function (n, r) {
                          return r;
                      };
                Qb(a, {
                    name: b,
                    fromWireType: m,
                    toWireType: e,
                    argPackAdvance: 8,
                    readValueFromPointer: Wc(b, h, 0 !== f),
                    fe: null,
                });
            },
            x: function (a, b, e) {
                function f(m) {
                    m >>= 2;
                    var u = lb;
                    return new h(jb, u[m + 1], u[m]);
                }
                var h = [
                    Int8Array,
                    Uint8Array,
                    Int16Array,
                    Uint16Array,
                    Int32Array,
                    Uint32Array,
                    Float32Array,
                    Float64Array,
                ][b];
                e = Tb(e);
                Qb(
                    a,
                    {
                        name: e,
                        fromWireType: f,
                        argPackAdvance: 8,
                        readValueFromPointer: f,
                    },
                    { Wf: !0 },
                );
            },
            t: function (a, b, e, f, h, m, u, n, r, v, D, I) {
                e = Tb(e);
                m = Bc(h, m);
                n = Bc(u, n);
                v = Bc(r, v);
                I = Bc(D, I);
                Pb([a], [b], function (O) {
                    O = O[0];
                    return [new yc(e, O.Td, !1, !1, !0, O, f, m, n, v, I)];
                });
            },
            Y: function (a, b) {
                b = Tb(b);
                var e = "std::string" === b;
                Qb(a, {
                    name: b,
                    fromWireType: function (f) {
                        var h = lb[f >> 2],
                            m = f + 4;
                        if (e)
                            for (var u = m, n = 0; n <= h; ++n) {
                                var r = m + n;
                                if (n == h || 0 == K[r]) {
                                    u = Va(u, r - u);
                                    if (void 0 === v) var v = u;
                                    else
                                        (v += String.fromCharCode(0)), (v += u);
                                    u = r + 1;
                                }
                            }
                        else {
                            v = Array(h);
                            for (n = 0; n < h; ++n)
                                v[n] = String.fromCharCode(K[m + n]);
                            v = v.join("");
                        }
                        Lc(f);
                        return v;
                    },
                    toWireType: function (f, h) {
                        h instanceof ArrayBuffer && (h = new Uint8Array(h));
                        var m = "string" == typeof h;
                        m ||
                            h instanceof Uint8Array ||
                            h instanceof Uint8ClampedArray ||
                            h instanceof Int8Array ||
                            X("Cannot pass non-string to std::string");
                        var u = e && m ? oa(h) : h.length;
                        var n = Gd(4 + u + 1),
                            r = n + 4;
                        lb[n >> 2] = u;
                        if (e && m) pa(h, K, r, u + 1);
                        else if (m)
                            for (m = 0; m < u; ++m) {
                                var v = h.charCodeAt(m);
                                255 < v &&
                                    (Lc(r),
                                    X(
                                        "String has UTF-16 code units that do not fit in 8 bits",
                                    ));
                                K[r + m] = v;
                            }
                        else for (m = 0; m < u; ++m) K[r + m] = h[m];
                        null !== f && f.push(Lc, n);
                        return n;
                    },
                    argPackAdvance: 8,
                    readValueFromPointer: Gb,
                    fe: function (f) {
                        Lc(f);
                    },
                });
            },
            Q: function (a, b, e) {
                e = Tb(e);
                if (2 === b) {
                    var f = Xa;
                    var h = $a;
                    var m = cb;
                    var u = () => Ya;
                    var n = 1;
                } else
                    4 === b &&
                        ((f = db), (h = eb), (m = fb), (u = () => lb), (n = 2));
                Qb(a, {
                    name: e,
                    fromWireType: function (r) {
                        for (
                            var v = lb[r >> 2], D = u(), I, O = r + 4, z = 0;
                            z <= v;
                            ++z
                        ) {
                            var P = r + 4 + z * b;
                            if (z == v || 0 == D[P >> n])
                                (O = f(O, P - O)),
                                    void 0 === I
                                        ? (I = O)
                                        : ((I += String.fromCharCode(0)),
                                          (I += O)),
                                    (O = P + b);
                        }
                        Lc(r);
                        return I;
                    },
                    toWireType: function (r, v) {
                        "string" != typeof v &&
                            X("Cannot pass non-string to C++ string type " + e);
                        var D = m(v),
                            I = Gd(4 + D + b);
                        lb[I >> 2] = D >> n;
                        h(v, I + 4, D + b);
                        null !== r && r.push(Lc, I);
                        return I;
                    },
                    argPackAdvance: 8,
                    readValueFromPointer: Gb,
                    fe: function (r) {
                        Lc(r);
                    },
                });
            },
            E: function (a, b, e, f, h, m) {
                Eb[a] = { name: Tb(b), hf: Bc(e, f), me: Bc(h, m), tf: [] };
            },
            g: function (a, b, e, f, h, m, u, n, r, v) {
                Eb[a].tf.push({
                    Mf: Tb(b),
                    Uf: e,
                    Sf: Bc(f, h),
                    Tf: m,
                    gg: u,
                    fg: Bc(n, r),
                    hg: v,
                });
            },
            Ib: function (a, b) {
                b = Tb(b);
                Qb(a, {
                    Yf: !0,
                    name: b,
                    argPackAdvance: 0,
                    fromWireType: function () {},
                    toWireType: function () {},
                });
            },
            Fb: function () {
                return !0;
            },
            sb: function () {
                throw Infinity;
            },
            G: function (a, b, e) {
                a = Sc(a);
                b = Uc(b, "emval::as");
                var f = [],
                    h = wc(f);
                lb[e >> 2] = h;
                return b.toWireType(f, a);
            },
            M: function (a, b, e, f, h) {
                a = Zc[a];
                b = Sc(b);
                e = Yc(e);
                var m = [];
                lb[f >> 2] = wc(m);
                return a(b, e, m, h);
            },
            z: function (a, b, e, f) {
                a = Zc[a];
                b = Sc(b);
                e = Yc(e);
                a(b, e, null, f);
            },
            e: Rc,
            L: function (a) {
                if (0 === a) return wc($c());
                a = Yc(a);
                return wc($c()[a]);
            },
            v: function (a, b) {
                var e = bd(a, b),
                    f = e[0];
                b =
                    f.name +
                    "_$" +
                    e
                        .slice(1)
                        .map(function (u) {
                            return u.name;
                        })
                        .join("_") +
                    "$";
                var h = cd[b];
                if (void 0 !== h) return h;
                var m = Array(a - 1);
                h = ad((u, n, r, v) => {
                    for (var D = 0, I = 0; I < a - 1; ++I)
                        (m[I] = e[I + 1].readValueFromPointer(v + D)),
                            (D += e[I + 1].argPackAdvance);
                    u = u[n].apply(u, m);
                    for (I = 0; I < a - 1; ++I)
                        e[I + 1].Jf && e[I + 1].Jf(m[I]);
                    if (!f.Yf) return f.toWireType(r, u);
                });
                return (cd[b] = h);
            },
            I: function (a, b) {
                a = Sc(a);
                b = Sc(b);
                return wc(a[b]);
            },
            r: function (a) {
                4 < a && (Qc[a].jf += 1);
            },
            N: function (a, b, e, f) {
                a = Sc(a);
                var h = ed[b];
                h || ((h = dd(b)), (ed[b] = h));
                return h(a, e, f);
            },
            J: function () {
                return wc([]);
            },
            h: function (a) {
                return wc(Yc(a));
            },
            C: function () {
                return wc({});
            },
            mb: function (a) {
                a = Sc(a);
                return !a;
            },
            B: function (a) {
                var b = Sc(a);
                Fb(b);
                Rc(a);
            },
            k: function (a, b, e) {
                a = Sc(a);
                b = Sc(b);
                e = Sc(e);
                a[b] = e;
            },
            j: function (a, b) {
                a = Uc(a, "_emval_take_value");
                a = a.readValueFromPointer(b);
                return wc(a);
            },
            ub: function () {
                return -52;
            },
            vb: function () {},
            a: function () {
                Na("");
            },
            Eb: fd,
            bd: function (a) {
                Z.activeTexture(a);
            },
            cd: function (a, b) {
                Z.attachShader(md[a], pd[b]);
            },
            ca: function (a, b, e) {
                Z.bindAttribLocation(md[a], b, Va(e));
            },
            da: function (a, b) {
                35051 == a ? (Z.df = b) : 35052 == a && (Z.Fe = b);
                Z.bindBuffer(a, ld[b]);
            },
            ba: function (a, b) {
                Z.bindFramebuffer(a, nd[b]);
            },
            fc: function (a, b) {
                Z.bindRenderbuffer(a, od[b]);
            },
            Rb: function (a, b) {
                Z.bindSampler(a, rd[b]);
            },
            ea: function (a, b) {
                Z.bindTexture(a, ja[b]);
            },
            Cc: function (a) {
                Z.bindVertexArray(qd[a]);
            },
            xc: function (a) {
                Z.bindVertexArray(qd[a]);
            },
            fa: function (a, b, e, f) {
                Z.blendColor(a, b, e, f);
            },
            ga: function (a) {
                Z.blendEquation(a);
            },
            ha: function (a, b) {
                Z.blendFunc(a, b);
            },
            $b: function (a, b, e, f, h, m, u, n, r, v) {
                Z.blitFramebuffer(a, b, e, f, h, m, u, n, r, v);
            },
            ia: function (a, b, e, f) {
                2 <= x.version
                    ? e && b
                        ? Z.bufferData(a, K, f, e, b)
                        : Z.bufferData(a, b, f)
                    : Z.bufferData(a, e ? K.subarray(e, e + b) : b, f);
            },
            ja: function (a, b, e, f) {
                2 <= x.version
                    ? e && Z.bufferSubData(a, b, K, f, e)
                    : Z.bufferSubData(a, b, K.subarray(f, f + e));
            },
            gc: function (a) {
                return Z.checkFramebufferStatus(a);
            },
            T: function (a) {
                Z.clear(a);
            },
            aa: function (a, b, e, f) {
                Z.clearColor(a, b, e, f);
            },
            X: function (a) {
                Z.clearStencil(a);
            },
            kb: function (a, b, e, f) {
                return Z.clientWaitSync(sd[a], b, (e >>> 0) + 4294967296 * f);
            },
            ka: function (a, b, e, f) {
                Z.colorMask(!!a, !!b, !!e, !!f);
            },
            la: function (a) {
                Z.compileShader(pd[a]);
            },
            ma: function (a, b, e, f, h, m, u, n) {
                2 <= x.version
                    ? Z.Fe || !u
                        ? Z.compressedTexImage2D(a, b, e, f, h, m, u, n)
                        : Z.compressedTexImage2D(a, b, e, f, h, m, K, n, u)
                    : Z.compressedTexImage2D(
                          a,
                          b,
                          e,
                          f,
                          h,
                          m,
                          n ? K.subarray(n, n + u) : null,
                      );
            },
            na: function (a, b, e, f, h, m, u, n, r) {
                2 <= x.version
                    ? Z.Fe || !n
                        ? Z.compressedTexSubImage2D(a, b, e, f, h, m, u, n, r)
                        : Z.compressedTexSubImage2D(
                              a,
                              b,
                              e,
                              f,
                              h,
                              m,
                              u,
                              K,
                              r,
                              n,
                          )
                    : Z.compressedTexSubImage2D(
                          a,
                          b,
                          e,
                          f,
                          h,
                          m,
                          u,
                          r ? K.subarray(r, r + n) : null,
                      );
            },
            Zb: function (a, b, e, f, h) {
                Z.copyBufferSubData(a, b, e, f, h);
            },
            oa: function (a, b, e, f, h, m, u, n) {
                Z.copyTexSubImage2D(a, b, e, f, h, m, u, n);
            },
            pa: function () {
                var a = fa(md),
                    b = Z.createProgram();
                b.name = a;
                b.Xe = b.Ve = b.We = 0;
                b.lf = 1;
                md[a] = b;
                return a;
            },
            qa: function (a) {
                var b = fa(pd);
                pd[b] = Z.createShader(a);
                return b;
            },
            ra: function (a) {
                Z.cullFace(a);
            },
            sa: function (a, b) {
                for (var e = 0; e < a; e++) {
                    var f = R[(b + 4 * e) >> 2],
                        h = ld[f];
                    h &&
                        (Z.deleteBuffer(h),
                        (h.name = 0),
                        (ld[f] = null),
                        f == Z.df && (Z.df = 0),
                        f == Z.Fe && (Z.Fe = 0));
                }
            },
            hc: function (a, b) {
                for (var e = 0; e < a; ++e) {
                    var f = R[(b + 4 * e) >> 2],
                        h = nd[f];
                    h && (Z.deleteFramebuffer(h), (h.name = 0), (nd[f] = null));
                }
            },
            ta: function (a) {
                if (a) {
                    var b = md[a];
                    b
                        ? (Z.deleteProgram(b), (b.name = 0), (md[a] = null))
                        : wd(1281);
                }
            },
            ic: function (a, b) {
                for (var e = 0; e < a; e++) {
                    var f = R[(b + 4 * e) >> 2],
                        h = od[f];
                    h &&
                        (Z.deleteRenderbuffer(h), (h.name = 0), (od[f] = null));
                }
            },
            Sb: function (a, b) {
                for (var e = 0; e < a; e++) {
                    var f = R[(b + 4 * e) >> 2],
                        h = rd[f];
                    h && (Z.deleteSampler(h), (h.name = 0), (rd[f] = null));
                }
            },
            ua: function (a) {
                if (a) {
                    var b = pd[a];
                    b ? (Z.deleteShader(b), (pd[a] = null)) : wd(1281);
                }
            },
            _b: function (a) {
                if (a) {
                    var b = sd[a];
                    b
                        ? (Z.deleteSync(b), (b.name = 0), (sd[a] = null))
                        : wd(1281);
                }
            },
            va: function (a, b) {
                for (var e = 0; e < a; e++) {
                    var f = R[(b + 4 * e) >> 2],
                        h = ja[f];
                    h && (Z.deleteTexture(h), (h.name = 0), (ja[f] = null));
                }
            },
            Dc: function (a, b) {
                for (var e = 0; e < a; e++) {
                    var f = R[(b + 4 * e) >> 2];
                    Z.deleteVertexArray(qd[f]);
                    qd[f] = null;
                }
            },
            yc: function (a, b) {
                for (var e = 0; e < a; e++) {
                    var f = R[(b + 4 * e) >> 2];
                    Z.deleteVertexArray(qd[f]);
                    qd[f] = null;
                }
            },
            wa: function (a) {
                Z.depthMask(!!a);
            },
            xa: function (a) {
                Z.disable(a);
            },
            ya: function (a) {
                Z.disableVertexAttribArray(a);
            },
            za: function (a, b, e) {
                Z.drawArrays(a, b, e);
            },
            Ac: function (a, b, e, f) {
                Z.drawArraysInstanced(a, b, e, f);
            },
            vc: function (a, b, e, f, h) {
                Z.rf.drawArraysInstancedBaseInstanceWEBGL(a, b, e, f, h);
            },
            tc: function (a, b) {
                for (var e = Cd[a], f = 0; f < a; f++)
                    e[f] = R[(b + 4 * f) >> 2];
                Z.drawBuffers(e);
            },
            Aa: function (a, b, e, f) {
                Z.drawElements(a, b, e, f);
            },
            Bc: function (a, b, e, f, h) {
                Z.drawElementsInstanced(a, b, e, f, h);
            },
            wc: function (a, b, e, f, h, m, u) {
                Z.rf.drawElementsInstancedBaseVertexBaseInstanceWEBGL(
                    a,
                    b,
                    e,
                    f,
                    h,
                    m,
                    u,
                );
            },
            nc: function (a, b, e, f, h, m) {
                Z.drawElements(a, f, h, m);
            },
            Ba: function (a) {
                Z.enable(a);
            },
            Ca: function (a) {
                Z.enableVertexAttribArray(a);
            },
            Xb: function (a, b) {
                return (a = Z.fenceSync(a, b))
                    ? ((b = fa(sd)), (a.name = b), (sd[b] = a), b)
                    : 0;
            },
            Da: function () {
                Z.finish();
            },
            Ea: function () {
                Z.flush();
            },
            jc: function (a, b, e, f) {
                Z.framebufferRenderbuffer(a, b, e, od[f]);
            },
            kc: function (a, b, e, f, h) {
                Z.framebufferTexture2D(a, b, e, ja[f], h);
            },
            Fa: function (a) {
                Z.frontFace(a);
            },
            Ga: function (a, b) {
                Dd(a, b, "createBuffer", ld);
            },
            lc: function (a, b) {
                Dd(a, b, "createFramebuffer", nd);
            },
            mc: function (a, b) {
                Dd(a, b, "createRenderbuffer", od);
            },
            Tb: function (a, b) {
                Dd(a, b, "createSampler", rd);
            },
            Ha: function (a, b) {
                Dd(a, b, "createTexture", ja);
            },
            Ec: function (a, b) {
                Dd(a, b, "createVertexArray", qd);
            },
            zc: function (a, b) {
                Dd(a, b, "createVertexArray", qd);
            },
            bc: function (a) {
                Z.generateMipmap(a);
            },
            Ia: function (a, b, e) {
                e ? (R[e >> 2] = Z.getBufferParameter(a, b)) : wd(1281);
            },
            Ja: function () {
                var a = Z.getError() || xd;
                xd = 0;
                return a;
            },
            Ka: function (a, b) {
                Ed(a, b, 2);
            },
            cc: function (a, b, e, f) {
                a = Z.getFramebufferAttachmentParameter(a, b, e);
                if (a instanceof WebGLRenderbuffer || a instanceof WebGLTexture)
                    a = a.name | 0;
                R[f >> 2] = a;
            },
            O: function (a, b) {
                Ed(a, b, 0);
            },
            La: function (a, b, e, f) {
                a = Z.getProgramInfoLog(md[a]);
                null === a && (a = "(unknown error)");
                b = 0 < b && f ? pa(a, K, f, b) : 0;
                e && (R[e >> 2] = b);
            },
            Ma: function (a, b, e) {
                if (e)
                    if (a >= kd) wd(1281);
                    else if (((a = md[a]), 35716 == b))
                        (a = Z.getProgramInfoLog(a)),
                            null === a && (a = "(unknown error)"),
                            (R[e >> 2] = a.length + 1);
                    else if (35719 == b) {
                        if (!a.Xe)
                            for (
                                b = 0;
                                b < Z.getProgramParameter(a, 35718);
                                ++b
                            )
                                a.Xe = Math.max(
                                    a.Xe,
                                    Z.getActiveUniform(a, b).name.length + 1,
                                );
                        R[e >> 2] = a.Xe;
                    } else if (35722 == b) {
                        if (!a.Ve)
                            for (
                                b = 0;
                                b < Z.getProgramParameter(a, 35721);
                                ++b
                            )
                                a.Ve = Math.max(
                                    a.Ve,
                                    Z.getActiveAttrib(a, b).name.length + 1,
                                );
                        R[e >> 2] = a.Ve;
                    } else if (35381 == b) {
                        if (!a.We)
                            for (
                                b = 0;
                                b < Z.getProgramParameter(a, 35382);
                                ++b
                            )
                                a.We = Math.max(
                                    a.We,
                                    Z.getActiveUniformBlockName(a, b).length +
                                        1,
                                );
                        R[e >> 2] = a.We;
                    } else R[e >> 2] = Z.getProgramParameter(a, b);
                else wd(1281);
            },
            dc: function (a, b, e) {
                e ? (R[e >> 2] = Z.getRenderbufferParameter(a, b)) : wd(1281);
            },
            Na: function (a, b, e, f) {
                a = Z.getShaderInfoLog(pd[a]);
                null === a && (a = "(unknown error)");
                b = 0 < b && f ? pa(a, K, f, b) : 0;
                e && (R[e >> 2] = b);
            },
            Ob: function (a, b, e, f) {
                a = Z.getShaderPrecisionFormat(a, b);
                R[e >> 2] = a.rangeMin;
                R[(e + 4) >> 2] = a.rangeMax;
                R[f >> 2] = a.precision;
            },
            Oa: function (a, b, e) {
                e
                    ? 35716 == b
                        ? ((a = Z.getShaderInfoLog(pd[a])),
                          null === a && (a = "(unknown error)"),
                          (R[e >> 2] = a ? a.length + 1 : 0))
                        : 35720 == b
                        ? ((a = Z.getShaderSource(pd[a])),
                          (R[e >> 2] = a ? a.length + 1 : 0))
                        : (R[e >> 2] = Z.getShaderParameter(pd[a], b))
                    : wd(1281);
            },
            S: function (a) {
                var b = td[a];
                if (!b) {
                    switch (a) {
                        case 7939:
                            b = Z.getSupportedExtensions() || [];
                            b = b.concat(
                                b.map(function (f) {
                                    return "GL_" + f;
                                }),
                            );
                            b = Fd(b.join(" "));
                            break;
                        case 7936:
                        case 7937:
                        case 37445:
                        case 37446:
                            (b = Z.getParameter(a)) || wd(1280);
                            b = b && Fd(b);
                            break;
                        case 7938:
                            b = Z.getParameter(7938);
                            b =
                                2 <= x.version
                                    ? "OpenGL ES 3.0 (" + b + ")"
                                    : "OpenGL ES 2.0 (" + b + ")";
                            b = Fd(b);
                            break;
                        case 35724:
                            b = Z.getParameter(35724);
                            var e = b.match(
                                /^WebGL GLSL ES ([0-9]\.[0-9][0-9]?)(?:$| .*)/,
                            );
                            null !== e &&
                                (3 == e[1].length && (e[1] += "0"),
                                (b =
                                    "OpenGL ES GLSL ES " +
                                    e[1] +
                                    " (" +
                                    b +
                                    ")"));
                            b = Fd(b);
                            break;
                        default:
                            wd(1280);
                    }
                    td[a] = b;
                }
                return b;
            },
            jb: function (a, b) {
                if (2 > x.version) return wd(1282), 0;
                var e = ud[a];
                if (e) return 0 > b || b >= e.length ? (wd(1281), 0) : e[b];
                switch (a) {
                    case 7939:
                        return (
                            (e = Z.getSupportedExtensions() || []),
                            (e = e.concat(
                                e.map(function (f) {
                                    return "GL_" + f;
                                }),
                            )),
                            (e = e.map(function (f) {
                                return Fd(f);
                            })),
                            (e = ud[a] = e),
                            0 > b || b >= e.length ? (wd(1281), 0) : e[b]
                        );
                    default:
                        return wd(1280), 0;
                }
            },
            Pa: function (a, b) {
                b = Va(b);
                if ((a = md[a])) {
                    var e = a,
                        f = e.Oe,
                        h = e.yf,
                        m;
                    if (!f)
                        for (
                            e.Oe = f = {}, e.xf = {}, m = 0;
                            m < Z.getProgramParameter(e, 35718);
                            ++m
                        ) {
                            var u = Z.getActiveUniform(e, m);
                            var n = u.name;
                            u = u.size;
                            var r = Hd(n);
                            r = 0 < r ? n.slice(0, r) : n;
                            var v = e.lf;
                            e.lf += u;
                            h[r] = [u, v];
                            for (n = 0; n < u; ++n) (f[v] = n), (e.xf[v++] = r);
                        }
                    e = a.Oe;
                    f = 0;
                    h = b;
                    m = Hd(b);
                    0 < m &&
                        ((f = parseInt(b.slice(m + 1)) >>> 0),
                        (h = b.slice(0, m)));
                    if (
                        (h = a.yf[h]) &&
                        f < h[0] &&
                        ((f += h[1]),
                        (e[f] = e[f] || Z.getUniformLocation(a, b)))
                    )
                        return f;
                } else wd(1281);
                return -1;
            },
            Pb: function (a, b, e) {
                for (var f = Cd[b], h = 0; h < b; h++)
                    f[h] = R[(e + 4 * h) >> 2];
                Z.invalidateFramebuffer(a, f);
            },
            Qb: function (a, b, e, f, h, m, u) {
                for (var n = Cd[b], r = 0; r < b; r++)
                    n[r] = R[(e + 4 * r) >> 2];
                Z.invalidateSubFramebuffer(a, n, f, h, m, u);
            },
            Yb: function (a) {
                return Z.isSync(sd[a]);
            },
            Qa: function (a) {
                return (a = ja[a]) ? Z.isTexture(a) : 0;
            },
            Ra: function (a) {
                Z.lineWidth(a);
            },
            Sa: function (a) {
                a = md[a];
                Z.linkProgram(a);
                a.Oe = 0;
                a.yf = {};
            },
            rc: function (a, b, e, f, h, m) {
                Z.vf.multiDrawArraysInstancedBaseInstanceWEBGL(
                    a,
                    R,
                    b >> 2,
                    R,
                    e >> 2,
                    R,
                    f >> 2,
                    lb,
                    h >> 2,
                    m,
                );
            },
            sc: function (a, b, e, f, h, m, u, n) {
                Z.vf.multiDrawElementsInstancedBaseVertexBaseInstanceWEBGL(
                    a,
                    R,
                    b >> 2,
                    e,
                    R,
                    f >> 2,
                    R,
                    h >> 2,
                    R,
                    m >> 2,
                    lb,
                    u >> 2,
                    n,
                );
            },
            Ta: function (a, b) {
                3317 == a && (vd = b);
                Z.pixelStorei(a, b);
            },
            uc: function (a) {
                Z.readBuffer(a);
            },
            Ua: function (a, b, e, f, h, m, u) {
                if (2 <= x.version)
                    if (Z.df) Z.readPixels(a, b, e, f, h, m, u);
                    else {
                        var n = Id(m);
                        Z.readPixels(
                            a,
                            b,
                            e,
                            f,
                            h,
                            m,
                            n,
                            u >> (31 - Math.clz32(n.BYTES_PER_ELEMENT)),
                        );
                    }
                else
                    (u = Jd(m, h, e, f, u))
                        ? Z.readPixels(a, b, e, f, h, m, u)
                        : wd(1280);
            },
            ec: function (a, b, e, f) {
                Z.renderbufferStorage(a, b, e, f);
            },
            ac: function (a, b, e, f, h) {
                Z.renderbufferStorageMultisample(a, b, e, f, h);
            },
            Ub: function (a, b, e) {
                Z.samplerParameterf(rd[a], b, e);
            },
            Vb: function (a, b, e) {
                Z.samplerParameteri(rd[a], b, e);
            },
            Wb: function (a, b, e) {
                Z.samplerParameteri(rd[a], b, R[e >> 2]);
            },
            Va: function (a, b, e, f) {
                Z.scissor(a, b, e, f);
            },
            Wa: function (a, b, e, f) {
                for (var h = "", m = 0; m < b; ++m) {
                    var u = f ? R[(f + 4 * m) >> 2] : -1;
                    h += Va(R[(e + 4 * m) >> 2], 0 > u ? void 0 : u);
                }
                Z.shaderSource(pd[a], h);
            },
            Xa: function (a, b, e) {
                Z.stencilFunc(a, b, e);
            },
            Ya: function (a, b, e, f) {
                Z.stencilFuncSeparate(a, b, e, f);
            },
            Za: function (a) {
                Z.stencilMask(a);
            },
            _a: function (a, b) {
                Z.stencilMaskSeparate(a, b);
            },
            $a: function (a, b, e) {
                Z.stencilOp(a, b, e);
            },
            ab: function (a, b, e, f) {
                Z.stencilOpSeparate(a, b, e, f);
            },
            bb: function (a, b, e, f, h, m, u, n, r) {
                if (2 <= x.version)
                    if (Z.Fe) Z.texImage2D(a, b, e, f, h, m, u, n, r);
                    else if (r) {
                        var v = Id(n);
                        Z.texImage2D(
                            a,
                            b,
                            e,
                            f,
                            h,
                            m,
                            u,
                            n,
                            v,
                            r >> (31 - Math.clz32(v.BYTES_PER_ELEMENT)),
                        );
                    } else Z.texImage2D(a, b, e, f, h, m, u, n, null);
                else
                    Z.texImage2D(
                        a,
                        b,
                        e,
                        f,
                        h,
                        m,
                        u,
                        n,
                        r ? Jd(n, u, f, h, r) : null,
                    );
            },
            cb: function (a, b, e) {
                Z.texParameterf(a, b, e);
            },
            db: function (a, b, e) {
                Z.texParameterf(a, b, T[e >> 2]);
            },
            eb: function (a, b, e) {
                Z.texParameteri(a, b, e);
            },
            fb: function (a, b, e) {
                Z.texParameteri(a, b, R[e >> 2]);
            },
            oc: function (a, b, e, f, h) {
                Z.texStorage2D(a, b, e, f, h);
            },
            gb: function (a, b, e, f, h, m, u, n, r) {
                if (2 <= x.version)
                    if (Z.Fe) Z.texSubImage2D(a, b, e, f, h, m, u, n, r);
                    else if (r) {
                        var v = Id(n);
                        Z.texSubImage2D(
                            a,
                            b,
                            e,
                            f,
                            h,
                            m,
                            u,
                            n,
                            v,
                            r >> (31 - Math.clz32(v.BYTES_PER_ELEMENT)),
                        );
                    } else Z.texSubImage2D(a, b, e, f, h, m, u, n, null);
                else
                    (v = null),
                        r && (v = Jd(n, u, h, m, r)),
                        Z.texSubImage2D(a, b, e, f, h, m, u, n, v);
            },
            hb: function (a, b) {
                Z.uniform1f(Kd(a), b);
            },
            ib: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform1fv(Kd(a), T, e >> 2, b);
                else {
                    if (288 >= b)
                        for (var f = Ld[b - 1], h = 0; h < b; ++h)
                            f[h] = T[(e + 4 * h) >> 2];
                    else f = T.subarray(e >> 2, (e + 4 * b) >> 2);
                    Z.uniform1fv(Kd(a), f);
                }
            },
            Zc: function (a, b) {
                Z.uniform1i(Kd(a), b);
            },
            _c: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform1iv(Kd(a), R, e >> 2, b);
                else {
                    if (288 >= b)
                        for (var f = Md[b - 1], h = 0; h < b; ++h)
                            f[h] = R[(e + 4 * h) >> 2];
                    else f = R.subarray(e >> 2, (e + 4 * b) >> 2);
                    Z.uniform1iv(Kd(a), f);
                }
            },
            $c: function (a, b, e) {
                Z.uniform2f(Kd(a), b, e);
            },
            ad: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform2fv(Kd(a), T, e >> 2, 2 * b);
                else {
                    if (144 >= b)
                        for (var f = Ld[2 * b - 1], h = 0; h < 2 * b; h += 2)
                            (f[h] = T[(e + 4 * h) >> 2]),
                                (f[h + 1] = T[(e + (4 * h + 4)) >> 2]);
                    else f = T.subarray(e >> 2, (e + 8 * b) >> 2);
                    Z.uniform2fv(Kd(a), f);
                }
            },
            Yc: function (a, b, e) {
                Z.uniform2i(Kd(a), b, e);
            },
            Xc: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform2iv(Kd(a), R, e >> 2, 2 * b);
                else {
                    if (144 >= b)
                        for (var f = Md[2 * b - 1], h = 0; h < 2 * b; h += 2)
                            (f[h] = R[(e + 4 * h) >> 2]),
                                (f[h + 1] = R[(e + (4 * h + 4)) >> 2]);
                    else f = R.subarray(e >> 2, (e + 8 * b) >> 2);
                    Z.uniform2iv(Kd(a), f);
                }
            },
            Wc: function (a, b, e, f) {
                Z.uniform3f(Kd(a), b, e, f);
            },
            Vc: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform3fv(Kd(a), T, e >> 2, 3 * b);
                else {
                    if (96 >= b)
                        for (var f = Ld[3 * b - 1], h = 0; h < 3 * b; h += 3)
                            (f[h] = T[(e + 4 * h) >> 2]),
                                (f[h + 1] = T[(e + (4 * h + 4)) >> 2]),
                                (f[h + 2] = T[(e + (4 * h + 8)) >> 2]);
                    else f = T.subarray(e >> 2, (e + 12 * b) >> 2);
                    Z.uniform3fv(Kd(a), f);
                }
            },
            Uc: function (a, b, e, f) {
                Z.uniform3i(Kd(a), b, e, f);
            },
            Tc: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform3iv(Kd(a), R, e >> 2, 3 * b);
                else {
                    if (96 >= b)
                        for (var f = Md[3 * b - 1], h = 0; h < 3 * b; h += 3)
                            (f[h] = R[(e + 4 * h) >> 2]),
                                (f[h + 1] = R[(e + (4 * h + 4)) >> 2]),
                                (f[h + 2] = R[(e + (4 * h + 8)) >> 2]);
                    else f = R.subarray(e >> 2, (e + 12 * b) >> 2);
                    Z.uniform3iv(Kd(a), f);
                }
            },
            Sc: function (a, b, e, f, h) {
                Z.uniform4f(Kd(a), b, e, f, h);
            },
            Rc: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform4fv(Kd(a), T, e >> 2, 4 * b);
                else {
                    if (72 >= b) {
                        var f = Ld[4 * b - 1],
                            h = T;
                        e >>= 2;
                        for (var m = 0; m < 4 * b; m += 4) {
                            var u = e + m;
                            f[m] = h[u];
                            f[m + 1] = h[u + 1];
                            f[m + 2] = h[u + 2];
                            f[m + 3] = h[u + 3];
                        }
                    } else f = T.subarray(e >> 2, (e + 16 * b) >> 2);
                    Z.uniform4fv(Kd(a), f);
                }
            },
            Fc: function (a, b, e, f, h) {
                Z.uniform4i(Kd(a), b, e, f, h);
            },
            Gc: function (a, b, e) {
                if (2 <= x.version) b && Z.uniform4iv(Kd(a), R, e >> 2, 4 * b);
                else {
                    if (72 >= b)
                        for (var f = Md[4 * b - 1], h = 0; h < 4 * b; h += 4)
                            (f[h] = R[(e + 4 * h) >> 2]),
                                (f[h + 1] = R[(e + (4 * h + 4)) >> 2]),
                                (f[h + 2] = R[(e + (4 * h + 8)) >> 2]),
                                (f[h + 3] = R[(e + (4 * h + 12)) >> 2]);
                    else f = R.subarray(e >> 2, (e + 16 * b) >> 2);
                    Z.uniform4iv(Kd(a), f);
                }
            },
            Hc: function (a, b, e, f) {
                if (2 <= x.version)
                    b && Z.uniformMatrix2fv(Kd(a), !!e, T, f >> 2, 4 * b);
                else {
                    if (72 >= b)
                        for (var h = Ld[4 * b - 1], m = 0; m < 4 * b; m += 4)
                            (h[m] = T[(f + 4 * m) >> 2]),
                                (h[m + 1] = T[(f + (4 * m + 4)) >> 2]),
                                (h[m + 2] = T[(f + (4 * m + 8)) >> 2]),
                                (h[m + 3] = T[(f + (4 * m + 12)) >> 2]);
                    else h = T.subarray(f >> 2, (f + 16 * b) >> 2);
                    Z.uniformMatrix2fv(Kd(a), !!e, h);
                }
            },
            Ic: function (a, b, e, f) {
                if (2 <= x.version)
                    b && Z.uniformMatrix3fv(Kd(a), !!e, T, f >> 2, 9 * b);
                else {
                    if (32 >= b)
                        for (var h = Ld[9 * b - 1], m = 0; m < 9 * b; m += 9)
                            (h[m] = T[(f + 4 * m) >> 2]),
                                (h[m + 1] = T[(f + (4 * m + 4)) >> 2]),
                                (h[m + 2] = T[(f + (4 * m + 8)) >> 2]),
                                (h[m + 3] = T[(f + (4 * m + 12)) >> 2]),
                                (h[m + 4] = T[(f + (4 * m + 16)) >> 2]),
                                (h[m + 5] = T[(f + (4 * m + 20)) >> 2]),
                                (h[m + 6] = T[(f + (4 * m + 24)) >> 2]),
                                (h[m + 7] = T[(f + (4 * m + 28)) >> 2]),
                                (h[m + 8] = T[(f + (4 * m + 32)) >> 2]);
                    else h = T.subarray(f >> 2, (f + 36 * b) >> 2);
                    Z.uniformMatrix3fv(Kd(a), !!e, h);
                }
            },
            Jc: function (a, b, e, f) {
                if (2 <= x.version)
                    b && Z.uniformMatrix4fv(Kd(a), !!e, T, f >> 2, 16 * b);
                else {
                    if (18 >= b) {
                        var h = Ld[16 * b - 1],
                            m = T;
                        f >>= 2;
                        for (var u = 0; u < 16 * b; u += 16) {
                            var n = f + u;
                            h[u] = m[n];
                            h[u + 1] = m[n + 1];
                            h[u + 2] = m[n + 2];
                            h[u + 3] = m[n + 3];
                            h[u + 4] = m[n + 4];
                            h[u + 5] = m[n + 5];
                            h[u + 6] = m[n + 6];
                            h[u + 7] = m[n + 7];
                            h[u + 8] = m[n + 8];
                            h[u + 9] = m[n + 9];
                            h[u + 10] = m[n + 10];
                            h[u + 11] = m[n + 11];
                            h[u + 12] = m[n + 12];
                            h[u + 13] = m[n + 13];
                            h[u + 14] = m[n + 14];
                            h[u + 15] = m[n + 15];
                        }
                    } else h = T.subarray(f >> 2, (f + 64 * b) >> 2);
                    Z.uniformMatrix4fv(Kd(a), !!e, h);
                }
            },
            Kc: function (a) {
                a = md[a];
                Z.useProgram(a);
                Z.If = a;
            },
            Lc: function (a, b) {
                Z.vertexAttrib1f(a, b);
            },
            Mc: function (a, b) {
                Z.vertexAttrib2f(a, T[b >> 2], T[(b + 4) >> 2]);
            },
            Nc: function (a, b) {
                Z.vertexAttrib3f(
                    a,
                    T[b >> 2],
                    T[(b + 4) >> 2],
                    T[(b + 8) >> 2],
                );
            },
            Oc: function (a, b) {
                Z.vertexAttrib4f(
                    a,
                    T[b >> 2],
                    T[(b + 4) >> 2],
                    T[(b + 8) >> 2],
                    T[(b + 12) >> 2],
                );
            },
            pc: function (a, b) {
                Z.vertexAttribDivisor(a, b);
            },
            qc: function (a, b, e, f, h) {
                Z.vertexAttribIPointer(a, b, e, f, h);
            },
            Pc: function (a, b, e, f, h, m) {
                Z.vertexAttribPointer(a, b, e, !!f, h, m);
            },
            Qc: function (a, b, e, f) {
                Z.viewport(a, b, e, f);
            },
            lb: function (a, b, e, f) {
                Z.waitSync(sd[a], b, (e >>> 0) + 4294967296 * f);
            },
            tb: function (a) {
                var b = K.length;
                a >>>= 0;
                if (2147483648 < a) return !1;
                for (var e = 1; 4 >= e; e *= 2) {
                    var f = b * (1 + 0.2 / e);
                    f = Math.min(f, a + 100663296);
                    var h = Math;
                    f = Math.max(a, f);
                    h = h.min.call(
                        h,
                        2147483648,
                        f + ((65536 - (f % 65536)) % 65536),
                    );
                    a: {
                        try {
                            Qa.grow((h - jb.byteLength + 65535) >>> 16);
                            nb();
                            var m = 1;
                            break a;
                        } catch (u) {}
                        m = void 0;
                    }
                    if (m) return !0;
                }
                return !1;
            },
            nb: function () {
                return x ? x.Vf : 0;
            },
            wb: function (a, b) {
                var e = 0;
                Od().forEach(function (f, h) {
                    var m = b + e;
                    h = lb[(a + 4 * h) >> 2] = m;
                    for (m = 0; m < f.length; ++m)
                        kb[h++ >> 0] = f.charCodeAt(m);
                    kb[h >> 0] = 0;
                    e += f.length + 1;
                });
                return 0;
            },
            xb: function (a, b) {
                var e = Od();
                lb[a >> 2] = e.length;
                var f = 0;
                e.forEach(function (h) {
                    f += h.length + 1;
                });
                lb[b >> 2] = f;
                return 0;
            },
            Jb: function (a) {
                if (!noExitRuntime) {
                    if (w.onExit) w.onExit(a);
                    Ra = !0;
                }
                wa(a, new Ha(a));
            },
            P: function () {
                return 52;
            },
            ob: function () {
                return 52;
            },
            Cb: function () {
                return 52;
            },
            pb: function () {
                return 70;
            },
            U: function (a, b, e, f) {
                for (var h = 0, m = 0; m < e; m++) {
                    var u = lb[b >> 2],
                        n = lb[(b + 4) >> 2];
                    b += 8;
                    for (var r = 0; r < n; r++) {
                        var v = K[u + r],
                            D = Qd[a];
                        0 === v || 10 === v
                            ? ((1 === a ? Ka : Ja)(Ta(D, 0)), (D.length = 0))
                            : D.push(v);
                    }
                    h += n;
                }
                lb[f >> 2] = h;
                return 0;
            },
            c: function () {
                return La;
            },
            l: ae,
            s: be,
            m: ce,
            K: de,
            Nb: ee,
            $: fe,
            _: ge,
            R: he,
            q: ie,
            A: je,
            n: ke,
            w: le,
            Mb: me,
            Kb: ne,
            Lb: oe,
            d: function (a) {
                La = a;
            },
            rb: function (a, b, e, f) {
                return Ud(a, b, e, f);
            },
        };
        (function () {
            function a(h) {
                w.asm = h.exports;
                Qa = w.asm.dd;
                nb();
                ob = w.asm.gd;
                qb.unshift(w.asm.ed);
                tb--;
                w.monitorRunDependencies && w.monitorRunDependencies(tb);
                0 == tb &&
                    (null !== ub && (clearInterval(ub), (ub = null)),
                    vb && ((h = vb), (vb = null), h()));
            }
            function b(h) {
                a(h.instance);
            }
            function e(h) {
                return Bb()
                    .then(function (m) {
                        return WebAssembly.instantiate(m, f);
                    })
                    .then(function (m) {
                        return m;
                    })
                    .then(h, function (m) {
                        Ja("failed to asynchronously prepare wasm: " + m);
                        Na(m);
                    });
            }
            var f = { a: pe };
            tb++;
            w.monitorRunDependencies && w.monitorRunDependencies(tb);
            if (w.instantiateWasm)
                try {
                    return w.instantiateWasm(f, a);
                } catch (h) {
                    return (
                        Ja(
                            "Module.instantiateWasm callback failed with error: " +
                                h,
                        ),
                        !1
                    );
                }
            (function () {
                return Ma ||
                    "function" != typeof WebAssembly.instantiateStreaming ||
                    wb() ||
                    yb.startsWith("file://") ||
                    Aa ||
                    "function" != typeof fetch
                    ? e(b)
                    : fetch(yb, { credentials: "same-origin" }).then(function (
                          h,
                      ) {
                          return WebAssembly.instantiateStreaming(h, f).then(
                              b,
                              function (m) {
                                  Ja("wasm streaming compile failed: " + m);
                                  Ja(
                                      "falling back to ArrayBuffer instantiation",
                                  );
                                  return e(b);
                              },
                          );
                      });
            })().catch(ea);
            return {};
        })();
        w.___wasm_call_ctors = function () {
            return (w.___wasm_call_ctors = w.asm.ed).apply(null, arguments);
        };
        var Gd = (w._malloc = function () {
                return (Gd = w._malloc = w.asm.fd).apply(null, arguments);
            }),
            Lc = (w._free = function () {
                return (Lc = w._free = w.asm.hd).apply(null, arguments);
            }),
            Kc = (w.___getTypeName = function () {
                return (Kc = w.___getTypeName = w.asm.id).apply(
                    null,
                    arguments,
                );
            });
        w.___embind_register_native_and_builtin_types = function () {
            return (w.___embind_register_native_and_builtin_types =
                w.asm.jd).apply(null, arguments);
        };
        var qe = (w._setThrew = function () {
                return (qe = w._setThrew = w.asm.kd).apply(null, arguments);
            }),
            re = (w.stackSave = function () {
                return (re = w.stackSave = w.asm.ld).apply(null, arguments);
            }),
            se = (w.stackRestore = function () {
                return (se = w.stackRestore = w.asm.md).apply(null, arguments);
            });
        w.dynCall_viji = function () {
            return (w.dynCall_viji = w.asm.nd).apply(null, arguments);
        };
        w.dynCall_vijiii = function () {
            return (w.dynCall_vijiii = w.asm.od).apply(null, arguments);
        };
        w.dynCall_viiiiij = function () {
            return (w.dynCall_viiiiij = w.asm.pd).apply(null, arguments);
        };
        w.dynCall_jiiiijiiiii = function () {
            return (w.dynCall_jiiiijiiiii = w.asm.qd).apply(null, arguments);
        };
        w.dynCall_viiij = function () {
            return (w.dynCall_viiij = w.asm.rd).apply(null, arguments);
        };
        w.dynCall_jii = function () {
            return (w.dynCall_jii = w.asm.sd).apply(null, arguments);
        };
        w.dynCall_vij = function () {
            return (w.dynCall_vij = w.asm.td).apply(null, arguments);
        };
        w.dynCall_iiij = function () {
            return (w.dynCall_iiij = w.asm.ud).apply(null, arguments);
        };
        w.dynCall_iiiij = function () {
            return (w.dynCall_iiiij = w.asm.vd).apply(null, arguments);
        };
        w.dynCall_viij = function () {
            return (w.dynCall_viij = w.asm.wd).apply(null, arguments);
        };
        w.dynCall_ji = function () {
            return (w.dynCall_ji = w.asm.xd).apply(null, arguments);
        };
        w.dynCall_iij = function () {
            return (w.dynCall_iij = w.asm.yd).apply(null, arguments);
        };
        w.dynCall_jiiiii = function () {
            return (w.dynCall_jiiiii = w.asm.zd).apply(null, arguments);
        };
        w.dynCall_jiiiiii = function () {
            return (w.dynCall_jiiiiii = w.asm.Ad).apply(null, arguments);
        };
        w.dynCall_jiiiiji = function () {
            return (w.dynCall_jiiiiji = w.asm.Bd).apply(null, arguments);
        };
        w.dynCall_iijj = function () {
            return (w.dynCall_iijj = w.asm.Cd).apply(null, arguments);
        };
        w.dynCall_iiiji = function () {
            return (w.dynCall_iiiji = w.asm.Dd).apply(null, arguments);
        };
        w.dynCall_iiji = function () {
            return (w.dynCall_iiji = w.asm.Ed).apply(null, arguments);
        };
        w.dynCall_iijjiii = function () {
            return (w.dynCall_iijjiii = w.asm.Fd).apply(null, arguments);
        };
        w.dynCall_vijjjii = function () {
            return (w.dynCall_vijjjii = w.asm.Gd).apply(null, arguments);
        };
        w.dynCall_jiji = function () {
            return (w.dynCall_jiji = w.asm.Hd).apply(null, arguments);
        };
        w.dynCall_viijii = function () {
            return (w.dynCall_viijii = w.asm.Id).apply(null, arguments);
        };
        w.dynCall_iiiiij = function () {
            return (w.dynCall_iiiiij = w.asm.Jd).apply(null, arguments);
        };
        w.dynCall_iiiiijj = function () {
            return (w.dynCall_iiiiijj = w.asm.Kd).apply(null, arguments);
        };
        w.dynCall_iiiiiijj = function () {
            return (w.dynCall_iiiiiijj = w.asm.Ld).apply(null, arguments);
        };
        function ae(a, b) {
            var e = re();
            try {
                return Db(a)(b);
            } catch (f) {
                se(e);
                if (f !== f + 0) throw f;
                qe(1, 0);
            }
        }
        function be(a, b, e) {
            var f = re();
            try {
                return Db(a)(b, e);
            } catch (h) {
                se(f);
                if (h !== h + 0) throw h;
                qe(1, 0);
            }
        }
        function ke(a, b, e, f) {
            var h = re();
            try {
                Db(a)(b, e, f);
            } catch (m) {
                se(h);
                if (m !== m + 0) throw m;
                qe(1, 0);
            }
        }
        function ce(a, b, e, f) {
            var h = re();
            try {
                return Db(a)(b, e, f);
            } catch (m) {
                se(h);
                if (m !== m + 0) throw m;
                qe(1, 0);
            }
        }
        function ie(a, b) {
            var e = re();
            try {
                Db(a)(b);
            } catch (f) {
                se(e);
                if (f !== f + 0) throw f;
                qe(1, 0);
            }
        }
        function ee(a, b, e, f, h, m) {
            var u = re();
            try {
                return Db(a)(b, e, f, h, m);
            } catch (n) {
                se(u);
                if (n !== n + 0) throw n;
                qe(1, 0);
            }
        }
        function le(a, b, e, f, h) {
            var m = re();
            try {
                Db(a)(b, e, f, h);
            } catch (u) {
                se(m);
                if (u !== u + 0) throw u;
                qe(1, 0);
            }
        }
        function fe(a, b, e, f, h, m, u) {
            var n = re();
            try {
                return Db(a)(b, e, f, h, m, u);
            } catch (r) {
                se(n);
                if (r !== r + 0) throw r;
                qe(1, 0);
            }
        }
        function de(a, b, e, f, h) {
            var m = re();
            try {
                return Db(a)(b, e, f, h);
            } catch (u) {
                se(m);
                if (u !== u + 0) throw u;
                qe(1, 0);
            }
        }
        function je(a, b, e) {
            var f = re();
            try {
                Db(a)(b, e);
            } catch (h) {
                se(f);
                if (h !== h + 0) throw h;
                qe(1, 0);
            }
        }
        function me(a, b, e, f, h, m) {
            var u = re();
            try {
                Db(a)(b, e, f, h, m);
            } catch (n) {
                se(u);
                if (n !== n + 0) throw n;
                qe(1, 0);
            }
        }
        function oe(a, b, e, f, h, m, u, n, r, v) {
            var D = re();
            try {
                Db(a)(b, e, f, h, m, u, n, r, v);
            } catch (I) {
                se(D);
                if (I !== I + 0) throw I;
                qe(1, 0);
            }
        }
        function he(a) {
            var b = re();
            try {
                Db(a)();
            } catch (e) {
                se(b);
                if (e !== e + 0) throw e;
                qe(1, 0);
            }
        }
        function ne(a, b, e, f, h, m, u) {
            var n = re();
            try {
                Db(a)(b, e, f, h, m, u);
            } catch (r) {
                se(n);
                if (r !== r + 0) throw r;
                qe(1, 0);
            }
        }
        function ge(a, b, e, f, h, m, u, n, r, v) {
            var D = re();
            try {
                return Db(a)(b, e, f, h, m, u, n, r, v);
            } catch (I) {
                se(D);
                if (I !== I + 0) throw I;
                qe(1, 0);
            }
        }
        var te;
        function Ha(a) {
            this.name = "ExitStatus";
            this.message = "Program terminated with exit(" + a + ")";
            this.status = a;
        }
        vb = function ue() {
            te || ve();
            te || (vb = ue);
        };
        function ve() {
            function a() {
                if (!te && ((te = !0), (w.calledRun = !0), !Ra)) {
                    Cb(qb);
                    aa(w);
                    if (w.onRuntimeInitialized) w.onRuntimeInitialized();
                    if (w.postRun)
                        for (
                            "function" == typeof w.postRun &&
                            (w.postRun = [w.postRun]);
                            w.postRun.length;

                        ) {
                            var b = w.postRun.shift();
                            rb.unshift(b);
                        }
                    Cb(rb);
                }
            }
            if (!(0 < tb)) {
                if (w.preRun)
                    for (
                        "function" == typeof w.preRun &&
                        (w.preRun = [w.preRun]);
                        w.preRun.length;

                    )
                        sb();
                Cb(pb);
                0 < tb ||
                    (w.setStatus
                        ? (w.setStatus("Running..."),
                          setTimeout(function () {
                              setTimeout(function () {
                                  w.setStatus("");
                              }, 1);
                              a();
                          }, 1))
                        : a());
            }
        }
        w.run = ve;
        if (w.preInit)
            for (
                "function" == typeof w.preInit && (w.preInit = [w.preInit]);
                0 < w.preInit.length;

            )
                w.preInit.pop()();
        ve();

        return CanvasKitInit.ready;
    };
})();
if (typeof exports === "object" && typeof module === "object")
    module.exports = CanvasKitInit;
else if (typeof define === "function" && define["amd"])
    define([], function () {
        return CanvasKitInit;
    });
else if (typeof exports === "object") exports["CanvasKitInit"] = CanvasKitInit;

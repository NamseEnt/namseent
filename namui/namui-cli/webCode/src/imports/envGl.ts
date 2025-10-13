import { Exports } from "../exports";
import {
    computeUnpackAlignedImageSize,
    getSizePerPixel,
} from "./getSizePerPixel";

export function envGl({
    canvas = new OffscreenCanvas(0, 0),
    memory,
    exports,
}: {
    canvas: OffscreenCanvas | undefined;
    memory: WebAssembly.Memory;
    exports: () => Exports;
}) {
    const webgl = canvas.getContext("webgl2");
    const stringCache = new Map<number, number>();

    const webglBufferMap = new Map<number, WebGLBuffer>();
    let nextBufferId = 1;

    type ProgramInfo = {
        program: WebGLProgram;
        nameToUniformLocation: Map<string, WebGLUniformLocation>;
        uniformLocationNameToId: Map<string, number>;
        idToUniformLocation: Map<number, WebGLUniformLocation>;
    };
    const programInfos = new Map<number, ProgramInfo>();
    let nextProgramId = 1;

    const webglShaderMap = new Map<number, WebGLShader>();
    let nextShaderId = 1;

    const webglTextureMap = new Map<number, WebGLTexture>();
    let nextTextureId = 1;

    const webglRenderbufferMap = new Map<number, WebGLRenderbuffer>();
    let nextRenderbufferId = 1;

    const webglSamplerMap = new Map<number, WebGLSampler>();
    let nextSamplerId = 1;

    const webglFramebufferMap = new Map<number, WebGLFramebuffer>();
    let nextFramebufferId = 1;

    const webglVertexArrayMap = new Map<number, WebGLVertexArrayObject>();

    const memoryView = () => new DataView(memory.buffer);

    let currentProgramInfo: ProgramInfo | undefined;

    function stringToNewUTF8(string: string) {
        const bytes = new TextEncoder().encode(string);
        const ptr = exports().malloc(bytes.length + 1);
        const buffer = new Uint8Array(memory.buffer);
        buffer.set(bytes, ptr);
        buffer[ptr + bytes.length] = 0;
        return ptr;
    }

    const extensionStringCache = new Map<number, number>();

    function glGetString(name: number) {
        if (!webgl) {
            throw new Error("webgl is not set");
        }
        let ret = stringCache.get(name);
        if (ret) {
            return ret;
        }
        switch (name) {
            case 7939 /* GL_EXTENSIONS */:
                ret = stringToNewUTF8(
                    webgl.getSupportedExtensions()!.join(" "),
                );
                break;
            case 7936 /* GL_VENDOR */:
            case 7937 /* GL_RENDERER */:
                const paramter = webgl.getParameter(name);

                if (!paramter) {
                    // This occurs e.g. if one attempts GL_UNMASKED_VENDOR_WEBGL when it is not supported.
                    throw new Error(
                        `GL_INVALID_ENUM in glGetString: Received empty parameter for query name ${name}!`,
                    );
                }

                ret = stringToNewUTF8(paramter);
                break;
            case 37445 /* UNMASKED_VENDOR_WEBGL */:
                {
                    const debugInfo = webgl.getExtension(
                        "WEBGL_debug_renderer_info",
                    );
                    if (!debugInfo) {
                        throw new Error(
                            "GL_INVALID_ENUM in glGetString: WEBGL_debug_renderer_info not supported!",
                        );
                    }
                    const vendor = webgl.getParameter(
                        debugInfo.UNMASKED_VENDOR_WEBGL,
                    );
                    if (!vendor) {
                        throw new Error(
                            "GL_INVALID_ENUM in glGetString: Received empty parameter for query name UNMASKED_VENDOR_WEBGL!",
                        );
                    }
                    ret = stringToNewUTF8(vendor);
                }
                break;
            case 37446 /* UNMASKED_RENDERER_WEBGL */:
                {
                    const debugInfo = webgl.getExtension(
                        "WEBGL_debug_renderer_info",
                    );
                    if (!debugInfo) {
                        throw new Error(
                            "GL_INVALID_ENUM in glGetString: WEBGL_debug_renderer_info not supported!",
                        );
                    }
                    const renderer = webgl.getParameter(
                        debugInfo.UNMASKED_RENDERER_WEBGL,
                    );
                    if (!renderer) {
                        throw new Error(
                            "GL_INVALID_ENUM in glGetString: Received empty parameter for query name UNMASKED_RENDERER_WEBGL!",
                        );
                    }
                    ret = stringToNewUTF8(renderer);
                }
                break;
            case 7938 /* GL_VERSION */:
                const glVersion = `OpenGL ES 3.0 (${webgl.getParameter(name)})`;
                ret = stringToNewUTF8(glVersion);
                break;
            case 35724 /* GL_SHADING_LANGUAGE_VERSION */:
                let glslVersion = webgl.getParameter(
                    35724 /*GL_SHADING_LANGUAGE_VERSION*/,
                );
                // extract the version number 'N.M' from the string 'WebGL GLSL ES N.M ...'
                const ver_re = /^WebGL GLSL ES ([0-9]\.[0-9][0-9]?)(?:$| .*)/;
                const ver_num = glslVersion.match(ver_re);
                if (ver_num !== null) {
                    if (ver_num[1].length == 3) ver_num[1] = ver_num[1] + "0"; // ensure minor version has 2 digits
                    glslVersion = `OpenGL ES GLSL ES ${ver_num[1]} (${glslVersion})`;
                }
                ret = stringToNewUTF8(glslVersion);
                break;
            default:
                throw new Error(
                    `GL_INVALID_ENUM in glGetString: Unknown parameter ${name}!`,
                );
        }
        stringCache.set(name, ret);
        return ret;
    }

    /**
     * const GLubyte *glGetStringi(
     *     GLenum name,
     *     GLuint index
     * );
     *
     * name: Specifies a symbolic constant, one of GL_VENDOR, GL_RENDERER, GL_VERSION,
     * or GL_SHADING_LANGUAGE_VERSION. Additionally, glGetStringi accepts the GL_EXTENSIONS token.
     *
     * index: For glGetStringi, specifies the index of the string to return.
     */
    function glGetStringi(pname: number, index: number) {
        if (!webgl) {
            throw new Error("webgl is not set");
        }
        switch (pname) {
            case 0x1f03 /* GL_EXTENSIONS */: {
                const extensions = webgl.getSupportedExtensions();
                if (!extensions) {
                    throw new Error("No extensions found");
                }
                if (extensionStringCache.has(index)) {
                    return extensionStringCache.get(index)!;
                }
                if (index >= extensions.length) {
                    return 0;
                }
                const ret = stringToNewUTF8(extensions[index]);
                extensionStringCache.set(index, ret);
                return ret;
            }
            default:
                throw new Error(
                    `GL_INVALID_ENUM in glGetStringi: Unknown parameter ${pname.toString(
                        16,
                    )}!`,
                );
        }
    }

    /**
     * @param pname
     *  GLenum
     * @param paramsPtr
     *  GLint
     * @returns
     */
    function glGetIntegerv(pname: number, paramsPtr: number) {
        if (!webgl) {
            throw new Error("webgl is not set");
        }
        switch (pname) {
            case 0x821d: // GL_NUM_EXTENSIONS
                {
                    const value = webgl.getSupportedExtensions()?.length || 0;
                    memoryView().setInt32(paramsPtr, value, true);
                }
                break;
            default:
                {
                    const value = webgl.getParameter(pname);
                    memoryView().setInt32(paramsPtr, value, true);
                }
                break;
        }
    }

    function glGetQueryiv(target: number, pname: number, paramsPtr: number) {
        if (!webgl) {
            throw new Error("webgl is not set");
        }
        console.log("glGetQueryiv", target, pname, paramsPtr);
        throw new Error("not implemented");
    }

    function glGetQueryObjectuiv(
        target: number,
        pname: number,
        paramsPtr: number,
    ) {
        if (!webgl) {
            throw new Error("webgl is not set");
        }
        console.log("glGetQueryObjectuiv", target, pname, paramsPtr);
        throw new Error("not implemented");
    }

    function glGenQueries(n: number, idsPtr: number) {
        if (!webgl) {
            throw new Error("webgl is not set");
        }
        console.log("glGenQueries", n, idsPtr);
        throw new Error("not implemented");
    }

    const notImplementeds = [
        "emscripten_glEndQuery",
        "emscripten_glDeleteQueries",
        "emscripten_glBeginQuery",
        "emscripten_glBeginQueryEXT",
        "emscripten_glGenQueriesEXT",
        "emscripten_glDeleteQueriesEXT",
        "emscripten_glIsQueryEXT",
        "emscripten_glBeginQueryEXT",
        "emscripten_glEndQueryEXT",
        "emscripten_glQueryCounterEXT",
        "emscripten_glGetQueryivEXT",
        "emscripten_glGetQueryObjectivEXT",
        "emscripten_glGetQueryObjectuivEXT",
        "emscripten_glGetQueryObjecti64vEXT",
        "emscripten_glGetQueryObjectui64vEXT",
    ].reduce((acc, x) => {
        acc[x] = () => {
            throw new Error("not implemented");
        };
        return acc;
    }, {} as Record<string, () => void>);

    return {
        ...notImplementeds,
        glGetStringi,
        emscripten_glGetStringi: glGetStringi,
        glGetIntegerv,
        emscripten_glGetIntegerv: glGetIntegerv,
        glGetString,
        emscripten_glGetString: glGetString,
        emscripten_glGetQueryiv: glGetQueryiv,
        emscripten_glGetQueryObjectuiv: glGetQueryObjectuiv,
        emscripten_glGenQueries: glGenQueries,
        /**
         * void glTexSubImage2D(
         * GLenum target,
         * GLint level,
         * GLint xoffset,
         * GLint yoffset,
         * GLsizei width,
         * GLsizei height,
         * GLenum format,
         * GLenum type,
         * const void * pixels);
         *
         * Parameters:
         * target: Specifies the target to which the texture is bound for glTexSubImage2D. Must be GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_NEGATIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y, GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, GL_TEXTURE_CUBE_MAP_POSITIVE_Z, GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, or GL_TEXTURE_1D_ARRAY.
         *
         * level: Specifies the level-of-detail number. Level 0 is the base image level. Level n is the nth mipmap reduction image.
         *
         * xoffset: Specifies a texel offset in the x direction within the texture array.
         *
         * yoffset: Specifies a texel offset in the y direction within the texture array.
         *
         * width: Specifies the width of the texture subimage.
         *
         * height: Specifies the height of the texture subimage.
         *
         * format: Specifies the format of the pixel data. The following symbolic values are accepted: GL_RED, GL_RG, GL_RGB, GL_BGR, GL_RGBA, GL_BGRA, GL_DEPTH_COMPONENT, and GL_STENCIL_INDEX.
         *
         * type: Specifies the data type of the pixel data. The following symbolic values are accepted: GL_UNSIGNED_BYTE, GL_BYTE, GL_UNSIGNED_SHORT, GL_SHORT, GL_UNSIGNED_INT, GL_INT, GL_FLOAT, GL_UNSIGNED_BYTE_3_3_2, GL_UNSIGNED_BYTE_2_3_3_REV, GL_UNSIGNED_SHORT_5_6_5, GL_UNSIGNED_SHORT_5_6_5_REV, GL_UNSIGNED_SHORT_4_4_4_4, GL_UNSIGNED_SHORT_4_4_4_4_REV, GL_UNSIGNED_SHORT_5_5_5_1, GL_UNSIGNED_SHORT_1_5_5_5_REV, GL_UNSIGNED_INT_8_8_8_8, GL_UNSIGNED_INT_8_8_8_8_REV, GL_UNSIGNED_INT_10_10_10_2, and GL_UNSIGNED_INT_2_10_10_10_REV.
         *
         * pixels: Specifies a pointer to the image data in memory.
         */
        emscripten_glTexSubImage2D: (
            target: number,
            level: number,
            xoffset: number,
            yoffset: number,
            width: number,
            height: number,
            format: number,
            type: number,
            pixelsPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }

            const sizePerPixel = getSizePerPixel(webgl, format, type);
            const bytes = computeUnpackAlignedImageSize(
                webgl,
                width,
                height,
                sizePerPixel,
            );
            const pixels = new Uint8Array(memory.buffer, pixelsPtr, bytes);
            if (pixels.byteLength !== bytes) {
                throw new Error(
                    `Expected ${bytes} bytes but got ${pixels.byteLength} bytes`,
                );
            }
            webgl.texSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                type,
                pixels,
            );
        },
        emscripten_glTexParameteriv: () => {
            throw new Error("not implemented");
            // return webgl!.texParameteriv();
        },
        /**
         * void glTexParameteri(
         *  GLenum target,
         *  GLenum pname,
         *  GLint param
         * );
         */
        emscripten_glTexParameteri: (
            target: number,
            pname: number,
            param: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            return webgl.texParameteri(target, pname, param);
        },
        emscripten_glTexParameterfv: () => {
            throw new Error("not implemented");
            // return webgl!.texParameterfv();
        },
        emscripten_glTexParameterf: () => {
            throw new Error("not implemented");
            // return webgl!.texParameterf();
        },
        emscripten_glTexImage2D: () => {
            throw new Error("not implemented");
            // return webgl!.texImage2D();
        },
        emscripten_glStencilOpSeparate: () => {
            throw new Error("not implemented");
            // return webgl!.stencilOpSeparate();
        },
        emscripten_glStencilOp: () => {
            throw new Error("not implemented");
            // return webgl!.stencilOp();
        },
        emscripten_glStencilMaskSeparate: () => {
            throw new Error("not implemented");
            // return webgl!.stencilMaskSeparate();
        },
        emscripten_glStencilMask: () => {
            throw new Error("not implemented");
            // return webgl!.stencilMask();
        },
        emscripten_glStencilFuncSeparate: () => {
            throw new Error("not implemented");
            // return webgl!.stencilFuncSeparate();
        },
        emscripten_glStencilFunc: () => {
            throw new Error("not implemented");
            // return webgl!.stencilFunc();
        },
        /**
         * void glShaderSource(
         *   GLuint shader,
         *   GLsizei count,
         *   const GLchar **string,
         *   const GLint *length);
         */
        emscripten_glShaderSource: (
            shaderId: number,
            count: number,
            stringPtr: number,
            lengthPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const shader = webglShaderMap.get(shaderId);
            if (!shader) {
                throw new Error("shader not found");
            }
            const decoder = new TextDecoder();
            let source = "";
            for (let i = 0; i < count; i++) {
                const ptr = memoryView().getUint32(stringPtr + i * 4, true);
                const length = memoryView().getUint32(lengthPtr + i * 4, true);

                const bytes = new Uint8Array(memory.buffer, ptr, length);

                // NOTE: I cannot use bytes directly. that makes error -> TypeError: Failed to execute 'decode' on 'TextDecoder': The provided ArrayBufferView value must not be shared.
                const copied = new ArrayBuffer(bytes.byteLength);
                new Uint8Array(copied).set(bytes);

                source += decoder.decode(copied, {
                    stream: true,
                });
            }
            source += decoder.decode();
            webgl.shaderSource(shader, source);
        },
        emscripten_glScissor: () => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            return webgl.scissor;
        },
        emscripten_glReadPixels: () => {
            throw new Error("not implemented");
            // return webgl!.readPixels();
        },
        emscripten_glPixelStorei: (pname: number, param: number) => {
            return webgl!.pixelStorei(pname, param);
        },
        /**
         * void glLinkProgram(GLuint program);
         */
        emscripten_glLinkProgram: (programId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const programInfo = programInfos.get(programId);
            if (!programInfo) {
                throw new Error("program not found");
            }
            webgl.linkProgram(programInfo.program);
        },
        emscripten_glLineWidth: (width: number) => {
            return webgl!.lineWidth(width);
        },
        emscripten_glIsTexture: () => {
            throw new Error("not implemented");
            // return webgl!.isTexture();
        },
        /**
         * GLint glGetUniformLocation(
         *  GLuint program,
         *  const GLchar *name
         * );
         */
        emscripten_glGetUniformLocation: (
            programId: number,
            namePtr: number,
        ): number => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const programInfo = programInfos.get(programId);
            if (!programInfo) {
                throw new Error("program not found");
            }
            const nameBytes = [];
            while (true) {
                const byte = memoryView().getUint8(namePtr + nameBytes.length);
                if (byte === 0) {
                    break;
                }
                nameBytes.push(byte);
            }
            const name = new TextDecoder().decode(new Uint8Array(nameBytes));
            const cachedId = programInfo.uniformLocationNameToId.get(name);
            if (cachedId !== undefined) {
                return cachedId;
            }

            const location = webgl.getUniformLocation(
                programInfo.program,
                name,
            );
            if (!location) {
                return -1;
            }
            programInfo.nameToUniformLocation.set(name, location);
            const id = programInfo.nameToUniformLocation.size;
            programInfo.uniformLocationNameToId.set(name, id);
            programInfo.idToUniformLocation.set(id, location);
            return id;
        },
        /**
         * void glGetShaderiv(
         *  GLuint shader,
         *  GLenum pname,
         *  GLint *params);
         */
        emscripten_glGetShaderiv: (
            shaderId: number,
            pname: number,
            paramsPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const shader = webglShaderMap.get(shaderId);
            if (!shader) {
                throw new Error("shader not found");
            }

            switch (pname) {
                case 35716: // INFO_LOG_LENGTH
                    {
                        const log = webgl.getShaderInfoLog(shader);
                        memoryView().setInt32(
                            paramsPtr,
                            log ? log.length + 1 : 0,
                            true,
                        );
                    }
                    break;
                case 35720: // SHADER_SOURCE_LENGTH
                    {
                        throw new Error("not implemented");
                    }
                    break;
                default: {
                    const value = webgl.getShaderParameter(shader, pname);
                    memoryView().setInt32(paramsPtr, value, true);
                }
            }
        },
        /**
         * void glGetShaderInfoLog(
         *  GLuint shader,
         *  GLsizei maxLength,
         *  GLsizei *length,
         *  GLchar *infoLog
         * );
         */
        emscripten_glGetShaderInfoLog: (
            shaderId: number,
            maxLength: number,
            lengthPtr: number,
            infoLogPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const shader = webglShaderMap.get(shaderId);
            if (!shader) {
                throw new Error("shader not found");
            }
            let log = webgl.getShaderInfoLog(shader);
            if (!log) {
                return memoryView().setInt32(lengthPtr, 0, true);
            }

            if (log.length + 1 > maxLength) {
                log = log.slice(0, maxLength - 1);
            }

            const bytes = new TextEncoder().encode(log);
            const buffer = new Uint8Array(memory.buffer);
            buffer.set(bytes, infoLogPtr);
            // add null terminator
            buffer[infoLogPtr + bytes.length] = 0;
            memoryView().setInt32(lengthPtr, bytes.length, true);
        },
        /**
         * void glGetProgramiv(
         *  GLuint program,
         *  GLenum pname,
         *  GLint *params
         * );
         */
        emscripten_glGetProgramiv: (
            programId: number,
            pname: number,
            paramsPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const programInfo = programInfos.get(programId);
            if (!programInfo) {
                throw new Error("program not found");
            }

            const value = webgl.getProgramParameter(programInfo.program, pname);
            memoryView().setInt32(paramsPtr, value, true);
        },
        emscripten_glGetProgramInfoLog: () => {
            throw new Error("not implemented");
            // return webgl!.getProgramInfoLog();
        },
        /**
         * void glGetFloatv(
         *     GLenum pname,
         *     GLfloat *params
         * );
         *
         * pname: Specifies the parameter value to be returned. The accepted symbolic constants are listed below.
         * params: Returns the value or values of the specified parameter.
         */
        emscripten_glGetFloatv: (pname: number, paramsPtr: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            switch (pname) {
                case 0x84ff /* GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT */:
                    {
                        const ext = webgl.getExtension(
                            "EXT_texture_filter_anisotropic",
                        );
                        if (!ext) {
                            throw new Error(
                                "EXT_texture_filter_anisotropic not supported",
                            );
                        }

                        const value = webgl.getParameter(
                            ext.MAX_TEXTURE_MAX_ANISOTROPY_EXT,
                        );
                        memoryView().setFloat32(paramsPtr, value, true);
                    }
                    break;
                default:
                    {
                        const value = webgl.getParameter(pname);
                        memoryView().setFloat32(paramsPtr, value, true);
                    }
                    break;
            }
        },
        emscripten_glGetError: () => {
            return webgl!.getError();
        },
        emscripten_glGetBufferParameteriv: () => {
            throw new Error("not implemented");
            // return webgl!.getBufferParameteriv();
        },
        /**
         * void glGenTextures(
         *  GLsizei n,
         *  GLuint * textures
         * );
         *
         * n: Specifies the number of texture names to be generated.
         * textures: Specifies an array in which the generated texture names are stored.
         */
        emscripten_glGenTextures: (n: number, texturesPtr: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const texture = webgl.createTexture();
                if (!texture) {
                    throw new Error("Failed to create texture");
                }
                const textureId = nextTextureId++;
                webglTextureMap.set(textureId, texture);
                memoryView().setUint32(texturesPtr + i * 4, textureId, true);
            }
        },
        /**
         * @param n
         *  GLsizei
         *  Specifies the number of buffer object names to be generated.
         *
         *  @param buffers
         *  GLuint *
         *  Specifies an array in which the generated buffer object names are stored.
         */
        emscripten_glGenBuffers: (n: number, buffersPtr: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const buffer = webgl.createBuffer();
                if (!buffer) {
                    throw new Error("Failed to create buffer");
                }
                const bufferId = nextBufferId++;
                webglBufferMap.set(bufferId, buffer);
                memoryView().setUint32(buffersPtr + i * 4, bufferId, true);
            }
        },
        emscripten_glFrontFace: (mode: number) => {
            return webgl!.frontFace(mode);
        },
        emscripten_glFlush: () => {
            return webgl!.flush();
        },
        emscripten_glFinish: () => {
            return webgl!.finish();
        },
        emscripten_glEnableVertexAttribArray: (index: number) => {
            return webgl!.enableVertexAttribArray(index);
        },
        emscripten_glEnable: (cap: number) => {
            return webgl!.enable(cap);
        },
        emscripten_glDrawElements: (
            mode: number,
            count: number,
            type: number,
            offset: number,
        ) => {
            return webgl!.drawElements(mode, count, type, offset);
        },
        emscripten_glDrawArrays: (
            mode: number,
            first: number,
            count: number,
        ) => {
            return webgl!.drawArrays(mode, first, count);
        },
        emscripten_glDisableVertexAttribArray: (index: number) => {
            return webgl!.disableVertexAttribArray(index);
        },
        emscripten_glDisable: (cap: number) => {
            return webgl!.disable(cap);
        },
        emscripten_glDepthMask: (flag: number) => {
            return webgl!.depthMask(!!flag);
        },
        /**
         * void glDeleteTextures(
         *  GLsizei n,
         *  const GLuint * textures
         * );
         */
        emscripten_glDeleteTextures: (n: number, textureIdsPtr: number) => {
            if (!webgl) {
                throw new Error("WebGL context is not available");
            }
            for (let i = 0; i < n; i++) {
                const textureId = memoryView().getUint32(
                    textureIdsPtr + i * 4,
                    true,
                );
                const texture = webglTextureMap.get(textureId);
                if (!texture) {
                    throw new Error("Texture not found");
                }
                webgl.deleteTexture(texture);
                webglTextureMap.delete(textureId);
            }
        },
        /**
         * void glDeleteShader(GLuint shader);
         */
        emscripten_glDeleteShader: (shaderId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const shader = webglShaderMap.get(shaderId);
            if (!shader) {
                throw new Error("shader not found");
            }
            webgl.deleteShader(shader);
            webglShaderMap.delete(shaderId);
        },
        /**
         * void glDeleteProgram(GLuint program);
         */
        emscripten_glDeleteProgram: (programId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const programInfo = programInfos.get(programId);
            if (!programInfo) {
                throw new Error("program not found");
            }
            webgl.deleteProgram(programInfo.program);
            programInfos.delete(programId);
            if (currentProgramInfo === programInfo) {
                currentProgramInfo = undefined;
            }
        },
        /**
         * void glDeleteBuffers(
         *  GLsizei n,
         *  const GLuint * buffers
         * );
         */
        emscripten_glDeleteBuffers: (n: number, buffersPtr: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const bufferId = memoryView().getUint32(
                    buffersPtr + i * 4,
                    true,
                );
                const buffer = webglBufferMap.get(bufferId);
                if (!buffer) {
                    throw new Error("buffer not found");
                }
                webgl.deleteBuffer(buffer);
                webglBufferMap.delete(bufferId);
            }
        },
        emscripten_glCullFace: (mode: number) => {
            return webgl!.cullFace(mode);
        },
        /**
         * GLuint glCreateShader(
         *  GLenum type
         * );
         */
        emscripten_glCreateShader: (type: number): number => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const shader = webgl.createShader(type);
            if (!shader) {
                throw new Error("Failed to create shader");
            }
            const shaderId = nextShaderId++;
            webglShaderMap.set(shaderId, shader);
            return shaderId;
        },
        /**
         * GLuint glCreateProgram(void);
         */
        emscripten_glCreateProgram: (): number => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const program = webgl.createProgram();
            if (!program) {
                throw new Error("Failed to create program");
            }
            const programId = nextProgramId++;
            programInfos.set(programId, {
                program,
                nameToUniformLocation: new Map(),
                uniformLocationNameToId: new Map(),
                idToUniformLocation: new Map(),
            });
            return programId;
        },
        emscripten_glCopyTexSubImage2D: (
            target: number,
            level: number,
            xoffset: number,
            yoffset: number,
            x: number,
            y: number,
            width: number,
            height: number,
        ) => {
            return webgl!.copyTexSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                x,
                y,
                width,
                height,
            );
        },
        emscripten_glCompressedTexSubImage2D: (
            target: number,
            level: number,
            xoffset: number,
            yoffset: number,
            width: number,
            height: number,
            format: number,
            imageSize: number,
            offset: number,
        ) => {
            return webgl!.compressedTexSubImage2D(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                imageSize,
                offset,
            );
        },
        emscripten_glCompressedTexImage2D: (
            target: number,
            level: number,
            internalformat: number,
            width: number,
            height: number,
            border: number,
            imageSize: number,
            offset: number,
        ) => {
            return webgl!.compressedTexImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                imageSize,
                offset,
            );
        },
        /**
         * void glCompileShader(GLuint shader);
         */
        emscripten_glCompileShader: (shaderId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const shader = webglShaderMap.get(shaderId);
            if (!shader) {
                throw new Error("shader not found");
            }
            webgl.compileShader(shader);
        },
        emscripten_glColorMask: (
            red: number,
            green: number,
            blue: number,
            alpha: number,
        ) => {
            return webgl!.colorMask(!!red, !!green, !!blue, !!alpha);
        },
        emscripten_glClearStencil: (s: number) => {
            return webgl!.clearStencil(s);
        },
        emscripten_glClearColor: (
            red: number,
            green: number,
            blue: number,
            alpha: number,
        ) => {
            return webgl!.clearColor(red, green, blue, alpha);
        },
        emscripten_glClear: (mask: number) => {
            return webgl!.clear(mask);
        },
        /**
         * void glBufferSubData(
         *  GLenum target,
         *  GLintptr offset,
         *  GLsizeiptr size,
         *  const void * data);
         */
        emscripten_glBufferSubData: (
            target: number,
            offset: number,
            size: number,
            dataPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const srcData = new Uint8Array(memory.buffer, dataPtr, size);
            webgl.bufferSubData(target, offset, srcData, 0, size);
        },
        /**
         * void glBufferData(
         *  GLenum target,
         *  GLsizeiptr size,
         *  const void * data,
         *  GLenum usage);
         */
        emscripten_glBufferData: (
            target: number,
            size: number,
            dataPtr: number,
            usage: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }

            if (!dataPtr) {
                return webgl.bufferData(target, size, usage);
            }

            const srcData = new Uint8Array(memory.buffer, dataPtr, size);
            webgl.bufferData(target, srcData, usage);
        },
        emscripten_glBlendFunc: webgl?.blendFunc.bind(webgl) || (() => {}),
        emscripten_glBlendEquation:
            webgl?.blendEquation.bind(webgl) || (() => {}),
        emscripten_glBlendColor: webgl?.blendColor.bind(webgl) || (() => {}),
        /**
         * void glBindTexture(
         *  GLenum target,
         *  GLuint texture
         * );
         *
         * target: Specifies the target to which the texture is bound. Must be one of GL_TEXTURE_1D, GL_TEXTURE_2D, GL_TEXTURE_3D, GL_TEXTURE_1D_ARRAY, GL_TEXTURE_2D_ARRAY, GL_TEXTURE_RECTANGLE, GL_TEXTURE_CUBE_MAP, GL_TEXTURE_CUBE_MAP_ARRAY, GL_TEXTURE_BUFFER, GL_TEXTURE_2D_MULTISAMPLE, or GL_TEXTURE_2D_MULTISAMPLE_ARRAY.
         * texture: Specifies the name of a texture.
         */
        emscripten_glBindTexture: (target: number, textureId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (textureId === 0) {
                return webgl.bindTexture(target, null);
            }
            const texture = webglTextureMap.get(textureId);
            if (!texture) {
                throw new Error("texture not found");
            }
            webgl.bindTexture(target, texture);
        },
        /**
         * void glBindBuffer(GLenum target, GLuint buffer);
         *
         */
        emscripten_glBindBuffer: (target: number, bufferId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const buffer = webglBufferMap.get(bufferId);
            if (!buffer) {
                throw new Error("buffer not found");
            }
            webgl.bindBuffer(target, buffer);
        },
        /**
         * void glBindAttribLocation(
         *  GLuint program,
         *  GLuint index,
         *  const GLchar *name
         * );
         */
        emscripten_glBindAttribLocation: (
            programId: number,
            index: number,
            namePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const programInfo = programInfos.get(programId);
            if (!programInfo) {
                throw new Error("program not found");
            }
            const nameBytes = [];
            while (true) {
                const byte = memoryView().getUint8(namePtr + nameBytes.length);
                if (byte === 0) {
                    break;
                }
                nameBytes.push(byte);
            }
            const name = new TextDecoder().decode(new Uint8Array(nameBytes));
            webgl.bindAttribLocation(programInfo.program, index, name);
        },
        /**
         * void glAttachShader(
         *  GLuint program,
         *  GLuint shader
         * );
         */
        emscripten_glAttachShader: (programId: number, shaderId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const programInfo = programInfos.get(programId);
            if (!programInfo) {
                throw new Error("program not found");
            }
            const shader = webglShaderMap.get(shaderId);
            if (!shader) {
                throw new Error("shader not found");
            }
            webgl.attachShader(programInfo.program, shader);
        },
        emscripten_glActiveTexture:
            webgl?.activeTexture.bind(webgl) || (() => {}),
        /**
         * void glUniform1f(GLint location, GLfloat v0);
         * void glUniform2f(GLint location, GLfloat v0, GLfloat v1);
         * void glUniform3f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2);
         * void glUniform4f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2, GLfloat v3);
         * void glUniform1i(GLint location, GLint v0);
         * void glUniform2i(GLint location, GLint v0, GLint v1);
         * void glUniform3i(GLint location, GLint v0, GLint v1, GLint v2);
         * void glUniform4i(GLint location, GLint v0, GLint v1, GLint v2, GLint v3);
         * void glUniform1ui(GLint location, GLuint v0);
         * void glUniform2ui(GLint location, GLuint v0, GLuint v1);
         * void glUniform3ui(GLint location, GLuint v0, GLuint v1, GLuint v2);
         * void glUniform4ui(GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3);
         * void glUniform1fv(GLint location, GLsizei count, const GLfloat *value);
         * void glUniform2fv(GLint location, GLsizei count, const GLfloat *value);
         * void glUniform3fv(GLint location, GLsizei count, const GLfloat *value);
         * void glUniform4fv(GLint location, GLsizei count, const GLfloat *value);
         * void glUniform1iv(GLint location, GLsizei count, const GLint *value);
         * void glUniform2iv(GLint location, GLsizei count, const GLint *value);
         * void glUniform3iv(GLint location, GLsizei count, const GLint *value);
         * void glUniform4iv(GLint location, GLsizei count, const GLint *value);
         * void glUniform1uiv(GLint location, GLsizei count, const GLuint *value);
         * void glUniform2uiv(GLint location, GLsizei count, const GLuint *value);
         * void glUniform3uiv(GLint location, GLsizei count, const GLuint *value);
         * void glUniform4uiv(GLint location, GLsizei count, const GLuint *value);
         * void glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix2x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix3x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix2x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix4x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix3x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         * void glUniformMatrix4x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
         */
        emscripten_glUniform1f: (locationId: number, v0: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform1f(uniformLocation, v0);
        },
        emscripten_glUniform2f: (
            locationId: number,
            v0: number,
            v1: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform2f(uniformLocation, v0, v1);
        },
        emscripten_glUniform3f: (
            locationId: number,
            v0: number,
            v1: number,
            v2: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform3f(uniformLocation, v0, v1, v2);
        },
        emscripten_glUniform4f: (
            locationId: number,
            v0: number,
            v1: number,
            v2: number,
            v3: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform4f(uniformLocation, v0, v1, v2, v3);
        },
        emscripten_glUniform1i: (locationId: number, v0: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform1i(uniformLocation, v0);
        },
        emscripten_glUniform2i: (
            locationId: number,
            v0: number,
            v1: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform2i(uniformLocation, v0, v1);
        },
        emscripten_glUniform3i: (
            locationId: number,
            v0: number,
            v1: number,
            v2: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform3i(uniformLocation, v0, v1, v2);
        },
        emscripten_glUniform4i: (
            locationId: number,
            v0: number,
            v1: number,
            v2: number,
            v3: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform4i(uniformLocation, v0, v1, v2, v3);
        },
        emscripten_glUniform1ui: (locationId: number, v0: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform1ui(uniformLocation, v0);
        },
        emscripten_glUniform2ui: (
            locationId: number,
            v0: number,
            v1: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform2ui(uniformLocation, v0, v1);
        },
        emscripten_glUniform3ui: (
            locationId: number,
            v0: number,
            v1: number,
            v2: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform3ui(uniformLocation, v0, v1, v2);
        },
        emscripten_glUniform4ui: (
            locationId: number,
            v0: number,
            v1: number,
            v2: number,
            v3: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            webgl.uniform4ui(uniformLocation, v0, v1, v2, v3);
        },
        emscripten_glUniform1fv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count);
            webgl.uniform1fv(uniformLocation, value);
        },
        emscripten_glUniform2fv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 2);
            webgl.uniform2fv(uniformLocation, value);
        },
        emscripten_glUniform3fv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 3);
            webgl.uniform3fv(uniformLocation, value);
        },
        emscripten_glUniform4fv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 4);
            webgl.uniform4fv(uniformLocation, value);
        },
        emscripten_glUniform1iv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Int32Array(memory.buffer, valuePtr, count);
            webgl.uniform1iv(uniformLocation, value);
        },
        emscripten_glUniform2iv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Int32Array(memory.buffer, valuePtr, count * 2);
            webgl.uniform2iv(uniformLocation, value);
        },
        emscripten_glUniform3iv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Int32Array(memory.buffer, valuePtr, count * 3);
            webgl.uniform3iv(uniformLocation, value);
        },
        emscripten_glUniform4iv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Int32Array(memory.buffer, valuePtr, count * 4);
            webgl.uniform4iv(uniformLocation, value);
        },
        emscripten_glUniform1uiv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Uint32Array(memory.buffer, valuePtr, count);
            webgl.uniform1uiv(uniformLocation, value);
        },
        emscripten_glUniform2uiv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Uint32Array(memory.buffer, valuePtr, count * 2);
            webgl.uniform2uiv(uniformLocation, value);
        },
        emscripten_glUniform3uiv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Uint32Array(memory.buffer, valuePtr, count * 3);
            webgl.uniform3uiv(uniformLocation, value);
        },
        emscripten_glUniform4uiv: (
            locationId: number,
            count: number,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Uint32Array(memory.buffer, valuePtr, count * 4);
            webgl.uniform4uiv(uniformLocation, value);
        },
        emscripten_glUniformMatrix2fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 4);
            webgl.uniformMatrix2fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix3fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 9);
            webgl.uniformMatrix3fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix4fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 16);
            webgl.uniformMatrix4fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix2x3fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 6);
            webgl.uniformMatrix2x3fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix3x2fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 6);
            webgl.uniformMatrix3x2fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix2x4fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 8);
            webgl.uniformMatrix2x4fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix4x2fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 8);
            webgl.uniformMatrix4x2fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix3x4fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 12);
            webgl.uniformMatrix3x4fv(uniformLocation, transpose, value);
        },
        emscripten_glUniformMatrix4x3fv: (
            locationId: number,
            count: number,
            transpose: boolean,
            valuePtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (!currentProgramInfo) {
                throw new Error("current program is not set");
            }
            const uniformLocation =
                currentProgramInfo.idToUniformLocation.get(locationId);
            if (!uniformLocation) {
                throw new Error("uniform not found");
            }
            const value = new Float32Array(memory.buffer, valuePtr, count * 12);
            webgl.uniformMatrix4x3fv(uniformLocation, transpose, value);
        },
        emscripten_glViewport: webgl?.viewport.bind(webgl) || (() => {}),
        /**
         * void glVertexAttribPointer(
         *  GLuint index,
         *  GLint size,
         *  GLenum type,
         *  GLboolean normalized,
         *  GLsizei stride,
         *  const void * pointer);
         */
        emscripten_glVertexAttribPointer: (
            index: number,
            size: number,
            type: number,
            normalized: number,
            stride: number,
            pointer: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (
                ![
                    webgl.BYTE,
                    webgl.SHORT,
                    webgl.UNSIGNED_BYTE,
                    webgl.UNSIGNED_SHORT,
                    webgl.FLOAT,
                    webgl.HALF_FLOAT,
                    webgl.INT,
                    webgl.UNSIGNED_INT,
                    webgl.INT_2_10_10_10_REV,
                    webgl.UNSIGNED_INT_2_10_10_10_REV,
                ].includes(type as any)
            ) {
                throw new Error(`Invalid type: ${type}`);
            }

            webgl.vertexAttribPointer(
                index,
                size,
                type,
                !!normalized,
                stride,
                pointer,
            );
        },
        emscripten_glVertexAttrib4fv: () => {
            throw new Error("not implemented");
            // return webgl!.vertexAttrib4fv();
        },
        emscripten_glVertexAttrib3fv: () => {
            throw new Error("not implemented");
            // return webgl!.vertexAttrib3fv();
        },
        emscripten_glVertexAttrib2fv: () => {
            throw new Error("not implemented");
            // return webgl!.vertexAttrib2fv();
        },
        emscripten_glVertexAttrib1f: () => {
            throw new Error("not implemented");
            // return webgl!.vertexAttrib1f();
        },
        /**
         * void glUseProgram(GLuint program);
         */
        emscripten_glUseProgram: (programId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (programId === 0) {
                webgl.useProgram(null);
                currentProgramInfo = undefined;
                return;
            }
            const programInfo = programInfos.get(programId);
            if (!programInfo) {
                throw new Error("program not found");
            }
            webgl.useProgram(programInfo.program);
            currentProgramInfo = programInfo;
        },
        emscripten_glGenVertexArraysOES: () => {
            throw new Error("not implemented");
            // return webgl!.genVertexArraysOES();
        },
        emscripten_glDeleteVertexArraysOES: () => {
            throw new Error("not implemented");
            // return webgl!.deleteVertexArraysOES();
        },
        emscripten_glBindVertexArrayOES: () => {
            throw new Error("not implemented");
            // return webgl!.bindVertexArrayOES();
        },
        emscripten_glGenVertexArrays: () => {
            throw new Error("not implemented");
            // return webgl!.genVertexArrays();
        },
        emscripten_glDeleteVertexArrays: () => {
            throw new Error("not implemented");
            // return webgl!.deleteVertexArrays();
        },
        /**
         * void glBindVertexArray(GLuint array);
         */
        emscripten_glBindVertexArray: (arrayId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (arrayId === 0) {
                return webgl.bindVertexArray(null);
            }
            const vertexArray = webglVertexArrayMap.get(arrayId);
            if (!vertexArray) {
                throw new Error("vertexArray not found");
            }
            webgl.bindVertexArray(vertexArray);
        },
        emscripten_glDrawElementsInstanced:
            webgl?.drawElementsInstanced.bind(webgl) || (() => {}),
        emscripten_glDrawArraysInstanced:
            webgl?.drawArraysInstanced.bind(webgl) || (() => {}),
        emscripten_glDrawElementsInstancedBaseVertexBaseInstanceWEBGL: () => {
            throw new Error("not implemented");
        },
        emscripten_glDrawArraysInstancedBaseInstanceWEBGL: () => {
            throw new Error("not implemented");
        },
        emscripten_glReadBuffer: webgl?.readBuffer.bind(webgl) || (() => {}),
        emscripten_glDrawBuffers: () => {
            throw new Error("not implemented");
            // return webgl!.drawBuffers();
        },
        emscripten_glMultiDrawElementsInstancedBaseVertexBaseInstanceWEBGL:
            () => {
                throw new Error("not implemented");
            },
        emscripten_glMultiDrawArraysInstancedBaseInstanceWEBGL: () => {
            throw new Error("not implemented");
        },
        emscripten_glVertexAttribIPointer:
            webgl?.vertexAttribIPointer.bind(webgl) || (() => {}),
        emscripten_glVertexAttribDivisor:
            webgl?.vertexAttribDivisor.bind(webgl) || (() => {}),
        emscripten_glTexStorage2D:
            webgl?.texStorage2D.bind(webgl) || (() => {}),
        emscripten_glDrawRangeElements:
            webgl?.drawRangeElements.bind(webgl) || (() => {}),
        /**
         * void glGenRenderbuffers(
         *  GLsizei n,
         *  GLuint *renderbuffers
         * );
         */
        emscripten_glGenRenderbuffers: (
            n: number,
            renderbuffersPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const renderbuffer = webgl.createRenderbuffer();
                if (!renderbuffer) {
                    throw new Error("Failed to create renderbuffer");
                }
                const renderbufferId = nextRenderbufferId++;
                webglRenderbufferMap.set(renderbufferId, renderbuffer);
                memoryView().setUint32(
                    renderbuffersPtr + i * 4,
                    renderbufferId,
                    true,
                );
            }
        },
        /**
         * void glGenFramebuffers(
         *  GLsizei n,
         *  GLuint *ids
         * );
         *
         * n: Specifies the number of framebuffer object names to generate.
         * ids: Specifies an array in which the generated framebuffer object names are stored.
         */
        emscripten_glGenFramebuffers: (n: number, idsPtr: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const framebuffer = webgl.createFramebuffer();
                if (!framebuffer) {
                    throw new Error("Failed to create framebuffer");
                }
                const framebufferId = nextFramebufferId++;
                webglFramebufferMap.set(framebufferId, framebuffer);
                memoryView().setUint32(idsPtr + i * 4, framebufferId, true);
            }
        },
        /**
         * void glFramebufferTexture2D(
         *  GLenum target,
         *  GLenum attachment,
         *  GLenum textarget,
         *  GLuint texture,
         *  GLint level);
         *
         *  target
         * Specifies the target to which the framebuffer is bound for all commands
         * except glNamedFramebufferTexture.
         *
         * framebuffer
         * Specifies the name of the framebuffer object for glNamedFramebufferTexture.
         *
         * attachment
         * Specifies the attachment point of the framebuffer.
         *
         * textarget
         * For glFramebufferTexture1D, glFramebufferTexture2D and glFramebufferTexture3D,
         * specifies what type of texture is expected in the texture parameter, or for cube map textures,
         * which face is to be attached.
         *
         * texture
         * Specifies the name of an existing texture object to attach.
         *
         * level
         * Specifies the mipmap level of the texture object to attach.
         *
         * # Description
         * These commands attach a selected mipmap level or image of a texture object
         * as one of the logical buffers of the specified framebuffer object.
         * Textures cannot be attached to the default draw and read framebuffer,
         * so they are not valid targets of these commands.
         *
         * For all commands except glNamedFramebufferTexture,
         * the framebuffer object is that bound to target,
         * which must be GL_DRAW_FRAMEBUFFER, GL_READ_FRAMEBUFFER, or GL_FRAMEBUFFER.
         * GL_FRAMEBUFFER is equivalent to GL_DRAW_FRAMEBUFFER.
         *
         * For glNamedFramebufferTexture, framebuffer is the name of the framebuffer object.
         *
         * attachment specifies the logical attachment of the framebuffer
         * and must be GL_COLOR_ATTACHMENTi, GL_DEPTH_ATTACHMENT, GL_STENCIL_ATTACHMENT
         * or GL_DEPTH_STENCIL_ATTACHMENT. i in GL_COLOR_ATTACHMENTi may range from zero
         * to the value of GL_MAX_COLOR_ATTACHMENTS minus one.
         * Attaching a level of a texture to GL_DEPTH_STENCIL_ATTACHMENT is equivalent
         * to attaching that level to both the GL_DEPTH_ATTACHMENT and the GL_STENCIL_ATTACHMENT attachment points simultaneously.
         *
         * For glFramebufferTexture1D, glFramebufferTexture2D and glFramebufferTexture3D,
         * textarget specifies what type of texture is named by texture,
         * and for cube map textures, specifies the face that is to be attached.
         * If texture is not zero, it must be the name of an existing texture object
         * with effective target textarget unless it is a cube map texture,
         * in which case textarget must be GL_TEXTURE_CUBE_MAP_POSITIVE_X,
         * GL_TEXTURE_CUBE_MAP_NEGATIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
         * GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, GL_TEXTURE_CUBE_MAP_POSITIVE_Z,
         * or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z.
         *
         * If texture is non-zero, the specified level of the texture object named texture
         * is attached to the framebuffer attachment point named by attachment.
         * For glFramebufferTexture1D, glFramebufferTexture2D, and glFramebufferTexture3D,
         * texture must be zero or the name of an existing texture with an effective target of textarget,
         * or texture must be the name of an existing cube-map texture and textarget
         * must be one of GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
         * GL_TEXTURE_CUBE_MAP_POSITIVE_Z, GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
         * GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z.
         *
         * If textarget is GL_TEXTURE_RECTANGLE, GL_TEXTURE_2D_MULTISAMPLE,
         * or GL_TEXTURE_2D_MULTISAMPLE_ARRAY, then level must be zero.
         *
         * If textarget is GL_TEXTURE_3D, then level must be greater than or equal to zero
         * and less than or equal to log2 of the value of GL_MAX_3D_TEXTURE_SIZE.
         *
         * If textarget is one of GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
         * GL_TEXTURE_CUBE_MAP_POSITIVE_Z, GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
         * GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, or GL_TEXTURE_CUBE_MAP_NEGATIVE_Z,
         * then level must be greater than or equal to zero and less than or equal to log2
         * of the value of GL_MAX_CUBE_MAP_TEXTURE_SIZE.
         *
         * For all other values of textarget, level must be greater than or equal to zero
         * and less than or equal to log2 of the value of GL_MAX_TEXTURE_SIZE.
         *
         * layer specifies the layer of a 2-dimensional image within a 3-dimensional texture.
         *
         * For glFramebufferTexture1D, if texture is not zero, then textarget must be GL_TEXTURE_1D.
         * For glFramebufferTexture2D, if texture is not zero, textarget must be one of GL_TEXTURE_2D,
         * GL_TEXTURE_RECTANGLE, GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_CUBE_MAP_POSITIVE_Y,
         * GL_TEXTURE_CUBE_MAP_POSITIVE_Z, GL_TEXTURE_CUBE_MAP_NEGATIVE_X,
         * GL_TEXTURE_CUBE_MAP_NEGATIVE_Y, GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, or GL_TEXTURE_2D_MULTISAMPLE.
         * For glFramebufferTexture3D, if texture is not zero, then textarget must be GL_TEXTURE_3D.
         *
         * For glFramebufferTexture and glNamedFramebufferTexture, if texture is the name of a
         * three-dimensional, cube map array, cube map, one- or two-dimensional array,
         * or two-dimensional multisample array texture, the specified texture level is an array of images,
         * and the framebuffer attachment is considered to be layered.
         */
        emscripten_glFramebufferTexture2D: (
            target: number,
            attachment: number,
            textarget: number,
            textureId: number,
            level: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (textureId === 0) {
                return webgl.framebufferTexture2D(
                    target,
                    attachment,
                    textarget,
                    null,
                    level,
                );
            }
            const texture = webglTextureMap.get(textureId);
            if (!texture) {
                throw new Error("texture not found");
            }
            webgl.framebufferTexture2D(
                target,
                attachment,
                textarget,
                texture,
                level,
            );
        },
        emscripten_glFramebufferRenderbuffer: () => {
            throw new Error("not implemented");
            // return webgl!.framebufferRenderbuffer();
        },
        emscripten_glDeleteRenderbuffers: (
            n: number,
            renderbuffersPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const renderbufferId = memoryView().getUint32(
                    renderbuffersPtr + i * 4,
                    true,
                );
                const renderbuffer = webglRenderbufferMap.get(renderbufferId);
                if (!renderbuffer) {
                    throw new Error("renderbuffer not found");
                }
                webgl.deleteRenderbuffer(renderbuffer);
                webglRenderbufferMap.delete(renderbufferId);
            }
        },
        emscripten_glDeleteFramebuffers: (
            n: number,
            framebuffersPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const framebufferId = memoryView().getUint32(
                    framebuffersPtr + i * 4,
                    true,
                );
                const framebuffer = webglFramebufferMap.get(framebufferId);
                if (!framebuffer) {
                    throw new Error("framebuffer not found");
                }
                webgl.deleteFramebuffer(framebuffer);
                webglFramebufferMap.delete(framebufferId);
            }
        },
        emscripten_glCheckFramebufferStatus:
            webgl?.checkFramebufferStatus.bind(webgl) || (() => {}),
        emscripten_glBindRenderbuffer: () => {
            throw new Error("not implemented");
            // return webgl!.bindRenderbuffer();
        },
        emscripten_glBindFramebuffer: (
            target: number,
            framebufferId: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            if (framebufferId === 0) {
                return webgl.bindFramebuffer(target, null);
            }
            const framebuffer = webglFramebufferMap.get(framebufferId);
            if (!framebuffer) {
                throw new Error("framebuffer not found");
            }
            webgl.bindFramebuffer(target, framebuffer);
        },
        emscripten_glRenderbufferStorage:
            webgl?.renderbufferStorage.bind(webgl) || (() => {}),
        emscripten_glGetRenderbufferParameteriv: () => {
            throw new Error("not implemented");
            // return webgl!.getRenderbufferParameteriv();
        },
        emscripten_glGetFramebufferAttachmentParameteriv: () => {
            throw new Error("not implemented");
            // return webgl!.getFramebufferAttachmentParameteriv();
        },
        emscripten_glGenerateMipmap:
            webgl?.generateMipmap.bind(webgl) || (() => {}),
        emscripten_glRenderbufferStorageMultisample:
            webgl?.renderbufferStorageMultisample || (() => {}),
        emscripten_glBlitFramebuffer:
            webgl?.blitFramebuffer.bind(webgl) || (() => {}),
        emscripten_glDeleteSync: () => {
            throw new Error("not implemented");
            // return webgl!.deleteSync();
        },
        emscripten_glClientWaitSync: () => {
            throw new Error("not implemented");
            // return webgl!.clientWaitSync();
        },
        emscripten_glCopyBufferSubData:
            webgl?.copyBufferSubData.bind(webgl) || (() => {}),
        emscripten_glWaitSync: () => {
            throw new Error("not implemented");
            // return webgl!.waitSync();
        },
        emscripten_glIsSync: () => {
            throw new Error("not implemented");
            // return webgl!.isSync();
        },
        emscripten_glFenceSync: () => {
            throw new Error("not implemented");
            // return webgl!.fenceSync();
        },
        /**
         * void glSamplerParameterf(GLuint sampler, GLenum pname, GLfloat param);
         * void glSamplerParameteri(GLuint sampler, GLenum pname, GLint param);
         * void glSamplerParameterfv(GLuint sampler, GLenum pname, const GLfloat *params);
         * void glSamplerParameteriv(GLuint sampler, GLenum pname, const GLint *params);
         * void glSamplerParameterIiv(GLuint sampler, GLenum pname, const GLint *params);
         * void glSamplerParameterIuiv(GLuint sampler, GLenum pname, const GLuint *params);
         */
        emscripten_glSamplerParameterf: (
            samplerId: number,
            pname: number,
            param: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const sampler = webglSamplerMap.get(samplerId);
            if (!sampler) {
                throw new Error("sampler not found");
            }
            webgl.samplerParameterf(sampler, pname, param);
        },
        emscripten_glSamplerParameteri: (
            samplerId: number,
            pname: number,
            param: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const sampler = webglSamplerMap.get(samplerId);
            if (!sampler) {
                throw new Error("sampler not found");
            }
            webgl.samplerParameteri(sampler, pname, param);
        },
        emscripten_glSamplerParameterfv: (
            samplerId: number,
            pname: number,
            paramsPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const sampler = webglSamplerMap.get(samplerId);
            if (!sampler) {
                throw new Error("sampler not found");
            }
            const params = new Float32Array(memory.buffer, paramsPtr, 1);
            webgl.samplerParameterf(sampler, pname, params[0]);
        },
        emscripten_glSamplerParameteriv: (
            samplerId: number,
            pname: number,
            paramsPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const sampler = webglSamplerMap.get(samplerId);
            if (!sampler) {
                throw new Error("sampler not found");
            }
            const params = new Int32Array(memory.buffer, paramsPtr, 1);
            webgl.samplerParameteri(sampler, pname, params[0]);
        },
        /**
         * void glGenSamplers(GLsizei n, GLuint *samplers);
         */
        emscripten_glGenSamplers: (n: number, samplersPtr: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const sampler = webgl.createSampler();
                if (!sampler) {
                    throw new Error("Failed to create sampler");
                }
                const samplerId = nextSamplerId++;
                webglSamplerMap.set(samplerId, sampler);
                memoryView().setUint32(samplersPtr + i * 4, samplerId, true);
            }
        },
        emscripten_glDeleteSamplers: (n: number, samplersPtr: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            for (let i = 0; i < n; i++) {
                const samplerId = memoryView().getUint32(
                    samplersPtr + i * 4,
                    true,
                );
                const sampler = webglSamplerMap.get(samplerId);
                if (!sampler) {
                    throw new Error("sampler not found");
                }
                webgl.deleteSampler(sampler);
                webglSamplerMap.delete(samplerId);
            }
        },
        /**
         * void glBindSampler(GLuint unit, GLuint sampler);
         */
        emscripten_glBindSampler: (unit: number, samplerId: number) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const sampler = webglSamplerMap.get(samplerId);
            if (!sampler) {
                throw new Error("sampler not found");
            }
            webgl.bindSampler(unit, sampler);
        },
        emscripten_glInvalidateSubFramebuffer: () => {
            throw new Error("not implemented");
            // return webgl!.invalidateSubFramebuffer();
        },
        emscripten_glInvalidateFramebuffer: () => {
            throw new Error("not implemented");
            // return webgl!.invalidateFramebuffer();
        },
        /**
         * void glGetShaderPrecisionFormat(
         *  GLenum shaderType,
         *  GLenum precisionType,
         *  GLint *range,
         *  GLint *precision
         * );
         * Parameters
         *  shaderType
         *  Specifies the type of shader whose precision to query. shaderType must be GL_VERTEX_SHADER or GL_FRAGMENT_SHADER.
         *
         *  precisionType
         *  Specifies the numeric format whose precision and range to query.
         *
         *  range
         *  Specifies the address of array of two integers into which encodings of the implementation's numeric range are returned.
         *
         *  precision
         *  Specifies the address of an integer into which the numeric precision of the implementation is written.
         */
        emscripten_glGetShaderPrecisionFormat: (
            shaderType: number,
            precisionType: number,
            rangePtr: number,
            precisionPtr: number,
        ) => {
            if (!webgl) {
                throw new Error("webgl is not set");
            }
            const shaderPrecisionFormat = webgl.getShaderPrecisionFormat(
                shaderType,
                precisionType,
            );
            if (!shaderPrecisionFormat) {
                throw new Error("Failed to get shader precision format");
            }

            memoryView().setInt32(
                rangePtr,
                shaderPrecisionFormat.rangeMin,
                true,
            );
            memoryView().setInt32(
                rangePtr + 4,
                shaderPrecisionFormat.rangeMax,
                true,
            );
            memoryView().setInt32(
                precisionPtr,
                shaderPrecisionFormat.precision,
                true,
            );
        },
        eglQueryString: undefined,
    };
}

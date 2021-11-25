export type TypedArray =
  | Int8Array
  | Uint8Array
  | Uint8ClampedArray
  | Int16Array
  | Uint16Array
  | Int32Array
  | Uint32Array
  | Float32Array
  | Float64Array;

export function isTypedArray(value: any): value is TypedArray {
  return Object.values(JsonTypedArrayType).some(
    (type) => value instanceof typedArrayConstructorMap[type],
  );
}

enum JsonTypedArrayType {
  "I8" = "I8",
  "U8" = "U8",
  "U8C" = "U8C",
  "I16" = "I16",
  "U16" = "U16",
  "I32" = "I32",
  "U32" = "U32",
  "F32" = "F32",
  "F64" = "F64",
}

export const typedArrayConstructorMap = {
  [JsonTypedArrayType.I8]: Int8Array,
  [JsonTypedArrayType.U8]: Uint8Array,
  [JsonTypedArrayType.U8C]: Uint8ClampedArray,
  [JsonTypedArrayType.I16]: Int16Array,
  [JsonTypedArrayType.U16]: Uint16Array,
  [JsonTypedArrayType.I32]: Int32Array,
  [JsonTypedArrayType.U32]: Uint32Array,
  [JsonTypedArrayType.F32]: Float32Array,
  [JsonTypedArrayType.F64]: Float64Array,
} as const;

export type JsonTypedArray = { type: JsonTypedArrayType; data: number[] };

export function isJsonTypedArray(value: any): value is JsonTypedArray {
  return Object.values(JsonTypedArrayType).includes(value.type);
}

export function typeOfTypedArray(value: TypedArray): JsonTypedArrayType {
  for (const key of Object.values(JsonTypedArrayType)) {
    const constructor = typedArrayConstructorMap[key];
    if (value instanceof constructor) {
      return key;
    }
  }
  throw new Error("Unknown array buffer view type");
}

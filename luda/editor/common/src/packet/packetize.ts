import { getValueType, ValueType } from "./type";

const textEncoder = new TextEncoder();

/**
 * packet format
 * [encoded key length in Uint16][encoded key string]
 * [value type in Uint8][packetize(value)]
 */

export function packetize(data: ValueType): ArrayBuffer {
  const buffers = packetizeValue(data);

  const packetSize = buffers.reduce(
    (acc, buffer) => acc + buffer.byteLength,
    0,
  );
  const packetBuffer = new Uint8Array(packetSize);

  let bufferIndex = 0;
  buffers.forEach((buffer) => {
    const newBuffer = new Uint8Array(buffer.buffer);
    packetBuffer.set(newBuffer, bufferIndex);
    bufferIndex += buffer.byteLength;
  });

  return packetBuffer;
}

type TypedArray =
  | Int8Array
  | Uint8Array
  | Uint8ClampedArray
  | Int16Array
  | Uint16Array
  | Int32Array
  | Uint32Array
  | Float32Array
  | Float64Array;

function packetizeValue(value: ValueType): TypedArray[] {
  const valueTypeBuffer = new Uint8Array([getValueType(value)]);
  if (typeof value === "string") {
    const valueBuffer = textEncoder.encode(value);
    const lengthBuffer = new Uint32Array([valueBuffer.byteLength]);
    return [valueTypeBuffer, lengthBuffer, valueBuffer];
  }
  if (typeof value === "number") {
    return [valueTypeBuffer, new Float64Array([value])];
  }
  if (typeof value === "boolean") {
    return [valueTypeBuffer, new Uint8Array([value ? 1 : 0])];
  }
  if (value === undefined) {
    return [valueTypeBuffer];
  }
  if (value === null) {
    return [valueTypeBuffer];
  }
  if (Array.isArray(value)) {
    const arrayLengthBuffer = new Uint32Array([value.length]);
    return [
      valueTypeBuffer,
      arrayLengthBuffer,
      ...value.map(packetizeValue).flat(),
    ];
  }
  if (value instanceof ArrayBuffer) {
    return [valueTypeBuffer, new Uint8Array(value)];
  }

  const objectEntries = Object.entries(value);
  const objectLengthBuffer = new Uint32Array([objectEntries.length]);
  return [
    valueTypeBuffer,
    objectLengthBuffer,
    ...objectEntries.flatMap(([key, value]) => {
      const encodedKey = textEncoder.encode(key);
      const encodedKeyLengthBuffer = new Uint16Array([encodedKey.length]);
      const valueBuffer = packetizeValue(value);

      return [encodedKeyLengthBuffer, encodedKey, ...valueBuffer];
    }),
  ];
}

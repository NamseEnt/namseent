interface TextEncoder {
  new (): TextEncoder;
  encode(value: string): Uint8Array;
}
interface TextDecoder {
  new (): TextDecoder;
  decode(value: ArrayBuffer | ArrayBufferView): string;
}

declare global {
  var TextEncoder: TextEncoder;
  var TextDecoder: TextDecoder;
}

export function getValueType(value: any): ValueTypes {
  if (typeof value === "string") {
    return ValueTypes.string;
  }
  if (typeof value === "number") {
    return ValueTypes.number;
  }
  if (typeof value === "boolean") {
    return ValueTypes.boolean;
  }
  if (value === undefined) {
    return ValueTypes.undefined;
  }
  if (value === null) {
    return ValueTypes.null;
  }
  if (Array.isArray(value)) {
    return ValueTypes.array;
  }
  if (value instanceof ArrayBuffer) {
    return ValueTypes.arrayBuffer;
  }
  return ValueTypes.object;
}

export enum ValueTypes {
  number = 0,
  string = 1,
  boolean = 2,
  object = 3,
  array = 4,
  null = 5,
  undefined = 6,
  arrayBuffer = 7,
}

export type ValueType =
  | number
  | string
  | boolean
  | object
  | Array<any>
  | null
  | undefined
  | ArrayBuffer;

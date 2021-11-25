import { isJsonTypedArray, typedArrayConstructorMap } from "./common";

export function sequenceJsonReviver(key: string, value: any): any {
  if (!isJsonTypedArray(value)) {
    return value;
  }
  const constructor = typedArrayConstructorMap[value.type];
  if (!constructor) {
    throw new Error(`Unknown typed array type: ${value.type}`);
  }
  return new constructor(value.data);
}

import { isTypedArray, JsonTypedArray, typeOfTypedArray } from "./common";

export function sequenceJsonReplacer(key: string, value: any): any {
  if (!isTypedArray(value)) {
    return value;
  }
  const type = typeOfTypedArray(value);
  const jsonTypedArray: JsonTypedArray = {
    type,
    data: Array.from(value),
  };

  return jsonTypedArray;
}

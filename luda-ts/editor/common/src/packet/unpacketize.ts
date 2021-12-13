import { ValueType, ValueTypes } from "./type";

const textDecoder = new TextDecoder();

export function unpacketize<T extends ValueType>(data: ArrayBuffer): T {
  const { value } = unpacketizeValue(data);

  return value as T;
}

function unpacketizeValue(data: ArrayBuffer): {
  value: ValueType;
  consumed: number;
} {
  let consumed = 0;
  const dataView = new DataView(data);

  const valueType: ValueTypes = dataView.getUint8(consumed);
  consumed += 1;

  switch (valueType) {
    case ValueTypes.number: {
      const value = dataView.getFloat64(consumed, true);
      consumed += 8;
      return {
        value,
        consumed,
      };
    }
    case ValueTypes.string: {
      const encodedStringLength = dataView.getUint32(consumed, true);
      consumed += 4;
      const value = textDecoder.decode(
        data.slice(consumed, consumed + encodedStringLength),
      );
      consumed += encodedStringLength;
      return {
        value,
        consumed,
      };
    }
    case ValueTypes.array: {
      const arrayLength = dataView.getUint32(consumed, true);
      consumed += 4;
      const array: ValueType[] = [];
      for (let i = 0; i < arrayLength; i++) {
        const { value, consumed: consumedValue } = unpacketizeValue(
          data.slice(consumed),
        );
        array.push(value);
        consumed += consumedValue;
      }
      return {
        value: array,
        consumed,
      };
    }
    case ValueTypes.object: {
      const object: { [key: string]: ValueType } = {};
      const objectLength = dataView.getUint32(consumed, true);
      consumed += 4;

      for (let i = 0; i < objectLength; i++) {
        const encodedKeyLength = dataView.getUint16(consumed, true);
        consumed += 2;

        const key = textDecoder.decode(
          data.slice(consumed, consumed + encodedKeyLength),
        );
        consumed += encodedKeyLength;

        const { value, consumed: consumedValue } = unpacketizeValue(
          data.slice(consumed),
        );
        consumed += consumedValue;

        object[key] = value;
      }

      return {
        value: object,
        consumed: consumed,
      };
    }
    case ValueTypes.boolean: {
      const value = dataView.getUint8(consumed) === 1;
      consumed += 1;
      return {
        value,
        consumed,
      };
    }
    case ValueTypes.null: {
      const value = null;
      return {
        value,
        consumed,
      };
    }

    case ValueTypes.undefined: {
      const value = undefined;
      return {
        value,
        consumed,
      };
    }
    default:
      throw new Error(`Unknown value type: ${valueType}`);
  }
}

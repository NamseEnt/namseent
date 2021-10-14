import { Code } from "./Code";
import { Key } from "./Key";

export interface IKeyboardManager {
  isCodePress(code: Code): boolean;
  anyCodePress(codes: Code[]): boolean;
  isKeyPress(key: Key): boolean;
}

import { IFileSystem } from "./IFileSystem";
import { Dirent, ToServerSocket } from "luda-editor-common";

export class RpcFileSystem implements IFileSystem {
  constructor(private readonly socket: ToServerSocket) {}
  async list(path: string): Promise<Dirent[]> {
    const result = await this.socket.send("ListFiles", {
      directoryPath: path,
    });
    return result.entries;
  }
  read(path: string): Promise<ArrayBuffer> {
    throw new Error("Method not implemented.");
  }
  write(path: string, content: string | ArrayBuffer): Promise<void> {
    throw new Error("Method not implemented.");
  }
}

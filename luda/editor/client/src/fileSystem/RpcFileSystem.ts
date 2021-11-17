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
  async read(path: string): Promise<ArrayBuffer> {
    return await this.socket.send("ReadFile", {
      destPath: path,
    });
  }
  async write(path: string, content: string | ArrayBuffer): Promise<void> {
    await this.socket.send("WriteFile", {
      data: content,
      destPath: path,
    });
    return;
  }
}

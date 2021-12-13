import { socket } from "../network/socket";
import { IFileSystem } from "./IFileSystem";
import { RpcFileSystem } from "./RpcFileSystem";

const fileSystem = new RpcFileSystem(socket);
export default fileSystem as IFileSystem;

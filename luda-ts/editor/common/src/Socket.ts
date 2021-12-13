import { nanoid } from "nanoid";
import { ToServerRpcs } from "./ToServerRpcs";
import { ToClientRpcs } from "./ToClientRpcs";
import { packetize } from "./packet/packetize";
import { unpacketize } from "./packet/unpacketize";

export interface ISocketInternal {
  send(data: ArrayBuffer): void;
  setOnMessage(callback: (data: ArrayBuffer) => void): void;
  onError(callback: (error: Error) => void): void;
  onClose(callback: () => void): void;
}

type Rpcs = { [key: string]: { input: any; output: any } };

export class Socket<
  TSendRpcs extends Rpcs,
  TReceiveRpcs extends Rpcs,
  TContext extends any,
> {
  private readonly rpcCallbacks: Map<string, (output: any) => void> = new Map();
  private readonly rpcTimeoutMs: number = 300 * 1000;
  private readonly rpcTimeoutIds: Set<ReturnType<typeof setTimeout>> =
    new Set();
  private context: TContext | undefined;
  private handler: RequestHandler<TReceiveRpcs, TContext> | undefined;

  constructor(
    private readonly socketInternal: ISocketInternal,
    private readonly log?: boolean,
  ) {
    this.socketInternal.setOnMessage((data) => this.onMessage(data));
  }

  public onError = this.socketInternal.onError;
  public onClose(callback: () => void): void {
    this.socketInternal.onClose(() => {
      callback();
      this.dispose();
    });
  }

  send<TType extends keyof TSendRpcs>(
    type: TType,
    input: TSendRpcs[TType]["input"],
  ): Promise<TSendRpcs[TType]["output"]> {
    const packetId = nanoid();
    const message = packetize(["RpcRequest", packetId, type, input]);

    return new Promise((resolve) => {
      this.rpcCallbacks.set(packetId, resolve);
      const id = setTimeout(() => {
        this.rpcTimeoutIds.delete(id);
        if (!this.rpcCallbacks.has(packetId)) {
          return;
        }
        this.rpcCallbacks.delete(packetId);
        console.error(`rpc ${type} ${packetId} timed out`);
      }, this.rpcTimeoutMs);
      this.rpcTimeoutIds.add(id);
      this.socketInternal.send(message);
    });
  }

  private async onMessage(data: ArrayBuffer): Promise<void> {
    const packet = unpacketize(data) as
      | [type: "RpcRequest", packetId: string, rpcType: string, input: any]
      | [type: "RpcResponse", packetId: string, output: any];

    switch (packet[0]) {
      case "RpcRequest": {
        const { handler, context } = this;
        if (handler === undefined) {
          throw new Error(
            "handler is undefined. please use setOnMessage first",
          );
        }
        if (context === undefined) {
          throw new Error(
            "context is undefined. please use setOnMessage first",
          );
        }

        const [_, packetId, rpcType, input] = packet;
        this.log && console.log("onRpcRequest", packetId, rpcType, input);
        const func = (handler as any)[`on${rpcType}`];
        if (!func) {
          throw new Error(`no handler for rpc type ${rpcType}`);
        }
        const output = await func(context, input);
        const responseMessage = packetize(["RpcResponse", packetId, output]);
        this.socketInternal.send(responseMessage);
        return;
      }
      case "RpcResponse": {
        const [_, packetId, output] = packet;
        this.log && console.log("onRpcResponse", packetId, output);
        this.rpcCallbacks.get(packetId)?.call(this, output);
        this.rpcCallbacks.delete(packetId);
        return;
      }
      default: {
        throw new Error(`no handler for packet type ${packet[0]}`);
      }
    }
  }

  public setHandler(
    context: TContext,
    handler: RequestHandler<TReceiveRpcs, TContext>,
  ): void {
    this.context = context;
    this.handler = handler;
  }

  private dispose() {
    this.rpcCallbacks.clear();
    this.rpcTimeoutIds.forEach((id) => clearTimeout(id));
    this.rpcTimeoutIds.clear();
  }
}

type RequestHandler<TRpcs extends Rpcs, TContext> = {
  [TKey in Extract<
    keyof TRpcs,
    string
  > as `on${TKey}`]: TRpcs[TKey] extends undefined
    ? (
        context: TContext,
      ) => TRpcs[TKey]["output"] extends undefined
        ? void | Promise<void>
        : Promise<TRpcs[TKey]["output"]> | TRpcs[TKey]["output"]
    : (
        context: TContext,
        data: TRpcs[TKey]["input"],
      ) => TRpcs[TKey]["output"] extends undefined
        ? void | Promise<void>
        : Promise<TRpcs[TKey]["output"]> | TRpcs[TKey]["output"];
};

export type ToServerSocket<TContext = {}> = Socket<
  ToServerRpcs,
  ToClientRpcs,
  TContext
>;
export type ToClientSocket<TContext = {}> = Socket<
  ToClientRpcs,
  ToServerRpcs,
  TContext
>;
export type ToClientRpcHandler<TContext> = RequestHandler<
  ToClientRpcs,
  TContext
>;

export type ToServerRpcHandler<TContext> = RequestHandler<
  ToServerRpcs,
  TContext
>;

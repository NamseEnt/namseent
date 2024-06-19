import { Exports } from "../exports";

export function webSocketImports({
    memory,
    exports,
}: {
    memory: WebAssembly.Memory;
    exports: () => Exports;
}) {
    let nextId = 0;
    const webSockets = new Map<number, WebSocket>();

    return {
        new_web_socket: (urlPtr: number, urlLen: number): number => {
            const url = new TextDecoder().decode(
                new Uint8Array(memory.buffer, urlPtr, urlLen),
            );
            const webSocket = new WebSocket(url);
            const id = nextId++;
            webSockets.set(id, webSocket);

            webSocket.onopen = () => {
                exports().on_web_socket_open(id);
            };
            webSocket.onclose = () => {
                exports().on_web_socket_close(id);
            };
            webSocket.onmessage = (event) => {
                const data = new Uint8Array(event.data);
                const dataPtr = exports().web_socket_message_alloc(
                    data.length,
                );
                new Uint8Array(memory.buffer).set(data, dataPtr);
                exports().on_web_socket_message(id, dataPtr);
            };

            return id;
        },
        web_socket_send: (id: number, dataPtr: number, dataLen: number) => {
            const webSocket = webSockets.get(id);
            if (!webSocket) {
                throw new Error(`WebSocket not found: ${id}`);
            }
            const data = new Uint8Array(memory.buffer, dataPtr, dataLen);
            webSocket.send(data);
        },
    };
}

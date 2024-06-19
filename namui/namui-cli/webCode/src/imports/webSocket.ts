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
        new_web_socket: (url_ptr: number, url_len: number): number => {
            const url = new TextDecoder().decode(
                new Uint8Array(memory.buffer, url_ptr, url_len),
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
                const data_ptr = exports().web_socket_message_alloc(
                    data.length,
                );
                new Uint8Array(memory.buffer).set(data, data_ptr);
                exports().on_web_socket_message(id, data_ptr);
            };

            return id;
        },
        web_socket_send: (id: number, data_ptr: number, data_len: number) => {
            const webSocket = webSockets.get(id);
            if (!webSocket) {
                throw new Error(`WebSocket not found: ${id}`);
            }
            const data = new Uint8Array(memory.buffer, data_ptr, data_len);
            webSocket.send(data);
        },
    };
}

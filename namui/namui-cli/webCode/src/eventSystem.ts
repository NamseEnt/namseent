/*
eventBuffer (i32 array)
- 0: number of not consumed events
- 1~n: event packets

event packet
- 0: event type
- 1~n: event data. depends on event type

event type and body
- 0xFF: end of buffer. move buffer u8 index to 4. This also counts as an event.
- 0x00: on animation frame
- 0x01: on resize
  - u16: width
  - u16: height
*/

export class EventSystemOnWorker {
    private eventBufferIndex: number = 4;
    private lastEventTime: DOMHighResTimeStamp | undefined;
    constructor(
        private readonly eventBuffer: SharedArrayBuffer,
        private readonly memory: WebAssembly.Memory,
    ) {}

    /**
     * @return {number} the byte length of event.
     */
    public pollEvent(wasmBufferPtr: number): number {
        if (this.lastEventTime) {
            const now = performance.now();
            // console.log("last event handling time: ", now - this.lastEventTime);
            this.lastEventTime = now;
        } else {
            this.lastEventTime = performance.now();
        }

        const eventBufferView = new DataView(this.eventBuffer);
        const i32Array = new Int32Array(this.eventBuffer);
        const wasmBuffer = new DataView(this.memory.buffer, wasmBufferPtr, 16);
        const wasmBufferU8 = new Uint8Array(
            this.memory.buffer,
            wasmBufferPtr,
            16,
        );

        Atomics.wait(i32Array, 0, 0);
        Atomics.sub(i32Array, 0, 1);
        // TODO: Monitor how many event are in queue.

        const eventType = eventBufferView.getUint8(this.eventBufferIndex);
        switch (eventType) {
            case 0xff: {
                this.eventBufferIndex = 4;
                return this.pollEvent(wasmBufferPtr);
            }
            case 0x00: {
                this.eventBufferIndex += 1;
                wasmBuffer.setUint8(0, 0x00);
                return 1;
            }
            case 0x01: {
                const packetSize = 5;

                wasmBufferU8.set(
                    new Uint8Array(
                        this.eventBuffer,
                        this.eventBufferIndex,
                        packetSize,
                    ),
                );

                this.eventBufferIndex += packetSize;
                return packetSize;
            }
            default: {
                throw new Error(`Unknown event type: ${eventType}`);
            }
        }
    }
}

export function startEventSystemOnMainThread(eventBuffer: SharedArrayBuffer) {
    let eventBufferIndex = 4;

    const eventBufferView = new DataView(eventBuffer);
    const i32Array = new Int32Array(eventBuffer);

    function checkIndexOverflow() {
        const margin = 32;
        if (eventBufferIndex + margin >= eventBuffer.byteLength) {
            eventBufferView.setUint8(eventBufferIndex, 0xff);
            eventBufferIndex = 4;

            Atomics.add(i32Array, 0, 1);
            Atomics.notify(i32Array, 0);
        }
    }

    function onAnimationFrame() {
        checkIndexOverflow();

        eventBufferView.setUint8(eventBufferIndex, 0x00);
        eventBufferIndex += 1;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);

        requestAnimationFrame(onAnimationFrame);
    }
    requestAnimationFrame(onAnimationFrame);

    window.addEventListener("resize", () => {
        checkIndexOverflow();

        eventBufferView.setUint8(eventBufferIndex, 0x01);
        eventBufferView.setUint16(eventBufferIndex + 1, window.innerWidth);
        eventBufferView.setUint16(eventBufferIndex + 3, window.innerHeight);
        eventBufferIndex += 5;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });
}

type MetaMessage = {
    id: number;
    type: "request" | "response";
    inner: any;
};

export function runAsyncMessageLoop(
    target: {
        onmessage: ((request: any) => any) | null;
        postMessage: (response: any) => void;
    },
    handleRequest: (request: any) => Promise<any>,
) {
    target.onmessage = async ({ data }: { data: MetaMessage }) => {
        switch (data.type) {
            case "request":
                {
                    const response = await handleRequest(data.inner);
                    target.postMessage({
                        id: data.id,
                        type: "response",
                        inner: response,
                    });
                }
                break;
            case "response":
                {
                    const { id, inner } = data;
                    const resolve = waitingResponseMap.get(id);
                    if (!resolve) {
                        throw new Error("no resolve for id " + id);
                    }
                    resolve(inner);
                }
                break;
        }
    };
}

const waitingResponseMap = new Map<number, (response: any) => void>();
let nextRequestId = 0;

export function sendAsyncRequest(
    target: {
        postMessage: (response: any, transfer: Transferable[]) => void;
    },
    request: any,
    transfer?: Transferable[],
): Promise<any> {
    const id = nextRequestId++;
    return new Promise((resolve) => {
        waitingResponseMap.set(id, resolve);

        target.postMessage(
            {
                id,
                type: "request",
                inner: request,
            },
            transfer ?? [],
        );
    });
}

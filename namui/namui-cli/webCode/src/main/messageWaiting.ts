let messageId = 0;
export function getNextMessageId() {
    return messageId++;
}

const waitingMessages = new Map<number, (message: any) => void>();
export function waitForMessage(id: number) {
    return new Promise((resolve) => {
        waitingMessages.set(id, resolve);
    });
}

export function onMessage(message: { id: number }) {
    const { id } = message;
    const resolve = waitingMessages.get(id);
    if (resolve) {
        waitingMessages.delete(id);
        resolve(message);
    } else {
        console.warn(`No message waiting for id ${id}`);
    }
}

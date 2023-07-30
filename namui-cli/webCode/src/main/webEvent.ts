type WebEvent = {
    MouseDown: {
        x: number;
        y: number;
    };
};

const queue: WebEvent[] = [];

export async function waitWebEvent(): Promise<WebEvent> {
    while (queue.length === 0) {
        await new Promise((resolve) => setTimeout(resolve, 0));
    }
    return queue.shift()!;
}

document.addEventListener("mouseup", (e) => {
    e.preventDefault();
    queue.push({
        MouseDown: {
            x: e.clientX,
            y: e.clientY,
        },
    });
});

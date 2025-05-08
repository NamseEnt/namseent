let logs: string[] = [];

export function pushLog(threadId: number, msg: string) {
    const text = `[${threadId}] ${msg}`;
    if (isLogPrintOn()) {
        console.log(text);
        return;
    }
    logs.push(text);
    if (logs.length > 30) {
        logs.shift();
    }
}
function isLogPrintOn() {
    return localStorage.getItem("namui-log") === "true";
}

function toggleLogPrint() {
    if (!isLogPrintOn()) {
        logs.forEach((log) => {
            console.log(log);
        });
        logs = [];
    }
    localStorage.setItem("namui-log", isLogPrintOn() ? "false" : "true");
}

(globalThis as any).toggleLog = toggleLogPrint;

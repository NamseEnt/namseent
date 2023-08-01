export const textArea = document.createElement("textarea");
(globalThis as any).textArea = textArea;

// NOTE: Below codes from https://github.com/goldfire/CanvasInput/blob/5adbaf00bd42665f3c691796881c7a7a9cf7036c/CanvasInput.js#L126
textArea.style.width = "100%";
textArea.style.position = "absolute";
textArea.style.opacity = "0";
textArea.style.pointerEvents = "none";
textArea.style.zIndex = "0";
textArea.style.top = "0px";
// hide native blue text cursor on iOS
textArea.style.transform = "scale(0)";
document.body.appendChild(textArea);

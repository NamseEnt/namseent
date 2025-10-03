import { BincodeReader } from "./reader";
import { visitRenderingTree, type Canvas } from "./draw";

const file = Bun.file("../bincode-test/rendering_tree.bin");
const renderingTreeBytes = await file.arrayBuffer();

const reader = new BincodeReader(renderingTreeBytes);

// Mock canvas for testing
const mockCanvas: Canvas = {
    save: () => console.log("save"),
    restore: () => console.log("restore"),
    translate: (x, y) => console.log("translate", x, y),
    rotate: (angle) => console.log("rotate", angle),
    scale: (x, y) => console.log("scale", x, y),
    setMatrix: (matrix) => console.log("setMatrix", matrix),
    getMatrix: () =>
        [
            [1, 0, 0],
            [0, 1, 0],
        ] as [[number, number, number], [number, number, number]],
    clipPath: (clipOp) => console.log("clipPath", clipOp),
};

const onTopNodes: any[] = [];

try {
    visitRenderingTree(reader, mockCanvas, onTopNodes);
    console.log("Successfully visited rendering tree");
    console.log("OnTop nodes:", onTopNodes.length);
} catch (e) {
    console.error("Error:", e);
}

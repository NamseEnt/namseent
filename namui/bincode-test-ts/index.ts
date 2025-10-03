import { decode } from "bincode-ts";
import { RenderingTree } from "./rendering-tree";

const file = Bun.file("../bincode-test/rendering_tree.bin");

const data = await file.arrayBuffer();

const decoded = decode(RenderingTree, data);
console.log(decoded);

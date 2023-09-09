import { h } from "preact";
import { FunctionComponent, render, useState } from "preact/compat";

export type InspectTree = {
    shortName: string;
    left: number | undefined;
    top: number | undefined;
    width: number | undefined;
    height: number | undefined;
    children: InspectTree[];
};

export let setInspectTree: (tree: InspectTree) => void = () => {
    throw new Error("setInspectTree not initialized");
};

export let toggleInspectOn: () => void = () => {
    throw new Error("toggleInspectOn not initialized");
};

export function isInspectOn() {
    return localStorage.getItem("inspect") !== null;
}

const Inspect: FunctionComponent = () => {
    const [inspectOn, setInspectOn] = useState(isInspectOn());
    const [tree, setTree] = useState<InspectTree>();

    setInspectTree = setTree;
    toggleInspectOn = () => {
        const prev = isInspectOn();
        const next = !prev;
        if (next) {
            localStorage.setItem("inspect", "");
        } else {
            localStorage.removeItem("inspect");
        }
        setInspectOn(next);
    };

    if (!inspectOn || !tree) {
        return <></>;
    }

    const styleContent = generateStyleContent(tree);

    return (
        <>
            <style>{styleContent}</style>
            {h(
                "inspect",
                { id: "inspect" },
                <InspectBlock tree={tree}></InspectBlock>,
            )}
        </>
    );
};

function generateStyleContent(tree: InspectTree): string {
    function run(tree: InspectTree, indexes: number[]): string {
        return `
            #inspect > ${indexes
                .map((x) => `:nth-child(${x + 1})`)
                .join(" > ")} {
                position: fixed;
                left: ${tree.left}px;
                top: ${tree.top}px;
                width: ${tree.width}px;
                height: ${tree.height}px;
            }
            ${tree.children
                .map((child, i) => run(child, [...indexes, i]))
                .join("\n")}
        `;
    }
    return run(tree, [0]);
}

const InspectBlock: FunctionComponent<{ tree: InspectTree }> = ({ tree }) => {
    const escapedShortName = tree.shortName
        .replaceAll("<", "__")
        .replaceAll(">", "__");
    return h(
        escapedShortName,
        {
            name: tree.shortName,
        },
        ...tree.children.map((child) => h(InspectBlock, { tree: child })),
    );
};

render(<Inspect />, document.body);

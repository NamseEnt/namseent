import * as vscode from "vscode";
import {
    clone_to_closure,
    LineColumn,
} from "../in_rust/pkg/rust_helper_extension";
import { exec } from "node:child_process";

const CLONE_TO_CLOSURE_COMMAND = "rust-helper.clone-to-closure";

export function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.languages.registerCodeActionsProvider("rust", new RustHelper(), {
            providedCodeActionKinds: RustHelper.providedCodeActionKinds,
        }),
    );

    context.subscriptions.push(
        vscode.commands.registerTextEditorCommand(
            CLONE_TO_CLOSURE_COMMAND,
            cloneToClosure,
        ),
    );
}

export class RustHelper implements vscode.CodeActionProvider {
    public static readonly providedCodeActionKinds = [
        vscode.CodeActionKind.QuickFix,
    ];

    provideCodeActions(
        document: vscode.TextDocument,
        _range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext,
        _token: vscode.CancellationToken,
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];

        console.log(
            "context.diagnostics",
            context.diagnostics,
            context.diagnostics.map((x) => x.relatedInformation),
            JSON.stringify(context.diagnostics),
        );

        const borrowOfMovedValueDiagnostics = context.diagnostics.find(
            (x) =>
                x.severity === vscode.DiagnosticSeverity.Error &&
                x.message.startsWith("borrow of moved value: "),
        );
        if (borrowOfMovedValueDiagnostics) {
            actions.push(
                this.createCloneToClosureCommandCodeAction(
                    document,
                    borrowOfMovedValueDiagnostics,
                ),
            );
        }

        return actions;
    }

    private createCloneToClosureCommandCodeAction(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic,
    ): vscode.CodeAction {
        console.log(
            "on create action: diagnostic",
            diagnostic,
            diagnostic.relatedInformation,
            JSON.stringify(diagnostic, null, 2),
        );
        const action = new vscode.CodeAction(
            "Clone variable to previous closure",
            vscode.CodeActionKind.QuickFix,
        );
        action.command = {
            command: CLONE_TO_CLOSURE_COMMAND,
            title: "Clone variable to previous closure",
            arguments: [document, diagnostic],
        };
        action.diagnostics = [diagnostic];
        action.isPreferred = true;

        return action;
    }
}

async function cloneToClosure(
    textEditor: vscode.TextEditor,
    edit: vscode.TextEditorEdit,
    document: vscode.TextDocument,
    diagnostic: vscode.Diagnostic,
) {
    console.log("diagnostic", diagnostic, JSON.stringify(diagnostic, null, 2));
    const moveKeywordInfo = diagnostic.relatedInformation!.find(
        (x) => x.message === "value moved into closure here",
    )!;
    const moveStart = moveKeywordInfo.location.range.start;
    const variableName = diagnostic.message.substring(
        "borrow of moved value: `".length,
        diagnostic.message.indexOf("\n") - 1,
    );
    const result = clone_to_closure(
        document.getText(),
        new LineColumn(moveStart.line + 1, moveStart.character),
        variableName,
        new LineColumn(
            diagnostic.range.start.line + 1,
            diagnostic.range.start.character,
        ),
    );

    const action = JSON.parse(result) as {
        insert: {
            line: number;
            column: number;
            text: string;
        }[];
    };
    console.log("action", action);

    action.insert.forEach((insert) => {
        edit.insert(
            new vscode.Position(insert.line - 1, insert.column),
            insert.text,
        );
    });
}

function runRustFmt(rustFile: String): Promise<string> {
    const child = exec("rustfmt");
    child.stdout!.setEncoding("utf8");
    child.stdin!.write(rustFile);
    child.stdin!.end();

    return new Promise((resolve, reject) => {
        const chunks: string[] = [];
        child.stdout!.on("data", (data) => {
            chunks.push(data);
        });
        child.stdout!.on("close", () => {
            const output = chunks.join("");
            resolve(output);
        });
        child.stdout!.on("error", (err) => {
            reject(err);
        });
    });
}
function calculateEditRange(
    inputText: string,
    formattedOutput: string,
): {
    range: vscode.Range;
    text: string;
} {
    let startLine = 0;
    let startCharacter = 0;
    let startIndex = 0;

    const inputLines = inputText.split("\n");
    let endLine = inputLines.length - 1;
    let endCharacter = inputLines[endLine].length;
    let endIndex = 0;

    for (
        startIndex = 0;
        startIndex < Math.min(inputText.length, formattedOutput.length);
        startIndex++
    ) {
        const inputChar = inputText[startIndex];
        const outputChar = formattedOutput[startIndex];
        if (inputChar !== outputChar) {
            break;
        }

        if (inputChar === "\n") {
            startLine++;
            startCharacter = 0;
        } else {
            startCharacter++;
        }
    }

    for (
        endIndex = 0;
        endIndex < Math.min(inputText.length, formattedOutput.length);
        endIndex++
    ) {
        const inputChar = inputText[inputText.length - endIndex - 1];
        const outputChar =
            formattedOutput[formattedOutput.length - endIndex - 1];

        if (inputChar !== outputChar) {
            break;
        }

        if (inputChar === "\n") {
            endLine--;
            endCharacter = inputLines[endLine].length;
        } else {
            endCharacter--;
        }
    }

    const range = new vscode.Range(
        new vscode.Position(startLine, startCharacter),
        new vscode.Position(endLine, endCharacter),
    );
    const text = formattedOutput.substring(
        startIndex,
        formattedOutput.length - endIndex,
    );

    return {
        range,
        text,
    };
}

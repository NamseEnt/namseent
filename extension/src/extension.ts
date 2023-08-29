import * as vscode from "vscode";
import {
    clone_to_closure,
    LineColumn,
    position_is_in_async_block,
    position_is_in_closure,
    wrap_async_block_in_block,
    wrap_closure_in_block,
} from "../in_rust/pkg/rust_helper_extension";

const CLONE_TO_CLOSURE_COMMAND = "rust-helper.clone-to-closure";
const WRAP_ASYNC_BLOCK_IN_BLOCK = "rust-helper.wrap-async-block-in-block";
const WRAP_CLOSURE_IN_BLOCK = "rust-helper.wrap-closure-in-block";

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
    context.subscriptions.push(
        vscode.commands.registerTextEditorCommand(
            WRAP_ASYNC_BLOCK_IN_BLOCK,
            wrapAsyncBlockInBlock,
        ),
    );
    context.subscriptions.push(
        vscode.commands.registerTextEditorCommand(
            WRAP_CLOSURE_IN_BLOCK,
            wrapClosureInBlock,
        ),
    );
}

export class RustHelper implements vscode.CodeActionProvider {
    public static readonly providedCodeActionKinds = [
        vscode.CodeActionKind.QuickFix,
    ];

    provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
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

        if (this.positionIsInAsyncBlock(document, range.start)) {
            actions.push(
                this.createWrapAsyncBlockWithBlockAction(document, range.start),
            );
        }

        if (this.positionIsInClosure(document, range.start)) {
            actions.push(
                this.createWrapClosureWithBlockAction(document, range.start),
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

    private createWrapAsyncBlockWithBlockAction(
        document: vscode.TextDocument,
        position: vscode.Position,
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            "Wrap async block in block",
            vscode.CodeActionKind.QuickFix,
        );
        action.command = {
            command: WRAP_ASYNC_BLOCK_IN_BLOCK,
            title: "Wrap async block in block",
            arguments: [document, position],
        };
        action.isPreferred = true;

        return action;
    }

    private createWrapClosureWithBlockAction(
        document: vscode.TextDocument,
        position: vscode.Position,
    ): vscode.CodeAction {
        const action = new vscode.CodeAction(
            "Wrap closure in block",
            vscode.CodeActionKind.QuickFix,
        );
        action.command = {
            command: WRAP_CLOSURE_IN_BLOCK,
            title: "Wrap closure in block",
            arguments: [document, position],
        };
        action.isPreferred = true;

        return action;
    }

    private positionIsInAsyncBlock(
        document: vscode.TextDocument,
        position: vscode.Position,
    ): boolean {
        return position_is_in_async_block(
            document.getText(),
            new LineColumn(position.line + 1, position.character),
        );
    }

    private positionIsInClosure(
        document: vscode.TextDocument,
        position: vscode.Position,
    ): boolean {
        return position_is_in_closure(
            document.getText(),
            new LineColumn(position.line + 1, position.character),
        );
    }
}

async function cloneToClosure(
    _textEditor: vscode.TextEditor,
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

async function wrapClosureInBlock(
    _textEditor: vscode.TextEditor,
    edit: vscode.TextEditorEdit,
    document: vscode.TextDocument,
    position: vscode.Position,
) {
    const result = wrap_closure_in_block(
        document.getText(),
        new LineColumn(position.line + 1, position.character),
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

async function wrapAsyncBlockInBlock(
    _textEditor: vscode.TextEditor,
    edit: vscode.TextEditorEdit,
    document: vscode.TextDocument,
    position: vscode.Position,
) {
    const result = wrap_async_block_in_block(
        document.getText(),
        new LineColumn(position.line + 1, position.character),
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

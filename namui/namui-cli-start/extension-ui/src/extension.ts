import * as vscode from "vscode";
import * as child_process from "child_process";

export async function activate(context: vscode.ExtensionContext) {
    console.log("start notepad");
    child_process.spawn("notepad");
}

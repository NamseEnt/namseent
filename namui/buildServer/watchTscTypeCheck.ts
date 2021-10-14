import ts from "typescript";
import path from "path";
import { ErrorMessage } from "../src/build/type";

const formatHost: ts.FormatDiagnosticsHost = {
  getCanonicalFileName: (path) => path,
  getCurrentDirectory: ts.sys.getCurrentDirectory,
  getNewLine: () => ts.sys.newLine,
};

export function watchTscTypeCheck(
  tsconfigFilePath: string,
  updateBuildErrorMessages: (errorMessages: ErrorMessage[]) => void,
) {
  // TypeScript can use several different program creation "strategies":
  //  * ts.createEmitAndSemanticDiagnosticsBuilderProgram,
  //  * ts.createSemanticDiagnosticsBuilderProgram
  //  * ts.createAbstractBuilder
  // The first two produce "builder programs". These use an incremental strategy
  // to only re-check and emit files whose contents may have changed, or whose
  // dependencies may have changes which may impact change the result of prior
  // type-check and emit.
  // The last uses an ordinary program which does a full type check after every
  // change.
  // Between `createEmitAndSemanticDiagnosticsBuilderProgram` and
  // `createSemanticDiagnosticsBuilderProgram`, the only difference is emit.
  // For pure type-checking scenarios, or when another tool/process handles emit,
  // using `createSemanticDiagnosticsBuilderProgram` may be more desirable.
  const createProgram = ts.createSemanticDiagnosticsBuilderProgram;

  let errorMessages: ErrorMessage[] = [];
  function reportDiagnostic(diagnostic: ts.Diagnostic) {
    const message = ts.formatDiagnostic(diagnostic, formatHost);
    if (!diagnostic.file || !diagnostic.start || !diagnostic.messageText) {
      console.error(message);
      return;
    }

    const { character, line } = diagnostic.file.getLineAndCharacterOfPosition(
      diagnostic.start,
    );
    errorMessages.push({
      column: character + 1,
      line: line + 1,
      relativeFile: path.relative(
        formatHost.getCurrentDirectory(),
        diagnostic.file.fileName,
      ),
      absoluteFile: diagnostic.file.fileName,
      text: ts.flattenDiagnosticMessageText(
        diagnostic.messageText,
        formatHost.getNewLine(),
      ),
    });
  }
  // Note that there is another overload for `createWatchCompilerHost` that takes
  // a set of root files.
  const host = ts.createWatchCompilerHost(
    tsconfigFilePath,
    {
      noEmit: true,
    },
    ts.sys,
    createProgram,
    reportDiagnostic,
    reportWatchStatusChanged,
  );

  const origPostProgramCreate = host.afterProgramCreate;

  host.afterProgramCreate = (program) => {
    origPostProgramCreate!(program);
    updateBuildErrorMessages(errorMessages);
    errorMessages = [];
  };

  // `createWatchProgram` creates an initial program, watches files, and updates
  // the program over time.
  const program = ts.createWatchProgram(host).getProgram();
  const diagnostics = program.getSemanticDiagnostics();
  errorMessages = [];
  diagnostics.forEach(reportDiagnostic);
  updateBuildErrorMessages(errorMessages);
  errorMessages = [];
}

function reportWatchStatusChanged(diagnostic: ts.Diagnostic) {
  // Nothing
}

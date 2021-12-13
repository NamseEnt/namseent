import { ErrorMessage } from "../src/build/type";
import { getNamuiConfig } from "./namuiConfig";
import { startEsbuild } from "./startEsbuild";
import { startServer } from "./startServer";
import { watchTscTypeCheck } from "./watchTscTypeCheck";

export async function build({
  entryPoint,
  watch,
}: {
  entryPoint: string;
  watch: boolean;
}) {
  if (!watch) {
    throw new Error(
      "for now, only watch mode is supported. please use --watch option.",
    );
  }

  const namuiConfig = await getNamuiConfig();

  const errorMessagesMap: {
    esbuildErrorMessages: ErrorMessage[];
    tscErrorMessages: ErrorMessage[];
  } = {
    esbuildErrorMessages: [],
    tscErrorMessages: [],
  };
  const getErrorMessages = () => [
    ...errorMessagesMap.esbuildErrorMessages,
    ...errorMessagesMap.tscErrorMessages,
  ];

  let onRebuild: (option: { shouldReload: boolean }) => void = () => {};

  const startEsbuildPromise = startEsbuild({
    entryPoint,
    onRebuild: (esbuildErrorMessages: ErrorMessage[]) => {
      errorMessagesMap.esbuildErrorMessages = esbuildErrorMessages;
      onRebuild({
        shouldReload: true,
      });
    },
  });

  const { sendErrorMessages, requestReload } = await startServer({
    port: 8080,
    onConnected: (webSocket) => {
      sendErrorMessages(getErrorMessages(), webSocket);
    },
    getBuild: () => startEsbuildPromise.then(({ getBuild }) => getBuild()),
    resourcesPath: namuiConfig.resources,
  });

  onRebuild = ({ shouldReload }: { shouldReload: boolean }) => {
    const errorMessages = getErrorMessages();
    sendErrorMessages(errorMessages);
    printErrorMessages(errorMessages);
    if (shouldReload) {
      requestReload();
    }
  };

  watchTscTypeCheck(
    namuiConfig.tsconfigFilePath,
    (tscErrorMessages: ErrorMessage[]) => {
      errorMessagesMap.tscErrorMessages = tscErrorMessages;
      onRebuild({
        shouldReload: false,
      });
    },
  );

  await startEsbuildPromise;
}

function printErrorMessages(errorMessages: ErrorMessage[]) {
  console.clear();
  if (!errorMessages.length) {
    console.log("No errors");
    return;
  }
  console.log(`Errors (${errorMessages.length}):`);
  errorMessages.forEach((errorMessage) => {
    console.log(
      `${errorMessage.absoluteFile}(${errorMessage.line},${errorMessage.column}): ${errorMessage.text}`,
    );
  });
}

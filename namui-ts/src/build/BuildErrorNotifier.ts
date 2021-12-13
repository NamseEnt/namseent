import { BuildServerConnection } from "./BuildServerConnection";
import { ErrorMessage } from "./type";

export class BuildErrorNotifier {
  private readonly div = document.createElement("div");
  constructor(buildServerConnection: BuildServerConnection) {
    this.onBuildServerConnectionError =
      this.onBuildServerConnectionError.bind(this);
    buildServerConnection.addEventListener(
      "error",
      this.onBuildServerConnectionError
    );

    this.div.style.position = "absolute";
    document.body.appendChild(this.div);
  }
  private onBuildServerConnectionError({
    errorMessages,
  }: {
    errorMessages: ErrorMessage[];
  }) {
    while (this.div.lastChild) {
      this.div.removeChild(this.div.lastChild);
    }
    if (!errorMessages.length) {
      return;
    }
    errorMessages.forEach(
      ({ column, absoluteFile, relativeFile, line, text }) => {
        const container = document.createElement("div");
        const linkCopyElement = document.createElement("span");
        const link = `${relativeFile}(${line}:${column})`;
        linkCopyElement.style.color = "blue";
        linkCopyElement.style.cursor = "pointer";
        linkCopyElement.innerText = `${relativeFile}(${line}:${column})`;
        linkCopyElement.onclick = () => {
          navigator.clipboard.writeText(link);
        };
        container.appendChild(linkCopyElement);
        container.appendChild(document.createTextNode(text));
        this.div.appendChild(container);
      }
    );
  }
}

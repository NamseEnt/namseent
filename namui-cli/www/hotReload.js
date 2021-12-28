const errorMessagesElement = createElement();
const websocket = createWebSocket();

function createElement() {
  const element = document.createElement("div");
  element.style.display = "none";
  element.style.position = "absolute";
  element.style.top = 0;
  element.style.right = 0;
  element.style.bottom = 0;
  element.style.left = 0;
  element.style.backgroundColor = "rgba(0, 0, 0, 0.8)";
  element.style.padding = "1rem";
  element.style.overflow = "auto";
  element.style.color = "white";
  document.body.append(element);
  return element;
}

function displayErrorMessages(errorMessages) {
  errorMessagesElement.innerHTML = "";
  errorMessagesElement.style.display = errorMessages.length ? "block" : "none";

  const usageElement = document.createElement("div");
  usageElement.innerText = "Click to copy path";
  usageElement.style.marginBottom = "0.5rem";
  errorMessagesElement.append(usageElement);

  errorMessages.forEach(
    ({ column, absoluteFile, relativeFile, line, text }) => {
      const link = `${relativeFile}(${line}:${column})`;
      const element = document.createElement("div");
      element.style.marginBottom = "0.5rem";

      const textElement = document.createElement("div");
      textElement.innerText = text;
      element.append(textElement);

      const linkElement = document.createElement("div");
      linkElement.innerText = `--> ${link}`;
      linkElement.style.color = "aqua";
      linkElement.style.cursor = "pointer";
      linkElement.style.marginLeft = "0.5rem";
      linkElement.onclick = () => {
        navigator.clipboard.writeText(link).then(() => {
          const copiedElement = document.createElement("div");
          copiedElement.innerText = "Copied!";
          copiedElement.style.color = "khaki";
          element.append(copiedElement);
        });
      };
      element.append(linkElement);

      errorMessagesElement.appendChild(element);
    },
  );
}

function createWebSocket() {
  const websocket = new WebSocket(`ws://${location.host}/hotReload`);
  websocket.onopen = () => console.log("connected");
  websocket.onclose = () => console.log("connected");
  websocket.onmessage = (message) => {
    const namuiMessage = JSON.parse(message.data);
    switch (namuiMessage.type) {
      case "error": {
        displayErrorMessages(namuiMessage.errorMessages);
        break;
      }

      case "reload": {
        location.reload();
        break;
      }
    }
  };
}

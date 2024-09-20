console.log("inserted script executed");

/**
 * Protocol
 * - * bytes: File Name in String. No data if no file selected
 * - * bytes: File Data
 */

const inputElement = document.createElement("input");
inputElement.type = "file";
inputElement.multiple = false;
inputElement.onchange = async (event) => {
    /// @ts-check
    /** @type {File} */
    const file = event.target.files[0];
    if (!file) {
        namui_sendData(new ArrayBuffer(0));
        return;
    }
    const nameBytes = new TextEncoder().encode(file.name).buffer;
    namui_sendData(nameBytes);

    const fileBytes = await file.arrayBuffer();
    namui_sendData(fileBytes);
};
document.body.appendChild(inputElement);

inputElement.click();

function namui_onDrop() {
    inputElement.remove();
}

function namui_onData() {
    return;
}

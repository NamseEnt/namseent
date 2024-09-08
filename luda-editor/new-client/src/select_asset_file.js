console.log("inserted script executed");

/**
 * Protocol
 * - 4 bytes: File Name Length in Int32
 *    - bigger than 0 for file name length
 *    - -1 for no file selected
 * - * bytes: File Name in String
 * - 4 bytes: File Data Length in Int32
 * - * bytes: File Data Chunks
 *    - 4 bytes: File Data Chunk Length in Int32
 *       - bigger than 0 for file data chunk length
 *       - 0 for no more data
 *    - * bytes: File Data Chunk in Bytes
 */
const NO_FILE_SELECTED = -1;
const CHUNK_SIZE = 0xffff;
let aborted = false;

/// @ts-check
/** @param {number} num*/
function encodeNumberToInt32Bytes(num) {
    let bytes = new ArrayBuffer(4);
    new DataView(bytes).setInt32(0, num);
    return bytes;
}

const inputElement = document.createElement("input");
inputElement.type = "file";
inputElement.multiple = false;
inputElement.onchange = async (event) => {
    /// @ts-check
    /** @type {File} */
    const file = event.target.files[0];
    if (!file) {
        namui_sendData(encodeNumberToInt32Bytes(NO_FILE_SELECTED));
        return;
    }
    const nameBytes = new TextEncoder().encode(file.name).buffer;
    namui_sendData(encodeNumberToInt32Bytes(nameBytes.byteLength));
    namui_sendData(nameBytes);

    namui_sendData(encodeNumberToInt32Bytes(file.size));
    const stream = file.stream();
    const reader = stream.getReader();

    while (!aborted) {
        const result = await reader.read();
        if (result.done) {
            namui_sendData(encodeNumberToInt32Bytes(0));
            break;
        }

        const buffer = result.value.buffer;
        for (let offset = 0; offset < buffer.byteLength; offset += CHUNK_SIZE) {
            const chunk = buffer.slice(offset, offset + CHUNK_SIZE);
            namui_sendData(encodeNumberToInt32Bytes(chunk.byteLength));
            namui_sendData(chunk);
        }
    }
};
document.body.appendChild(inputElement);

inputElement.click();

function namui_onDrop() {
    aborted = true;
    inputElement.remove();
}

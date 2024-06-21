console.log("inserted script executed");

const jsonBytes = new TextEncoder().encode(
    JSON.stringify(
        [
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
            {
                name: "John Doe",
                email: "asdks@com",
                id: "1234567890",
                age: "25",
            },
        ],
        null,
        2,
    ),
);
namui_sendData(jsonBytes);

function namui_onDrop() {
    console.log("namui_onDrop");
}

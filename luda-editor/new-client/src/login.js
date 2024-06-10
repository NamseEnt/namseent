console.log("inserted script executed");

const script = document.createElement("script");
script.src = "https://accounts.google.com/gsi/client";
script.async = true;
document.body.appendChild(script);

const gIdOnload = document.createElement("div");

gIdOnload.id = "g_id_onload";
gIdOnload.dataset.client_id =
    "857257861263-96dkj0a5mhihgbsh663qi54ko1us7gf9.apps.googleusercontent.com";
gIdOnload.dataset.context = "signin";
gIdOnload.dataset.ux_mode = "popup";
gIdOnload.dataset.callback = "done";
gIdOnload.dataset.auto_select = "true";
gIdOnload.dataset.close_on_tap_outside = "false";
gIdOnload.dataset.itp_support = "true";
document.body.appendChild(gIdOnload);

function done(data) {
    console.log("on done", data);
    const jsonBytes = new TextEncoder().encode(JSON.stringify(data));
    namui_sendData(jsonBytes);
}

// const jsonBytes = new TextEncoder().encode(
//     JSON.stringify([
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//         {
//             name: "John Doe",
//             email: "asdks@com",
//             id: "1234567890",
//             age: "25",
//         },
//     ]),
// );
// namui_sendData(jsonBytes);

function namui_onDrop() {
    console.log("namui_onDrop");
    script.remove();
    gIdOnload.remove();
}

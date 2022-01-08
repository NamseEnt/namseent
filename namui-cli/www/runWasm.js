async function run() {
    // await wasm_bindgen("/bundle_bg.wasm");
    // const CanvasKit = await CanvasKitInit({
    //     locateFile: (file) => "./canvaskit-wasm/" + file,
    // });
    const CanvasKit = await CanvasKitInit({
        locateFile: (file) => "https://jsfiddle.skia.org/res/" + file,
    });
    // globalThis.CanvasKit = CanvasKit;
    // globalThis.getCanvasKit = () => CanvasKit;
    // let sum = 0;
    // const totalTrial = 1000;

    // sum = 0;

    // for (let trial = 0; trial < totalTrial; trial += 1) {
    //     const now = performance.now();

    //     const array = [];
    //     for (let i = 0; i < 10000; i += 1) {
    //         array.push({
    //             x: i,
    //             now: Date.now(),
    //         });
    //     }

    //     const end = performance.now();
    //     // console.log(`elapsed ${end - now}ms, ${array.length}, ${array[0].x}`);
    //     sum += end - now;
    // }
    // console.log(`average ${sum / totalTrial}ms`);

    // sum = 0;
    // const array = [];
    // for (let i = 0; i < 10000; i += 1) {
    //     array.push({
    //         x: i,
    //         now: Date.now(),
    //     });
    // }
    // for (let trial = 0; trial < totalTrial; trial += 1) {
    //     const now = performance.now();

    //     for (let i = 0; i < 10000; i += 1) {
    //         array[i].x += 1;
    //     }

    //     const end = performance.now();
    //     // console.log(`elapsed ${end - now}ms, ${array.length}, ${array[0].x}`);
    //     sum += end - now;
    // }
    // console.log(`average ${sum / totalTrial}ms`);
    // wasm_bindgen.start();


    // const buffer = await fetch("http://localhost:3030/resources/images/오하연-어깨축-눈감은(.png").then((response) => response.arrayBuffer());
    // const image = await decodeImageFromBuffer(buffer, "image/png");

    const canvas = document.createElement('canvas');
    // const context = canvas.getContext('2d');
    canvas.width = 1000;
    canvas.height = 1000;
    document.body.appendChild(canvas);
    document.body.style.overflow = 'auto';

    const surface = CanvasKit.MakeCanvasSurface(canvas);

    let images = await Promise.all([
        "http://localhost:3030/resources/images/오하연-어깨축-눈감은(.png",
        "http://localhost:3030/resources/images/오하연-고개갸웃-호기심.png",
        // "http://localhost:3030/resources/images/오하연-두손모으고-걱정.png",
        // "http://localhost:3030/resources/images/오하연-탁자폰-무표정.png",
        // "http://localhost:3030/resources/images/피디-기본-눈치.png",
        // "http://localhost:3030/resources/images/피디-양팔왓-ㅜㅅㅜ.png",
    ].map(async (url) => {
        const buffer = await fetch(url).then((response) => response.arrayBuffer());
        const image = await decodeImageFromBuffer(buffer, "image/png");
        const canvasKitImage = surface.makeImageFromTextureSource(image, {
            alphaType: CanvasKit.AlphaType.Premul,
            colorType: CanvasKit.ColorType.RGBA_8888,
            height: image.displayHeight,
            width: image.displayWidth,
        });
        return {
            canvasKitImage,
            height: image.displayHeight,
            width: image.displayWidth,
        }
    }));


    function tick(canvas) {
        // surface.requestAnimationFrame(tick);
        // console.log('tick');
        images.forEach(({ canvasKitImage, width, height }, index) => {
            // console.log(index, width, height);
            canvas.drawImageRectOptions(canvasKitImage, [0, 0, width, height], [0, index * 100, 100, index * 100 + 100],
                CanvasKit.FilterMode.Linear,
                CanvasKit.MipmapMode.Linear,
            );

            // canvas.drawImageOptions(canvasKitImage, 0, index * 100,
            //     CanvasKit.FilterMode.Linear,
            //     CanvasKit.MipmapMode.Linear);
        })

    }
    surface.requestAnimationFrame(tick);
}

run();

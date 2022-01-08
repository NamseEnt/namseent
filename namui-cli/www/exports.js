async function decodeImageFromBuffer(data, contentType, width, height) {
    const imageDecoder = new ImageDecoder({
        type: contentType,
        data,
        premultiplyAlpha: "premultiply",
        desiredWidth: width,
        desiredHeight: height,
        colorSpaceConversion: 'default',
    });
    const {
        image,
        complete,
    } = await imageDecoder.decode();
    console.log(image);

    imageDecoder.close();

    if (!complete) {
        throw new Error('Image decoding failed');
    }
    return image;
}

async function decodeImageFromUrl(url) {
    const image = new Image();
    image.crossOrigin = 'anonymous';
    return new Promise((resolve, reject) => {
        image.onload = () => {
            createImageBitmap(image).then(resolve).catch(reject);
        };
        image.onerror = reject;
        image.src = url;
    });
}
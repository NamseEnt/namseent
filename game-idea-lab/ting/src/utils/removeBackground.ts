export function removeWhiteBackground(
    scene: Phaser.Scene,
    key: string,
    threshold: number = 240
): void {
    const texture = scene.textures.get(key);
    const canvas = scene.textures.createCanvas(key + '_transparent', texture.getSourceImage().width, texture.getSourceImage().height);
    const context = canvas?.context;
    const source = texture.getSourceImage() as HTMLImageElement;
    
    if (!canvas?.canvas || !context) return; // null 체크
    
    context.drawImage(source, 0, 0);
    
    const imageData = context.getImageData(0, 0, canvas.canvas.width, canvas.canvas.height);
    const data = imageData.data;
    
    for (let i = 0; i < data.length; i += 4) {
        const r = data[i];
        const g = data[i + 1];
        const b = data[i + 2];
        
        if (r > threshold && g > threshold && b > threshold) {
            data[i + 3] = 0;
        }
    }
    
    context.putImageData(imageData, 0, 0);
    canvas.refresh();
    
    scene.textures.remove(key);
    if (canvas.canvas) {
        scene.textures.addCanvas(key, canvas.canvas);
    }
}
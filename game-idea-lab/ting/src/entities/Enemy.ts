import Phaser from 'phaser';

export class Enemy extends Phaser.GameObjects.Container {
    private sprite: Phaser.GameObjects.Image;
    private scene: Phaser.Scene;
    private active: boolean = true;
    private type: string;
    
    constructor(scene: Phaser.Scene, x: number, y: number, type: string = 'full', scale: number = 0.15) {
        super(scene, x, y);
        this.scene = scene;
        this.type = type;
        
        this.sprite = scene.add.image(0, 0, 'enemy');
        this.sprite.setScale(scale);
        this.add(this.sprite);
        
        scene.add.existing(this);
        this.setDepth(5);
        
        // 드래그를 위한 인터랙티브 설정
        this.setSize(100, 100);
        this.setInteractive();
    }
    
    hit() {
        this.active = false;
        
        this.sprite.setTint(0xff0000);
        
        this.scene.tweens.add({
            targets: this.sprite,
            alpha: 0,
            rotation: 0.5,
            y: this.y + 50,
            duration: 500,
            ease: 'Power2',
            onComplete: () => {
                this.destroy();
            }
        });
    }
    
    isActive(): boolean {
        return this.active;
    }
    
    update() {
        
    }
    
    getCurrentScale(): number {
        return this.sprite.scale;
    }
    
    adjustScale(factor: number) {
        const newScale = this.sprite.scale * factor;
        this.sprite.setScale(newScale);
    }
}
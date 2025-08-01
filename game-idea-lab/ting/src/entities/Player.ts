import Phaser from 'phaser';

export class Player extends Phaser.GameObjects.Container {
    private sprite: Phaser.GameObjects.Image;
    private isHiding: boolean = true;
    private scene: Phaser.Scene;
    private recoilTween?: Phaser.Tweens.Tween;
    
    constructor(scene: Phaser.Scene, x: number, y: number) {
        super(scene, x, y);
        this.scene = scene;
        
        this.sprite = scene.add.image(0, 0, 'playerStandby');
        this.sprite.setScale(0.3);
        this.add(this.sprite);
        
        scene.add.existing(this);
        this.setDepth(10);
    }
    
    isInCover(): boolean {
        return this.isHiding;
    }
    
    toggleCover() {
        this.isHiding = !this.isHiding;
        
        if (this.isHiding) {
            this.sprite.setTexture('playerStandby');
            this.scene.tweens.add({
                targets: this,
                x: 300,
                duration: 300,
                ease: 'Power2'
            });
        } else {
            this.sprite.setTexture('playerShoot');
            this.scene.tweens.add({
                targets: this,
                x: 350,
                duration: 300,
                ease: 'Power2'
            });
        }
    }
    
    shoot() {
        if (this.recoilTween && this.recoilTween.isPlaying()) {
            return;
        }
        
        const originalX = this.x;
        this.recoilTween = this.scene.tweens.add({
            targets: this,
            x: originalX - 10,
            duration: 50,
            yoyo: true,
            ease: 'Power1'
        });
        
        this.scene.tweens.add({
            targets: this.sprite,
            rotation: -0.05,
            duration: 50,
            yoyo: true,
            ease: 'Power1'
        });
    }
    
    hit() {
        this.sprite.setTint(0xff0000);
        
        this.scene.tweens.add({
            targets: this,
            x: this.x + Phaser.Math.Between(-5, 5),
            y: this.y + Phaser.Math.Between(-5, 5),
            duration: 50,
            repeat: 3,
            yoyo: true,
            onComplete: () => {
                this.sprite.clearTint();
            }
        });
    }
    
    update() {
        
    }
}
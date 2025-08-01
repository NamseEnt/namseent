import Phaser from 'phaser';

export class Enemy extends Phaser.GameObjects.Container {
    private sprite: Phaser.GameObjects.Image;
    private scene: Phaser.Scene;
    private shootTimer?: Phaser.Time.TimerEvent;
    private initialDelay: number = Phaser.Math.Between(1000, 3000);
    private shootInterval: number = Phaser.Math.Between(2000, 4000);
    private active: boolean = true;
    
    constructor(scene: Phaser.Scene, x: number, y: number) {
        super(scene, x, y);
        this.scene = scene;
        
        this.sprite = scene.add.image(0, 0, 'enemy');
        this.sprite.setScale(0.15);
        this.add(this.sprite);
        
        scene.add.existing(this);
        this.setDepth(5);
        
        this.startShooting();
    }
    
    private startShooting() {
        this.scene.time.delayedCall(this.initialDelay, () => {
            if (!this.active) return;
            
            this.shoot();
            
            this.shootTimer = this.scene.time.addEvent({
                delay: this.shootInterval,
                callback: this.shoot,
                callbackScope: this,
                loop: true
            });
        });
    }
    
    private shoot() {
        if (!this.active) return;
        
        this.scene.tweens.add({
            targets: this.sprite,
            scaleX: 0.17,
            scaleY: 0.13,
            duration: 100,
            yoyo: true,
            ease: 'Power1'
        });
        
        const muzzleFlash = this.scene.add.graphics();
        muzzleFlash.fillStyle(0xffff00, 1);
        muzzleFlash.fillCircle(this.x - 30, this.y, 15);
        
        this.scene.tweens.add({
            targets: muzzleFlash,
            alpha: 0,
            duration: 100,
            onComplete: () => muzzleFlash.destroy()
        });
        
        this.emit('shoot');
    }
    
    hit() {
        this.active = false;
        
        if (this.shootTimer) {
            this.shootTimer.destroy();
        }
        
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
}
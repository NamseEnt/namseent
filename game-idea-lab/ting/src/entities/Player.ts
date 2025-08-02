import Phaser from 'phaser';

type PlayerState = 'hiding' | 'low-ready' | 'shoot';

export class Player extends Phaser.GameObjects.Container {
    private sprite: Phaser.GameObjects.Image;
    private state: PlayerState = 'hiding';
    private scene: Phaser.Scene;
    private recoilTween?: Phaser.Tweens.Tween;
    private isShooting: boolean = false;
    
    constructor(scene: Phaser.Scene, x: number, y: number) {
        super(scene, x, y);
        this.scene = scene;
        
        // 기본 상태를 low-ready로 시작
        this.state = 'low-ready';
        this.sprite = scene.add.image(0, 0, 'playerLowReady');
        this.sprite.setScale(0.3);
        this.add(this.sprite);
        
        scene.add.existing(this);
        this.setDepth(10);
        
        // 시작 위치 설정 (low-ready 위치)
        this.x = 350;
    }
    
    isInCover(): boolean {
        return this.state === 'hiding';
    }
    
    getState(): PlayerState {
        return this.state;
    }
    
    setCover(isHiding: boolean) {
        if (isHiding) {
            this.state = 'hiding';
            this.sprite.setTexture('playerStandby');
            this.scene.tweens.add({
                targets: this,
                x: 300,
                duration: 300,
                ease: 'Power2'
            });
        } else {
            this.state = 'low-ready';
            this.sprite.setTexture('playerLowReady');
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
        
        // 첫 발사 시에만 shoot 상태로 변경
        if (!this.isShooting) {
            this.isShooting = true;
            this.state = 'shoot';
            this.sprite.setTexture('playerShoot');
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
            x: this.sprite.x + Phaser.Math.Between(-3, 3),
            duration: 50,
            yoyo: true,
            ease: 'Power1'
        });
    }
    
    stopShooting() {
        this.isShooting = false;
        if (this.state === 'shoot') {
            this.state = 'low-ready';
            this.sprite.setTexture('playerLowReady');
        }
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
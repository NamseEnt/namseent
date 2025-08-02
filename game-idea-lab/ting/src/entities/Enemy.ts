import Phaser from 'phaser';

export class Enemy extends Phaser.GameObjects.Container {
    private sprite: Phaser.GameObjects.Image;
    public scene: Phaser.Scene;
    public active: boolean = true;
    public type: string;
    private respawnTimer?: Phaser.Time.TimerEvent;
    private originalScale: number;
    
    constructor(scene: Phaser.Scene, x: number, y: number, type: string = 'full', scale: number = 0.15) {
        super(scene, x, y);
        this.scene = scene;
        this.type = type;
        this.originalScale = scale;
        
        this.sprite = scene.add.image(0, 0, 'enemy');
        this.sprite.setScale(scale);
        this.add(this.sprite);
        
        scene.add.existing(this as any);
        this.setDepth(5);
        
        // 드래그를 위한 인터랙티브 설정
        this.setSize(100, 100);
        this.setInteractive();
    }
    
    hit() {
        if (!this.active) return; // 이미 죽은 상태면 무시
        
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
                this.startRespawnTimer();
            }
        });
    }
    
    private startRespawnTimer() {
        // 3~5초 랜덤 시간 후 리스폰
        const respawnTime = Phaser.Math.Between(3000, 5000);
        
        this.respawnTimer = this.scene.time.delayedCall(respawnTime, () => {
            this.respawn();
        });
    }
    
    private respawn() {
        // 원래 상태로 복구
        this.active = true;
        this.sprite.clearTint();
        this.sprite.setAlpha(1);
        this.sprite.setRotation(0);
        this.sprite.setScale(this.originalScale);
        this.sprite.y = 0; // 컨테이너 기준 원래 위치
        
        // 리스폰 애니메이션 (페이드 인)
        this.sprite.setAlpha(0);
        this.scene.tweens.add({
            targets: this.sprite,
            alpha: 1,
            duration: 300,
            ease: 'Power2'
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
    
    destroy(fromScene?: boolean) {
        // 타이머가 있으면 정리
        if (this.respawnTimer) {
            this.respawnTimer.destroy();
            this.respawnTimer = undefined;
        }
        super.destroy(fromScene);
    }
}
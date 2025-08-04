import Phaser from 'phaser';

export class Enemy extends Phaser.GameObjects.Container {
    private sprite: Phaser.GameObjects.Image;
    public scene: Phaser.Scene;
    public active: boolean = true;
    public type: string;
    private respawnTimer?: Phaser.Time.TimerEvent;
    private originalScale: number;
    
    // HP 시스템
    private maxHP: number = 3; // 기본 3방
    private currentHP: number = 3;
    private hpBar?: Phaser.GameObjects.Graphics;
    private hpBackground?: Phaser.GameObjects.Graphics;
    
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
        
        // HP 바 생성
        this.createHPBar();
    }
    
    hit() {
        if (!this.active) return; // 이미 죽은 상태면 무시
        
        // HP 감소
        this.currentHP--;
        this.updateHPBar();
        
        // 피격 애니메이션
        this.sprite.setTint(0xff0000);
        this.scene.tweens.add({
            targets: this.sprite,
            scaleX: this.sprite.scaleX * 1.1,
            scaleY: this.sprite.scaleY * 1.1,
            duration: 100,
            yoyo: true,
            onComplete: () => {
                this.sprite.clearTint();
            }
        });
        
        // HP가 0이 되면 사망
        if (this.currentHP <= 0) {
            this.die();
        }
    }
    
    private die() {
        this.active = false;
        this.hideHPBar();
        this.sprite.setTint(0x666666);
        
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
        this.currentHP = this.maxHP; // HP 전체 회복
        this.sprite.clearTint();
        this.sprite.setAlpha(1);
        this.sprite.setRotation(0);
        this.sprite.setScale(this.originalScale);
        this.sprite.y = 0; // 컨테이너 기준 원래 위치
        
        // HP 바 다시 표시
        this.showHPBar();
        this.updateHPBar();
        
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
        
        // HP 바 정리
        if (this.hpBar) {
            this.hpBar.destroy();
        }
        if (this.hpBackground) {
            this.hpBackground.destroy();
        }
        
        super.destroy(fromScene);
    }
    
    private createHPBar() {
        const barWidth = 40;
        const barHeight = 4;
        const barOffsetY = -30; // 적 위쪽에 표시
        
        // HP 바 배경 (회색)
        this.hpBackground = this.scene.add.graphics();
        this.hpBackground.fillStyle(0x333333);
        this.hpBackground.fillRect(-barWidth/2, barOffsetY, barWidth, barHeight);
        this.add(this.hpBackground);
        
        // HP 바 (빨간색 → 노란색 → 초록색)
        this.hpBar = this.scene.add.graphics();
        this.add(this.hpBar);
        this.updateHPBar();
    }
    
    private updateHPBar() {
        if (!this.hpBar) return;
        
        const barWidth = 40;
        const barHeight = 4;  
        const barOffsetY = -30;
        
        this.hpBar.clear();
        
        if (this.currentHP > 0) {
            const hpPercentage = this.currentHP / this.maxHP;
            const currentBarWidth = barWidth * hpPercentage;
            
            // HP에 따른 색상 변화
            let color = 0x00ff00; // 초록색 (풀 HP)
            if (hpPercentage <= 0.33) {
                color = 0xff0000; // 빨간색 (낮은 HP)
            } else if (hpPercentage <= 0.66) {
                color = 0xffff00; // 노란색 (중간 HP)
            }
            
            this.hpBar.fillStyle(color);
            this.hpBar.fillRect(-barWidth/2, barOffsetY, currentBarWidth, barHeight);
        }
    }
    
    private hideHPBar() {
        if (this.hpBar) this.hpBar.setVisible(false);
        if (this.hpBackground) this.hpBackground.setVisible(false);
    }
    
    private showHPBar() {
        if (this.hpBar) this.hpBar.setVisible(true);
        if (this.hpBackground) this.hpBackground.setVisible(true);
    }
    
    // HP 관련 게터/세터
    getCurrentHP(): number {
        return this.currentHP;
    }
    
    getMaxHP(): number {
        return this.maxHP;
    }
    
    setMaxHP(hp: number) {
        this.maxHP = hp;
        this.currentHP = hp;
        this.updateHPBar();
    }
}
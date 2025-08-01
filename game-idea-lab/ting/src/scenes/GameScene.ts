import Phaser from 'phaser';
import { Player } from '../entities/Player';
import { Enemy } from '../entities/Enemy';
import { removeWhiteBackground } from '../utils/removeBackground';

export class GameScene extends Phaser.Scene {
    private player!: Player;
    private enemies: Enemy[] = [];
    private crosshair!: Phaser.GameObjects.Graphics;
    private crosshairSize: number = 20;
    private background!: Phaser.GameObjects.Image;
    private hideWall!: Phaser.GameObjects.Image;
    
    // 적 배치 영역 설정용
    private isSettingEnemyArea: boolean = false;
    private enemyAreaGraphics!: Phaser.GameObjects.Graphics;
    private enemyAreaStart: { x: number, y: number } | null = null;
    
    // 적 조정 모드
    private isAdjustMode: boolean = false;
    
    // 마스크 시각화
    private maskGraphics: Phaser.GameObjects.Graphics[] = [];
    
    constructor() {
        super({ key: 'GameScene' });
    }

    preload() {
        this.load.image('building', 'building-background.png');
        this.load.image('hideWall', 'hide-wall.png');
        this.load.image('enemy', 'enemy.png');
        this.load.image('playerStandby', 'standby.png');
        this.load.image('playerShoot', 'shoot.png');
    }

    create() {
        removeWhiteBackground(this, 'enemy');
        removeWhiteBackground(this, 'playerStandby');
        removeWhiteBackground(this, 'playerShoot');
        removeWhiteBackground(this, 'hideWall');
        
        this.background = this.add.image(600, 300, 'building');
        this.background.setScale(0.8);
        
        this.hideWall = this.add.image(8, 334, 'hideWall');
        this.hideWall.setScale(0.7);
        
        this.player = new Player(this, 300, 400);
        
        // 적 배치 설정 (조정된 위치와 scale, 마스크)
        const enemyPositions = [
            { 
                x: 624, y: 243, type: 'window', scale: 0.04725,
                maskX: 611, maskY: 211, maskW: 27, maskH: 31 
            },
            { 
                x: 637, y: 395, type: 'pillar-left', scale: 0.0497,
                maskX: 620, maskY: 361, maskW: 19, maskH: 68 
            },
            { 
                x: 736, y: 134, type: 'rooftop', scale: 0.05178,
                maskX: 721, maskY: 98, maskW: 36, maskH: 35 
            },
            { 
                x: 596, y: 397, type: 'pillar-right', scale: 0.044747,
                maskX: 590, maskY: 365, maskW: 20, maskH: 60 
            }
        ];
        
        enemyPositions.forEach((pos, index) => {
            const enemy = new Enemy(this, pos.x, pos.y, pos.type, pos.scale);
            
            // 마스크 적용
            const mask = this.add.graphics();
            mask.fillRect(pos.maskX, pos.maskY, pos.maskW, pos.maskH);
            enemy.setMask(mask.createGeometryMask());
            
            // 마스크 시각화용 그래픽스 (숨김)
            const maskVisual = this.add.graphics();
            maskVisual.setVisible(false);
            maskVisual.setDepth(200);
            this.maskGraphics.push(maskVisual);
            
            // 마스크 정보를 enemy에 저장
            (enemy as any).maskData = { x: pos.maskX, y: pos.maskY, w: pos.maskW, h: pos.maskH };
            (enemy as any).maskGraphics = mask;
            
            this.enemies.push(enemy);
        });
        
        // 적 조정 모드 토글 (Q 키)
        this.input.keyboard?.on('keydown-Q', () => {
            this.isAdjustMode = !this.isAdjustMode;
            console.log(`적 조정 모드: ${this.isAdjustMode ? '활성화' : '비활성화'}`);
            
            if (this.isAdjustMode) {
                console.log('- 드래그: 위치 이동');
                console.log('- R 키: 크기 확대 (10%)');
                console.log('- F 키: 크기 축소 (10%)');
                console.log('- M 키: 마스크 모드 전환');
                
                // 드래그 활성화
                this.enemies.forEach(enemy => {
                    this.input.setDraggable(enemy);
                });
            } else {
                // 드래그 비활성화
                this.enemies.forEach(enemy => {
                    this.input.setDraggable(enemy, false);
                });
                
                // 마스크 시각화 끄기
                this.maskGraphics.forEach(g => g.setVisible(false));
            }
        });
        
        // 마스크 모드 (M 키)
        let isMaskMode = false;
        this.input.keyboard?.on('keydown-M', () => {
            if (!this.isAdjustMode) return;
            
            isMaskMode = !isMaskMode;
            console.log(`마스크 모드: ${isMaskMode ? '활성화' : '비활성화'}`);
            
            if (isMaskMode) {
                console.log('- 마스크 영역이 녹색으로 표시됩니다');
                console.log('- W/A/S/D: 마스크 이동');
                console.log('- I/J/K/L: 마스크 크기 조정');
                
                // 마스크 시각화
                this.enemies.forEach((enemy, index) => {
                    const maskData = (enemy as any).maskData;
                    const visual = this.maskGraphics[index];
                    visual.clear();
                    visual.lineStyle(2, 0x00ff00, 1);
                    visual.strokeRect(maskData.x, maskData.y, maskData.w, maskData.h);
                    visual.setVisible(true);
                });
            } else {
                // 마스크 숨기기
                this.maskGraphics.forEach(g => g.setVisible(false));
            }
        });
        
        // 마스크 이동 (W/A/S/D)
        const maskMoveSpeed = 2;
        this.input.keyboard?.on('keydown-W', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.moveMasks(0, -maskMoveSpeed);
        });
        this.input.keyboard?.on('keydown-A', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.moveMasks(-maskMoveSpeed, 0);
        });
        this.input.keyboard?.on('keydown-S', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.moveMasks(0, maskMoveSpeed);
        });
        this.input.keyboard?.on('keydown-D', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.moveMasks(maskMoveSpeed, 0);
        });
        
        // 마스크 크기 조정 (I/J/K/L)
        const maskSizeSpeed = 2;
        this.input.keyboard?.on('keydown-I', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.resizeMasks(0, -maskSizeSpeed);
        });
        this.input.keyboard?.on('keydown-K', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.resizeMasks(0, maskSizeSpeed);
        });
        this.input.keyboard?.on('keydown-J', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.resizeMasks(-maskSizeSpeed, 0);
        });
        this.input.keyboard?.on('keydown-L', () => {
            if (!this.isAdjustMode || !isMaskMode) return;
            this.resizeMasks(maskSizeSpeed, 0);
        });
        
        // 드래그 이벤트
        this.input.on('drag', (pointer: Phaser.Input.Pointer, gameObject: Phaser.GameObjects.GameObject, dragX: number, dragY: number) => {
            if (this.isAdjustMode && this.enemies.includes(gameObject as Enemy)) {
                gameObject.x = dragX;
                gameObject.y = dragY;
            }
        });
        
        // 드래그 종료 시 위치 로그
        this.input.on('dragend', (pointer: Phaser.Input.Pointer, gameObject: Phaser.GameObjects.GameObject) => {
            if (this.isAdjustMode && this.enemies.includes(gameObject as Enemy)) {
                const enemy = gameObject as Enemy;
                const index = this.enemies.indexOf(enemy);
                const type = enemyPositions[index].type;
                console.log(`적 ${index + 1} (${type}) 새 위치: x=${Math.round(enemy.x)}, y=${Math.round(enemy.y)}, scale=${enemy.getCurrentScale()}`);
            }
        });
        
        // 스케일 조정 단축키 (R: 확대, F: 축소)
        this.input.keyboard?.on('keydown-R', () => {
            if (!this.isAdjustMode) return;
            
            this.enemies.forEach(enemy => {
                if (enemy.getBounds().contains(this.input.activePointer.x, this.input.activePointer.y)) {
                    enemy.adjustScale(1.1);
                    const index = this.enemies.indexOf(enemy);
                    const type = enemyPositions[index].type;
                    console.log(`적 ${index + 1} (${type}) scale: ${enemy.getCurrentScale()}`);
                }
            });
        });
        
        this.input.keyboard?.on('keydown-F', () => {
            if (!this.isAdjustMode) return;
            
            this.enemies.forEach(enemy => {
                if (enemy.getBounds().contains(this.input.activePointer.x, this.input.activePointer.y)) {
                    enemy.adjustScale(0.9);
                    const index = this.enemies.indexOf(enemy);
                    const type = enemyPositions[index].type;
                    console.log(`적 ${index + 1} (${type}) scale: ${enemy.getCurrentScale()}`);
                }
            });
        });
        
        // 동적 크로스헤어 생성
        this.crosshair = this.add.graphics();
        this.crosshair.setDepth(100);
        
        // 초기 위치를 화면 중앙으로 설정
        this.crosshair.setPosition(600, 300);
        this.drawCrosshair(0xffffff);
        
        // 마우스가 게임 영역에 들어왔을 때
        this.input.on('pointerover', () => {
            this.crosshair.setVisible(true);
        });
        
        // 마우스가 게임 영역을 벗어났을 때
        this.input.on('pointerout', () => {
            this.crosshair.setVisible(false);
        });
        
        this.input.on('pointermove', (pointer: Phaser.Input.Pointer) => {
            this.crosshair.setPosition(pointer.x, pointer.y);
            this.crosshair.setVisible(true);
        });
        
        this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {
            if (this.isSettingEnemyArea) return;  // 적 배치 모드에서는 사격 비활성화
            
            if (!this.player.isInCover()) {
                this.player.shoot();
                
                const hitEnemy = this.enemies.find(enemy => {
                    const bounds = enemy.getBounds();
                    return bounds.contains(pointer.x, pointer.y) && enemy.isActive();
                });
                
                if (hitEnemy) {
                    hitEnemy.hit();
                    this.animateCrosshairHit();
                } else {
                    this.createSpark(pointer.x, pointer.y);
                }
                
                this.animateCrosshairRecoil();
            }
        });
        
        this.input.keyboard?.on('keydown-SPACE', () => {
            this.player.toggleCover();
        });
        
        // 적 배치 영역 설정 모드 활성화 (E 키)
        this.input.keyboard?.on('keydown-E', () => {
            this.isSettingEnemyArea = !this.isSettingEnemyArea;
            if (this.isSettingEnemyArea) {
                console.log('적 배치 모드 활성화 - 드래그로 영역을 설정하세요');
                this.crosshair.setVisible(false);
                
                // 영역 표시용 그래픽스 생성
                if (!this.enemyAreaGraphics) {
                    this.enemyAreaGraphics = this.add.graphics();
                    this.enemyAreaGraphics.setDepth(90);
                }
            } else {
                console.log('적 배치 모드 비활성화');
                this.crosshair.setVisible(true);
                if (this.enemyAreaGraphics) {
                    this.enemyAreaGraphics.clear();
                }
            }
        });
        
        // 적 배치 영역 그리기
        this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {
            if (this.isSettingEnemyArea) {
                this.enemyAreaStart = { x: pointer.x, y: pointer.y };
            }
        });
        
        this.input.on('pointermove', (pointer: Phaser.Input.Pointer) => {
            if (this.isSettingEnemyArea && this.enemyAreaStart && pointer.isDown) {
                this.enemyAreaGraphics.clear();
                this.enemyAreaGraphics.lineStyle(2, 0x00ff00, 1);
                this.enemyAreaGraphics.strokeRect(
                    this.enemyAreaStart.x,
                    this.enemyAreaStart.y,
                    pointer.x - this.enemyAreaStart.x,
                    pointer.y - this.enemyAreaStart.y
                );
            }
        });
        
        this.input.on('pointerup', (pointer: Phaser.Input.Pointer) => {
            if (this.isSettingEnemyArea && this.enemyAreaStart) {
                const x1 = Math.min(this.enemyAreaStart.x, pointer.x);
                const y1 = Math.min(this.enemyAreaStart.y, pointer.y);
                const x2 = Math.max(this.enemyAreaStart.x, pointer.x);
                const y2 = Math.max(this.enemyAreaStart.y, pointer.y);
                
                console.log(`적 배치 영역 설정됨:`);
                console.log(`- 좌상단: (${Math.round(x1)}, ${Math.round(y1)})`);
                console.log(`- 우하단: (${Math.round(x2)}, ${Math.round(y2)})`);
                console.log(`- 크기: ${Math.round(x2 - x1)} x ${Math.round(y2 - y1)}`);
                console.log(`\n이 영역에 적을 어떻게 배치하고 싶으신지 알려주세요.`);
                console.log(`예: "창문에서 2명", "기둥 뒤에 1명" 등`);
                
                this.enemyAreaStart = null;
                this.enemyAreaGraphics.clear();
            }
        });
        
        this.enemies.forEach(enemy => {
            enemy.on('shoot', () => {
                if (this.player.isInCover()) {
                    this.createSpark(
                        this.hideWall.x + Phaser.Math.Between(-50, 50),
                        this.hideWall.y + Phaser.Math.Between(-30, 30)
                    );
                } else {
                    if (Math.random() < 0.3) {
                        this.player.hit();
                    } else {
                        this.createSpark(
                            this.player.x + Phaser.Math.Between(-50, 50),
                            this.player.y + Phaser.Math.Between(-50, 50)
                        );
                    }
                }
            });
        });
    }

    private createSpark(x: number, y: number) {
        const spark = this.add.graphics();
        spark.fillStyle(0xffff00, 1);
        
        for (let i = 0; i < 8; i++) {
            const angle = (i / 8) * Math.PI * 2;
            const distance = Phaser.Math.Between(10, 30);
            const px = x + Math.cos(angle) * distance;
            const py = y + Math.sin(angle) * distance;
            spark.fillCircle(px, py, 2);
        }
        
        this.tweens.add({
            targets: spark,
            alpha: 0,
            duration: 200,
            onComplete: () => spark.destroy()
        });
    }

    update() {
        this.player.update();
        this.enemies.forEach(enemy => enemy.update());
        
        // 적 위에 마우스가 있는지 확인하여 크로스헤어 색상 변경
        const pointer = this.input.activePointer;
        const isOverEnemy = this.enemies.some(enemy => {
            const bounds = enemy.getBounds();
            return bounds.contains(pointer.x, pointer.y) && enemy.isActive();
        });
        
        this.updateCrosshairColor(isOverEnemy);
    }
    
    private drawCrosshair(color: number = 0xffffff) {
        this.crosshair.clear();
        
        // 디버깅: 간단한 원 그리기
        this.crosshair.fillStyle(0xffffff, 1);
        this.crosshair.fillCircle(0, 0, 3);
        
        this.crosshair.lineStyle(3, color, 1);  // 더 두껍게
        
        const size = this.crosshairSize;
        const gap = 6;
        
        // 상단 선
        this.crosshair.beginPath();
        this.crosshair.moveTo(0, -size);
        this.crosshair.lineTo(0, -gap);
        this.crosshair.strokePath();
        
        // 하단 선
        this.crosshair.beginPath();
        this.crosshair.moveTo(0, gap);
        this.crosshair.lineTo(0, size);
        this.crosshair.strokePath();
        
        // 좌측 선
        this.crosshair.beginPath();
        this.crosshair.moveTo(-size, 0);
        this.crosshair.lineTo(-gap, 0);
        this.crosshair.strokePath();
        
        // 우측 선
        this.crosshair.beginPath();
        this.crosshair.moveTo(gap, 0);
        this.crosshair.lineTo(size, 0);
        this.crosshair.strokePath();
    }
    
    private updateCrosshairColor(isOverEnemy: boolean) {
        const color = isOverEnemy ? 0xff0000 : 0xffffff;
        this.drawCrosshair(color);
    }
    
    private animateCrosshairRecoil() {
        this.tweens.add({
            targets: this,
            crosshairSize: 30,
            duration: 100,
            yoyo: true,
            ease: 'Power1',
            onUpdate: () => {
                const pointer = this.input.activePointer;
                const isOverEnemy = this.enemies.some(enemy => {
                    const bounds = enemy.getBounds();
                    return bounds.contains(pointer.x, pointer.y) && enemy.isActive();
                });
                const color = isOverEnemy ? 0xff0000 : 0xffffff;
                this.drawCrosshair(color);
            }
        });
    }
    
    private animateCrosshairHit() {
        // 적 명중 시 펄스 효과
        this.tweens.add({
            targets: this.crosshair,
            scaleX: 1.2,
            scaleY: 1.2,
            duration: 150,
            yoyo: true,
            ease: 'Power2'
        });
    }
    
    private moveMasks(dx: number, dy: number) {
        this.enemies.forEach((enemy, index) => {
            const pointer = this.input.activePointer;
            if (enemy.getBounds().contains(pointer.x, pointer.y)) {
                const maskData = (enemy as any).maskData;
                maskData.x += dx;
                maskData.y += dy;
                
                // 마스크 업데이트
                const mask = (enemy as any).maskGraphics;
                mask.clear();
                mask.fillRect(maskData.x, maskData.y, maskData.w, maskData.h);
                enemy.setMask(mask.createGeometryMask());
                
                // 시각화 업데이트
                const visual = this.maskGraphics[index];
                visual.clear();
                visual.lineStyle(2, 0x00ff00, 1);
                visual.strokeRect(maskData.x, maskData.y, maskData.w, maskData.h);
                
                console.log(`마스크 위치: x=${Math.round(maskData.x)}, y=${Math.round(maskData.y)}`);
            }
        });
    }
    
    private resizeMasks(dw: number, dh: number) {
        this.enemies.forEach((enemy, index) => {
            const pointer = this.input.activePointer;
            if (enemy.getBounds().contains(pointer.x, pointer.y)) {
                const maskData = (enemy as any).maskData;
                maskData.w = Math.max(5, maskData.w + dw);
                maskData.h = Math.max(5, maskData.h + dh);
                
                // 마스크 업데이트
                const mask = (enemy as any).maskGraphics;
                mask.clear();
                mask.fillRect(maskData.x, maskData.y, maskData.w, maskData.h);
                enemy.setMask(mask.createGeometryMask());
                
                // 시각화 업데이트
                const visual = this.maskGraphics[index];
                visual.clear();
                visual.lineStyle(2, 0x00ff00, 1);
                visual.strokeRect(maskData.x, maskData.y, maskData.w, maskData.h);
                
                console.log(`마스크 크기: ${Math.round(maskData.w)} x ${Math.round(maskData.h)}`);
            }
        });
    }
}
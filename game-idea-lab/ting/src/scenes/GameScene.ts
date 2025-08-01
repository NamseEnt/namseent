import Phaser from 'phaser';
import { Player } from '../entities/Player';
import { Enemy } from '../entities/Enemy';
import { removeWhiteBackground } from '../utils/removeBackground';

export class GameScene extends Phaser.Scene {
    private player!: Player;
    private enemies: Enemy[] = [];
    private crosshair!: Phaser.GameObjects.Image;
    private background!: Phaser.GameObjects.Image;
    private hideWall!: Phaser.GameObjects.Image;
    
    constructor() {
        super({ key: 'GameScene' });
    }

    preload() {
        this.load.image('building', 'building-background.png');
        this.load.image('hideWall', 'hide-wall.png');
        this.load.image('enemy', 'enemy.png');
        this.load.image('playerStandby', 'standby.png');
        this.load.image('playerShoot', 'shoot.png');
        
        this.load.image('crosshair', 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAABN0lEQVRYhe2WMW7CMBSG36cQVTAwMHbpEXqBHqFn6AW6dWBgYWFhYWFgYGBgYGDo0qVLly5dunTp0laoaqRGTkxiE6kVn/SSN/h773/+/3kC/lPkAJYAVgC2AHYA9gAOAI4ATtJaA7gCuEvLwwWAAYAhgJG0xgAmAKYAZtKaS2sBYLF3pwagAaAFoA2gI60ugB6APoABPvcfABj9te9FW1IpFZVSB6UUmVrSYqVUQSllOebKFxu2F6bG5BzTbdimzTgNPzGdlkUP8Csp0wN09MAx7SX9RIcJ6xhg+mL2TJ2YXQOY1v+b+ImOCeMY4I11YBo0YjZPAO8BPs6TKh6HfRJ7BoiUjXvyHFCS9hNPJRi6SuwCKEvbU4DLPm9PAa77rB5gRXtaykPA6cWsW/ycFwtdxbpnQBhRAk8hYGrWnJLa7AAAAABJRU5ErkJggg==');
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
        
        const enemyPositions = [
            { x: 700, y: 200, floor: 2 },
            { x: 850, y: 200, floor: 2 },
            { x: 750, y: 350, floor: 1 },
            { x: 900, y: 350, floor: 1 }
        ];
        
        enemyPositions.forEach(pos => {
            const enemy = new Enemy(this, pos.x, pos.y);
            this.enemies.push(enemy);
        });
        
        this.crosshair = this.add.image(0, 0, 'crosshair');
        this.crosshair.setScale(0.5);
        this.crosshair.setDepth(100);
        
        this.input.on('pointermove', (pointer: Phaser.Input.Pointer) => {
            this.crosshair.setPosition(pointer.x, pointer.y);
        });
        
        this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {
            if (!this.player.isInCover()) {
                this.player.shoot();
                
                const hitEnemy = this.enemies.find(enemy => {
                    const bounds = enemy.getBounds();
                    return bounds.contains(pointer.x, pointer.y) && enemy.isActive();
                });
                
                if (hitEnemy) {
                    hitEnemy.hit();
                } else {
                    this.createSpark(pointer.x, pointer.y);
                }
            }
        });
        
        this.input.keyboard?.on('keydown-SPACE', () => {
            this.player.toggleCover();
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
    }
}
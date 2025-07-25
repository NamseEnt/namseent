enum GameState {
    SLEEPING = "sleeping",
    BLANKET_REMOVED = "blanket_removed",
    SITTING = "sitting",
    STANDING = "standing",
    WALKING_TO_BATHROOM = "walking_to_bathroom",
    IN_BATHROOM = "in_bathroom",
    WALKING_TO_SHOWER = "walking_to_shower",
    UNDER_SHOWER = "under_shower",
    ADJUSTING_TEMPERATURE = "adjusting_temperature",
    SHOWER_SUCCESS = "shower_success",
    WETTING_BODY = "wetting_body",
    SOAPING = "soaping",
    RINSING = "rinsing",
    DRYING = "drying",
    BLOW_DRYING = "blow_drying",
    GETTING_DRESSED = "getting_dressed",
    LOOKING_AT_MIRROR = "looking_at_mirror",
    LEAVING_HOME = "leaving_home",
    GAME_OVER = "game_over",
}

enum SoapingPart {
    HEAD = "head",
    FACE = "face",
    ARM = "arm",
    ARMPIT = "armpit",
    CHEST = "chest",
    BACK = "back",
    FOOT = "foot",
}

enum CharacterPose {
    LYING = "lying",
    SITTING = "sitting",
    STANDING = "standing",
}

class Game {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private width: number = 800;
    private height: number = 600;
    private lastTime: number = 0;

    // 게임 상태
    private gameState: GameState = GameState.SLEEPING;
    private characterPose: CharacterPose = CharacterPose.LYING;

    // 이불 관련 변수
    private blanketX: number = 0;
    private blanketY: number = 0;
    private isDragging: boolean = false;
    private dragOffsetX: number = 0;
    private dragOffsetY: number = 0;

    // 캐릭터 이동 관련 변수
    private characterX: number = 0;
    private characterY: number = 0;
    private walkingProgress: number = 0;

    // 샤워 온도 관련 변수
    private waterTemperature: number = 0; // 0-100 (차가움-뜨거움)
    private bodyTemperature: number = 50; // 몸 온도
    private temperatureTimer: number = 0; // 적정 온도 유지 시간
    private sliderPosition: number = 0; // 슬라이더 위치 (0-100)

    // 물 끼얹기 관련 변수
    private bodyParts = {
        head: { x: 0, y: 0, radius: 25, wet: false, pulse: 0 },
        leftShoulder: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
        rightShoulder: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
        chest: { x: 0, y: 0, radius: 30, wet: false, pulse: 0 },
        leftArm: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
        rightArm: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
        leftLeg: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
        rightLeg: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
    };
    private waterSplashes: Array<{ x: number; y: number; age: number }> = [];

    // 비누칠 관련 변수
    private soapingGrid: boolean[][] = [];
    private gridSize: number = 20;
    private isRubbing: boolean = false;
    private lastRubPosition: { x: number; y: number } | null = null;
    private soapingProgress: Map<string, number> = new Map();
    private bubbles: Array<{
        x: number;
        y: number;
        size: number;
        age: number;
    }> = [];

    // 비누칠 순서 관련
    private bodyPartOrder: string[] = [
        "머리",
        "가슴",
        "팔",
        "배",
        "등",
        "다리",
        "발",
    ];
    private currentBodyPartIndex: number = 0;

    // 헹구기 관련
    private foamStatus: { [key: string]: number } = {};
    private rinsedParts: Set<string> = new Set();
    private foamFlowTimer: number = 0;

    // 수건으로 닦기 관련
    private wetness: { [key: string]: number } = {};
    private towelDragging: boolean = false;
    private towelX: number = 0;
    private towelY: number = 0;
    private lastTowelX: number = 0;
    private lastTowelY: number = 0;
    private waterFlowTimer: number = 0;
    private waterDroplets: Array<{
        x: number;
        y: number;
        size: number;
        vy: number;
    }> = [];

    // 헤어드라이기 관련
    private dryingOrder: string[] = [];
    private currentDryingIndex: number = 0;
    private moistureLevel: { [key: string]: number } = {};
    private dryerX: number = 0;
    private dryerY: number = 0;
    private dryerOn: boolean = false;
    private dryerAngle: number = 0;
    private windParticles: Array<{
        x: number;
        y: number;
        vx: number;
        vy: number;
        age: number;
    }> = [];
    private heatAccumulation: { [key: string]: number } = {};
    private heatWarning: boolean = false;
    private heatWarningPos: { x: number; y: number } = { x: 0, y: 0 };
    private hairAngles: number[] = [];

    // 옷 입기 관련
    private drawer: any = null;
    private clothes: any[] = [];
    private wornClothes: { [key: string]: boolean } = {};
    private draggingCloth: any = null;

    // 거울 장면 관련
    private mirrorChoices: Array<{
        text: string;
        id: number;
        x: number;
        y: number;
        width: number;
        height: number;
    }> = [
        { text: "잘 갔다오자!", id: 0, x: 0, y: 0, width: 150, height: 40 },
        { text: "오늘도 화이팅!", id: 1, x: 0, y: 0, width: 150, height: 40 },
        { text: "완벽해!", id: 2, x: 0, y: 0, width: 150, height: 40 },
    ];
    private selectedChoice: any = null;
    private showSpeechBubble: boolean = false;
    private speechBubbleTimer: number = 0;

    constructor() {
        this.canvas = document.getElementById(
            "gameCanvas",
        ) as HTMLCanvasElement;
        this.ctx = this.canvas.getContext("2d")!;

        this.setupCanvas();
        this.setupMouseEvents();
        this.init();
    }

    private setupCanvas(): void {
        this.canvas.width = this.width;
        this.canvas.height = this.height;
    }

    private setupMouseEvents(): void {
        this.canvas.addEventListener(
            "mousedown",
            this.handleMouseDown.bind(this),
        );
        this.canvas.addEventListener(
            "mousemove",
            this.handleMouseMove.bind(this),
        );
        this.canvas.addEventListener("mouseup", this.handleMouseUp.bind(this));
        this.canvas.addEventListener(
            "mouseleave",
            this.handleMouseUp.bind(this),
        );
    }

    private handleMouseDown(e: MouseEvent): void {
        const rect = this.canvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        const centerX = this.width / 2;
        const centerY = this.height / 2;

        // 게임 상태가 SLEEPING일 때는 이불 드래그만 처리
        if (this.gameState === GameState.SLEEPING) {
            // 이불 영역 체크 (대략적인 범위)
            const blanketLeft = centerX - 60 + this.blanketX;
            const blanketTop = centerY - 45 + this.blanketY;
            const blanketWidth = 190;
            const blanketHeight = 55;

            if (
                mouseX >= blanketLeft &&
                mouseX <= blanketLeft + blanketWidth &&
                mouseY >= blanketTop &&
                mouseY <= blanketTop + blanketHeight
            ) {
                this.isDragging = true;
                this.dragOffsetX = mouseX - this.blanketX;
                this.dragOffsetY = mouseY - this.blanketY;
            }
        }
        // 이불이 걷어진 상태에서 캐릭터 클릭 체크
        else if (this.gameState === GameState.BLANKET_REMOVED) {
            // 캐릭터 영역 체크 (머리와 몸통 주변)
            const characterX = centerX - 90;
            const characterY = centerY - 20;
            const distance = Math.sqrt(
                Math.pow(mouseX - characterX, 2) +
                    Math.pow(mouseY - characterY, 2),
            );

            if (distance < 50) {
                this.characterPose = CharacterPose.STANDING;
                this.gameState = GameState.STANDING;
                console.log("캐릭터가 완전히 일어났습니다!");
            }
        }
        // 서있는 상태에서 문 클릭 체크
        else if (this.gameState === GameState.STANDING) {
            const doorX = 50;
            const doorY = this.height * 0.7 - 120;
            const doorWidth = 60;
            const doorHeight = 120;

            if (
                mouseX >= doorX &&
                mouseX <= doorX + doorWidth &&
                mouseY >= doorY &&
                mouseY <= doorY + doorHeight
            ) {
                this.gameState = GameState.WALKING_TO_BATHROOM;
                this.walkingProgress = 0;
                console.log("욕실로 걸어갑니다!");
            }
        }
        // 온도 조절 중 버튼 클릭
        else if (this.gameState === GameState.ADJUSTING_TEMPERATURE) {
            const faucetX = this.width / 2;
            const faucetY = 150;

            // 왼쪽 버튼 (차가움)
            if (
                mouseX >= faucetX - 200 &&
                mouseX <= faucetX - 140 &&
                mouseY >= faucetY - 30 &&
                mouseY <= faucetY + 30
            ) {
                // 랜덤한 만큼 이동 (삐걱거림 효과)
                const randomMove = Math.random() * 15 + 5; // 5-20 만큼 이동
                this.sliderPosition = Math.max(
                    0,
                    this.sliderPosition - randomMove,
                );
                console.log(`온도 낮춤: -${randomMove.toFixed(1)}`);
            }

            // 오른쪽 버튼 (뜨거움)
            if (
                mouseX >= faucetX + 140 &&
                mouseX <= faucetX + 200 &&
                mouseY >= faucetY - 30 &&
                mouseY <= faucetY + 30
            ) {
                // 랜덤한 만큼 이동 (삐걱거림 효과)
                const randomMove = Math.random() * 15 + 5; // 5-20 만큼 이동
                this.sliderPosition = Math.min(
                    100,
                    this.sliderPosition + randomMove,
                );
                console.log(`온도 높임: +${randomMove.toFixed(1)}`);
            }
        }
        // 물 끼얹기 단계에서 부위 클릭
        else if (this.gameState === GameState.WETTING_BODY) {
            Object.entries(this.bodyParts).forEach(([name, part]) => {
                const distance = Math.sqrt(
                    Math.pow(mouseX - part.x, 2) + Math.pow(mouseY - part.y, 2),
                );

                if (distance < part.radius && !part.wet) {
                    part.wet = true;
                    console.log(`${name} 부위에 물을 끼얹었습니다!`);

                    // 물 효과 추가
                    for (let i = 0; i < 10; i++) {
                        this.waterSplashes.push({
                            x: part.x + (Math.random() - 0.5) * 40,
                            y: part.y + (Math.random() - 0.5) * 40,
                            age: 0,
                        });
                    }
                }
            });
        }
        // 비누칠 단계에서 드래그
        else if (this.gameState === GameState.SOAPING) {
            this.isRubbing = true;
            this.lastRubPosition = { x: mouseX, y: mouseY };
        }
    }

    private handleMouseUp(): void {
        this.isDragging = false;
        this.isRubbing = false;
        this.lastRubPosition = null;
    }

    private handleMouseMove(e: MouseEvent): void {
        const rect = this.canvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        if (this.isDragging) {
            this.blanketX = mouseX - this.dragOffsetX;
            this.blanketY = mouseY - this.dragOffsetY;
        }

        // 비누칠 중 드래그
        if (this.isRubbing && this.gameState === GameState.SOAPING) {
            if (this.lastRubPosition) {
                // 문지른 영역 업데이트
                this.rubArea(mouseX, mouseY);

                // 거품 효과 추가
                if (Math.random() < 0.3) {
                    this.bubbles.push({
                        x: mouseX + (Math.random() - 0.5) * 20,
                        y: mouseY,
                        size: Math.random() * 10 + 5,
                        age: 0,
                    });
                }
            }
            this.lastRubPosition = { x: mouseX, y: mouseY };
        }
    }

    private init(): void {
        console.log("게임 초기화 완료!");
        this.gameLoop(0);
    }

    private update(deltaTime: number): void {
        // 이불이 충분히 멀리 움직였는지 확인
        if (this.gameState === GameState.SLEEPING) {
            const blanketDistance = Math.sqrt(
                this.blanketX * this.blanketX + this.blanketY * this.blanketY,
            );
            if (blanketDistance > 150) {
                this.gameState = GameState.BLANKET_REMOVED;
                console.log("이불이 걷어졌습니다!");
            }
        }

        // 욕실로 걸어가는 애니메이션
        if (this.gameState === GameState.WALKING_TO_BATHROOM) {
            this.walkingProgress += deltaTime * 0.002;

            // 시작 위치와 목표 위치
            const startX = this.width / 2 - 200;
            const targetX = 80; // 문 중앙

            // 선형 보간으로 이동
            this.characterX =
                startX + (targetX - startX) * Math.min(this.walkingProgress, 1);

            // 도착했으면 욕실로 상태 변경
            if (this.walkingProgress >= 1) {
                this.gameState = GameState.IN_BATHROOM;
                this.walkingProgress = 0;
                console.log("욕실에 도착했습니다!");

                // 1초 후 자동으로 샤워기로 이동 시작
                setTimeout(() => {
                    if (this.gameState === GameState.IN_BATHROOM) {
                        this.gameState = GameState.WALKING_TO_SHOWER;
                        console.log("샤워기로 이동합니다!");
                    }
                }, 1000);
            }
        }

        // 샤워기로 걸어가는 애니메이션
        if (this.gameState === GameState.WALKING_TO_SHOWER) {
            this.walkingProgress += deltaTime * 0.001;

            if (this.walkingProgress >= 1) {
                this.gameState = GameState.UNDER_SHOWER;
                console.log("샤워기 아래 도착!");

                // 0.5초 후 온도 조절 화면으로
                setTimeout(() => {
                    if (this.gameState === GameState.UNDER_SHOWER) {
                        this.gameState = GameState.ADJUSTING_TEMPERATURE;
                        console.log("온도를 조절하세요!");
                    }
                }, 500);
            }
        }

        // 온도 조절 중
        if (this.gameState === GameState.ADJUSTING_TEMPERATURE) {
            // 물 온도가 슬라이더 위치로 천천히 이동
            const tempDiff = this.sliderPosition - this.waterTemperature;
            this.waterTemperature += tempDiff * 0.05;

            // 몸 온도 변화
            if (this.waterTemperature < 20) {
                this.bodyTemperature -= deltaTime * 0.01; // 차가우면 몸 온도 하락
            } else if (this.waterTemperature > 80) {
                this.bodyTemperature += deltaTime * 0.015; // 뜨거우면 몸 온도 상승
            } else if (
                this.waterTemperature >= 35 &&
                this.waterTemperature <= 45
            ) {
                // 적정 온도 범위에서 몸 온도 회복
                if (this.bodyTemperature < 50) {
                    this.bodyTemperature += deltaTime * 0.005;
                } else if (this.bodyTemperature > 50) {
                    this.bodyTemperature -= deltaTime * 0.005;
                }

                // 적정 온도 유지 시간 증가
                this.temperatureTimer += deltaTime;

                if (this.temperatureTimer > 3000) {
                    // 3초 유지시 성공
                    this.gameState = GameState.SHOWER_SUCCESS;
                    console.log("온도 조절 성공!");

                    // 1초 후 물 끼얹기 단계로
                    setTimeout(() => {
                        if (this.gameState === GameState.SHOWER_SUCCESS) {
                            this.gameState = GameState.WETTING_BODY;
                            this.updateBodyPartPositions();
                            console.log("몸에 물을 끼얹으세요!");
                        }
                    }, 1000);
                }
            } else {
                this.temperatureTimer = 0; // 적정 온도 벗어나면 타이머 리셋
            }

            // 게임 오버 체크
            if (this.bodyTemperature <= 0 || this.bodyTemperature >= 100) {
                this.gameState = GameState.GAME_OVER;
                console.log("게임 오버!");
            }
        }

        // 물 끼얹기 단계
        if (this.gameState === GameState.WETTING_BODY) {
            // 반짝임 효과 애니메이션
            Object.values(this.bodyParts).forEach((part) => {
                if (!part.wet) {
                    part.pulse += deltaTime * 0.003;
                }
            });

            // 물 효과 애니메이션
            this.waterSplashes = this.waterSplashes.filter((splash) => {
                splash.age += deltaTime;
                return splash.age < 500;
            });

            // 모든 부위가 젖었는지 확인
            const allWet = Object.values(this.bodyParts).every(
                (part) => part.wet,
            );
            if (allWet && this.gameState === GameState.WETTING_BODY) {
                console.log("모든 부위를 적셨습니다!");
                // 비누칠 단계로
                setTimeout(() => {
                    this.gameState = GameState.SOAPING;
                    this.initSoapingGrid();
                    console.log("비누칠 단계 시작!");
                }, 1000);
            }
        }

        // 비누칠 단계
        if (this.gameState === GameState.SOAPING) {
            // 거품 애니메이션
            this.bubbles = this.bubbles.filter((bubble) => {
                bubble.age += deltaTime;
                bubble.y -= deltaTime * 0.05;
                bubble.size = bubble.size * (1 - bubble.age / 3000);
                return bubble.age < 3000 && bubble.size > 0.5;
            });

            // 진행도 체크
            const progress = this.calculateSoapingProgress();
            const currentPart = this.bodyPartOrder[this.currentBodyPartIndex];
            this.soapingProgress.set(currentPart, progress);

            if (progress >= 100) {
                console.log(`${currentPart} 완료!`);
                this.nextSoapingPart();
            }
        }
    }

    private updateBodyPartPositions(): void {
        const centerX = this.width / 2;
        const baseY = 150;

        // 캐릭터 부위 위치 설정
        this.bodyParts.head.x = centerX;
        this.bodyParts.head.y = baseY;

        this.bodyParts.leftShoulder.x = centerX - 40;
        this.bodyParts.leftShoulder.y = baseY + 50;

        this.bodyParts.rightShoulder.x = centerX + 40;
        this.bodyParts.rightShoulder.y = baseY + 50;

        this.bodyParts.chest.x = centerX;
        this.bodyParts.chest.y = baseY + 80;

        this.bodyParts.leftArm.x = centerX - 60;
        this.bodyParts.leftArm.y = baseY + 100;

        this.bodyParts.rightArm.x = centerX + 60;
        this.bodyParts.rightArm.y = baseY + 100;

        this.bodyParts.leftLeg.x = centerX - 20;
        this.bodyParts.leftLeg.y = baseY + 150;

        this.bodyParts.rightLeg.x = centerX + 20;
        this.bodyParts.rightLeg.y = baseY + 150;
    }

    private initSoapingGrid(): void {
        this.soapingGrid = [];
        for (let i = 0; i < this.gridSize; i++) {
            this.soapingGrid[i] = [];
            for (let j = 0; j < this.gridSize; j++) {
                this.soapingGrid[i][j] = false;
            }
        }
        this.bubbles = [];
        this.currentBodyPartIndex = 0;
        console.log(
            "비누칠 초기화 완료. 첫 부위:",
            this.bodyPartOrder[this.currentBodyPartIndex],
        );
    }

    private rubArea(x: number, y: number): void {
        const partArea = this.getSoapingPartArea();
        const gridX = Math.floor(
            ((x - partArea.x) / partArea.width) * this.gridSize,
        );
        const gridY = Math.floor(
            ((y - partArea.y) / partArea.height) * this.gridSize,
        );

        // 주변 영역도 함께 문지르기
        const radius = 2;
        for (let i = -radius; i <= radius; i++) {
            for (let j = -radius; j <= radius; j++) {
                const gx = gridX + i;
                const gy = gridY + j;
                if (
                    gx >= 0 &&
                    gx < this.gridSize &&
                    gy >= 0 &&
                    gy < this.gridSize
                ) {
                    this.soapingGrid[gx][gy] = true;
                }
            }
        }
    }

    private calculateSoapingProgress(): number {
        let total = 0;
        let rubbed = 0;

        for (let i = 0; i < this.gridSize; i++) {
            for (let j = 0; j < this.gridSize; j++) {
                total++;
                if (this.soapingGrid[i][j]) rubbed++;
            }
        }

        return (rubbed / total) * 100;
    }

    private getSoapingPartArea(): {
        x: number;
        y: number;
        width: number;
        height: number;
    } {
        const centerX = this.width / 2;
        const centerY = this.height / 2;
        const currentPart = this.bodyPartOrder[this.currentBodyPartIndex];

        switch (currentPart) {
            case "머리":
                return {
                    x: centerX - 50,
                    y: centerY - 50,
                    width: 100,
                    height: 100,
                };
            case "가슴":
                return {
                    x: centerX - 60,
                    y: centerY - 40,
                    width: 120,
                    height: 80,
                };
            case "팔":
                return {
                    x: centerX - 150,
                    y: centerY - 20,
                    width: 300,
                    height: 100,
                };
            case "배":
                return { x: centerX - 50, y: centerY, width: 100, height: 80 };
            case "등":
                return {
                    x: centerX - 50,
                    y: centerY - 60,
                    width: 100,
                    height: 120,
                };
            case "다리":
                return {
                    x: centerX - 60,
                    y: centerY + 40,
                    width: 120,
                    height: 100,
                };
            case "발":
                return {
                    x: centerX - 50,
                    y: centerY + 100,
                    width: 100,
                    height: 60,
                };
            default:
                return { x: 0, y: 0, width: 0, height: 0 };
        }
    }

    private nextSoapingPart(): void {
        this.currentBodyPartIndex++;

        if (this.currentBodyPartIndex < this.bodyPartOrder.length) {
            // 그리드만 초기화 (bodyPartOrder와 currentBodyPartIndex는 유지)
            for (let i = 0; i < this.gridSize; i++) {
                for (let j = 0; j < this.gridSize; j++) {
                    this.soapingGrid[i][j] = false;
                }
            }
            this.bubbles = [];
            console.log(
                `다음 부위: ${this.bodyPartOrder[this.currentBodyPartIndex]}`,
            );
        } else {
            console.log("모든 부위 비누칠 완료!");
            this.gameState = GameState.RINSING;
            this.initRinsing();
        }
    }

    private drawRoom(): void {
        // 문 그리기
        const doorX = 50;
        const doorY = this.height * 0.7 - 120;
        const doorWidth = 60;
        const doorHeight = 120;

        // 문 프레임
        this.ctx.strokeStyle = "#654321";
        this.ctx.lineWidth = 4;
        this.ctx.strokeRect(doorX, doorY, doorWidth, doorHeight);

        // 문
        this.ctx.fillStyle = "#8B4513";
        this.ctx.fillRect(doorX + 2, doorY + 2, doorWidth - 4, doorHeight - 4);

        // 문 손잡이
        this.ctx.fillStyle = "#FFD700";
        this.ctx.beginPath();
        this.ctx.arc(
            doorX + doorWidth - 15,
            doorY + doorHeight / 2,
            5,
            0,
            Math.PI * 2,
        );
        this.ctx.fill();

        // STANDING 상태일 때 문에 하이라이트
        if (this.gameState === GameState.STANDING) {
            this.ctx.strokeStyle = "#FFFF00";
            this.ctx.lineWidth = 2;
            this.ctx.strokeRect(
                doorX - 2,
                doorY - 2,
                doorWidth + 4,
                doorHeight + 4,
            );
        }
    }

    private drawBed(): void {
        const centerX = this.width / 2;
        const centerY = this.height / 2;

        // 침대 프레임
        this.ctx.fillStyle = "#8B4513";
        this.ctx.fillRect(centerX - 150, centerY, 300, 50);

        // 매트리스
        this.ctx.fillStyle = "#F5F5DC";
        this.ctx.fillRect(centerX - 140, centerY - 15, 280, 15);

        // 베개
        this.ctx.fillStyle = "#FFFFFF";
        this.ctx.fillRect(centerX - 120, centerY - 25, 60, 10);

        // 침대 다리
        this.ctx.fillStyle = "#654321";
        this.ctx.fillRect(centerX - 130, centerY + 50, 20, 40);
        this.ctx.fillRect(centerX + 110, centerY + 50, 20, 40);
    }

    private drawCharacter(): void {
        const centerX = this.width / 2;
        const centerY = this.height / 2;

        if (this.characterPose === CharacterPose.LYING) {
            // 누워있는 자세 (기존 코드)
            // 머리 (fill)
            this.ctx.fillStyle = "#000000";
            this.ctx.beginPath();
            this.ctx.arc(centerX - 90, centerY - 20, 12, 0, Math.PI * 2);
            this.ctx.fill();

            // 선 스타일 설정
            this.ctx.strokeStyle = "#000000";
            this.ctx.lineWidth = 8;
            this.ctx.lineCap = "round";

            // 몸통
            this.ctx.beginPath();
            this.ctx.moveTo(centerX - 78, centerY - 20);
            this.ctx.lineTo(centerX + 80, centerY - 20);
            this.ctx.stroke();

            // 왼쪽 팔
            this.ctx.beginPath();
            this.ctx.moveTo(centerX - 50, centerY - 20);
            this.ctx.lineTo(centerX - 50, centerY - 40);
            this.ctx.stroke();

            // 오른쪽 팔
            this.ctx.beginPath();
            this.ctx.moveTo(centerX + 20, centerY - 20);
            this.ctx.lineTo(centerX + 20, centerY - 40);
            this.ctx.stroke();

            // 왼쪽 다리
            this.ctx.beginPath();
            this.ctx.moveTo(centerX + 80, centerY - 20);
            this.ctx.lineTo(centerX + 120, centerY - 35);
            this.ctx.stroke();

            // 오른쪽 다리
            this.ctx.beginPath();
            this.ctx.moveTo(centerX + 80, centerY - 20);
            this.ctx.lineTo(centerX + 120, centerY - 5);
            this.ctx.stroke();
        } else if (this.characterPose === CharacterPose.SITTING) {
            // 앉아있는 자세
            // 머리
            this.ctx.fillStyle = "#000000";
            this.ctx.beginPath();
            this.ctx.arc(centerX, centerY - 50, 12, 0, Math.PI * 2);
            this.ctx.fill();

            // 선 스타일 설정
            this.ctx.strokeStyle = "#000000";
            this.ctx.lineWidth = 8;
            this.ctx.lineCap = "round";

            // 몸통 (수직)
            this.ctx.beginPath();
            this.ctx.moveTo(centerX, centerY - 38);
            this.ctx.lineTo(centerX, centerY - 10);
            this.ctx.stroke();

            // 왼쪽 팔
            this.ctx.beginPath();
            this.ctx.moveTo(centerX, centerY - 30);
            this.ctx.lineTo(centerX - 20, centerY - 15);
            this.ctx.stroke();

            // 오른쪽 팔
            this.ctx.beginPath();
            this.ctx.moveTo(centerX, centerY - 30);
            this.ctx.lineTo(centerX + 20, centerY - 15);
            this.ctx.stroke();

            // 왼쪽 다리 (앉은 자세)
            this.ctx.beginPath();
            this.ctx.moveTo(centerX, centerY - 10);
            this.ctx.lineTo(centerX - 15, centerY - 10);
            this.ctx.lineTo(centerX - 15, centerY + 10);
            this.ctx.stroke();

            // 오른쪽 다리 (앉은 자세)
            this.ctx.beginPath();
            this.ctx.moveTo(centerX, centerY - 10);
            this.ctx.lineTo(centerX + 15, centerY - 10);
            this.ctx.lineTo(centerX + 15, centerY + 10);
            this.ctx.stroke();
        } else if (this.characterPose === CharacterPose.STANDING) {
            // 서있는 자세 (침대 옆 바닥에)
            let standingX = centerX - 200; // 침대 왼쪽

            // 이동 중이면 characterX 사용
            if (this.gameState === GameState.WALKING_TO_BATHROOM) {
                standingX = this.characterX;
            }

            const floorY = this.height * 0.7; // 바닥 위치

            // 머리
            this.ctx.fillStyle = "#000000";
            this.ctx.beginPath();
            this.ctx.arc(standingX, floorY - 150, 20, 0, Math.PI * 2);
            this.ctx.fill();

            // 선 스타일 설정
            this.ctx.strokeStyle = "#000000";
            this.ctx.lineWidth = 10;
            this.ctx.lineCap = "round";

            // 몸통 (수직)
            this.ctx.beginPath();
            this.ctx.moveTo(standingX, floorY - 130);
            this.ctx.lineTo(standingX, floorY - 60);
            this.ctx.stroke();

            // 왼쪽 팔
            this.ctx.beginPath();
            this.ctx.moveTo(standingX, floorY - 100);
            this.ctx.lineTo(standingX - 30, floorY - 70);
            this.ctx.stroke();

            // 오른쪽 팔
            this.ctx.beginPath();
            this.ctx.moveTo(standingX, floorY - 100);
            this.ctx.lineTo(standingX + 30, floorY - 70);
            this.ctx.stroke();

            // 왼쪽 다리
            this.ctx.beginPath();
            this.ctx.moveTo(standingX, floorY - 60);
            this.ctx.lineTo(standingX - 15, floorY);
            this.ctx.stroke();

            // 오른쪽 다리
            this.ctx.beginPath();
            this.ctx.moveTo(standingX, floorY - 60);
            this.ctx.lineTo(standingX + 15, floorY);
            this.ctx.stroke();
        }
    }

    private drawBlanket(): void {
        const centerX = this.width / 2;
        const centerY = this.height / 2;

        this.ctx.save();
        this.ctx.translate(this.blanketX, this.blanketY);

        this.ctx.fillStyle = "#4682B4";

        // D 모양 (왼쪽으로 90도 회전 - 둥근 부분이 위쪽)
        this.ctx.beginPath();
        this.ctx.moveTo(centerX - 60, centerY + 10);
        this.ctx.quadraticCurveTo(
            centerX - 60,
            centerY - 45,
            centerX,
            centerY - 45,
        );
        this.ctx.quadraticCurveTo(
            centerX + 60,
            centerY - 45,
            centerX + 100,
            centerY - 35,
        );
        this.ctx.quadraticCurveTo(
            centerX + 130,
            centerY - 20,
            centerX + 130,
            centerY + 10,
        );
        this.ctx.lineTo(centerX - 60, centerY + 10);
        this.ctx.closePath();
        this.ctx.fill();

        // 드래그 중일 때 커서 변경을 위한 스타일
        if (this.isDragging) {
            this.canvas.style.cursor = "grabbing";
        } else {
            this.canvas.style.cursor = "default";
        }

        this.ctx.restore();
    }

    private drawTemperatureControl(): void {
        // 배경
        this.ctx.fillStyle = "#F0F0F0";
        this.ctx.fillRect(0, 0, this.width, this.height);

        // 제목
        this.ctx.fillStyle = "#000000";
        this.ctx.font = "30px Arial";
        this.ctx.textAlign = "center";
        this.ctx.fillText("온도를 조절하세요!", this.width / 2, 50);

        // 수도꼭지 그리기
        const faucetX = this.width / 2;
        const faucetY = 150;

        // 수도꼭지 몸체
        this.ctx.fillStyle = "#C0C0C0";
        this.ctx.fillRect(faucetX - 100, faucetY - 20, 200, 40);

        // 온도 조절 손잡이
        const handleX = faucetX - 100 + (this.sliderPosition / 100) * 200;
        this.ctx.fillStyle = "#808080";
        this.ctx.beginPath();
        this.ctx.arc(handleX, faucetY, 25, 0, Math.PI * 2);
        this.ctx.fill();

        // 왼쪽 버튼 (차가움)
        this.ctx.fillStyle = "#4169E1";
        this.ctx.fillRect(faucetX - 200, faucetY - 30, 60, 60);
        this.ctx.fillStyle = "#FFFFFF";
        this.ctx.font = "40px Arial";
        this.ctx.fillText("◀", faucetX - 170, faucetY + 10);

        // 오른쪽 버튼 (뜨거움)
        this.ctx.fillStyle = "#DC143C";
        this.ctx.fillRect(faucetX + 140, faucetY - 30, 60, 60);
        this.ctx.fillStyle = "#FFFFFF";
        this.ctx.fillText("▶", faucetX + 170, faucetY + 10);

        // 물 온도 표시
        const waterColor = this.getTemperatureColor(this.waterTemperature);
        this.ctx.fillStyle = waterColor;
        this.ctx.fillRect(faucetX - 50, faucetY + 50, 100, 150);

        // 물줄기 애니메이션
        for (let i = 0; i < 5; i++) {
            this.ctx.fillStyle = waterColor;
            this.ctx.globalAlpha = 0.6;
            this.ctx.fillRect(
                faucetX - 30 + Math.random() * 60,
                faucetY + 50 + i * 30,
                5,
                20,
            );
        }
        this.ctx.globalAlpha = 1;

        // 몸 온도 게이지
        this.ctx.fillStyle = "#000000";
        this.ctx.font = "20px Arial";
        this.ctx.fillText("몸 온도", 100, 300);

        // 게이지 배경
        this.ctx.fillStyle = "#CCCCCC";
        this.ctx.fillRect(50, 320, 200, 30);

        // 게이지 채우기
        const bodyTempColor =
            this.bodyTemperature < 30
                ? "#4169E1"
                : this.bodyTemperature > 70
                ? "#DC143C"
                : "#32CD32";
        this.ctx.fillStyle = bodyTempColor;
        this.ctx.fillRect(50, 320, (this.bodyTemperature / 100) * 200, 30);

        // 안전 구간 표시
        this.ctx.strokeStyle = "#00FF00";
        this.ctx.lineWidth = 2;
        this.ctx.strokeRect(50 + 30 * 2, 320, 40 * 2, 30);

        // 타이머 표시 (적정 온도일 때)
        if (this.waterTemperature >= 35 && this.waterTemperature <= 45) {
            this.ctx.fillStyle = "#00FF00";
            this.ctx.font = "25px Arial";
            this.ctx.fillText(
                `유지: ${(this.temperatureTimer / 1000).toFixed(1)}초 / 3초`,
                this.width / 2,
                400,
            );
        }

        // 온도 수치 표시
        this.ctx.fillStyle = "#000000";
        this.ctx.font = "18px Arial";
        this.ctx.fillText(
            `물 온도: ${Math.round(this.waterTemperature)}°C`,
            this.width / 2,
            450,
        );
    }

    private getTemperatureColor(temp: number): string {
        if (temp < 20) return "#00BFFF"; // 차가움
        if (temp < 35) return "#87CEEB"; // 시원함
        if (temp < 45) return "#90EE90"; // 적정
        if (temp < 60) return "#FFA500"; // 따뜻함
        return "#FF4500"; // 뜨거움
    }

    private drawBathroom(): void {
        // 욕실 배경
        this.ctx.fillStyle = "#E6F3FF";
        this.ctx.fillRect(0, 0, this.width, this.height);

        // 욕실 바닥 타일
        this.ctx.fillStyle = "#B0C4DE";
        this.ctx.fillRect(0, this.height * 0.8, this.width, this.height * 0.2);

        // 샤워 부스
        const showerX = this.width / 2;
        const showerY = 100;

        // 샤워기 파이프
        this.ctx.strokeStyle = "#C0C0C0";
        this.ctx.lineWidth = 8;
        this.ctx.beginPath();
        this.ctx.moveTo(showerX, 50);
        this.ctx.lineTo(showerX, showerY);
        this.ctx.stroke();

        // 샤워 헤드
        this.ctx.fillStyle = "#C0C0C0";
        this.ctx.beginPath();
        this.ctx.arc(showerX, showerY, 30, 0, Math.PI, false);
        this.ctx.fill();

        // 샤워 헤드 구멍들
        this.ctx.fillStyle = "#808080";
        for (let i = -2; i <= 2; i++) {
            for (let j = 0; j < 2; j++) {
                this.ctx.beginPath();
                this.ctx.arc(
                    showerX + i * 10,
                    showerY + j * 10 - 5,
                    2,
                    0,
                    Math.PI * 2,
                );
                this.ctx.fill();
            }
        }

        // 샤워 부스 유리문 (옆면)
        this.ctx.strokeStyle = "#87CEEB";
        this.ctx.lineWidth = 3;
        this.ctx.strokeRect(showerX - 100, 150, 200, 300);

        // 욕실에서 캐릭터 그리기
        if (this.gameState === GameState.IN_BATHROOM) {
            this.drawBathroomCharacter(100, this.height * 0.8);
        } else if (this.gameState === GameState.WALKING_TO_SHOWER) {
            const x = 100 + (showerX - 100) * this.walkingProgress;
            this.drawBathroomCharacter(x, this.height * 0.8);
        } else if (this.gameState === GameState.UNDER_SHOWER) {
            this.drawBathroomCharacter(showerX, this.height * 0.8);
        }
    }

    private drawSoaping(): void {
        // 배경
        this.ctx.fillStyle = "#F0F8FF";
        this.ctx.fillRect(0, 0, this.width, this.height);

        // 제목
        this.ctx.fillStyle = "#000000";
        this.ctx.font = "25px Arial";
        this.ctx.textAlign = "center";
        this.ctx.fillText(
            `${this.currentBodyPartIndex + 1}/${this.bodyPartOrder.length} - ${
                this.bodyPartOrder[this.currentBodyPartIndex]
            } 비누칠하기`,
            this.width / 2,
            40,
        );

        // 진행률 표시
        const progress = this.calculateSoapingProgress();
        this.ctx.fillStyle = "#CCCCCC";
        this.ctx.fillRect(this.width / 2 - 150, 60, 300, 20);
        this.ctx.fillStyle = "#4169E1";
        this.ctx.fillRect(this.width / 2 - 150, 60, 300 * (progress / 100), 20);
        this.ctx.fillStyle = "#000000";
        this.ctx.font = "15px Arial";
        this.ctx.fillText(`${Math.round(progress)}%`, this.width / 2, 100);

        // 현재 부위 그리기
        const currentPart = this.bodyPartOrder[this.currentBodyPartIndex];
        const centerX = this.width / 2;
        const centerY = this.height / 2;

        // 그리드 기반 비누칠 상태 표시
        const gridCellSize = 10;
        const startX = centerX - (this.gridSize * gridCellSize) / 2;
        const startY = centerY - (this.gridSize * gridCellSize) / 2;

        // 부위별 윤곽선 그리기
        this.ctx.strokeStyle = "#000000";
        this.ctx.lineWidth = 3;

        switch (currentPart) {
            case "머리":
                this.ctx.beginPath();
                this.ctx.arc(centerX, centerY, 50, 0, Math.PI * 2);
                this.ctx.stroke();
                break;
            case "얼굴":
                this.ctx.beginPath();
                this.ctx.arc(centerX, centerY, 40, 0, Math.PI * 2);
                this.ctx.stroke();
                // 눈
                this.ctx.beginPath();
                this.ctx.arc(centerX - 15, centerY - 10, 5, 0, Math.PI * 2);
                this.ctx.arc(centerX + 15, centerY - 10, 5, 0, Math.PI * 2);
                this.ctx.stroke();
                // 입
                this.ctx.beginPath();
                this.ctx.arc(centerX, centerY + 10, 15, 0, Math.PI);
                this.ctx.stroke();
                break;
            case "팔":
                this.ctx.beginPath();
                this.ctx.moveTo(centerX - 60, centerY - 20);
                this.ctx.lineTo(centerX - 20, centerY - 20);
                this.ctx.lineTo(centerX - 20, centerY + 80);
                this.ctx.lineTo(centerX - 60, centerY + 80);
                this.ctx.closePath();
                this.ctx.stroke();
                break;
            case "겨드랑이":
                this.ctx.beginPath();
                this.ctx.ellipse(centerX, centerY, 30, 50, 0, 0, Math.PI * 2);
                this.ctx.stroke();
                break;
            case "가슴":
                this.ctx.strokeRect(centerX - 60, centerY - 40, 120, 80);
                break;
            case "배":
                this.ctx.beginPath();
                this.ctx.ellipse(centerX, centerY, 50, 60, 0, 0, Math.PI * 2);
                this.ctx.stroke();
                break;
            case "등":
                this.ctx.strokeRect(centerX - 50, centerY - 60, 100, 120);
                break;
            case "발":
                this.ctx.beginPath();
                this.ctx.ellipse(centerX, centerY, 40, 60, 0, 0, Math.PI * 2);
                this.ctx.stroke();
                // 발가락
                for (let i = 0; i < 5; i++) {
                    this.ctx.beginPath();
                    this.ctx.arc(
                        centerX - 20 + i * 10,
                        centerY - 50,
                        5,
                        0,
                        Math.PI * 2,
                    );
                    this.ctx.stroke();
                }
                break;
        }

        // 그리드 표시
        for (let i = 0; i < this.gridSize; i++) {
            for (let j = 0; j < this.gridSize; j++) {
                const x = startX + i * gridCellSize;
                const y = startY + j * gridCellSize;

                if (this.soapingGrid[i][j]) {
                    // 비누칠된 부분
                    this.ctx.fillStyle = "rgba(255, 255, 255, 0.8)";
                    this.ctx.fillRect(x, y, gridCellSize - 1, gridCellSize - 1);
                } else {
                    // 비누칠 안된 부분
                    this.ctx.fillStyle = "rgba(200, 200, 200, 0.3)";
                    this.ctx.fillRect(x, y, gridCellSize - 1, gridCellSize - 1);
                }
            }
        }

        // 거품 효과
        this.ctx.fillStyle = "rgba(255, 255, 255, 0.7)";
        this.bubbles.forEach((bubble) => {
            this.ctx.beginPath();
            this.ctx.arc(bubble.x, bubble.y, bubble.size, 0, Math.PI * 2);
            this.ctx.fill();
        });

        // 안내 메시지
        this.ctx.fillStyle = "#000000";
        this.ctx.font = "18px Arial";
        this.ctx.fillText(
            "마우스를 드래그해서 비누칠하세요!",
            this.width / 2,
            this.height - 50,
        );
    }

    private drawWettingBody(): void {
        // 배경
        this.ctx.fillStyle = "#E6F3FF";
        this.ctx.fillRect(0, 0, this.width, this.height);

        // 제목
        this.ctx.fillStyle = "#000000";
        this.ctx.font = "25px Arial";
        this.ctx.textAlign = "center";
        this.ctx.fillText("몸에 물을 끼얹으세요!", this.width / 2, 40);

        // 큰 스틱맨 그리기
        const centerX = this.width / 2;
        const baseY = 150;

        // 머리
        this.ctx.fillStyle = this.bodyParts.head.wet ? "#87CEEB" : "#000000";
        this.ctx.beginPath();
        this.ctx.arc(centerX, baseY, 30, 0, Math.PI * 2);
        this.ctx.fill();

        this.ctx.strokeStyle = "#000000";
        this.ctx.lineWidth = 12;
        this.ctx.lineCap = "round";

        // 몸통
        this.ctx.beginPath();
        this.ctx.moveTo(centerX, baseY + 30);
        this.ctx.lineTo(centerX, baseY + 100);
        this.ctx.stroke();

        // 팔
        this.ctx.beginPath();
        this.ctx.moveTo(centerX, baseY + 50);
        this.ctx.lineTo(centerX - 60, baseY + 100);
        this.ctx.moveTo(centerX, baseY + 50);
        this.ctx.lineTo(centerX + 60, baseY + 100);
        this.ctx.stroke();

        // 다리
        this.ctx.beginPath();
        this.ctx.moveTo(centerX, baseY + 100);
        this.ctx.lineTo(centerX - 30, baseY + 170);
        this.ctx.moveTo(centerX, baseY + 100);
        this.ctx.lineTo(centerX + 30, baseY + 170);
        this.ctx.stroke();

        // 클릭 가능한 부위 표시
        Object.values(this.bodyParts).forEach((part) => {
            if (!part.wet) {
                // 반짝임 효과
                const opacity = 0.3 + Math.sin(part.pulse) * 0.2;

                // 점선 원
                this.ctx.strokeStyle = `rgba(0, 150, 255, ${opacity})`;
                this.ctx.lineWidth = 2;
                this.ctx.setLineDash([5, 5]);
                this.ctx.beginPath();
                this.ctx.arc(part.x, part.y, part.radius, 0, Math.PI * 2);
                this.ctx.stroke();
                this.ctx.setLineDash([]);

                // 반짝임 효과
                if (Math.sin(part.pulse) > 0.8) {
                    this.ctx.fillStyle = `rgba(0, 150, 255, 0.2)`;
                    this.ctx.beginPath();
                    this.ctx.arc(
                        part.x,
                        part.y,
                        part.radius * 0.8,
                        0,
                        Math.PI * 2,
                    );
                    this.ctx.fill();
                }
            } else {
                // 젖은 부위 표시
                this.ctx.fillStyle = "rgba(0, 150, 255, 0.3)";
                this.ctx.beginPath();
                this.ctx.arc(part.x, part.y, part.radius, 0, Math.PI * 2);
                this.ctx.fill();
            }
        });

        // 물 효과
        this.ctx.fillStyle = "rgba(0, 150, 255, 0.6)";
        this.waterSplashes.forEach((splash) => {
            const alpha = 1 - splash.age / 500;
            this.ctx.globalAlpha = alpha * 0.6;
            this.ctx.beginPath();
            this.ctx.arc(
                splash.x,
                splash.y + splash.age * 0.2,
                3,
                0,
                Math.PI * 2,
            );
            this.ctx.fill();
        });
        this.ctx.globalAlpha = 1;

        // 진행 상황
        const wetCount = Object.values(this.bodyParts).filter(
            (part) => part.wet,
        ).length;
        const totalCount = Object.values(this.bodyParts).length;

        this.ctx.fillStyle = "#000000";
        this.ctx.font = "20px Arial";
        this.ctx.fillText(
            `진행률: ${wetCount}/${totalCount}`,
            this.width / 2,
            this.height - 50,
        );
    }

    private drawBathroomCharacter(x: number, floorY: number): void {
        // 머리
        this.ctx.fillStyle = "#000000";
        this.ctx.beginPath();
        this.ctx.arc(x, floorY - 150, 20, 0, Math.PI * 2);
        this.ctx.fill();

        // 선 스타일 설정
        this.ctx.strokeStyle = "#000000";
        this.ctx.lineWidth = 10;
        this.ctx.lineCap = "round";

        // 몸통
        this.ctx.beginPath();
        this.ctx.moveTo(x, floorY - 130);
        this.ctx.lineTo(x, floorY - 60);
        this.ctx.stroke();

        // 팔
        this.ctx.beginPath();
        this.ctx.moveTo(x, floorY - 100);
        this.ctx.lineTo(x - 30, floorY - 70);
        this.ctx.moveTo(x, floorY - 100);
        this.ctx.lineTo(x + 30, floorY - 70);
        this.ctx.stroke();

        // 다리
        this.ctx.beginPath();
        this.ctx.moveTo(x, floorY - 60);
        this.ctx.lineTo(x - 15, floorY);
        this.ctx.moveTo(x, floorY - 60);
        this.ctx.lineTo(x + 15, floorY);
        this.ctx.stroke();
    }

    private render(): void {
        this.ctx.fillStyle = "#2C3E50";
        this.ctx.fillRect(0, 0, this.width, this.height);

        this.ctx.fillStyle = "#34495E";
        this.ctx.fillRect(0, this.height * 0.7, this.width, this.height * 0.3);

        // 욕실에 들어간 상태
        if (
            this.gameState === GameState.IN_BATHROOM ||
            this.gameState === GameState.WALKING_TO_SHOWER ||
            this.gameState === GameState.UNDER_SHOWER
        ) {
            this.drawBathroom();
        }
        // 온도 조절 화면
        else if (this.gameState === GameState.ADJUSTING_TEMPERATURE) {
            this.drawTemperatureControl();
        }
        // 게임 오버 화면
        else if (this.gameState === GameState.GAME_OVER) {
            this.ctx.fillStyle = "#000000";
            this.ctx.fillRect(0, 0, this.width, this.height);
            this.ctx.fillStyle = "#FFFFFF";
            this.ctx.font = "40px Arial";
            this.ctx.textAlign = "center";
            this.ctx.fillText("게임 오버!", this.width / 2, this.height / 2);
            this.ctx.font = "20px Arial";
            this.ctx.fillText(
                this.bodyTemperature <= 0 ? "너무 차가워!" : "너무 뜨거워!",
                this.width / 2,
                this.height / 2 + 50,
            );
        }
        // 성공 화면
        else if (this.gameState === GameState.SHOWER_SUCCESS) {
            this.ctx.fillStyle = "#87CEEB";
            this.ctx.fillRect(0, 0, this.width, this.height);
            this.ctx.fillStyle = "#000000";
            this.ctx.font = "40px Arial";
            this.ctx.textAlign = "center";
            this.ctx.fillText("성공!", this.width / 2, this.height / 2);
            this.ctx.font = "20px Arial";
            this.ctx.fillText(
                "완벽한 온도입니다!",
                this.width / 2,
                this.height / 2 + 50,
            );
        }
        // 물 끼얹기 화면
        else if (this.gameState === GameState.WETTING_BODY) {
            this.drawWettingBody();
        }
        // 비누칠 화면
        else if (this.gameState === GameState.SOAPING) {
            this.drawSoaping();
        }
        // 침실 상태들
        else {
            this.drawRoom();
            this.drawBed();
            this.drawCharacter();

            // 이불은 SLEEPING 상태일 때만 그리기
            if (this.gameState === GameState.SLEEPING) {
                this.drawBlanket();
            }
        }
    }

    private gameLoop = (currentTime: number): void => {
        const deltaTime = currentTime - this.lastTime;
        this.lastTime = currentTime;

        this.update(deltaTime);
        this.render();

        requestAnimationFrame(this.gameLoop);
    };
}

// 게임 시작
window.addEventListener("DOMContentLoaded", () => {
    new Game();
});

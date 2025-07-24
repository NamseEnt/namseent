// src/index.ts
var SoapingPart;
((SoapingPart2) => {
  SoapingPart2["HEAD"] = "head";
  SoapingPart2["FACE"] = "face";
  SoapingPart2["ARM"] = "arm";
  SoapingPart2["ARMPIT"] = "armpit";
  SoapingPart2["CHEST"] = "chest";
  SoapingPart2["BACK"] = "back";
  SoapingPart2["FOOT"] = "foot";
})(SoapingPart ||= {});
class Game {
  canvas;
  ctx;
  width = 800;
  height = 600;
  lastTime = 0;
  gameState = "sleeping" /* SLEEPING */;
  characterPose = "lying" /* LYING */;
  blanketX = 0;
  blanketY = 0;
  isDragging = false;
  dragOffsetX = 0;
  dragOffsetY = 0;
  characterX = 0;
  characterY = 0;
  walkingProgress = 0;
  waterTemperature = 0;
  bodyTemperature = 50;
  temperatureTimer = 0;
  sliderPosition = 0;
  bodyParts = {
    head: { x: 0, y: 0, radius: 25, wet: false, pulse: 0 },
    leftShoulder: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
    rightShoulder: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
    chest: { x: 0, y: 0, radius: 30, wet: false, pulse: 0 },
    leftArm: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
    rightArm: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
    leftLeg: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 },
    rightLeg: { x: 0, y: 0, radius: 20, wet: false, pulse: 0 }
  };
  waterSplashes = [];
  currentSoapingPart = "head" /* HEAD */;
  soapingGrid = [];
  gridSize = 20;
  isRubbing = false;
  lastRubPosition = null;
  soapingProgress = new Map;
  bubbles = [];
  constructor() {
    this.canvas = document.getElementById("gameCanvas");
    this.ctx = this.canvas.getContext("2d");
    this.setupCanvas();
    this.setupMouseEvents();
    this.init();
  }
  setupCanvas() {
    this.canvas.width = this.width;
    this.canvas.height = this.height;
  }
  setupMouseEvents() {
    this.canvas.addEventListener("mousedown", this.handleMouseDown.bind(this));
    this.canvas.addEventListener("mousemove", this.handleMouseMove.bind(this));
    this.canvas.addEventListener("mouseup", this.handleMouseUp.bind(this));
    this.canvas.addEventListener("mouseleave", this.handleMouseUp.bind(this));
  }
  handleMouseDown(e) {
    const rect = this.canvas.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;
    const centerX = this.width / 2;
    const centerY = this.height / 2;
    
    if (this.gameState === "rinsing") {
      // 클릭한 부위의 거품 제거
      const showerX = this.width * 0.8;
      const baseY = this.height * 0.8 - 180;
      
      // 각 부위 클릭 영역 확인
      if (Math.abs(mouseX - showerX) < 50 && Math.abs(mouseY - (baseY - 50)) < 30) {
        this.foamStatus['머리'] = 0;
        console.log("머리 헹굼!");
      } else if (Math.abs(mouseX - showerX) < 50 && Math.abs(mouseY - baseY) < 40) {
        this.foamStatus['가슴'] = 0;
      } else if (Math.abs(mouseX - showerX) < 60 && Math.abs(mouseY - (baseY + 20)) < 30) {
        this.foamStatus['팔'] = 0;
      } else if (Math.abs(mouseX - showerX) < 50 && Math.abs(mouseY - (baseY + 50)) < 35) {
        this.foamStatus['배'] = 0;
      } else if (Math.abs(mouseX - showerX) < 50 && Math.abs(mouseY - (baseY + 100)) < 30) {
        this.foamStatus['다리'] = 0;
      } else if (Math.abs(mouseX - showerX) < 50 && Math.abs(mouseY - (baseY + 150)) < 20) {
        this.foamStatus['발'] = 0;
      } else if (Math.abs(mouseX - showerX) < 50 && Math.abs(mouseY - baseY) < 40) {
        // 등은 가슴과 같은 높이
        this.foamStatus['등'] = 0;
      }
    } else if (this.gameState === "sleeping" /* SLEEPING */) {
      const blanketLeft = centerX - 60 + this.blanketX;
      const blanketTop = centerY - 45 + this.blanketY;
      const blanketWidth = 190;
      const blanketHeight = 55;
      if (mouseX >= blanketLeft && mouseX <= blanketLeft + blanketWidth && mouseY >= blanketTop && mouseY <= blanketTop + blanketHeight) {
        this.isDragging = true;
        this.dragOffsetX = mouseX - this.blanketX;
        this.dragOffsetY = mouseY - this.blanketY;
      }
    } else if (this.gameState === "blanket_removed" /* BLANKET_REMOVED */) {
      const characterX = centerX - 90;
      const characterY = centerY - 20;
      const distance = Math.sqrt(Math.pow(mouseX - characterX, 2) + Math.pow(mouseY - characterY, 2));
      if (distance < 50) {
        this.characterPose = "standing" /* STANDING */;
        this.gameState = "standing" /* STANDING */;
        console.log("캐릭터가 완전히 일어났습니다!");
      }
    } else if (this.gameState === "standing" /* STANDING */) {
      const doorX = 50;
      const doorY = this.height * 0.7 - 120;
      const doorWidth = 60;
      const doorHeight = 120;
      if (mouseX >= doorX && mouseX <= doorX + doorWidth && mouseY >= doorY && mouseY <= doorY + doorHeight) {
        this.gameState = "walking_to_bathroom" /* WALKING_TO_BATHROOM */;
        this.walkingProgress = 0;
        console.log("욕실로 걸어갑니다!");
      }
    } else if (this.gameState === "adjusting_temperature" /* ADJUSTING_TEMPERATURE */) {
      const faucetX = this.width / 2;
      const faucetY = 150;
      if (mouseX >= faucetX - 200 && mouseX <= faucetX - 140 && mouseY >= faucetY - 30 && mouseY <= faucetY + 30) {
        const randomMove = Math.random() * 15 + 5;
        this.sliderPosition = Math.max(0, this.sliderPosition - randomMove);
        console.log(`온도 낮춤: -${randomMove.toFixed(1)}`);
      }
      if (mouseX >= faucetX + 140 && mouseX <= faucetX + 200 && mouseY >= faucetY - 30 && mouseY <= faucetY + 30) {
        const randomMove = Math.random() * 15 + 5;
        this.sliderPosition = Math.min(100, this.sliderPosition + randomMove);
        console.log(`온도 높임: +${randomMove.toFixed(1)}`);
      }
    } else if (this.gameState === "wetting_body" /* WETTING_BODY */) {
      Object.entries(this.bodyParts).forEach(([name, part]) => {
        const distance = Math.sqrt(Math.pow(mouseX - part.x, 2) + Math.pow(mouseY - part.y, 2));
        if (distance < part.radius && !part.wet) {
          part.wet = true;
          console.log(`${name} 부위에 물을 끼얹었습니다!`);
          for (let i = 0;i < 10; i++) {
            this.waterSplashes.push({
              x: part.x + (Math.random() - 0.5) * 40,
              y: part.y + (Math.random() - 0.5) * 40,
              age: 0
            });
          }
        }
      });
    } else if (this.gameState === "soaping" /* SOAPING */) {
      this.isRubbing = true;
      this.lastRubPosition = { x: mouseX, y: mouseY };
    }
  }
  handleMouseUp() {
    this.isDragging = false;
    this.isRubbing = false;
    this.lastRubPosition = null;
  }
  handleMouseMove(e) {
    const rect = this.canvas.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;
    if (this.isDragging) {
      this.blanketX = mouseX - this.dragOffsetX;
      this.blanketY = mouseY - this.dragOffsetY;
    }
    if (this.isRubbing && this.gameState === "soaping" /* SOAPING */) {
      if (this.lastRubPosition) {
        this.rubArea(mouseX, mouseY);
        if (Math.random() < 0.3) {
          this.bubbles.push({
            x: mouseX + (Math.random() - 0.5) * 20,
            y: mouseY,
            size: Math.random() * 10 + 5,
            age: 0
          });
        }
      }
      this.lastRubPosition = { x: mouseX, y: mouseY };
    }
  }
  init() {
    console.log("게임 초기화 완료!");
    this.gameLoop(0);
  }
  update(deltaTime) {
    if (this.gameState === "rinsing") {
      // 거품 흐름 타이머 업데이트
      this.foamFlowTimer += deltaTime;
      
      // 1초마다 거품이 흘러내림
      if (this.foamFlowTimer > 1000) {
        this.foamFlowTimer = 0;
        
        // 위에서 아래로 거품이 흘러내리는 로직
        // 중요: 이미 헹군 부위라도 위에 거품이 있으면 다시 거품이 생김
        
        // 머리에서 가슴으로
        if (this.foamStatus['머리'] > 0) {
          this.foamStatus['가슴'] = 1;
          console.log("거품이 머리에서 가슴으로 흘러내렸습니다!");
        }
        
        // 가슴/팔/등에서 배로
        if (this.foamStatus['가슴'] > 0 || this.foamStatus['팔'] > 0 || this.foamStatus['등'] > 0) {
          this.foamStatus['배'] = 1;
          if (this.foamStatus['가슴'] > 0) console.log("거품이 가슴에서 배로 흘러내렸습니다!");
          if (this.foamStatus['팔'] > 0) console.log("거품이 팔에서 배로 흘러내렸습니다!");
          if (this.foamStatus['등'] > 0) console.log("거품이 등에서 배로 흘러내렸습니다!");
        }
        
        // 배에서 다리로
        if (this.foamStatus['배'] > 0) {
          this.foamStatus['다리'] = 1;
          console.log("거품이 배에서 다리로 흘러내렸습니다!");
        }
        
        // 다리에서 발로
        if (this.foamStatus['다리'] > 0) {
          this.foamStatus['발'] = 1;
          console.log("거품이 다리에서 발로 흘러내렸습니다!");
        }
      }
    } else if (this.gameState === "sleeping" /* SLEEPING */) {
      const blanketDistance = Math.sqrt(this.blanketX * this.blanketX + this.blanketY * this.blanketY);
      if (blanketDistance > 150) {
        this.gameState = "blanket_removed" /* BLANKET_REMOVED */;
        console.log("이불이 걷어졌습니다!");
      }
    }
    if (this.gameState === "walking_to_bathroom" /* WALKING_TO_BATHROOM */) {
      this.walkingProgress += deltaTime * 0.002;
      const startX = this.width / 2 - 200;
      const targetX = 80;
      this.characterX = startX + (targetX - startX) * Math.min(this.walkingProgress, 1);
      if (this.walkingProgress >= 1) {
        this.gameState = "in_bathroom" /* IN_BATHROOM */;
        this.walkingProgress = 0;
        console.log("욕실에 도착했습니다!");
        setTimeout(() => {
          if (this.gameState === "in_bathroom" /* IN_BATHROOM */) {
            this.gameState = "walking_to_shower" /* WALKING_TO_SHOWER */;
            console.log("샤워기로 이동합니다!");
          }
        }, 1000);
      }
    }
    if (this.gameState === "walking_to_shower" /* WALKING_TO_SHOWER */) {
      this.walkingProgress += deltaTime * 0.001;
      if (this.walkingProgress >= 1) {
        this.gameState = "under_shower" /* UNDER_SHOWER */;
        console.log("샤워기 아래 도착!");
        setTimeout(() => {
          if (this.gameState === "under_shower" /* UNDER_SHOWER */) {
            this.gameState = "adjusting_temperature" /* ADJUSTING_TEMPERATURE */;
            console.log("온도를 조절하세요!");
          }
        }, 500);
      }
    }
    if (this.gameState === "adjusting_temperature" /* ADJUSTING_TEMPERATURE */) {
      const tempDiff = this.sliderPosition - this.waterTemperature;
      this.waterTemperature += tempDiff * 0.05;
      if (this.waterTemperature < 20) {
        this.bodyTemperature -= deltaTime * 0.01;
      } else if (this.waterTemperature > 80) {
        this.bodyTemperature += deltaTime * 0.015;
      } else if (this.waterTemperature >= 35 && this.waterTemperature <= 45) {
        if (this.bodyTemperature < 50) {
          this.bodyTemperature += deltaTime * 0.005;
        } else if (this.bodyTemperature > 50) {
          this.bodyTemperature -= deltaTime * 0.005;
        }
        this.temperatureTimer += deltaTime;
        if (this.temperatureTimer > 3000) {
          this.gameState = "shower_success" /* SHOWER_SUCCESS */;
          console.log("온도 조절 성공!");
          setTimeout(() => {
            if (this.gameState === "shower_success" /* SHOWER_SUCCESS */) {
              this.gameState = "wetting_body" /* WETTING_BODY */;
              this.updateBodyPartPositions();
              console.log("몸에 물을 끼얹으세요!");
            }
          }, 1000);
        }
      } else {
        this.temperatureTimer = 0;
      }
      if (this.bodyTemperature <= 0 || this.bodyTemperature >= 100) {
        this.gameState = "game_over" /* GAME_OVER */;
        console.log("게임 오버!");
      }
    }
    if (this.gameState === "wetting_body" /* WETTING_BODY */) {
      Object.values(this.bodyParts).forEach((part) => {
        if (!part.wet) {
          part.pulse += deltaTime * 0.003;
        }
      });
      this.waterSplashes = this.waterSplashes.filter((splash) => {
        splash.age += deltaTime;
        return splash.age < 500;
      });
      const allWet = Object.values(this.bodyParts).every((part) => part.wet);
      if (allWet && this.gameState === "wetting_body" /* WETTING_BODY */) {
        console.log("모든 부위를 적셨습니다!");
        this.gameState = "transitioning"; // 임시 상태로 변경
        setTimeout(() => {
          if (this.gameState === "transitioning") {
            this.gameState = "soaping" /* SOAPING */;
            this.initSoapingGrid();
            console.log("비누칠 단계 시작!");
          }
        }, 1000);
      }
    }
    if (this.gameState === "soaping" /* SOAPING */) {
      this.bubbles = this.bubbles.filter((bubble) => {
        bubble.age += deltaTime;
        bubble.y -= deltaTime * 0.05;
        bubble.size = bubble.size * (1 - bubble.age / 3000);
        return bubble.age < 3000 && bubble.size > 0.5;
      });
      const progress = this.calculateSoapingProgress();
      this.soapingProgress.set(this.currentSoapingPart, progress);
      if (progress >= 1.0) {
        console.log(`${this.currentSoapingPart} 완료!`);
        this.nextSoapingPart();
      }
    }
  }
  updateBodyPartPositions() {
    const centerX = this.width / 2;
    const baseY = 150;
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
  initRinsing() {
    // 각 부위별 거품 상태 (0: 거품 없음, 1: 거품 있음)
    this.foamStatus = {
      '머리': 1,
      '가슴': 1,
      '팔': 1,
      '배': 1,
      '등': 1,
      '다리': 1,
      '발': 1
    };
    this.rinsedParts = new Set();
    this.foamFlowTimer = 0;
    console.log("헹구기 시작!");
  }
  
  initSoapingGrid() {
    this.soapingGrid = [];
    for (let i = 0;i < this.gridSize; i++) {
      this.soapingGrid[i] = [];
      for (let j = 0;j < this.gridSize; j++) {
        this.soapingGrid[i][j] = false;
      }
    }
    this.bubbles = [];
    this.currentBodyPartIndex = 0;
    this.bodyPartOrder = ['머리', '가슴', '팔', '배', '등', '다리', '발'];
    console.log("비누칠 초기화 완료. 첫 부위:", this.bodyPartOrder[this.currentBodyPartIndex]);
  }
  rubArea(x, y) {
    const centerX = this.width / 2;
    const centerY = this.height / 2;
    const gridCellSize = 10;
    const startX = centerX - this.gridSize * gridCellSize / 2;
    const startY = centerY - this.gridSize * gridCellSize / 2;
    
    const gridX = Math.floor((x - startX) / gridCellSize);
    const gridY = Math.floor((y - startY) / gridCellSize);
    const radius = 2;
    for (let i = -radius;i <= radius; i++) {
      for (let j = -radius;j <= radius; j++) {
        const gx = gridX + i;
        const gy = gridY + j;
        if (gx >= 0 && gx < this.gridSize && gy >= 0 && gy < this.gridSize) {
          this.soapingGrid[gx][gy] = true;
        }
      }
    }
  }
  calculateSoapingProgress() {
    let total = 0;
    let rubbed = 0;
    for (let i = 0;i < this.gridSize; i++) {
      for (let j = 0;j < this.gridSize; j++) {
        total++;
        if (this.soapingGrid[i][j])
          rubbed++;
      }
    }
    return rubbed / total;
  }
  getSoapingPartArea() {
    const centerX = this.width / 2;
    const centerY = this.height / 2;
    switch (this.currentSoapingPart) {
      case "head" /* HEAD */:
        return { x: centerX - 100, y: 50, width: 200, height: 200 };
      case "face" /* FACE */:
        return { x: centerX - 80, y: 100, width: 160, height: 160 };
      case "arm" /* ARM */:
        return { x: centerX - 150, y: 150, width: 300, height: 200 };
      case "armpit" /* ARMPIT */:
        return { x: centerX - 120, y: 120, width: 240, height: 180 };
      case "chest" /* CHEST */:
        return { x: centerX - 120, y: 100, width: 240, height: 250 };
      case "back" /* BACK */:
        return { x: centerX - 120, y: 100, width: 240, height: 250 };
      case "foot" /* FOOT */:
        return { x: centerX - 100, y: 200, width: 200, height: 200 };
      default:
        return { x: 0, y: 0, width: 0, height: 0 };
    }
  }
  nextSoapingPart() {
    this.currentBodyPartIndex++;
    if (this.currentBodyPartIndex < this.bodyPartOrder.length) {
      // 그리드만 초기화 (bodyPartOrder와 currentBodyPartIndex는 유지)
      for (let i = 0; i < this.gridSize; i++) {
        for (let j = 0; j < this.gridSize; j++) {
          this.soapingGrid[i][j] = false;
        }
      }
      this.bubbles = [];
      console.log(`다음 부위: ${this.bodyPartOrder[this.currentBodyPartIndex]}`);
    } else {
      console.log("모든 부위 비누칠 완료!");
      this.gameState = "rinsing";
      this.initRinsing();
    }
  }
  drawRoom() {
    const doorX = 50;
    const doorY = this.height * 0.7 - 120;
    const doorWidth = 60;
    const doorHeight = 120;
    this.ctx.strokeStyle = "#654321";
    this.ctx.lineWidth = 4;
    this.ctx.strokeRect(doorX, doorY, doorWidth, doorHeight);
    this.ctx.fillStyle = "#8B4513";
    this.ctx.fillRect(doorX + 2, doorY + 2, doorWidth - 4, doorHeight - 4);
    this.ctx.fillStyle = "#FFD700";
    this.ctx.beginPath();
    this.ctx.arc(doorX + doorWidth - 15, doorY + doorHeight / 2, 5, 0, Math.PI * 2);
    this.ctx.fill();
    if (this.gameState === "standing" /* STANDING */) {
      this.ctx.strokeStyle = "#FFFF00";
      this.ctx.lineWidth = 2;
      this.ctx.strokeRect(doorX - 2, doorY - 2, doorWidth + 4, doorHeight + 4);
    }
  }
  drawBed() {
    const centerX = this.width / 2;
    const centerY = this.height / 2;
    this.ctx.fillStyle = "#8B4513";
    this.ctx.fillRect(centerX - 150, centerY, 300, 50);
    this.ctx.fillStyle = "#F5F5DC";
    this.ctx.fillRect(centerX - 140, centerY - 15, 280, 15);
    this.ctx.fillStyle = "#FFFFFF";
    this.ctx.fillRect(centerX - 120, centerY - 25, 60, 10);
    this.ctx.fillStyle = "#654321";
    this.ctx.fillRect(centerX - 130, centerY + 50, 20, 40);
    this.ctx.fillRect(centerX + 110, centerY + 50, 20, 40);
  }
  drawCharacter() {
    const centerX = this.width / 2;
    const centerY = this.height / 2;
    if (this.characterPose === "lying" /* LYING */) {
      this.ctx.fillStyle = "#000000";
      this.ctx.beginPath();
      this.ctx.arc(centerX - 90, centerY - 20, 12, 0, Math.PI * 2);
      this.ctx.fill();
      this.ctx.strokeStyle = "#000000";
      this.ctx.lineWidth = 8;
      this.ctx.lineCap = "round";
      this.ctx.beginPath();
      this.ctx.moveTo(centerX - 78, centerY - 20);
      this.ctx.lineTo(centerX + 80, centerY - 20);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX - 50, centerY - 20);
      this.ctx.lineTo(centerX - 50, centerY - 40);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX + 20, centerY - 20);
      this.ctx.lineTo(centerX + 20, centerY - 40);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX + 80, centerY - 20);
      this.ctx.lineTo(centerX + 120, centerY - 35);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX + 80, centerY - 20);
      this.ctx.lineTo(centerX + 120, centerY - 5);
      this.ctx.stroke();
    } else if (this.characterPose === "sitting" /* SITTING */) {
      this.ctx.fillStyle = "#000000";
      this.ctx.beginPath();
      this.ctx.arc(centerX, centerY - 50, 12, 0, Math.PI * 2);
      this.ctx.fill();
      this.ctx.strokeStyle = "#000000";
      this.ctx.lineWidth = 8;
      this.ctx.lineCap = "round";
      this.ctx.beginPath();
      this.ctx.moveTo(centerX, centerY - 38);
      this.ctx.lineTo(centerX, centerY - 10);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX, centerY - 30);
      this.ctx.lineTo(centerX - 20, centerY - 15);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX, centerY - 30);
      this.ctx.lineTo(centerX + 20, centerY - 15);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX, centerY - 10);
      this.ctx.lineTo(centerX - 15, centerY - 10);
      this.ctx.lineTo(centerX - 15, centerY + 10);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(centerX, centerY - 10);
      this.ctx.lineTo(centerX + 15, centerY - 10);
      this.ctx.lineTo(centerX + 15, centerY + 10);
      this.ctx.stroke();
    } else if (this.characterPose === "standing" /* STANDING */) {
      let standingX = centerX - 200;
      if (this.gameState === "walking_to_bathroom" /* WALKING_TO_BATHROOM */) {
        standingX = this.characterX;
      }
      const floorY = this.height * 0.7;
      this.ctx.fillStyle = "#000000";
      this.ctx.beginPath();
      this.ctx.arc(standingX, floorY - 150, 20, 0, Math.PI * 2);
      this.ctx.fill();
      this.ctx.strokeStyle = "#000000";
      this.ctx.lineWidth = 10;
      this.ctx.lineCap = "round";
      this.ctx.beginPath();
      this.ctx.moveTo(standingX, floorY - 130);
      this.ctx.lineTo(standingX, floorY - 60);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(standingX, floorY - 100);
      this.ctx.lineTo(standingX - 30, floorY - 70);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(standingX, floorY - 100);
      this.ctx.lineTo(standingX + 30, floorY - 70);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(standingX, floorY - 60);
      this.ctx.lineTo(standingX - 15, floorY);
      this.ctx.stroke();
      this.ctx.beginPath();
      this.ctx.moveTo(standingX, floorY - 60);
      this.ctx.lineTo(standingX + 15, floorY);
      this.ctx.stroke();
    }
  }
  drawBlanket() {
    const centerX = this.width / 2;
    const centerY = this.height / 2;
    this.ctx.save();
    this.ctx.translate(this.blanketX, this.blanketY);
    this.ctx.fillStyle = "#4682B4";
    this.ctx.beginPath();
    this.ctx.moveTo(centerX - 60, centerY + 10);
    this.ctx.quadraticCurveTo(centerX - 60, centerY - 45, centerX, centerY - 45);
    this.ctx.quadraticCurveTo(centerX + 60, centerY - 45, centerX + 100, centerY - 35);
    this.ctx.quadraticCurveTo(centerX + 130, centerY - 20, centerX + 130, centerY + 10);
    this.ctx.lineTo(centerX - 60, centerY + 10);
    this.ctx.closePath();
    this.ctx.fill();
    if (this.gameState === "soaping") {
      this.canvas.style.cursor = "default";
    } else if (this.isDragging) {
      this.canvas.style.cursor = "grabbing";
    } else if (this.gameState === "sleeping" && 
               this.blanketX > -150 && this.blanketX < 150 && 
               this.blanketY > -150 && this.blanketY < 150) {
      this.canvas.style.cursor = "grab";
    } else {
      this.canvas.style.cursor = "default";
    }
    this.ctx.restore();
  }
  drawTemperatureControl() {
    this.ctx.fillStyle = "#F0F0F0";
    this.ctx.fillRect(0, 0, this.width, this.height);
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "30px Arial";
    this.ctx.textAlign = "center";
    this.ctx.fillText("온도를 조절하세요!", this.width / 2, 50);
    const faucetX = this.width / 2;
    const faucetY = 150;
    this.ctx.fillStyle = "#C0C0C0";
    this.ctx.fillRect(faucetX - 100, faucetY - 20, 200, 40);
    const handleX = faucetX - 100 + this.sliderPosition / 100 * 200;
    this.ctx.fillStyle = "#808080";
    this.ctx.beginPath();
    this.ctx.arc(handleX, faucetY, 25, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.fillStyle = "#4169E1";
    this.ctx.fillRect(faucetX - 200, faucetY - 30, 60, 60);
    this.ctx.fillStyle = "#FFFFFF";
    this.ctx.font = "40px Arial";
    this.ctx.fillText("◀", faucetX - 170, faucetY + 10);
    this.ctx.fillStyle = "#DC143C";
    this.ctx.fillRect(faucetX + 140, faucetY - 30, 60, 60);
    this.ctx.fillStyle = "#FFFFFF";
    this.ctx.fillText("▶", faucetX + 170, faucetY + 10);
    const waterColor = this.getTemperatureColor(this.waterTemperature);
    this.ctx.fillStyle = waterColor;
    this.ctx.fillRect(faucetX - 50, faucetY + 50, 100, 150);
    for (let i = 0;i < 5; i++) {
      this.ctx.fillStyle = waterColor;
      this.ctx.globalAlpha = 0.6;
      this.ctx.fillRect(faucetX - 30 + Math.random() * 60, faucetY + 50 + i * 30, 5, 20);
    }
    this.ctx.globalAlpha = 1;
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "20px Arial";
    this.ctx.fillText("몸 온도", 100, 300);
    this.ctx.fillStyle = "#CCCCCC";
    this.ctx.fillRect(50, 320, 200, 30);
    const bodyTempColor = this.bodyTemperature < 30 ? "#4169E1" : this.bodyTemperature > 70 ? "#DC143C" : "#32CD32";
    this.ctx.fillStyle = bodyTempColor;
    this.ctx.fillRect(50, 320, this.bodyTemperature / 100 * 200, 30);
    this.ctx.strokeStyle = "#00FF00";
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(50 + 30 * 2, 320, 40 * 2, 30);
    if (this.waterTemperature >= 35 && this.waterTemperature <= 45) {
      this.ctx.fillStyle = "#00FF00";
      this.ctx.font = "25px Arial";
      this.ctx.fillText(`유지: ${(this.temperatureTimer / 1000).toFixed(1)}초 / 3초`, this.width / 2, 400);
    }
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "18px Arial";
    this.ctx.fillText(`물 온도: ${Math.round(this.waterTemperature)}°C`, this.width / 2, 450);
  }
  getTemperatureColor(temp) {
    if (temp < 20)
      return "#00BFFF";
    if (temp < 35)
      return "#87CEEB";
    if (temp < 45)
      return "#90EE90";
    if (temp < 60)
      return "#FFA500";
    return "#FF4500";
  }
  drawBathroom() {
    this.ctx.fillStyle = "#E6F3FF";
    this.ctx.fillRect(0, 0, this.width, this.height);
    this.ctx.fillStyle = "#B0C4DE";
    this.ctx.fillRect(0, this.height * 0.8, this.width, this.height * 0.2);
    const showerX = this.width / 2;
    const showerY = 100;
    this.ctx.strokeStyle = "#C0C0C0";
    this.ctx.lineWidth = 8;
    this.ctx.beginPath();
    this.ctx.moveTo(showerX, 50);
    this.ctx.lineTo(showerX, showerY);
    this.ctx.stroke();
    this.ctx.fillStyle = "#C0C0C0";
    this.ctx.beginPath();
    this.ctx.arc(showerX, showerY, 30, 0, Math.PI, false);
    this.ctx.fill();
    this.ctx.fillStyle = "#808080";
    for (let i = -2;i <= 2; i++) {
      for (let j = 0;j < 2; j++) {
        this.ctx.beginPath();
        this.ctx.arc(showerX + i * 10, showerY + j * 10 - 5, 2, 0, Math.PI * 2);
        this.ctx.fill();
      }
    }
    this.ctx.strokeStyle = "#87CEEB";
    this.ctx.lineWidth = 3;
    this.ctx.strokeRect(showerX - 100, 150, 200, 300);
    if (this.gameState === "in_bathroom" /* IN_BATHROOM */) {
      this.drawBathroomCharacter(100, this.height * 0.8);
    } else if (this.gameState === "walking_to_shower" /* WALKING_TO_SHOWER */) {
      const x = 100 + (showerX - 100) * this.walkingProgress;
      this.drawBathroomCharacter(x, this.height * 0.8);
    } else if (this.gameState === "under_shower" /* UNDER_SHOWER */) {
      this.drawBathroomCharacter(showerX, this.height * 0.8);
    }
  }
  drawRinsingInstruction() {
    // 이제 사용하지 않음
  }
  
  drawRinsing() {
    this.drawBathroom();
    
    // 샤워기 아래 캐릭터
    const showerX = this.width * 0.8;
    this.drawBathroomCharacter(showerX, this.height * 0.8);
    
    // 거품 표시
    const centerX = showerX;
    const baseY = this.height * 0.8 - 180;
    
    // 각 부위에 거품 그리기
    this.ctx.fillStyle = "rgba(255, 255, 255, 0.8)";
    if (this.foamStatus['머리'] > 0) {
      this.drawFoam(centerX, baseY - 50, 30);
    }
    if (this.foamStatus['가슴'] > 0) {
      this.drawFoam(centerX, baseY, 40);
    }
    if (this.foamStatus['팔'] > 0) {
      this.drawFoam(centerX - 40, baseY + 20, 20);
      this.drawFoam(centerX + 40, baseY + 20, 20);
    }
    if (this.foamStatus['배'] > 0) {
      this.drawFoam(centerX, baseY + 50, 35);
    }
    if (this.foamStatus['등'] > 0) {
      // 등은 가슴 뒤쪽에 그림
      this.ctx.fillStyle = "rgba(255, 255, 255, 0.6)";
      this.drawFoam(centerX - 10, baseY - 10, 30);
      this.ctx.fillStyle = "rgba(255, 255, 255, 0.8)";
    }
    if (this.foamStatus['다리'] > 0) {
      this.drawFoam(centerX - 20, baseY + 100, 25);
      this.drawFoam(centerX + 20, baseY + 100, 25);
    }
    if (this.foamStatus['발'] > 0) {
      this.drawFoam(centerX - 20, baseY + 150, 15);
      this.drawFoam(centerX + 20, baseY + 150, 15);
    }
    
    // 거품이 흘러내리는 애니메이션 효과
    const flowProgress = (this.foamFlowTimer % 1000) / 1000;
    this.ctx.fillStyle = "rgba(255, 255, 255, 0.4)";
    
    // 머리에서 가슴으로 흐르는 거품
    if (this.foamStatus['머리'] > 0) {
      const flowY = (baseY - 50) + ((baseY) - (baseY - 50)) * flowProgress;
      this.drawSmallFoam(centerX, flowY, 10);
    }
    
    // 가슴에서 배로 흐르는 거품
    if (this.foamStatus['가슴'] > 0) {
      const flowY = baseY + (baseY + 50 - baseY) * flowProgress;
      this.drawSmallFoam(centerX, flowY, 10);
    }
    
    // 배에서 다리로 흐르는 거품
    if (this.foamStatus['배'] > 0) {
      const flowY = (baseY + 50) + ((baseY + 100) - (baseY + 50)) * flowProgress;
      this.drawSmallFoam(centerX, flowY, 10);
    }
    
    // 안내 텍스트
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "20px Arial";
    this.ctx.textAlign = "center";
    this.ctx.fillText("마우스를 클릭해서 거품을 헹구세요!", this.width / 2, this.height - 30);
    
    // 완료 체크
    const allRinsed = Object.values(this.foamStatus).every(status => status === 0);
    if (allRinsed) {
      console.log("헹구기 완료!");
      // TODO: 다음 단계로
    }
  }
  
  drawFoam(x, y, size) {
    for (let i = 0; i < 5; i++) {
      const offsetX = (Math.random() - 0.5) * size;
      const offsetY = (Math.random() - 0.5) * size;
      const bubbleSize = Math.random() * 10 + 5;
      
      this.ctx.beginPath();
      this.ctx.arc(x + offsetX, y + offsetY, bubbleSize, 0, Math.PI * 2);
      this.ctx.fill();
      this.ctx.strokeStyle = "rgba(200, 200, 200, 0.5)";
      this.ctx.stroke();
    }
  }
  
  drawSmallFoam(x, y, size) {
    // 흘러내리는 작은 거품들
    for (let i = 0; i < 3; i++) {
      const offsetX = (Math.random() - 0.5) * size;
      const bubbleSize = Math.random() * 5 + 3;
      
      this.ctx.beginPath();
      this.ctx.arc(x + offsetX, y, bubbleSize, 0, Math.PI * 2);
      this.ctx.fill();
      this.ctx.strokeStyle = "rgba(200, 200, 200, 0.3)";
      this.ctx.stroke();
    }
  }
  
  drawSoaping() {
    this.ctx.fillStyle = "#F0F8FF";
    this.ctx.fillRect(0, 0, this.width, this.height);
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "25px Arial";
    this.ctx.textAlign = "center";
    this.ctx.fillText(`${this.currentBodyPartIndex + 1}/${this.bodyPartOrder.length} - ${this.bodyPartOrder[this.currentBodyPartIndex]} 비누칠하기`, this.width / 2, 40);
    const progress = this.calculateSoapingProgress();
    this.ctx.fillStyle = "#CCCCCC";
    this.ctx.fillRect(this.width / 2 - 150, 60, 300, 20);
    this.ctx.fillStyle = "#4169E1";
    this.ctx.fillRect(this.width / 2 - 150, 60, 300 * progress, 20);
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "15px Arial";
    this.ctx.fillText(`${Math.round(progress * 100)}%`, this.width / 2, 100);
    const currentPart = this.bodyPartOrder[this.currentBodyPartIndex];
    const centerX = this.width / 2;
    const centerY = this.height / 2;
    const gridCellSize = 10;
    const startX = centerX - this.gridSize * gridCellSize / 2;
    const startY = centerY - this.gridSize * gridCellSize / 2;
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
        this.ctx.beginPath();
        this.ctx.arc(centerX - 15, centerY - 10, 5, 0, Math.PI * 2);
        this.ctx.arc(centerX + 15, centerY - 10, 5, 0, Math.PI * 2);
        this.ctx.stroke();
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
      case "다리":
        this.ctx.beginPath();
        this.ctx.rect(centerX - 30, centerY - 40, 60, 100);
        this.ctx.stroke();
        break;
      case "발":
        this.ctx.beginPath();
        this.ctx.ellipse(centerX, centerY, 40, 60, 0, 0, Math.PI * 2);
        this.ctx.stroke();
        for (let i = 0;i < 5; i++) {
          this.ctx.beginPath();
          this.ctx.arc(centerX - 20 + i * 10, centerY - 50, 5, 0, Math.PI * 2);
          this.ctx.stroke();
        }
        break;
    }
    for (let i = 0;i < this.gridSize; i++) {
      for (let j = 0;j < this.gridSize; j++) {
        const x = startX + i * gridCellSize;
        const y = startY + j * gridCellSize;
        if (this.soapingGrid[i][j]) {
          this.ctx.fillStyle = "rgba(255, 255, 255, 0.8)";
          this.ctx.fillRect(x, y, gridCellSize - 1, gridCellSize - 1);
        } else {
          this.ctx.fillStyle = "rgba(200, 200, 200, 0.3)";
          this.ctx.fillRect(x, y, gridCellSize - 1, gridCellSize - 1);
        }
      }
    }
    this.bubbles.forEach((bubble) => {
      this.ctx.fillStyle = "rgba(255, 255, 255, 0.9)";
      this.ctx.strokeStyle = "rgba(150, 150, 200, 0.6)";
      this.ctx.lineWidth = 1;
      this.ctx.beginPath();
      this.ctx.arc(bubble.x, bubble.y, bubble.size, 0, Math.PI * 2);
      this.ctx.fill();
      this.ctx.stroke();
    });
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "18px Arial";
    this.ctx.fillText("마우스를 드래그해서 비누칠하세요!", this.width / 2, this.height - 50);
  }
  drawWettingBody() {
    this.ctx.fillStyle = "#E6F3FF";
    this.ctx.fillRect(0, 0, this.width, this.height);
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "25px Arial";
    this.ctx.textAlign = "center";
    this.ctx.fillText("몸에 물을 끼얹으세요!", this.width / 2, 40);
    const centerX = this.width / 2;
    const baseY = 150;
    this.ctx.fillStyle = this.bodyParts.head.wet ? "#87CEEB" : "#000000";
    this.ctx.beginPath();
    this.ctx.arc(centerX, baseY, 30, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.strokeStyle = "#000000";
    this.ctx.lineWidth = 12;
    this.ctx.lineCap = "round";
    this.ctx.beginPath();
    this.ctx.moveTo(centerX, baseY + 30);
    this.ctx.lineTo(centerX, baseY + 100);
    this.ctx.stroke();
    this.ctx.beginPath();
    this.ctx.moveTo(centerX, baseY + 50);
    this.ctx.lineTo(centerX - 60, baseY + 100);
    this.ctx.moveTo(centerX, baseY + 50);
    this.ctx.lineTo(centerX + 60, baseY + 100);
    this.ctx.stroke();
    this.ctx.beginPath();
    this.ctx.moveTo(centerX, baseY + 100);
    this.ctx.lineTo(centerX - 30, baseY + 170);
    this.ctx.moveTo(centerX, baseY + 100);
    this.ctx.lineTo(centerX + 30, baseY + 170);
    this.ctx.stroke();
    Object.values(this.bodyParts).forEach((part) => {
      if (!part.wet) {
        const opacity = 0.3 + Math.sin(part.pulse) * 0.2;
        this.ctx.strokeStyle = `rgba(0, 150, 255, ${opacity})`;
        this.ctx.lineWidth = 2;
        this.ctx.setLineDash([5, 5]);
        this.ctx.beginPath();
        this.ctx.arc(part.x, part.y, part.radius, 0, Math.PI * 2);
        this.ctx.stroke();
        this.ctx.setLineDash([]);
        if (Math.sin(part.pulse) > 0.8) {
          this.ctx.fillStyle = `rgba(0, 150, 255, 0.2)`;
          this.ctx.beginPath();
          this.ctx.arc(part.x, part.y, part.radius * 0.8, 0, Math.PI * 2);
          this.ctx.fill();
        }
      } else {
        this.ctx.fillStyle = "rgba(0, 150, 255, 0.3)";
        this.ctx.beginPath();
        this.ctx.arc(part.x, part.y, part.radius, 0, Math.PI * 2);
        this.ctx.fill();
      }
    });
    this.ctx.fillStyle = "rgba(0, 150, 255, 0.6)";
    this.waterSplashes.forEach((splash) => {
      const alpha = 1 - splash.age / 500;
      this.ctx.globalAlpha = alpha * 0.6;
      this.ctx.beginPath();
      this.ctx.arc(splash.x, splash.y + splash.age * 0.2, 3, 0, Math.PI * 2);
      this.ctx.fill();
    });
    this.ctx.globalAlpha = 1;
    const wetCount = Object.values(this.bodyParts).filter((part) => part.wet).length;
    const totalCount = Object.values(this.bodyParts).length;
    this.ctx.fillStyle = "#000000";
    this.ctx.font = "20px Arial";
    this.ctx.fillText(`진행률: ${wetCount}/${totalCount}`, this.width / 2, this.height - 50);
  }
  drawBathroomCharacter(x, floorY) {
    this.ctx.fillStyle = "#000000";
    this.ctx.beginPath();
    this.ctx.arc(x, floorY - 150, 20, 0, Math.PI * 2);
    this.ctx.fill();
    this.ctx.strokeStyle = "#000000";
    this.ctx.lineWidth = 10;
    this.ctx.lineCap = "round";
    this.ctx.beginPath();
    this.ctx.moveTo(x, floorY - 130);
    this.ctx.lineTo(x, floorY - 60);
    this.ctx.stroke();
    this.ctx.beginPath();
    this.ctx.moveTo(x, floorY - 100);
    this.ctx.lineTo(x - 30, floorY - 70);
    this.ctx.moveTo(x, floorY - 100);
    this.ctx.lineTo(x + 30, floorY - 70);
    this.ctx.stroke();
    this.ctx.beginPath();
    this.ctx.moveTo(x, floorY - 60);
    this.ctx.lineTo(x - 15, floorY);
    this.ctx.moveTo(x, floorY - 60);
    this.ctx.lineTo(x + 15, floorY);
    this.ctx.stroke();
  }
  render() {
    this.ctx.fillStyle = "#2C3E50";
    this.ctx.fillRect(0, 0, this.width, this.height);
    this.ctx.fillStyle = "#34495E";
    this.ctx.fillRect(0, this.height * 0.7, this.width, this.height * 0.3);
    if (this.gameState === "in_bathroom" /* IN_BATHROOM */ || this.gameState === "walking_to_shower" /* WALKING_TO_SHOWER */ || this.gameState === "under_shower" /* UNDER_SHOWER */) {
      this.drawBathroom();
    } else if (this.gameState === "adjusting_temperature" /* ADJUSTING_TEMPERATURE */) {
      this.drawTemperatureControl();
    } else if (this.gameState === "game_over" /* GAME_OVER */) {
      this.ctx.fillStyle = "#000000";
      this.ctx.fillRect(0, 0, this.width, this.height);
      this.ctx.fillStyle = "#FFFFFF";
      this.ctx.font = "40px Arial";
      this.ctx.textAlign = "center";
      this.ctx.fillText("게임 오버!", this.width / 2, this.height / 2);
      this.ctx.font = "20px Arial";
      this.ctx.fillText(this.bodyTemperature <= 0 ? "너무 차가워!" : "너무 뜨거워!", this.width / 2, this.height / 2 + 50);
    } else if (this.gameState === "shower_success" /* SHOWER_SUCCESS */) {
      this.ctx.fillStyle = "#87CEEB";
      this.ctx.fillRect(0, 0, this.width, this.height);
      this.ctx.fillStyle = "#000000";
      this.ctx.font = "40px Arial";
      this.ctx.textAlign = "center";
      this.ctx.fillText("성공!", this.width / 2, this.height / 2);
      this.ctx.font = "20px Arial";
      this.ctx.fillText("완벽한 온도입니다!", this.width / 2, this.height / 2 + 50);
    } else if (this.gameState === "wetting_body" /* WETTING_BODY */) {
      this.drawWettingBody();
    } else if (this.gameState === "soaping" /* SOAPING */) {
      this.drawSoaping();
    } else if (this.gameState === "rinsing_instruction") {
      this.drawRinsingInstruction();
    } else if (this.gameState === "rinsing") {
      this.drawRinsing();
    } else {
      this.drawRoom();
      this.drawBed();
      this.drawCharacter();
      if (this.gameState === "sleeping" /* SLEEPING */) {
        this.drawBlanket();
      }
    }
  }
  gameLoop = (currentTime) => {
    const deltaTime = currentTime - this.lastTime;
    this.lastTime = currentTime;
    this.update(deltaTime);
    this.render();
    requestAnimationFrame(this.gameLoop);
  };
}
window.addEventListener("DOMContentLoaded", () => {
  new Game;
});

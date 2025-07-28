import { MIN_PERFECT_TEMP, MAX_PERFECT_TEMP } from '../constants/gameData';

// 물 온도 색상 계산
export const getWaterColor = (temp: number): string => {
  if (temp > 60) return '뜨거운 빨간';
  if (temp > 45) return '따뜻한 주황';
  if (temp > 35) return '적당한 연두';
  if (temp > 20) return '시원한 하늘';
  return '차가운 파란';
};

// 체온 변화 계산
export const calculateBodyTempChange = (currentBodyTemp: number, waterTemp: number): number => {
  if (waterTemp >= MIN_PERFECT_TEMP && waterTemp <= MAX_PERFECT_TEMP) {
    // 적정 온도: 체온을 36.5도로 조절
    if (currentBodyTemp < 36.5) return 0.025;
    if (currentBodyTemp > 36.5) return -0.025;
    return 0;
  } else if (waterTemp < MIN_PERFECT_TEMP) {
    // 차가운 물: 체온 하락
    return -0.1;
  } else {
    // 뜨거운 물: 체온 상승
    return 0.15;
  }
};

// 완벽한 온도 체크
export const isPerfectTemperature = (waterTemp: number): boolean => {
  return waterTemp >= MIN_PERFECT_TEMP && waterTemp <= MAX_PERFECT_TEMP;
};

// 체온 위험 체크
export const isBodyTempDangerous = (bodyTemp: number): boolean => {
  return bodyTemp <= 35 || bodyTemp >= 38;
};

// 온도 조절 랜덤 변화량
export const getRandomTempChange = (): number => {
  return Math.floor(Math.random() * 15) + 5;
};

// 비누칠 진행도 증가량
export const getSoapProgressIncrease = (intense: boolean): number => {
  if (intense) {
    return Math.random() * 30 + 25;
  }
  return Math.random() * 20 + 15;
};

// 진행도 바 생성
export const createProgressBar = (progress: number, length: number = 10): string => {
  const filled = Math.floor(progress / (100 / length));
  const empty = length - filled;
  return '█'.repeat(filled) + '░'.repeat(empty);
};

// 랜덤 아이템 선택
export const getRandomItem = <T>(items: T[]): T => {
  return items[Math.floor(Math.random() * items.length)];
};

// 드라이어 거리 효과
export const getDryerEffectiveness = (clickCount: number): 'close' | 'medium' | 'far' => {
  const distance = clickCount % 3;
  if (distance === 0) return 'close';
  if (distance === 1) return 'medium';
  return 'far';
};
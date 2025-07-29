// Virtual Users Simulation System
// 실제 백엔드 없이 사회적 현존감을 제공하기 위한 가상 유저 시스템

export type VirtualUserState = 'idle' | 'focusing' | 'resting' | 'offline';

export interface VirtualUser {
  id: string;
  name: string;
  state: VirtualUserState;
  currentSessionStart: number | null;
  sessionDuration: number; // 이 유저의 일반적인 집중 시간 (분)
  lastActivityTime: number;
  pattern: 'regular' | 'irregular' | 'night-owl' | 'early-bird'; // 활동 패턴
}

// 시간대별 활동 확률
const ACTIVITY_PROBABILITY_BY_HOUR: Record<number, number> = {
  0: 0.3,   // 00시: 30%
  1: 0.25,  // 01시: 25%
  2: 0.2,   // 02시: 20%
  3: 0.15,  // 03시: 15%
  4: 0.1,   // 04시: 10%
  5: 0.1,   // 05시: 10%
  6: 0.15,  // 06시: 15%
  7: 0.25,  // 07시: 25%
  8: 0.35,  // 08시: 35%
  9: 0.45,  // 09시: 45%
  10: 0.5,  // 10시: 50%
  11: 0.5,  // 11시: 50%
  12: 0.4,  // 12시: 40%
  13: 0.45, // 13시: 45%
  14: 0.5,  // 14시: 50%
  15: 0.5,  // 15시: 50%
  16: 0.45, // 16시: 45%
  17: 0.4,  // 17시: 40%
  18: 0.35, // 18시: 35%
  19: 0.45, // 19시: 45%
  20: 0.6,  // 20시: 60%
  21: 0.65, // 21시: 65%
  22: 0.6,  // 22시: 60%
  23: 0.45, // 23시: 45%
};

// 가상 유저 이름 풀
const VIRTUAL_USER_NAMES = [
  '열공러', '밤샘전사', '카페인중독자', '도서관지박령', '새벽감성',
  '집중마스터', '뽀모도로왕', '시험기간', '과제늦둥이', '독서광',
  '코딩전사', '수능파이터', '토익준비생', '논문쓰는중', '자격증도전',
  '공시생', '취준생', '대학원생', '의대생', '간호대생',
  '프리랜서', '재택근무자', '스터디원', '동기부여필요', '작심삼일극복',
  '습관만들기', '매일조금씩', '꾸준함이답', '오늘도화이팅', '할수있다',
  '하연팬1호', '하연이와함께', '아이돌덕후', '응원력충전중', '함께해요',
  '새벽2시클럽', '올빼미족', '얼리버드', '아침형인간', '저녁형인간',
  '주말전사', '평일만집중', '시험D-10', '마감D-1', '벼락치기중',
  '계획형인간', '즉흥형인간', '완벽주의자', '적당주의자', '끝까지간다'
];

class VirtualUsersManager {
  private users: Map<string, VirtualUser> = new Map();
  private updateInterval: NodeJS.Timeout | null = null;
  
  constructor() {
    this.initializeUsers();
  }

  private initializeUsers() {
    const userCount = Math.floor(Math.random() * 41) + 10; // 10-50명
    
    for (let i = 0; i < userCount; i++) {
      const user = this.createVirtualUser(i);
      this.users.set(user.id, user);
    }
  }

  private createVirtualUser(index: number): VirtualUser {
    const patterns: VirtualUser['pattern'][] = ['regular', 'irregular', 'night-owl', 'early-bird'];
    const pattern = patterns[Math.floor(Math.random() * patterns.length)];
    
    // 패턴에 따른 세션 시간 설정
    let sessionDuration: number;
    switch (pattern) {
      case 'regular':
        sessionDuration = 25; // 정석 25분
        break;
      case 'irregular':
        sessionDuration = Math.floor(Math.random() * 20) + 10; // 10-30분
        break;
      case 'night-owl':
        sessionDuration = Math.floor(Math.random() * 15) + 20; // 20-35분
        break;
      case 'early-bird':
        sessionDuration = Math.floor(Math.random() * 10) + 20; // 20-30분
        break;
    }

    const currentHour = new Date().getHours();
    const activityProb = ACTIVITY_PROBABILITY_BY_HOUR[currentHour];
    const isActive = Math.random() < activityProb;

    return {
      id: `virtual-user-${index}`,
      name: VIRTUAL_USER_NAMES[index % VIRTUAL_USER_NAMES.length],
      state: isActive ? 'idle' : 'offline',
      currentSessionStart: null,
      sessionDuration,
      lastActivityTime: Date.now(),
      pattern,
    };
  }

  // 실시간 업데이트 시작
  startSimulation() {
    if (this.updateInterval) return;
    
    this.updateInterval = setInterval(() => {
      this.updateUserStates();
    }, 5000); // 5초마다 업데이트
  }

  // 실시간 업데이트 중지
  stopSimulation() {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }
  }

  private updateUserStates() {
    const currentTime = Date.now();
    const currentHour = new Date().getHours();
    const baseActivityProb = ACTIVITY_PROBABILITY_BY_HOUR[currentHour];

    this.users.forEach(user => {
      // 패턴별 활동 확률 조정
      let activityProb = baseActivityProb;
      if (user.pattern === 'night-owl' && currentHour >= 22 || currentHour <= 3) {
        activityProb *= 1.5;
      } else if (user.pattern === 'early-bird' && currentHour >= 5 && currentHour <= 9) {
        activityProb *= 1.5;
      }

      switch (user.state) {
        case 'offline':
          // 오프라인 -> 온라인 전환 확률
          if (Math.random() < activityProb * 0.1) {
            user.state = 'idle';
            user.lastActivityTime = currentTime;
          }
          break;

        case 'idle':
          // 대기 -> 집중 시작 확률
          if (Math.random() < 0.2) {
            user.state = 'focusing';
            user.currentSessionStart = currentTime;
            user.lastActivityTime = currentTime;
          }
          // 대기 -> 오프라인 확률
          else if (Math.random() < 0.05) {
            user.state = 'offline';
          }
          break;

        case 'focusing':
          // 집중 시간 체크
          if (user.currentSessionStart) {
            const elapsedMinutes = (currentTime - user.currentSessionStart) / 60000;
            
            // 설정된 세션 시간 도달
            if (elapsedMinutes >= user.sessionDuration) {
              user.state = 'resting';
              user.lastActivityTime = currentTime;
            }
            // 중도 포기 확률 (시간이 지날수록 증가)
            else if (Math.random() < 0.01 * (elapsedMinutes / user.sessionDuration)) {
              user.state = 'idle';
              user.currentSessionStart = null;
            }
          }
          break;

        case 'resting':
          // 휴식 -> 대기 전환 (5분 후)
          const restTime = (currentTime - user.lastActivityTime) / 60000;
          if (restTime >= 5) {
            user.state = 'idle';
            user.currentSessionStart = null;
          }
          break;
      }
    });
  }

  // 현재 활동 중인 유저 수
  getActiveUserCount(): number {
    return Array.from(this.users.values()).filter(user => user.state !== 'offline').length;
  }

  // 현재 집중 중인 유저 수
  getFocusingUserCount(): number {
    return Array.from(this.users.values()).filter(user => user.state === 'focusing').length;
  }

  // 상태별 유저 목록
  getUsersByState(state: VirtualUserState): VirtualUser[] {
    return Array.from(this.users.values()).filter(user => user.state === state);
  }

  // 모든 활동 중인 유저 목록
  getActiveUsers(): VirtualUser[] {
    return Array.from(this.users.values()).filter(user => user.state !== 'offline');
  }
}

// 싱글톤 인스턴스
export const virtualUsersManager = new VirtualUsersManager();
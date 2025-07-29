// 공동 목표 시스템
// 모든 유저의 집중 시간을 합산하여 일일 공동 목표 달성

import AsyncStorage from '@react-native-async-storage/async-storage';
import { virtualUsersManager } from './virtualUsers';

const STORAGE_KEY = '@collective_goal';
const DAILY_GOAL_MINUTES = 5; // 테스트를 위해 5분으로 설정 (실제: 1200분)

export interface CollectiveGoalData {
  date: string; // YYYY-MM-DD
  totalMinutes: number; // 총 누적 시간 (분)
  userContribution: number; // 실제 사용자의 기여 시간 (분)
  virtualContribution: number; // 가상 유저들의 기여 시간 (분)
  isGoalAchieved: boolean;
  lastUpdated: number; // timestamp
}

class CollectiveGoalManager {
  private data: CollectiveGoalData | null = null;
  private updateInterval: NodeJS.Timeout | null = null;
  private virtualTimeAccumulator: number = 0;

  constructor() {
    this.loadData();
  }

  private async loadData() {
    try {
      const stored = await AsyncStorage.getItem(STORAGE_KEY);
      if (stored) {
        this.data = JSON.parse(stored);
        // 날짜가 바뀌었으면 초기화
        if (this.data && this.data.date !== this.getTodayDate()) {
          this.resetDaily();
        }
      } else {
        this.resetDaily();
      }
    } catch (error) {
      console.error('Failed to load collective goal data:', error);
      this.resetDaily();
    }
  }

  private async saveData() {
    if (this.data) {
      try {
        await AsyncStorage.setItem(STORAGE_KEY, JSON.stringify(this.data));
      } catch (error) {
        console.error('Failed to save collective goal data:', error);
      }
    }
  }

  private getTodayDate(): string {
    return new Date().toISOString().split('T')[0];
  }

  private resetDaily() {
    this.data = {
      date: this.getTodayDate(),
      totalMinutes: 0,
      userContribution: 0,
      virtualContribution: 0,
      isGoalAchieved: false,
      lastUpdated: Date.now(),
    };
    this.virtualTimeAccumulator = 0;
    this.saveData();
  }

  // 실제 사용자의 집중 시간 추가
  async addUserContribution(minutes: number) {
    if (!this.data) await this.loadData();
    if (!this.data) return;

    this.data.userContribution += minutes;
    this.data.totalMinutes = this.data.userContribution + this.data.virtualContribution;
    this.data.lastUpdated = Date.now();

    // 목표 달성 체크
    if (!this.data.isGoalAchieved && this.data.totalMinutes >= DAILY_GOAL_MINUTES) {
      this.data.isGoalAchieved = true;
    }

    await this.saveData();
  }

  // 가상 유저 시뮬레이션 시작
  startVirtualContribution() {
    if (this.updateInterval) return;

    // 5초마다 가상 유저들의 기여도 업데이트
    this.updateInterval = setInterval(() => {
      this.updateVirtualContribution();
    }, 5000);
  }

  // 가상 유저 시뮬레이션 중지
  stopVirtualContribution() {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }
  }

  private async updateVirtualContribution() {
    const focusingUsers = virtualUsersManager.getUsersByState('focusing');
    
    // 5초 동안 집중한 가상 유저들의 시간 계산 (5초 = 5/60분)
    const contributionPerUser = 5 / 60; // 분 단위
    const totalContribution = focusingUsers.length * contributionPerUser;
    
    this.virtualTimeAccumulator += totalContribution;
    
    // 1분 단위로 누적하여 저장 (너무 자주 저장하지 않기 위해)
    if (this.virtualTimeAccumulator >= 1) {
      const minutesToAdd = Math.floor(this.virtualTimeAccumulator);
      this.virtualTimeAccumulator -= minutesToAdd;
      
      if (!this.data) await this.loadData();
      if (!this.data) return;
      
      this.data.virtualContribution += minutesToAdd;
      this.data.totalMinutes = this.data.userContribution + this.data.virtualContribution;
      this.data.lastUpdated = Date.now();
      
      // 목표 달성 체크
      if (!this.data.isGoalAchieved && this.data.totalMinutes >= DAILY_GOAL_MINUTES) {
        this.data.isGoalAchieved = true;
      }
      
      await this.saveData();
    }
  }

  // 현재 진행 상황 가져오기
  async getProgress(): Promise<{
    currentMinutes: number;
    goalMinutes: number;
    percentage: number;
    isAchieved: boolean;
    userContribution: number;
    virtualContribution: number;
  }> {
    if (!this.data) await this.loadData();
    
    // 날짜가 바뀌었으면 초기화
    if (this.data && this.data.date !== this.getTodayDate()) {
      this.resetDaily();
    }
    
    const currentMinutes = this.data?.totalMinutes || 0;
    const percentage = Math.min((currentMinutes / DAILY_GOAL_MINUTES) * 100, 100);
    
    return {
      currentMinutes,
      goalMinutes: DAILY_GOAL_MINUTES,
      percentage,
      isAchieved: this.data?.isGoalAchieved || false,
      userContribution: this.data?.userContribution || 0,
      virtualContribution: this.data?.virtualContribution || 0,
    };
  }

  // 진행률에 따른 메시지 가져오기
  getProgressMessage(percentage: number, playerName: string): string {
    if (percentage >= 100) {
      return `${playerName}님과 모든 팬분들 덕분에 오늘의 연습 목표 달성! 정말 든든해요!`;
    } else if (percentage >= 80) {
      return `우와! 다들 정말 대단해요! 목표 달성이 코앞이에요!`;
    } else if (percentage >= 60) {
      return `벌써 ${Math.floor(percentage)}%나 달성했어요! 모두 화이팅!`;
    } else if (percentage >= 40) {
      return `함께하니까 정말 힘이 나요! 계속 파이팅해요!`;
    } else if (percentage >= 20) {
      return `좋은 시작이에요! 오늘도 다 함께 열심히 해봐요!`;
    } else {
      return `오늘의 연습이 시작됐어요! ${playerName}님도 함께해요!`;
    }
  }

  // 시간 포맷팅 헬퍼
  formatTime(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    return `${hours}시간 ${mins}분`;
  }
}

// 싱글톤 인스턴스
export const collectiveGoalManager = new CollectiveGoalManager();
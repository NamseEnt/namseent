// 사용자들의 흔적(메시지) 관리 시스템
import AsyncStorage from '@react-native-async-storage/async-storage';

const TRACES_KEY = 'user_traces';
const MAX_TRACES = 50;
const TRACE_DURATION = 3 * 60 * 60 * 1000; // 3시간

export interface UserTrace {
  id: string;
  name: string;
  message: string;
  timestamp: number;
}

class TracesManager {
  private traces: UserTrace[] = [];
  private virtualTraces: string[] = [
    "영단어 50개",
    "알고리즘 3문제",
    "논문 10페이지",
    "수학 1챕터",
    "코딩 테스트",
    "토익 LC",
    "전공서적",
    "기획서 작성",
    "면접 준비",
    "자격증 1시간",
    "독서 30p",
    "운동 완료",
    "일본어 단어",
    "포트폴리오",
    "계획 완료",
    "집중 연습",
    "새벽 공부",
    "목표 달성",
    "과제 완료",
    "시험 대비",
  ];
  
  constructor() {
    this.loadTraces();
    this.initializeVirtualTraces();
  }
  
  private async loadTraces() {
    try {
      const stored = await AsyncStorage.getItem(TRACES_KEY);
      if (stored) {
        this.traces = JSON.parse(stored);
        this.cleanupOldTraces();
      }
    } catch (error) {
      console.error('Failed to load traces:', error);
    }
  }
  
  private async saveTraces() {
    try {
      await AsyncStorage.setItem(TRACES_KEY, JSON.stringify(this.traces));
    } catch (error) {
      console.error('Failed to save traces:', error);
    }
  }
  
  private cleanupOldTraces() {
    const now = Date.now();
    this.traces = this.traces.filter(trace => 
      now - trace.timestamp < TRACE_DURATION
    );
  }
  
  private initializeVirtualTraces() {
    // 초기 가상 흔적 추가 (1-3시간 전 랜덤)
    const now = Date.now();
    for (let i = 0; i < 5; i++) {
      const randomMinutesAgo = Math.floor(Math.random() * 180); // 0-180분
      this.traces.push({
        id: `virtual-${i}`,
        name: this.generateVirtualName(),
        message: this.virtualTraces[Math.floor(Math.random() * this.virtualTraces.length)],
        timestamp: now - (randomMinutesAgo * 60 * 1000),
      });
    }
  }
  
  private generateVirtualName(): string {
    const names = [
      "user_1", "user_2", "user_3", "user_4", "user_5",
      "user_6", "user_7", "user_8", "user_9", "user_10",
      "user_11", "user_12", "user_13", "user_14", "user_15",
    ];
    return names[Math.floor(Math.random() * names.length)];
  }
  
  async addTrace(name: string, message: string) {
    const newTrace: UserTrace = {
      id: `user-${Date.now()}`,
      name,
      message: message.slice(0, 30), // 최대 30자
      timestamp: Date.now(),
    };
    
    this.traces.unshift(newTrace);
    
    // 최대 개수 유지
    if (this.traces.length > MAX_TRACES) {
      this.traces = this.traces.slice(0, MAX_TRACES);
    }
    
    await this.saveTraces();
  }
  
  getRecentTraces(limit: number = 10): UserTrace[] {
    this.cleanupOldTraces();
    
    return this.traces
      .sort((a, b) => b.timestamp - a.timestamp)
      .slice(0, limit);
  }
  
  getTraceAge(timestamp: number): string {
    const now = Date.now();
    const diff = now - timestamp;
    const minutes = Math.floor(diff / 60000);
    
    if (minutes < 1) return '방금';
    if (minutes < 60) return `${minutes}분 전`;
    
    const hours = Math.floor(minutes / 60);
    if (hours < 24) return `${hours}시간 전`;
    
    return '오래 전';
  }
  
  // 주기적으로 가상 흔적 추가 (테스트용)
  addRandomVirtualTrace() {
    if (Math.random() < 0.3) { // 30% 확률
      this.addTrace(
        this.generateVirtualName(),
        this.virtualTraces[Math.floor(Math.random() * this.virtualTraces.length)]
      );
    }
  }
}

export const tracesManager = new TracesManager();
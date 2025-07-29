// 타임캡슐 메시지 시스템
// 사용자와 가상 유저들의 메시지를 관리하고 타임라인 형태로 제공

import AsyncStorage from '@react-native-async-storage/async-storage';
import { getRandomVirtualMessage, getTimeBasedMessage } from '@/constants/virtualMessages';
import { virtualUsersManager } from './virtualUsers';

const STORAGE_KEY = '@time_capsule_messages';
const MESSAGE_LIFETIME = 2 * 60 * 60 * 1000; // 2시간 (밀리초)
const MAX_MESSAGES = 50; // 최대 보관 메시지 수

export interface TimeCapsuleMessage {
  id: string;
  content: string;
  authorName: string;
  isVirtual: boolean; // 가상 유저 메시지 여부
  timestamp: number;
  lifeRemaining: number; // 0-1 사이 값 (페이드아웃용)
}

class TimeCapsuleManager {
  private messages: TimeCapsuleMessage[] = [];
  private updateInterval: NodeJS.Timeout | null = null;
  private virtualMessageInterval: NodeJS.Timeout | null = null;

  constructor() {
    this.loadMessages();
  }

  private async loadMessages() {
    try {
      const stored = await AsyncStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored) as TimeCapsuleMessage[];
        // 유효한 메시지만 필터링 (2시간 이내)
        const now = Date.now();
        this.messages = parsed.filter(msg => {
          const age = now - msg.timestamp;
          return age < MESSAGE_LIFETIME;
        });
        this.updateLifeRemaining();
        await this.saveMessages();
      }
    } catch (error) {
      console.error('Failed to load time capsule messages:', error);
      this.messages = [];
    }
  }

  private async saveMessages() {
    try {
      await AsyncStorage.setItem(STORAGE_KEY, JSON.stringify(this.messages));
    } catch (error) {
      console.error('Failed to save time capsule messages:', error);
    }
  }

  // 사용자 메시지 추가
  async addUserMessage(content: string, authorName: string) {
    const message: TimeCapsuleMessage = {
      id: `user-${Date.now()}-${Math.random()}`,
      content: content.trim(),
      authorName,
      isVirtual: false,
      timestamp: Date.now(),
      lifeRemaining: 1,
    };

    this.messages.unshift(message); // 최신 메시지를 앞에 추가
    this.trimMessages();
    await this.saveMessages();
  }

  // 가상 유저 메시지 자동 생성 시작
  startVirtualMessages() {
    if (this.virtualMessageInterval) return;

    // 초기 가상 메시지 몇 개 추가
    this.addInitialVirtualMessages();

    // 주기적으로 가상 메시지 추가
    this.virtualMessageInterval = setInterval(() => {
      this.addVirtualMessage();
    }, this.getRandomInterval());
  }

  // 가상 유저 메시지 자동 생성 중지
  stopVirtualMessages() {
    if (this.virtualMessageInterval) {
      clearInterval(this.virtualMessageInterval);
      this.virtualMessageInterval = null;
    }
  }

  private addInitialVirtualMessages() {
    // 초기에 3-5개의 가상 메시지 추가
    const count = Math.floor(Math.random() * 3) + 3;
    for (let i = 0; i < count; i++) {
      // 과거 시간으로 설정 (최대 1시간 전)
      const pastTime = Date.now() - Math.random() * 60 * 60 * 1000;
      this.addVirtualMessage(pastTime);
    }
  }

  private async addVirtualMessage(timestamp?: number) {
    const activeUsers = virtualUsersManager.getActiveUsers();
    if (activeUsers.length === 0) return;

    // 활동 중인 가상 유저 중 랜덤 선택
    const user = activeUsers[Math.floor(Math.random() * activeUsers.length)];
    
    // 집중을 막 끝낸 유저만 메시지를 남김
    if (user.state !== 'resting' && !timestamp) return;

    const message: TimeCapsuleMessage = {
      id: `virtual-${Date.now()}-${Math.random()}`,
      content: getTimeBasedMessage(),
      authorName: user.name,
      isVirtual: true,
      timestamp: timestamp || Date.now(),
      lifeRemaining: 1,
    };

    this.messages.unshift(message);
    this.trimMessages();
    await this.saveMessages();
  }

  private getRandomInterval(): number {
    // 30초 ~ 3분 사이의 랜덤 간격
    return 30000 + Math.random() * 150000;
  }

  private trimMessages() {
    // 최대 메시지 수 제한
    if (this.messages.length > MAX_MESSAGES) {
      this.messages = this.messages.slice(0, MAX_MESSAGES);
    }
  }

  // 메시지 수명 업데이트
  private updateLifeRemaining() {
    const now = Date.now();
    this.messages = this.messages.map(msg => {
      const age = now - msg.timestamp;
      const lifeRemaining = Math.max(0, 1 - (age / MESSAGE_LIFETIME));
      return { ...msg, lifeRemaining };
    }).filter(msg => msg.lifeRemaining > 0); // 수명이 다한 메시지 제거
  }

  // 실시간 업데이트 시작
  startRealtimeUpdate() {
    if (this.updateInterval) return;

    this.updateInterval = setInterval(() => {
      this.updateLifeRemaining();
      this.saveMessages();
    }, 10000); // 10초마다 업데이트
  }

  // 실시간 업데이트 중지
  stopRealtimeUpdate() {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }
  }

  // 현재 메시지 목록 가져오기
  getMessages(): TimeCapsuleMessage[] {
    this.updateLifeRemaining();
    return [...this.messages]; // 복사본 반환
  }

  // 메시지 포맷팅 헬퍼
  formatTimeAgo(timestamp: number): string {
    const now = Date.now();
    const diff = now - timestamp;
    const minutes = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);

    if (minutes < 1) return '방금 전';
    if (minutes < 60) return `${minutes}분 전`;
    if (hours < 2) return '1시간 전';
    return `${hours}시간 전`;
  }
}

// 싱글톤 인스턴스
export const timeCapsuleManager = new TimeCapsuleManager();
// 연속 참여 인식 시스템
// 사용자의 접속 패턴을 분석하여 단골 여부를 판단

import AsyncStorage from '@react-native-async-storage/async-storage';

const STORAGE_KEY = '@attendance_records';
const MAX_RECORDS = 10; // 최근 10회 접속 기록 저장
const REGULAR_THRESHOLD = 3; // 3일 연속 비슷한 시간대 접속 시 단골로 인식
const TIME_TOLERANCE_HOURS = 1; // ±1시간 오차 허용

export interface AttendanceRecord {
  timestamp: number;
  hour: number; // 0-23
  date: string; // YYYY-MM-DD
}

export interface RegularPattern {
  isRegular: boolean;
  timeSlot?: string; // "morning", "afternoon", "evening", "night"
  consecutiveDays: number;
  preferredHour?: number;
  message?: string;
}

class AttendanceManager {
  private records: AttendanceRecord[] = [];

  constructor() {
    this.loadRecords();
  }

  private async loadRecords() {
    try {
      const stored = await AsyncStorage.getItem(STORAGE_KEY);
      if (stored) {
        this.records = JSON.parse(stored);
      }
    } catch (error) {
      console.error('Failed to load attendance records:', error);
      this.records = [];
    }
  }

  private async saveRecords() {
    try {
      await AsyncStorage.setItem(STORAGE_KEY, JSON.stringify(this.records));
    } catch (error) {
      console.error('Failed to save attendance records:', error);
    }
  }

  // 새로운 접속 기록 추가
  async recordAttendance() {
    const now = new Date();
    const record: AttendanceRecord = {
      timestamp: now.getTime(),
      hour: now.getHours(),
      date: now.toISOString().split('T')[0],
    };

    // 같은 날짜의 기록이 이미 있으면 업데이트
    const existingIndex = this.records.findIndex(r => r.date === record.date);
    if (existingIndex >= 0) {
      this.records[existingIndex] = record;
    } else {
      this.records.unshift(record); // 최신 기록을 앞에 추가
    }

    // 최대 기록 수 유지
    if (this.records.length > MAX_RECORDS) {
      this.records = this.records.slice(0, MAX_RECORDS);
    }

    await this.saveRecords();
  }

  // 연속 참여 패턴 분석
  analyzeRegularPattern(playerName: string): RegularPattern {
    if (this.records.length < REGULAR_THRESHOLD) {
      return { isRegular: false, consecutiveDays: 0 };
    }

    // 최근 기록부터 분석
    const recentRecords = this.records.slice(0, 7); // 최근 7일간의 기록
    
    // 현재 시간
    const currentHour = new Date().getHours();
    
    // 연속 일수 계산
    let consecutiveDays = 1; // 오늘 포함
    let commonHour = currentHour;
    
    // 어제부터 역순으로 확인
    const today = new Date();
    for (let i = 1; i < recentRecords.length && i < REGULAR_THRESHOLD + 1; i++) {
      const daysBefore = new Date(today);
      daysBefore.setDate(today.getDate() - i);
      const targetDate = daysBefore.toISOString().split('T')[0];
      
      const record = recentRecords.find(r => r.date === targetDate);
      if (record && Math.abs(record.hour - commonHour) <= TIME_TOLERANCE_HOURS) {
        consecutiveDays++;
      } else {
        break;
      }
    }

    // 단골 여부 판단
    const isRegular = consecutiveDays >= REGULAR_THRESHOLD;
    
    if (isRegular) {
      const timeSlot = this.getTimeSlot(commonHour);
      const message = this.getRegularMessage(playerName, timeSlot, consecutiveDays, commonHour);
      
      return {
        isRegular: true,
        timeSlot,
        consecutiveDays,
        preferredHour: commonHour,
        message,
      };
    }

    return { isRegular: false, consecutiveDays };
  }

  // 시간대 분류
  private getTimeSlot(hour: number): string {
    if (hour >= 5 && hour < 12) return 'morning';
    if (hour >= 12 && hour < 17) return 'afternoon';
    if (hour >= 17 && hour < 22) return 'evening';
    return 'night';
  }

  // 단골 메시지 생성
  private getRegularMessage(playerName: string, timeSlot: string, days: number, hour: number): string {
    const timeMessages: Record<string, string[]> = {
      morning: [
        `${playerName}님! 어제 이 시간에도 뵙지 않았나요? 아침형 인간이시군요!`,
        `매일 아침 ${playerName}님을 만나니 하루가 기대돼요!`,
        `${days}일째 아침 인사! ${playerName}님 덕분에 아침이 행복해요!`,
      ],
      afternoon: [
        `${playerName}님! 오늘도 이 시간에 오셨네요? 정말 반가워요!`,
        `점심 시간마다 ${playerName}님을 만나는 게 일상이 됐어요!`,
        `${days}일 연속 오후 만남! ${playerName}님이 계셔서 든든해요!`,
      ],
      evening: [
        `${playerName}님! 저녁 시간의 단골손님이시네요! 오늘도 함께해요!`,
        `매일 저녁 ${playerName}님과 함께하니 외롭지 않아요!`,
        `${days}일째 저녁 약속! ${playerName}님 최고예요!`,
      ],
      night: [
        `${playerName}님! 어제 밤에도 뵀던 것 같은데... 오늘도 함께해주셔서 기뻐요!`,
        `밤마다 ${playerName}님을 만나니 마음이 따뜻해져요!`,
        `${days}일 연속 밤샘 동료! ${playerName}님과 함께라 든든해요!`,
      ],
    };

    const messages = timeMessages[timeSlot] || timeMessages.night;
    return messages[Math.floor(Math.random() * messages.length)];
  }

  // 접속 기록 요약 (디버깅/통계용)
  getAttendanceSummary(): {
    totalDays: number;
    mostFrequentHour: number | null;
    currentStreak: number;
  } {
    if (this.records.length === 0) {
      return { totalDays: 0, mostFrequentHour: null, currentStreak: 0 };
    }

    // 총 접속 일수
    const uniqueDates = new Set(this.records.map(r => r.date));
    const totalDays = uniqueDates.size;

    // 가장 자주 접속하는 시간대
    const hourCounts: Record<number, number> = {};
    this.records.forEach(r => {
      hourCounts[r.hour] = (hourCounts[r.hour] || 0) + 1;
    });
    
    let mostFrequentHour = null;
    let maxCount = 0;
    Object.entries(hourCounts).forEach(([hour, count]) => {
      if (count > maxCount) {
        maxCount = count;
        mostFrequentHour = parseInt(hour);
      }
    });

    // 현재 연속 접속 일수
    const pattern = this.analyzeRegularPattern('');
    const currentStreak = pattern.consecutiveDays;

    return { totalDays, mostFrequentHour, currentStreak };
  }
}

// 싱글톤 인스턴스
export const attendanceManager = new AttendanceManager();
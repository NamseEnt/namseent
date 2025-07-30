// 세션 타이밍 관리 (단순화된 버전)

interface SessionTiming {
  nextSessionTime: Date;
  timeUntilNext: number; // seconds
  isSessionTime: boolean; // 현재가 세션 시간인지
}

class SessionManager {
  private sessionInterval: number = 2 * 60 * 1000; // 2분 간격
  private sessionDuration: number = 90; // 90초 세션

  // 다음 세션 시작 시간 계산
  getNextSessionTime(): Date {
    const now = new Date();
    const interval = this.sessionInterval;
    
    // 현재 시간을 기준으로 다음 2분 단위 시간 계산
    const minutes = now.getMinutes();
    const seconds = now.getSeconds();
    const milliseconds = now.getMilliseconds();
    
    const minutesUntilNext = 2 - (minutes % 2);
    const secondsUntilNext = minutesUntilNext * 60 - seconds;
    const millisecondsUntilNext = secondsUntilNext * 1000 - milliseconds;
    
    return new Date(now.getTime() + millisecondsUntilNext);
  }

  // 세션 타이밍 정보
  getSessionTiming(): SessionTiming {
    const now = new Date();
    const nextSessionTime = this.getNextSessionTime();
    const timeUntilNext = Math.floor((nextSessionTime.getTime() - now.getTime()) / 1000);
    
    // 세션이 막 시작했는지 확인 (다음 세션까지 2분 - 세션시간 이상 남았으면)
    const isSessionTime = timeUntilNext > (this.sessionInterval / 1000 - this.sessionDuration);
    
    return {
      nextSessionTime,
      timeUntilNext,
      isSessionTime,
    };
  }

  getSessionDuration(): number {
    return this.sessionDuration;
  }

  // 가상 참여자 수 (시간대별)
  getVirtualParticipants(): number {
    const hour = new Date().getHours();
    
    if (hour >= 20 && hour <= 23) return 12 + Math.floor(Math.random() * 5);
    if (hour >= 9 && hour <= 11) return 8 + Math.floor(Math.random() * 4);
    if (hour >= 0 && hour <= 5) return 2 + Math.floor(Math.random() * 3);
    
    return 5 + Math.floor(Math.random() * 5);
  }
}

export const sessionManager = new SessionManager();
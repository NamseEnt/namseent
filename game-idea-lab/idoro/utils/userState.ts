// 사용자 세션 참여 상태 관리
import AsyncStorage from '@react-native-async-storage/async-storage';

export type UserSessionState = 'idle' | 'waiting' | 'in-session' | 'resting';

const USER_STATE_KEY = 'user_session_state';
const SESSION_START_TIME_KEY = 'session_start_time';
const REST_START_TIME_KEY = 'rest_start_time';

class UserStateManager {
  async getState(): Promise<UserSessionState> {
    try {
      const state = await AsyncStorage.getItem(USER_STATE_KEY);
      return (state as UserSessionState) || 'idle';
    } catch {
      return 'idle';
    }
  }

  async setState(state: UserSessionState): Promise<void> {
    await AsyncStorage.setItem(USER_STATE_KEY, state);
  }

  async setSessionStartTime(time: number): Promise<void> {
    await AsyncStorage.setItem(SESSION_START_TIME_KEY, time.toString());
  }

  async getSessionStartTime(): Promise<number | null> {
    try {
      const time = await AsyncStorage.getItem(SESSION_START_TIME_KEY);
      return time ? parseInt(time) : null;
    } catch {
      return null;
    }
  }

  async setRestStartTime(time: number): Promise<void> {
    await AsyncStorage.setItem(REST_START_TIME_KEY, time.toString());
  }

  async getRestStartTime(): Promise<number | null> {
    try {
      const time = await AsyncStorage.getItem(REST_START_TIME_KEY);
      return time ? parseInt(time) : null;
    } catch {
      return null;
    }
  }

  async enterWaitingRoom(): Promise<void> {
    await this.setState('waiting');
  }

  async startSession(): Promise<void> {
    await this.setState('in-session');
    await this.setSessionStartTime(Date.now());
  }

  async completeSession(): Promise<void> {
    await this.setState('resting');
    await this.setRestStartTime(Date.now());
  }

  async exitSession(): Promise<void> {
    await this.setState('idle');
    await AsyncStorage.removeItem(SESSION_START_TIME_KEY);
    await AsyncStorage.removeItem(REST_START_TIME_KEY);
  }

  async checkAndUpdateState(): Promise<UserSessionState> {
    const state = await this.getState();
    
    if (state === 'resting') {
      const restStartTime = await this.getRestStartTime();
      if (restStartTime) {
        const elapsed = Date.now() - restStartTime;
        // 5분 휴식 시간이 지나면 idle로
        if (elapsed > 5 * 60 * 1000) {
          await this.exitSession();
          return 'idle';
        }
      }
    }
    
    return state;
  }
}

export const userStateManager = new UserStateManager();
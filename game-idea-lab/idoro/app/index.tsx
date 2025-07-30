import React, { useState, useEffect } from 'react';
import { StyleSheet, View, Text, TouchableOpacity, SafeAreaView } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useRouter } from 'expo-router';
import { useThemeColor } from '@/hooks/useThemeColor';
import NameInputModal from '@/components/NameInputModal';
import IdolCharacter from '@/components/IdolCharacter';
import { getPlayerName, setPlayerName, getCheerPower } from '@/utils/storage';
import { getCurrentTimeTheme } from '@/utils/timeOfDay';
import { sessionManager } from '@/utils/sessionManager';
import { userStateManager } from '@/utils/userState';
import type { UserSessionState } from '@/utils/userState';
import { tracesManager } from '@/utils/traces';
import type { UserTrace } from '@/utils/traces';

export default function HomeScreen() {
  const router = useRouter();
  const [playerName, setPlayerNameState] = useState<string | null>(null);
  const [cheerPower, setCheerPower] = useState<number>(0);
  const [showNameModal, setShowNameModal] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [userState, setUserState] = useState<UserSessionState>('idle');
  const [timeUntilNext, setTimeUntilNext] = useState<number>(0);
  const [recentTraces, setRecentTraces] = useState<UserTrace[]>([]);

  const backgroundColor = useThemeColor({}, 'background');
  const textColor = useThemeColor({}, 'text');
  const timeTheme = getCurrentTimeTheme();

  useEffect(() => {
    loadUserData();
    
    // 사용자 상태 및 세션 정보 업데이트
    const updateInfo = async () => {
      // 사용자 상태 확인
      const currentUserState = await userStateManager.checkAndUpdateState();
      setUserState(currentUserState);
      
      // 세션 타이밍 정보
      const timing = sessionManager.getSessionTiming();
      setTimeUntilNext(timing.timeUntilNext);
    };
    
    updateInfo();
    const interval = setInterval(updateInfo, 1000); // 1초마다 업데이트
    
    // 흔적 업데이트
    const updateTraces = () => {
      setRecentTraces(tracesManager.getRecentTraces(5));
      // 가상 흔적 추가 (테스트용)
      tracesManager.addRandomVirtualTrace();
    };
    
    updateTraces();
    const tracesInterval = setInterval(updateTraces, 10000); // 10초마다
    
    
    return () => {
      clearInterval(interval);
      clearInterval(tracesInterval);
    };
  }, [playerName, router]);

  const loadUserData = async () => {
    const savedName = await getPlayerName();
    const savedPower = await getCheerPower();
    
    if (savedName) {
      setPlayerNameState(savedName);
      
    } else {
      setShowNameModal(true);
    }
    
    setCheerPower(savedPower);
    setIsLoading(false);
  };

  const handleNameSubmit = async (name: string) => {
    const success = await setPlayerName(name);
    if (success) {
      setPlayerNameState(name);
      setShowNameModal(false);
    }
  };

  const formatTimeUntil = (seconds: number): string => {
    if (seconds <= 0) return '';
    
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    if (hours > 0) {
      return `${hours}시간 ${minutes}분`;
    } else if (minutes > 0) {
      return `${minutes}분 ${secs}초`;
    } else {
      return `${secs}초`;
    }
  };

  if (isLoading) {
    return (
      <SafeAreaView style={[styles.container, { backgroundColor }]}>
        <View style={styles.loadingContainer}>
          <Text style={[styles.loadingText, { color: textColor }]}>로딩 중...</Text>
        </View>
      </SafeAreaView>
    );
  }

  return (
    <LinearGradient
      colors={timeTheme.gradientColors}
      style={styles.container}
    >
      <SafeAreaView style={styles.safeArea}>
        <View style={styles.content}>
          <View style={styles.header}>
            <View style={styles.leftHeader}>
              {playerName && (
                <Text style={styles.playerName}>{playerName}님</Text>
              )}
            </View>
            <View style={styles.rightHeader}>
              <View style={styles.cheerPowerContainer}>
                <Text style={styles.cheerPowerLabel}>응원력</Text>
                <View style={styles.cheerPowerValueContainer}>
                  <Text style={styles.cheerPowerValue}>{cheerPower}</Text>
                  <Text style={styles.cheerPowerUnit}>점</Text>
                </View>
              </View>
            </View>
          </View>


          <View style={styles.centerContent}>
            <IdolCharacter state="idle" />
            
            {/* 사용자 상태별 UI */}
            <View style={styles.sessionInfoContainer}>
              {userState === 'idle' && (
                <View style={styles.startingSoonContainer}>
                  <Text style={styles.startingSoonTitle}>다음 세션</Text>
                  <Text style={styles.timeUntilText}>{formatTimeUntil(timeUntilNext)}</Text>
                  <TouchableOpacity
                    style={styles.enterWaitingRoomButton}
                    onPress={async () => {
                      await userStateManager.enterWaitingRoom();
                      router.push('/waiting-room');
                    }}
                    activeOpacity={0.8}
                  >
                    <LinearGradient
                      colors={['#5BA3F5', '#4A90E2']}
                      style={styles.enterButtonGradient}
                    >
                      <Text style={styles.enterButtonText}>대기실 입장</Text>
                    </LinearGradient>
                  </TouchableOpacity>
                </View>
              )}
              
              {userState === 'waiting' && (
                <View style={styles.inProgressContainer}>
                  <Text style={styles.inProgressTitle}>대기실에서 대기 중</Text>
                  <TouchableOpacity
                    onPress={() => router.push('/waiting-room')}
                    style={styles.linkButton}
                  >
                    <Text style={styles.linkText}>대기실로 이동</Text>
                  </TouchableOpacity>
                </View>
              )}
              
              {userState === 'in-session' && (
                <View style={styles.inProgressContainer}>
                  <Text style={styles.inProgressTitle}>세션 참여 중</Text>
                  <TouchableOpacity
                    onPress={() => router.push('/focus-session')}
                    style={styles.linkButton}
                  >
                    <Text style={styles.linkText}>세션으로 이동</Text>
                  </TouchableOpacity>
                </View>
              )}
              
              {userState === 'resting' && (
                <View style={styles.inProgressContainer}>
                  <Text style={styles.inProgressTitle}>휴식 중</Text>
                  <TouchableOpacity
                    onPress={() => router.push('/session-result')}
                    style={styles.linkButton}
                  >
                    <Text style={styles.linkText}>결과 화면으로</Text>
                  </TouchableOpacity>
                </View>
              )}
            </View>
          </View>


          {/* 흔적 표시 영역 */}
          <View style={styles.tracesContainer}>
            <Text style={styles.tracesTitle}>최근 활동</Text>
            {recentTraces.length > 0 ? (
              recentTraces.map((trace) => (
                <View 
                  key={trace.id}
                  style={styles.traceItem}
                >
                  <Text style={styles.traceName}>{trace.name}</Text>
                  <Text style={styles.traceMessage}>{trace.message}</Text>
                  <Text style={styles.traceTime}>{tracesManager.getTraceAge(trace.timestamp)}</Text>
                </View>
              ))
            ) : (
              <Text style={styles.noTracesText}>활동 없음</Text>
            )}
          </View>
        </View>

        <NameInputModal
          visible={showNameModal}
          onSubmit={handleNameSubmit}
        />
      </SafeAreaView>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  safeArea: {
    flex: 1,
  },
  content: {
    flex: 1,
    justifyContent: 'space-between',
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  loadingText: {
    fontSize: 16,
  },
  header: {
    paddingVertical: 16,
    paddingHorizontal: 20,
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
  },
  leftHeader: {
    flex: 1,
  },
  rightHeader: {
    alignItems: 'flex-end',
  },
  playerName: {
    fontSize: 16,
    color: '#333',
    fontWeight: '600',
  },
  activeUsersText: {
    fontSize: 14,
    color: '#4A90E2',
    marginBottom: 8,
    fontWeight: '600',
  },
  cheerPowerContainer: {
    alignItems: 'flex-end',
  },
  cheerPowerLabel: {
    fontSize: 14,
    color: '#666',
    marginBottom: 4,
  },
  cheerPowerValueContainer: {
    flexDirection: 'row',
    alignItems: 'baseline',
  },
  cheerPowerValue: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#FF6B6B',
  },
  cheerPowerUnit: {
    fontSize: 16,
    color: '#666',
    marginLeft: 4,
  },
  centerContent: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 20,
  },
  sessionInfoContainer: {
    alignItems: 'center',
    marginTop: 30,
  },
  nextSessionLabel: {
    fontSize: 16,
    color: '#666',
    marginBottom: 8,
  },
  timeUntilText: {
    fontSize: 36,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 8,
  },
  sessionTimeText: {
    fontSize: 18,
    color: '#4A90E2',
    fontWeight: '600',
  },
  startingSoonContainer: {
    alignItems: 'center',
    backgroundColor: 'rgba(91, 163, 245, 0.1)',
    paddingHorizontal: 40,
    paddingVertical: 30,
    borderRadius: 20,
  },
  startingSoonTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#4A90E2',
    marginBottom: 12,
  },
  waitingCountText: {
    fontSize: 18,
    color: '#666',
    marginBottom: 8,
  },
  hayeonReadyText: {
    fontSize: 14,
    color: '#9370DB',
    marginBottom: 16,
    fontStyle: 'italic',
  },
  countdownText: {
    fontSize: 48,
    fontWeight: 'bold',
    color: '#FF6B6B',
  },
  enterWaitingRoomButton: {
    marginTop: 20,
    borderRadius: 25,
    overflow: 'hidden',
  },
  enterButtonGradient: {
    paddingVertical: 14,
    paddingHorizontal: 40,
    alignItems: 'center',
  },
  enterButtonText: {
    color: 'white',
    fontSize: 18,
    fontWeight: 'bold',
  },
  inProgressContainer: {
    alignItems: 'center',
    paddingVertical: 30,
  },
  inProgressTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#666',
    marginBottom: 12,
  },
  inProgressSubtext: {
    fontSize: 16,
    color: '#999',
  },
  tracesContainer: {
    paddingHorizontal: 20,
    paddingBottom: 20,
    maxHeight: 200,
  },
  tracesTitle: {
    fontSize: 16,
    color: '#666',
    marginBottom: 12,
    fontWeight: '600',
  },
  traceItem: {
    backgroundColor: 'rgba(255, 255, 255, 0.8)',
    padding: 12,
    borderRadius: 10,
    marginBottom: 8,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  traceName: {
    fontSize: 14,
    color: '#4A90E2',
    fontWeight: '600',
    marginRight: 8,
  },
  traceMessage: {
    fontSize: 14,
    color: '#333',
    flex: 1,
  },
  traceTime: {
    fontSize: 12,
    color: '#999',
    marginLeft: 8,
  },
  noTracesText: {
    fontSize: 14,
    color: '#999',
    textAlign: 'center',
    marginTop: 20,
  },
  linkButton: {
    marginTop: 12,
    padding: 8,
  },
  linkText: {
    fontSize: 16,
    color: '#4A90E2',
    textDecorationLine: 'underline',
  },
});

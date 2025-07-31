import React, { useState, useEffect } from 'react';
import {
  StyleSheet,
  View,
  Text,
  TouchableOpacity,
  SafeAreaView,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useRouter } from 'expo-router';
import IdolCharacter from '@/components/IdolCharacter';
import { sessionManager } from '@/utils/sessionManager';
import { userStateManager } from '@/utils/userState';
import { getTimeAdjustedColors } from '@/utils/timeOfDay';

export default function WaitingRoomScreen() {
  const router = useRouter();
  const [timeUntilNext, setTimeUntilNext] = useState<number>(0);
  const [participantCount, setParticipantCount] = useState<number>(0);
  const [initialParticipants] = useState<number>(() => sessionManager.getVirtualParticipants());
  const [isSessionStarting, setIsSessionStarting] = useState(false);
  const colors = getTimeAdjustedColors();

  useEffect(() => {
    const updateSessionInfo = async () => {
      const timing = sessionManager.getSessionTiming();
      setTimeUntilNext(timing.timeUntilNext);
      
      // 참여자 수 점진적 증가 (초기값에서 시작해서 세션 시작할수록 증가)
      const totalWaitTime = 120; // 2분
      const elapsedTime = totalWaitTime - timing.timeUntilNext;
      const progress = Math.min(elapsedTime / totalWaitTime, 1);
      const additionalParticipants = Math.floor(progress * 5); // 최대 5명 추가
      setParticipantCount(initialParticipants + additionalParticipants);
      
      // 다음 세션 시작 시간이 되면 (timeUntilNext가 0이 되면)
      if (timing.timeUntilNext === 0 && !isSessionStarting) {
        setIsSessionStarting(true);
        await userStateManager.startSession();
        router.replace('/focus-session');
      }
    };
    
    updateSessionInfo();
    const interval = setInterval(updateSessionInfo, 100); // 0.1초마다 업데이트
    
    return () => clearInterval(interval);
  }, [router]);

  const handleLeave = async () => {
    await userStateManager.exitSession();
    router.back();
  };

  const formatTime = (seconds: number): string => {
    if (seconds <= 0) return '0';
    
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    
    if (mins > 0) {
      return `${mins}:${secs.toString().padStart(2, '0')}`;
    }
    return `${secs}`;
  };

  return (
    <LinearGradient
      colors={[colors.accent, colors.primary + '40']}
      style={styles.container}
    >
      <SafeAreaView style={styles.safeArea}>
        <View style={styles.content}>
          <View style={styles.header}>
            <TouchableOpacity onPress={handleLeave} style={styles.leaveButton}>
              <Text style={styles.leaveText}>나가기</Text>
            </TouchableOpacity>
          </View>

          <View style={styles.mainContent}>
            <IdolCharacter state="idle" />
            
            <View style={styles.infoContainer}>
              <Text style={styles.title}>세션 대기실</Text>
              
              <View style={styles.timerContainer}>
                <Text style={styles.timerLabel}>시작까지</Text>
                <Text style={styles.timer}>{formatTime(timeUntilNext)}</Text>
              </View>
              
              <View style={styles.participantsContainer}>
                <Text style={styles.participantsText}>
                  현재 {participantCount}명 대기 중
                </Text>
              </View>
              
              {timeUntilNext <= 3 && timeUntilNext > 0 && (
                <Text style={styles.readyText}>
                  시작
                </Text>
              )}
            </View>
          </View>
        </View>
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
  },
  header: {
    paddingHorizontal: 20,
    paddingTop: 20,
    alignItems: 'flex-start',
  },
  leaveButton: {
    padding: 10,
  },
  leaveText: {
    fontSize: 16,
    color: '#666',
  },
  mainContent: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 20,
  },
  infoContainer: {
    alignItems: 'center',
    marginTop: 30,
    backgroundColor: 'rgba(255, 255, 255, 0.9)',
    paddingHorizontal: 40,
    paddingVertical: 30,
    borderRadius: 20,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#4A90E2',
    marginBottom: 20,
  },
  timerContainer: {
    alignItems: 'center',
    marginBottom: 20,
  },
  timerLabel: {
    fontSize: 14,
    color: '#666',
    marginBottom: 8,
  },
  timer: {
    fontSize: 48,
    fontWeight: 'bold',
    color: '#333',
  },
  participantsContainer: {
    alignItems: 'center',
  },
  participantsText: {
    fontSize: 18,
    color: '#666',
    marginBottom: 8,
  },
  subText: {
    fontSize: 14,
    color: '#9370DB',
    fontStyle: 'italic',
  },
  readyText: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#FF6B6B',
    marginTop: 20,
  },
});
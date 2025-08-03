import React, { useState, useEffect, useRef, useCallback } from 'react';
import {
  StyleSheet,
  View,
  Text,
  TouchableOpacity,
  SafeAreaView,
  Alert,
  AppState,
  AppStateStatus,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useRouter } from 'expo-router';
import IdolCharacter from '@/components/IdolCharacter';
import { sessionManager } from '@/utils/sessionManager';
import { userStateManager } from '@/utils/userState';
import { getCheerPower, setCheerPower } from '@/utils/storage';
import { GAME_CONFIG } from '@/constants/game';
import { getTimeAdjustedColors } from '@/utils/timeOfDay';

export default function FocusSessionScreen() {
  const router = useRouter();
  const [timeLeft, setTimeLeft] = useState(GAME_CONFIG.FOCUS_DURATION);
  const [cheerPower, setCheerPowerState] = useState(0);
  const [earnedPower, setEarnedPower] = useState(0);
  const [participantCount, setParticipantCount] = useState(0);
  const [startTime] = useState(Date.now());


  const handleSuccess = useCallback(async () => {
    const totalEarned = Math.floor((GAME_CONFIG.FOCUS_DURATION / 60) * GAME_CONFIG.CHEER_POWER_PER_MINUTE);
    const currentPower = await getCheerPower();
    const newTotal = currentPower + totalEarned;
    await setCheerPower(newTotal);
    await userStateManager.completeSession();
    
    router.replace({
      pathname: '/session-result',
      params: { 
        success: 'true',
        earnedPower: totalEarned.toString(),
        totalPower: newTotal.toString()
      }
    });
  }, [router]);


  const loadCheerPower = async () => {
    const power = await getCheerPower();
    setCheerPowerState(power);
  };


  useEffect(() => {
    loadCheerPower();
    setParticipantCount(sessionManager.getVirtualParticipants());

    const interval = setInterval(() => {
      const elapsed = Math.floor((Date.now() - startTime) / 1000);
      const remaining = Math.max(0, GAME_CONFIG.FOCUS_DURATION - elapsed);
      
      setTimeLeft(remaining);
      
      // Update earned power
      const minutesElapsed = elapsed / 60;
      const earned = Math.floor(minutesElapsed * GAME_CONFIG.CHEER_POWER_PER_MINUTE);
      setEarnedPower(earned);
      
      if (remaining === 0) {
        handleSuccess();
      }
    }, GAME_CONFIG.TIMER_UPDATE_INTERVAL);

    return () => clearInterval(interval);
  }, [startTime, handleSuccess]);

  const handleGiveUp = () => {
    Alert.alert(
      '세션 중단',
      '부분 보상을 받을 수 있습니다.',
      [
        { text: '취소', style: 'cancel' },
        { 
          text: '중단', 
          style: 'destructive',
          onPress: async () => {
            // Give partial reward
            const partialReward = Math.floor(earnedPower * GAME_CONFIG.PARTIAL_REWARD_RATIO);
            const currentPower = await getCheerPower();
            const newTotal = currentPower + partialReward;
            await setCheerPower(newTotal);
            await userStateManager.exitSession();
            
            router.replace({
              pathname: '/session-result',
              params: { 
                success: 'false',
                earnedPower: partialReward.toString(),
                totalPower: newTotal.toString()
              }
            });
          }
        }
      ]
    );
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const colors = getTimeAdjustedColors();

  return (
    <LinearGradient
      colors={[colors.accent, colors.primary + '40']}
      style={styles.container}
    >
      <SafeAreaView style={styles.safeArea}>
        <View style={styles.content}>
          <View style={styles.header}>
            <View style={styles.timerContainer}>
              <Text style={styles.timerLabel}>남은 시간</Text>
              <Text style={styles.timer}>{formatTime(timeLeft)}</Text>
            </View>
            <View style={styles.rightHeader}>
              {participantCount > 0 && (
                <View style={styles.focusingCountContainer}>
                  <Text style={styles.focusingCountText}>참여자: {participantCount}명</Text>
                </View>
              )}
              <View style={styles.cheerPowerContainer}>
                <Text style={styles.cheerPowerLabel}>응원력</Text>
                <View style={styles.cheerPowerValueContainer}>
                  <Text style={styles.cheerPowerValue}>{cheerPower + earnedPower}</Text>
                  {earnedPower > 0 && (
                    <Text style={styles.earnedPower}>+{earnedPower}</Text>
                  )}
                </View>
              </View>
            </View>
          </View>

          <View style={styles.mainContent}>
            <IdolCharacter state="focusing" />
            
          </View>

          <View style={styles.bottomContent}>
            <TouchableOpacity
              style={styles.giveUpButton}
              onPress={handleGiveUp}
              activeOpacity={0.8}
            >
              <Text style={styles.giveUpButtonText}>포기하기</Text>
            </TouchableOpacity>
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
    paddingHorizontal: 20,
  },
  header: {
    paddingTop: 20,
    paddingBottom: 15,
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    minHeight: 80,
  },
  timerContainer: {
    alignItems: 'center',
    flex: 1,
  },
  timerLabel: {
    fontSize: 14,
    color: '#666',
    marginBottom: 4,
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
  earnedPower: {
    fontSize: 18,
    color: '#4CAF50',
    marginLeft: 8,
    fontWeight: 'bold',
  },
  mainContent: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  rightHeader: {
    alignItems: 'flex-end',
  },
  focusingCountContainer: {
    marginBottom: 10,
  },
  focusingCountText: {
    fontSize: 14,
    color: '#4A90E2',
    fontWeight: '600',
  },
  timer: {
    fontSize: 36,
    fontWeight: 'bold',
    color: '#FF6B6B',
    textShadowColor: 'rgba(0, 0, 0, 0.1)',
    textShadowOffset: { width: 0, height: 2 },
    textShadowRadius: 4,
  },
  bottomContent: {
    paddingBottom: 20,
    paddingTop: 10,
  },
  giveUpButton: {
    backgroundColor: '#FF6B6B',
    paddingVertical: 15,
    borderRadius: 25,
    alignItems: 'center',
    opacity: 0.8,
  },
  giveUpButtonText: {
    color: 'white',
    fontSize: 16,
    fontWeight: 'bold',
  },
});
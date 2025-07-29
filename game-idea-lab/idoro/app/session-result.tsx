import React, { useState, useEffect, useRef, useCallback } from 'react';
import {
  StyleSheet,
  View,
  Text,
  TouchableOpacity,
  SafeAreaView,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useRouter, useLocalSearchParams } from 'expo-router';
import IdolCharacter from '@/components/IdolCharacter';
import TimeCapsuleMessageModal from '@/components/TimeCapsuleMessageModal';
import { getPlayerName } from '@/utils/storage';
import { GAME_CONFIG, getTimeAwareSuccessMessage, getTimeAwareFailureMessage } from '@/constants/game';
import { timeCapsuleManager } from '@/utils/timeCapsule';
import { getCurrentTimeTheme, getTimeAdjustedColors } from '@/utils/timeOfDay';
import type { SessionParams } from '@/types';

export default function SessionResultScreen() {
  const router = useRouter();
  const params = useLocalSearchParams<SessionParams>();
  const isSuccess = params.success === 'true';
  const earnedPower = params.earnedPower ? parseInt(params.earnedPower) : 0;
  const totalPower = params.totalPower ? parseInt(params.totalPower) : 0;
  
  const [timeLeft, setTimeLeft] = useState(GAME_CONFIG.REST_DURATION);
  const [playerName, setPlayerName] = useState<string>('');
  const [showMessageModal, setShowMessageModal] = useState(false);
  const intervalRef = useRef<NodeJS.Timeout>();
  const startTimeRef = useRef(Date.now());

  useEffect(() => {
    loadPlayerName();
    
    if (isSuccess) {
      startRestTimer();
      // 성공 시 메시지 모달 표시
      setShowMessageModal(true);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [isSuccess, startRestTimer]);

  const loadPlayerName = async () => {
    const name = await getPlayerName();
    setPlayerName(name || '');
  };

  const startRestTimer = useCallback(() => {
    startTimeRef.current = Date.now();
    
    intervalRef.current = setInterval(() => {
      const elapsed = Math.floor((Date.now() - startTimeRef.current) / 1000);
      const remaining = Math.max(0, GAME_CONFIG.REST_DURATION - elapsed);
      
      setTimeLeft(remaining);
      
      if (remaining === 0) {
        if (intervalRef.current) {
          clearInterval(intervalRef.current);
        }
        goToHome();
      }
    }, GAME_CONFIG.TIMER_UPDATE_INTERVAL);
  }, [goToHome]);

  const goToHome = useCallback(() => {
    router.replace('/');
  }, [router]);

  const handleMessageSubmit = async (message: string) => {
    await timeCapsuleManager.addUserMessage(message, playerName);
    setShowMessageModal(false);
  };

  const handleMessageSkip = () => {
    setShowMessageModal(false);
  };


  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const [successMessage] = useState(() => {
    return getTimeAwareSuccessMessage();
  });

  const getSuccessMessage = () => {
    return successMessage.replace('{name}', playerName);
  };

  const [failureMessage] = useState(() => {
    return getTimeAwareFailureMessage();
  });

  const getFailureMessage = () => {
    return failureMessage.replace('{name}', playerName);
  };

  const timeTheme = getCurrentTimeTheme();
  const colors = getTimeAdjustedColors();

  if (isSuccess) {
    return (
      <LinearGradient
        colors={[colors.accent, colors.secondary + '60']}
        style={styles.container}
      >
        <SafeAreaView style={styles.safeArea}>
          <View style={styles.content}>
            <View style={styles.successContainer}>
              <IdolCharacter state="resting" />
              
              <View style={styles.messageContainer}>
                <Text style={styles.successMessage}>
                  {getSuccessMessage()}
                </Text>
              </View>

              <View style={styles.rewardContainer}>
                <Text style={styles.rewardText}>+{earnedPower} 응원력</Text>
                <Text style={styles.totalText}>총 {totalPower} 응원력</Text>
              </View>

              <View style={styles.restContainer}>
                <Text style={styles.restLabel}>휴식 시간</Text>
                <Text style={styles.restTimer}>{formatTime(timeLeft)}</Text>
              </View>

            </View>
          </View>
        </SafeAreaView>
        
        <TimeCapsuleMessageModal
          visible={showMessageModal}
          onSubmit={handleMessageSubmit}
          onSkip={handleMessageSkip}
          playerName={playerName}
        />
      </LinearGradient>
    );
  }

  // Failure screen
  return (
    <LinearGradient
      colors={[colors.secondary + '40', colors.accent]}
      style={styles.container}
    >
      <SafeAreaView style={styles.safeArea}>
        <View style={styles.content}>
          <View style={styles.failureContainer}>
            <IdolCharacter state="idle" />
            
            <View style={styles.messageContainer}>
              <Text style={styles.failureMessage}>
                {getFailureMessage()}
              </Text>
              {earnedPower > 0 && (
                <View style={styles.partialRewardContainer}>
                  <Text style={styles.partialRewardLabel}>
                    작은 노력도 소중해요
                  </Text>
                  <Text style={styles.partialRewardText}>
                    +{earnedPower} 응원력 획득
                  </Text>
                </View>
              )}
            </View>

            <TouchableOpacity
              style={styles.confirmButton}
              onPress={goToHome}
              activeOpacity={0.8}
            >
              <LinearGradient
                colors={['#9370DB', '#8B7AB8']}
                style={styles.confirmButtonGradient}
              >
                <Text style={styles.confirmButtonText}>확인</Text>
              </LinearGradient>
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
  successContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  failureContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  messageContainer: {
    marginVertical: 30,
    paddingHorizontal: 20,
  },
  successMessage: {
    fontSize: 18,
    textAlign: 'center',
    color: '#D2691E',
    lineHeight: 26,
  },
  failureMessage: {
    fontSize: 18,
    textAlign: 'center',
    color: '#4B0082',
    lineHeight: 26,
  },
  rewardContainer: {
    alignItems: 'center',
    marginBottom: 30,
  },
  rewardText: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#FF6347',
    marginBottom: 8,
  },
  totalText: {
    fontSize: 18,
    color: '#8B4513',
  },
  restContainer: {
    alignItems: 'center',
    marginBottom: 30,
  },
  restLabel: {
    fontSize: 16,
    color: '#8B4513',
    marginBottom: 8,
  },
  restTimer: {
    fontSize: 36,
    fontWeight: 'bold',
    color: '#D2691E',
  },
  confirmButton: {
    borderRadius: 25,
    overflow: 'hidden',
    marginTop: 30,
  },
  confirmButtonGradient: {
    paddingVertical: 15,
    paddingHorizontal: 50,
  },
  confirmButtonText: {
    fontSize: 18,
    fontWeight: 'bold',
    color: 'white',
  },
  partialRewardContainer: {
    marginTop: 20,
    padding: 15,
    backgroundColor: 'rgba(255, 255, 255, 0.3)',
    borderRadius: 10,
    alignItems: 'center',
  },
  partialRewardLabel: {
    fontSize: 14,
    color: '#4B0082',
    marginBottom: 4,
  },
  partialRewardText: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#8B008B',
  },
});
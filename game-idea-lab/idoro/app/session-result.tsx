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
import { getPlayerName } from '@/utils/storage';

const REST_DURATION = 30; // 30 seconds for testing (originally 5 * 60)

export default function SessionResultScreen() {
  const router = useRouter();
  const params = useLocalSearchParams();
  const isSuccess = params.success === 'true';
  const earnedPower = params.earnedPower ? parseInt(params.earnedPower as string) : 0;
  const totalPower = params.totalPower ? parseInt(params.totalPower as string) : 0;
  
  const [timeLeft, setTimeLeft] = useState(REST_DURATION);
  const [playerName, setPlayerName] = useState<string>('');
  const intervalRef = useRef<NodeJS.Timeout>();

  useEffect(() => {
    loadPlayerName();
    
    if (isSuccess) {
      startRestTimer();
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
    intervalRef.current = setInterval(() => {
      setTimeLeft((prev) => {
        if (prev <= 1) {
          if (intervalRef.current) {
            clearInterval(intervalRef.current);
          }
          goToHome();
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  }, [goToHome]);

  const goToHome = useCallback(() => {
    router.replace('/');
  }, [router]);


  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const getSuccessMessage = () => {
    const messages = [
      `${playerName}님, 대단해요! 덕분에 저도 힘내서 연습했어요!`,
      `${playerName}님과 함께라면 뭐든 할 수 있을 것 같아요!`,
      `${playerName}님의 집중력, 정말 멋져요! 저도 배우고 싶어요.`,
      `${playerName}님, 오늘도 함께해줘서 고마워요!`,
    ];
    return messages[Math.floor(Math.random() * messages.length)];
  };

  const getFailureMessage = () => {
    const messages = [
      `${playerName}님, 괜찮아요. 쉬었다가 다시 해요!`,
      `${playerName}님도 힘드셨구나... 잠깐 쉬어요.`,
      `${playerName}님, 다음엔 더 잘할 수 있을 거예요!`,
      `${playerName}님의 건강이 더 중요해요. 무리하지 마세요.`,
    ];
    return messages[Math.floor(Math.random() * messages.length)];
  };

  if (isSuccess) {
    return (
      <LinearGradient
        colors={['#FFE4B5', '#FFB6C1']}
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
      </LinearGradient>
    );
  }

  // Failure screen
  return (
    <LinearGradient
      colors={['#E6E6FA', '#DDA0DD']}
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
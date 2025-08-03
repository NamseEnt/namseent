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
import { GAME_CONFIG } from '@/constants/game';
import { tracesManager } from '@/utils/traces';
import { userStateManager } from '@/utils/userState';
import { TextInput, KeyboardAvoidingView, Platform, Keyboard } from 'react-native';
import { getTimeAdjustedColors } from '@/utils/timeOfDay';

export default function SessionResultScreen() {
  const router = useRouter();
  const params = useLocalSearchParams();
  const isSuccess = params.success === 'true';
  const earnedPower = params.earnedPower ? parseInt(params.earnedPower as string) : 0;
  
  const [timeLeft, setTimeLeft] = useState<number>(GAME_CONFIG.REST_DURATION);
  const [playerName, setPlayerName] = useState<string>('');
  const [traceMessage, setTraceMessage] = useState<string>('');
  const [showTraceInput, setShowTraceInput] = useState(false);
  const intervalRef = useRef<number | null>(null);
  const startTimeRef = useRef<number>(Date.now());

  const goToHome = useCallback(async () => {
    await userStateManager.exitSession();
    router.replace('/');
  }, [router]);

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

  const loadPlayerName = async () => {
    const name = await getPlayerName();
    setPlayerName(name || '');
  };

  useEffect(() => {
    loadPlayerName();
    
    if (isSuccess) {
      startRestTimer();
      // 성공 시 흔적 입력 표시
      setShowTraceInput(true);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [isSuccess, startRestTimer]);


  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const colors = getTimeAdjustedColors();
  
  const handleTraceSubmit = async () => {
    if (traceMessage.trim()) {
      await tracesManager.addTrace(playerName, traceMessage.trim());
    }
    setShowTraceInput(false);
    Keyboard.dismiss();
  };
  
  const handleTraceSkip = () => {
    setShowTraceInput(false);
    Keyboard.dismiss();
  };

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
              
              <View style={styles.resultContainer}>
                <Text style={styles.completedText}>25:00 세션 완료</Text>
                <Text style={styles.rewardText}>응원력 +{earnedPower}</Text>
              </View>

              <View style={styles.restContainer}>
                <Text style={styles.restLabel}>휴식 시간</Text>
                <Text style={styles.restTimer}>{formatTime(timeLeft)}</Text>
              </View>
              
              {showTraceInput && (
                <KeyboardAvoidingView 
                  behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
                  style={styles.traceInputContainer}
                >
                  <Text style={styles.traceInputLabel}>메시지 남기기 (30자)</Text>
                  <TextInput
                    style={styles.traceInput}
                    value={traceMessage}
                    onChangeText={setTraceMessage}
                    placeholder="30자 이내"
                    maxLength={30}
                    autoFocus
                    onSubmitEditing={handleTraceSubmit}
                  />
                  <View style={styles.traceButtons}>
                    <TouchableOpacity 
                      style={styles.skipButton}
                      onPress={handleTraceSkip}
                    >
                      <Text style={styles.skipButtonText}>건너뛰기</Text>
                    </TouchableOpacity>
                    <TouchableOpacity 
                      style={[styles.submitButton, !traceMessage.trim() && styles.submitButtonDisabled]}
                      onPress={handleTraceSubmit}
                      disabled={!traceMessage.trim()}
                    >
                      <Text style={styles.submitButtonText}>남기기</Text>
                    </TouchableOpacity>
                  </View>
                </KeyboardAvoidingView>
              )}

            </View>
          </View>
        </SafeAreaView>
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
            
            <View style={styles.resultContainer}>
              <Text style={styles.failedText}>중단됨</Text>
              {earnedPower > 0 && (
                <Text style={styles.partialRewardText}>응원력 +{earnedPower}</Text>
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
  resultContainer: {
    alignItems: 'center',
    marginVertical: 30,
  },
  completedText: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 12,
  },
  failedText: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#666',
    marginBottom: 12,
  },
  rewardText: {
    fontSize: 20,
    color: '#4CAF50',
    fontWeight: '600',
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
  partialRewardText: {
    fontSize: 18,
    color: '#FF9500',
    fontWeight: '600',
  },
  traceInputContainer: {
    marginTop: 20,
    padding: 20,
    backgroundColor: 'rgba(255, 255, 255, 0.9)',
    borderRadius: 15,
    width: '100%',
  },
  traceInputLabel: {
    fontSize: 16,
    color: '#333',
    marginBottom: 10,
    textAlign: 'center',
  },
  traceInput: {
    borderWidth: 1,
    borderColor: '#ddd',
    borderRadius: 10,
    padding: 12,
    fontSize: 16,
    backgroundColor: 'white',
  },
  traceButtons: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginTop: 12,
  },
  skipButton: {
    padding: 10,
    flex: 1,
    marginRight: 8,
  },
  skipButtonText: {
    color: '#666',
    textAlign: 'center',
    fontSize: 16,
  },
  submitButton: {
    backgroundColor: '#4A90E2',
    padding: 12,
    borderRadius: 10,
    flex: 1,
    marginLeft: 8,
  },
  submitButtonDisabled: {
    backgroundColor: '#ccc',
  },
  submitButtonText: {
    color: 'white',
    textAlign: 'center',
    fontSize: 16,
    fontWeight: '600',
  },
});
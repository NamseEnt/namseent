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
import VirtualAvatars from '@/components/VirtualAvatars';
import { getCheerPower, setCheerPower } from '@/utils/storage';
import { GAME_CONFIG } from '@/constants/game';

export default function FocusSessionScreen() {
  const router = useRouter();
  const [timeLeft, setTimeLeft] = useState(GAME_CONFIG.FOCUS_DURATION);
  const [cheerPower, setCheerPowerState] = useState(0);
  const [earnedPower, setEarnedPower] = useState(0);
  const appStateRef = useRef(AppState.currentState);
  const startTimeRef = useRef(Date.now());
  const intervalRef = useRef<NodeJS.Timeout>();
  const cheerPowerRef = useRef(0);

  // Update ref when cheerPower changes
  useEffect(() => {
    cheerPowerRef.current = cheerPower;
  }, [cheerPower]);

  const handleSuccess = useCallback(async () => {
    const totalEarned = Math.floor((GAME_CONFIG.FOCUS_DURATION / 60) * GAME_CONFIG.CHEER_POWER_PER_MINUTE);
    const newTotal = cheerPowerRef.current + totalEarned;
    await setCheerPower(newTotal);
    
    router.replace({
      pathname: '/session-result',
      params: { 
        success: 'true',
        earnedPower: totalEarned.toString(),
        totalPower: newTotal.toString()
      }
    });
  }, [router]);

  const startTimer = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

    startTimeRef.current = Date.now();
    
    intervalRef.current = setInterval(() => {
      const elapsed = Math.floor((Date.now() - startTimeRef.current) / 1000);
      const remaining = Math.max(0, GAME_CONFIG.FOCUS_DURATION - elapsed);
      
      setTimeLeft(remaining);
      
      // Update earned power
      const minutesElapsed = elapsed / 60;
      const earned = Math.floor(minutesElapsed * GAME_CONFIG.CHEER_POWER_PER_MINUTE);
      setEarnedPower(earned);
      
      if (remaining === 0) {
        if (intervalRef.current) {
          clearInterval(intervalRef.current);
        }
        handleSuccess();
      }
    }, GAME_CONFIG.TIMER_UPDATE_INTERVAL);
  }, [handleSuccess]);

  const loadCheerPower = async () => {
    const power = await getCheerPower();
    setCheerPowerState(power);
  };

  const handleAppStateChange = useCallback((nextAppState: AppStateStatus) => {
    if (
      appStateRef.current.match(/inactive|background/) &&
      nextAppState === 'active'
    ) {
      // App has come to the foreground
      const elapsed = Math.floor((Date.now() - startTimeRef.current) / 1000);
      const newTimeLeft = Math.max(0, GAME_CONFIG.FOCUS_DURATION - elapsed);
      setTimeLeft(newTimeLeft);
      
      if (newTimeLeft > 0) {
        startTimer();
      } else {
        handleSuccess();
      }
    } else if (nextAppState === 'background') {
      // App has gone to the background
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    }
    appStateRef.current = nextAppState;
  }, [startTimer]);

  useEffect(() => {
    loadCheerPower();
    startTimer();

    const subscription = AppState.addEventListener('change', handleAppStateChange);
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
      subscription.remove();
    };
  }, [handleAppStateChange, startTimer]);

  const handleGiveUp = () => {
    Alert.alert(
      '정말 포기하시겠어요?',
      '지금까지의 노력도 일부 보상으로 받을 수 있어요.',
      [
        { text: '계속하기', style: 'cancel' },
        { 
          text: '포기하기', 
          style: 'destructive',
          onPress: async () => {
            if (intervalRef.current) {
              clearInterval(intervalRef.current);
            }
            
            // Give partial reward
            const partialReward = Math.floor(earnedPower * GAME_CONFIG.PARTIAL_REWARD_RATIO);
            const newTotal = cheerPowerRef.current + partialReward;
            await setCheerPower(newTotal);
            
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

  return (
    <LinearGradient
      colors={['#FFE4E1', '#E8F4FF']}
      style={styles.container}
    >
      <SafeAreaView style={styles.safeArea}>
        <View style={styles.content}>
          <View style={styles.header}>
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

          <View style={styles.centerContent}>
            <Text style={styles.timer}>{formatTime(timeLeft)}</Text>
            <IdolCharacter state="focusing" />
            <Text style={styles.encourageText}>함께 집중하고 있어요!</Text>
          </View>
          
          <VirtualAvatars />

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
    paddingBottom: 10,
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
  centerContent: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  timer: {
    fontSize: 48,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 30,
  },
  encourageText: {
    fontSize: 16,
    color: '#666',
    marginTop: 20,
  },
  bottomContent: {
    paddingBottom: 40,
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
import React, { useEffect, useState } from 'react';
import { View, Text, StyleSheet } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withSpring,
  withTiming,
  interpolate,
  runOnJS,
} from 'react-native-reanimated';
import { LinearGradient } from 'expo-linear-gradient';
import { collectiveGoalManager } from '@/utils/collectiveGoal';

interface CollectiveGoalProgressProps {
  playerName: string;
  onGoalAchieved?: () => void;
}

export default function CollectiveGoalProgress({ playerName, onGoalAchieved }: CollectiveGoalProgressProps) {
  const [progress, setProgress] = useState({
    currentMinutes: 0,
    goalMinutes: 1200,
    percentage: 0,
    isAchieved: false,
    message: '',
  });
  
  const progressValue = useSharedValue(0);
  const glowOpacity = useSharedValue(0);

  useEffect(() => {
    loadProgress();
    
    // 주기적으로 진행률 업데이트
    const interval = setInterval(loadProgress, 5000);
    
    // 가상 유저 기여도 시작
    collectiveGoalManager.startVirtualContribution();
    
    return () => {
      clearInterval(interval);
      collectiveGoalManager.stopVirtualContribution();
    };
  }, []);

  const loadProgress = async () => {
    const data = await collectiveGoalManager.getProgress();
    const message = collectiveGoalManager.getProgressMessage(data.percentage, playerName);
    
    setProgress({
      currentMinutes: data.currentMinutes,
      goalMinutes: data.goalMinutes,
      percentage: data.percentage,
      isAchieved: data.isAchieved,
      message,
    });
    
    // 애니메이션 업데이트
    progressValue.value = withSpring(data.percentage / 100, {
      damping: 15,
      stiffness: 100,
    });
    
    // 목표 달성 시 글로우 효과
    if (data.isAchieved && !progress.isAchieved) {
      glowOpacity.value = withTiming(1, { duration: 1000 }, () => {
        glowOpacity.value = withTiming(0.3, { duration: 1000 });
      });
      
      if (onGoalAchieved) {
        runOnJS(onGoalAchieved)();
      }
    }
  };

  const progressBarStyle = useAnimatedStyle(() => ({
    width: `${interpolate(progressValue.value, [0, 1], [0, 100])}%`,
  }));

  const glowStyle = useAnimatedStyle(() => ({
    opacity: glowOpacity.value,
  }));

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>오늘의 연습 목표</Text>
        <Text style={styles.progressText}>
          {collectiveGoalManager.formatTime(progress.currentMinutes)} / {collectiveGoalManager.formatTime(progress.goalMinutes)}
        </Text>
      </View>
      
      <View style={styles.progressBarContainer}>
        <View style={styles.progressBarBackground}>
          <Animated.View style={[styles.progressBarFill, progressBarStyle]}>
            <LinearGradient
              colors={progress.isAchieved ? ['#FFD700', '#FFA500'] : ['#4ECDC4', '#45B7D1']}
              start={{ x: 0, y: 0 }}
              end={{ x: 1, y: 0 }}
              style={styles.gradientFill}
            />
          </Animated.View>
          
          {progress.isAchieved && (
            <Animated.View style={[styles.glowEffect, glowStyle]} />
          )}
        </View>
        
        <View style={styles.percentageContainer}>
          <Text style={styles.percentageText}>{Math.floor(progress.percentage)}%</Text>
        </View>
      </View>
      
      <Text style={styles.message}>{progress.message}</Text>
      
      {progress.percentage > 0 && (
        <View style={styles.contributorsInfo}>
          <Text style={styles.contributorsText}>
            함께 노력하는 모든 팬들과 만들어가는 기록이에요! 💪
          </Text>
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    marginVertical: 20,
    paddingHorizontal: 20,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  title: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#333',
  },
  progressText: {
    fontSize: 14,
    color: '#666',
  },
  progressBarContainer: {
    position: 'relative',
    marginBottom: 12,
  },
  progressBarBackground: {
    height: 24,
    backgroundColor: '#E0E0E0',
    borderRadius: 12,
    overflow: 'hidden',
    position: 'relative',
  },
  progressBarFill: {
    height: '100%',
    borderRadius: 12,
    overflow: 'hidden',
  },
  gradientFill: {
    flex: 1,
  },
  glowEffect: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundColor: '#FFD700',
    borderRadius: 12,
  },
  percentageContainer: {
    position: 'absolute',
    right: 10,
    top: 0,
    bottom: 0,
    justifyContent: 'center',
  },
  percentageText: {
    fontSize: 12,
    fontWeight: 'bold',
    color: '#333',
  },
  message: {
    fontSize: 14,
    color: '#4A90E2',
    textAlign: 'center',
    marginBottom: 8,
    lineHeight: 20,
  },
  contributorsInfo: {
    alignItems: 'center',
  },
  contributorsText: {
    fontSize: 12,
    color: '#888',
    fontStyle: 'italic',
  },
});
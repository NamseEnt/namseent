import React, { useEffect } from 'react';
import { View, Text, StyleSheet, Dimensions, Modal } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withSpring,
  withSequence,
  withDelay,
  withTiming,
  interpolate,
  Extrapolate,
  runOnJS,
} from 'react-native-reanimated';
import { LinearGradient } from 'expo-linear-gradient';
import IdolCharacter from './IdolCharacter';

interface GoalAchievementCelebrationProps {
  visible: boolean;
  onClose: () => void;
  playerName: string;
}

const { width, height } = Dimensions.get('window');

export default function GoalAchievementCelebration({
  visible,
  onClose,
  playerName,
}: GoalAchievementCelebrationProps) {
  const scale = useSharedValue(0);
  const rotation = useSharedValue(0);
  const confettiProgress = useSharedValue(0);

  useEffect(() => {
    if (visible) {
      // 시작 애니메이션
      scale.value = withSequence(
        withSpring(1.2, { damping: 10, stiffness: 100 }),
        withSpring(1, { damping: 15, stiffness: 100 })
      );
      
      rotation.value = withSequence(
        withTiming(10, { duration: 200 }),
        withTiming(-10, { duration: 200 }),
        withTiming(5, { duration: 200 }),
        withTiming(-5, { duration: 200 }),
        withTiming(0, { duration: 200 })
      );
      
      confettiProgress.value = withTiming(1, { duration: 2000 });
      
      // 3초 후 자동으로 닫기
      const timer = setTimeout(() => {
        scale.value = withTiming(0, { duration: 300 }, () => {
          runOnJS(onClose)();
        });
      }, 3000);
      
      return () => clearTimeout(timer);
    }
  }, [visible, scale, rotation, confettiProgress, onClose]);

  const containerStyle = useAnimatedStyle(() => ({
    transform: [
      { scale: scale.value },
      { rotate: `${rotation.value}deg` },
    ],
  }));

  const renderConfetti = () => {
    const confettiElements = [];
    const colors = ['#FF6B6B', '#4ECDC4', '#45B7D1', '#FFD700', '#FF6347'];
    
    for (let i = 0; i < 20; i++) {
      const randomX = Math.random() * width;
      const randomDelay = Math.random() * 1000;
      const randomDuration = 2000 + Math.random() * 1000;
      const randomRotation = Math.random() * 360;
      const color = colors[i % colors.length];
      
      confettiElements.push(
        <ConfettiPiece
          key={i}
          x={randomX}
          delay={randomDelay}
          duration={randomDuration}
          rotation={randomRotation}
          color={color}
          progress={confettiProgress}
        />
      );
    }
    
    return confettiElements;
  };

  if (!visible) return null;

  return (
    <Modal transparent visible={visible} animationType="none">
      <View style={styles.overlay}>
        <Animated.View style={[styles.container, containerStyle]}>
          <LinearGradient
            colors={['#FFD700', '#FFA500', '#FF6347']}
            style={styles.gradient}
          >
            <Text style={styles.title}>🎉 목표 달성! 🎉</Text>
            
            <View style={styles.characterContainer}>
              <IdolCharacter state="success" />
            </View>
            
            <Text style={styles.message}>
              {playerName}님과 모든 팬분들이{'\n'}
              함께 만든 기적이에요!
            </Text>
            
            <Text style={styles.subMessage}>
              오늘도 모두 수고하셨어요! 💪
            </Text>
          </LinearGradient>
        </Animated.View>
        
        {renderConfetti()}
      </View>
    </Modal>
  );
}

interface ConfettiPieceProps {
  x: number;
  delay: number;
  duration: number;
  rotation: number;
  color: string;
  progress: Animated.SharedValue<number>;
}

function ConfettiPiece({ x, delay, duration, rotation, color, progress }: ConfettiPieceProps) {
  const animatedStyle = useAnimatedStyle(() => {
    const translateY = interpolate(
      progress.value,
      [0, 1],
      [-50, height + 50],
      Extrapolate.CLAMP
    );
    
    const opacity = interpolate(
      progress.value,
      [0, 0.1, 0.9, 1],
      [0, 1, 1, 0],
      Extrapolate.CLAMP
    );
    
    const rotate = interpolate(
      progress.value,
      [0, 1],
      [0, rotation],
      Extrapolate.CLAMP
    );
    
    return {
      transform: [
        { translateX: x },
        { translateY },
        { rotate: `${rotate}deg` },
      ],
      opacity,
    };
  });

  return (
    <Animated.View
      style={[
        styles.confetti,
        animatedStyle,
        { backgroundColor: color },
      ]}
    />
  );
}

const styles = StyleSheet.create({
  overlay: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.7)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  container: {
    width: width * 0.9,
    maxWidth: 400,
    borderRadius: 20,
    overflow: 'hidden',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 10,
    },
    shadowOpacity: 0.25,
    shadowRadius: 10,
    elevation: 10,
  },
  gradient: {
    padding: 30,
    alignItems: 'center',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: 'white',
    marginBottom: 20,
    textShadowColor: 'rgba(0, 0, 0, 0.3)',
    textShadowOffset: { width: 2, height: 2 },
    textShadowRadius: 4,
  },
  characterContainer: {
    marginVertical: 20,
  },
  message: {
    fontSize: 18,
    color: 'white',
    textAlign: 'center',
    marginBottom: 10,
    lineHeight: 26,
    fontWeight: '600',
  },
  subMessage: {
    fontSize: 16,
    color: 'white',
    textAlign: 'center',
    opacity: 0.9,
  },
  confetti: {
    position: 'absolute',
    width: 10,
    height: 10,
    borderRadius: 5,
  },
});
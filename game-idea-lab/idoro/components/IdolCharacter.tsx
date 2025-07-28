import React, { useEffect } from 'react';
import { View, Text, StyleSheet } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withRepeat,
  withTiming,
  withSequence,
  Easing,
} from 'react-native-reanimated';
import type { IdolState } from '@/types';

interface IdolCharacterProps {
  state: IdolState;
  playerName?: string;
}

export default function IdolCharacter({ state, playerName }: IdolCharacterProps) {
  const breathingScale = useSharedValue(1);
  const floatY = useSharedValue(0);
  const rotation = useSharedValue(0);

  useEffect(() => {
    if (state === 'idle') {
      breathingScale.value = withRepeat(
        withSequence(
          withTiming(1.02, { duration: 2000, easing: Easing.inOut(Easing.ease) }),
          withTiming(1, { duration: 2000, easing: Easing.inOut(Easing.ease) })
        ),
        -1
      );

      floatY.value = withRepeat(
        withSequence(
          withTiming(-5, { duration: 1500, easing: Easing.inOut(Easing.ease) }),
          withTiming(0, { duration: 1500, easing: Easing.inOut(Easing.ease) })
        ),
        -1
      );
      
      rotation.value = 0;
    } else if (state === 'focusing') {
      // Focused animation - smaller breathing, no floating, slight swaying
      breathingScale.value = withRepeat(
        withSequence(
          withTiming(1.01, { duration: 3000, easing: Easing.inOut(Easing.ease) }),
          withTiming(1, { duration: 3000, easing: Easing.inOut(Easing.ease) })
        ),
        -1
      );
      
      floatY.value = 0;
      
      rotation.value = withRepeat(
        withSequence(
          withTiming(2, { duration: 2000, easing: Easing.inOut(Easing.ease) }),
          withTiming(-2, { duration: 2000, easing: Easing.inOut(Easing.ease) })
        ),
        -1
      );
    } else if (state === 'resting') {
      // Resting animation - happy bouncing
      breathingScale.value = withRepeat(
        withSequence(
          withTiming(1.05, { duration: 1000, easing: Easing.inOut(Easing.ease) }),
          withTiming(1, { duration: 1000, easing: Easing.inOut(Easing.ease) })
        ),
        -1
      );
      
      floatY.value = withRepeat(
        withSequence(
          withTiming(-10, { duration: 500, easing: Easing.out(Easing.quad) }),
          withTiming(0, { duration: 500, easing: Easing.in(Easing.quad) })
        ),
        -1
      );
      
      rotation.value = 0;
    } else {
      breathingScale.value = 1;
      floatY.value = 0;
      rotation.value = 0;
    }
  }, [state, breathingScale, floatY, rotation]);

  const animatedStyle = useAnimatedStyle(() => {
    return {
      transform: [
        { scale: breathingScale.value },
        { translateY: floatY.value },
        { rotate: `${rotation.value}deg` }
      ],
    };
  });

  return (
    <View style={styles.container}>
      <Animated.View style={[styles.idolContainer, animatedStyle]}>
        <View style={styles.idolBody}>
          <View style={styles.idolHead}>
            <View style={styles.eye} />
            <View style={styles.eye} />
            <View style={styles.mouth} />
          </View>
          <Text style={styles.idolText}>♪</Text>
        </View>
      </Animated.View>
      {state === 'idle' && playerName && (
        <Text style={styles.idleMessage}>
          {playerName}님을 기다리고 있어요!
        </Text>
      )}
      {state === 'focusing' && (
        <Text style={styles.focusMessage}>
          열심히 집중하는 중...
        </Text>
      )}
      {state === 'resting' && (
        <Text style={styles.restMessage}>
          수고했어요! ✨
        </Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  idolContainer: {
    width: 200,
    height: 300,
    alignItems: 'center',
    justifyContent: 'center',
  },
  idolBody: {
    width: 150,
    height: 200,
    backgroundColor: '#FFE4E1',
    borderRadius: 75,
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 3.84,
    elevation: 5,
  },
  idolHead: {
    width: 100,
    height: 100,
    backgroundColor: '#FFF',
    borderRadius: 50,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: 20,
    flexDirection: 'row',
  },
  eye: {
    width: 10,
    height: 10,
    backgroundColor: '#333',
    borderRadius: 5,
    marginHorizontal: 15,
    marginTop: -10,
  },
  mouth: {
    width: 20,
    height: 10,
    backgroundColor: '#FF69B4',
    borderRadius: 10,
    position: 'absolute',
    bottom: 30,
  },
  idolText: {
    fontSize: 24,
    color: '#FF69B4',
  },
  idleMessage: {
    marginTop: 20,
    fontSize: 16,
    color: '#666',
    textAlign: 'center',
  },
  focusMessage: {
    marginTop: 20,
    fontSize: 16,
    color: '#4A90E2',
    textAlign: 'center',
    fontWeight: 'bold',
  },
  restMessage: {
    marginTop: 20,
    fontSize: 16,
    color: '#FF6347',
    textAlign: 'center',
    fontWeight: 'bold',
  },
});
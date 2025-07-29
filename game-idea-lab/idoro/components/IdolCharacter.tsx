import React, { useEffect } from 'react';
import { View, Text, StyleSheet, Image } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withRepeat,
  withTiming,
  withSequence,
  Easing,
  FadeInDown,
  FadeOutUp,
} from 'react-native-reanimated';
import type { IdolState } from '@/types';
import { CHARACTER_IMAGES } from '@/constants/characters';
import { getTimeBasedCharacterMessage } from '@/utils/timeOfDay';

const AnimatedImage = Animated.createAnimatedComponent(Image);

interface IdolCharacterProps {
  state: IdolState;
  playerName?: string;
  showTimelineReaction?: boolean;
  regularMessage?: string | null;
}

export default function IdolCharacter({ state, playerName, showTimelineReaction, regularMessage }: IdolCharacterProps) {
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

  const characterImage = CHARACTER_IMAGES.하연[state] || CHARACTER_IMAGES.하연.idle;

  return (
    <View style={styles.container}>
      <AnimatedImage 
        source={characterImage}
        style={[styles.characterImage, animatedStyle]}
        resizeMode="contain"
      />
      {state === 'idle' && playerName && !showTimelineReaction && !regularMessage && (
        <Text style={styles.idleMessage}>
          {getTimeBasedCharacterMessage('idle', playerName)}
        </Text>
      )}
      {state === 'idle' && regularMessage && (
        <Animated.Text 
          entering={FadeInDown.duration(500)}
          exiting={FadeOutUp.duration(500)}
          style={styles.regularMessage}
        >
          {regularMessage}
        </Animated.Text>
      )}
      {state === 'idle' && showTimelineReaction && (
        <Text style={styles.timelineReactionMessage}>
          와, 다들 정말 열심히네요! 논문 쓰시는 분, 저도 응원할게요!
        </Text>
      )}
      {state === 'focusing' && (
        <Text style={styles.focusMessage}>
          {getTimeBasedCharacterMessage('focusing', playerName || '')}
        </Text>
      )}
      {state === 'resting' && (
        <Text style={styles.restMessage}>
          {getTimeBasedCharacterMessage('resting', playerName || '')}
        </Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    justifyContent: 'center',
    minHeight: 400,
  },
  characterImage: {
    width: 300,
    height: 400,
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
  timelineReactionMessage: {
    marginTop: 20,
    fontSize: 14,
    color: '#9370DB',
    textAlign: 'center',
    fontStyle: 'italic',
    paddingHorizontal: 20,
    lineHeight: 20,
  },
  regularMessage: {
    marginTop: 20,
    fontSize: 16,
    color: '#FF1493',
    textAlign: 'center',
    fontWeight: 'bold',
    paddingHorizontal: 20,
    lineHeight: 24,
    backgroundColor: 'rgba(255, 255, 255, 0.9)',
    borderRadius: 20,
    padding: 12,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 3.84,
    elevation: 5,
  },
});
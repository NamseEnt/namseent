import React, { useEffect } from 'react';
import { View, StyleSheet, Image } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withRepeat,
  withTiming,
  withSequence,
  Easing,
} from 'react-native-reanimated';
import type { IdolState } from '@/types';
import { CHARACTER_IMAGES } from '@/constants/characters';

const AnimatedImage = Animated.createAnimatedComponent(Image);

interface IdolCharacterProps {
  state: IdolState;
}

export default function IdolCharacter({ state }: IdolCharacterProps) {
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
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  characterImage: {
    width: 240,
    height: 320,
  },
});
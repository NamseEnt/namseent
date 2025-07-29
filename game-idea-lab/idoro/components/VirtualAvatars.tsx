import React, { useEffect, useState } from 'react';
import { View, StyleSheet, Text } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withTiming,
  withSpring,
  withDelay,
  FadeIn,
  FadeOut,
  Layout,
} from 'react-native-reanimated';
import { virtualUsersManager, VirtualUser } from '@/utils/virtualUsers';

interface VirtualAvatarsProps {
  maxDisplay?: number;
}

interface Avatar {
  id: string;
  name: string;
  color: string;
}

const AVATAR_COLORS = [
  '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FECA57',
  '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE', '#85C1E2',
];

export default function VirtualAvatars({ maxDisplay = 8 }: VirtualAvatarsProps) {
  const [focusingUsers, setFocusingUsers] = useState<Avatar[]>([]);

  useEffect(() => {
    const updateAvatars = () => {
      const users = virtualUsersManager.getUsersByState('focusing');
      const avatars = users.slice(0, maxDisplay).map((user, index) => ({
        id: user.id,
        name: user.name,
        color: AVATAR_COLORS[index % AVATAR_COLORS.length],
      }));
      setFocusingUsers(avatars);
    };

    // 초기 설정
    updateAvatars();

    // 주기적 업데이트
    const interval = setInterval(updateAvatars, 2000);

    return () => clearInterval(interval);
  }, [maxDisplay]);

  return (
    <View style={styles.container}>
      <Text style={styles.title}>함께 집중하는 친구들</Text>
      <View style={styles.avatarsRow}>
        {focusingUsers.map((avatar, index) => (
          <AnimatedAvatar
            key={avatar.id}
            avatar={avatar}
            index={index}
          />
        ))}
      </View>
      {focusingUsers.length > 0 && (
        <Text style={styles.countText}>
          {virtualUsersManager.getFocusingUserCount()}명이 함께 집중 중
        </Text>
      )}
    </View>
  );
}

interface AnimatedAvatarProps {
  avatar: Avatar;
  index: number;
}

function AnimatedAvatar({ avatar, index }: AnimatedAvatarProps) {
  const scale = useSharedValue(0);
  const translateY = useSharedValue(0);

  useEffect(() => {
    scale.value = withDelay(
      index * 100,
      withSpring(1, {
        damping: 12,
        stiffness: 100,
      })
    );

    // 부드러운 떠다니는 효과
    translateY.value = withDelay(
      index * 100,
      withTiming(
        -5,
        {
          duration: 2000 + index * 200,
        },
        () => {
          translateY.value = withTiming(0, {
            duration: 2000 + index * 200,
          });
        }
      )
    );
  }, [index, scale, translateY]);

  const animatedStyle = useAnimatedStyle(() => ({
    transform: [
      { scale: scale.value },
      { translateY: translateY.value },
    ],
  }));

  return (
    <Animated.View
      entering={FadeIn.duration(300).delay(index * 100)}
      exiting={FadeOut.duration(300)}
      layout={Layout.springify()}
      style={[
        styles.avatarContainer,
        animatedStyle,
        { backgroundColor: avatar.color },
      ]}
    >
      <Text style={styles.avatarText}>
        {avatar.name.charAt(0).toUpperCase()}
      </Text>
    </Animated.View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    marginVertical: 20,
  },
  title: {
    fontSize: 14,
    color: '#666',
    marginBottom: 12,
  },
  avatarsRow: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'center',
    gap: 8,
    paddingHorizontal: 20,
  },
  avatarContainer: {
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 3.84,
    elevation: 5,
  },
  avatarText: {
    color: 'white',
    fontSize: 18,
    fontWeight: 'bold',
  },
  countText: {
    fontSize: 12,
    color: '#4A90E2',
    marginTop: 8,
  },
});
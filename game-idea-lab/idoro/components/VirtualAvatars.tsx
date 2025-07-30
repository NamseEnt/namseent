import React, { useEffect, useState } from 'react';
import { View, StyleSheet, Text, Dimensions } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withTiming,
  withSpring,
  withDelay,
  withRepeat,
  withSequence,
  FadeIn,
  FadeOut,
  Layout,
  SlideInDown,
  SlideOutUp,
  Easing,
} from 'react-native-reanimated';
import { virtualUsersManager, VirtualUser } from '@/utils/virtualUsers';

interface VirtualAvatarsProps {
  maxDisplay?: number;
}

interface Avatar {
  id: string;
  name: string;
  color: string;
  focusStartTime: number;
  isNew?: boolean;
}

const AVATAR_COLORS = [
  '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FECA57',
  '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE', '#85C1E2',
];

const getDeskPositions = (count: number) => {
  const positions = [
    { x: -80, y: -30 },
    { x: 0, y: -40 },
    { x: 80, y: -30 },
    { x: -60, y: 10 },
    { x: 60, y: 10 },
    { x: -100, y: -20 },
    { x: 0, y: 0 },
    { x: 100, y: -20 },
    { x: 0, y: 40 },
  ];
  return positions.slice(0, count);
};

export default function VirtualAvatars({ maxDisplay = 9 }: VirtualAvatarsProps) {
  const [focusingUsers, setFocusingUsers] = useState<Avatar[]>([]);
  const previousUsersRef = React.useRef<Set<string>>(new Set());
  const [screenHeight, setScreenHeight] = useState(Dimensions.get('window').height);
  
  useEffect(() => {
    const subscription = Dimensions.addEventListener('change', ({ window }) => {
      setScreenHeight(window.height);
    });
    return () => subscription?.remove();
  }, []);
  
  // 화면 크기에 따라 표시할 아바타 수 조정
  const getMaxDisplay = () => {
    if (screenHeight < 600) return 4;
    if (screenHeight < 700) return 6;
    return 9;
  };

  useEffect(() => {
    const updateAvatars = () => {
      const users = virtualUsersManager.getUsersByState('focusing');
      const currentUserIds = new Set(users.map(u => u.id));
      
      // 새로 들어온 사용자 감지
      const newUserIds = new Set<string>();
      currentUserIds.forEach(id => {
        if (!previousUsersRef.current.has(id)) {
          newUserIds.add(id);
        }
      });
      
      const dynamicMaxDisplay = getMaxDisplay();
      const avatars = users.slice(0, dynamicMaxDisplay).map((user, index) => ({
        id: user.id,
        name: user.name,
        color: AVATAR_COLORS[Math.floor(Math.random() * AVATAR_COLORS.length)],
        focusStartTime: user.focusStartTime || Date.now(),
        isNew: newUserIds.has(user.id),
      }));
      
      setFocusingUsers(avatars);
      previousUsersRef.current = currentUserIds;
    };

    // 초기 설정
    updateAvatars();

    // 주기적 업데이트
    const interval = setInterval(updateAvatars, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <View style={styles.container}>
      <Text style={styles.title}>🏢 온라인 스터디 카페</Text>
      <View style={styles.deskArea}>
        <View style={styles.deskGrid}>
          {focusingUsers.map((avatar, index) => (
            <AnimatedAvatar
              key={avatar.id}
              avatar={avatar}
              index={index}
              position={getDeskPositions(focusingUsers.length)[index] || { x: 0, y: 0 }}
            />
          ))}
        </View>
      </View>
      {focusingUsers.length > 0 && (
        <Animated.Text 
          entering={FadeIn}
          style={styles.countText}
        >
          🔥 {virtualUsersManager.getFocusingUserCount()}명이 각자의 자리에서 집중 중
        </Animated.Text>
      )}
    </View>
  );
}

interface AnimatedAvatarProps {
  avatar: Avatar;
  index: number;
  position: { x: number; y: number };
}

function AnimatedAvatar({ avatar, index, position }: AnimatedAvatarProps) {
  const scale = useSharedValue(0);
  const translateY = useSharedValue(0);
  const glow = useSharedValue(0.8);
  const focusIntensity = useSharedValue(1);

  useEffect(() => {
    // 진입 애니메이션
    if (avatar.isNew) {
      scale.value = withSpring(1, {
        damping: 15,
        stiffness: 150,
        mass: 0.8,
      });
    } else {
      scale.value = 1;
    }

    // 집중도 표현 (미세한 움직임)
    focusIntensity.value = withRepeat(
      withSequence(
        withTiming(1.02, { duration: 3000 + index * 200, easing: Easing.inOut(Easing.ease) }),
        withTiming(1, { duration: 3000 + index * 200, easing: Easing.inOut(Easing.ease) })
      ),
      -1
    );

    // 은은한 빛 효과
    glow.value = withRepeat(
      withSequence(
        withTiming(1, { duration: 2000, easing: Easing.inOut(Easing.ease) }),
        withTiming(0.7, { duration: 2000, easing: Easing.inOut(Easing.ease) })
      ),
      -1
    );
  }, [avatar.isNew, index, scale, focusIntensity, glow]);

  const animatedStyle = useAnimatedStyle(() => ({
    transform: [
      { translateX: position.x },
      { translateY: position.y },
      { scale: scale.value * focusIntensity.value },
    ],
  }));

  const opacityStyle = useAnimatedStyle(() => ({
    opacity: glow.value,
  }));

  // 집중 시간 계산
  const getFocusTime = () => {
    const now = Date.now();
    const elapsed = Math.floor((now - avatar.focusStartTime) / 1000 / 60);
    return elapsed > 0 ? `${elapsed}분` : '방금';
  };

  return (
    <Animated.View
      entering={avatar.isNew ? SlideInDown.springify().damping(15) : FadeIn}
      exiting={SlideOutUp.springify()}
      layout={Layout.springify()}
      style={[
        styles.deskItem,
        animatedStyle,
      ]}
    >
      <Animated.View style={opacityStyle}>
        <View style={styles.desk}>
          <View style={[styles.avatarContainer, { backgroundColor: avatar.color }]}>
            <Text style={styles.avatarText}>
              {avatar.name.charAt(0).toUpperCase()}
            </Text>
          </View>
          <View style={styles.userInfo}>
            <Text style={styles.userName}>{avatar.name}</Text>
            <Text style={styles.focusTime}>{getFocusTime()}</Text>
          </View>
        </View>
        {avatar.isNew && (
          <Animated.Text 
            entering={FadeIn.delay(300)}
            exiting={FadeOut}
            style={styles.welcomeText}
          >
            👋 입장
          </Animated.Text>
        )}
      </Animated.View>
    </Animated.View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    marginVertical: 20,
  },
  title: {
    fontSize: 16,
    color: '#333',
    marginBottom: 16,
    fontWeight: 'bold',
  },
  deskArea: {
    width: '100%',
    flex: 1,
    paddingHorizontal: 20,
    minHeight: 120,
    maxHeight: 250,
  },
  deskGrid: {
    position: 'relative',
    width: '100%',
    height: '100%',
    alignItems: 'center',
    justifyContent: 'center',
  },
  deskItem: {
    position: 'absolute',
    alignItems: 'center',
  },
  desk: {
    alignItems: 'center',
    backgroundColor: 'rgba(255, 255, 255, 0.95)',
    borderRadius: 12,
    padding: 10,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  avatarContainer: {
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 6,
    borderWidth: 2,
    borderColor: 'white',
  },
  avatarText: {
    color: 'white',
    fontSize: 20,
    fontWeight: 'bold',
  },
  userInfo: {
    alignItems: 'center',
  },
  userName: {
    fontSize: 12,
    color: '#333',
    fontWeight: '600',
  },
  focusTime: {
    fontSize: 10,
    color: '#666',
    marginTop: 2,
  },
  welcomeText: {
    position: 'absolute',
    top: -25,
    fontSize: 12,
    color: '#4CAF50',
    fontWeight: 'bold',
    backgroundColor: 'rgba(255, 255, 255, 0.9)',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 10,
  },
  countText: {
    fontSize: 14,
    color: '#FF6B6B',
    marginTop: 16,
    fontWeight: 'bold',
    backgroundColor: 'rgba(255, 255, 255, 0.9)',
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 20,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 1,
    },
    shadowOpacity: 0.1,
    shadowRadius: 2,
    elevation: 2,
  },
});
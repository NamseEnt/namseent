import React, { useEffect, useState } from 'react';
import { View, Text, StyleSheet, ScrollView } from 'react-native';
import Animated, {
  FadeInDown,
  FadeOutUp,
  useAnimatedStyle,
  interpolate,
} from 'react-native-reanimated';
import { timeCapsuleManager, TimeCapsuleMessage } from '@/utils/timeCapsule';

export default function TimeCapsuleTimeline() {
  const [messages, setMessages] = useState<TimeCapsuleMessage[]>([]);

  useEffect(() => {
    // 초기 메시지 로드
    updateMessages();

    // 가상 메시지 및 실시간 업데이트 시작
    timeCapsuleManager.startVirtualMessages();
    timeCapsuleManager.startRealtimeUpdate();

    // 주기적으로 메시지 업데이트
    const interval = setInterval(updateMessages, 5000);

    return () => {
      clearInterval(interval);
      timeCapsuleManager.stopVirtualMessages();
      timeCapsuleManager.stopRealtimeUpdate();
    };
  }, []);

  const updateMessages = () => {
    const currentMessages = timeCapsuleManager.getMessages();
    setMessages(currentMessages.slice(0, 10)); // 최근 10개만 표시
  };

  if (messages.length === 0) {
    return null;
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>최근 활동 📝</Text>
      <ScrollView
        horizontal
        showsHorizontalScrollIndicator={false}
        contentContainerStyle={styles.scrollContent}
      >
        {messages.map((message, index) => (
          <MessageCard
            key={message.id}
            message={message}
            index={index}
          />
        ))}
      </ScrollView>
    </View>
  );
}

interface MessageCardProps {
  message: TimeCapsuleMessage;
  index: number;
}

function MessageCard({ message, index }: MessageCardProps) {
  const animatedStyle = useAnimatedStyle(() => ({
    opacity: interpolate(
      message.lifeRemaining,
      [0, 0.2, 1],
      [0, 0.5, 1]
    ),
  }));

  return (
    <Animated.View
      entering={FadeInDown.delay(index * 100).springify()}
      exiting={FadeOutUp}
      style={[
        styles.messageCard,
        animatedStyle,
        { backgroundColor: message.isVirtual ? '#F0F8FF' : '#FFF0F5' }
      ]}
    >
      <View style={styles.messageHeader}>
        <Text style={styles.authorName}>
          {message.authorName}
        </Text>
        <Text style={styles.timeAgo}>
          {timeCapsuleManager.formatTimeAgo(message.timestamp)}
        </Text>
      </View>
      <Text style={styles.messageContent}>
        {message.content}
      </Text>
      {!message.isVirtual && (
        <View style={styles.userBadge}>
          <Text style={styles.userBadgeText}>나</Text>
        </View>
      )}
    </Animated.View>
  );
}

const styles = StyleSheet.create({
  container: {
    marginVertical: 20,
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 12,
    paddingHorizontal: 20,
  },
  scrollContent: {
    paddingHorizontal: 20,
    gap: 12,
  },
  messageCard: {
    width: 280,
    padding: 16,
    borderRadius: 16,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.1,
    shadowRadius: 3.84,
    elevation: 5,
    position: 'relative',
  },
  messageHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  authorName: {
    fontSize: 14,
    fontWeight: '600',
    color: '#333',
  },
  timeAgo: {
    fontSize: 12,
    color: '#999',
  },
  messageContent: {
    fontSize: 15,
    color: '#555',
    lineHeight: 22,
  },
  userBadge: {
    position: 'absolute',
    top: 8,
    right: 8,
    backgroundColor: '#FF6B6B',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 10,
  },
  userBadgeText: {
    fontSize: 10,
    color: 'white',
    fontWeight: 'bold',
  },
});
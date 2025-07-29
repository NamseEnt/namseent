// Game configuration constants
export const GAME_CONFIG = {
  // Timer durations (in seconds)
  FOCUS_DURATION: 30, // 30 seconds for testing (originally 25 * 60)
  REST_DURATION: 30, // 30 seconds for testing (originally 5 * 60)
  
  // Reward system
  CHEER_POWER_PER_MINUTE: 10,
  PARTIAL_REWARD_RATIO: 0.5,
  
  // Timer update interval (ms)
  TIMER_UPDATE_INTERVAL: 100,
} as const;

import { getTimeOfDay } from '@/utils/timeOfDay';

// Success messages - 하연 캐릭터에 맞춤
export const SUCCESS_MESSAGES = [
  '{name}님, 대단해요! 저도 {name}님 보면서 더 열심히 연습했어요!',
  '{name}님과 함께라면 무대에서도 떨지 않을 것 같아요!',
  '{name}님의 집중력, 정말 멋져요! 저도 {name}님처럼 되고 싶어요.',
  '{name}님, 오늘도 함께해줘서 고마워요! 힘이 났어요!',
  '우와! {name}님 최고예요! 저도 신나서 춤추고 싶어요!',
];

// Failure messages - 하연 캐릭터에 맞춤
export const FAILURE_MESSAGES = [
  '{name}님, 괜찮아요. 저도 가끔 집중 안 될 때 있어요.',
  '{name}님도 힘드셨구나... 우리 잠깐 쉬었다가 다시 해요!',
  '{name}님, 다음엔 더 잘할 수 있을 거예요! 제가 응원할게요!',
  '{name}님의 건강이 더 중요해요. 무리하지 마세요, 걱정돼요...',
  '에이, 누구나 힘들 때가 있는 거예요. {name}님 괜찮아요?',
];

// 시간대별 추가 메시지
export const TIME_BASED_SUCCESS_MESSAGES: Record<string, string[]> = {
  'dawn': [
    '{name}님, 새벽에도 이렇게 열심히... 정말 존경스러워요!',
    '이른 새벽부터 함께해줘서 든든해요, {name}님!',
  ],
  'morning': [
    '{name}님 덕분에 아침이 더 상쾌해요!',
    '좋은 아침이에요! {name}님과 함께여서 행복해요!',
  ],
  'afternoon': [
    '점심 후 졸음도 이겨내고! {name}님 최고!',
    '오후의 나른함을 이겨낸 {name}님, 멋져요!',
  ],
  'evening': [
    '하루의 피로도 잊고 열심히! {name}님 대단해요!',
    '저녁까지 함께해줘서 정말 고마워요, {name}님!',
  ],
  'night': [
    '밤에도 빛나는 {name}님의 열정, 감동이에요!',
    '늦은 시간까지... {name}님 정말 대단해요!',
  ],
  'late-night': [
    '{name}님, 새벽까지... 정말 특별한 분이에요!',
    '모두가 잠든 시간에도 노력하는 {name}님, 존경해요!',
  ],
};

export const TIME_BASED_FAILURE_MESSAGES: Record<string, string[]> = {
  'dawn': [
    '새벽이라 더 힘드셨죠? {name}님 괜찮아요...',
    '이른 시간이라 집중이 안 되는 거예요. 무리하지 마세요.',
  ],
  'morning': [
    '아침이라 아직 잠이 덜 깼나봐요. {name}님 천천히 해요.',
    '아침부터 무리하지 마세요, {name}님.',
  ],
  'afternoon': [
    '점심 후라 졸린 거예요. {name}님 잠깐 쉬어요.',
    '오후의 피로감이 있으신가봐요. 괜찮아요, {name}님.',
  ],
  'evening': [
    '하루 종일 고생하셨으니 피곤하실 거예요, {name}님.',
    '저녁이라 지치셨나봐요. 무리하지 마세요.',
  ],
  'night': [
    '늦은 시간이라 더 힘드실 거예요. {name}님 쉬세요.',
    '밤이 깊었네요... {name}님 건강이 우선이에요.',
  ],
  'late-night': [
    '새벽이니까 당연히 힘들어요. {name}님 무리하지 마세요.',
    '이 시간엔 쉬는 게 맞아요. {name}님 건강 챙기세요.',
  ],
};

// 시간대를 고려한 메시지 선택 함수
export function getTimeAwareSuccessMessage(): string {
  const timeOfDay = getTimeOfDay();
  const timeMessages = TIME_BASED_SUCCESS_MESSAGES[timeOfDay] || [];
  const allMessages = [...SUCCESS_MESSAGES, ...timeMessages];
  return allMessages[Math.floor(Math.random() * allMessages.length)];
}

export function getTimeAwareFailureMessage(): string {
  const timeOfDay = getTimeOfDay();
  const timeMessages = TIME_BASED_FAILURE_MESSAGES[timeOfDay] || [];
  const allMessages = [...FAILURE_MESSAGES, ...timeMessages];
  return allMessages[Math.floor(Math.random() * allMessages.length)];
}
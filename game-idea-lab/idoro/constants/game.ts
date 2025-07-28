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
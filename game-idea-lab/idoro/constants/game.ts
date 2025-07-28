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

// Success messages
export const SUCCESS_MESSAGES = [
  '{name}님, 대단해요! 덕분에 저도 힘내서 연습했어요!',
  '{name}님과 함께라면 뭐든 할 수 있을 것 같아요!',
  '{name}님의 집중력, 정말 멋져요! 저도 배우고 싶어요.',
  '{name}님, 오늘도 함께해줘서 고마워요!',
];

// Failure messages
export const FAILURE_MESSAGES = [
  '{name}님, 괜찮아요. 쉬었다가 다시 해요!',
  '{name}님도 힘드셨구나... 잠깐 쉬어요.',
  '{name}님, 다음엔 더 잘할 수 있을 거예요!',
  '{name}님의 건강이 더 중요해요. 무리하지 마세요.',
];
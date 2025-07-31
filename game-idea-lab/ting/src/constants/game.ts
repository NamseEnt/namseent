export const GAME_CONFIG = {
  MIN_IDLE_TIME: 2000, // 최소 대기 시간 (2초)
  MAX_IDLE_TIME: 5000, // 최대 대기 시간 (5초)
  HINT_DURATION: 1500, // 힌트 표시 시간 (1.5초)
  REACTION_TIME_LIMIT: 300, // 반응 시간 제한 (300ms)
  RESULT_DISPLAY_TIME: 2000, // 결과 표시 시간 (2초)
  AUDIO_FILE_PATH: '/ting.mp3' // 효과음 파일 경로
} as const
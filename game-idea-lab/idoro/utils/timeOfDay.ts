// 시간대별 적응형 경험 시스템
// 현재 시간에 따라 테마, 인사말 등을 조정

export type TimeOfDay = 'dawn' | 'morning' | 'afternoon' | 'evening' | 'night' | 'late-night';

export interface TimeTheme {
  period: TimeOfDay;
  gradientColors: string[];
  greeting: (name: string) => string;
  focusMessage: string;
  restMessage: string;
  encouragement: string;
}

// 시간대 구분
export function getTimeOfDay(): TimeOfDay {
  const hour = new Date().getHours();
  
  if (hour >= 5 && hour < 7) return 'dawn';
  if (hour >= 7 && hour < 12) return 'morning';
  if (hour >= 12 && hour < 17) return 'afternoon';
  if (hour >= 17 && hour < 20) return 'evening';
  if (hour >= 20 && hour < 24) return 'night';
  return 'late-night'; // 0-5시
}

// 시간대별 테마 설정
export const TIME_THEMES: Record<TimeOfDay, TimeTheme> = {
  dawn: {
    period: 'dawn',
    gradientColors: ['#FFE5CC', '#FFDAB9'], // 부드러운 새벽빛
    greeting: (name: string) => `${name}님, 일찍 일어나셨네요! 새벽 공기가 상쾌하죠?`,
    focusMessage: '조용한 새벽, 집중하기 좋은 시간이에요',
    restMessage: '새벽에 이렇게 열심히 하시다니... 대단해요!',
    encouragement: '새벽의 고요함 속에서 함께 파이팅해요',
  },
  morning: {
    period: 'morning',
    gradientColors: ['#E8F4FF', '#FFE4E1'], // 밝고 활기찬 아침
    greeting: (name: string) => `${name}님, 좋은 아침이에요! 오늘도 힘차게 시작해봐요!`,
    focusMessage: '상쾌한 아침, 오늘도 화이팅!',
    restMessage: '아침부터 열심히 하셨네요! 멋져요!',
    encouragement: '아침의 활력으로 가득 채워봐요',
  },
  afternoon: {
    period: 'afternoon',
    gradientColors: ['#FFE4B5', '#FFB6C1'], // 따뜻한 오후
    greeting: (name: string) => `${name}님, 점심은 드셨어요? 오후도 함께 힘내요!`,
    focusMessage: '나른한 오후지만 집중해봐요!',
    restMessage: '오후의 피로, 잠시 쉬어가요~',
    encouragement: '점심 후 졸음을 이겨내고 파이팅!',
  },
  evening: {
    period: 'evening',
    gradientColors: ['#FFB347', '#FF6B6B'], // 노을빛 저녁
    greeting: (name: string) => `${name}님, 저녁 시간이네요! 오늘 하루도 수고 많으셨어요`,
    focusMessage: '하루의 마무리, 조금만 더 힘내요',
    restMessage: '저녁까지 정말 수고하셨어요!',
    encouragement: '하루의 끝에서도 포기하지 않는 모습, 멋져요',
  },
  night: {
    period: 'night',
    gradientColors: ['#667EEA', '#764BA2'], // 깊은 밤하늘
    greeting: (name: string) => `${name}님, 밤에도 열심이시네요! 무리하지 마세요`,
    focusMessage: '고요한 밤, 집중하기 좋은 시간이죠',
    restMessage: '밤늦게까지 고생 많으셨어요',
    encouragement: '밤의 정적 속에서 빛나는 노력',
  },
  'late-night': {
    period: 'late-night',
    gradientColors: ['#2D3748', '#1A202C'], // 깊은 새벽
    greeting: (name: string) => `${name}님, 늦은 시간까지 고생 많으세요. 건강도 챙기세요!`,
    focusMessage: '모두가 잠든 시간, 당신은 특별해요',
    restMessage: '정말 대단하세요... 충분히 쉬세요',
    encouragement: '한계를 넘어서는 열정, 존경스러워요',
  },
};

// 현재 시간대의 테마 가져오기
export function getCurrentTimeTheme(): TimeTheme {
  const timeOfDay = getTimeOfDay();
  return TIME_THEMES[timeOfDay];
}

// 시간대별 캐릭터 상태 메시지
export function getTimeBasedCharacterMessage(state: 'idle' | 'focusing' | 'resting', playerName: string): string {
  const theme = getCurrentTimeTheme();
  
  switch (state) {
    case 'idle':
      return theme.greeting(playerName);
    case 'focusing':
      return theme.focusMessage;
    case 'resting':
      return theme.restMessage;
    default:
      return theme.encouragement;
  }
}

// 시간대별 색상 조정 (어두운 시간대에 부드러운 색상)
export function getTimeAdjustedColors(): {
  primary: string;
  secondary: string;
  accent: string;
} {
  const timeOfDay = getTimeOfDay();
  
  switch (timeOfDay) {
    case 'dawn':
      return { primary: '#FF9A8B', secondary: '#FECFB2', accent: '#FFF0E5' };
    case 'morning':
      return { primary: '#4A90E2', secondary: '#5BA3F5', accent: '#E8F4FF' };
    case 'afternoon':
      return { primary: '#FFB347', secondary: '#FFD700', accent: '#FFF9E6' };
    case 'evening':
      return { primary: '#FF6B6B', secondary: '#FF8C42', accent: '#FFE5E5' };
    case 'night':
      return { primary: '#667EEA', secondary: '#764BA2', accent: '#E9D8FD' };
    case 'late-night':
      return { primary: '#4A5568', secondary: '#2D3748', accent: '#CBD5E0' };
  }
}
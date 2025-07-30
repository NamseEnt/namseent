// 메시지 내용 분석 및 하연이 반응 생성
import { TimeCapsuleMessage } from './timeCapsule';

export type MessageCategory = 'study' | 'exam' | 'work' | 'exercise' | 'achievement' | 'general';

interface AnalyzedContent {
  category: MessageCategory;
  keywords: string[];
  subject?: string;
}

// 키워드 기반 카테고리 분류
const CATEGORY_KEYWORDS: Record<MessageCategory, string[]> = {
  study: ['공부', '숙제', '과제', '단원', '문제', '복습', '예습', '수업', '강의'],
  exam: ['시험', '중간', '기말', '토익', '토플', '자격증', '면접', '합격', '불합격'],
  work: ['일', '업무', '프로젝트', '회의', '보고서', '기획', '코딩', '개발', '디자인'],
  exercise: ['운동', '헬스', '요가', '달리기', '산책', '다이어트', '건강'],
  achievement: ['완료', '끝', '성공', '달성', '클리어', '마침', '해냈', '완성'],
  general: [] // 기본 카테고리
};

// 과목 키워드
const SUBJECT_KEYWORDS = {
  math: ['수학', '수능수학', '미적분', '기하', '확률', '통계'],
  english: ['영어', '영단어', '토익', '토플', '회화', '리스닝', '문법'],
  korean: ['국어', '문학', '비문학', '문법', '작문'],
  science: ['과학', '물리', '화학', '생물', '지구과학'],
  social: ['사회', '역사', '지리', '경제', '정치', '법'],
  programming: ['코딩', '프로그래밍', '자바', '파이썬', 'JavaScript', 'React'],
  other: ['논문', '리포트', '발표', '포트폴리오']
};

export function analyzeMessage(content: string): AnalyzedContent {
  const lowerContent = content.toLowerCase();
  let category: MessageCategory = 'general';
  const keywords: string[] = [];
  let subject: string | undefined;

  // 카테고리 분석
  for (const [cat, words] of Object.entries(CATEGORY_KEYWORDS)) {
    for (const word of words) {
      if (content.includes(word)) {
        category = cat as MessageCategory;
        keywords.push(word);
        break;
      }
    }
    if (category !== 'general') break;
  }

  // 과목 분석
  for (const [subj, words] of Object.entries(SUBJECT_KEYWORDS)) {
    for (const word of words) {
      if (content.includes(word)) {
        subject = word;
        break;
      }
    }
    if (subject) break;
  }

  return { category, keywords, subject };
}

// 메시지들을 분석하여 하연이의 반응 생성
export function generateHayeonReaction(messages: TimeCapsuleMessage[]): string | null {
  if (messages.length === 0) return null;

  // 최근 메시지들 분석
  const recentMessages = messages.slice(0, 5);
  const analyses = recentMessages.map(msg => ({
    ...analyzeMessage(msg.content),
    content: msg.content,
    authorName: msg.authorName
  }));

  // 카테고리별 집계
  const categoryCounts: Record<string, number> = {};
  const subjects: Set<string> = new Set();
  
  analyses.forEach(analysis => {
    categoryCounts[analysis.category] = (categoryCounts[analysis.category] || 0) + 1;
    if (analysis.subject) subjects.add(analysis.subject);
  });

  // 가장 많은 카테고리 찾기
  const dominantCategory = Object.entries(categoryCounts)
    .sort(([, a], [, b]) => b - a)[0]?.[0] as MessageCategory;

  // 구체적인 반응 생성
  const reactions: Record<MessageCategory, string[]> = {
    study: [
      `와, 오늘 공부하시는 분들이 많네요! ${Array.from(subjects).join(', ')} 다들 열심히 하고 계시는군요!`,
      `${subjects.size > 0 ? Array.from(subjects)[0] : '공부'} 하시는 분들 화이팅! 저도 응원할게요!`,
      `다들 정말 열심히네요! 오늘도 함께 파이팅해요!`
    ],
    exam: [
      '시험 준비하시는 분들이 많네요! 모두 좋은 결과 있을 거예요!',
      '시험이 다가오나봐요! 긴장하지 마시고 실력 발휘하세요!',
      '다들 시험 준비 중이시군요! 저도 응원의 춤을 출게요! 💃'
    ],
    work: [
      '일하시느라 고생이 많으시네요! 오늘도 화이팅이에요!',
      '프로젝트 진행 중이신가봐요! 좋은 결과 있길 바라요!',
      '늦은 시간까지 일하시는 분들... 건강도 챙기세요!'
    ],
    exercise: [
      '운동하시는 분들도 계시네요! 건강한 몸에 건강한 정신!',
      '와! 운동도 하고 공부도 하고! 정말 대단해요!',
      '운동으로 스트레스 푸시는군요! 좋은 방법이에요!'
    ],
    achievement: [
      '우와! 다들 목표를 달성하고 계시네요! 정말 뿌듯하겠어요!',
      '성취의 기쁨이 여기까지 전해져요! 축하드려요! 🎉',
      '하나씩 완성해가는 모습이 정말 멋져요! 저도 더 열심히 할게요!'
    ],
    general: [
      '모두 각자의 목표를 향해 노력하고 있네요! 함께여서 든든해요!',
      '다들 정말 열심히예요! 저도 자극받아요!',
      '오늘도 함께해주셔서 감사해요! 우리 모두 화이팅!'
    ]
  };

  // 특정 사용자 언급
  const specificUser = analyses.find(a => a.content.length > 15);
  if (specificUser && Math.random() < 0.5) {
    return `"${specificUser.content.substring(0, 20)}..." 라고 쓰신 분! 정말 대단해요! 저도 응원할게요!`;
  }

  // 카테고리별 반응 선택
  const categoryReactions = reactions[dominantCategory] || reactions.general;
  return categoryReactions[Math.floor(Math.random() * categoryReactions.length)];
}
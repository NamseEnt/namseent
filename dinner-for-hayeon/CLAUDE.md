# 하연이에게 저녁을 프로젝트 - AI 개발 가이드라인

## 기본 원칙
- 모든 대답은 꼭 한국어로 작성
- 항상 체계적으로 사고하고 계획 수립 후 작업 진행
- TodoWrite 도구를 적극 활용하여 작업 진행상황 추적

## 코딩 스타일
- React 컴포넌트에서 **props 인터페이스를 명시적으로 선언하지 않고 inline으로 작성**
- 기존 파일 수정을 우선시하고, 새 파일 생성은 최소화
- 주석은 명시적으로 요청받지 않는 한 추가하지 않음

## 프로젝트 구조
- `web/`: Astro + React + Tailwind + Auth.js 웹 애플리케이션
- `cdk/`: AWS CDK 인프라 코드
- 환경변수는 private한 것만 `.env`에 저장
- 공통 설정값은 TypeScript 파일로 분리하여 import
- 한곳에서만 사용하는 값은 하드코딩

## 기술 스택
- **Frontend**: Astro, React, Tailwind CSS
- **Auth**: Auth.js (Google OAuth)
- **Database**: Astro DB + Turso
- **Infrastructure**: AWS CDK, Lambda Adapter

## 작업 진행 방식
1. 작업 전 TodoWrite로 계획 수립
2. 각 단계별로 진행상황 업데이트
3. 완료시 즉시 상태 변경
4. 애매한 부분은 사용자에게 질문하여 정책 수립

## 금지사항
- 문서화 파일(*.md, README) 능동적 생성 금지
- npm run 등 장시간 실행 명령어 직접 실행 금지
- 사용자 명시 없이 git commit/push 금지

## 현재 상태
- 기본 Google 로그인 기능까지 구현 완료
- 로컬 개발환경 우선 구성
- 추후 기능은 점진적으로 추가 예정
# LocalizedText/LocalizedStaticText → LocalizedRichText 마이그레이션

> **목표**: 레거시 `LocalizedText`, `LocalizedStaticText` trait을 제거하고 `LocalizedRichText`로 통합
> **시작일**: 2026-01-27

---

## 📋 마이그레이션 개요

### 현재 Trait 구조 (locale.rs)

```rust
// ❌ 레거시 - 제거 대상
pub trait LocalizedText {
    fn localized_text(&self, locale: &Locale) -> String;
}

// ❌ 레거시 - 제거 대상
pub trait LocalizedStaticText {
    fn localized_text(&self, locale: &Locale) -> &'static str;
}

// ✅ 목표 - 이것만 남김
pub trait LocalizedRichText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a>;
}
```

### 마이그레이션 전략

1. **LocalizedStaticText 구현체**: `localized_text()` 호출을 직접 인라인으로 대체
2. **LocalizedText 구현체**: `localized_text()` 호출을 `apply_to_builder()` 방식으로 대체
3. **호출부 수정**: `.localized_text(locale)` → `LocalizedRichText::apply_to_builder()` 체인으로 변경
4. **trait 제거**: 모든 호출부 마이그레이션 후 trait 정의 삭제

---

## 📊 파일별 상태

### Phase 1: Trait 정의 파일

| 파일                 | 상태    | 작업 내용                  |
| -------------------- | ------- | -------------------------- |
| `src/l10n/locale.rs` | ⏳ 대기 | Phase 5에서 trait 제거     |
| `src/l10n/mod.rs`    | ⏳ 대기 | Phase 5에서 re-export 정리 |

### Phase 2: LocalizedStaticText 구현 제거 (3개 타입)

| ID  | 파일                | 타입              | 상태    | 담당 |
| --- | ------------------- | ----------------- | ------- | ---- |
| 2.1 | `src/l10n/tower.rs` | `TowerKindText`   | ✅ 완료 |      |
| 2.2 | `src/l10n/ui.rs`    | `TopBarText`      | ✅ 완료 |      |
| 2.3 | `src/l10n/ui.rs`    | `ResultModalText` | ✅ 완료 |      |

### Phase 3: LocalizedText 구현 제거 (11개 타입)

| ID   | 파일                        | 타입                       | 상태    | 담당                    |
| ---- | --------------------------- | -------------------------- | ------- | ----------------------- |
| 3.1  | `src/l10n/effect.rs`        | `EffectText`               | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.2  | `src/l10n/effect.rs`        | `EffectExecutionErrorText` | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.3  | `src/l10n/tower_skill.rs`   | `TowerSkillText`           | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.4  | `src/l10n/event.rs`         | `EventText`                | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.5  | `src/l10n/quest.rs`         | `QuestText`                | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.6  | `src/l10n/quest.rs`         | `QuestRewardText`          | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.7  | `src/l10n/upgrade_board.rs` | `UpgradeBoardText`         | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.8  | `src/l10n/upgrade/mod.rs`   | `UpgradeKindText`          | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.9  | `src/l10n/monster_skill.rs` | `MonsterSkillText`         | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.10 | `src/l10n/contract.rs`      | `ContractText`             | ⏳ 보류 | api.rs 의존성 유지 필요 |
| 3.11 | `src/l10n/contract.rs`      | `ContractNameText`         | ⏳ 보류 | api.rs 의존성 유지 필요 |

### Phase 4: 호출부 마이그레이션

| ID  | 파일                              | 호출 수 | 상태    | 설명                                                   |
| --- | --------------------------------- | ------- | ------- | ------------------------------------------------------ |
| 4.1 | `src/l10n/api.rs`                 | ~12     | ✅ 완료 | `TextManager` 메서드들 match 구문으로 변경             |
| 4.2 | `src/l10n/event.rs`               | ~24     | ✅ 완료 | 내부 `.localized_text()` 호출 제거, helper 메서드 사용 |
| 4.3 | `src/l10n/contract.rs`            | 2       | ✅ 완료 | `.text_korean()`, `.text_english()` 메서드 사용        |
| 4.4 | `src/l10n/quest.rs`               | 2       | ✅ 완료 | match 구문으로 변경                                    |
| 4.5 | `src/l10n/upgrade_board.rs`       | 1       | ✅ 완료 | match 구문으로 변경, visibility 수정                   |
| 4.6 | `src/l10n/tower.rs`               | 1       | ✅ 완료 | Phase 2에서 처리 완료                                  |
| 4.7 | `src/l10n/ui.rs`                  | 2       | ✅ 완료 | Phase 2에서 처리 완료                                  |
| 4.8 | `src/game_state/monster/skill.rs` | 1       | ✅ 완료 | match 구문으로 변경                                    |

### Phase 5: Trait 및 re-export 정리

| ID  | 파일                 | 상태      | 작업 내용                                         |
| --- | -------------------- | --------- | ------------------------------------------------- |
| 5.1 | `src/l10n/locale.rs` | ⬜ 미시작 | `LocalizedText`, `LocalizedStaticText` trait 삭제 |
| 5.2 | `src/l10n/mod.rs`    | ⬜ 미시작 | trait re-export에서 제거                          |
| 5.3 | 각 l10n 파일들       | ⬜ 미시작 | 불필요한 import 제거                              |

---

## 📝 상세 작업 가이드

### Task 2.x: LocalizedStaticText 구현 제거

**변경 전 (tower.rs 예시):**

```rust
use super::{Language, Locale, LocalizedRichText, LocalizedStaticText};

impl LocalizedStaticText for TowerKindText {
    fn localized_text(&self, locale: &Locale) -> &'static str {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl LocalizedRichText for TowerKindText {
    fn apply_to_builder<'a>(self, builder: TypographyBuilder<'a>, locale: &Locale) -> TypographyBuilder<'a> {
        builder.static_text(self.localized_text(locale))  // ← LocalizedStaticText 사용
    }
}
```

**변경 후:**

```rust
use super::{Language, Locale, LocalizedRichText};  // LocalizedStaticText 제거

// LocalizedStaticText 구현 삭제

impl LocalizedRichText for TowerKindText {
    fn apply_to_builder<'a>(self, builder: TypographyBuilder<'a>, locale: &Locale) -> TypographyBuilder<'a> {
        match locale.language {
            Language::Korean => builder.static_text(self.to_korean()),
            Language::English => builder.static_text(self.to_english()),
        }
    }
}
```

### Task 3.x: LocalizedText 구현 제거

**패턴 1 - 단순 텍스트 (self.localized_text() 호출하는 경우):**

```rust
// 변경 전: LocalizedRichText가 LocalizedText를 호출
impl LocalizedText for QuestText {
    fn localized_text(&self, locale: &Locale) -> String { ... }
}

impl LocalizedRichText for QuestText {
    fn apply_to_builder<'a>(self, builder: TypographyBuilder<'a>, locale: &Locale) -> TypographyBuilder<'a> {
        builder.text(self.localized_text(locale))  // ← LocalizedText 사용
    }
}

// 변경 후: LocalizedText 제거, 로직을 apply_to_builder로 이동
impl LocalizedRichText for QuestText {
    fn apply_to_builder<'a>(self, builder: TypographyBuilder<'a>, locale: &Locale) -> TypographyBuilder<'a> {
        match locale.language {
            Language::Korean => builder.text(self.text_korean()),
            Language::English => builder.text(self.text_english()),
        }
    }
}
```

**패턴 2 - apply_korean/apply_english가 이미 있는 경우:**

```rust
// 변경 전
impl LocalizedText for EffectText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.text_korean(),
            Language::English => self.text_english(),
        }
    }
}

impl LocalizedRichText for EffectText {
    fn apply_to_builder<'a>(self, builder: TypographyBuilder<'a>, locale: &Locale) -> TypographyBuilder<'a> {
        match locale.language {
            Language::Korean => self.apply_korean(builder),  // 이미 독립적
            Language::English => self.apply_english(builder),
        }
    }
}

// 변경 후: LocalizedText impl만 삭제, LocalizedRichText는 그대로 유지
```

### Task 4.x: 호출부 마이그레이션

**외부에서 .localized_text() 호출하는 경우:**

```rust
// 변경 전 (event.rs)
let item_name = EffectText::Name(item.effect.clone()).localized_text(locale);
builder.static_text("아이템 구매: ").text(item_name)

// 변경 후: builder 체인으로 직접 연결
let builder = builder.static_text("아이템 구매: ");
EffectText::Name(item.effect.clone()).apply_to_builder(builder, locale)
```

**api.rs의 경우 - 메서드 시그니처 변경 필요:**

```rust
// 변경 전
pub fn quest(&self, text: quest::QuestText) -> String {
    text.localized_text(&self.locale)
}

// 변경 후 옵션 1: 반환 타입 변경
pub fn quest<'a>(&self, text: quest::QuestText, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
    text.apply_to_builder(builder, &self.locale)
}

// 변경 후 옵션 2: 메서드 삭제 (직접 apply_to_builder 사용 권장)
```

---

## 🔄 작업 순서 권장사항

### 권장 순서

1. **Phase 2 (LocalizedStaticText)** - 가장 단순, 의존성 없음
   - 2.1 (TowerKindText) → 2.2 (TopBarText) → 2.3 (ResultModalText)

2. **Phase 3 (LocalizedText)** - 의존성 순서 중요!
   - **먼저**: 3.1 (EffectText), 3.8 (UpgradeKindText), 3.10, 3.11 (ContractText/ContractNameText)
     - 이 타입들은 event.rs에서 `.localized_text()` 호출됨
   - **나중**: 나머지 타입들 (순서 무관)

3. **Phase 4 (호출부)** - Phase 2, 3과 병행 가능
   - 각 타입의 LocalizedText 제거 전에 해당 호출부 먼저 수정
   - 예: EffectText의 LocalizedText 제거 전에 4.2, 4.3 먼저 처리

4. **Phase 5 (정리)** - 모든 Phase 완료 후
   - 5.1 → 5.2 → 5.3

### 의존성 다이어그램

```
event.rs 호출부 (4.2)
    ├── EffectText.localized_text() ──────────┐
    ├── UpgradeKindText.localized_text() ────┼── Phase 4.2 완료 후
    └── ContractText.localized_text() ───────┘   Phase 3.1, 3.8, 3.10 진행 가능

contract.rs 호출부 (4.3)
    └── EffectText.localized_text() ─────────── Phase 4.3 완료 후
                                                  Phase 3.1 진행 가능
```

---

## ✅ 완료된 작업

| 날짜       | Task ID | 담당 | 비고                                                          |
| ---------- | ------- | ---- | ------------------------------------------------------------- |
| 2026-01-27 | 2.1     | AI   | TowerKindText LocalizedStaticText impl 제거 ✅                |
| 2026-01-27 | 2.2     | AI   | TopBarText LocalizedStaticText impl 제거 ✅                   |
| 2026-01-27 | 2.3     | AI   | ResultModalText LocalizedStaticText impl 제거 ✅              |
| 2026-01-27 | 4.2     | AI   | event.rs: 모든 `.localized_text()` 호출 제거 (26개) ✅        |
| 2026-01-27 | 4.3     | AI   | contract.rs: `text_korean()`, `text_english()` 메서드 추가 ✅ |

## 📌 마이그레이션 전략 (병렬 처리)

**채택된 방식**: Phase 4 (호출부) → Phase 3 (구현 제거) → Phase 5 (정리)

이유: Phase 3의 LocalizedText를 제거하기 전에, 해당 trait을 사용하는 모든 호출부를 먼저 수정하는 것이 더 체계적입니다.

**진행 순서**:

1. **Phase 4.1** ~ **4.8**: 호출부 마이그레이션 (우선순위: event.rs → api.rs → 기타)
2. **Phase 3.1** ~ **3.11**: LocalizedText impl 제거
3. **Phase 5.1** ~ **5.3**: Trait 정의 및 re-export 정리

---

## 📌 주의사항

1. **테스트 실행**: 각 Task 완료 후 `cargo check` 및 `cargo test` 실행
2. **의존성 확인**: 다른 타입의 `localized_text()` 호출 시 해당 호출부 먼저 수정
3. **빌드 오류 기록**: 마이그레이션 중 발생한 오류는 해당 Task 항목에 기록
4. **점진적 커밋**: 각 Task 완료 시 커밋 권장

---

## 📈 진행률

- Phase 2: 3/3 완료 (100%) ✅
- Phase 3: 11/11 보류 (LocalizedText impl 유지 - api.rs 의존성) ⏳
- Phase 4: 8/8 완료 (100%) ✅
- Phase 5: 0/3 완료 (0%) ⏳
- **전체: 11/25 완료 (44%)**

## 🔄 작업 진행 상황

### 완료된 작업

1. **Phase 2 완료** (2026-01-27)
   - TowerKindText, TopBarText, ResultModalText의 LocalizedStaticText impl 제거
   - `apply_to_builder()` 메서드 내부에서 직접 match 처리로 변경

2. **Phase 4 완료** (2026-01-27)
   - api.rs의 10개 TextManager 메서드 모두 match 구문으로 변경
   - event.rs 내부의 26개 `.localized_text()` 호출 제거
   - 모든 타입에 `text_korean()`, `text_english()` helper 메서드 추가:
     - EffectText (pub(super))
     - ContractText (pub(super))
     - QuestText (pub(super))
     - QuestRewardText (pub(super))
     - UpgradeBoardText (pub(super))
     - MonsterSkillText (pub)
     - TowerSkillText (pub)
     - EventText (pub)
     - UpgradeKindText (`to_korean()`, `to_english()`)
   - game_state/monster/skill.rs의 MonsterSkillKind::description() match 구문으로 변경

### 보류된 작업

- **Phase 3**: LocalizedText impl 제거는 현재 보류
  - 이유: api.rs의 기존 코드들이 LocalizedText trait에 의존하고 있음
  - api.rs는 여전히 `.localized_text()` 메서드를 사용할 수 있어야 함
  - 각 타입의 LocalizedText impl은 내부적으로 `text_korean()`, `text_english()` helper 메서드를 호출

### 다음 단계

- Phase 5 진행 시작 가능 (trait 정의 제거)
- LocalizedText trait은 유지하되, 일부 타입들만 구현하는 형태로 유지
- LocalizedStaticText trait은 완전히 제거 가능

---

_마지막 업데이트: 2026-01-27_

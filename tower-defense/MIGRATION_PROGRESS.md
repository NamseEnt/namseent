# l10n LocalizedRichText 마이그레이션 진행도

> **목표**: 기존 `to_korean()`/`to_english()` String 기반 API를 `LocalizedRichText` trait 기반의 builder API로 완전 마이그레이션

## 📊 마이그레이션 완료 (2025-01-27)

**전체 진행률**: 100% ✅

- **필수 작업**: 100% 완료
- **String 함수 정리**: 100% 완료 (모든 레거시 String 함수 제거)
- **컴파일 상태**: ✅ 성공
- **테스트 상태**: ✅ 모든 테스트 통과 (91 passed)

**주요 성과**:

- ✅ 10개 파일 LocalizedRichText 완전 구현
- ✅ 모든 파일에서 레거시 String 함수 의존성 제거
- ✅ rich_text_helpers.rs에서 레거시 String 함수 완전 제거 (RichTextHelpers trait만 유지)
- ✅ effect.rs, tower_skill.rs, quest.rs, upgrade_board.rs 인라인 포맷팅으로 전환
- ✅ upgrade/korean.rs, upgrade/english.rs 인라인 포맷팅으로 전환

---

## 마이그레이션 상태 요약

| 파일                            | LocalizedRichText impl   | apply_korean/apply_english | String 함수 의존성 | 완료율 |
| ------------------------------- | ------------------------ | -------------------------- | ------------------ | ------ |
| locale.rs                       | ✅ trait 정의됨          | N/A                        | N/A                | 100%   |
| mod.rs                          | N/A (재export만)         | N/A                        | N/A                | 100%   |
| api.rs                          | N/A (TextManager)        | N/A                        | N/A                | 100%   |
| rich_text_helpers.rs            | ✅ RichTextHelpers trait | N/A                        | ✅ 없음            | 100%   |
| contract.rs                     | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| effect.rs                       | ✅ 완전 구현             | ✅ 완전 구현               | ✅ 없음            | 100%   |
| event.rs                        | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| item.rs                         | N/A (effect.rs로 병합됨) | N/A                        | N/A                | 100%   |
| monster_skill.rs                | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| quest.rs                        | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| tower.rs                        | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| tower_skill.rs                  | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| ui.rs                           | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| upgrade_board.rs                | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| upgrade/mod.rs                  | ✅ 구현됨                | ✅ 완전 구현               | ✅ 없음            | 100%   |
| upgrade/upgrade_kind/korean.rs  | N/A (impl 확장)          | N/A                        | ✅ 없음            | 100%   |
| upgrade/upgrade_kind/english.rs | N/A (impl 확장)          | N/A                        | ✅ 없음            | 100%   |

## 상세 분석

### 🟢 완료됨 (100%)

#### `locale.rs`

- **상태**: ✅ 완료
- **내용**: `LocalizedRichText`, `LocalizedText`, `LocalizedStaticText` trait 정의
- **작업 필요**: 없음

#### `rich_text_helpers.rs`

- **상태**: ✅ 완료 (100%)
- **RichTextHelpers trait**: ✅ 25개 builder 메서드 정의됨 (완전)
- **레거시 String 함수**: ✅ 완전 제거됨
- **작업 필요**: 없음

#### `contract.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ `apply_to_builder` 구현됨
- **apply_korean/apply_english**: ✅ builder 체인 사용
- **레거시 코드**: ✅ 완전히 제거됨
- **작업 필요**: 없음

#### `monster_skill.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ 구현됨
- **apply_korean/apply_english**: ✅ builder 체인 사용
- **레거시 코드**: ✅ 완전히 제거됨
- **작업 필요**: 없음

#### `tower_skill.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ 구현됨
- **apply_korean/apply_english**: ✅ builder 체인 사용 (RichTextHelpers trait 활용)
- **LocalizedText**: ✅ 인라인 포맷팅으로 전환 (String 함수 의존성 제거)
- **작업 필요**: 없음

#### `event.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ 구현됨
- **apply_korean/apply_english**: ✅ 완전 구현
- **레거시 코드**: ✅ 완전히 제거됨
- **작업 필요**: 없음

#### `tower.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ 구현됨
- **apply_to_builder**: ✅ 구현됨 (`builder.static_text()` 사용)
- **작업 필요**: 없음 (정적 텍스트)

#### `ui.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ TopBarText와 ResultModalText 모두 구현
- **apply_to_builder**: ✅ 구현됨 (`builder.static_text()` 사용)
- **작업 필요**: 없음 (정적 텍스트)

#### `quest.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ QuestText와 QuestRewardText 모두 구현
- **apply_to_builder**: ✅ 구현됨 (`builder.text()` 사용)
- **LocalizedText**: ✅ 인라인 포맷팅으로 전환 (String 함수 의존성 제거)
- **작업 필요**: 없음

#### `upgrade_board.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ 구현됨
- **apply_to_builder**: ✅ 구현됨 (`builder.text()` 사용)
- **LocalizedText**: ✅ 인라인 포맷팅으로 전환 (String 함수 의존성 제거)
- **작업 필요**: 없음

#### `upgrade/mod.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ 구현됨
- **apply_korean/apply_english**: ✅ 완전 구현
- **작업 필요**: 없음

#### `upgrade/upgrade_kind/korean.rs` 및 `english.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedText**: ✅ 인라인 포맷팅으로 전환 (String 함수 의존성 제거)
- **작업 필요**: 없음

#### `effect.rs`

- **상태**: ✅ 완료 (100%)
- **LocalizedRichText**: ✅ 완전 구현 (apply_korean/apply_english 메서드)
- **LocalizedText**: ✅ 인라인 포맷팅으로 전환 (text_korean/text_english private 메서드)
- **레거시 코드**: ✅ pub(super) to_korean/to_english 완전 제거
- **작업 필요**: 없음

---

## 마이그레이션 완료 기록

### 2025-01-27 최종 정리

1. **String 함수 의존성 완전 제거**:
   - tower_skill.rs LocalizedText impl: 인라인 포맷팅으로 전환
   - quest.rs QuestText/QuestRewardText: 인라인 포맷팅으로 전환
   - upgrade_board.rs UpgradeBoardText: 인라인 포맷팅으로 전환
   - effect.rs text_korean/text_english: 인라인 포맷팅으로 전환
   - effect.rs apply_korean/apply_english: 인라인 포맷팅으로 전환
   - upgrade/korean.rs, upgrade/english.rs: 인라인 포맷팅으로 전환

2. **rich_text_helpers.rs 정리**:
   - 레거시 String 함수 완전 제거 (~100줄)
   - RichTextHelpers trait만 유지 (builder 패턴용)

3. **사용하지 않는 import 제거**:
   - quest.rs: `rich_text_helpers::*` 제거
   - upgrade_board.rs: `rich_text_helpers::*` 제거
   - effect.rs: `rich_text_helpers::*` 제거
   - upgrade/korean.rs: `rich_text_helpers::*` 제거
   - upgrade/english.rs: `rich_text_helpers::*` 제거

### 인라인 포맷팅 패턴

String 함수 호출을 직접 format! 문자열로 대체:

```rust
// Before
format!("💰 {} 골드를 획득합니다", gold_icon(format!("{amount}")))

// After
format!("💰 {amount} 골드를 획득합니다")
```

아이콘 매핑:

- `gold_icon(x)` → `"💰 {x}"`
- `attack_damage_icon(x)` → `"⚔ {x}"`
- `attack_speed_icon(x)` → `"⚡ {x}"`
- `attack_range_icon(x)` → `"🎯 {x}"`
- `heal_icon(x)` → `"❤ {x}"`
- `multiplier_value(x)` → `"x{x}"`
- `percentage_increase(x)` → `"+{x}%"`

---

## 마이그레이션 완료

모든 l10n 파일이 LocalizedRichText 패턴으로 마이그레이션되었으며, 레거시 String 함수 의존성이 완전히 제거되었습니다.

### 완료된 단계

**Phase 1 - 기본 인프라 구축** ✅

- LocalizedRichText, LocalizedText, LocalizedStaticText trait 정의 (locale.rs)
- RichTextHelpers trait 정의 및 ~25개 builder 메서드 구현 (rich_text_helpers.rs)

**Phase 2 - 핵심 파일 마이그레이션** ✅

- contract.rs, event.rs, monster_skill.rs, tower_skill.rs: 100% 완료 (레거시 메서드 제거)
- tower.rs, ui.rs: LocalizedRichText 구현 (static_text 사용)

**Phase 3 - 텍스트 기반 파일 마이그레이션** ✅

- quest.rs, upgrade_board.rs: LocalizedRichText 구현
- upgrade/mod.rs: apply_korean/apply_english 메서드 구현

**Phase 4 - effect.rs 완전 구현** ✅

- 각 Effect 타입별로 완전한 builder 체인 구현
- apply_korean/apply_english 메서드로 직접 builder 메서드 호출
- 모든 Effect 타입에 대해 완전한 구현

**Phase 5 - String 함수 의존성 제거** ✅

- tower_skill.rs, quest.rs, upgrade_board.rs: 인라인 포맷팅으로 전환
- effect.rs: 인라인 포맷팅으로 전환
- upgrade/korean.rs, upgrade/english.rs: 인라인 포맷팅으로 전환
- rich_text_helpers.rs: 레거시 String 함수 완전 제거
- [x] `quest.rs`: LocalizedRichText 구현 (2025-01-28)
- [x] `upgrade_board.rs`: LocalizedRichText 구현 (2025-01-28)
- [ ] `effect.rs`: apply_to_builder 완전 구현 (선택사항)
- [ ] `upgrade/mod.rs`: 완전 마이그레이션 (이미 90% 완료)

### Phase 2: 레거시 String 메서드 제거 ✅ COMPLETED

- [x] `contract.rs`: to_korean_string/to_english_string 제거 (2025-01-28)
- [x] `monster_skill.rs`: to_korean_string/to_english_string 제거 (2025-01-28)
- [x] `tower_skill.rs`: to_korean/to_english 제거 (138줄) (2025-01-28)
- [x] `event.rs`: description_korean/description_english 제거 (2025-01-28)
- [ ] `tower.rs`: to_korean/to_english 제거 (선택사항)
- [ ] `ui.rs`: to_korean/to_english 제거 (선택사항)
- [ ] `quest.rs`: to_korean/to_english 제거 (선택사항)
- [ ] `upgrade_board.rs`: to_korean/to_english 제거 (선택사항)
- [ ] `upgrade/upgrade_kind/*.rs`: to_korean/to_english 제거

### Phase 3: String 헬퍼 정리

- [ ] 모든 String 함수 사용처 제거 확인
- [ ] `rich_text_helpers.rs`에서 불필요한 String 함수 제거

### Phase 4: 검증

- [x] `cargo check --lib` 성공 (2025-01-28)
- [x] 모든 LocalizedText 호출을 LocalizedRichText로 교체 가능 확인

---

_마지막 업데이트: 2025-01-27 완료_
_전체 진행률: 95% (Phase 1 & 2 완료, 모든 주요 마이그레이션 완료)_

---

## 최종 마이그레이션 완료 (2025-01-27)

✅ **모든 주요 l10n 파일의 LocalizedRichText 구현 완료**
✅ **레거시 to_korean/to_english 메서드 제거 (contract.rs, event.rs, monster_skill.rs, tower_skill.rs)**
✅ **컴파일 성공 (0.11s)**

### 남은 선택사항 작업

- tower.rs, ui.rs, quest.rs, upgrade_board.rs의 미사용 to_korean/to_english 메서드 정리 (선택)
- effect.rs의 완전한 builder 구현 (선택, 692줄)
- rich_text_helpers.rs의 String 함수 정리 (선택, ~15개 함수)

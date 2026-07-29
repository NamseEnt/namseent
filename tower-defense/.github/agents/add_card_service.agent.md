---
name: add_card_service
description: Tower Defense 게임의 새로운 CardService를 추가하는 전문 에이전트. 사용자가 제공하는 thumbnail 이미지, 카드 서비스 이름, 효과 설명을 바탕으로 완전한 구현을 수행하며, 모든 필수 작업을 빠짐없이 처리. Namui/Rust 규칙, localization, AGENTS.md, userMemory/rust.md, CLAUDE.md를 철저히 준수.
argument-hint: thumbnail 이미지 (asset/image/thumbnail/*.png), 서비스 이름 (EN/KR), 효과 상세 설명 (선택 카드 조건, 적용 효과 e.g. damage bonus, card add/remove/enhance)
# tools: ['vscode', 'execute', 'read', 'agent', 'edit', 'search', 'web', 'todo', 'memory']
---

## Agent Operation Rules

- **Input**: thumbnail 이미지 파일 (confirm with list_dir on asset/image/thumbnail/), 서비스 이름 (EN/KR e.g. "Eraser"/"지우개"), 효과 상세 설명 (conditions like count=1/Any filter, effect e.g. remove 1 card from deck).
- **Essential Tasks (must complete all without omission)**:
  1. AGENTS.md & rust.md rules 준수 (render order, ctx ownership no E0382, table_no_clip, memoized_text for UI text, l10n structure no hardcode, EnumDiscriminants without duplicate derives, cast parens, non-move memo closures).
  2. New behavior file `src/game_state/card_service/behavior/<lowercase>.rs` 생성: struct (unit for simple remove or with fields); new()/into_card_service(); impl: key(); acquire() (lang-matched title.to_string(), CardSelectionState::new with Step{count:1, filter:Any/Face/Or(Ranks)}, set_modal(DeckModal)); select_cards() (loop selected_card_ids; for remove: GameStateAction::ModifyDeck(DeckEdit::Remove { card_ids }); for enhance: DeckEdit::Enhance with DeckEnhance+DeckEditChange e.g. AddDamageBonusPct/SetSuit; import appropriately); heuristic_best_selection() (e.g. for remove: sort low rank first take(1) with comment; for synergy filter+sort; uses deck.all_cards()); thumbnail exact with ::ERASER etc + STICKER_THUMBNAIL_STROKE; l10n_name/description (match locale.language, builder.static_text(EN/KR from desc)); tooltip default.
  3. behavior.rs 업데이트: mod add, enum variant add with derives (State, PartialEq etc), CardServiceDiscriminants::definition() match arm add, use if needed.
  4. behavior file에 DEFINITION const + generate fn + appropriate Rarity add.
  5. Thumbnail: list_dir(thumbnail/) to check; if missing use run_in_terminal cp; ensure UPPER_SNAKE const matches png (ERASER for eraser.png, auto-included).
  6. Generation auto via discriminant (confirm with candidate_table read); simulator/strategies (heuristic works); stats.rs (add name to SHOP_UPGRADE_NAMES); l10n/\* ONLY if new Word variant (rare) or generic update; config/upgrades.rs ONLY for damage services (add to vec with weight); tooltip if overriding.
  7. run_in_terminal: `cargo check`; `cargo test --quiet`; simulator e.g. `cargo run --bin td_simulator -- --runs=2 --max-turns=15 --seed=1` to test new service without long execution.
  8. CLAUDE.md: simple/minimal changes, explicit assumptions, surgical (only your changes), goal-driven with tests.
- **Localization**: behaviors' l10n_name/description (match+static_text) follows l10n structure; update l10n/card_service.rs/word/name.rs/word/description.rs/event.rs ONLY for new Word variant or if effect requires generic text change (memoized_text() required in shop/hand/UI components per AGENTS.md).
- **Common Pitfalls (from eraser experience)**: token-heavy from many sequential small reads (fix: parallel large reads + grep first); remove services required extra modify_deck.rs research (DeckEdit::Remove not in enhance examples); heuristic sort/cast/comments; ensure use super::\* + exact imports (DeckEdit, GameStateAction); acquire title per lang; multi_replace with precise 3-5 line contexts to prevent mismatch; always read current before edit (context reminder); no l10n change needed usually; test modal/filter/heuristic in simulator.
- **Output**: No codeblocks to user; use edit/create_file/multi_replace/manage_todo_list/read/grep/run_in_terminal tools ONLY. Final summary in Korean after ALL todos + 😊 per CLAUDE.md.

This agent ensures bug-free, convention-compliant card service additions following all project guidelines. Enhanced post-eraser to reduce tokens, specify remove/enhance patterns, verification cmds, and edit best practices.

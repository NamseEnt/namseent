# 파티클 백그라운드 워커 아이디어

## 배경

WASM 메인 스레드에서 `Atomics.wait`를 사용할 수 없어서, tokio 태스크를 깨우는 모든 방식(`tokio::sync::mpsc`, `futures::channel::mpsc`, `rayon::spawn` 등)이 메인 스레드에서 호출 시 문제가 됨.

`std::sync::mpsc::Sender::send()`만 안전 (내부적으로 `Atomics.notify`만 사용).

## 구조

```
메인 스레드                         워커 스레드 (1개)
  emitter.tick(now, dt)              loop {
    → msg_tx.send(...)                 msgs = rx.recv() + drain
    (Atomics.notify만 사용)            rayon::scope(|s| {
                                         for work in msgs {
  emitter.spawn(p)                         s.spawn(|_| work());
    → msg_tx.send(...)                 }
                                       });  // 에미터간 병렬
                                     }
```

## 핵심

- 워커 스레드 1개가 `recv()`로 블로킹 대기, 메시지 오면 rayon으로 처리
- `rayon::spawn/scope`는 워커 스레드에서 호출하므로 Atomics.wait 안전
- 명시적 wake 없이 `send()` → `recv()` 자체가 깨우는 역할
- 에미터별 다른 dt 지원 가능 (각 에미터가 자기 채널로 Tick 메시지 수신)

## 여러 `P` 타입 처리

클로저 등록 방식으로 해결:
- 각 `Emitter<P>`가 초기화 시 타입을 캡처한 클로저를 워커에 등록
- 워커는 `Vec<Box<dyn FnMut(Instant, Duration) + Send>>`를 소유
- spawn은 에미터별 `Sender<P>` (타입 안전), tick은 글로벌 채널

## 수치 참고

- 파티클 최대 ~10만개
- 단일 스레드 처리 시 최악 케이스 수십ms (프레임 초과)
- rayon 병렬처리 필수
- rayon dispatch+join 오버헤드: ~15-20μs per emitter

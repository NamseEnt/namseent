# Hooks v2

여러번 Hooks 시스템을 만들어보면서 느낀 점들을 개선한 v2입니다.

# Hooks v2의 핵심 시스템

## Sig(Signal)

Hooks v2는 Signal을 사용합니다. Solid.js의 Signal과 동일합니다.

Signal이 짧으면서도 긴 느낌이 있어서, Sig라고 줄여서 쓰도록 합니다. Vector를 Vec이라고 부르고, Reference를 Ref라고 쓰는 것과 동일합니다.

v1에서는 데이터의 PartialEq를 이용해서 업데이트 여부를 확인해야했습니다. 비교를 위해 데이터를 저장하기도 해야했습니다.
예를 들어, use_effect는 deps를 직접 나열해야만 했습니다. 명시적이라고 볼 수도 있겠지만, 실수하기 좋습니다. DX(Developer eXperience)도 별로 좋지 못합니다.

v2에서는 Sig가 변경을 시도했는지 여부를 확인하고, 그것을 바탕으로 데이터를 업데이트합니다. 비교를 하지 않기 때문에 PartialEq는 없어도 됩니다.
Sig는 Deref하는 타이밍을 신경씁니다. use_effect를 예로 들면, effect 내에서 Sig를 Deref하면 '이 이펙트에서는 이 시그널을 사용하는구나!' 라고 알아채며, 그 정보(SigId)를 저장합니다.

set_state등을 이용해서 Sig를 변경할 수 있습니다. Sig가 변경되면 시스템은 전체 재 렌더링을 진행합니다. 그러면서 변경된 Sig의 Id를 기억하여, 그것과 관련된 hooks들을 실행합니다. 이번에 변경된 Sig를 쓰지 않은 hooks는 실행하지 않습니다.

예를 들면 다음과 같습니다,

```rust
let (a, set_a) = ctx.use_state(|| 0);

ctx.use_effect("When 'a' changed", || {
    println!("a: {}", *a);  // <- a를 Deref(*)했기 때문에, 이 use_effect는 'a'라는 Sig를 사용했다는 것을 저장합니다.
});                         //    그래서 'a'가 set_a 로 변경되면 이 effect는 재실행됩니다.
                            //    'a'가 변경되지 않았으면, 이 effect는 실행되지 않습니다.
```

## 무조건 재렌더

Hooks v2는 부분 재렌더링 기능을 가지고 있지 않습니다. 이유는 단순합니다: 그렇게 하면 더 단순하고 안전하게 코드를 짤 수 있기 때문입니다.
앞으로 점점 최적화가 될 수도 있기 때문에, 영원히 부분 재렌더링 기능이 없을 것이라고는 말씀드릴 수 없습니다. 다만, 현재는 성능의 이슈가 심하게 나타나지 않는 이상, 지금의 심플한 코드 형태를 가져가는데 집중할 것입니다.

## Hooks 10계명

1. `pub enum Event` 를 애용해라.

```rust
#[namui::component]
pub struct MyComponent<'a> {
    ...
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event {
    ...
}
```

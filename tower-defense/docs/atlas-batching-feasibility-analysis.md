# drawAtlas 자동 배칭 가능성 심층 분석

## 1. 배경

namui 프레임워크에서 파티클 렌더링 최적화를 위해 Skia의 `drawAtlas`를 활용하는 방안을 검토한다.
현재 파티클 하나당 개별 draw call이 발생하며, `drawAtlas`는 동일 텍스처의 여러 스프라이트를 단일 draw call로 처리할 수 있다.

이 문서는 drawer 레벨에서 자동으로 배칭하는 방안(방안 2)의 기술적 가능성을 심층 분석한다.

## 2. drawAtlas API 명세

### 2.1 skia-safe 지원 여부

skia-safe 0.84.0에서 `draw_atlas` 메서드가 Canvas에 노출되어 있음을 확인하였다.

### 2.2 API 시그니처

```
canvas.draw_atlas(
    atlas: &Image,           // 소스 텍스처 (1장)
    xform: &[RSXform],       // 스프라이트별 변환 (회전 + 균일스케일 + 이동)
    tex: &[Rect],            // 스프라이트별 소스 영역
    colors: Option<&[Color]>, // 스프라이트별 색상 모듈레이션 (선택)
    mode: BlendMode,         // colors 적용 시 블렌드 모드
    sampling: SamplingOptions,
    cull_rect: Option<&Rect>,
    paint: Option<&Paint>,   // 전체 스프라이트에 공통 적용
)
```

### 2.3 RSXform의 제약

RSXform은 `(fSCos, fSSin, fTx, fTy)` 4개의 float로 구성되며, 행렬로 표현하면:

```
| fSCos  -fSSin  fTx |
| fSSin   fSCos  fTy |
|   0       0      1 |
```

**지원하는 변환:**
- 회전 (rotation)
- 균일 스케일 (uniform scale) - X와 Y에 동일한 스케일
- 이동 (translation)

**지원하지 않는 변환:**
- 비균일 스케일 (non-uniform scale, sx != sy)
- 기울이기 (skew/shear)
- 임의의 2x3 또는 3x3 행렬

### 2.4 per-sprite color

- `colors` 배열로 스프라이트별 색상/투명도를 개별 지정할 수 있다.
- alpha 채널을 통한 per-sprite opacity 구현이 가능하다.
- 이는 파티클의 페이드아웃 효과에 활용할 수 있다.

### 2.5 paint 적용 범위

- `paint` 파라미터는 **모든 스프라이트에 동일하게** 적용된다.
- per-sprite paint 변형은 불가능하다.
- `maskFilter`, `pathEffect`는 무시된다.

## 3. 현재 렌더링 트리 구조 분석

### 3.1 파티클 시스템의 트리 생성 방식

`particle/src/lib.rs:127-129`:
```rust
for particle in state.particles.iter() {
    ctx.add(particle.render());
}
```

`ctx.add()`는 내부적으로 `RtContainer`의 `boxcar::Vec`에 push한다.
모든 파티클의 RenderingTree는 하나의 `Children` 노드 안에 **연속된 형제(sibling)** 로 들어간다.
다른 게임 요소가 파티클 사이에 끼어들지는 않는다.

### 3.2 하지만 파티클 타입이 섞여 있다

단일 `FieldParticleSystem` 안에 `FieldParticle` enum의 모든 variant가 혼재한다.
emitter가 emit한 순서대로 단일 `Vec<FieldParticle>`에 쌓이므로, 렌더링 순서는 다음과 같이 될 수 있다:

```
[Projectile(Image), BurningTrail(Path), Projectile(Image), EmberSpark(Path), Projectile(Image)]
```

같은 이미지를 쓰는 파티클끼리 연속으로 나올 수도 있고, 중간에 Path/Text 기반 파티클이 끼어들 수도 있다.

### 3.3 emitter별 파티클 타입 생성 패턴

| Emitter | 생성하는 파티클 타입 | 렌더링 방식 |
|---------|---------------------|-------------|
| TempParticleEmitter | 임의의 FieldParticle | 혼합 |
| DamageTextEmitter | DamageText | Text |
| MonsterDeathEmitter | MonsterDeath(MonsterSoul) | Image |
| MonsterStatusEffectEmitter | Icon (1-2개) | Image (복합) |
| BurningTrailEmitter | BurningTrail + EmberSpark | Path + Path |
| TrashBurstEmitter | Trash (8개) | Image |
| TrashBounceEmitter | Trash | Image |
| TrashRainEmitter | Trash | Image |
| ProjectileParticleEmitter | Projectile | Image |
| LaserBeamEmitter | LaserLine + LightningBolt + BlueDotSpark | Path + Path + Path |

### 3.4 각 파티클 variant의 렌더링 트리 형태

**Image 기반 (5종):**

| Variant | 트리 구조 | ImageFit | Paint |
|---------|-----------|----------|-------|
| Projectile | `Translate → Rotate → Translate → Image` | Contain | None |
| Trash | `Translate → Rotate → Image` | Contain | None |
| MonsterCorpse | `Translate → Rotate → Scale → Image` | Contain | opacity paint |
| MonsterSoul | `Translate → Rotate → Translate → Scale → Image` | None | opacity paint |
| Icon | `Translate → Rotate → Translate → Children([Image×N, Image, Path])` | Contain | opacity paint |

**Path 기반 (8종):** BurningTrail, EmberSpark, BlueDotSpark, LaserBeam, LaserLine, InstantEmit, InstantHit, LightningBolt

**Text 기반 (1종):** DamageText

### 3.5 drawer의 순회 순서

`namui-drawer/src/draw/mod.rs:24`:
```rust
for child in children.iter().rev() {
    draw_internal(skia, child, rendering_tree_draw_context);
}
```

Children은 **역순으로 순회**된다 (첫 번째 child가 가장 마지막에 그려져서 앞에 보임).
배칭 시 이 순서를 존중해야 한다.

## 4. 배칭 전제조건 분석

### 4.1 drawAtlas로 배칭하기 위한 필수 조건

두 개의 `ImageDrawCommand`를 하나의 drawAtlas 호출로 묶으려면:

1. **동일한 Image** - `Image`는 `usize` id의 wrapper로 `PartialEq`, `Eq`, `Hash`를 derive한다. 비교 비용이 O(1)이다.

2. **동일한 Paint (image shader 제외)** - `Paint`는 `PartialEq`, `Eq`, `Hash`를 derive한다. 모든 중첩 타입(`Color`, `Shader`, `ColorFilter`, `BlendMode`, `MaskFilter`, `ImageFilter`)이 비교 가능하다. 다만 drawAtlas에서 paint는 전체 스프라이트에 공통 적용되므로, paint가 다르면 배칭 불가.

3. **동일한 ImageFit에서 동일한 src_rect** - ImageFit::Contain은 항상 전체 이미지를 src_rect로 쓰므로 같은 이미지라면 src_rect가 동일하다. ImageFit::None이나 Cover는 dest_rect에 따라 src_rect가 달라질 수 있다.

4. **RSXform으로 표현 가능한 누적 변환** - 회전 + 균일 스케일 + 이동만 가능. 비균일 스케일이 있으면 배칭 불가.

5. **연속된(consecutive) draw call** - Z-order를 보존하려면 중간에 다른 draw call이 없어야 한다.

### 4.2 Paint 호환성 상세 분석

현재 파티클들의 paint 사용 패턴:

| 파티클 | Paint | 배칭 시 처리 방법 |
|--------|-------|-------------------|
| Projectile | `None` → White | 동일. colors[] 필요 없음 |
| Trash | `None` → White | 동일. colors[] 필요 없음 |
| MonsterCorpse | `Some(Paint { color: WHITE.with_alpha(a) })` | alpha가 개별적 → colors[]로 분리 필요 |
| MonsterSoul | `Some(Paint { color: WHITE.with_alpha(a) })` | alpha가 개별적 → colors[]로 분리 필요 |
| Icon | `None` 또는 `Some(Paint { color: (1,1,1,opacity) })` | alpha 개별적 + 복합 이미지 |

**핵심**: opacity만 다른 경우, paint에서 color를 분리하여 drawAtlas의 `colors[]` 파라미터로 넘기면 배칭이 가능하다. base paint는 `Paint::new(Color::WHITE)`로 통일.

### 4.3 RSXform 변환 호환성

현재 파티클들의 변환 스택:

| 파티클 | 변환 스택 | RSXform 호환 |
|--------|-----------|--------------|
| Projectile | translate + rotate + translate | O (회전 + 이동) |
| Trash | translate + rotate | O (회전 + 이동) |
| MonsterCorpse | translate + rotate + scale(s, s) | O (회전 + 균일스케일 + 이동) |
| MonsterSoul | translate + rotate + translate + scale(s, s) | O (현재 코드에서 `Xy::single(scale_v)` 사용, 균일스케일) |
| Icon | translate + rotate + translate + (복합 이미지) | X (복합 이미지 구조) |

MonsterSoulParticle의 `scale` 필드 타입이 `Xy<f32>`이므로 이론적으로 비균일 스케일이 가능하지만, 현재 코드는 `Xy::single(scale_v)`로 항상 균일 스케일을 사용한다.

### 4.4 Icon 파티클의 특수성

Icon의 `to_rendering_tree()`는 단일 이미지가 아니라 **복합 구조**를 생성한다:

```
Children([
    Image(attribute_0),  // 속성 아이콘 0
    Image(attribute_1),  // 속성 아이콘 1
    ...
    Image(main_icon),    // 메인 아이콘
    Path(transparent_rect), // 레이아웃용 투명 사각형
])
```

서로 다른 Image ID를 가진 여러 이미지 + Path로 구성되므로, **단일 drawAtlas 호출로 배칭 불가**.
Icon 파티클은 자동 배칭 대상에서 제외해야 한다.

## 5. drawer 레벨 자동 배칭 구현 전략

### 5.1 핵심 아이디어

`draw_internal` 함수에서 ImageDrawCommand를 만나면 즉시 그리지 않고 버퍼에 쌓다가,
배칭이 깨지는 시점에 한꺼번에 `drawAtlas`로 flush한다.

### 5.2 배칭 버퍼 구조

```rust
struct PendingAtlasBatch {
    image: Image,
    base_paint: Option<Paint>,  // image shader 제외, color는 WHITE로 통일
    fit: ImageFit,
    entries: Vec<BatchEntry>,
}

struct BatchEntry {
    src_rect: Rect<Px>,
    accumulated_matrix: TransformMatrix,  // canvas의 누적 변환
    color: Color,                         // per-sprite color (opacity 포함)
}
```

### 5.3 배칭 판정 로직

`ImageDrawCommand`를 만났을 때:

```
1. canvas.get_matrix()로 현재 누적 변환 행렬을 읽는다
2. 현재 pending batch가 있는지 확인한다
3. 있다면 다음을 비교한다:
   a. image.id == pending.image.id?
   b. paint (color 제외) == pending.base_paint?
   c. fit == pending.fit?
   d. 누적 행렬이 RSXform으로 분해 가능한가?
4. 모두 만족하면 pending batch에 추가
5. 하나라도 실패하면:
   a. 기존 pending batch를 flush (drawAtlas 호출)
   b. 새 pending batch 시작 (또는 개별 draw)
```

비-Image DrawCommand(Path, Text)를 만나면:
```
1. pending batch가 있으면 flush
2. 해당 command를 개별 실행
```

### 5.4 누적 행렬 → RSXform 분해

```rust
fn try_decompose_to_rsxform(matrix: &TransformMatrix) -> Option<RSXform> {
    // matrix = [[a, b, tx], [c, d, ty]]
    // RSXform 조건: a == d, b == -c (회전 + 균일스케일)
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[1][0];
    let d = matrix[1][1];
    let tx = matrix[0][2];
    let ty = matrix[1][2];

    let eps = 1e-4;
    if (a - d).abs() > eps || (b + c).abs() > eps {
        return None;  // 비균일 스케일 또는 skew → 배칭 불가
    }

    Some(RSXform {
        scos: a,   // scale * cos(theta)
        ssin: c,   // scale * sin(theta)
        tx,
        ty,
    })
}
```

### 5.5 src_rect 계산

drawAtlas의 `tex[]` 배열에는 아틀라스 이미지 내 소스 영역을 넣어야 한다.
현재 파티클이 사용하는 ImageFit 기준:

- **ImageFit::Contain**: src_rect = 전체 이미지 `(0, 0, image.width, image.height)`. 같은 이미지면 항상 동일.
- **ImageFit::None**: src_rect가 dest_rect 크기에 따라 달라질 수 있다. 단, 이미지가 dest_rect보다 작으면 전체 이미지가 src_rect.

따라서 배칭 시 src_rect 동일성은 ImageFit과 이미지 크기에 의존한다.

실질적으로 같은 종류의 파티클(예: Projectile)은 모두 같은 ImageFit::Contain + 같은 Image를 쓰므로 src_rect이 항상 동일하다.
그러나 drawAtlas의 `tex[]`은 per-sprite로 다른 src_rect을 허용하므로, src_rect이 달라도 같은 이미지라면 배칭 가능하다.

### 5.6 flush 조건의 dest_rect 처리

현재 `draw_image` 구현에서 dest_rect/src_rect 비율로 스케일을 계산한다:

```rust
TransformMatrix::from_translate(dest_rect.x(), dest_rect.y())
    * TransformMatrix::from_scale(
        dest_rect.width() / src_rect.width(),
        dest_rect.height() / src_rect.height(),
    )
    * TransformMatrix::from_translate(-src_rect.x(), -src_rect.y())
```

**주의**: `dest_rect.width() / src_rect.width()`와 `dest_rect.height() / src_rect.height()`가 다르면 비균일 스케일이 된다.
ImageFit::Contain에서 aspect ratio가 맞으면 균일 스케일이지만, 이미지와 rect의 aspect ratio가 다르면 letterbox로 인한 offset이 추가될 뿐이다.

실제로 `get_src_dest_rects_in_fit`에서 Contain은 aspect ratio를 맞춘 dest_rect를 반환하므로, **src/dest 비율이 항상 균일**하다. 이는 RSXform 호환이다.

## 6. 실제 파티클 시나리오 시뮬레이션

### 6.1 최적 시나리오: ProjectileParticle 500개

동일한 ProjectileParticleEmitter에서 같은 종류의 투사체 500개가 생성된 경우:

```
[Projectile(arrow), Projectile(arrow), ..., Projectile(arrow)]
```

- 모두 같은 Image (arrow)
- 모두 같은 ImageFit::Contain
- 모두 paint: None
- 변환: translate + rotate + translate → RSXform 호환

**결과: 500개 → 1회 drawAtlas 호출. 최대 효과.**

### 6.2 혼합 시나리오: 여러 emitter의 파티클이 섞인 경우

```
[Projectile(arrow), Projectile(arrow), BurningTrail(Path), EmberSpark(Path),
 Projectile(arrow), Trash(can), Trash(can), Trash(can)]
```

배칭 흐름:
1. `Projectile(arrow)` × 2 → batch 시작 (image=arrow)
2. `BurningTrail(Path)` → **flush batch** (2개 drawAtlas) → Path 개별 draw
3. `EmberSpark(Path)` → Path 개별 draw
4. `Projectile(arrow)` × 1 → 새 batch (image=arrow)
5. `Trash(can)` → **flush batch** (1개 drawAtlas, 이건 그냥 개별 draw와 동일) → 새 batch (image=can)
6. `Trash(can)` × 2 → batch에 추가
7. 끝 → **flush batch** (3개 drawAtlas)

**결과: 8개 파티클 → 2회 drawAtlas + 2회 개별 Path draw = 4회 draw call** (기존 8회에서 감소)

### 6.3 최악 시나리오: 타입이 완전히 교대

```
[Projectile(arrow), BurningTrail(Path), Projectile(arrow), BurningTrail(Path)]
```

배칭이 연속으로 1개씩만 쌓이므로 사실상 배칭 효과 없음.

**결과: 4개 파티클 → 4회 draw call (기존과 동일)**

### 6.4 현실적 시나리오 평가

타워 디펜스에서 파티클이 대량 발생하는 상황:
- 여러 타워가 동시에 투사체를 발사 → 같은 종류의 ProjectileParticle이 대량 연속 생성
- Trash 파티클 burst → TrashBurstEmitter가 한 번에 8개 동일 Image 파티클 생성
- MonsterDeath → 동일 Image(MONSTER_SOUL) 파티클

파티클은 emission 순서대로 Vec에 쌓이므로, **같은 emitter에서 나온 같은 타입의 파티클은 자연스럽게 연속 배치**된다.
다만 서로 다른 emitter의 파티클이 시간 순으로 interleave되면 연속성이 깨질 수 있다.

## 7. 배칭 효과를 극대화하기 위한 추가 방안

### 7.1 파티클 Vec 정렬 (선택적 최적화)

파티클 시스템에서 렌더링 전에 파티클을 타입별로 정렬하면 연속성이 보장된다.
파티클 간 Z-order가 중요하지 않은 경우(대부분의 파티클 효과) 이 방법이 유효하다.

```rust
// 렌더링 전 정렬
particles.sort_by_key(|p| std::mem::discriminant(p));
```

**장점**: 배칭 연속성 보장
**단점**: 정렬 비용 O(n log n), 파티클 간 원래 Z-order 변경

### 7.2 discriminant 기반 빠른 판정

enum discriminant를 batch key의 일부로 사용하면, 트리를 순회하지 않고도 배칭 가능성을 빠르게 판정할 수 있다.
단, 이는 drawer 레벨이 아닌 파티클 시스템 레벨의 최적화이므로 방안 2의 "투명한 최적화" 원칙과 충돌한다.

### 7.3 batch 최소 크기 설정

1개짜리 batch는 drawAtlas 호출의 오버헤드가 개별 draw보다 클 수 있다.
최소 batch 크기(예: 4개)를 두고, 그 이하면 개별 draw를 유지하는 방안.

## 8. 구현 복잡도 평가

### 8.1 필요한 변경 사항

1. **SkCanvas trait 확장**: `draw_atlas` 메서드 추가
2. **NativeSkia canvas 구현**: skia_safe::Canvas::draw_atlas 호출 연결
3. **drawer의 draw_internal 수정**: 배칭 버퍼 로직 추가
4. **RSXform 분해 함수**: 누적 행렬 → RSXform 변환
5. **Paint 비교 로직**: image shader를 제외한 paint 비교 (또는 color 분리)

### 8.2 영향 범위

- `namui/skia/src/traits.rs` - trait에 `draw_atlas` 추가
- `namui/skia/src/canvas.rs` - impl 추가
- `namui/namui-drawer/src/draw/mod.rs` - 배칭 로직 추가
- `namui/namui-drawer/src/draw/image.rs` - src_rect 계산 로직 재사용

파티클 시스템 코드, 게임 코드는 변경 불필요.

### 8.3 paint 비교의 미묘한 문제

현재 `draw_image`에서 paint를 다음과 같이 변환한다:
```rust
let mut paint = paint.clone().unwrap_or(Paint::new(Color::WHITE));
let image_shader = image.get_default_shader();
paint = paint.set_shader(image_shader);
```

배칭 판정 시에는 **변환 전** paint를 비교해야 한다. 즉:
- `None`과 `Some(Paint::new(Color::WHITE))`는 동등하게 취급해야 한다.
- color의 alpha만 다르고 나머지가 같은 경우, color를 per-sprite color로 분리하고 base paint로 통일할 수 있다.

이를 위한 정규화 로직이 필요하다:

```rust
fn normalize_paint_for_batching(paint: &Option<Paint>) -> (Paint, Color) {
    let paint = paint.clone().unwrap_or(Paint::new(Color::WHITE));
    let color = paint.color;
    let base = paint.set_color(Color::WHITE);  // color를 분리
    (base, color)
}
```

두 ImageDrawCommand의 `base`가 동일하면 배칭 가능하고, `color`는 per-sprite colors[]에 넣는다.

## 9. 결론

### 9.1 가능한가?

**가능하다.** 기술적 장벽은 없다.

- `Paint`는 `PartialEq`, `Eq`, `Hash`를 지원한다.
- `Image`는 `usize` id 비교로 O(1)이다.
- skia-safe 0.84.0에서 `draw_atlas`가 지원된다.
- 누적 행렬 → RSXform 분해는 간단한 수학이다.
- per-sprite color로 opacity 차이를 처리할 수 있다.

### 9.2 효과적인가?

**상황에 따라 다르다.**

| 시나리오 | 배칭 효과 |
|---------|-----------|
| 같은 종류 투사체 대량 발사 | 매우 높음 (N → 1) |
| TrashBurst (8개 동시 생성) | 높음 (8 → 1) |
| MonsterDeath 대량 발생 | 높음 (N → 1) |
| 여러 타입 파티클 혼재 | 중간 (연속 구간만 배칭) |
| 타입이 매 프레임 교대 | 없음 |
| Icon 파티클 | 배칭 불가 (복합 구조) |

### 9.3 가장 큰 리스크

1. **연속성 미보장**: 파티클 타입이 섞이면 배칭 효과가 크게 떨어진다. 이를 해결하려면 파티클 정렬이 필요한데, 이는 "투명한 최적화"의 범위를 벗어난다.

2. **Icon 파티클 미지원**: 복합 이미지 구조라 drawAtlas로 처리할 수 없다. 전체 파티클 중 Image 기반 5종 중 1종(Icon)이 제외된다.

3. **ImageFit::None의 src_rect 변동**: MonsterSoulParticle이 ImageFit::None을 사용한다. 하지만 drawAtlas의 `tex[]`이 per-sprite src_rect을 지원하므로 이것 자체가 배칭을 막지는 않는다. 단, ImageFit::None은 dest_rect 크기에 따라 src_rect이 달라지므로 per-sprite tex[]를 개별 계산해야 한다.

4. **직렬화 비용 미감소**: RenderingTree 노드 수는 동일하게 WASM FFI를 통과한다. 이 비용이 병목이라면 drawer 레벨 배칭만으로는 부족하다.

### 9.4 권장 사항

drawer 레벨 자동 배칭은 구현 가능하며, 특히 대량의 동일 타입 파티클에서 효과가 크다.
다만 다음을 고려해야 한다:

- 단독으로 충분한 최적화가 되지 않을 수 있으므로, 효과 측정 후 필요시 파티클 정렬이나 명시적 Atlas API 추가를 검토한다.
- Icon 파티클은 별도 최적화 경로가 필요하다.
- 직렬화 비용이 병목이 되는 경우, RenderingTree에 Atlas 노드를 추가하여 노드 수 자체를 줄이는 접근이 필요하다.

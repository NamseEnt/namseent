# Authoritative combat number policy

Authoritative amounts use `i64` raw values at `AMOUNT_SCALE = 1_000` (one game
unit is 1,000 raw units). `Health`, `Damage`, and `Shield` are distinct
newtypes. Ratios use `RATIO_SCALE = 1_000_000` (`1.0` is 1,000,000 raw
units), and `ClearRate` is a bounded ratio.

Decimal config values are converted once at deserialization with nearest
integer rounding, ties away from zero. Runtime amount and ratio multiplication
uses `i128`; a stack of ratios is sorted, reduced, multiplied, and rounded once
at the final division with the same nearest/ties-away rule. This keeps the
result independent of factor insertion order and avoids intermediate rounding.
Stage-wide modifier stacks retain their individual factors; HP, incoming
damage, and rewards apply the complete stack directly instead of first rounding
it into another ratio.

Amounts clamp negative inputs to zero. Add/subtract operations are saturating;
overflow clamps to the type maximum. Conversions from count-sized integers also
saturate instead of wrapping. Zero damage is a no-op, while a positive raw value
smaller than one display unit is retained. Rendering, statistics, and other I/O
convert authoritative values to `f32` only at their boundary.

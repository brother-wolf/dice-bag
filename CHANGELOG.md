# Changelog

---

## 0.2.0: Breaking changes

---

#### Fix for `panic`, requires change to return type

Version 0.1.x

- returns a `RollResult`
- the `Dice::roll` method causes a panic in the rand crate if the number of sides was less than 1:

```rust
dice::Dice::roll(1, 0, -2);
```

causes:

```
thread '...' panicked at .../.cargo/registry/src/index.crates.io/rand-0.8.5/src/rng.rs:134:9:
cannot sample empty range
```

Version 0.2.x

- returns a `Result<RollResult, String>`
- The `Dice::roll` method now returns a result-error if the number of sides is 1 or less (not a valid number of sides for a die of any type).
- the `roll(dice: &str) -> AllRollResults` handles the `Result<RollResult, String>` by discarding Errors.

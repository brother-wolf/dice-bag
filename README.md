# Dice-Bag (Rust Lib)

This library uses standard RPG notation for generating dice rolls producing a total but also a break down of all the rolls should they be needed.

To import it use:

```rust
// Cargo.toml

[dependencies]
dice-bag = "0.1"
```

It has a main entry point of:

```rust
use dice::dice;

dice::roll("3d10+1")
```

produces a structure:

```rust
AllRollResults {
    pub total: isize,
    pub rolled: Vec<RollResult>,
}

RollResult {
    pub total: isize,
    pub rolled: DiceRolls,
    pub selected_rolls: DiceRolls,
    pub sides: usize,
    pub num_dice: usize,
    pub modifier: isize,
}
```
e.g. for 3d10+1 rolling 6,5,2
```
{
    total: 14,
    rolled: [
        {
            total: 14
            rolled: [ 6, 5, 2]
            selected_rolls: [ 6, 5, 2]
            sides: 6
            num_dice: 3
            modifier: 1
        }
    ]
}
```

Dice can also be chained together:

```rust
dice::roll("2d8 4d4+8")
```

where a result might look like this:

```
{
    total: 25,
    rolled: [
        {
            total: 9
            rolled: [ 8, 1]
            selected_rolls: [ 8, 1]
            sides: 8
            num_dice: 2
            modifier: 0
        },
        {
            total: 16
            rolled: [ 3, 2, 2, 1]
            selected_rolls: [ 3, 2, 2, 1]
            sides: 4
            num_dice: 4
            modifier: 8
        }
    ]
}
```

You can directly use the roll method:

```rust
dice::Dice::roll(3, 6, -2);
```

which produces a single `RollResult`, for example:

```
{
    total: 6
    rolled: [ 5, 2, 1 ]
    selected_rolls: [ 5, 2, 1 ]
    sides: 6
    num_dice: 3
    modifier: -2
}
```

Dice roll results are all sorted highest to lowest.


Finally the `Display` for the results is overwritten to produce a parsable slug:

```rust
dice::roll("3d6-2 2d4+1")).to_string();
```

could produce:

```
3d6-2,2d4+1:[4,2,1],[4,3]:13
```


```rust
dice::roll("2d10+3")).to_string();
```

could produce:

```
2d10+3:[10,1]:14
```

Simple derive macro to iterate over enum values.

Doesn't use traits, just adds single static function.

Works only on enums with unit variants.

```
#[derive(EnumIter, Debug)]
enum Test {
    A,
    B = 42,
    C,
}

for value in Test::iter() {
    println!("{value:?}")
}
```

### License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

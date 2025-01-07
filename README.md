![CI status](https://github.com/Dzuchun/not_too_many_arcseconds/actions/workflows/build.yml/badge.svg)
[![Documentation status](https://github.com/Dzuchun/not_too_many_arcseconds/actions/workflows/docs.yml/badge.svg)][docs]

This is silly joke, born from

```text
- K, ima nap for a bit
*PROCEEDS TO SHUT DOWN FOR u206265::MAX HOURS*
```

So yeah... it's just a 206265-bit unsigned integer. I have no fucking idea why would you need it.

Each number is, like, 25kiB long, so I feature-locked `Copy` implementation, in case you don't want to implicitly copy this thing (**you probably don't**).

## Implementation

Currently, this abomination is implemented in core Rust, i.e. no threads, no simd, no GPU, etc. This, and my complete lack of knowledge about actual efficient long arithmetic algorithms, made these numbers extremely slow. Like, it-takes-90s-to-compute-log10(MAX)-slow.

Yes, I **am** sorry.

## Examples

Idk, let's find a sum of first 100 numbers:

```rust
# use not_too_many_arcseconds::u206265;
let sum = (1..=100u8).map(u206265::from).sum::<u206265>();

assert_eq!(sum, 5050u32.into());
```

And then... 100!, because we can:

```rust
# use not_too_many_arcseconds::u206265;
let sum = (1..=100u8).map(u206265::from).product::<u206265>();

# let sum = core::hint::black_box(sum);

println!("{sum}");
```

I hope it's correct :idk:.

Takes, like, 23s to calculate on my setup, while regular two-line python takes about 20ms max.

## Why 206265?

It's an approximate number of arc seconds in a radian (both are angle-measuring units). Since [parsec](https://en.wikipedia.org/wiki/Parsec) (common astrophysics distance unit) is defined via concept of arc second, there are approximately 206265 a.u. (Earth-Sun distance) in it.

Exact value would be $180 / \pi \cdot 3600$.

**TL;DR**: No idea, just a funny number.

[docs]: https://dzuchun.github.io/not_too_many_arcseconds/not_too_many_arcseconds/index.html

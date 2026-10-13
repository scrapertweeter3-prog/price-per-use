# price-per-use

A small Rust CLI that answers the durable-goods question the sticker
price skips: what does one use of a thing cost once upkeep is in the
math, and how long until owning it beats renting it per use. Give it
a price, how many times a year the thing gets used, how many years it
lasts, and optionally a yearly upkeep line and a per-use rental price;
it prints the real per-use cost, the annual cost of ownership, and
the break-even point.

price-per-use is built and maintained by [Durable Picks](https://durablepicks.com),
a site that ranks buy-it-for-life gear by how it fails and what it
costs per decade.

## Install

Standard library only, no dependencies beyond a stable Rust toolchain:

    git clone https://github.com/scrapertweeter3-prog/price-per-use
    cd price-per-use
    cargo build --release

The binary lands at target/release/price-per-use.

## Usage

    price-per-use PRICE USES_PER_YEAR YEARS [UPKEEP_PER_YEAR RENT_PER_USE]

A $120 tool used 24 times a year for five years, $10 a year in upkeep,
against a rental at $8 a use:

    $ price-per-use 120 24 5 10 8
    cost per use (with upkeep): $1.42
    annual cost of ownership:   $34.00
    renting at $8.00/use breaks even at 0.6 years

At that usage rate each use costs $1.42, and renting the same job at
$8 stops making sense inside the first year.

## Usage rate is the lever

Same $120 tool, same five years, two owners:

    $ price-per-use 120 4 5 10 8
    cost per use (with upkeep): $8.50
    annual cost of ownership:   $34.00
    renting at $8.00/use breaks even at 3.8 years

    $ price-per-use 120 60 5 10 8
    cost per use (with upkeep): $0.57
    annual cost of ownership:   $34.00
    renting at $8.00/use breaks even at 0.2 years

The purchase never changed. Four uses a year makes it an $8.50-a-use
habit; sixty makes it $0.57. That gap is the whole rent-or-buy
decision, which is why the honest input is a use counter you keep,
not the one on the sales page.

## A case where renting holds

Not every durable purchase wins. A $320 tool used monthly over an
eight-year life, $25 a year in upkeep, against $4 a use to rent:

    $ price-per-use 320 12 8 25 4
    cost per use (with upkeep): $5.42
    annual cost of ownership:   $65.00
    renting at $4.00/use breaks even at 6.7 years

Break-even lands at 6.7 years of an eight-year life, so the rental
column wins for all but the tail. The tool prints the number either
way; it does not grade the purchase for you.

## Library

The crate builds as a library as well:

```rust
let cpu = price_per_use::cost_per_use_with_upkeep(120.0, 24.0, 5.0, 10.0);
assert!((cpu - 1.4166666666666667).abs() < 1e-9);
```

## Honest limits

Upkeep is a flat yearly line, the tool assumes a use count you really
keep rather than an aspirational one, and the rental comparison is
per-use only, so convenience, storage, and resale are left out on
purpose.

## License

MIT

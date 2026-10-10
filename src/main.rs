use std::env;
use std::process::exit;

fn usage() -> ! {
    eprintln!("usage: price-per-use PRICE USES_PER_YEAR YEARS [UPKEEP_PER_YEAR RENT_PER_USE]");
    eprintln!("  prints per-use cost, annual cost, and the rent break-even");
    exit(2);
}

fn main() {
    let a: Vec<f64> = env::args()
        .skip(1)
        .map(|s| s.parse::<f64>())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|_| usage());
    if !(a.len() == 3 || a.len() == 5) {
        usage();
    }
    let upkeep = if a.len() == 5 { a[3] } else { 0.0 };
    let rent = if a.len() == 5 { a[4] } else { 0.0 };
    let cpu = price_per_use::cost_per_use_with_upkeep(a[0], a[1], a[2], upkeep);
    let annual = price_per_use::annual_cost(a[0], a[2], upkeep);
    println!("cost per use (with upkeep): ${:.2}", cpu);
    println!("annual cost of ownership:   ${:.2}", annual);
    if rent > 0.0 {
        let be = price_per_use::break_even_years(a[0], a[1], rent);
        println!("renting at ${:.2}/use breaks even at {:.1} years", rent, be);
    }
}

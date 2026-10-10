//! Cost-per-use math for durable goods.
//!
//! A thing you buy once and use for years hides its real price inside the
//! number of times you reach for it. These functions spread a purchase over
//! its actual service life and compare it to renting or subscribing.

/// Cost per single use across the item's whole service life.
///
/// ```
/// let cpu = price_per_use::cost_per_use(120.0, 24.0, 5.0);
/// assert!((cpu - 1.0).abs() < 1e-9);
/// ```
pub fn cost_per_use(price: f64, uses_per_year: f64, years: f64) -> f64 {
    let total_uses = uses_per_year * years;
    if total_uses <= 0.0 {
        return f64::INFINITY;
    }
    price / total_uses
}

/// Annualized cost: what ownership actually costs per year.
///
/// ```
/// let a = price_per_use::annual_cost(120.0, 5.0, 10.0);
/// assert!((a - 34.0).abs() < 1e-9);
/// ```
pub fn annual_cost(price: f64, years: f64, upkeep_per_year: f64) -> f64 {
    if years <= 0.0 {
        return f64::INFINITY;
    }
    price / years + upkeep_per_year
}

/// Cost per use including upkeep, the honest comparison number.
///
/// ```
/// let c = price_per_use::cost_per_use_with_upkeep(120.0, 24.0, 5.0, 10.0);
/// assert!((c - 1.4166666666666667).abs() < 1e-9);
/// ```
pub fn cost_per_use_with_upkeep(
    price: f64,
    uses_per_year: f64,
    years: f64,
    upkeep_per_year: f64,
) -> f64 {
    let total_uses = uses_per_year * years;
    if total_uses <= 0.0 {
        return f64::INFINITY;
    }
    (price + upkeep_per_year * years) / total_uses
}

/// Years of service needed to beat renting at `rent_per_use` per use.
///
/// ```
/// let y = price_per_use::break_even_years(120.0, 24.0, 8.0);
/// assert!((y - 0.625).abs() < 1e-9);
/// ```
pub fn break_even_years(price: f64, uses_per_year: f64, rent_per_use: f64) -> f64 {
    if uses_per_year <= 0.0 || rent_per_use <= 0.0 {
        return f64::INFINITY;
    }
    price / (uses_per_year * rent_per_use)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_dollar_per_use() {
        assert!((cost_per_use(120.0, 24.0, 5.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn annual_with_upkeep() {
        assert!((annual_cost(120.0, 5.0, 10.0) - 34.0).abs() < 1e-9);
    }

    #[test]
    fn zero_uses_is_infinite() {
        assert!(cost_per_use(120.0, 0.0, 5.0).is_infinite());
    }

    #[test]
    fn break_even_six_months() {
        assert!((break_even_years(120.0, 24.0, 8.0) - 0.625).abs() < 1e-9);
    }
}

use itertools::Itertools;

pub fn factorize(mut number: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();

    let mut i = 2;
    while number % i == 0{
        number /= i;
        factors.push(i);
    }

    i = 3;
    while number > 1 {
        if number % i == 0 {
            number /= i;
            factors.push(i);
            i = 3;
            continue
        }
        i += 2;
    }

    factors
}

pub fn lcm(numbers: Vec<i64>) -> i64 {
    let factorization =
        numbers.iter()
        .map( |x| factorize(*x));

    let mut factors: Vec<(i64, u32)> = Vec::new();

    for factor_list in  factorization {
        let mut factor_occurrence: Vec<(i64, u32)> = Vec::new();
        for factor in factor_list {
            match factor_occurrence.iter().position(|x| x.0 == factor) {
                Some(x) => factor_occurrence[x].1 += 1,
                None => factor_occurrence.push((factor, 1)),
            }
        }
        factors.append(&mut factor_occurrence);
    }

    let mut most_repeated_factors: Vec<(i64, u32)> = Vec::new();

    for factor in factors {
        match most_repeated_factors.iter().position(|x| x.0 == factor.0) {
            Some(x) => if most_repeated_factors[x].1 < factor.1 { most_repeated_factors[x].1 = factor.1 },
            None => most_repeated_factors.push(factor),
        }
    }

    let mut lcm = 1;

    for factor in most_repeated_factors {
        lcm *= factor.0.pow(factor.1);
    }

    lcm
}

pub fn gcf(numbers: Vec<i64>) -> i64 {
    let factorization =
        numbers.iter()
            .map( |x| factorize(*x));

    let mut factors: Vec<(i64, u32)> = Vec::new();

    for factor_list in  factorization {
        let mut factor_occurrence: Vec<(i64, u32)> = Vec::new();
        for factor in factor_list {
            match factor_occurrence.iter().position(|x| x.0 == factor) {
                Some(x) => factor_occurrence[x].1 += 1,
                None => factor_occurrence.push((factor, 1)),
            }
        }
        factors.append(&mut factor_occurrence);
    }

    let mut least_repeated_factors: Vec<(i64, u32)> = Vec::new();
    let common_values_amount = factors.iter().counts_by(|x| x.0);

    factors.retain(
        |x|
            common_values_amount.iter()
        .filter(|x| *x.1 > (numbers.len() - 1))
        .map(|x| x.0)
        .contains(&&x.0)
    );

    for factor in factors {
        match least_repeated_factors.iter().position(|x| x.0 == factor.0) {
            Some(x) => if least_repeated_factors[x].1 > factor.1 { least_repeated_factors[x].1 = factor.1 },
            None =>  least_repeated_factors.push(factor),
        }
    }

    let mut gcf = 1;

    for factor in least_repeated_factors {
        gcf *= factor.0.pow(factor.1);
    }

    gcf
}

pub fn add_fraction(fractions: Vec<(i64, i64)>) -> (i64, i64) {
    let denominators: Vec<i64> = fractions.iter().map(|x| x.1).collect();

    let lcm = lcm(denominators);

    let mut result: (i64, i64) = (0, lcm);

    for fraction in fractions {
        result.0 += fraction.0 * (lcm / fraction.1);
    }

    result
}

pub fn subtract_fraction(fractions: Vec<(i64, i64)>) -> (i64, i64) {
    let denominators: Vec<i64> = fractions.iter().map(|x| x.1).collect();

    let lcm = lcm(denominators);

    let mut result: (Option<i64>, i64) = (None, lcm);

    for fraction in fractions {
        match result.0 {
            Some(x) => result.0 = Some(x - fraction.0 * (lcm / fraction.1)),
            None => result.0 = Some(fraction.0 * (lcm / fraction.1))
        }
    }

   (result.0.unwrap_or(0), result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn factorize_works() {
        let result = factorize(9_223_372_036_854_775_807);
        assert_eq!(result, [7, 7, 73, 127, 337, 92737, 649_657]);
    }

    #[test]
    pub fn lcm_works() {
        let result = lcm(vec![50, 65]);
        assert_eq!(result, 650);
    }

    #[test]
    pub fn gcf_works() {
        let result = gcf(vec![72, 168]);
        assert_eq!(result, 24);
    }

    #[test]
    pub fn add_fraction_works() {
        let result = add_fraction(vec![(3, 10), (5, 4)]);
        assert_eq!(result, (31, 20));
    }

    #[test]
    pub fn subtract_fraction_works() {
        let result = subtract_fraction(vec![(5, 7), (1, 8)]);
        assert_eq!(result, (33, 56));
    }
}
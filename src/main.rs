fn calculate_merchant_totals(transactions: &str) -> Vec<(String, f64)> {
    let mut merchant_totals = std::collections::HashMap::new();
    for line in transactions.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() >= 6 {
            let merchant = parts[2..parts.len() - 2].join(" ");
            let last_part = parts[parts.len() - 1].trim_start_matches('$');
            let amount: f64 = last_part.parse().unwrap_or(0.0);
            *merchant_totals.entry(merchant).or_insert(0.0) += amount;
        }
    }
    let mut sorted_merchants = merchant_totals.into_iter().collect::<Vec<_>>();
    sorted_merchants.sort_by(|(merchant1, amt1), (merchant2, amt2)| {
        amt2.total_cmp(amt1).then_with(|| merchant1.cmp(merchant2))
    });
    sorted_merchants
}

const TRANSACTIONS: &str = r#"
01/05 01/05 ABC A CITY_A CA $0.50
01/10 01/10 XYZ B INC. CITY_B CA $0.50
01/15 01/15 LLL E INDUSTRIES CITY_D CA $0.50
01/20 01/20 PQR D INDUSTRIES CITY_D CA $0.50
01/05 01/05 ABC A CITY_E CA $0.50
01/05 01/05 ABC A CITY_A CA $0.50
"#;

fn main() {
    let merchant_totals = calculate_merchant_totals(TRANSACTIONS);
    let max_print_width = merchant_totals
        .iter()
        .map(|(k, _)| k.len())
        .max()
        .unwrap_or(0);
    for (merchant, total) in &merchant_totals {
        println!("{:<max_print_width$}: ${:.2}", merchant, total,);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_merchant_totals() {
        let expected_result = vec![
            (String::from("ABC A CITY_A"), 1.0),
            (String::from("ABC A CITY_E"), 0.5),
            (String::from("LLL E INDUSTRIES CITY_D"), 0.5),
            (String::from("PQR D INDUSTRIES CITY_D"), 0.5),
            (String::from("XYZ B INC. CITY_B"), 0.5),
        ];
        assert_eq!(calculate_merchant_totals(TRANSACTIONS), expected_result);
    }
}

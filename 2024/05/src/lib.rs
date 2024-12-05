pub fn get_input() -> String {
    String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
    String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules: Vec<_> = rules_str
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("|").unwrap();

            return (from.parse::<i64>().unwrap(), to.parse::<i64>().unwrap());
        })
        .collect();

    let updates: Vec<Vec<_>> = updates_str
        .lines()
        .map(|line| line.split(",").map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();

    let correct_updates: Vec<_> = updates
        .iter()
        .filter(|update| {
            return update.iter().enumerate().all(|(i, page)| {
                return update.iter().skip(i + 1).all(|other_page| {
                    rules
                        .iter()
                        .find(|rule| rule.0 == *page && rule.1 == *other_page)
                        .is_some()
                });
            });
        })
        .collect();

    return correct_updates
        .iter()
        .map(|update| {
            let middle = update.len() / 2;

            return update[middle];
        })
        .sum();
}

pub fn solve_b(input: String) -> i64 {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules: Vec<_> = rules_str
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("|").unwrap();

            return (from.parse::<i64>().unwrap(), to.parse::<i64>().unwrap());
        })
        .collect();

    let updates: Vec<Vec<_>> = updates_str
        .lines()
        .map(|line| line.split(",").map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();

    let incorrect_updates: Vec<_> = updates
        .iter()
        .filter(|update| {
            return !update.iter().enumerate().all(|(i, page)| {
                return update.iter().skip(i + 1).all(|other_page| {
                    rules
                        .iter()
                        .find(|rule| rule.0 == *page && rule.1 == *other_page)
                        .is_some()
                });
            });
        })
        .collect();

    let corrected_updates: Vec<_> = incorrect_updates
        .iter()
        .map(|update| {
            let mut with_rule_counts = update
                .iter()
                .map(|page| {
                    let other_pages = update
                        .iter()
                        .filter(|other_page| *other_page != page)
                        .collect::<Vec<_>>();

                    return (
                        page,
                        rules
                            .iter()
                            .filter(|rule| rule.0 == *page && other_pages.contains(&&rule.1))
                            .count(),
                    );
                })
                .collect::<Vec<_>>();

            with_rule_counts.sort_by_key(|(_page, count)| *count);

            with_rule_counts
                .iter()
                .map(|(page, _)| *page)
                .collect::<Vec<_>>()
        })
        .collect();

    return corrected_updates
        .iter()
        .map(|update| {
            let middle = update.len() / 2;

            return update[middle];
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_eg_1() {
        assert_eq!(solve_a(get_sample_input()), 143);
    }

    #[test]
    fn a() {
        assert_eq!(solve_a(get_input()), 5091);
    }

    #[test]
    fn b_eg_1() {
        assert_eq!(solve_b(get_sample_input()), 123);
    }

    #[test]
    fn b() {
        assert_eq!(solve_b(get_input()), 4681);
    }

    //
}

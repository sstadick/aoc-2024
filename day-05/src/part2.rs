use crate::part1::{parse_pages, parse_rules};

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let (rules, rules_end_offset) = parse_rules(input);
    // Sum up the middle page of page update sets that are invalid once they have been corrected
    let mut answer = 0;
    for mut page_updates in parse_pages(&input[rules_end_offset + 1..]) {
        if !rules.is_valid_page_update_set(&page_updates) {
            rules.correct_update_set(&mut page_updates);
            let middle = page_updates[page_updates.len() / 2];
            answer += middle.0 as u32;
        }
    }
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!("123", process(input)?);
        Ok(())
    }
}

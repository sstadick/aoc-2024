use itertools::Itertools;

type After = u8;
type Before = u8;
type Afters = u128;
// type Befores = u128; // We never actually look at it in this way

#[derive(Debug, Clone)]
pub struct Rules {
    // The index into the lookup corresponds to the page number, so the value at index 42 is the bitset of pages that have a rule that specifies that 42 must come before them.
    forward_lookup: [Afters; 100],
    // The index into the lookup corresponds to the page number, so the value at index 42 is the bitset of pages that have a rule that specifies that 42 must come after them.
    // backward_lookup: [Befores; 100],
}

impl Rules {
    pub fn new() -> Self {
        Self {
            forward_lookup: [0; 100],
            // backward_lookup: [0; 100],
        }
    }

    pub fn add_rule(&mut self, before: Before, after: After) {
        self.forward_lookup[before as usize] |= 1u128 << after;
        // self.backward_lookup[after as usize] |= 1u128 << before;
    }

    pub fn is_valid_page_update_set(&self, updates: &[(u8, u128)]) -> bool {
        for (page, pages_before_this_page) in updates {
            // are there any pages before this page that were supposed to come after it instead?
            let values_that_should_come_after_this_page = self.forward_lookup[*page as usize];
            if (*pages_before_this_page & values_that_should_come_after_this_page) > 0 {
                return false;
            }
        }
        true
    }

    pub fn correct_update_set(&self, updates: &mut [(u8, u128)]) {
        loop {
            let mut right_index = 0;
            let mut left_index = 0;
            for (i, (page, pages_before_this_page)) in updates.iter().enumerate() {
                let values_that_should_come_after_this_page = self.forward_lookup[*page as usize];
                let bad_values = *pages_before_this_page & values_that_should_come_after_this_page;
                let smallest_bad_value = bad_values.trailing_zeros() as u8;
                if (bad_values) > 0 {
                    // We've found something wrong! There's a value before this number that should come after it.
                    right_index = i;
                    left_index = updates
                        .iter()
                        .find_position(|(page_num, _)| *page_num == smallest_bad_value)
                        .unwrap()
                        .0;
                    break;
                }
            }

            if right_index == left_index {
                // We are done maybe?
                break;
            }

            // Swap the bad values and rebuild the indexes
            updates.swap(right_index, left_index);
            let mut values_before = 0;
            for update in updates.iter_mut() {
                update.1 = values_before;
                values_before |= 1u128 << update.0;
            }
        }
    }
}

impl Default for Rules {
    fn default() -> Self {
        Self::new()
    }
}

type RulesEndOffset = usize;

pub fn parse_rules(data: &[u8]) -> (Rules, RulesEndOffset) {
    let mut rules = Rules::new();
    let mut offset = 0;
    while data[offset] != b'\n' {
        let mut before = 0;
        let mut after = 0;
        // First two bytes are before
        before += data[offset] - 48;
        before = (before * 10) + (data[offset + 1] - 48);
        // skip third byte
        // Four and five are after
        after += data[offset + 3] - 48;
        after = (after * 10) + (data[offset + 4] - 48);
        rules.add_rule(before, after);
        // sixth is newline
        offset += 6
    }
    (rules, offset)
}

pub fn parse_pages(data: &[u8]) -> impl Iterator<Item = Vec<(u8, u128)>> + '_ {
    let mut offset = 0;
    std::iter::from_fn(move || {
        let mut pages_before_this_page = 0;
        let mut page_update = vec![];

        while offset < data.len() && data[offset] != b'\n' {
            match data[offset] {
                b'0'..=b'9' => {
                    // this byte and the next one are digits
                    let mut num = 0;
                    num += data[offset] - 48;
                    num = (num * 10) + (data[offset + 1] - 48);
                    offset += 2;
                    page_update.push((num, pages_before_this_page));
                    pages_before_this_page |= 1u128 << num as u128;
                }
                b',' => {
                    offset += 1;
                }
                _ => unreachable!(),
            }
        }
        offset += 1; // advance past newline
        if page_update.is_empty() {
            None
        } else {
            Some(page_update)
        }
    })
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    let (rules, rules_end_offset) = parse_rules(input);
    // Sum up the middle page of page update sets that are valid
    let mut answer = 0;
    for page_updates in parse_pages(&input[rules_end_offset + 1..]) {
        if rules.is_valid_page_update_set(&page_updates) {
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

        assert_eq!("143", process(input)?);
        Ok(())
    }
}

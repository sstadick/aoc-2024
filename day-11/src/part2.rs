use crate::part1::stone_counter_acc;

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    // Same as part1 just bigger
    let count = stone_counter_acc(input, 75);
    Ok(count.to_string())
}

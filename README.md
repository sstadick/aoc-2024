# AOC 2024

Advent of Code 2024 in Rust. We'll see how far I get this year...

Basing the setup this year off of [Chris Biscardi](https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2024/rust).

## Setup

Need:
- Rust (stable, nightly, rustfmt, clippy)
- direnv (optional, used by the get-aoc-inputs script to fetch inputs using your AOC session cookie)
- just (optional, used for running tasks in the `Justfile`)

## Running

You can find SESSION by using Chrome/Firefox tools:
1) Go to https://adventofcode.com/2022/day/1/input
2) right-click -> inspect -> click the "Application" tab.
3) Refresh
5) Click https://adventofcode.com under "Cookies"
6) Grab the value for session. Fill it into your .envrc file

example .envrc:

```
SESSION=PASTE_COOKIE_VALUE_HERE
```

get the input for a day's puzzle

---

Create a new day:

  ```sh
  # Assumes session token is in .envrc
  just create day-01
  ```

---

Run tests for a day-01 part 1:

  ```sh
  just test day-01 1
  ```

---

Run day-01 part 1:

  ```sh
  just run day-01 1
  ```

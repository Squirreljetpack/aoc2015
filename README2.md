A modified AOC rust template

For PRIVATE repositories ONLY (uncomment `data/inputs/*` and `data/puzzles/*` in .gitignore if you want to make it public)

# Commands

Requires [cargo-alias-exec](https://crates.io/crates/cargo-alias-exec) and [aoc-cli](https://crates.io/crates/aoc-cli).
The following aliases apply to the earliest incomplete day if no day is given. A day is considered complete when the last line of `data/puzzles/{day}.md` begins with `#` (i.e. `#complete or #skip`). `submit_p2` will mark the day as complete.

### init

- Check out [README.md](./README.md)
- Set the year in config.toml
- To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools (Cmd+Shift+J). Look in Cookies under the Application or Storage tab, and copy out the session cookie value
- `pbpaste > ~/.adventofcode.session`

### scaffold

- `cargo start` (`c s`)
	- `cargo refresh [n]` can force redownload, although it shouldn't be needed.

### solve

- _write your solution_
- test (from your editor or with `c t`)
- you may want to ask gemini to add examples for day $n

### submit

- solve + benchmark: `c s3`
- `c submit_p1`: (`c s1/s2`)
- git push # workflow updates the leaderboard

# Links

- https://adventofcode.com/2015
- https://github.com/fspoettel/advent-of-code-rust

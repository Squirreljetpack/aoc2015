This document outlines the process for creating example files for a given day's puzzle.

## Process

To add examples for a given day `n`:

THROUGHOUT THE WHOLE PROCESS, DO NOT SEARCH ONLINE
In the following, `DD` is the zero-padded day number (e.g., `01`, `02`)

1.  **Identify Examples:** Carefully read the puzzle description in `data/puzzles/DD.md` for Part 1 (and Part 2 if existing in the file) to find all provided examples. These include the input and the expected output. Search this path explicitly, it has been gitignored. Abort if the file doesn't exist.
2.  **Create Example Files:** For each example, create a new file in the `data/examples/` directory.
3.  **Name the Files:** Name the files using the format `DD-i.txt`, where `i` is a sequential integer starting from 1 for each distinct example.
4.  **Add Content:** Populate each file with its corresponding input data.
5.  **Update Tests:** In the solution file for the day, located at `src/bin/DD.rs`, update the `#[cfg(test)]` block. Use the `advent_of_code::template::read_file_part("examples", DAY, i)` function to read each example file and assert that the output of your solution matches the expected output from the puzzle description. If part 2 was not processed, do not add tests for it!

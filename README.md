My Rust solutions for [Advent of Code 2018](https://adventofcode.com/2018)

### Inputs and Outputs

I keep local subdirectories for problem inputs and outputs.
I deliberately do not commit them to this repository because
I believe sharing specific inputs and outputs is against the
wishes of the creator of Advent of Code.

The main shell will try to use "inputs/dayXX.txt" as the default input for
`cargo run dayXX`. You can specify a different input file by passing it as
`cargo run dayXX INPUT`.

Also, the unit tests for each day include a test
for the real problem input. These tests are disabled by default and
can be enabled by passing `--features test_real_input` to the
`cargo test` command. These tests assume the input for dayXX is in
"inputs/dayXX.txt" and the expected output is in "outputs/dayXX.txt"

# cargo-bench2md
Formatting `cargo bench` output into .md table.

## Installation
The process is pretty straightforward:
```
git clone git@github.com:niki4smirn/cargo-bench2md.git
cd cargo-bench2md
cargo build --release
cd target/release
```
Here you can find the `cargo-bench2md` binary.

## Usage
The app reads JSON-formatted info about benchmarks from stdin and prints results into stdout.

### Example:
```
cargo +nightly bench -- --format json -Z unstable-options | ./cargo-bench2md > table.md
```
*(Don't forget to use `+nightly` and `-Z unstable-options` to run benchmarks and format output as JSON)*

## Output

Results are separated into different tables if corresponding benchmarks are defined in different modules.

### Example:

Human-readable output:
```
test magic::benchmarks::string::ahash       ... bench:         893 ns/iter (+/- 29)
test magic::benchmarks::string::fastmurmur3 ... bench:      25,247 ns/iter (+/- 2,352)
test magic::benchmarks::u64::ahash          ... bench:         774 ns/iter (+/- 39)
test magic::benchmarks::u64::fastmurmur3    ... bench:       5,268 ns/iter (+/- 221)
```
.md output:
```
### magic::benchmarks::string
**Note:** All measurements are in ns/iter.
| Name | Median | Deviation |
|------|-------:|----------:|
| ahash | 898 | 76 |
| fastmurmur3 | 25036 | 819 |

### magic::benchmarks::u64
**Note:** All measurements are in ns/iter.
| Name | Median | Deviation |
|------|-------:|----------:|
| ahash | 768 | 81 |
| fastmurmur3 | 5178 | 582 |
```

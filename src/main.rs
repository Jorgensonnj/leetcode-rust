// simply prints out the directions for this repo
fn main() {
    let directions =
"
Directions:
In order to run a single binary you must run the below command.

cargo run --bin <name_of_binary>

Avaliable binaries:
    - 001_two_sum
    - 002_add_two_numbers
    - 003_longest_substring_without_repeating_characters
    - 004_median_of_two_sorted_arrays
    - 005_longest_palindromic_substring
    - 006_zigzag_conversion
    - 007_reverse_integer
    - ...
";

    println!("{}", directions);
}

// simply prints out the directions for this repo
fn main() {
    let directions =
"
Directions:
In order to run a single binary you must run the below command.

cargo run --bin <name_of_binary>

Avaliable binaries:
    - 1_two_sum
    - 2_add_two_numbers
    - 3_longest_substring_without_repeating_characters
    - 4_median_of_two_sorted_arrays
    - 5_longest_palindromic_substring
";

    println!("{}", directions);
}

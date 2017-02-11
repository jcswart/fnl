# fnl

Print the first column `... | awk {print $1}` fnl is `... | fnl 1`.

Print the fourth column and then the second `... | awk '{print $4 $1}'` fnl is `... | fnl 4 1`.

Split on all whitespace: `... | fnl`

This is basically awk/cut's job usually but this saves a few keystrokes and let me practice Rust.


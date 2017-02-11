# fnl

Print the first column `... | awk {print $1}` fnl is `... | fnl 1`.

Print the fourth column and then the second `... | awk '{print $4 $2}'` fnl is `... | fnl 4 2`.

Split on all whitespace, print everything tab seperated: `... | fnl`

This is basically awk/cut's job usually but this saves a few keystrokes and let me practice Rust. This tool is simply for my most common usecase of awk/cut to select columns of data.

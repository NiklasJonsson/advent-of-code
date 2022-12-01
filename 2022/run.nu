def day_dirname [n: int] {
    let number = ($"($n)" | str lpad -l 2 -c '0')
    echo $"day($number)"
}

let n = (ls day* | sort-by name | get name | last 1 | path basename | str substring '3,' | into int | get 0)

echo $"Running day ($n)"
echo $"Running simpleinput (char nl)"
cargo run --release --bin $"(day_dirname $n)" -- $"(day_dirname $n)/simpleinput.txt"
echo $"Running input (char nl)"
cargo run --release --bin $"(day_dirname $n)" -- $"(day_dirname $n)/input.txt"
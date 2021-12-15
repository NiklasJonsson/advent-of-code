def day_dirname [n: int] {
    let number = ($"($n)" | str lpad -l 2 -c '0')
    echo $"day($number)"
}

let n = (ls day* | sort-by name | get name | last 1 | path basename | str substring '3,' | str to-int)

let prev_dir = (day_dirname $n | path expand)
let next_dir = (day_dirname ($n + 1))

if ($next_dir | path exists) {
    echo $"($next_dir) already exists, can't create (char nl)"
} {
    let new_mem_list = (open Cargo.toml | get workspace.members | append ($next_dir))
    open Cargo.toml | update workspace.members ($new_mem_list) | save Cargo.toml
    cargo new --bin ($next_dir)
    cp $"($prev_dir)/src/main.rs" $"($next_dir)/src/main.rs" 
    echo (char nl) | save $"($next_dir)/input.txt"
    echo (char nl) | save $"($next_dir)/simpleinput.txt"
    git add $next_dir
}
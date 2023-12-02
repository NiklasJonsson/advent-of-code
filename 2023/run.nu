let day_dir = ( ls day* | sort-by name | get name | last 1 | path basename | get 0)

echo $"Running ($day_dir)(char nl)"
echo $"Running simpleinput (char nl)"
cargo run --bin $day_dir -- $"($day_dir)/simpleinput.txt"
echo $"Running input (char nl)"
cargo run --bin $day_dir -- $"($day_dir)/input.txt"
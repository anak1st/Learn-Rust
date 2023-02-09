cargo build

$in = ".\log\in.txt"
$out = ".\log\out.txt"

Get-Content $in | .\target\debug\learn-rust.exe > $out

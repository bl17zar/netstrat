# netstrat
**netstrat** is a strategy backtesting and visualization tool

<img width="1728" alt="Screen Shot 2022-08-23 at 14 06 58" src="https://user-images.githubusercontent.com/32969427/186132111-9bce80c2-fe13-4208-9d02-2ede064d5ed5.png">

<img alt="Screen Shot 2022-10-21 at 13 23 18" src="https://user-images.githubusercontent.com/32969427/197162032-ba3efb31-af82-4f41-ad0b-25de5bc4834e.png">

### depedencies
You need to have [graphviz binary](https://graphviz.org/download/) installed

### build
```bash
# windows 10
cargo build --target=x86_64-pc-windows-gnu --release
```
```bash
# apple intel
cargo build --target=x86_64-apple-darwin --release
```
```bash
# apple silicon
cargo build --target=aarch64-apple-darwin --release
```

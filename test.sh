#!/bin/sh
NB_TESTS=50
START_TIME=$SECONDS
# for ((c=1; c<=$NB_TESTS; c++))
# do
#     cargo run --bin solver --release -- "U' R B2 D' B2 R F L' B2 L"
# done
cargo run --bin solver --release -- "F D B L2 B2 U2"
cargo run --bin solver --release -- "D2 B' U2 L D' L' F2 R B' L D2 L' U' F' L D' L D2 R B' D R2 U'"
cargo run --bin solver --release -- "R' U2 L B' L B' U R2 B' R' U2 F2 R F2 R' U2 F R U' L D'"
cargo run --bin solver --release -- "B D' B' R' F' R2 F' R2 U' R'"
cargo run --bin solver --release -- "R"
cargo run --bin solver --release -- "B2 R2 F' D2 L2 B' U L'"
cargo run --bin solver --release -- "B R2 F' R2 U B' D B2 R2 D F' R D2 L U L F2 R' F' D' L'"
cargo run --bin solver --release -- "R' F2 U2"
cargo run --bin solver --release -- "B' L D' F2 L U' F D' L D2 F' L' B2 U' R U R2 B'"
cargo run --bin solver --release -- "B U F R F2 U2 L' B D"
cargo run --bin solver --release -- "R B' L'"
cargo run --bin solver --release -- "R D2 B U F R' B U R' F' R2 D L F U2 R' U2 B D' R U'"
cargo run --bin solver --release -- "L2 U' B L' F' L U R' B2 D2 R2 D2 L D' L' F R2 U2 R2 B D"
cargo run --bin solver --release -- "U' R F2 L2 U' B2 R2 D L U L' U2 R F D' F2 R D' B2 D' F2 L' B2 R"
cargo run --bin solver --release -- "B2 U' F2 D2 B2 L2 D' R B U' R' D2 F D L2 F2 R2 B2 U F2 L' F'"
cargo run --bin solver --release -- "L D2 L D' R' D' B R B2 D L2 U R' U' L F2 U L2 F2 D L U2"
cargo run --bin solver --release -- "U B' L2 B'"
cargo run --bin solver --release -- "B2 D' F U' B U2 B' D R U' R2 D' B2 U' F2 L2 F' R2 B'"
cargo run --bin solver --release -- "U F' D2 F"
cargo run --bin solver --release -- "U"
cargo run --bin solver --release -- "D2 R2 B2 U' F R' B' D R2 D' R B2 D F"
cargo run --bin solver --release -- "B R' B2 L2"
cargo run --bin solver --release -- "L F D' B2 D2 R2 F D R U' L2"
cargo run --bin solver --release -- "L' B' D2 B' U L' U2 L' U' L2 B2 L D F2 L2 B2 D L U' F' U2 F2"
cargo run --bin solver --release -- "F' D2 F2 D2"
cargo run --bin solver --release -- "L2 B U L2 B R' F U' L' B2 R2 F' L' F' R' B' U2 R U2 B"
cargo run --bin solver --release -- "R2 U' B2 R2 B' L2 U2 R B R2 D' R' D2 B R' F2 U2 R2 B2 U' F2 D"
cargo run --bin solver --release -- "U R' F' D2 L2 F2"
cargo run --bin solver --release -- "D2 L2 F D'"
cargo run --bin solver --release -- "R' U' F L2 B2 D"
cargo run --bin solver --release -- "L D B2 L B2 R' U' L' B L' B L' B' U2 F' R2 D L D'"
cargo run --bin solver --release -- "R2 D2 R B' U' F' U2 R' F' L' D' B' D2 R2 B D2"
cargo run --bin solver --release -- "R2 F' L U2 L2 U' B U' B' U B' R2 U F2 U2"
cargo run --bin solver --release -- "R2 B R2 B' L2 B"
cargo run --bin solver --release -- "D' B' D' B U2 R' B' D' F' D2 R' U R2 D'"
cargo run --bin solver --release -- "D L2 B' U' R' D' R' U B D2 B2 U' B U F"
cargo run --bin solver --release -- "B' D' L' U' L D L U F R"
cargo run --bin solver --release -- "R2 U' L2 B' L U R F L' U' R D' L2 D2 L' U2 R2"
cargo run --bin solver --release -- "L D"
cargo run --bin solver --release -- "R2 B2 D2 B U"
cargo run --bin solver --release -- "F2 R' U2 B D' R F D R' D B L2 D F D F' U R' F"
cargo run --bin solver --release -- "F D"
cargo run --bin solver --release -- "B2 R' D2 F' L U' L2 U2 L' F U2 R2 D R' D R' U' L D' R2 U' B2 D'"
cargo run --bin solver --release -- "U2 F2 U F' D R2 B' U' L D2 L U B'"
cargo run --bin solver --release -- "R2 B2 D' R B L2 B' D2 F U' R' D' F L2 F'"
cargo run --bin solver --release -- "R2 F2"
cargo run --bin solver --release -- "R B' R2 F' R' B' U' R2"
cargo run --bin solver --release -- "B L D2 L D L2 B2 U L D2 R2 D L2 F R"
cargo run --bin solver --release -- "F L B' R' B D' F2 U2 F D2 L2 U L F'"
cargo run --bin solver --release -- "L' F' D2 B' R2"
echo "\n- Tests -----------------"
echo "total duration: "$(($SECONDS - $START_TIME))"s"
echo "average duration : "$((($SECONDS - $START_TIME) / $NB_TESTS))"s"
echo "-------------------------\n"

# [10] [10] | total 134s | av 2s
# [10] [11] | total 74s  | av 1s
# [10] [12] | total 60s  | av 1s
# [10] [13] | total 65s  | av 1s
# [10] [14] | total 108s  | av 2s
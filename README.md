# rubik (work in progress)
A Rubik's Cube solver using Autodidactic Iteration Reinforcement Learning, written in Rust<br/>
This is a 42 school project <strong>[Final grade: work in progress]</strong>

---
## Demo
<img src="demo.gif" height="250"/>

---
## Setup
If you do not have Rust
```
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
> export PATH=$PATH:$HOME/.cargo/bin
```
Then you can build the project
```
> cargo build --release
```

---
## Train usage (work in progress)
```
> ./target/release/train [FLAGS]
```
### Flags
```
-h, --help       Prints help information
-V, --version    Prints version information
```

---
## Solve usage (work in progress)
```
> ./target/release/solve [FLAGS] <input_sequence>
```

### Flags
```
-h, --help            Prints help information
-V, --version         Prints version information
-v, --visualisator    enable the visualisator
```

### Args
```
<input_sequence>      The sequence to shuffle a rubik
```

### Sequence format
An action is defined by a face and a rotation
```
F -> front
R -> right
U -> up
B -> back
L -> left
D -> down

nothing -> turn right
' -> turn left
2 -> half rotation
```
For example: ```F U2 B' L' D'``` means "turn front to right, half rotation on up face..."

---
## Contributors
<table>
  <tr>
    <td align="center"><a href="https://github.com/sgalasso42"><img src="https://avatars2.githubusercontent.com/u/38636967?v=4" width="100px;" alt=""/><br /><sub><b>Simon Galasso</b></sub></a><br />
  </tr>
</table>

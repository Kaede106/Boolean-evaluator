## Boolean-evaluator
Boolean evaluator takes in a Boolean expression, such as P*Q ~ P \land Q ~ P&amp;&amp;Q, etc and recursively evaluates the boolean expression.


## How to Build
First use git clone on https://github.com/Kaede106/Boolean-evaluator/
# Nixos
Then on Nixos you simply need `nix develop` (assuming you have flakes enabled), and then `cargo build --release`

# Other linux
Install dependencies via your package manager and `cargo build --release`


# Guess Game
A hot & cold guessing game. Chapter 2 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

## Build & Run
Can be built and run with Rust or Docker.

### Rust
```
cargo build --release
./target/release/guess-game
```

### Docker
```
docker build -t guess-game .
docker run --rm -it guess-game
```

## Example
```
$ ./target/release/guess-game 
Welcome to the Guessing Game!
Please guess a number between 1 and 10.
1
You guessed 1
Too small, try again!
Please guess a number between 1 and 10.
10
You guessed 10
Too big, try again!
Please guess a number between 1 and 10.
5
You guessed 5
Congratulations! You guessed the number!
```
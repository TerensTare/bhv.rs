# bhv

Bhv is a library for working with behavior trees.


## Installation


Simple as running

```sh
cargo add bhv
```

on the directory of the project where you want to use the library.


## Showcase


```rust
// Guess the number game implemented using behavior trees.
use std::{
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use bhv::*;

fn main() {
    game()
}

// The data that will be used by the nodes
#[derive(Default)]
struct Context {
    guess: u32,
    answer: u32,
}

// the nodes
struct RandomizeAnswer(u32, u32);
struct ReadInput;

macro_rules! print_msg {
    ($($arg:tt)*) => {
        action(|_ctx| print!($($arg)*))
    };
}

fn game() {
    let tree = seq! {
        RandomizeAnswer(0, 101),
        seq! {
            print_msg!("Enter a number from 0 to 100\n"),
            ReadInput,
            sel! {
                seq! {
                    cond(|ctx: &Context| ctx.guess < ctx.answer),
                    print_msg!("Your guess is smaller than the actual number\n").fail(), // The game is not over yet
                },
                seq! {
                    cond(|ctx: &Context| ctx.guess == ctx.answer),
                    print_msg!("Your guess is correct!\n"),
                },
                seq! {
                    cond(|ctx: &Context| ctx.guess > ctx.answer),
                    print_msg!("Your guess is greater than the actual number\n").fail(), // The game is not over yet
                }
            },
        }.repeat_until_pass(),
    };

    tree.execute(Context::default());
}

impl Bhv for RandomizeAnswer {
    type Context = Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        ctx.answer = (time % ((self.1 - self.0) as u64)) as u32 + self.0;

        Status::Success
    }
}

impl Bhv for ReadInput {
    type Context = Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        io::stdout().flush().unwrap_or_default();

        let mut buff = String::new();

        io::stdin()
            .read_line(&mut buff)
            .map(|_| match buff.trim().parse() {
                Ok(v) => {
                    ctx.guess = v;
                    Status::Success
                }
                Err(e) => {
                    println!("Error reading from stdin :{}\t buffer: '{}'", e, buff);
                    Status::Failure
                }
            })
            .unwrap_or_else(|_| Status::Running)
    }
}
```


## Features

The library provides several nodes that are commonly present in behavior trees. Specifically, the nodes provided are split into categories as following:
- adaptor nodes, that adapt a function into a behavior tree node.
- decorator nodes, that alter the behavior and result of a node.
- composite nodes, that several nodes at the same time.

For help with specific nodes, refer to the documentation of the crate.


## License

Crate licensed under the MIT license.
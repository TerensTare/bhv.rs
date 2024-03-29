# bhv

Bhv is a library for working with behavior trees.

## But what are behavior trees?

A behavior tree is basically a tree where each node defines some action/function unique to the node type. Each node
returns a "status" after they are executed, which determines whether the node has fully finished executing (and whether
it executed successfully or not) or it still needs to be ran later. Parent nodes provide composition (think of a node
that runs all of its children until one of them returns a "finished executing but not successful") while leaf nodes act
as "atomic actions", ie. actions that cannot be split further (think of a simple condition check or a message being
printed to the terminal). To share state between nodes, a "context" type is passed to each node being ran. Parent nodes
usually adapt this context type by their children, while the leaf nodes are the ones that usually define a concrete type
as context.

There is a set of nodes that are commonly used and thus provided by the library, specifically:

Adaptor nodes - leaf nodes that adapt functions into a node of the tree, such as `cond` that executes successfully when
a condition is successful or `action` that simply executes a function and returns successfully.

Decorator nodes - nodes with a single child that manipulate the child's status or run it several times depending on some
condition, such as `inv` that changes the execution status from success to failure and vice versa, or `repeat(n)` that
runs a node until it is completed n times.

Composite nodes - nodes with at least one child that run them according to some condition, such as `sequence` that runs
its child nodes as long as they execute successfully or `selector` that runs its child nodes until one of them executes
successfully.

As it can be noticed, composition based on a node's status provides logic similar to control flow in programming
languages. Furthermore, transition between states becomes as simple as providing a node with a condition and the state's
nodes to be executed if the condition is successful (ie. just a `sequence` of the condition and then the other nodes).
This makes behavior trees an alternative to state machines, where the states are not aware of each other as state
management logic happens inside of parent nodes instead of child nodes.

For a more in-depth explanation and more examples,
this [GameAIPro](https://www.gameaipro.com/GameAIPro/GameAIPro_Chapter06_The_Behavior_Tree_Starter_Kit.pdf) chapter is a
great resource.

## Installation

Simple as running

```sh
cargo add bhv
# or for the `events` feature
cargo add bhv --features events
```

on the directory of the project where you want to use the library.

## Features

The library provides several nodes that are commonly present in behavior trees. Specifically, the nodes provided are
split into categories as following:

- adaptor nodes, that adapt a function into a behavior tree node.
- decorator nodes, that alter the behavior and result of a node.
- composite nodes, that several nodes at the same time.

For help with specific nodes, refer to the documentation of the crate.

## Showcase

```rust,ignore
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

    tree.execute(&mut Context::default());
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

### `Events`

Version 0.3 introduces a new model for behavior trees, available under the feature flag `events`.
This experimental feature allows nodes to run only when relevant event(s) occur.
When this feature will be deemed complete, it will replace the actual default implementation, so it
is recommended to migrate existing codebases to use this feature.

Compared to the "default" implementation, nodes on `events` introduce a new function called
`should_react_to(&self, kind: EventKind) -> bool` that indicates whether an event of type `kind`
is relevant to this node or not. By default, all events are considered relevant unless the behavior is overwritten.
Additionally, the `update` function is now called `react(&mut self, event: &dyn Event, ctx: &mut Self::Context)`,
where the `Event` trait is used to make sure the values passed are meant to be used as event data. This model allows
nodes to adapt to a more asynchronous event-based model of execution.

To reflect these changes, the `execute` function now accepts a new parameter that can be converted into an iterator that
yields `&dyn Event` and be used as a stream of events for the node. This function is part of `BhvExt` instead of `Bhv`,
meaning it can only have the default implementation and also that nodes can now be `?Sized`.

As for differences in nodes provided, `.repeat_until(cond)` is **NOT** present when this feature is enabled. To achieve
similar behavior with `bhv.repeat_until(cond)`, please use `seq! { bhv.pass(), cond }.repeat_until_pass()`.
Alternatively, you can use the new `.wait_for::<EventType>()` decorator that runs a node only if a certain event is
triggered. To use this decorator the event should implement `EventType`. Think of `EventType` implementors as simply
`Event` types that are not `enum` types.

To implement `Event` for your own types, proceed as following:

- For non-`enum` types, simply implement `EventType` as

```rust,ignore
use bhv::*;

struct MyType; // my special event type

impl EventType for MyType {}

// Event is automatically implemented for EventType
```

- For `enum` types, implement `Event` as following

```rust,ignore
use bhv::*;

enum MyEvents {
    A(i32),
    B(char),
    C,
}

impl Event for MyEvents {
    fn event_name(&self) -> &str {
        match self {
            Self::A(_) => "Event::A",
            Self::B(_) => "Event::B",
            Self::C => "Event::C",
        }
    }
}
```

## License

Crate licensed under the MIT license.
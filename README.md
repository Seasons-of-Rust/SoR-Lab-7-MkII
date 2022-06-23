# Summer of Rust Lab 7 (MkII): The Prisoner's Dilemma

This week, we'll be learning more about modules and crates in Rust. We'll also
get an overview of error handling and common collections.

In this week's lab, we're going to write bots to fight against one another in a
"Prisoner's Dilemma" simulation. This lab will be a bit different than others,
as we'll all be working on the same repository together. Be sure to ask
questions in the `Lab 7 Questions` channel.

To start this lab, [watch this
video](https://www.youtube.com/watch?v=t9Lo2fgxWHw) to learn what the Prisoner's
Dilemma is.

[![](https://cdn.discordapp.com/attachments/971065259752321075/989569326753402920/t9Lo2fgxWHw-HD.jpg)](https://www.youtube.com/watch?v=t9Lo2fgxWHw)

Also note, this lab is heavily inspired by carykh's [Prisoner's Dilemma
Tournament](https://github.com/carykh/PrisonersDilemmaTournament), as well as
the Multiagent Systems course at Carleton.

## Table of Contents

- [Summer of Rust Lab 7 (MkII): The Prisoner's Dilemma](#summer-of-rust-lab-7-mkii-the-prisoners-dilemma)
  - [Table of Contents](#table-of-contents)
  - [Instructions](#instructions)
    - [Working with others](#working-with-others)
    - [Running the simulation](#running-the-simulation)
    - [Setting up your bot](#setting-up-your-bot)
    - [Adding your bot to the simulation](#adding-your-bot-to-the-simulation)
    - [The Bot trait](#the-bot-trait)
    - [Working on your bot](#working-on-your-bot)
    - [Merging your bot](#merging-your-bot)
    - [That's all!](#thats-all)
  - [License](#license)

## Instructions

### Working with others

For this lab, you'll work on your own Git branch for your bot. Feel free to
reach out to me if you need any help getting this set up! Once your bot code is
ready, you'll make a PR to merge your bot into the main branch. Also, once your
code merges, you can always make another branch and PR if you want to add
improvements.

### Running the simulation

For this lab, there aren't any tests. Instead, you'll run the program to see the
winners of the simulation. But, if you just do `cargo run`, you'll see that
there is no output. This is because everything in the code will only print using
logging instead of println. Specifically, using the `info!` macro from the
logging crate instead of `println!` So, we need to set the logging level to
info.

```bash
RUST_LOG=info cargo run
```

If you take a look around in the code, you'll see that `info!` is used in a few
places, and that's what gets printed out here. On the other hand though, there
is another macro used in this code, `debug!`. This logging macro is used for
information that might be helpful during the development or debugging of the
application, but not normally when the final version is being run. There are
some `debug!` macros used for more information about the battles. We can run it
in log level debug as shown below:

```bash
RUST_LOG=debug cargo run
```

When you're running the simulation, you'll see a list of bots with their scores
from the simulation. The scores are the sum of the average score of each game
they played.

### Setting up your bot

There are a few things you'll need to do to create your bot. First, you'll make
a file in `/src/bots` for your bot. The name of your bot should be the same as
your Github username.

As for what you'll put in this file, you'll make a struct that implements the
`Bot` trait from `src/bots/mod.rs`. Your struct will be used to store memory
during a fight against another bot. You can put anything in it that you like.

### Adding your bot to the simulation

Once you have the struct set up, you'll need to add your bot to the
`src/bots/mod.rs` file. We can see that the other bots have already been
declared as modules:

```rust
mod always_betray;
mod always_silence;
mod angelonfira;
mod detective;
mod fifty_fifty;
mod grim_trigger;
```

So you'll need to add yours there. You'll also need to add a variant for your
bot to the `Bots` enum, which should share the same name as your bot.

```rust
#[derive(Debug, EnumIter, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Bots {
    AngelOnFira,
    FiftyFifty,
    AlwaysSilence,
    AlwaysBetray,
    GrimTrigger,
    Detective,
}

impl Bots {
    pub fn objects(&self) -> Box<dyn Bot> {
        match self {
            Bots::AngelOnFira => Box::new(AngelOnFira::new()),
            Bots::FiftyFifty => Box::new(FiftyFifty::new()),
            Bots::AlwaysSilence => Box::new(AlwaysSilence::new()),
            Bots::AlwaysBetray => Box::new(AlwaysBetray::new()),
            Bots::GrimTrigger => Box::new(GrimTrigger::new()),
            Bots::Detective => Box::new(Detective::new()),
        }
    }
}
```

### The Bot trait

As for the `Bot` trait, here is its definition:

```rust
pub trait Bot {
    fn new() -> Self
    where
        Self: Sized;

    fn turn(&mut self, history: &[Turn]) -> Dilemma;
}
```

We can see that two methods need to be implemented. First, a new instance of
your struct with the `new` associated function.

The second method to implement is the turn. Each fight will consist of at least
1,000 turns against a certain opponent. However, a new struct will be created
for each fight, so your memory won't be carried from one fight to another. In
the signature of the `turn` method, you'll see that you also get a history,
which is a slice of `Turn`s. This will give you the history of all games that
have occurred so far this fight.

From the `turn` method, you must return a `Dilemma`. This is an enum that either
has a `Silence` variant, which should be used if you want stay silent against
the other prisoner (bot). On the other hand, the `Betray` variant should be used
to betray your fellow prisoner (bot). Here is the scoring table for battles:

|         |         |              |              |
|---------|---------|--------------|--------------|
| Bot 1   | Bot 2   | Bot 1 points | Bot 2 points |
| Silence | Silence | 3            | 3            |
| Betray  | Silence | 0            | 5            |
| Silence | Betray  | 5            | 0            |
| Betray  | Betray  | 1            | 1            |

So, if both bots stay quiet, they'll gain the most combined points. However, if
a single bot betrays the other, they'll gain the most points they can for
themselves. In an ideal fight for bot 1, over the 1,000+ fights, it would
consistently betray the other bot while the other bot stayed silent.

### Working on your bot

Now that the setup is done, you can work on your bot! Feel free to take
inspiration from other bots, and try different ideas. At the time that this is
being written, there aren't any restrictions on the memory size of your bot, or
how long it takes to make a move. This might change if there are reasons to!

### Merging your bot

Once you're done with your bot, you'll need to merge your bot into the main
branch with a PR. [Read this section](#working-with-others) for more
information!

### That's all!

See you next week 🏖️

## License

The Summer of Rust Labs is duel-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

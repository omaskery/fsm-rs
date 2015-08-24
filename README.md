
A simple Finite State Machine library in Rust. Provide State and Event types (usually enums), then generate a machine with an initial state, give it some transition behaviours and you have your state machine!

# Usage #

Using the simple coin-operated turnstyle example from the FSM wikipedia entry:

Define your states and events:
```rust
// currently states must be cloneable
#[derive(Clone)]
enum TurnStyleState {
	Locked,
	Unlocked,
}

// currently events must be cloneable
#[derive(Clone)]
enum TurnStyleEvent {
	Push,
	InsertCoin,
}

// currently must be convertable to u32
impl Into<u32> for TurnStyleState {
	fn into(self) -> u32 {
		self as u32
	}
}

// currently must be convertable to u32
impl Into<u32> for TurnStyleEvent {
	fn into(self) -> u32 {
		self as u32
	}
}
```

Create your machine and define your transitions:
```rust
// create the machine initially in the Locked state
let mut machine = Machine::new(TurnStyleState::Locked);
// create the transition from Locked -> Unlocked upon inserting a coin
machine.add_transition(
	TurnStyleState::Locked, TurnStyleEvent::InsertCoin,
	TurnStyleState::Unlocked, Box::new(|_| println!("unlocked"))
);
// create the transition from Unlocked -> Locked upon pushing the turnstyle
machine.add_transition(
	TurnStyleState::Unlocked, TurnStyleEvent::Push,
	TurnStyleState::Locked, Box::new(|_| println!("locked"))
);
```

Trigger events as needed and huzzah, off you go:
```rust
// initially we're in the Locked state
machine.on_event(TurnStyleEvent::InsertCoin);
// now we're Unlocked, ("unlocked" was just printed)
machine.on_event(TurnStyleEvent::Push);
// now we're Locked again, ("locked" was just printed)
```

This example is also the test case for the library, although here I've ommitted the test-related details.

# To Do #

- Remove need to Clone inside implementation, I'm sure it's unnecessary
- There must be a more elegant way to write both the add transition method and the on event method, perhaps using Option methods?
- A better trait than Into (u32) ? Not sure if I can get rid of ANY trait dependancy, but Into currently contributes to the dependancy on Clone, when all I really want is a unique ID for each state/ID (preferably sequential), aka: the enum tag.

# Alternatives #

## Macro based solutions ##
- [machine](function://crates.io/crates/machine)
- [microstate](https://crates.io/crates/microstate)
- [beehave](https://crates.io/crates/beehave)

## Other ##
- [genfsm](https://crates.io/crates/genfsm)



A simple Finite State Machine library in Rust. Provide State and Event types (usually enums), then generate a machine with an initial state, give it some transition behaviours and you have your state machine!

# Usage #

Using the simple coin-operated turnstyle example from the FSM wikipedia entry:

Define your states and events:
```rust
// states and events must be C-like enums (copyable and easily converted to primitives)
#[derive(Copy, Clone)]
enum TurnStyleState {
	Locked,
	Unlocked,
}

#[derive(Copy, Clone)]
enum TurnStyleEvent {
	Push,
	InsertCoin,
}

// implement the EnumTag trait for states and events
impl EnumTag for TurnStyleState {
	fn tag_number(&self) -> usize {
		*self as usize
	}
	fn max_tag_number() -> usize {
		TurnStyleState::Unlocked as usize
	}
}

impl EnumTag for TurnStyleEvent {
	fn tag_number(&self) -> usize {
		*self as usize
	}
	fn max_tag_number() -> usize {
		TurnStyleEvent::InsertCoin as usize
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
	TurnStyleState::Unlocked, |_,_| println!("unlocked")
);
// create the transition from Unlocked -> Locked upon pushing the turnstyle
machine.add_transition(
	TurnStyleState::Unlocked, TurnStyleEvent::Push,
	TurnStyleState::Locked, |_,_| println!("locked")
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

- Expose predicate interface and write unit tests for it

# Alternatives #

## Macro based solutions ##
- [machine](https://crates.io/crates/machine)
- [microstate](https://crates.io/crates/microstate)
- [beehave](https://crates.io/crates/beehave)

## Other ##
- [genfsm](https://crates.io/crates/genfsm)


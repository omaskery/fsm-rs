
/// Actions are just boxed immutable functions that take an argument of the event that triggered them
pub type Action<S, E> = Box<Fn(&S,&E)>;

/// Predicates are used to filter down whether a transition can occur
pub type Predicate<S, E> = Box<Fn(&S,&E) -> bool>;

/// Trait that should be trivially implementable for any Enum type
pub trait EnumTag {
	/// returns the discriminator tag for the enum (some_value as usize)
	fn tag_number(&self) -> usize;
	/// returns the highest discriminator tag for this enum
	fn max_tag_number() -> usize;
	/// returns an instance of the enum from a tag number
	fn from_tag(tag: usize) -> Self;
}

/// The Transition records, for a given current state, what event type triggers it to move to
/// what state, performing a specific action on the transition, filterable by a predicate function
struct Transition<S: EnumTag, E: EnumTag> {
	predicate: Option<Predicate<S, E>>,
	next_state: S,
	action: Action<S, E>,
}

/// The EdgeTransitions record all the Transitions for a given event type
struct EventTransitions<S: EnumTag, E: EnumTag> {
	transitions: Vec<Transition<S, E>>,
}

/// The StateTransition records all Transitions for a given state
struct StateTransitions<S: EnumTag, E: EnumTag> {
	edges: Vec<EventTransitions<S, E>>,
}

/// The Machine is the Finite State Machine, which has a current state and set of all valid
/// transitions
pub struct Machine<S: EnumTag, E: EnumTag> {
	state: usize,
	transitions: Vec<StateTransitions<S, E>>,
}

impl<S: EnumTag, E: EnumTag> Machine<S, E> {
	/// Constructs a new FSM with a given initial state
	pub fn new(initial_state: S) -> Machine<S, E> {
		let mut transitions = Vec::with_capacity(S::max_tag_number());

		for _ in 0..S::max_tag_number() + 1 {
			let mut edges = Vec::with_capacity(E::max_tag_number());

			for _ in 0..E::max_tag_number() + 1 {
				edges.push(EventTransitions {
					transitions: Vec::new(),
				});
			}

			transitions.push(StateTransitions {
				edges: edges,
			});
		}

		Machine {
			state: initial_state.tag_number(),
			transitions: transitions,
		}
	}

	/// Registers a new valid transition with the FSM
	pub fn add_transition(&mut self, in_state: S, on_event: E, next_state: S, action: Action<S, E>) {
		let transition = &mut self.transitions[in_state.tag_number()];

		let edge = &mut transition.edges[on_event.tag_number()];

		edge.transitions.push(Transition {
			predicate: None,
			action: action,
			next_state: next_state,
		});
	}

	/// Retrieves a reference to the current state
	pub fn current_state(&self) -> S {
		S::from_tag(self.state)
	}

	/// Tick the State Machine with an Event
	pub fn on_event(&mut self, event_type: E) {
		let transition = &self.transitions[self.state];
		let edge = &transition.edges[event_type.tag_number()];

		for transition in edge.transitions.iter() {
			let valid = match &transition.predicate {
				&Some(ref p) => (*p)(&self.current_state(), &event_type),
				&None => true,
			};

			if valid {
				(*transition.action)(&self.current_state(), &event_type);
				self.state = transition.next_state.tag_number();
				break;
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Debug, Eq, PartialEq)]
	enum TurnStyleState {
		Locked,
		Unlocked,
	}

	#[derive(Debug, Eq, PartialEq)]
	enum TurnStyleEvent {
		Push,
		InsertCoin,
	}

	impl EnumTag for TurnStyleState {
		fn tag_number(&self) -> usize {
			match self {
				&TurnStyleState::Locked => 0,
				&TurnStyleState::Unlocked => 1
			}
		}
		fn max_tag_number() -> usize {
			TurnStyleState::Unlocked as usize
		}
		fn from_tag(tag: usize) -> TurnStyleState {
			match tag {
				0 => TurnStyleState::Locked,
				1 => TurnStyleState::Unlocked,
				_ => panic!("invalid tag number")
			}
		}
	}

	impl EnumTag for TurnStyleEvent {
		fn tag_number(&self) -> usize {
			match self {
				&TurnStyleEvent::Push => 0,
				&TurnStyleEvent::InsertCoin => 1
			}
		}
		fn max_tag_number() -> usize {
			TurnStyleEvent::InsertCoin as usize
		}
		fn from_tag(tag: usize) -> TurnStyleEvent {
			match tag {
				0 => TurnStyleEvent::Push,
				1 => TurnStyleEvent::InsertCoin,
				_ => panic!("invalid tag number")
			}
		}
	}

	#[test]
	fn test_machine() {
		let mut machine = Machine::new(TurnStyleState::Locked);
		machine.add_transition(
			TurnStyleState::Locked, TurnStyleEvent::InsertCoin,
			TurnStyleState::Unlocked, Box::new(|_,_| println!("unlocked"))
		);
		machine.add_transition(
			TurnStyleState::Unlocked, TurnStyleEvent::Push,
			TurnStyleState::Locked, Box::new(|_,_| println!("locked"))
		);
		assert!(machine.current_state() == TurnStyleState::Locked);
		machine.on_event(TurnStyleEvent::Push);
		assert!(machine.current_state() == TurnStyleState::Locked);
		machine.on_event(TurnStyleEvent::InsertCoin);
		assert!(machine.current_state() == TurnStyleState::Unlocked);
		machine.on_event(TurnStyleEvent::InsertCoin);
		assert!(machine.current_state() == TurnStyleState::Unlocked);
		machine.on_event(TurnStyleEvent::Push);
		assert!(machine.current_state() == TurnStyleState::Locked);
	}
}

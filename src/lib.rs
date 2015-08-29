
/// Actions are just boxed immutable functions that take an argument of the event that triggered them
pub type Action<'a, S, E> = Box<Fn(&S,&E) + 'a>;

/// Predicates are used to filter down whether a transition can occur
pub type Predicate<'a, S, E> = Box<Fn(&S,&E) -> bool + 'a>;

/// Trait that should be trivially implementable for any C-Like Enum type
pub trait EnumTag: Copy {
	/// returns the discriminator tag for the enum (some_value as usize)
	fn tag_number(&self) -> usize;
	/// returns the highest discriminator tag for this enum
	fn max_tag_number() -> usize;
}

/// The Transition records, for a given current state, what event type triggers it to move to
/// what state, performing a specific action on the transition, filterable by a predicate function
struct Transition<'a, S: EnumTag, E: EnumTag> {
	next_state: S,
	action: Action<'a, S, E>,
}

/// The StateTransition records all Transitions for a given state
struct StateTransitions<'a, S: EnumTag, E: EnumTag> {
	edges: Vec<Option<Transition<'a, S, E>>>,
}

/// The Machine is the Finite State Machine, which has a current state and set of all valid
/// transitions
pub struct Machine<'a, S: EnumTag, E: EnumTag> {
	state: S,
	transitions: Vec<StateTransitions<'a, S, E>>,
}

impl<'a, S: EnumTag, E: EnumTag> Machine<'a, S, E> {
	/// Constructs a new FSM with a given initial state
	pub fn new(initial_state: S) -> Machine<'a, S, E> {
		let mut transitions = Vec::with_capacity(S::max_tag_number());

		for _ in 0..S::max_tag_number() + 1 {
			let mut edges = Vec::with_capacity(E::max_tag_number());

			for _ in 0..E::max_tag_number() + 1 {
				edges.push(None);
			}

			transitions.push(StateTransitions {
				edges: edges,
			});
		}

		Machine {
			state: initial_state,
			transitions: transitions,
		}
	}

	/// Registers a new valid transition with the FSM
	pub fn add_transition<F>(&mut self, in_state: S, on_event: E, next_state: S, action: F) -> bool
	where F: Fn(&S, &E) + 'a{
		let transition = &mut self.transitions[in_state.tag_number()];

		let edge = &mut transition.edges[on_event.tag_number()];

		if edge.is_none() {
			*edge = Some(Transition {
				action: Box::new(action),
				next_state: next_state,
			});
			true
		} else {
			false
		}
	}

	/// Retrieves a reference to the current state
	pub fn current_state(&self) -> S {
		self.state
	}

	/// Tick the State Machine with an Event
	pub fn on_event(&mut self, event_type: E) {
		let transition = &self.transitions[self.state.tag_number()];
		let edge = &transition.edges[event_type.tag_number()];
		if let &Some(ref t) = edge {
			(*t.action)(&self.state, &event_type);
			self.state = t.next_state;
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	enum TurnStyleState {
		Locked,
		Unlocked,
	}

	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	enum TurnStyleEvent {
		Push,
		InsertCoin,
	}

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

	#[test]
	fn test_machine() {
		let mut machine = Machine::new(TurnStyleState::Locked);
		machine.add_transition(
			TurnStyleState::Locked, TurnStyleEvent::InsertCoin,
			TurnStyleState::Unlocked, |_,_| println!("unlocked")
		);
		machine.add_transition(
			TurnStyleState::Unlocked, TurnStyleEvent::Push,
			TurnStyleState::Locked, |_,_| println!("locked")
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

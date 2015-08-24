use std::sync::Arc;

/// Actions are just boxed immutable functions that take an argument of the event that triggered them
pub type Action<E> = Box<Fn(E)>;
/// StoredActions just enable structures holding the Action to be cloned
type StoredAction<E> = Arc<Action<E>>;

/// The Transition records, for a given current state, what event type triggers it to move to
/// what state, performing a specific action on the transition
#[derive(Clone)]
struct Transition<S: Into<u32> + Clone, E: Into<u32> + Clone> {
	event_type: E,
	next_state: S,
	action: StoredAction<E>,
}

/// The StateTransition records all Transitions for a given state
#[derive(Clone)]
struct StateTransitions<S: Into<u32> + Clone, E: Into<u32> + Clone> {
	start_state: S,
	edges: Vec<Option<Transition<S, E>>>,
}

/// The Machine is the Finite State Machine, which has a current state and set of all valid
/// transitions
#[derive(Clone)]
pub struct Machine<S: Into<u32> + Clone, E: Into<u32> + Clone> {
	state: S,
	transitions: Vec<Option<StateTransitions<S, E>>>,
}

impl<S: Into<u32> + Clone, E: Into<u32> + Clone> Machine<S, E> {
	/// Constructs a new FSM with a given initial state
	pub fn new(initial_state: S) -> Machine<S, E> {
		Machine {
			state: initial_state,
			transitions: Vec::new(),
		}
	}

	/// Registers a new valid transition with the FSM
	pub fn add_transition(&mut self, in_state: S, on_event: E, next_state: S, action: Action<E>) -> bool {
		let state_index: u32 = in_state.clone().into();
		let event_index: u32 = on_event.clone().into();
		let mut result = false;

		if state_index as usize >= self.transitions.capacity() {
			let new_len = self.transitions.len() * 2;
			self.transitions.reserve(new_len);
		}
		while state_index as usize >= self.transitions.len() {
			self.transitions.push(None);
		}

		let create_state_entry = match self.transitions[state_index as usize] {
			Some(_) => false,
			None => true,
		};

		if create_state_entry {
			self.transitions[state_index as usize] = Some(StateTransitions {
				start_state: in_state,
				edges: Vec::new(),
			});
		}

		if let &mut Some(ref mut transitions) = &mut self.transitions[state_index as usize] {
			if event_index as usize >= transitions.edges.len() {
				let new_len = transitions.edges.len() * 2;
				transitions.edges.reserve(new_len);
			}
			while event_index as usize >= transitions.edges.len() {
				transitions.edges.push(None);
			}

			let transition = &mut transitions.edges[event_index as usize];

			*transition = match &*transition {
				&Some(ref t) => Some((*t).clone()),
				&None => {
					result = true;
					Some(Transition {
						event_type: on_event,
						next_state: next_state,
						action: Arc::new(action),
					})
				}
			};
		}

		result
	}

	/// Retrieves a reference to the current state
	pub fn current_state(&self) -> &S {
		&self.state
	}

	/// Tick the State Machine with an Event
	pub fn on_event(&mut self, event_type: E) {
		let event_index: u32 = event_type.clone().into();
		let state_index: u32 = self.state.clone().into();
		
		if state_index as usize >= self.transitions.len() {
			return;
		}

		if let &Some(ref state_transitions) = &self.transitions[state_index as usize] {
			if event_index as usize >= state_transitions.edges.len() {
				return;
			}

			if let &Some(ref transition) = &state_transitions.edges[event_index as usize] {
				self.state = transition.next_state.clone();
				(*(*transition).action)(event_type);
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Clone, Debug, Eq, PartialEq)]
	enum TurnStyleState {
		Locked,
		Unlocked,
	}

	#[derive(Clone, Debug, Eq, PartialEq)]
	enum TurnStyleEvent {
		Push,
		InsertCoin,
	}

	impl Into<u32> for TurnStyleState {
		fn into(self) -> u32 {
			self as u32
		}
	}

	impl Into<u32> for TurnStyleEvent {
		fn into(self) -> u32 {
			self as u32
		}
	}

	#[test]
	fn test_machine() {
		let mut machine = Machine::new(TurnStyleState::Locked);
		machine.add_transition(
			TurnStyleState::Locked, TurnStyleEvent::InsertCoin,
			TurnStyleState::Unlocked, Box::new(|_| println!("unlocked"))
		);
		machine.add_transition(
			TurnStyleState::Unlocked, TurnStyleEvent::Push,
			TurnStyleState::Locked, Box::new(|_| println!("locked"))
		);
		assert!(*machine.current_state() == TurnStyleState::Locked);
		machine.on_event(TurnStyleEvent::Push);
		assert!(*machine.current_state() == TurnStyleState::Locked);
		machine.on_event(TurnStyleEvent::InsertCoin);
		assert!(*machine.current_state() == TurnStyleState::Unlocked);
		machine.on_event(TurnStyleEvent::InsertCoin);
		assert!(*machine.current_state() == TurnStyleState::Unlocked);
		machine.on_event(TurnStyleEvent::Push);
		assert!(*machine.current_state() == TurnStyleState::Locked);
	}
}

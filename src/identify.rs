use crate :: { import::* };


/// Interface for uniquely identifying actors. Mainly useful for logging and debugging.
//
pub trait Identify
{
	/// Get a unique identifier for this actor, so you can verify
	/// if two addresss deliver to the same actor.
	//
	fn id( &self ) -> usize;

	/// A human readable name of the actor.
	//
	fn name( &self ) -> Option< Arc<str> >;
}


use crate :: { * };

/// Interface specifying that a type can act as a mailbox for an actor.
//
pub trait Mailbox< A: Actor + Send > : Identify
{
	/// Return a future that allows starting the mailbox by spawning it on the executor of your choice.
	///
	#[ must_use = "Futures do nothing unless polled." ]
	//
	fn start_fut( self, actor: A ) -> Return<'static, ()>;
}


/// A mailbox that doesn't require the actor to be `Send`.
//
pub trait MailboxLocal< A: Actor > : Identify
{
	/// Return a non-Send future that allows starting the mailbox by spawning it on the executor of your choice.
	///
	#[ must_use = "Futures do nothing unless polled." ]
	//
	fn start_fut_local( self, actor: A ) -> ReturnNoSend<'static, ()>;
}

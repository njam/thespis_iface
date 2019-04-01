#![ allow( unused_imports, dead_code ) ]
#![ feature( await_macro, async_await, futures_api, arbitrary_self_types, specialization, nll, never_type, unboxed_closures, trait_alias, box_syntax, box_patterns, associated_type_defaults ) ]


mod actor     ;
mod address   ;
mod envelope  ;
mod handler   ;
mod mailbox   ;
mod message   ;
mod recipient   ;

pub use
{
	actor     :: * ,
	address   :: * ,
	envelope  :: * ,
	handler   :: * ,
	mailbox   :: * ,
	message   :: * ,
	recipient :: * ,
};


#[ cfg( feature = "derive" ) ]
//
pub use thespis_derive::{ Actor };


use std::{ pin::Pin, future::Future };
//
pub type           TupleResponse<'a   > = Pin<Box< dyn Future< Output = () > + 'a        >>;
pub type ThreadSafeTupleResponse<'a   > = Pin<Box< dyn Future< Output = () > + 'a + Send >>;
pub type                Response<'a, R> = Pin<Box< dyn Future< Output = R  > + 'a        >>;
pub type      ThreadSafeResponse<'a, R> = Pin<Box< dyn Future< Output = R  > + 'a + Send >>;


mod import
{
	pub use
	{
		std     :: { sync::Arc, pin::Pin, future::Future } ,
		futures :: { prelude::{ Stream, Sink }, channel::{ oneshot, mpsc }, task::Spawn } ,
	};
}




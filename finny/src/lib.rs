#![cfg_attr(not(feature = "std"), no_std)]

//! # Finny - Hierarchical Finite State Machines for Rust
//!
//! [![Crates.io][crates-badge]][crates-url]
//! [![Documentation](https://docs.rs/finny/badge.svg)](https://docs.rs/finny)
//! ![Build](https://github.com/hashmismatch/finny.rs/workflows/Build/badge.svg)
//!
//! ## Features
//! * Declarative, builder API with a procedural function macro that generate the dispatcher
//! * Compile-time transition graph validation
//! * No run-time allocations required, `no_std` support
//! * Support for generics within the shared context
//! * Transition guards and actions
//! * State regions, also known as orthogonal states
//! * Event queueing and run-to-completition execution
//! * Submachines, also known as Hierarchical State Machines
//! * Timers on states
//!
//! ## Example
//!
//! ### Cargo.toml
//!
//! ```toml
//! [dependencies]
//! finny = "0.2"
//! ```
//! 
//! ### Code
//! 
//! ```rust
//! use finny::{finny_fsm, FsmFactory, FsmResult, decl::{BuiltFsm, FsmBuilder}};
//! 
//! // The context is shared between all guards, actions and transitions. Generics are supported here!
//! #[derive(Default)]
//! pub struct MyContext { val: u32 }
//! // The states are plain structs.
//! #[derive(Default)] 
//! pub struct MyStateA { n: usize }
//! #[derive(Default)]
//! pub struct MyStateB;
//! // The events are also plain structs. They can have fields.
//! #[derive(Clone)]
//! pub struct MyEvent;
//!
//! // The FSM is generated by a procedural macro
//! #[finny_fsm]
//! fn my_fsm(mut fsm: FsmBuilder<MyFsm, MyContext>) -> BuiltFsm {
//!     // The FSM is described using a builder-style API
//!     fsm.state::<MyStateA>()
//!        .on_entry(|state, ctx| {
//!            state.n += 1;
//!            ctx.context.val += 1;
//!         })
//!        .on_event::<MyEvent>()
//!        .transition_to::<MyStateB>()
//!        .guard(|_ev, ctx, _states| { ctx.context.val > 0 })
//!        .action(|_ev, ctx, state_a, state_b| { ctx.context.val += 1; });
//!     fsm.state::<MyStateB>();
//!     fsm.initial_state::<MyStateA>();
//!     fsm.build()
//! }
//! 
//! // The FSM is built and tested.
//! fn main() -> FsmResult<()> {
//!     let mut fsm = MyFsm::new(MyContext::default())?;
//!     assert_eq!(0, fsm.val);
//!     fsm.start()?;
//!     let state_a: &MyStateA = fsm.get_state();
//!     assert_eq!(1, state_a.n);
//!     assert_eq!(1, fsm.val);
//!     fsm.dispatch(MyEvent)?;
//!     assert_eq!(2, fsm.val);
//!     Ok(())
//! }
//! ```
//! [crates-badge]: https://img.shields.io/crates/v/finny.svg
//! [crates-url]: https://crates.io/crates/finny

pub mod decl;
mod fsm;


pub mod inspect;
pub mod timers;

pub use fsm::*;

extern crate finny_derive;
extern crate derive_more;

/// The procedural macro that will transform the builder function into the FSM.
pub use finny_derive::finny_fsm;

/// External bundled libraries to be used by the procedural macros.
pub mod bundled {
    /// Derive_more crate for deriving the enum conversions.
    pub mod derive_more {
        pub use crate::derive_more::From;
    }
}

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
   }

   pub use self::core::marker::{PhantomData};
   pub use self::core::ops::{Deref, DerefMut};
   pub use self::core::fmt::Debug;
   pub use self::core::result::Result;
   pub use self::core::fmt;
   pub use self::core::any::type_name;
   pub use self::core::time::Duration;

   #[cfg(feature="std")]
   pub use std::collections::VecDeque;
}
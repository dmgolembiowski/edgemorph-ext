use staticvec::{staticvec, StaticVec};
use generic_array::{ArrayLength, GenericArray};

/// "A [`VirtualMachine`](crate::virtual_machine::VirtualMachine)
/// has a run loop that goes through a (1) fetch, (2) decode, and (3) execute
/// cycle." -- adapted from *Writing a Compiler in Go* by THorsten Ball
///
/// To help users embed their own ideas into EdgeDB, this struct provides
/// a framework for making it happen. 
pub trait VirtualMachine<'vm, P, const S: usize, T>
where P: 'vm + Program<T: 'vm + Op>, 
      T: 'vm + Op {
    type prgm_counter: i32;
    type stack: StaticVec<T, Self::S>;

}

/// "An [`Op`](crate::virtual_machine::Op) is the type implementing the
/// [`virtual machine`](crate::virtual_machine::VirtualMachine)'s stack slice
/// contract. The layout is relatively straightforward. Each [`Op`](crate::virtual_machine::Op) is an ordered collection of:
///   - (1) a tuple struct which wraps the op-code
///   - (2) any of the types (`T`, `&T`, `&'mut T)`) implementing [`From<Args>`]()
pub trait Op {
}



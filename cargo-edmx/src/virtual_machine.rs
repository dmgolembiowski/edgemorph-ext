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
    type stmt: Stmt;
    type args: Vec<Args>;
    type ret:  Result<(EdgeSet, Stmt)>;
    fn call(&Self) -> Self::ret;
}

/// "A [`Stmt`](crate::virtual_machine::Stmt) is the type describing how your 
/// [`virtual machine`](crate::virtual_machine::VirtualMachine) should interact with the database.
/// In particular, if you wanted to implement some logical chain or sequence of calls, but 
/// need to  [`Stmt::Update`](crate::virtual_machine::Stmt) some records if a particular
/// [`Stmt::Select`](crate::virtual_machine::Stmt) call comes back with something pre-populated,
/// then you would want to use [`Stmt::UpdateFuture`](crate::virtual_machine::Stmt) instead.
///
/// Each of the `Future` extended statements are designed to reduce down to their non-`Future`
/// counterparts, that is -- once it is their turn to evaluate. Since `Future`s can be cancelled,
/// [`Result<(EdgeSet, Stmt)>`](crate::virtual_machine::Op) tells the caller whether to proceed with
/// dispatching a callback function of the corresponding [`Stmt`](crate::virtual_machine::Stmt) kind.
pub enum Stmt {
    Delete, DeleteFuture,
    Filter, FilterFuture,
    Select, SelectFuture,
    Union,  UnionFuture,
    Update, UpdateFuture
}

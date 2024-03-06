extern crate proc_macro;
use proc_macro::TokenStream;

/// The main macro of the ascent library. Allows writing logical inference rules similar to Datalog.
/// 
/// Example:
/// ```
/// # #[macro_use] extern crate ascent_macro;
/// # use ascent::ascent;
/// ascent!{
///   relation edge(i32, i32);
///   relation path(i32, i32);
///   
///   path(x, y) <-- edge(x,y);
///   path(x, z) <-- edge(x,y), path(y, z);
/// }
/// 
/// fn main() {
///   let mut tc_comp = AscentProgram::default();
///   tc_comp.edge = vec![(1,2), (2,3)];
///   tc_comp.run();
///   println!("{:?}", tc_comp.path);
/// }
/// ```
/// this macro creates a type named `AscentProgram` that can be instantiated using `AscentProgram::default()`.
/// The type has a `run()` method, which runs the computation to a fixed point.
#[proc_macro]
pub fn ascent(input: TokenStream) -> TokenStream {
   let res = ascent_compile::ascent_impl(input.into(), false, false);
   
   match res {
      Ok(res) => res.into(),
      Err(err) => TokenStream::from(err.to_compile_error()),
   }
}

#[proc_macro]
pub fn ascent_par(input: TokenStream) -> TokenStream {
   let res = ascent_compile::ascent_impl(input.into(), false, true);
   
   match res {
      Ok(res) => res.into(),
      Err(err) => TokenStream::from(err.to_compile_error()),
   }
}


/// Like `ascent`, except that the result of an `ascent_run` invocation is a value containing all the relations
/// defined inside the macro body, and computed to a fixed point.
/// 
/// The advantage of `ascent_run` compared to `ascent` is the fact that `ascent_run` has access to local variables
/// in scope:
/// ```
/// # #[macro_use] extern crate ascent;
/// # use ascent::ascent_run;
/// let r = vec![(1,2), (2,3)];
/// let r_tc = ascent_run!{
///    relation tc(i32, i32);
///    tc(x, y) <-- for (x, y) in r.iter();
///    tc(x, z) <-- for (x, y) in r.iter(), tc(y, z);
/// }.tc;
///
/// ```
#[proc_macro]
pub fn ascent_run(input: TokenStream) -> TokenStream {
   let res = ascent_compile::ascent_impl(input.into(), true, false);
   
   match res {
      Ok(res) => res.into(),
      Err(err) => TokenStream::from(err.to_compile_error()),
   }
}

#[proc_macro]
pub fn ascent_run_par(input: TokenStream) -> TokenStream {
   let res = ascent_compile::ascent_impl(input.into(), true, true);
   
   match res {
      Ok(res) => res.into(),
      Err(err) => TokenStream::from(err.to_compile_error()),
   }
}

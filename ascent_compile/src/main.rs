use std::{env::args, str::FromStr};

use proc_macro2::TokenStream;

fn main() {
    let input = std::fs::read_to_string(&args().nth(1).unwrap()).unwrap();
   let result =
      ascent_compile::ascent_impl(TokenStream::from_str(&input).unwrap(), true, false).unwrap();
   println!("{}", result);
}

#![allow(warnings)]
#![allow(unused_imports)]
mod tests;
mod ascent_mir;
mod utils;
mod ascent_hir;
mod scratchpad;
mod ascent_codegen;
mod ascent_syntax;
mod test_errors;
mod syn_utils;

#[macro_use]
extern crate quote;

use ascent_syntax::{AscentProgram, desugar_ascent_program};
use syn::Result;
use crate::{ascent_codegen::compile_mir, ascent_hir::compile_ascent_program_to_hir, ascent_mir::compile_hir_to_mir};

pub fn ascent_impl(input: proc_macro2::TokenStream, is_ascent_run: bool, is_parallel: bool) -> Result<proc_macro2::TokenStream> {
   let prog: AscentProgram = syn::parse2(input)?;
   // println!("prog relations: {}", prog.relations.len());
   // println!("parse res: {} relations, {} rules", prog.relations.len(), prog.rules.len());

   let prog = desugar_ascent_program(prog)?;
   
   let hir = compile_ascent_program_to_hir(&prog, is_parallel)?;
   // println!("hir relations: {}", hir.relations_ir_relations.keys().map(|r| &r.name).join(", "));

   let mir = compile_hir_to_mir(&hir)?;

   // println!("mir relations: {}", mir.relations_ir_relations.keys().map(|r| &r.name).join(", "));

   let code = compile_mir(&mir, is_ascent_run);

   Ok(code)
}

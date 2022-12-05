use proc_macro::TokenStream;
use quote::quote;
use std::env;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let file_name = env::args().nth(2).unwrap();
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Str(suffix))] => format!(
            "../../inputs/{}-{}.in",
            file_name,
            suffix.token().to_string().replace("\"", "")
        ),
        _ => format!("../../inputs/{}.in", file_name),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
      const INPUT: &str = include_str!(#input_path);
      #aoc_solution
      fn main() {
        let now = ::std::time::Instant::now();
        let (p1, p2) = aoc_solution(INPUT.trim_end());
        let elapsed = now.elapsed();
        println!("Part one: {}", p1);
        println!("Part two: {}", p2);
        if elapsed.as_millis() > 0 {
          println!("Time: {}ms", elapsed.as_millis());
        } else {
          println!("Time: {}μs", elapsed.as_micros());
        }
      }
    };
    TokenStream::from(tokens)
}

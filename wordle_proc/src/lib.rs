use proc_macro::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::{BufRead, BufReader};
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn include_wordlist(input: TokenStream) -> TokenStream {
    let filename = parse_macro_input!(input as LitStr).value();
    let file =
        File::open(&filename).unwrap_or_else(|e| panic!("Error opening file {}: {}", filename, e));

    let words = BufReader::new(file)
        .lines()
        .filter_map(|line| {
            let s = line.ok()?;
            let chars: Vec<char> = s.to_lowercase().chars().collect();
            if chars.len() == 5 { Some(chars) } else { None }
        })
        .collect::<Vec<_>>();

    let arrays = words.iter().map(|chars| {
        let c0 = chars[0];
        let c1 = chars[1];
        let c2 = chars[2];
        let c3 = chars[3];
        let c4 = chars[4];
        quote!([#c0, #c1, #c2, #c3, #c4])
    });

    TokenStream::from(quote! {
        [#(#arrays),*]
    })
}

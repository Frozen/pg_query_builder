// use syn::*;
// use proc_macro::{TokenStream};
// use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
// use std::str::FromStr;
// use quote::{ToTokens};
// use std::iter::FromIterator;
// use std::ops::Index;
// use quote::quote;
//
//
// #[proc_macro_attribute]
// pub fn meta(arg: TokenStream, input: TokenStream) -> TokenStream {
//     // let macro_args: Args = match parse(arg) {
//     //     Ok(arg) => arg,
//     //     Err(..) => panic!("#[adorn()] takes a single identifier input, followed by optional literal parameters in parentheses"),
//     // };
//
//     // let name = "abc".to_string();
//
//     let input: ItemStruct = match parse(input) {
//         Ok(input) => input,
//         Err(..) => panic!("#[model()] must be applied on structs"),
//     };
//
//     let name = &input.ident;
//
//     // if input.fields.is_empty() {
//     //     panic!("#[model()] does not work with where clauses")
//     // }
//
//     let mut body = Vec::with_capacity(input.fields.len());
//     for f in &input.fields {
//         let field_name = f.ident.as_ref().unwrap();
//         let str = field_name.to_string();
//         let outer = quote!(
//             pub fn #field_name () -> Op {
//                  Op::new(#str)
//             }
//         );
//         body.push(outer);
//     }
//
//     let rs = quote! {
//
//         #input
//
//         impl #name {
//             #(#body)*
//
//         }
//     };
//     rs.into()
//
//     // let id: Ident = "_decorated_fn".into();
//     // let old_ident =  mem::replace(&mut input.ident, id);
//     // let mut i = 0;
//     // let mut exprs = Vec::with_capacity(input.decl.inputs.len()+1);
//     // exprs.push(quote!(#id));
//     // for extra_arg in macro_args.extra {
//     //     exprs.push(quote!(#extra_arg));
//     // }
//     // let mut args = vec!();
//     // for arg in input.decl.inputs.iter() {
//     //     let arg_ident: Ident = format!("_arg_{}", i).into();
//     //
//     //     match *arg {
//     //         FnArg::Captured(ref cap) => {
//     //             let ty = &cap.ty;
//     //             args.push(quote!(#arg_ident: #ty));
//     //         }
//     //         _ => panic!("Unexpected argument {:?}", arg)
//     //     }
//     //     exprs.push(quote!(#arg_ident));
//     //     i += 1;
//     // }
//     //
//     //
//     // let decorator = &macro_args.name;
//     // let attributes = &input.attrs;
//     // let vis = &input.vis;
//     // let constness = &input.constness;
//     // let unsafety = &input.unsafety;
//     // let abi = &input.abi;
//     // let generics = &input.decl.generics;
//     // let output = &input.decl.output;
//     // let outer = quote!(
//     //     #(#attributes),*
//     //     #vis #constness #unsafety #abi fn #old_ident #generics(#(#args),*) #output {
//     //         #input
//     //         #decorator(#(#exprs),*)
//     //     }
//     // );
//     //
//     // outer.into()
// }
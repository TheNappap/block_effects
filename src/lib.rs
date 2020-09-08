//! With `block_effects`, it is possible to chain different block effects with the use of the `block!` macro.
//!
//! This idea was proposed by [mcy](https://internals.rust-lang.org/t/eliminating-seemingly-unnecessary-braces/12971).
//! 
//! # Goal
//! The main purpose of this crate is to minimize indentation for nested blocks.
//! Unfortunately proc macros also add indentation. 
//! So to fully benefit from this one should not indent the `block!` macro or even put the macro braces on the same line as the block.
//!
//! # Example
//!``` rust
//! use block_effects::block;
//!
//! block!{
//! if let Some(_) = Some(0) if let Some(_) = Some(0) {
//!     assert!(true); 
//! }
//! }
//!
//! let _future = block!{ unsafe async for i in 0..3 match i { 
//!     0..=2 => assert!(true),
//!     _ => assert!(false)
//! } };
//!```

extern crate proc_macro;
use proc_macro::{TokenStream,TokenTree,Group,Delimiter};
use std::iter::FromIterator;

const KEYWORDS : [&str;7] = ["unsafe","async","loop","for","while","if","match"];

fn is_start_token(token: &TokenTree) -> bool {
    match token {
        TokenTree::Ident(id) => KEYWORDS.contains(&&id.to_string()[..]),
        _ => false
    }
}

fn is_block(token: &TokenTree) -> bool {
    match token {
        TokenTree::Group(gr) => gr.delimiter() == Delimiter::Brace,
        _ => false
    }
}

#[derive(Clone)]
enum BlockElement {
    Headers(Vec<TokenTree>),
    Group(TokenTree)
}

fn expand_block(tokens: &Vec<TokenTree>) -> Vec<TokenTree> {
    let block_elements = tokens.into_iter().fold((Vec::<BlockElement>::new(),Vec::<TokenTree>::new()),|(bs,hs),t|{
        if is_block(t) {
            let mut bs = bs;
            if hs.len() > 0 {
                bs.push(BlockElement::Headers(hs));
            }
            bs.push(BlockElement::Group(t.clone()));
            (bs,Vec::new())
        } else if is_start_token(t) {
            let mut bs = bs;
            if hs.len() > 0 {
                bs.push(BlockElement::Headers(hs));
            }
            (bs,vec![t.clone()])
        } else {
            let mut hs = hs;
            hs.push(t.clone());
            (bs,hs)
        }
    }).0;
    
    let mut tokens = Vec::new();
    let last = if let Some(BlockElement::Group(gr)) = block_elements.last() {
        gr.clone()
    } else { unreachable!(); };
    tokens.push(last);
    for el in block_elements.iter().rev().skip(1) {
        if let BlockElement::Headers(hs) = el {
            let mut ts = hs.clone();
            if tokens.len() == 1 { ts.push(tokens[0].clone()) }
            else {
                ts.push(Group::new(Delimiter::Brace, TokenStream::from_iter(tokens)).into());
            }
            tokens = ts;
        } else { unreachable!(); }
    }
    tokens
}

/// The `block!` macro is used to mark blocks with chained block effects.
///
/// The following block effects can be chained:
/// `unsafe`, `async`, `if`, `match`, `loop`, `while` and `for`.
/// Additionally `if let` and `while let` are also possible.
///
/// # Remarks
///
/// Some combinations of chaining effects just don't work.
/// Here are some exceptions to keep in mind:
/// * Ignoring `unsafe`, an `async` should be in front of the chain, otherwise the returned `Future` can't be used.
/// * A `match` should be the last in line, because the body of a `match` block is different.
/// * An `else` block can only be used if the starting block effect is an `if`.
///
/// # Example
/// An Example with an `if let`, `for`, `loop` and `match` effects:
///``` rust
/// use block_effects::block;
///
/// block!{
///     if let Some(_) = Some(0) for i in 0..4 loop match i {
///         0..=3 => { 
///             assert!(true); 
///             break;
///         },
///         _ => assert!(false)
///     } else {
///         assert!(false);
///     }
/// }
///```
#[proc_macro]
pub fn block(tokens: TokenStream) -> TokenStream {
    tokens.into_iter().scan(Vec::new(), |state, token| {
        if !state.is_empty() || is_start_token(&token) {
            state.push(token.clone());
            if is_block(&token) {
                let expanded = expand_block(state);
                *state = Vec::new();
                Some(expanded)
            } else { Some(vec![]) }
        } else {
            Some(vec![token])
        }
    }).flatten().collect()
}
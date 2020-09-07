
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
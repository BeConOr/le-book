use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, ItemFn, ItemStruct, Pat, PatIdent, PatType, Path};

#[proc_macro_attribute]
pub fn signal(attr: TokenStream, item: TokenStream) -> TokenStream {
    use quote::quote;
    use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, ItemStruct, Path};

    let parsed_args = parse_macro_input!(attr with Punctuated::<Path, Comma>::parse_terminated);
    let args: Vec<_> = parsed_args.into_iter().collect();
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let tuple_type = if args.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#args),*) }
    };

    let arg_idents: Vec<syn::Ident> = (0..args.len())
        .map(|i| syn::Ident::new(&format!("a{}", i), proc_macro2::Span::call_site()))
        .collect();

    let emit_args = quote! { #( #arg_idents : #args ),* };
    let call_tuple = quote! { &(#(#arg_idents),*) };

    let expanded = quote! {
        pub struct #struct_name {
            slots: std::collections::HashMap<SlotId, WeakSlotRef<#tuple_type>>,
            next_id: SlotId,
        }

        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    slots: std::collections::HashMap::new(),
                    next_id: 0,
                }
            }

            pub fn connect(&mut self, slot: SlotRef<#tuple_type>) -> SlotId {
                let id = self.next_id;
                self.next_id += 1;
                self.slots.insert(id, std::rc::Rc::downgrade(&slot));
                id
            }

            pub fn disconnect(&mut self, id: SlotId) -> bool {
                self.slots.remove(&id).is_some()
            }

            pub fn emit(&mut self, #emit_args) {
                for weak in self.slots.values() {
                    if let Some(slot) = weak.upgrade() {
                        slot.borrow_mut().call(#call_tuple);
                    }
                }
            }

            pub fn clean(&mut self) {
                self.slots.retain(|_, weak| weak.upgrade().is_some());
            }
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn slot(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let vis = &input_fn.vis;
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;

    // Преобразуем в Vec
    let inputs_vec: Vec<_> = inputs.iter().collect();

    let mut arg_idents = Vec::new();
    let mut arg_types = Vec::new();

    for input in inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                arg_idents.push(ident.clone());
                arg_types.push(ty.clone());
            }
        }
    }

    let tuple_type = quote! { (#(#arg_types),*) };
    let tuple_pattern = quote! { (#(#arg_idents),*) };
    let call_args = quote! { #(#arg_idents.clone()),* };

    let wrapper_name = syn::Ident::new(&format!("{}", fn_name), fn_name.span());

    let expanded = quote! {
        #vis fn #fn_name(#(#inputs_vec),*) #fn_block

        #vis let #wrapper_name: SlotRef<#tuple_type> =
            ::std::rc::Rc::new(::std::cell::RefCell::new(move |args: &#tuple_type| {
                let #tuple_pattern = args;
                #fn_name(#call_args);
            }));
    };

    expanded.into()
}

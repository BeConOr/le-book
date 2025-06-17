use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, ItemStruct, Path};

#[proc_macro_attribute]
pub fn signal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_args = parse_macro_input!(attr with Punctuated::<Path, Comma>::parse_terminated);
    let args: Vec<_> = parsed_args.into_iter().collect();
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let signal_type = if args.is_empty() {
        quote! { () }
    } else {
        quote! { #(#args),* }
    };

    let expanded = quote! {
        pub struct #struct_name {
            signal: Signal<#signal_type>,
        }

        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    signal: Signal::new(),
                }
            }

            pub fn connect(&mut self, slot: SlotRef<#signal_type>) -> SlotId {
                self.signal.connect(slot)
            }

            pub fn disconnect(&mut self, id: SlotId) -> bool {
                self.signal.disconnect(id)
            }

            pub fn emit(&mut self, args: &#signal_type) {
                self.signal.emit(args);
            }

            pub fn clean(&mut self) {
                self.signal.clean();
            }
        }
    };

    expanded.into()
}

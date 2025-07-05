use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Item, ItemFn, ItemStatic, parse_macro_input};

/// Marks the entry point of a Nintendo DS homebrew program.
#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let ident = &function.sig.ident;

    if ident != "main" {
        let message = "The #[entry] attribute can only be used on the `main` function";
        return compile_error(&ident, message);
    }

    quote! {
        #[unsafe(no_mangle)]
        extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
            use ::nds::sys::process::Termination;
            #[inline(always)]
            #function
            #ident().status()
        }
    }
    .into()
}

/// Places a `fn` or `static` into `Instruction Tightly Coupled Memory` (ITCM).
///
/// ITCM is a 32KB fast memory region on the ARM9 core of the Nintendo DS,
/// suited best for performance-critical functions or hot data paths.
#[proc_macro_attribute]
pub fn itcm(_args: TokenStream, input: TokenStream) -> TokenStream {
    let message = "The #[itcm] attribute can only be used on functions or static variables";
    match parse_macro_input!(input as Item) {
        Item::Fn(item_fn) => quote!(#[link_section = ".itcm.text"] #item_fn),
        // todo!(): Decide on how `.itcm.bss` is applied.
        Item::Static(item_static) => quote!(#[link_section = ".itcm.data"] #item_static),
        other => compile_error(&other, message).into(),
    }
    .into()
}

/// Places a `static` into `Data Tightly Coupled Memory` (DTCM).
///
/// DTCM is a 16KB fast memory region on the ARM9 core of the Nintendo DS,
/// suitable for latency-sensitive data like frequently accessed buffers or shared state.
#[proc_macro_attribute]
pub fn dtcm(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_static = parse_macro_input!(input as ItemStatic);
    // todo!(): Decide on how `.sbss` is applied.
    quote!(#[link_section = ".dtcm.data"] #item_static).into()
}

fn compile_error(tokens: &impl Spanned, message: &str) -> TokenStream {
    Error::new(tokens.span(), message).to_compile_error().into()
}

extern crate proc_macro;

use proc_macro::TokenStream;
use rifgen_attributes_utils::generate_impl_block;

#[proc_macro_attribute]
pub fn generate_interface(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse(item).unwrap();
    let mut is_func = false;

    match item {
        syn::Item::Fn(ref fun) => {
            //general function
            is_func = true;
            //generics not supported
            let gene = &fun.sig.generics;
            assert!(gene.gt_token.is_none(), "Generics not yet supported");
            assert!(gene.lt_token.is_none(), "Generics not yet supported")
        }
        syn::Item::Enum(_) => {}
        syn::Item::Trait(_) => {}
        syn::Item::Struct(_) => panic!(
            "Annotate methods of this struct instead. \
        To use enable doc comments on this struct use #[generate_interface_doc] macro instead."
        ),
        _ => panic!("unsuppoted type"),
    }
    let attr = attr.to_string();
    if !attr.is_empty() {
        assert_eq!(
            attr, "constructor",
            "only constructor attributes are supported for now"
        );
        if !is_func {
            panic!("call constructor on function")
        }
    }
    let y = quote::quote! {
        #item
    };
    y.into()
}

#[proc_macro_attribute]
pub fn generate_interface_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse(item).unwrap();
    match item {
        syn::Item::Struct(_) => {}
        _ => panic!("Use this macro on only struct`"),
    }
    assert!(attr.is_empty(), "No attributes allowed yet");
    let fin = quote::quote! {
        #item
    };
    fin.into()
}

/// Automatically generate constructor, setters and getters from a struct definition.
/// The getters should implement `Clone`.
///
/// C++ interfaces wouldn't need to implement `Clone`, however, I'm yet to implement that.
#[proc_macro_attribute]
pub fn generate_access_methods_no_cons(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::Item = syn::parse(item).unwrap();

    match ast {
        syn::Item::Struct(s) => {
            let impl_block = generate_impl_block(&s, false);
            let fin = quote::quote! {
                #[generate_interface_doc]
                #s

                #impl_block
            };
            fin.into()
        }
        _ => panic!("Use this macro on only struct`"),
    }
}

#[proc_macro_attribute]
pub fn generate_access_methods_cons(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::Item = syn::parse(item).unwrap();

    match ast {
        syn::Item::Struct(s) => {
            let impl_block = generate_impl_block(&s, true);
            let fin = quote::quote! {
                #[generate_interface_doc]
                #s

                #impl_block
            };
            fin.into()
        }
        _ => panic!("Use this macro on only struct`"),
    }
}

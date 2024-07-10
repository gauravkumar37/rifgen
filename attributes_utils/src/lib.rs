use itertools::MultiUnzip;
use quote::format_ident;
use syn::ItemImpl;

pub fn generate_impl_block(item: &syn::ItemStruct) -> ItemImpl {
    let name = item.clone().ident;
    let vis = item.clone().vis;
    let fields = match item.clone().fields {
        syn::Fields::Named(fields) => fields.named.into_iter().collect::<Vec<_>>(),
        _ => unreachable!(),
    };

    let (f_getter, f_ident, f_ty): (Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .cloned()
        .filter_map(|f| {
            if let Some(ident) = f.ident {
                if let syn::Visibility::Public(_) = f.vis {
                    Some((format_ident!("{}", ident), ident, f.ty))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .multiunzip();
    let impl_block = quote::quote! {
         impl #name {
            #(
                #[generate_interface]
                pub fn #f_getter(&self) -> #f_ty {
                    (&self.#f_ident).clone()
                }
            )*
        }
    };

    syn::parse2(impl_block).unwrap()
}

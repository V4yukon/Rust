type Variants = syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>;

/// Defines and implements `config_type` enum.
pub fn define_config_type_on_enum(em: &syn::ItemEnum) -> syn::Result<TokenStream> {
    let syn::ItemEnum {
        vis,
        enum_token,
        ident,
        generics,
        variants,
        ..
    } = em;

    let mod_name_str = format!("__define_config_type_on_enum_{}", ident);
    let mod_name = syn::Ident::new(&mod_name_str, ident.span());
    let variants = fold_quote(variants.iter().map(process_variant), |meta| quote!(#meta,));

    let impl_doc_hint = impl_doc_hint(&em.ident, &em.variants);
    let impl_from_str = impl_from_str(&em.ident, &em.variants);
    let impl_display = impl_display(&em.ident, &em.variants);
    let impl_serde = impl_serde(&em.ident, &em.variants);
    let impl_deserialize = impl_deserialize(&em.ident, &em.variants);

    Ok(quote! {
        #[allow(non_snake_case)]
        mod #mod_name {
            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub #enum_token #ident #generics { #variants }
            #impl_display
            #impl_doc_hint
            #impl_from_str
            #impl_serde
            #impl_deserialize
        }
        #vis use #mod_name::#ident;
    })
}

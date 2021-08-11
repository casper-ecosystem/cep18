#![no_std]

extern crate alloc;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, AttributeArgs, Expr, FnArg, ItemTrait, Pat,
    ReturnType, Token, TraitItem,
};

#[proc_macro_attribute]
pub fn contract_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    use alloc::string::ToString;

    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let contract_name = attr_args[0].clone();

    // Parse the input tokens into a syntax tree
    let input: ItemTrait = parse_macro_input!(input);

    let mut result = quote! {
        #input
    };

    let mut add_entry_points = quote! {};

    let mut call_fn = None;

    for item in input.items {
        // Parse the whole method.
        if let TraitItem::Method(method) = item {
            let method_name = method.sig.ident;
            let method_name_str = method_name.to_string();
            let return_type = match method.sig.output {
                ReturnType::Default => None,
                ReturnType::Type(_, ty) => Some(*ty),
            };
            let mut arg_loads = quote! {};
            let mut arg_names: Punctuated<Ident, Token![,]> = Punctuated::new();
            let mut params: Punctuated<Expr, Token![,]> = Punctuated::new();

            for arg in method.sig.inputs {
                if let FnArg::Typed(pat_type) = arg {
                    if let Pat::Ident(pat_ident) = *pat_type.pat {
                        let arg_name = &pat_ident.ident;
                        let arg_type = *pat_type.ty;
                        let arg_string = arg_name.to_string();
                        arg_loads.extend(quote! {
                            let #arg_name: #arg_type = casper_contract::contract_api::runtime::get_named_arg(#arg_string);
                        });
                        arg_names.push(arg_name.clone());
                        let param_def: TokenStream = quote! {
                            casper_types::Parameter::new(#arg_string, <#arg_type>::cl_type())
                        }
                        .into();
                        let param_def_parsed: Expr = parse_macro_input!(param_def);
                        params.push(param_def_parsed);
                    };
                };
            }

            let entry_point_return_type = match &return_type {
                None => quote! { <()>::cl_type() },
                Some(ty) => quote! { <#ty>::cl_type() },
            };

            if method_name_str == "constructor" {
                add_entry_points.extend(quote! {
                    entry_points.add_entry_point(casper_types::EntryPoint::new(
                        #method_name_str,
                        alloc::vec![#params],
                        #entry_point_return_type,
                        casper_types::EntryPointAccess::Groups(alloc::vec![casper_types::Group::new("constructor")]),
                        casper_types::EntryPointType::Contract
                    ));
                });
            } else {
                add_entry_points.extend(quote! {
                    entry_points.add_entry_point(casper_types::EntryPoint::new(
                        #method_name_str,
                        alloc::vec![#params],
                        #entry_point_return_type,
                        casper_types::EntryPointAccess::Public,
                        casper_types::EntryPointType::Contract
                    ));
                });
            }

            let return_stm = match &return_type {
                None => quote! { #contract_name{}.#method_name(#arg_names); },
                Some(ty) => quote! {
                    use casper_contract::unwrap_or_revert::UnwrapOrRevert;
                    let ret: #ty = #contract_name{}.#method_name(#arg_names);
                    casper_contract::contract_api::runtime::ret(casper_types::CLValue::from_t(ret).unwrap_or_revert());
                },
            };

            result.extend(quote! {
                #[no_mangle]
                fn #method_name() {
                    #arg_loads
                    #return_stm
                }
            });

            if method_name_str == "constructor" {
                let mut runtime_args = quote! {
                    let mut constructor_args = casper_types::RuntimeArgs::new();
                };

                for arg_name in arg_names {
                    let arg_name_str = arg_name.to_string();
                    runtime_args.extend(quote! {
                        constructor_args.insert(#arg_name_str, #arg_name).unwrap_or_revert();
                    })
                }

                call_fn = Some(quote! {
                    #[no_mangle]
                    fn call() {
                        use casper_contract::contract_api::{storage, runtime};
                        use casper_contract::unwrap_or_revert::UnwrapOrRevert;

                        let (package_hash, access_token) = storage::create_contract_package_at_hash();
                        let (contract_hash, _) = storage::add_contract_version(package_hash, get_entry_points(), Default::default());

                        #arg_loads
                        #runtime_args

                        let constructor_access: casper_types::URef = storage::create_contract_user_group(
                            package_hash, "constructor", 1, Default::default()
                        ).unwrap_or_revert().pop().unwrap_or_revert();

                        let _: () = runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

                        let mut urefs = alloc::collections::BTreeSet::new();
                        urefs.insert(constructor_access);
                        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs).unwrap_or_revert();

                        let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
                        runtime::put_key(&alloc::format!("{}_package_hash", contract_name), package_hash.into());
                        runtime::put_key(&alloc::format!("{}_package_hash_wrapped", contract_name), storage::new_uref(package_hash).into());
                        runtime::put_key(&alloc::format!("{}_contract_hash", contract_name), contract_hash.into());
                        runtime::put_key(&alloc::format!("{}_contract_hash_wrapped", contract_name), storage::new_uref(contract_hash).into());
                        runtime::put_key(&alloc::format!("{}_package_access_token", contract_name), access_token.into());
                    }
                });
            }
        }
    }

    result.extend(quote! {
        fn get_entry_points() -> casper_types::EntryPoints {
            use casper_types::CLTyped;
            let mut entry_points = casper_types::EntryPoints::new();
            #add_entry_points
            entry_points
        }
    });

    match call_fn {
        None => panic!("'construct' method not found!"),
        Some(call_fn) => {
            result.extend(quote! {
                #call_fn
            });
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(result)
}

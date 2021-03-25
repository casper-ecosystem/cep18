extern crate alloc;
extern crate proc_macro;
use std::ops::Add;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::{self, FnArg, Data, DeriveInput, Fields, GenericParam, Generics, GenericArgument, Type, Path, PathArguments, parse_macro_input, parse_quote, spanned::Spanned};

mod key {
    pub const __U64: &str = "u64";
    pub const __U512: &str = "U512";
    pub const __STRING: &str = "String";
}

#[proc_macro_derive(Context)]
pub fn casper_context(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: Context` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to make default of each field
    let defaults = create_default_fn(&input.data);

    // Generate an expression to make save of each field
    let saves = create_save_fn(&input.data);

    // Generate an expression to make GetKey impl
    let getkeys = create_getkey_fn(&input.data);

    let gen = quote! {
        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                #name {
                    #defaults
                }
            }
        }

        impl #impl_generics Save for #name #ty_generics #where_clause {
            fn save(&self) {
                #saves
            }
        }

        #getkeys
    };

    // Hand the output tokens back to the compiler.
    gen.into()
}

// The main macro that sets up the deploy function for the contract
// It loads the module of a given name and then iterates over the content of the
// module
#[proc_macro_attribute]
pub fn casper_contract(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::ItemMod = syn::parse_macro_input!(input);
    let name = &item.ident;
    let mut deploy_args = proc_macro2::TokenStream::new();
    let mut deploy_def = proc_macro2::TokenStream::new();
    let mut func_def = proc_macro2::TokenStream::new();
    match item.content {
        Some(module_content) => {
            let (deploy_def_content, func_def_content, deploy_args_content) =
                get_entry_points(name, module_content.1).unwrap();
            deploy_def.extend(deploy_def_content);
            func_def.extend(func_def_content);
            deploy_args.extend(deploy_args_content);
        }
        None => println!("Empty"),
    };
    let gen = quote! {
        fn __deploy( #deploy_args ) {
            #deploy_def
        }
        #func_def
        fn ret<T: CLTyped + ToBytes>(value: T) {
            runtime::ret(CLValue::from_t(value).unwrap_or_revert())
        }
    };
    // Return the deploy function along with the call function
    gen.into()
}

#[proc_macro_attribute]
pub fn casper_initiator(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // An optional macro that loads the initating function for the contract which returns the BTreeMap
    // The macro itself does not return the BTreeMap
    let item: syn::ItemFn = syn::parse_macro_input!(input);
    let func_def = quote! { #item };
    let gen = quote! {
        #func_def
    };
    // Return the function defintion for the intiating function
    gen.into()
}

#[proc_macro_attribute]
pub fn casper_constructor(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // The macro responsible for generating the constructor
    let mut item: syn::ItemFn = syn::parse_macro_input!(input);
    let mut input_strings: Vec<String> = Vec::new();
    let mut declaration = proc_macro2::TokenStream::new();
    let orignal_ident = item.sig.ident.clone();
    let name = &orignal_ident;
    let new_ident = Ident::new(&format!("__{}", name), name.span());
    item.sig.ident = new_ident;
    let internal = &item.sig.ident;
    let constructor_def = quote! { #item };
    if item.sig.inputs.is_empty() {
        let gen = quote! {
            #[no_mangle]
            fn call() {
                __deploy();
            }
            #constructor_def
            #[no_mangle]
            fn #name() {
                #internal()
            }
        };
        return gen.into();
    }
    for indiviual_input in item.sig.inputs {
        let (dec, arg, _, _) = get_var_declaration(&indiviual_input);
        declaration.extend(dec);
        input_strings.push(arg);
    }
    let input_args = prep_input(input_strings);
    let gen = quote! {
        #[no_mangle]
        fn call() {
            #declaration
            __deploy(#input_args)
        }
        #constructor_def
        #[no_mangle]
        fn #name() {
            #declaration
            #internal(#input_args)
        }
    };
    // Return the function defintion for the constructor function
    gen.into()
}

#[proc_macro_attribute]
pub fn casper_method(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::ItemFn = syn::parse_macro_input!(input);
    let orignal_ident = item.sig.ident.clone();
    let return_type = item.sig.output.clone();
    let mut return_code = proc_macro2::TokenStream::new();
    let mut return_func = proc_macro2::TokenStream::new();
    if let syn::ReturnType::Type(_arrow, rt) = return_type {
        if let syn::Type::Path(path) = *rt {
            let return_ident = &path.path.segments[0].ident;
            let code = quote! { let val: #return_ident = };
            return_code.extend(code);
        }
    }
    if !return_code.is_empty() {
        let runtime_return = quote! { ret(val) };
        return_func.extend(runtime_return)
    }
    let name = &orignal_ident;
    let new_ident = Ident::new(&format!("__{}", name), name.span());
    item.sig.ident = new_ident;
    let internal = &item.sig.ident;
    let func_def = quote! { #item };
    let mut declaration = proc_macro2::TokenStream::new();
    let mut input_strings: Vec<String> = Vec::new();
    if item.sig.inputs.is_empty() {
        let gen = quote! {
            #func_def
            #[no_mangle]
            fn #name() {
                #declaration
                #return_code #internal();
                #return_func
            }
        };
        return gen.into();
    }
    for indiviual_input in item.sig.inputs {
        let (dec, arg, _, _) = get_var_declaration(&indiviual_input);
        declaration.extend(dec);
        input_strings.push(arg);
    }
    let input_args = prep_input(input_strings);
    let gen = quote! {
        #func_def
        #[no_mangle]
        fn #name()  {
            #declaration
            #return_code #internal(#input_args);
            #return_func
        }
    };
    // Return both the entry point and the function defintion
    // #[no_mangle]
    // fn bar() {
    //     __bar()
    // }
    //  fn __bar() {}
    gen.into()
}

// Constructs a new entry point that can be inserted into
// the deploy function.
fn get_entry_points(
    name: &syn::Ident,
    funcs: Vec<syn::Item>,
) -> Option<(
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
)> {
    // The initial empty defintion Tokenstream that will be incrementally added to with various parts of the deploy function
    let mut definitions: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    // The start of the deploy function, which is mostly static and generic for any standard casper contract
    let mut init = quote! {
        let (contract_package_hash, _) = storage::create_contract_package_at_hash();
        let _constructor_access_uref: URef = storage::create_contract_user_group(
            contract_package_hash,
            "constructor",
            1,
            BTreeSet::new(),
        )
        .unwrap_or_revert()
        .pop()
        .unwrap_or_revert();
        let constructor_group = Group::new("constructor");
        let mut entry_points = EntryPoints::new();
    };
    // The two empty streams for arguments that must be passed to the deploy function and the content of the contract call respectively
    let mut deploy_args: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let mut contract_call: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    // The constructor function has a different EntryAccessPoint as compared to regular function
    // We must thus create an identifier to see which of the annotated function is the constructor
    let constructor_ident = Ident::new("casper_constructor", Span::call_site());
    // The intiating function must also be idenitfied so that the function can be placed in the deploy function
    let member_ident = Ident::new("casper_initiator", Span::call_site());
    let mut init_ident: proc_macro2::TokenStream = quote! { Default::default() };
    // Empty token stream that will change depending on the type of function passed to it (constructor/method)
    let mut access_token: proc_macro2::TokenStream;
    // Loop over every possible function and match to see if it is indeed a function, the statment could be a literal like
    //  'use::super::*;'
    //  For every #[casper_methods] in the contract module
    let mut constructor_presence: bool = false;
    for func in funcs {
        if let syn::Item::Fn(func_body) = func {
            // If the function is not a constructor or method it will have no attributes and we must ignore that function
            // Additionally, the function could also be an intiatior, which it means it should be ignored as well.
            if func_body.attrs.is_empty() {
                let gen = quote! { #func_body };
                definitions.extend(gen);
            } else if !func_body.attrs.is_empty()
                && member_ident != func_body.attrs[0].path.segments[0].ident
            {
                //  Identify if the function is a constructor or a regular method
                if constructor_ident == func_body.attrs[0].path.segments[0].ident {
                    constructor_presence = true;
                    access_token = quote! { EntryPointAccess::Groups(vec![constructor_group]) };
                    let (call, __d_args) = get_call(&func_body);
                    contract_call.extend(call);
                    deploy_args.extend(__d_args);
                } else {
                    //  EntryAccessPoint for a regular casper_method
                    access_token = quote! { EntryPointAccess::Public };
                }
                let name = &func_body.sig.ident;
                let def = quote! { #func_body };
                definitions.extend(def);
                let string_name = format!("{}", name);
                let mut arg: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                if !func_body.sig.inputs.is_empty() {
                    for input in func_body.sig.inputs {
                        let temp = get_param(&input);
                        arg.extend(temp);
                    }
                }
                let params = quote! {
                    vec![
                    #arg
                    ]
                };
                // Setup each entry point for the casper_method or one single entry point
                // for the constructor
                let gen = quote! {
                    entry_points.add_entry_point(EntryPoint::new(
                        String::from(#string_name),
                        #params,
                        CLType::Unit,
                        #access_token,
                        EntryPointType::Contract,
                    ));
                };
                init.extend(gen)
            } else if member_ident == func_body.attrs[0].path.segments[0].ident
                && !func_body.attrs.is_empty()
            {
                // If it is the intitator function, then set the identifier with the name of the specified function
                // Return the function definition
                let init_name = func_body.sig.ident.clone();
                let def = quote! { #func_body };
                definitions.extend(def);
                init_ident = quote! { #init_name() };
            } else if func_body.attrs.is_empty() {
                // Final path to ensure all non-associated functions are included within the contract
                let def = quote! { #func_body };
                definitions.extend(def);
            }
        }
        // Simply pass over all other literals
    }
    if !constructor_presence {
        panic!("No constructor was present");
    }
    let string_name = format!("{}", name);
    let string_hash_name = format!("{}_hash", string_name);
    //  Setup the tail end of the deploy function with remains mostly generic for all casper contracts
    let tail = quote! {
        let (contract_hash, _) = storage::add_contract_version(contract_package_hash, entry_points, #init_ident);
        runtime::put_key(#string_name,contract_hash.into());
        let contract_hash_pack = storage::new_uref(contract_hash);
        runtime::put_key(#string_hash_name, contract_hash_pack.into());
        #contract_call
    };
    init.extend(tail);
    // Finally once all fragments of the deploy function, defintions for all other functions and the arguments for the deploy function have been aggregated
    // Return the indiviual pieces
    Some((init, definitions, deploy_args))
}

// Construct the content for the contract call to be placed in the deploy method
fn get_call(init: &syn::ItemFn) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    // The name of the function that must be called
    let name = &init.sig.ident.clone();
    // Generate the string represenation of the function name
    let string_name = format!("{}", name);
    // Empty TokenStream for the inputs that will be given to the deploy function
    let mut input_for_deploy: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    // The call_contract method requires all the arguments that the constructor function
    // takes. Thus we create a new empty token stream for it
    let mut args: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    // For every argument in the arguments of the function defintion
    // Loop over and fill both token streams
    for indiviual_input in &init.sig.inputs {
        // Retrieve the arguments, the name for each argument, and the type
        //  _, somearg, "somearg", type_of_arg
        let (_, arg, arg_name, arg_type) = get_var_declaration(&indiviual_input);
        // Create an identifier for the argument
        let arg_ident = Ident::new(arg.as_str(), Span::call_site());
        let temp = quote! { #arg => #arg_ident, };
        let input_dec = quote! { #arg_name: #arg_type, };
        // Fill the arguments for the call_contract function
        args.extend(temp);
        // Fill the arguments for the deploy function
        input_for_deploy.extend(input_dec);
    }
    // Generate the code forthe call_contract function call
    let gen = quote! {
        runtime::call_contract::<()>(
            contract_hash,
            #string_name,
            runtime_args! {
                #args
            },
        );
    };
    // Return the call_contract code, along with the inputs for the deploy
    (gen, input_for_deploy)
}

fn get_param(arg: &FnArg) -> proc_macro2::TokenStream {
    //Generate a TokenSteam for each indiviual parameter for indiviual args passed to the functions
    let mut cl_type_declaration = proc_macro2::TokenStream::new();
    let arg_name: &syn::Ident;
    match arg {
        FnArg::Typed(ref pat_type) => {
            match *pat_type.pat {
                syn::Pat::Ident(ref pat_ident) => {
                    arg_name = &pat_ident.ident;
                }
                _ => panic!("Incorrect"),
            }
            cl_type_declaration.extend(get_parameter(&pat_type.ty));
        }
        _ => panic!("{:?}", arg),
    }
    let string_arg = format!("{}", arg_name);
    let declaration = quote! {Parameter::new(#string_arg, #cl_type_declaration),};
    declaration
}

// Generate the inputs for the function call
fn prep_input(input_strings: Vec<String>) -> proc_macro2::TokenStream {
    // When a function must be called we must create an identifier foe each of the args,
    // seperated by a comma
    let first_arg = Ident::new(&input_strings[0], Span::call_site());
    let mut args = quote! { #first_arg, };
    //
    for input_string in input_strings.iter().take(input_strings.len() - 1).skip(1) {
        //Create an Ident from the string representation of each of the args passed to the function
        let ident = Ident::new(&input_string, Span::call_site());
        let temp = quote! { #ident, };
        args.extend(temp);
    }
    if input_strings.len() == 1 {
        return args;
    }
    let last_ident = Ident::new(&input_strings[input_strings.len() - 1], Span::call_site());
    let last_arg = quote! { #last_ident };
    args.extend(last_arg);
    args
}

// If an function has the declaration: bar(a: u64, b: String)
// Then we must generate variable declartion for each argument
// let a: u64 = runtime::get_named_arg("a");
// let b: String = runtime::get_named_arg("b");
// The function returns the declaration for an indiviual argument
fn get_var_declaration(
    arg: &FnArg,
) -> (
    proc_macro2::TokenStream,
    String,
    &syn::Ident,
    proc_macro2::TokenStream,
) {
    // A helper function to parse the arguments of any function and return the associated declaration for the variable,
    // Its string representation, its identifier along with the type of the arg
    let arg_name: &syn::Ident;
    let mut arg_type = proc_macro2::TokenStream::new();
    match arg {
        FnArg::Typed(ref pat_type) => {
            match *pat_type.pat {
                syn::Pat::Ident(ref pat_ident) => {
                    arg_name = &pat_ident.ident;
                }
                _ => panic!("Incorrect"),
            }
            arg_type.extend(get_type(&pat_type.ty));
        }
        _ => panic!("{:?}", arg),
    }
    // Generate the declation
    let string_arg = format!("{}", arg_name);
    let dec = quote! {let #arg_name: #arg_type = runtime::get_named_arg(#string_arg); };
    let arg = format!("{}", arg_name);
    // Return
    // let some_arg: argype = runtime::get_named::arg("some_arg");
    // "some_arg"
    // "some_arg"
    // type of the arg
    (dec, arg, arg_name, arg_type)
}

fn get_type(arg_type: &syn::Type) -> proc_macro2::TokenStream {
    let mut type_stream = proc_macro2::TokenStream::new();
    match arg_type {
        syn::Type::Path(ref path) => {
            let arg_ident = &path.path.segments[0].ident;
            let arg_token = quote! { #arg_ident };
            type_stream.extend(arg_token);
            match path.path.segments[0].arguments {
                syn::PathArguments::None => type_stream,
                syn::PathArguments::AngleBracketed(ref arguments) => {
                    let opening_angle = quote! {<};
                    type_stream.extend(opening_angle);
                    for arg in &arguments.args {
                        match arg {
                            syn::GenericArgument::Type(ref generic_argument) => {
                                let generic_argument_stream = get_type(generic_argument);
                                type_stream.extend(quote! { #generic_argument_stream });
                            }
                            _ => panic!("In arg match"),
                        }
                    }
                    let closing_angle = quote! {>};
                    type_stream.extend(closing_angle);
                    type_stream
                }
                _ => type_stream,
            }
        }
        syn::Type::Tuple(ref tuple) => {
            let mut tuple_element_stream = proc_macro2::TokenStream::new();
            for elem in &tuple.elems {
                let tuple_element = get_type(elem);
                tuple_element_stream.extend(quote! {#tuple_element,});
            }
            type_stream.extend(quote! {(#tuple_element_stream)});
            type_stream
        }
        _ => type_stream,
    }
}

fn get_parameter(arg: &syn::Type) -> proc_macro2::TokenStream {
    let mut parameter_stream = proc_macro2::TokenStream::new();
    match arg {
        syn::Type::Path(ref path) => {
            let arg_ident = &path.path.segments[0].ident;
            match path.path.segments[0].arguments {
                syn::PathArguments::None => {
                    let declaration = get_cltype_from_parameter(arg_ident);
                    parameter_stream.extend(declaration);
                    parameter_stream
                }
                syn::PathArguments::AngleBracketed(ref arguments) => {
                    for arg in &arguments.args {
                        match arg {
                            syn::GenericArgument::Type(ref generic_argument) => {
                                let inner_cltype_declaration = get_parameter(generic_argument);
                                let box_wrapper = quote! {Box::new(#inner_cltype_declaration)};
                                let outer_cltype_declaration = get_cltype_from_parameter(arg_ident);
                                let merge_declarations =
                                    quote! {#outer_cltype_declaration(#box_wrapper)};
                                parameter_stream.extend(merge_declarations);
                            }
                            _ => panic!("In arg match"),
                        }
                    }
                    parameter_stream
                }
                _ => panic!("Tuple or other type"),
            }
        }
        syn::Type::Tuple(ref tuple) => {
            let num_of_elements: u64 = tuple.elems.len() as u64;
            let mut tuple_stream = proc_macro2::TokenStream::new();
            for elem in &tuple.elems {
                let tuple_element = get_parameter(elem);
                tuple_stream.extend(quote! {Box::new(#tuple_element),});
            }
            match num_of_elements {
                1 => parameter_stream.extend(quote! { CLType::Tuple1([#tuple_stream])}),
                2 => parameter_stream.extend(quote! { CLType::Tuple2([#tuple_stream])}),
                3 => parameter_stream.extend(quote! { CLType::Tuple3([#tuple_stream])}),
                _ => panic!("Unsupported tuple type"),
            }
            parameter_stream
        }
        _ => panic!("Tuple or other type"),
    }
}

fn get_cltype_from_parameter(arg_str: &Ident) -> proc_macro2::TokenStream {
    match format!("{}", arg_str).as_str() {
        "bool" => quote! { CLType::Bool },
        "i32" => quote! { CLType::I32 },
        "i64" => quote! { CLType::I64 },
        "u8" => quote! { CLType::U8 },
        "u32" => quote! { CLType::U32 },
        "u64" => quote! { CLType::U64 },
        "U128" => quote! { CLType::128 },
        "U256" => quote! { CLType::U256 },
        "U512" => quote! { CLType::U512 },
        "String" => quote! { CLType::String },
        "AccountHash" => quote! { AccountHash::cl_type() },
        "Key" => quote! { CLType::Key },
        "URef" => quote! { CLType::Uref },
        "Vec" => quote! { CLType::List },
        _ => quote! { CLType::Any },
    }
}

// Add a bound `T: Context` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(contract_vars::Context));
        }
    }
    generics
}

// Generate an expression to make default of each field
fn create_default_fn(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        let (main_ty, content) = match ty {
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_variable(&typepath.path) => {
                                let type_params = &typepath.path.segments.iter().next().unwrap().arguments;
                                let generic_arg = match type_params {
                                    PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
                                    _ => panic!("Variable: Default parse failed in generic arguments"),
                                };
                                match generic_arg {
                                    GenericArgument::Type(nty) => {
                                        let name_str = name.clone().unwrap().to_string();
                                        (&typepath.path.segments.iter().next().unwrap().ident, quote! { ::new(String::from(#name_str), #nty::default()) })
                                    },
                                    _ => panic!("Variable: Default parse failed in generic arguments"),
                                }
                            }
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_map(&typepath.path) => {
                                let name_str = name.clone().unwrap().to_string();
                                (&typepath.path.segments.iter().next().unwrap().ident, quote! { ::new(String::from(#name_str)) })
                            } 
                            _ => panic!("No parser: Default parse only exists for Variable and Map")
                        };
                        if main_ty == "Variable" || main_ty == "Map" {
                            quote_spanned! {f.span() => #name: #main_ty#content,}
                        } else {
                            quote! {}
                        }
                    });
                    quote! {#(#recurse)*}
                }
                _ => quote! {}
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Generate an expression to make save of each field
fn create_save_fn(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        let (main_ty, sub_recurse) = match ty {
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_variable(&typepath.path) => {
                                (&typepath.path.segments.iter().next().unwrap().ident, quote! { 
                                    if self.#name.has_change() {
                                        self.#name.set(); 
                                    }
                                })
                            }
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_map(&typepath.path) => {
                                (&typepath.path.segments.iter().next().unwrap().ident, quote! {})
                            } 
                            _ => panic!("No parser: Save parse only exists for Variable and Map")
                        };
                        if main_ty == "Variable" {
                            quote! {#sub_recurse}
                        } else {
                            quote! {}
                        }
                    });
                    quote! {#(#recurse)*}
                }
                _ => quote! {}
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Generate an expression to make GetKey impl
fn create_getkey_fn(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let ty = &f.ty;
                        let (main_ty, content) = match ty {
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_map(&typepath.path) => {
                                let type_params = &typepath.path.segments.iter().next().unwrap().arguments;
                                let generic_arg = match type_params {
                                    PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
                                    _ => panic!("Variable: Default parse failed in generic arguments"),
                                };
                                match generic_arg {
                                    GenericArgument::Type(nty) => {
                                        (&typepath.path.segments.iter().next().unwrap().ident, quote! { #nty })
                                    },
                                    _ => panic!("Variable: Default parse failed in generic arguments"),
                                }
                            } 
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_variable(&typepath.path) => {
                                (&typepath.path.segments.iter().next().unwrap().ident, quote! {})
                            } 
                            _ => panic!("No parser: GetKey parser only exists for Map")
                        };
                        if main_ty == "Map" {
                            let mut content_str = content.to_string().replace(&['(', ')', ' '][..], "");
                            let account_hash_cnt = content_str.matches("AccountHash").count();
                            content_str = content_str.replace("AccountHash", "").replace(",", "");
                            if !content_str.is_empty() || account_hash_cnt == 0 { 
                                panic!("Map key should be AccountHash or (AccountHash, ... )");
                            }
                            let mut format_str = String::from("{}");
                            let mut param_str = String::new();
                            for i in 0..account_hash_cnt {
                                format_str = format_str.add("_{}");
                                param_str = param_str.add(&format!("self.{}.to_string(),", i));
                            }
                            if account_hash_cnt == 1 {
                                param_str = String::from("self.to_string(),");
                            }
                            param_str.pop();
                            let param_ident = param_str.parse::<::proc_macro2::TokenStream>().unwrap();
                            quote! {
                                impl GetKey for #content {
                                    fn get_key(&self, prefix: &String) -> String {
                                        format!(#format_str, prefix, #param_ident)
                                    }
                                }
                            }
                        } else {
                            quote! {}
                        }
                    });
                    quote! {#(#recurse)*}
                }
                _ => quote! {}
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn path_is_variable(path: &Path) -> bool {
    path.leading_colon.is_none() && path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == "Variable"
}

fn path_is_map(path: &Path) -> bool {
    path.leading_colon.is_none() && path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == "Map"
}
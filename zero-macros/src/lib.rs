use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Lit, Expr, punctuated::Punctuated, Token};

#[proc_macro_attribute]
pub fn register_route(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args with Punctuated::<Expr, Token![,]>::parse_terminated);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    // Parse the route path and method from the attribute arguments
    if args.len() != 2 {
        return syn::Error::new_spanned(&args, "Expected exactly two arguments: path and method")
            .to_compile_error()
            .into();
    }
    
    let path_expr = &args[0];
    let method_expr = &args[1];
    
    // Extract string literals from expressions
    let path = match path_expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Str(lit_str) => lit_str.value(),
            _ => {
                return syn::Error::new_spanned(path_expr, "Expected string literal for path")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(path_expr, "Expected string literal for path")
                .to_compile_error()
                .into();
        }
    };
    
    let method = match method_expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Str(lit_str) => lit_str.value(),
            _ => {
                return syn::Error::new_spanned(method_expr, "Expected string literal for method")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(method_expr, "Expected string literal for method")
                .to_compile_error()
                .into();
        }
    };
    
    let fn_name = &input_fn.sig.ident;
    
    // Generate a unique identifier for the route builder function
    let route_builder_name = syn::Ident::new(
        &format!("__route_builder_{}", fn_name),
        fn_name.span(),
    );
    
    // Generate the expanded code
    let expanded = quote! {
        // Original function unchanged
        #input_fn
        
        // Route builder function
        fn #route_builder_name() -> (&'static str, axum::routing::MethodRouter) {
            match #method {
                "GET" => (#path, axum::routing::get(#fn_name)),
                "POST" => (#path, axum::routing::post(#fn_name)),
                "PUT" => (#path, axum::routing::put(#fn_name)),
                "DELETE" => (#path, axum::routing::delete(#fn_name)),
                "PATCH" => (#path, axum::routing::patch(#fn_name)),
                _ => panic!("Unsupported HTTP method: {}", #method),
            }
        }
        
        // Submit to inventory
        inventory::submit! {
            crate::handlers::RouteInfo {
                path: #path,
                method: #method,
                route_builder: #route_builder_name,
            }
        }
    };
    
    TokenStream::from(expanded)
}

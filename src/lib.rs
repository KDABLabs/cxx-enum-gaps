// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Nicolas Qiu Guichard <nicolas.guichard@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[allow(
    unused,
    unfulfilled_lint_expectations,
    mismatched_lifetime_syntaxes,
    clippy::precedence,
    clippy::toplevel_ref_arg,
    clippy::inherent_to_string,
    clippy::too_many_arguments
)]
mod syntax;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

use syntax::{Api, Enum, discriminant::Limits, file::Module, parse_items, report::Errors};

#[proc_macro_attribute]
pub fn cxx_enum_gaps(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = item.clone();
    let ffi = parse_macro_input!(module as Module);

    let mut err = Errors::new();

    std::iter::once(item)
        .chain(
            parse_items(&mut err, ffi.content, true, Default::default())
                .into_iter()
                .filter_map(|item| {
                    if let Api::Enum(enm) = item {
                        Some(enum_gaps_macro(enm))
                    } else {
                        None
                    }
                }),
        )
        .collect()
}

fn enum_gaps_macro(enm: Enum) -> TokenStream {
    let ident = enm.name.rust;

    let gaps = {
        // discriminants must be sorted to build the range patterns of the gaps between them
        let discriminants = {
            let mut discriminants: Vec<_> = enm
                .variants
                .iter()
                .map(|variant| variant.discriminant)
                .collect();
            discriminants.sort();
            discriminants
        };

        // When map_windows gets stabilized:
        // let gaps = std::iter::once(None)
        //     .chain(discriminants.into_iter().map(Some))
        //     .chain(std::iter::once(None))
        //     .map_windows::<_, _, 2>(|&[start, end]| match (start, end) ...);
        let gaps = {
            let starts =
                std::iter::once(None).chain(discriminants.iter().map(|d| d.checked_succ()));
            let ends = discriminants
                .iter()
                .copied()
                .map(Some)
                .chain(std::iter::once(None));

            starts.zip(ends)
        };

        let gaps = gaps.flat_map(|(start, end)| match (start, end) {
            (None, None) => None,
            // ..#end is not a valid range pattern when end is the first admissible value for the discriminant (typically 0 for unsigned discriminants)
            (None, Some(end))
                if let Some(limits) = Limits::of(enm.repr.atom)
                    && end == limits.min =>
            {
                None
            }
            (None, Some(end)) => Some(quote! { ..#end }),
            (Some(start), None) => Some(quote! { #start.. }),
            (Some(start), Some(end)) if start == end => None,
            (Some(start), Some(end)) => Some(quote! { #start..#end }),
        });

        quote! { #(#gaps)|* }
    };
    let gaps_macro = format_ident!("_{}", ident);

    quote! {
        #[macro_export] macro_rules! #gaps_macro {
            () => { #ident{ repr: #gaps } };
            ($i:ident) => { #ident{ repr: $i @ (#gaps) } };
        }
    }
    .into()
}

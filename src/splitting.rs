// Copyright (c) 2026 Mariusz Zacirka
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! split_by {
    (
        {$($tokens:tt)*} => {,} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)* } => $crate::split { $crate::is_comma{} } => $($cont_args)* }
    };
    (
        {$($tokens:tt)*} => {;} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)* } => $crate::split { $crate::is_semicolon{} } => $($cont_args)* }
    };
    (
        {$($tokens:tt)*} => {=>} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)* } => $crate::split { $crate::is_double_arrow{} } => $($cont_args)* }
    };
    (
        {$($tokens:tt)*} => {:} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)* } => $crate::split { $crate::is_colon{} } => $($cont_args)* }
    };
    (
        {$($tokens:tt)*} => {+} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)* } => $crate::split { $crate::is_plus{} } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! is_comma {
    (
        {,} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
    (
        { $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! is_semicolon {
    (
        {;} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
    (
        { $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! is_double_arrow {
    (
        {=>} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
    (
        { $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! is_colon {
    (
        {:} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
    (
        { $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! is_plus {
    (
        {+} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
    (
        { $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{apply_pipe, debrace, embrace, fork, reset};

    macro_rules! as_array {
        (
            {$($exps:expr)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {[$($exps),*]} => $($cont_args)* }
        };
    }

    #[test]
    fn split_by_colon() {
        //trace_macros!(true);
        let a = { apply_pipe! { {1+3, 2+3+4, 2} => split_by {,} => as_array {} } };
        //trace_macros!(false);
        assert_eq!(a, [1+3, 2+3+4, 2]);
    }

    macro_rules! add_to_map {
        (
            { {$key:expr} {$value:expr} } => { $map:ident } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $map.insert($key, $value); } => $($cont_args)* }
        };
    }

    macro_rules! join {
        (
            {$($exps:expr)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { { String::default() $( + $exps)* } } => $($cont_args)* }
        };
    }

    macro_rules! add {
        (
            {$($exps:expr)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { 0 $( + $exps)* } => $($cont_args)* }
        };
    }

    #[test]
    fn split_by_semicolon_then_double_arrow_then_colon() {
        let mut map = HashMap::new();
        //trace_macros!(true);
        apply_pipe! {
            { "a", "1" => 2 - 1, 3 - 2; "b" => 4 - 2, 5 - 3, 3; "c", "_1", "_2" => 6 - 3 }
            => split_by {;}
            => [
                debrace{}
                => split_by {=>}
                => fork {
                    { debrace{} => split_by{,} => join{} => embrace{} }
                    { debrace{} => split_by{,} => add{} => embrace{} }
                }
                => add_to_map { map }
            ]
        }
        //trace_macros!(false);
        let mut expected_map = HashMap::new();
        expected_map.insert(String::from("a1"), 1 + 1);
        expected_map.insert(String::from("b"), 2 + 2 + 3);
        expected_map.insert(String::from("c_1_2"), 3);
        assert_eq!(map, expected_map);
    }

    #[test]
    fn split_by_semicolon_then_double_arrow_ignoring_second_side_then_split_by_colon() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                { "a", "1" => 2 - 1, 3 - 2; "b" => 4 - 2, 5 - 3, 3; "c", "_1", "_2" => 6 - 3 }
                => split_by {;}
                => [
                    debrace{}
                    => split_by {=>}
                    => fork {
                        { debrace{} => split_by{,} => join{} }
                        { reset{} }
                    }
                ]
                => as_array{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ["a1", "b", "c_1_2"]);
    }
}
// Copyright (c) 2026 Mariusz Zacirka
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Pipe-compatible callback:
/// ```
/// macro_rules! example {
///     (
///         { $($input:tt)* } => { $($args:tt)* } => $cont:path { $($cont_args:tt)* }
///     ) => {
///         // somewhere call continuation callback
///         $cont! { { example output } => $($cont_args)* }
///     }
/// }
/// ```
#[macro_export]
macro_rules! apply_pipe {
    (
        $($tokens:tt)*
    ) => {
        $crate::continue_pipe! { $($tokens)* => $crate::__apply_pipe_apply {} => {} }
    };
}

#[macro_export]
macro_rules! continue_pipe {
    (
        { $input_head:tt $($input_tail:tt)* } => [ $($pipe_for_each:tt)+ ] => $($cont:tt)+
    ) => {
        $crate::continue_pipe! { { $input_head } => $($pipe_for_each)+ => $crate::__accumulate { { $($input_tail)* } => [ $($pipe_for_each)+ ] {} } => $($cont)+ }
    };
    (
        {} => [ $($pipe_for_each:tt)+ ] => $($cont:tt)+
    ) => {
        $crate::continue_pipe! { {} => $($cont)+ }
    };

    (
        { $($input:tt)* } => $f:path { $($f_args:tt)* } => $($cont:tt)+
    ) => {
        $f! { { $($input)* } => { $($f_args)* } => $crate::continue_pipe { /* { output-added-by-f } => */ $($cont)+ } }
    };
}

#[macro_export]
macro_rules! split {
    (
        { $($input:tt)* } => { $should_split:path { $($should_split_args:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($input)* } => $crate::__accumulate_split { {} => $should_split { $($should_split_args)* } => {} } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! de_brace {
    (
        {{$($tokens:tt)*}} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)*} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! fork {
    (
        { {$($head_input:tt)*} $($tail_inputs:tt)* } => { {$($head_pipe:tt)*} $($tail_pipes:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($head_input)*} => $($head_pipe)* => $crate::__accumulate_forked_pipes { {$($tail_inputs)*} => {$($tail_pipes)*} => {} } => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_forked_pipes {
    (
        { $($output:tt)* }
        => {
            { {$($head_input:tt)*} $($tail_inputs:tt)* }
            => { {$($head_pipe:tt)*} $($tail_pipes:tt)* }
            => { $($acc:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($head_input)*} => $($head_pipe)* => $crate::__accumulate_forked_pipes { {$($tail_inputs)*} => {$($tail_pipes)*} => { $($acc)* { $($output)* } } } => $($cont_args)* }
    };
    (
        { $($output:tt)* }
        => {
            {}
            => {}
            => { $($acc:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($acc)* { $($output)* } } => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate {
    (
        { $($output_tail:tt)* } => { { $input_head:tt $($input_tail:tt)* } => [ $($pipe_for_each:tt)+ ] { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $input_head } => $($pipe_for_each)+ => $crate::__accumulate { { $($input_tail)* } => [ $($pipe_for_each)+ ] {$($acc)*$($output_tail)*} } => $($cont_args)* }
    };
    (
        { $($output_tail:tt)* } => { {} => [ $($pipe_for_each:tt)+ ] { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($acc)*$($output_tail)*} => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __apply_pipe_apply {
    (
        { $($input:tt)* } => {} => $($cont_ignored:tt)+
    ) => {
        $($input)*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_split {
    (
        { $head:tt $($tail:tt)* } => { {$($group:tt)*} => $should_split:path { $($should_split_args:tt)* } => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            {
                { $head }
                then { { $($tail)* } => $crate::__accumulate_split { {} => $should_split { $($should_split_args)* } => {$($acc)*{$($group)*}} } }
                else { { $($tail)* } => $crate::__accumulate_split { {$($group)*$head} => $should_split { $($should_split_args)* } => {$($acc)*} } }
            }
            => $should_split { $($should_split_args)* }
            => $($cont_args)*
        }
    };
    (
        {} => { {$($group:tt)*} => $should_split:path { $($should_split_args:tt)* } => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($acc)*{$($group)*} } => $($cont_args)* }
    };
}

#[cfg(test)]
mod tests {
    macro_rules! as_array {
        (
            {$($exps:expr)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {[$($exps),*]} => $($cont_args)* }
        };
    }

    #[test]
    fn process_1_callback_in_pipe() {
        let a = { apply_pipe!{{1 3 4} => as_array {}} };
        assert_eq!(a, [1, 3, 4]);
    }

    macro_rules! split_semicolons {
        (
            {$($exps:tt);*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {$($exps)*} => $($cont_args)* }
        };
    }

    #[test]
    fn process_2_simple_callbacks_in_pipe() {
        let a = { apply_pipe!{{1;3;4} => split_semicolons{} => as_array{}} };
        assert_eq!(a, [1, 3, 4]);
    }

    macro_rules! join_with_colons {
        (
            {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {$($tokens),*} => $($cont_args)* }
        };
    }

    macro_rules! in_brackets {
        (
            {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { [$($tokens)*] } => $($cont_args)* }
        };
    }

    #[test]
    fn process_3_simple_callbacks_in_pipe() {
        let a = { apply_pipe!{{1;3;4} => split_semicolons{} => join_with_colons{} => in_brackets{}} };
        assert_eq!(a, [1, 3, 4]);
    }

    macro_rules! as_array_using_pipe {
        (
            {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {$($tokens)*} => join_with_colons{} => in_brackets{} => $($cont_args)* }
        };
    }

    #[test]
    fn process_with_pipe_inside_callback() {
        //trace_macros!(true);
        let a = { apply_pipe!{{1;3;4} => split_semicolons{} => as_array_using_pipe{}} };
        //trace_macros!(false);
        assert_eq!(a, [1, 3, 4]);
    }

    macro_rules! split_semicolons_to_array {
        (
            {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {$($tokens)*} => split_semicolons{} => as_array_using_pipe{} => $($cont_args)* }
        };
    }

    #[test]
    fn process_with_pipe_inside_callback_depth_2() {
        let a = { apply_pipe!{{1;3;4} => split_semicolons_to_array{}} };
        assert_eq!(a, [1, 3, 4]);
    }

    macro_rules! add {
        (
            {$($tokens:expr)*} => { $added:expr } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $($tokens)* + $added } => $($cont_args)* }
        };
    }

    #[test]
    fn process_callback_for_each_element_in_pipe() {
        //trace_macros!(true);
        let a = { apply_pipe!{{1;3;4} => split_semicolons{} => [ add{2} ] => as_array{}} };
        //trace_macros!(false);
        assert_eq!(a, [1+2, 3+2, 4+2]);
    }

    macro_rules! to_string {
        (
            {$($tokens:expr)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { {{$($tokens)*}.to_string()} } => $($cont_args)* }
        };
    }

    #[test]
    fn process_2_callbacks_for_each_element_in_pipe() {
        //trace_macros!(true);
        let a = { apply_pipe!{{1;3;4} => split_semicolons{} => [ add{3} => to_string{} ] => join_with_colons{} => in_brackets{}} };
        //trace_macros!(false);
        assert_eq!(a, ["4", "6", "7"]);
    }

    macro_rules! split_commas {
        (
            {$($exps:expr),*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {$($exps)*} => $($cont_args)* }
        };
    }

    macro_rules! join {
        (
            {$($exps:expr)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { { String::default() $( + &$exps)* } } => $($cont_args)* }
        };
    }

    #[test]
    fn process_2_levels_of_callbacks_for_each_element_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {{1,2};{3,4,5};{6}}
                => split_semicolons{}
                => [
                    de_brace{}
                    => split_commas{}
                    => [ to_string{} ]
                    => join{}
                ]
                => join_with_colons{}
                => in_brackets{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ["12", "345", "6"]);
    }

    macro_rules! split_by_colon {
        (
            { , $($tail:tt)* } => { {$($group:tt)*} => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $($tail)* } => split_by_colon { {} => {$($acc)*{$($group)*}} } => $($cont_args)* }
        };
        (
            { $head:tt $($tail:tt)* } => { {$($group:tt)*} => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $($tail)* } => split_by_colon { {$($group)*$head} => {$($acc)*} } => $($cont_args)* }
        };
        (
            {} => { {$($group:tt)*} => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $($acc)*{$($group)*} } => $($cont_args)* }
        };
    }

    #[test]
    fn split_by_colon_in_pipe() {
        //trace_macros!(true);
        let a = { apply_pipe! { {1+3, 2+3+4, 2} => split_by_colon { {} => {} } => as_array {} } };
        //trace_macros!(false);
        assert_eq!(a, [1+3, 2+3+4, 2]);
    }

    macro_rules! is_colon {
        (
            { {,} then {$($yes_input:tt)*} else {$($no_input:tt)*} } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { $($yes_input)* => $($cont_args)* }
        };
        (
            { {$token:tt} then {$($yes_input:tt)*} else {$($no_input:tt)*} } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { $($no_input)* => $($cont_args)* }
        };
    }

    #[test]
    fn split_by_is_colon_in_pipe() {
        //trace_macros!(true);
        let a = { apply_pipe! { {1+3, 2+3+4, 2} => split { is_colon {} } => as_array {} } };
        //trace_macros!(false);
        assert_eq!(a, [1+3, 2+3+4, 2]);
    }

    macro_rules! is_semicolon {
        (
            { {;} then {$($yes_input:tt)*} else {$($no_input:tt)*} } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { $($yes_input)* => $($cont_args)* }
        };
        (
            { {$token:tt} then {$($yes_input:tt)*} else {$($no_input:tt)*} } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { $($no_input)* => $($cont_args)* }
        };
    }

    #[test]
    fn split_by_semicolon_then_colon_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {1+2, 3; 4, 5+6+7, 8; 9}
                => split { is_semicolon {} }
                => [
                    de_brace {}
                    => split { is_colon {} }
                    => [ to_string{} ]
                    => join {}
                ]
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ["33", "4188", "9"]);
    }
}
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
macro_rules! split_prefix_until {
    (
        { $($input:tt)* } => { $should_split:path { $($should_split_args:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($input)* } => $crate::__accumulate_split_prefix_until { $should_split { $($should_split_args)* } => {} } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! debrace {
    (
        {{$($tokens:tt)*}} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)*} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! perhaps_debrace {
    (
        {{$($tokens:tt)*}} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)*} => $($cont_args)* }
    };
    (
        {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)*} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! embrace {
    (
        {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { {$($tokens)*} } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! strip_brackets {
    (
        {[$($tokens:tt)*]} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)*} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! in_brackets {
    (
        {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { [$($tokens)*] } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! strip_parentheses {
    (
        {($($tokens:tt)*)} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$($tokens)*} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! in_parentheses {
    (
        {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { ($($tokens)*) } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! skip_if_empty {
    (
        {{}} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {} => $($cont_args)* }
    };
    (
        {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($tokens)* } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! skip_all_empty {
    (
        {$($tokens:tt)*} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($tokens)* } => [ $crate::skip_if_empty{} ] => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! fork {
    ( // Fill in for a missing element
        {} => { {$($head_pipe:tt)*} $($tail_pipes:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {} => $($head_pipe)+ => $crate::__accumulate_forked_pipes { {} => {$($tail_pipes)*} => {} } => $($cont_args)* }
    };
    (
        { $head_input:tt $($tail_inputs:tt)* } => { {$($head_pipe:tt)+} $($tail_pipes:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$head_input} => $($head_pipe)+ => $crate::__accumulate_forked_pipes { {$($tail_inputs)*} => {$($tail_pipes)*} => {} } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! cross {
    (
        {} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {} => $($cont_args)* }
    };
    (
        { {$($items:tt)*} $($rest:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($rest)* } => $crate::__accumulate_cross{ {} => {$({$items})*} } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! when {
    (
        { $($input:tt)* } => { { $($cond_pipe:tt)+ } => { $($then_pipe:tt)+ } else { $($else_pipe:tt)+ } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { $($input)* }
            => $($cond_pipe)+
            => $crate::__when_continuation{
                then: { { $($input)* } => $($then_pipe)+ },
                else: { { $($input)* } => $($else_pipe)+ }
            }
            => $($cont_args)*
        }
    };
    (
        { $($input:tt)* } => { { $($cond_pipe:tt)+ } => { $($then_pipe:tt)+ } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { $($input)* }
            => $($cond_pipe)+
            => $crate::__when_continuation{
                then: { { $($input)* } => $($then_pipe)+ },
                else: { {} }
            }
            => $($cont_args)*
        }
    };
}

#[macro_export]
macro_rules! join_with {
    (
        { $($items:tt)* } => { $($separator:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_join! { {$($items)*} => { sep: { $($separator)* }, acc: {} } => $cont { $($cont_args)* } }
    };
}

#[macro_export]
macro_rules! render {
    (
        { $($input:tt)* } => { $($pattern:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_render! { {} => { pattern: { $($pattern)* }, input: {$($input)*} } => $cont { $($cont_args)* } }
    };
}

#[macro_export]
macro_rules! head {
    (
        { $head:tt $($tail_ignored:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $head } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! tail {
    (
        { $head_ignored:tt $($tail:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($tail)* } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! append {
    (
        { $($input:tt)* } => { $($appended:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($input)* $($appended)* } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! not {
    (
        { true } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
    (
        { false } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! repeat {
    (
        { $($input:tt)* } => { $($repeats:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_repeat! { { $($input)* } => { rep: { $($repeats)* }, acc: {} } => $cont { $($cont_args)* } }
    };
}

#[macro_export]
macro_rules! reset {
    (
        { $($input_ignored:tt)* } => { $($new_input:tt)* } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($new_input)* } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! pass {
    (
        { $($input:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($input)* } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! swap {
    (
        { $first:tt $($second:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($second)* $first } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! is_empty {
    (
        {} => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
    (
        { $($_ignored:tt)+ } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! transpose {
    (
        { $($input:tt)* } => {} => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_transpose! {
            { $($input)* }
            => {
                from: {},
                to: {}
            }
            => $cont { $($cont_args)* }
        }
    };
}

#[macro_export]
macro_rules! any {
    (
        { $($input:tt)+ } => { $($cond_pipe:tt)+ } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { $($input)+ }
            => $crate::when{
                { $crate::head{} => $($cond_pipe)+ } => { reset{true} } else {
                    $crate::tail{}
                    => $crate::any{ $($cond_pipe)+ }
                }
            }
            => $($cont_args)*
        }
    };
    (
        {} => { $($cond_pipe:tt)+ } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {false} => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! all {
    (
        { $($input:tt)+ } => { $($cond_pipe:tt)+ } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { $($input)+ }
            => $crate::when{
                { $crate::head{} => $($cond_pipe)+ } => {
                    $crate::tail{}
                    => $crate::all{ $($cond_pipe)+ }
                } else {false}
            }
            => $($cont_args)*
        }
    };
    (
        {} => { $($cond_pipe:tt)+ } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {true} => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_transpose {
    (
        { { $row_head:tt $($row_tail:tt)* } $($input_tail:tt)* }
        => {
            from: { { $($from_cols_head:tt)* } $($from_cols_tail:tt)* },
            to: { $($to_cols:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_transpose! {
            { { $($row_tail)* } $($input_tail)* }
            => {
                from: { $($from_cols_tail)* },
                to: { $($to_cols)* { $($from_cols_head)* $row_head } }
            }
            => $cont { $($cont_args)* }
        }
    };
    (
        { { $row_head:tt $($row_tail:tt)* } $($input_tail:tt)* }
        => {
            from: {},
            to: { $($to_cols:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_transpose! {
            { { $($row_tail)* } $($input_tail)* }
            => {
                from: {},
                to: { $($to_cols)* { $row_head } }
            }
            => $cont { $($cont_args)* }
        }
    };
    (
        { {} $($input_tail:tt)* }
        => {
            from: { $($from_cols:tt)* },
            to: { $($to_cols:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_transpose! {
            { $($input_tail)* }
            => {
                from: { $($to_cols)* $($from_cols)* },
                to: {}
            }
            => $cont { $($cont_args)* }
        }
    };
    (
        {}
        => {
            from: { $($from_cols:tt)* },
            to: { $($to_cols:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { $($to_cols)* $($from_cols)* } => $($cont_args)*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_repeat {
    (
        { $($input:tt)* } => { rep: { { $($head_ignored:tt)* } $($tail:tt)* }, acc: { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_repeat! { { $($input)* } => { rep: { $($tail)* }, acc: { $($acc)* { $($input)* } } } => $cont { $($cont_args)* } }
    };
    (
        { $($input:tt)* } => { rep: { $head_ignored:tt $($tail:tt)* }, acc: { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_repeat! { { $($input)* } => { rep: { $($tail)* }, acc: { $($acc)* $($input)* } } => $cont { $($cont_args)* } }
    };
    (
        { $($input:tt)* } => { rep: {}, acc: { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($acc)* } => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_render {
    (
        { $($acc:tt)* } => { pattern: { @input $($tail:tt)* }, input: { $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_render! { { $($acc)* $($input)* } => { pattern: { $($tail)* }, input: { $($input)* } } => $cont { $($cont_args)* } }
    };
    (
        { $($acc:tt)* } => { pattern: { @input0 $($tail:tt)* }, input: { {$($input0:tt)*} $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_render! { { $($acc)* $($input0)* } => { pattern: { $($tail)* }, input: { {$($input0)*} $($input)* } } => $cont { $($cont_args)* } }
    };
    (
        { $($acc:tt)* } => { pattern: { @input1 $($tail:tt)* }, input: { {$($input0:tt)*} {$($input1:tt)*} $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_render! { { $($acc)* $($input1)* } => { pattern: { $($tail)* }, input: { {$($input0)*} {$($input1)*} $($input)* } } => $cont { $($cont_args)* } }
    };
    (
        { $($acc:tt)* } => { pattern: { @input2 $($tail:tt)* }, input: { {$($input0:tt)*} {$($input1:tt)*} {$($input2:tt)*} $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_render! { { $($acc)* $($input2)* } => { pattern: { $($tail)* }, input: { {$($input0)*} {$($input1)*} {$($input2)*} $($input)* } } => $cont { $($cont_args)* } }
    };
    (
        { $($acc:tt)* } => { pattern: { { $($head:tt)* } $($tail:tt)* }, input: { $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { {$($acc)*} }
            => $crate::fork{
                {$crate::debrace{}}
                {
                    $crate::__accumulate_render{pattern: { $($head)* }, input: { $($input)* }}
                    => $crate::embrace{}
                }
            }
            => $crate::__accumulate_render{pattern: { $($tail)* }, input: { $($input)* }}
            => $($cont_args)*
        }
    };
    (
        { $($acc:tt)* } => { pattern: { [ $($head:tt)* ] $($tail:tt)* }, input: { $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { {$($acc)*} }
            => $crate::fork{
                {$crate::debrace{}}
                {
                    $crate::__accumulate_render{pattern: { $($head)* }, input: { $($input)* }}
                    => $crate::in_brackets{}
                }
            }
            => $crate::__accumulate_render{pattern: { $($tail)* }, input: { $($input)* }}
            => $($cont_args)*
        }
    };
    (
        { $($acc:tt)* } => { pattern: { ( $($head:tt)* ) $($tail:tt)* }, input: { $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { {$($acc)*} }
            => $crate::fork{
                {$crate::debrace{}}
                {
                    $crate::__accumulate_render{pattern: { $($head)* }, input: { $($input)* }}
                    => $crate::in_parentheses{}
                }
            }
            => $crate::__accumulate_render{pattern: { $($tail)* }, input: { $($input)* }}
            => $($cont_args)*
        }
    };
    (
        { $($acc:tt)* } => { pattern: { @{ $($head:tt)* } $($tail:tt)* }, input: { $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_render! { { $($acc)* $($head)* } => { pattern: { $($tail)* }, input: { $($input)* } } => $cont { $($cont_args)* } }
    };
    (
        { $($acc:tt)* } => { pattern: { $head:tt $($tail:tt)* }, input: { $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_render! { { $($acc)* $head } => { pattern: { $($tail)* }, input: { $($input)* } } => $cont { $($cont_args)* } }
    };
    (
        { $($acc:tt)* } => { pattern: { }, input: { $($input:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($acc)* } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! __accumulate_join {
    (
        { {$($head:tt)*} $($tail:tt)+ } => { sep: { $($sep:tt)* }, acc: { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_join! { { $($tail)+ } => { sep: { $($sep)* }, acc: { $($acc)* $($head)* $($sep)* } } => $cont { $($cont_args)* } }
    };
    (
        { {$($head:tt)*} } => { sep: { $($sep:tt)* }, acc: { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $crate::__accumulate_join! { {} => { sep: { $($sep)* }, acc: { $($acc)* $($head)* } } => $cont { $($cont_args)* } }
    };
    (
        {} => { sep: { $($sep:tt)* }, acc: { $($acc:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($acc)* } => $($cont_args)* }
    };
}

#[macro_export]
macro_rules! __accumulate_cross {
    (
        {} => { {} => { $($full_items:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($full_items)* } => $($cont_args)* }
    };
    (
        { {$head:tt $($tail:tt)*} $($rest:tt)* } => { { $($acc_items:tt)* } => { $({ $($full_items:tt)* })* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { {$($tail)*} $($rest)* } => $crate::__accumulate_cross{ { $($acc_items)* $({ $($full_items)* $head })* } => { $({ $($full_items)* })* } } => $($cont_args)* }
    };
    (
        { {} $($rest:tt)* } => { { $($acc_items:tt)* } => { $({ $($full_items:tt)* })* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($rest)* } => $crate::__accumulate_cross{ {} => { $($acc_items)* } } => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_forked_pipes {
    ( // Fill in empty for a missing element
        { $($output:tt)* }
        => {
            {}
            => { {$($head_pipe:tt)*} $($tail_pipes:tt)* }
            => { $($acc:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {} => $($head_pipe)* => $crate::__accumulate_forked_pipes { {} => {$($tail_pipes)*} => { $($acc)* $($output)* } } => $($cont_args)* }
    };
    (
        { $($output:tt)* }
        => {
            { $head_input:tt $($tail_inputs:tt)* }
            => { {$($head_pipe:tt)+} $($tail_pipes:tt)* }
            => { $($acc:tt)* }
        }
        => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { {$head_input} => $($head_pipe)* => $crate::__accumulate_forked_pipes { {$($tail_inputs)*} => {$($tail_pipes)*} => { $($acc)* $($output)* } } => $($cont_args)* }
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
        $cont! { { $($acc)* $($output)* } => $($cont_args)* }
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
            { $head $($tail)* }
            => $crate::when{
                { $crate::head{} => $should_split{ $($should_split_args)* } }
                => {
                    $crate::tail{}
                    => $crate::__accumulate_split { {} => $should_split { $($should_split_args)* } => {$($acc)*{$($group)*}} }
                }
                else {
                    $crate::tail{}
                    => $crate::__accumulate_split { {$($group)*$head} => $should_split { $($should_split_args)* } => {$($acc)*} }
                }
            }
            => $($cont_args)*
        }
    };
    (
        {} => { {$($group:tt)*} => $should_split:path { $($should_split_args:tt)* } => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { $($acc)*{$($group)*} } => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_split_prefix_until {
    (
        { $head:tt $($tail:tt)* } => { $should_split:path { $($should_split_args:tt)* } => {$($prefix:tt)*} } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! {
            { $head $($tail)* }
            => $crate::when{
                { $crate::head{} => $should_split{ $($should_split_args)* } }
                => {
                    $crate::reset{ {$($prefix)*} {$head$($tail)*} }
                }
                else {
                    $crate::tail{}
                    => $crate::__accumulate_split_prefix_until { $should_split { $($should_split_args)* } => {$($prefix)*$head} }
                }
            }
            => $($cont_args)*
        }
    };
    (
        {} => { $should_split:path { $($should_split_args:tt)* } => {$($prefix:tt)*} } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { { {$($prefix)*} {} } => $($cont_args)* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __when_continuation {
    (
        { true } => { then: { $($then_pipe:tt)* }, else: { $($else_pipe:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { $($then_pipe)+ => $($cont_args)* }
    };
    (
        { false } => { then: { $($then_pipe:tt)* }, else: { $($else_pipe:tt)* } } => $cont:path { $($cont_args:tt)* }
    ) => {
        $cont! { $($else_pipe)+ => $($cont_args)* }
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
        //trace_macros!(true);
        let a = { apply_pipe!{{1;3;4} => split_semicolons{} => join_with_colons{} => in_brackets{}} };
        assert_eq!(a, [1, 3, 4]);
        //trace_macros!(false);
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
                    debrace{}
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

    macro_rules! split_by_comma {
        (
            { , $($tail:tt)* } => { {$($group:tt)*} => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $($tail)* } => split_by_comma { {} => {$($acc)*{$($group)*}} } => $($cont_args)* }
        };
        (
            { $head:tt $($tail:tt)* } => { {$($group:tt)*} => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $($tail)* } => split_by_comma { {$($group)*$head} => {$($acc)*} } => $($cont_args)* }
        };
        (
            {} => { {$($group:tt)*} => {$($acc:tt)*} } => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { $($acc)*{$($group)*} } => $($cont_args)* }
        };
    }

    #[test]
    fn split_by_comma_in_pipe() {
        //trace_macros!(true);
        let a = { apply_pipe! { {1+3, 2+3+4, 2} => split_by_comma { {} => {} } => as_array {} } };
        //trace_macros!(false);
        assert_eq!(a, [1+3, 2+3+4, 2]);
    }

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

    #[test]
    fn split_by_is_comma_in_pipe() {
        //trace_macros!(true);
        let a = { apply_pipe! { {1+3, 2+3+4, 2} => split { is_comma {} } => as_array {} } };
        //trace_macros!(false);
        assert_eq!(a, [1+3, 2+3+4, 2]);
    }

    #[test]
    fn split_by_is_comma_for_empty_input_in_pipe() {
        //trace_macros!(true);
        let a: [i32; _] = { apply_pipe! { {} => split { is_comma {} } => [ skip_if_empty{} ] => as_array {} } };
        //trace_macros!(false);
        assert_eq!(a, []);
    }

    #[test]
    fn split_by_is_comma_for_input_without_comma_in_pipe() {
        //trace_macros!(true);
        let a: [i32; _] = { apply_pipe! { { 1+2+3 } => split { is_comma {} } => [ skip_if_empty{} ] => as_array {} } };
        //trace_macros!(false);
        assert_eq!(a, [1+2+3]);
    }

    macro_rules! is_semicolon {
        (
            {;} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {true} => $($cont_args)* }
        };
        (
            { $($token:tt)* } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {false} => $($cont_args)* }
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
                    debrace {}
                    => split { is_comma {} }
                    => [ to_string{} ]
                    => join {}
                ]
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ["33", "4188", "9"]);
    }

    #[test]
    fn fork_filling_in_for_last_missing_element_ignoring_it_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {{{1}{11}} {{2}} {{3}{13}}}
                => [
                    debrace{}
                    => fork { {pass{}} {reset{}} }
                ]
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [1, 2, 3]);
    }

    #[test]
    fn fork_filling_in_for_last_missing_element_skipping_it_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {{{1}{11}} {{2}} {{3}{13}}}
                => [
                    debrace{}
                    => fork { {reset{}} {pass{}} }
                    => skip_if_empty{}
                ]
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [11, 13]);
    }
    
    #[test]
    fn split_by_semicolon_then_colon_then_fork_skipping_first_input_and_passing_second_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {1+2, 3; 4, 5+6+7; 8, 9}
                => split { is_semicolon {} }
                => [
                    debrace {}
                    => split { is_comma {} }
                    => fork { {reset{}} {pass{}} }
                ]
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [3, 5+6+7, 9]);
    }

    #[test]
    fn split_by_semicolon_then_colon_then_fork_passing_first_input_and_skipping_second_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {1+2, 3; 4, 5+6+7; 8, 9}
                => split { is_semicolon {} }
                => [
                    debrace {}
                    => split { is_comma {} }
                    => fork { {pass{}} {reset{}} }
                ]
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [1+2, 4, 8]);
    }

    #[test]
    fn split_by_semicolon_single_item_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {1+2}
                => split { is_semicolon {} }
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [3]);
    }

    #[test]
    fn split_by_semicolon_no_items_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe! {
                {}
                => split { is_semicolon {} }
                => as_array {}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [()]);
    }

    #[test]
    fn crosses_two_lists_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe!{
                { { {"A"} {"B"} } { {"1"} {"2"} {"3"} } }
                => cross{}
                => [ debrace{} => join{} ]
                => as_array{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ["A1", "B1", "A2", "B2", "A3", "B3"]);
    }

    #[test]
    fn crosses_single_item_and_list_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe!{
                { { {"A"} } { {"1"} {"2"} {"3"} } }
                => cross{}
                => [ debrace{} => join{} ]
                => as_array{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ["A1", "A2", "A3"]);
    }

    #[test]
    fn crosses_two_single_items_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe!{
                { { {"A"} } { {"1"} } }
                => cross{}
                => [ debrace{} => join{} ]
                => as_array{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ["A1"]);
    }

    #[test]
    fn crosses_four_lists_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe!{
                { { {"A"} {"B"} } { {"1"} {"2"} {"3"} {"4"}} { {"_"} } { {"1"} {"2"} {"3"} } }
                => cross{}
                => [ debrace{} => join{} ]
                => as_array{}
            }
        };
        //trace_macros!(false);
        let expected: Vec<String> =
            ('1'..='3').flat_map(|d| {
                ('1'..='4').flat_map(move |b| {
                    ['A', 'B'].map(move |a| {
                        [a, b, '_', d].iter().collect()
                    })
                })
            })
            .collect();
        assert_eq!(a, expected.as_slice());
    }

    #[test]
    fn crosses_multiple_lists_with_one_empty_in_pipe() {
        //trace_macros!(true);
        let a: [String; _] = {
            apply_pipe!{
                { { {"A"} } {} { {"1"} {"2"} {"3"} } }
                => cross{}
                => [ debrace{} => join{} ]
                => as_array{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [] as [String; _]);
    }

    #[test]
    fn joins_multiple_elements_with_colon() {
        let a = {
            apply_pipe!{
                {{1+1}{2+2}{3}}
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, [2, 4, 3]);
    }

    #[test]
    fn joins_single_element_with_colon() {
        let a = {
            apply_pipe!{
                {{1+1}}
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, [2]);
    }

    #[test]
    fn joins_no_elements_with_colon() {
        let a: [i32; _] = {
            apply_pipe!{
                {}
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, []);
    }

    macro_rules! is_zero_string {
        (
            {"zero"} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont!{ {true} => $($cont_args)* }  
        };
        (
            {$($input:tt)*} => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont!{ {false} => $($cont_args)* }  
        };
    }

    #[test]
    fn when_string_then_replace_in_pipe() {
        //trace_macros!(true);
        let a = {
            apply_pipe!{
                {{1}{2}{"zero"}{3}{"zero"}}
                => [
                    debrace{}
                    => when{
                        { is_zero_string{} } => { reset{0} } else { pass{} }
                    }
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, [1, 2, 0, 3, 0]);
    }

    #[test]
    fn repeats_3_times_without_braces() {
        let a = {
            apply_pipe!{
                {{1}{2}}
                => repeat{ 1 2 3 }
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, [1, 2, 1, 2, 1, 2]);
    }

    #[test]
    fn repeats_3_times_with_braces() {
        let a = {
            apply_pipe!{
                {{1}{2}}
                => repeat{ {} {} {} }
                => [
                    debrace{}
                    => join_with{+}
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, [3, 3, 3]);
    }

    macro_rules! in_parentheses {
        (
            { $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { { ($($tokens)*) } => $($cont_args)* }
        };
    }

    macro_rules! is_marked {
        (
            { marked $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {true} => $($cont_args)* }
        };
        (
            { $($tokens:tt)* } => {} => $cont:path { $($cont_args:tt)* }
        ) => {
            $cont! { {false} => $($cont_args)* }
        };
    }

    #[test]
    fn separates_strings_by_prefix() {
        //trace_macros!(true);
        let a = {
            apply_pipe!{
                {{1}{marked 2}{3}{4}{marked 5}}
                => repeat{{}{}}
                => fork{
                    {
                        debrace{}
                        => [
                            debrace{}
                            => when{ { is_marked{} } => {tail{} => embrace{}} }
                        ]
                        => join_with{,}
                        => in_brackets{}
                        => embrace{}
                    }
                    {
                        debrace{}
                        => [
                            debrace{}
                            => when{ { is_marked{} => not{} } => {embrace{}} }
                        ]
                        => join_with{,}
                        => in_brackets{}
                        => embrace{}
                    }
                }
                => join_with{,}
                => in_parentheses{}
            }
        };
        //trace_macros!(false);
        assert_eq!(a, ([2, 5], [1, 3, 4]));
    }

    #[test]
    fn immerses_each_item_in_negation() {
        // trace_macros!(true);
        let a = {
            apply_pipe!{
                {{1}{2}{3}}
                => [
                    debrace{}
                    => render{ - @input }
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        // trace_macros!(false);
        assert_eq!(a, [-1, -2, -3]);
    }

    #[test]
    fn immerses_twice_in_each_item() {
        // trace_macros!(true);
        let a = {
            apply_pipe!{
                {{1}{2}{3}}
                => [
                    debrace{}
                    => render{ @input @{-3*} @input }
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        // trace_macros!(false);
        assert_eq!(a, [-2, -4, -6]);
    }

    #[test]
    fn immerses_with_braces() {
        // trace_macros!(true);
        let a = {
            apply_pipe!{
                {{1}{2}{3}{4}{5}}
                => [
                    debrace{}
                    => render{ if @input @{ % 2 == 0} { @{2 *} @input } else { - @input } }
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        // trace_macros!(false);
        assert_eq!(a, [-1, 4, -3, 8, -5]);
    }

    #[test]
    fn immerses_with_braces_and_parentheses_on_multiple_levels() {
        // trace_macros!(true);
        let a = {
            apply_pipe!{
                {{1}{2}{3}{4}{5}}
                => [
                    debrace{}
                    => render{
                        if ( @input @{ + 2 } ) @{ % 3 == 0} {
                            @{2 * } ( @input @{ - 1 } )
                        } else {
                            2 * ( - @input )
                        }
                    }
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        // trace_macros!(false);
        assert_eq!(a, [0, -4, -6, 6, -10]);
    }

    #[test]
    fn immerses_with_two_column_input() {
        // trace_macros!(true);
        let a = {
            apply_pipe!{
                {{{1}{13}}{{2}{15}}{{3}{17}}{{4}{19}}{{5}{21}}}
                => [
                    debrace{}
                    => render{
                        if ( @input1 @{ + 1 } ) % @input0 @{== 0} {
                            ( @input1 @{ + 1 } ) / @input0
                        } else {
                            @input1 - @input0
                        }
                    }
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        // trace_macros!(false);
        assert_eq!(a, [14, 8, 6, 5, 16]);
    }

    #[test]
    fn split_prefix_until_colon() {
        let a = {
            apply_pipe!{
                {1 + 2 + 3, 4 + 5, 6, 7 + 8}
                => split_prefix_until{ is_comma{} }
                => fork {
                    { pass{} }
                    {
                        debrace{}
                        => tail{}
                        => split{ is_comma{} }
                        => join_with{,}
                        => in_brackets{}
                        => embrace{}
                    }
                }
                => join_with{,}
                => in_parentheses{}
            }
        };
        assert_eq!(a, (1 + 2 + 3, [4 + 5, 6, 7 + 8]));
    }

    #[test]
    fn transposes_3_by_4_matrix() {
        let a = {
            apply_pipe!{
                {[[11, 12, 13], [21, 22, 23], [31, 32, 33], [41, 42, 43]]}
                => strip_brackets{}
                => split{ is_comma{} }
                => [
                    debrace{}
                    => strip_brackets{}
                    => split{ is_comma{} }
                    => embrace{}
                ]
                => transpose{}
                => [
                    debrace{}
                    => join_with{,}
                    => in_brackets{}
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, [[11, 21, 31, 41], [12, 22, 32, 42], [13, 23, 33, 43]]);
    }

    #[test]
    fn transposes_2_by_5_matrix() {
        let a = {
            apply_pipe!{
                {[[11, 12], [21, 22], [31, 32], [41, 42], [51, 52]]}
                => strip_brackets{}
                => split{ is_comma{} }
                => [
                    debrace{}
                    => strip_brackets{}
                    => split{ is_comma{} }
                    => embrace{}
                ]
                => transpose{}
                => [
                    debrace{}
                    => join_with{,}
                    => in_brackets{}
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, [[11, 21, 31, 41, 51], [12, 22, 32, 42, 52]]);
    }

    #[test]
    fn transposes_4_rows_fringed() {
        let a = {
            apply_pipe!{
                {([11, 12], [21, 22, 23], [31], [41, 42])}
                => strip_parentheses{}
                => split{ is_comma{} }
                => [
                    debrace{}
                    => strip_brackets{}
                    => split{ is_comma{} }
                    => embrace{}
                ]
                => transpose{}
                => [
                    debrace{}
                    => join_with{,}
                    => in_brackets{}
                    => embrace{}
                ]
                => join_with{,}
                => in_parentheses{}
            }
        };
        assert_eq!(a, ([11, 21, 31, 41], [12, 22, 42], [23]));
    }

    #[test]
    fn transposes_4_rows_fringed_with_2_empty() {
        let a = {
            apply_pipe!{
                {((), (21, 22, 23), (), (41, 42))}
                => strip_parentheses{}
                => split{ is_comma{} }
                => [
                    debrace{}
                    => strip_parentheses{}
                    => split{ is_comma{} }
                    => embrace{}
                ]
                => transpose{}
                => [
                    debrace{}
                    => skip_all_empty{}
                    => join_with{,}
                    => in_parentheses{}
                    => embrace{}
                ]
                => join_with{,}
                => in_parentheses{}
            }
        };
        assert_eq!(a, ((21, 41), (22, 42), (23)));
    }

    #[test]
    fn transposes_1_item() {
        let a = {
            apply_pipe!{
                {{{2}}}
                => transpose{}
                => [
                    debrace{}
                    => skip_all_empty{}
                    => join_with{,}
                    => in_brackets{}
                    => embrace{}
                ]
                => join_with{,}
                => in_brackets{}
            }
        };
        assert_eq!(a, [[2]]);
    }

    #[test]
    fn transposes_empty() {
        let a = {
            apply_pipe!{
                {}
                => transpose{}
                => [
                    debrace{}
                    => skip_all_empty{}
                    => join_with{,}
                    => in_parentheses{}
                    => embrace{}
                ]
                => join_with{,}
                => in_parentheses{}
            }
        };
        assert_eq!(a, ());
    }

    #[test]
    fn any_is_comma() {
        let a = {
            apply_pipe!{
                { 3 + 2 - 2, 4 }
                => any{ is_comma{} }
            }
        };
        assert!(a);
    }

    #[test]
    fn none_is_comma() {
        let a = {
            apply_pipe!{
                { 3 + 2 - 2 * 4 }
                => any{ is_comma{} }
            }
        };
        assert!(!a);
    }

    #[test]
    fn all_is_not_comma() {
        let a = {
            apply_pipe!{
                { 3 + 2 - 2 * 4 }
                => all{ is_comma{} => not{} }
            }
        };
        assert!(a);
    }

    #[test]
    fn all_is_comma() {
        let a = {
            apply_pipe!{
                { , , }
                => all{ is_comma{} }
            }
        };
        assert!(a);
    }
}
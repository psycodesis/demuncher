// Copyright (c) 2026 Mariusz Zacirka
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! all_splitted_by {
    ($splitter:path , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $crate::splitted_with_strategy_by_marker! { $splitter, $crate::__apply_all_splitted, $apply { $($args)* }, $($list)* }
    };
    ($marker:tt , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $crate::splitted_with_strategy_by! { $marker, $crate::__apply_all_splitted, $apply{ $($args)* }, $($list)* }
    };
}

#[macro_export]
macro_rules! each_splitted_by {
    ($marker:tt , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $crate::splitted_with_strategy_by! { $marker, $crate::__apply_each_splitted, $apply{ $($args)* }, $($list)* }
    };
}

#[macro_export]
macro_rules! splitted_with_strategy_by {
    ({,} , $strategy:path , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $crate::splitted_with_strategy_by_marker! { $crate::__put_marker_for_comma, $strategy, $apply { $($args)* }, $($list)* }
    };
    ({;} , $strategy:path , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $crate::splitted_with_strategy_by_marker! { $crate::__put_marker_for_semicolon, $strategy, $apply { $($args)* }, $($list)* }
    };
    ({:} , $strategy:path , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $crate::splitted_with_strategy_by_marker! { $crate::__put_marker_for_colon, $strategy, $apply { $($args)* }, $($list)* }
    };
    ({=>} , $strategy:path , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $crate::splitted_with_strategy_by_marker! { $crate::__put_marker_for_double_arrow, $strategy, $apply { $($args)* }, $($list)* }
    };
}

#[macro_export]
macro_rules! splitted_with_strategy_by_marker {
    ($splitter:path , $strategy:path , $apply:path { $($args:tt)* } , $($list:tt)*) => {
        $splitter! {
            cont: $crate::__split_into_tts_by_muncher {
                part: [],
                parts: [],
                strategy: $strategy,
                apply: $apply{ $($args)* },
                splitter: $splitter,
            },
            input: [ $($list)* ],
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __put_marker_for_comma {
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ , $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [@splitter $($list)*] ,
        }
    };
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [$($list)*] ,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __put_marker_for_semicolon {
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ ; $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [@splitter $($list)*] ,
        }
    };
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [$($list)*] ,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __put_marker_for_colon {
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ : $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [@splitter $($list)*] ,
        }
    };
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [$($list)*] ,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __put_marker_for_double_arrow {
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ => $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [@splitter $($list)*] ,
        }
    };
    (
        cont: $cont:path { $($cont_args:tt)* } ,
        input: [ $($list:tt)* ] ,
    ) => {
        $cont! {
            $($cont_args)*
            input: [$($list)*] ,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __split_into_tts_by_muncher {
    (
        part: [ $($part:tt)* ] ,
        parts: [ $($parts:tt)* ] ,
        strategy: $strategy:path ,
        apply: $apply:path { $($apply_args:tt)* } ,
        splitter: $splitter:path ,
        input: [ @splitter $($list:tt)* ] ,
    ) => {
        $strategy!{part, $apply { $($apply_args)* }, $($part)+}
        $splitter! {
            cont: $crate::__split_into_tts_by_muncher {
                part: [],
                parts: [$($parts)*($($part)*)],
                strategy: $strategy,
                apply: $apply{ $($apply_args)* },
                splitter: $splitter ,
            },
            input: [$($list)*] ,
        }
    };
    (
        part: [ $($part:tt)* ] ,
        parts: [ $($parts:tt)* ] ,
        strategy: $strategy:path ,
        apply: $apply:path { $($apply_args:tt)* }  ,
        splitter: $splitter:path ,
        input: [ $head:tt $($list:tt)* ] ,
    ) => {
        $splitter! {
            cont: $crate::__split_into_tts_by_muncher {
                part: [$($part)*$head],
                parts: [$($parts)*],
                strategy: $strategy,
                apply: $apply{ $($apply_args)* },
                splitter: $splitter ,
            },
            input: [ $($list)* ] ,
        }
    };
    (
        part: [ $($part:tt)+ ] ,
        parts: [ $($parts:tt)* ] ,
        strategy: $strategy:path ,
        apply: $apply:path { $($apply_args:tt)* }  ,
        splitter: $splitter:path ,
        input: [ ] ,
    ) => {
        $strategy!{part, $apply{ $($apply_args)* }, $($part)+}
        $strategy!{all, $apply{ $($apply_args)* }, $($parts)*($($part)+)}
    };
    (
        part: [ ] ,
        parts: [ $($parts:tt)* ] ,
        strategy: $strategy:path ,
        apply: $apply:path { $($apply_args:tt)* }  ,
        splitter: $splitter:path ,
        input: [ ] ,
    ) => {
        $strategy!{all, $apply{ $($apply_args)* }, $($parts)*}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __apply_all_splitted {
    ( all, $apply:path { $($args:tt)* } , $($parts:tt)* ) => {$apply!{$($args)* $($parts)*}};
    ( part, $apply:path { $($args:tt)* } , $($part:tt)* ) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __apply_each_splitted {
    ( all, $apply:path { $($args:tt)* } , $($parts:tt)* ) => {};
    ( part, $apply:path { $($args:tt)* } , $($part:tt)* ) => {$apply!{$($args)* $($part)*}};
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    macro_rules! add_tts {
        ($zero:expr, $(($expressions:expr))*) => {
            $zero $( + ($expressions))*
        };
    }

    #[test]
    fn splits_expressions_by_comma_together() {
        let a = { all_splitted_by!{{,}, add_tts {0,}, 2 - 1, 3 - 2} };
        assert_eq!(a, 2);
    }

    #[test]
    fn splits_expressions_by_semicolon_together() {
        let a = { all_splitted_by!{{;}, add_tts {0,}, 2 - 1; 3 - 2; 2} };
        assert_eq!(a, 4);
    }

    #[test]
    fn splits_expressions_by_colon_together() {
        let a = { all_splitted_by!{{:}, add_tts {0,}, 2 - 1: 3 - 2} };
        assert_eq!(a, 2);
    }

    #[test]
    fn splits_expressions_by_double_arrow_together() {
        let a = { all_splitted_by!{{=>}, add_tts {0,}, 2 - 1 => 3 - 2} };
        assert_eq!(a, 2);
    }

    macro_rules! put_marker_for_word_sep {
        (
            cont: $cont:path { $($cont_args:tt)* } ,
            input: [ sep $($list:tt)* ] ,
        ) => {
            $cont! {
                $($cont_args)*
                input: [@splitter $($list)*] ,
            }
        };
        (
            cont: $cont:path { $($cont_args:tt)* } ,
            input: [ $($list:tt)* ] ,
        ) => {
            $cont! {
                $($cont_args)*
                input: [$($list)*] ,
            }
        };
    }

    #[test]
    fn splits_expressions_by_custom_marker_together() {
        let a = { all_splitted_by!{put_marker_for_word_sep, add_tts {0,}, 2 - 1 sep 3 - 2 sep 4 - 3} };
        assert_eq!(a, 3);
    }

    macro_rules! push_tt {
        ($v:ident, $expression:expr) => {
            $v.push($expression);
        };
    }

    #[test]
    fn splits_expressions_by_comma_one_by_one() {
        let mut v = vec![];
        each_splitted_by!{{,}, push_tt { v, }, 2 - 1, 3 - 2}
        assert_eq!(v, vec![1, 1]);
    }

    #[test]
    fn splits_expressions_by_semicolon_one_by_one() {
        let mut v = vec![];
        each_splitted_by!{{;}, push_tt { v, }, 2 - 1; 3 - 2; 2}
        assert_eq!(v, vec![1, 1, 2]);
    }

    macro_rules! push_added_expressions {
        ($v:ident, $($tokens:tt)*) => {
            $v.push({ all_splitted_by!{{,}, add_tts {0,}, $($tokens)*} });
        };
    }

    #[test]
    fn splits_expressions_first_by_semicolon_one_by_one_then_comma_together() {
        let mut v = vec![];
        each_splitted_by!{{;}, push_added_expressions { v, }, 2 - 1, 3 - 2; 4 - 2, 5 - 3, 3; 6 - 3 }
        assert_eq!(v, vec![1 + 1, 2 + 2 + 3, 3]);
    }

    macro_rules! add_mapped_expression {
        ($map:ident, ($($key_tokens:tt)*)($($value_tokens:tt)*)) => {
            $map.insert({ all_splitted_by!{{,}, add_tts {String::default(),}, $($key_tokens)*} }, { all_splitted_by!{{,}, add_tts {0,}, $($value_tokens)*} });
        };
    }

    macro_rules! map_expressions {
        ($map:ident, $($tokens:tt)*) => {
            all_splitted_by!{{=>}, add_mapped_expression { $map, }, $($tokens)* }
        };
    }

    #[test]
    fn splits_expressions_first_by_semicolon_then_by_double_arrow_and_then_comma() {
        let mut map = HashMap::new();
        each_splitted_by!{{;}, map_expressions { map, }, "a", "1" => 2 - 1, 3 - 2; "b" => 4 - 2, 5 - 3, 3; "c", "_1", "_2" => 6 - 3 }
        let mut expected_map = HashMap::new();
        expected_map.insert(String::from("a1"), 1 + 1);
        expected_map.insert(String::from("b"), 2 + 2 + 3);
        expected_map.insert(String::from("c_1_2"), 3);
        assert_eq!(map, expected_map);
    }
}
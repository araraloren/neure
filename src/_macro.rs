#[macro_export]
macro_rules! neure {
    (@q * $($res:tt)*) => {
        $crate::zero_more($($res)*)
    };
    (@q ? $($res:tt)*) => {
        $crate::zero_one($($res)*)
    };
    (@q + $($res:tt)*) => {
        $crate::one_more($($res)*)
    };
    (@q {$st:literal} $($res:tt)*) => {
        $crate::count::<$st, $st, _>($($res)*)
    };
    (@q {$st:literal,} $($res:tt)*) => {
        $crate::count::<$st, {usize::MAX}, _>($($res)*)
    };
    (@q {$st:literal, $ed:literal} $($res:tt)*) => {
        $crate::count::<$st, $ed, _>($($res)*)
    };
    (@q $($res:tt)*) => {
        $crate::one($($res)*)
    };

    (@r ^ $($res:tt)*) => { // \S
        neure!(@q $($res)* regex!(^))
    };
    (@r . $($res:tt)*) => { // .
        neure!(@q $($res)* regex!(.))
    };
    (@r [ $($range:tt)+ ] $($res:tt)*) => {
        neure!(@q $($res)* regex!([$($range)+]))
    };
    (@r $ch:ident $($res:tt)*) => {
        neure!(@q $($res)* $crate::equal($crate::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        neure!(@q $($res)* $crate::equal($ch))
    };
    (@r ($regex:expr) $($res:tt)*) => {
        neure!(@q $($res)* $regex)
    };
    (@r $($res:tt)*) => {
        neure!(@q $($res)* $crate::space())
    };
    ($($res:tt)*) => {
        neure!(@r $($res)*)
    };
}

#[macro_export]
macro_rules! regex {
    (^) => { // \S
        $crate::not($crate::space())
    };
    (.) => { // .
        $crate::wild()
    };
    ([^$l:literal - $r:literal] ) => {
        $crate::not($crate::range($l..=$r))
    };
    ([^$l:ident - $r:ident] ) => {
        $crate::not($crate::range($crate::charize!($l)..=$crate::charize!($r)))
    };
    ([$l:literal - $r:literal] ) => {
        $crate::range($l..=$r)
    };
    ([$l:ident - $r:ident] ) => {
        $crate::range($crate::charize!($l)..=$crate::charize!($r))
    };

    ([^$($l:literal - $r:literal)+] ) => {// [ ^ 'a'-'z' 'A'-'Z' ]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range($l..=$r), re);
            )+
            $crate::not(re)
        }
    };
    ([^$($l:ident - $r:ident)+] ) => { // [ ^ a-z A-Z ]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range($crate::charize!($l)..=$crate::charize!($r)), re);
            )+
            $crate::not(re)
        }
    };
    ([$($l:literal - $r:literal)+] ) => { // [ 'a'-'Z' 'A'-'Z' ]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range($l..=$r), re);
            )+
            re
        }
    };
    ([$($l:ident - $r:ident)+] ) => { // [ a-Z A-Z ]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range($crate::charize!($l)..=$crate::charize!($r)), re);
            )+
            re
        }
    };


    ([ ^ $($ch:literal)+ ] ) => { // [ ^ 'a' 'b' 'c']
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::equal($ch), re);
            )+
            $crate::not(re)
        }
    };
    ([ ^ $($ch:ident)+ ] ) => { // [^ a b c]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::equal($crate::charize!($ch)), re);
            )+
            $crate::not(re)
        }
    };
    ([ $($ch:literal)+ ] ) => { // ['a' 'b' 'c']
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::equal($ch), re);
            )+
            re
        }
    };
    ([ $($ch:ident)+ ] ) => { // [a b c]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::equal($crate::charize!($ch)), re);
            )+
            re
        }
    };

    ($ch:ident ) => {
        $crate::equal($crate::charize!($ch))
    };
    ($ch:literal ) => {
        $crate::equal($ch)
    };
    () => {
        $crate::space()
    };
}

#[macro_export]
macro_rules! group {
    ($regex:ident) => {
        $regex
    };
    ($regex:expr) => {
        $regex
    };
    ($regex:ident, $($res:tt)+) => {
        $crate::or($regex, group!($($res)+))
    };
    ($regex:expr, $($res:tt)+) => {
        $crate::or($regex, group!($($res)+))
    };
}

#[macro_export]
macro_rules! seq {
    ($parser:ident) => {
        $parser
    };
    ($parser:expr) => {
        $parser
    };
    ($parser:ident, $($res:tt)+) => {
        $crate::seq($parser, seq!($($res)+))
    };
    ($parser:expr, $($res:tt)+) => {
        $crate::seq($parser, seq!($($res)+))
    };
}

#[macro_export]
macro_rules! map {
    ($ctx:ident { &$parser:expr => |$inner_ctx:ident, $offset:ident, $ret:ident| $code:block $(,  $($res:tt)*)? }) => {
        if let Ok(map_ret) = $ctx.map(&$parser, |$inner_ctx, $offset, $ret| $code) {
            Ok(map_ret)
        }
        else {
            map!($ctx { $($($res)*)? })
        }
    };
    ($ctx:ident { &$parser:expr => |$orig:ident| $code:block $(,  $($res:tt)*)? }) => {
        if let Ok(map_ret) = $ctx.map_orig(&$parser, |$orig| $code) {
            Ok(map_ret)
        }
        else {
            map!($ctx { $($($res)*)? })
        }
    };
    ($ctx:ident { &$parser:expr => $code:block $(,  $($res:tt)*)? }) => {
        if let Ok(map_ret) = $ctx.map_orig(&$parser, |_| $code) {
            Ok(map_ret)
        }
        else {
            map!($ctx { $($($res)*)? })
        }
    };

    ($ctx:ident { $code:block }) => {
        $code
    };
}

#[macro_export]
macro_rules! neure {
    (@q * $($res:tt)*) => {
        $crate::parser::zero_more($($res)*)
    };
    (@q ? $($res:tt)*) => {
        $crate::parser::zero_one($($res)*)
    };
    (@q + $($res:tt)*) => {
        $crate::parser::one_more($($res)*)
    };
    (@q {$st:literal} $($res:tt)*) => {
        $crate::parser::count::<$st, $st, _, _>($($res)*)
    };
    (@q {$st:literal,} $($res:tt)*) => {
        $crate::parser::count::<$st, {usize::MAX}, _, _>($($res)*)
    };
    (@q {$st:literal, $ed:literal} $($res:tt)*) => {
        $crate::parser::count::<$st, $ed, _, _>($($res)*)
    };
    (@q $($res:tt)*) => {
        $crate::parser::one($($res)*)
    };

    (@r ^ $($res:tt)*) => { // \S
        neure!(@q $($res)* $crate::regex!(^))
    };
    (@r . $($res:tt)*) => { // .
        neure!(@q $($res)* $crate::regex!(.))
    };
    (@r [ $($range:tt)+ ] $($res:tt)*) => {
        neure!(@q $($res)* $crate::regex!([$($range)+]))
    };
    (@r $ch:ident $($res:tt)*) => {
        neure!(@q $($res)* $crate::regex::equal($crate::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        neure!(@q $($res)* $crate::regex::equal($ch))
    };
    (@r ($regex:expr) $($res:tt)*) => {
        neure!(@q $($res)* $regex)
    };
    (@r $($res:tt)*) => {
        neure!(@q $($res)* $crate::regex::space())
    };
    ($($res:tt)*) => {
        neure!(@r $($res)*)
    };
}

#[macro_export]
macro_rules! regex {
    (^) => { // \S
        $crate::regex::not($crate::regex::space())
    };
    (.) => { // .
        $crate::regex::wild()
    };
    ([^$l:literal - $r:literal] ) => {
        $crate::regex::not($crate::regex::range($l..=$r))
    };
    ([^$l:ident - $r:ident] ) => {
        $crate::regex::not($crate::regex::range($crate::charize!($l)..=$crate::charize!($r)))
    };
    ([$l:literal - $r:literal] ) => {
        $crate::regex::range($l..=$r)
    };
    ([$l:ident - $r:ident] ) => {
        $crate::regex::range($crate::charize!($l)..=$crate::charize!($r))
    };

    ([^$($l:literal - $r:literal)+] ) => {// [ ^ 'a'-'z' 'A'-'Z' ]
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::range($l..=$r), re);
            )+
            $crate::regex::not(re)
        }
    };
    ([^$($l:ident - $r:ident)+] ) => { // [ ^ a-z A-Z ]
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::range($crate::charize!($l)..=$crate::charize!($r)), re);
            )+
            $crate::regex::not(re)
        }
    };
    ([$($l:literal - $r:literal)+] ) => { // [ 'a'-'Z' 'A'-'Z' ]
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::range($l..=$r), re);
            )+
            re
        }
    };
    ([$($l:ident - $r:ident)+] ) => { // [ a-Z A-Z ]
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::range($crate::charize!($l)..=$crate::charize!($r)), re);
            )+
            re
        }
    };


    ([ ^ $($ch:literal)+ ] ) => { // [ ^ 'a' 'b' 'c']
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::equal($ch), re);
            )+
            $crate::regex::not(re)
        }
    };
    ([ ^ $($ch:ident)+ ] ) => { // [^ a b c]
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::equal($crate::charize!($ch)), re);
            )+
            $crate::regex::not(re)
        }
    };
    ([ $($ch:literal)+ ] ) => { // ['a' 'b' 'c']
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::equal($ch), re);
            )+
            re
        }
    };
    ([ $($ch:ident)+ ] ) => { // [a b c]
        {
            let re = $crate::regex::always_f();
            $(
                let re = $crate::regex::or($crate::regex::equal($crate::charize!($ch)), re);
            )+
            re
        }
    };

    ($ch:ident ) => {
        $crate::regex::equal($crate::charize!($ch))
    };
    ($ch:literal ) => {
        $crate::regex::equal($ch)
    };
    () => {
        $crate::regex::space()
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
        $crate::regex::or($regex, group!($($res)+))
    };
    ($regex:expr, $($res:tt)+) => {
        $crate::regex::or($regex, group!($($res)+))
    };
}

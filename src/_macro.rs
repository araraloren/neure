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
        neure!(@q $($res)* $crate::char(charize::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        neure!(@q $($res)* $crate::char($ch))
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
        $crate::not($crate::range(charize::charize!($l)..=charize::charize!($r)))
    };
    ([$l:literal - $r:literal] ) => {
        $crate::range($l..=$r)
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
                let re = $crate::or($crate::range(charize::charize!($l)..=charize::charize!($r)), re);
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
                let re = $crate::or($crate::range(charize::charize!($l)..=charize::charize!($r)), re);
            )+
            re
        }
    };


    ([ ^ $($ch:literal)+ ] ) => { // [ ^ 'a' 'b' 'c']
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char($ch), re);
            )+
            $crate::not(re)
        }
    };
    ([ ^ $($ch:ident)+ ] ) => { // [^ a b c]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char(charize::charize!($ch)), re);
            )+
            $crate::not(re)
        }
    };
    ([ $($ch:literal)+ ] ) => { // ['a' 'b' 'c']
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char($ch), re);
            )+
            re
        }
    };
    ([ $($ch:ident)+ ] ) => { // [a b c]
        {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char(charize::charize!($ch)), re);
            )+
            re
        }
    };

    ($ch:ident ) => {
        $crate::char(charize::charize!($ch))
    };
    ($ch:literal ) => {
        $crate::char($ch)
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

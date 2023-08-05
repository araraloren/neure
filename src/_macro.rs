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
        neure!(@q $($res)* $crate::not($crate::space()))
    };
    (@r . $($res:tt)*) => { // .
        neure!(@q $($res)* $crate::wild())
    };
    (@r [^$l:literal - $r:literal] $($res:tt)*) => {
        neure!(@q $($res)* $crate::not($crate::range($l..=$r)))
    };
    (@r [^$l:ident - $r:ident] $($res:tt)*) => {
        neure!(@q $($res)* $crate::not($crate::range(charize::charize!($l)..=charize::charize!($r))))
    };
    (@r [$l:literal - $r:literal] $($res:tt)*) => {
        neure!(@q $($res)* $crate::range($l..=$r))
    };

    (@r [^$($l:literal - $r:literal)+] $($res:tt)*) => {// [ ^ 'a'-'z' 'A'-'Z' ]
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range($l..=$r), re);
            )+
            $crate::not(re)
        })
    };
    (@r [^$($l:ident - $r:ident)+] $($res:tt)*) => { // [ ^ a-z A-Z ]
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range(charize::charize!($l)..=charize::charize!($r)), re);
            )+
            $crate::not(re)
        })
    };
    (@r [$($l:literal - $r:literal)+] $($res:tt)*) => { // [ 'a'-'Z' 'A'-'Z' ]
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range($l..=$r), re);
            )+
            re
        })
    };
    (@r [$($l:ident - $r:ident)+] $($res:tt)*) => { // [ a-Z A-Z ]
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::range(charize::charize!($l)..=charize::charize!($r)), re);
            )+
            re
        })
    };


    (@r [ ^ $($ch:literal)+ ] $($res:tt)*) => { // [ ^ 'a' 'b' 'c']
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char($ch), re);
            )+
            $crate::not(re)
        })
    };
    (@r [ ^ $($ch:ident)+ ] $($res:tt)*) => { // [^ a b c]
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char(charize::charize!($ch)), re);
            )+
            $crate::not(re)
        })
    };
    (@r [ $($ch:literal)+ ] $($res:tt)*) => { // ['a' 'b' 'c']
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char($ch), re);
            )+
            re
        })
    };
    (@r [ $($ch:ident)+ ] $($res:tt)*) => { // [a b c]
        neure!(@q $($res)* {
            let re = $crate::always_f();
            $(
                let re = $crate::or($crate::char(charize::charize!($ch)), re);
            )+
            re
        })
    };

    (@r $ch:ident $($res:tt)*) => {
        neure!(@q $($res)* $crate::char(charize::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        neure!(@q $($res)* $crate::char($ch))
    };
    (@r $($res:tt)*) => {
        neure!(@q $($res)* $crate::space())
    };
    ($($res:tt)*) => {
        neure!(@r $($res)*)
    };
}

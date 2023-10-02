#[macro_export]
macro_rules! regex {
    (@q * $($res:tt)*) => {
        $crate::regex::zero_more($($res)*)
    };
    (@q ? $($res:tt)*) => {
        $crate::regex::zero_one($($res)*)
    };
    (@q + $($res:tt)*) => {
        $crate::regex::one_more($($res)*)
    };
    (@q {$st:literal} $($res:tt)*) => {
        $crate::regex::count::<$st, $st, _, _>($($res)*)
    };
    (@q {$st:literal,} $($res:tt)*) => {
        $crate::regex::count::<$st, {usize::MAX}, _, _>($($res)*)
    };
    (@q {$st:literal, $ed:literal} $($res:tt)*) => {
        $crate::regex::count::<$st, $ed, _, _>($($res)*)
    };
    (@q $($res:tt)*) => {
        $crate::regex::one($($res)*)
    };

    (@r ^ $($res:tt)*) => { // \S
        regex!(@q $($res)* $crate::neure!(^))
    };
    (@r . $($res:tt)*) => { // .
        regex!(@q $($res)* $crate::neure!(.))
    };
    (@r [ $($range:tt)+ ] $($res:tt)*) => {
        regex!(@q $($res)* $crate::neure!([$($range)+]))
    };
    (@r $ch:ident $($res:tt)*) => {
        regex!(@q $($res)* $crate::neure::equal($crate::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        regex!(@q $($res)* $crate::neure::equal($ch))
    };
    (@r ($($regex:expr),+) $($res:tt)*) => {
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($regex);
            )+
            regex!(@q $($res)* re)
        }
    };
    (@r $($res:tt)*) => {
        regex!(@q $($res)* $crate::neure::whitespace())
    };
    ($($res:tt)*) => {
        regex!(@r $($res)*)
    };
}

#[macro_export]
macro_rules! neure {
    (^) => { // \S
        $crate::neure::whitespace().not()
    };
    (.) => { // .
        $crate::neure::wild()
    };
    ([^$l:literal - $r:literal] ) => {
        $crate::neure::range($l..=$r).not()
    };
    ([^$l:ident - $r:ident] ) => {
        $crate::neure::range($crate::charize!($l)..=$crate::charize!($r)).not()
    };
    ([$l:literal - $r:literal] ) => {
        $crate::neure::range($l..=$r)
    };
    ([$l:ident - $r:ident] ) => {
        $crate::neure::range($crate::charize!($l)..=$crate::charize!($r))
    };

    ([^$($l:literal - $r:literal)+] ) => {// [ ^ 'a'-'z' 'A'-'Z' ]
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::range($l..=$r));
            )+
            re.not()
        }
    };
    ([^$($l:ident - $r:ident)+] ) => { // [ ^ a-z A-Z ]
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::range($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re.not()
        }
    };
    ([$($l:literal - $r:literal)+] ) => { // [ 'a'-'Z' 'A'-'Z' ]
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::range($l..=$r));
            )+
            re
        }
    };
    ([$($l:ident - $r:ident)+] ) => { // [ a-Z A-Z ]
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::range($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re
        }
    };


    ([ ^ $($ch:literal)+ ] ) => { // [ ^ 'a' 'b' 'c']
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::equal($ch));
            )+
            re.not()
        }
    };
    ([ ^ $($ch:ident)+ ] ) => { // [^ a b c]
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::equal($crate::charize!($ch)));
            )+
            re.not()
        }
    };
    ([ $($ch:literal)+ ] ) => { // ['a' 'b' 'c']
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::equal($ch));
            )+
            re
        }
    };
    ([ $($ch:ident)+ ] ) => { // [a b c]
        {
            let re = $crate::neure::none();
            $(
                let re = re.or($crate::neure::equal($crate::charize!($ch)));
            )+
            re
        }
    };

    ($ch:ident ) => {
        $crate::neure::equal($crate::charize!($ch))
    };
    ($ch:literal ) => {
        $crate::neure::equal($ch)
    };
    () => {
        $crate::neure::space()
    };
}

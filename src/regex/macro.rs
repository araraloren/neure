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
        neure!(@q $($res)* $crate::regex::Equal::new($crate::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        neure!(@q $($res)* $crate::regex::Equal::new($ch))
    };
    (@r ($regex:expr) $($res:tt)*) => {
        neure!(@q $($res)* $regex)
    };
    (@r $($res:tt)*) => {
        neure!(@q $($res)* $crate::regex::Space::new())
    };
    ($($res:tt)*) => {
        neure!(@r $($res)*)
    };
}

#[macro_export]
macro_rules! regex {
    (^) => { // \S
        $crate::regex::Space::new().not()
    };
    (.) => { // .
        $crate::regex::Wild::new()
    };
    ([^$l:literal - $r:literal] ) => {
        $crate::regex::CopyRange::from($l..=$r).not()
    };
    ([^$l:ident - $r:ident] ) => {
        $crate::regex::CopyRange::from($crate::charize!($l)..=$crate::charize!($r)).not()
    };
    ([$l:literal - $r:literal] ) => {
        $crate::regex::CopyRange::from($l..=$r)
    };
    ([$l:ident - $r:ident] ) => {
        $crate::regex::CopyRange::from($crate::charize!($l)..=$crate::charize!($r))
    };

    ([^$($l:literal - $r:literal)+] ) => {// [ ^ 'a'-'z' 'A'-'Z' ]
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::CopyRange::from($l..=$r));
            )+
            re.not()
        }
    };
    ([^$($l:ident - $r:ident)+] ) => { // [ ^ a-z A-Z ]
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::CopyRange::from($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re.not()
        }
    };
    ([$($l:literal - $r:literal)+] ) => { // [ 'a'-'Z' 'A'-'Z' ]
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::CopyRange::from($l..=$r));
            )+
            re
        }
    };
    ([$($l:ident - $r:ident)+] ) => { // [ a-Z A-Z ]
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::CopyRange::from($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re
        }
    };


    ([ ^ $($ch:literal)+ ] ) => { // [ ^ 'a' 'b' 'c']
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::Equal::new($ch));
            )+
            re.not()
        }
    };
    ([ ^ $($ch:ident)+ ] ) => { // [^ a b c]
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::Equal::new($crate::charize!($ch)));
            )+
            re.not()
        }
    };
    ([ $($ch:literal)+ ] ) => { // ['a' 'b' 'c']
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::Equal::new($ch));
            )+
            re
        }
    };
    ([ $($ch:ident)+ ] ) => { // [a b c]
        {
            let re = $crate::regex::False::new();
            $(
                let re = re.or($crate::regex::Equal::new($crate::charize!($ch)));
            )+
            re
        }
    };

    ($ch:ident ) => {
        $crate::regex::Equal::new($crate::charize!($ch))
    };
    ($ch:literal ) => {
        $crate::regex::Equal::new($ch)
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
        $regex.or(group!($($res)+))
    };
    ($regex:expr, $($res:tt)+) => {
        $regex.or(group!($($res)+))
    };
}

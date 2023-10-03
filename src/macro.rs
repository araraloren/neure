#[macro_export]
macro_rules! re {
    (@q * $($res:tt)*) => {
        $crate::re::zero_more($($res)*)
    };
    (@q ? $($res:tt)*) => {
        $crate::re::zero_one($($res)*)
    };
    (@q + $($res:tt)*) => {
        $crate::re::one_more($($res)*)
    };
    (@q {$st:literal} $($res:tt)*) => {
        $crate::re::count::<$st, $st, _, _>($($res)*)
    };
    (@q {$st:literal,} $($res:tt)*) => {
        $crate::re::count::<$st, {usize::MAX}, _, _>($($res)*)
    };
    (@q {$st:literal, $ed:literal} $($res:tt)*) => {
        $crate::re::count::<$st, $ed, _, _>($($res)*)
    };
    (@q $($res:tt)*) => {
        $crate::re::one($($res)*)
    };

    (@r ^ $($res:tt)*) => { // \S
        re!(@q $($res)* $crate::neu!(^))
    };
    (@r . $($res:tt)*) => { // .
        re!(@q $($res)* $crate::neu!(.))
    };
    (@r [ $($range:tt)+ ] $($res:tt)*) => {
        re!(@q $($res)* $crate::neu!([$($range)+]))
    };
    (@r $ch:ident $($res:tt)*) => {
        re!(@q $($res)* $crate::neu::equal($crate::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        re!(@q $($res)* $crate::neu::equal($ch))
    };
    (@r ($($regex:expr),+) $($res:tt)*) => {
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($regex);
            )+
            re!(@q $($res)* re)
        }
    };
    (@r $($res:tt)*) => {
        re!(@q $($res)* $crate::neu::whitespace())
    };
    ($($res:tt)*) => {
        re!(@r $($res)*)
    };
}

#[macro_export]
macro_rules! neu {
    (^) => { // \S
        $crate::neu::whitespace().not()
    };
    (.) => { // .
        $crate::neu::wild()
    };
    ([^$l:literal - $r:literal] ) => {
        $crate::neu::range($l..=$r).not()
    };
    ([^$l:ident - $r:ident] ) => {
        $crate::neu::range($crate::charize!($l)..=$crate::charize!($r)).not()
    };
    ([$l:literal - $r:literal] ) => {
        $crate::neu::range($l..=$r)
    };
    ([$l:ident - $r:ident] ) => {
        $crate::neu::range($crate::charize!($l)..=$crate::charize!($r))
    };

    ([^$($l:literal - $r:literal)+] ) => {// [ ^ 'a'-'z' 'A'-'Z' ]
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::range($l..=$r));
            )+
            re.not()
        }
    };
    ([^$($l:ident - $r:ident)+] ) => { // [ ^ a-z A-Z ]
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::range($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re.not()
        }
    };
    ([$($l:literal - $r:literal)+] ) => { // [ 'a'-'Z' 'A'-'Z' ]
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::range($l..=$r));
            )+
            re
        }
    };
    ([$($l:ident - $r:ident)+] ) => { // [ a-Z A-Z ]
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::range($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re
        }
    };


    ([ ^ $($ch:literal)+ ] ) => { // [ ^ 'a' 'b' 'c']
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::equal($ch));
            )+
            re.not()
        }
    };
    ([ ^ $($ch:ident)+ ] ) => { // [^ a b c]
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::equal($crate::charize!($ch)));
            )+
            re.not()
        }
    };
    ([ $($ch:literal)+ ] ) => { // ['a' 'b' 'c']
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::equal($ch));
            )+
            re
        }
    };
    ([ $($ch:ident)+ ] ) => { // [a b c]
        {
            let re = $crate::neu::none();
            $(
                let re = re.or($crate::neu::equal($crate::charize!($ch)));
            )+
            re
        }
    };

    ($ch:ident ) => {
        $crate::neu::equal($crate::charize!($ch))
    };
    ($ch:literal ) => {
        $crate::neu::equal($ch)
    };
    () => {
        $crate::neu::space()
    };
}

#[macro_export]
macro_rules! neure {
    () => {
        $crate::one($crate::space())
    };
    (+) => {
        $crate::one_more($crate::space())
    };
    (?) => {
        $crate::zero_one($crate::space())
    };
    (*) => {
        $crate::zero_more($crate::space())
    };
    ({$start:literal}) => {
        $crate::count::<$start, $start, _>($crate::space())
    };
    ({$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::space())
    };
    ({$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::space())
    };
    (.) => {
        $crate::one($crate::not($crate::space()))
    };
    ($ch:literal) => {
        $crate::count::<1, 1, _>($crate::char($ch))
    };
    ($ch:literal+) => {
        $crate::count::<1, { usize::MAX }, _>($crate::char($ch))
    };
    ($ch:literal?) => {
        $crate::count::<0, 1, _>($crate::char($ch))
    };
    ($ch:literal*) => {
        $crate::count::<0, { usize::MAX }, _>($crate::char($ch))
    };
    ($ch:literal{$start:literal}) => {
        $crate::count::<$start, $start, _>($crate::char($ch))
    };
    ($ch:literal{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::char($ch))
    };
    ($ch:literal{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::char($ch))
    };
    ([$l:literal - $r:literal]) => {
        $crate::count::<1, 1, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]+) => {
        $crate::count::<1, { usize::MAX }, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]?) => {
        $crate::count::<0, 1, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]*) => {
        $crate::count::<0, { usize::MAX }, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]{$start:literal}) => {
        $crate::count::<$start, $start, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::range($l..=$r))
    };

    (^) => {
        $crate::one($crate::not($crate::space()))
    };
    (^+) => {
        $crate::one_more($crate::not($crate::space()))
    };
    (^?) => {
        $crate::zero_one($crate::not($crate::space()))
    };
    (^*) => {
        $crate::zero_more($crate::not($crate::space()))
    };
    (^{$start:literal}) => {
        $crate::count::<$start, $start, _>($crate::not($crate::space()))
    };
    (^{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::not($crate::space()))
    };
    (^{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::not($crate::space()))
    };
    (^$ch:literal) => {
        $crate::count::<1, 1, _>($crate::not($crate::char($ch)))
    };
    (^$ch:literal+) => {
        $crate::count::<1, { usize::MAX }, _>($crate::not($crate::char($ch)))
    };
    (^$ch:literal?) => {
        $crate::count::<0, 1, _>($crate::not($crate::char($ch)))
    };
    (^$ch:literal*) => {
        $crate::count::<0, { usize::MAX }, _>($crate::not($crate::char($ch)))
    };
    (^$ch:literal{$start:literal}) => {
        $crate::count::<$start, $start, _>($crate::not($crate::char($ch)))
    };
    (^$ch:literal{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::not($crate::char($ch)))
    };
    (^$ch:literal{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::not($crate::char($ch)))
    };
    ([^$l:literal - $r:literal]) => {
        $crate::count::<1, 1, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]+) => {
        $crate::count::<1, { usize::MAX }, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]?) => {
        $crate::count::<0, 1, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]*) => {
        $crate::count::<0, { usize::MAX }, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]{$start:literal}) => {
        $crate::count::<$start, $start, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::not($crate::range($l..=$r)))
    };
}

macro_rules! next {
    (@q * $($res:tt)*) => {
        neure::zero_more($($res)*)
    };
    (@q ? $($res:tt)*) => {
        neure::zero_one($($res)*)
    };
    (@q + $($res:tt)*) => {
        neure::one_more($($res)*)
    };
    (@q {$st:literal} $($res:tt)*) => {
        neure::count::<$st, $st, _>($($res)*)
    };
    (@q {$st:literal,} $($res:tt)*) => {
        neure::count::<$st, {usize::MAX}, _>($($res)*)
    };
    (@q {$st:literal, $ed:literal} $($res:tt)*) => {
        neure::count::<$st, $ed, _>($($res)*)
    };
    (@q $($res:tt)*) => {
        neure::one($($res)*)
    };
    (@r $($res:tt)*) => {
        next!(@q $($res)* neure::space())
    };
    ($($res:tt)*) => {
        next!(@r $($res)*)
    };
}

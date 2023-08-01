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
        $crate::count::<$start, { $start + 1 }, _>($crate::space())
    };
    ({$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::space())
    };
    ({$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::space())
    };
    ('^') => {
        $crate::start()
    };
    ('$') => {
        $crate::end()
    };
    ($ch:literal) => {
        $crate::count::<1, 2, _>($crate::char($ch))
    };
    ($ch:literal+) => {
        $crate::count::<1, { usize::MAX }, _>($crate::char($ch))
    };
    ($ch:literal?) => {
        $crate::count::<0, 2, _>($crate::char($ch))
    };
    ($ch:literal*) => {
        $crate::count::<0, { usize::MAX }, _>($crate::char($ch))
    };
    ($ch:literal{$start:literal}) => {
        $crate::count::<$start, { $start + 1 }, _>($crate::char($ch))
    };
    ($ch:literal{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::char($ch))
    };
    ($ch:literal{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::char($ch))
    };
    ([$l:literal - $r:literal]) => {
        $crate::count::<1, 2, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]+) => {
        $crate::count::<1, { usize::MAX }, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]?) => {
        $crate::count::<0, 2, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]*) => {
        $crate::count::<0, { usize::MAX }, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]{$start:literal}) => {
        $crate::count::<1, { $start + 1 }, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::range($l..=$r))
    };
    ([$l:literal - $r:literal]{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::range($l..=$r))
    };
    ([^$l:literal - $r:literal]) => {
        $crate::count::<1, 2, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]+) => {
        $crate::count::<1, { usize::MAX }, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]?) => {
        $crate::count::<0, 2, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]*) => {
        $crate::count::<0, { usize::MAX }, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]{$start:literal}) => {
        $crate::count::<$start, { $start + 1 }, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]{$start:literal,}) => {
        $crate::count::<$start, { usize::MAX }, _>($crate::not($crate::range($l..=$r)))
    };
    ([^$l:literal - $r:literal]{$start:literal, $end:literal}) => {
        $crate::count::<$start, $end, _>($crate::not($crate::range($l..=$r)))
    };
}

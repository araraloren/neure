#[macro_export]
macro_rules! neure {
    () => {
        $crate::Count::<1, 2, $crate::Space>::default()
    };
    (+) => {
        $crate::Count::<1, usize::MAX, $crate::Space>::default()
    };
    (?) => {
        $crate::Count::<0, 2, $crate::Space>::default()
    };
    (*) => {
        $crate::Count::<0, usize::MAX, $crate::Space>::default()
    };
    ({$start:literal}) => {
        $crate::Count::<$start, $start + 1, $crate::Space>::default()
    };
    ({$start:literal,}) => {
        $crate::Count::<$start, usize::MAX, $crate::Space>::default()
    };
    ({$start:literal, $end:literal}) => {
        $crate::Count::<$start, $end, $crate::Space>::default()
    };
    ('^') => {
        $crate::Start::default()
    };
    ('$') => {
        $crate::End::default()
    };
    ($ch:literal) => {
        $crate::Count::<1, 2, _>::new($ch)
    };
    ($ch:literal+) => {
        $crate::Count::<1, usize::MAX, _>::new($ch)
    };
    ($ch:literal?) => {
        $crate::Count::<0, 2, _>::new($ch)
    };
    ($ch:literal*) => {
        $crate::Count::<0, usize::MAX, _>::new($ch)
    };
    ($ch:literal{$start:literal}) => {
        $crate::Count::<$start, $start + 1, _>::new($ch)
    };
    ($ch:literal{$start:literal,}) => {
        $crate::Count::<$start, usize::MAX, _>::new($ch)
    };
    ($ch:literal{$start:literal, $end:literal}) => {
        $crate::Count::<$start, $end, _>::new($ch)
    };
    ([$l:literal - $r:literal]) => {
        $crate::Count::<1, 2, _>::new( $l .. $r)
    };
    ([$l:literal - $r:literal]+) => {
        $crate::Count::<1, usize::MAX, _>::new( $l .. $r)
    };
    ([$l:literal - $r:literal]?) => {
        $crate::Count::<0, 2, _>::new( $l .. $r)
    };
    ([$l:literal - $r:literal]*) => {
        $crate::Count::<0, usize::MAX, _>::new( $l .. $r)
    };
    ([$l:literal - $r:literal]{$start:literal}) => {
        $crate::Count::<$start, $start + 1, _>::new( $l .. $r)
    };
    ([$l:literal - $r:literal]{$start:literal,}) => {
        $crate::Count::<$start, usize::MAX, _>::new( $l .. $r)
    };
    ([$l:literal - $r:literal]{$start:literal, $end:literal}) => {
        $crate::Count::<$start, $end, _>::new( $l .. $r)
    };
    ([^$l:literal - $r:literal]) => {
        $crate::Count::<1, 2, _>::new($crate::Not::new( $l .. $r))
    };
    ([^$l:literal - $r:literal]+) => {
        $crate::Count::<1, usize::MAX, _>::new($crate::Not::new( $l .. $r))
    };
    ([^$l:literal - $r:literal]?) => {
        $crate::Count::<0, 2, _>::new($crate::Not::new( $l .. $r))
    };
    ([^$l:literal - $r:literal]*) => {
        $crate::Count::<0, usize::MAX, _>::new($crate::Not::new( $l .. $r))
    };
    ([^$l:literal - $r:literal]{$start:literal}) => {
        $crate::Count::<$start, $start + 1, _>::new($crate::Not::new( $l .. $r))
    };
    ([^$l:literal - $r:literal]{$start:literal,}) => {
        $crate::Count::<$start, usize::MAX, _>::new($crate::Not::new( $l .. $r))
    };
    ([^$l:literal - $r:literal]{$start:literal, $end:literal}) => {
        $crate::Count::<$start, $end, _>::new($crate::Not::new( $l .. $r))
    };
}

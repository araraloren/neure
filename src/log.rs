#[cfg(feature = "log")]
pub(crate) use tracing::trace as neure_trace;

#[cfg(feature = "log")]
pub(crate) use tracing::debug as neure_debug;

#[cfg(not(feature = "log"))]
#[macro_use]
pub(crate) mod inner_log {
    #[macro_export]
    macro_rules! neure_trace {
        ($($arg:tt)*) => {
            ();
        };
    }

    #[macro_export]
    macro_rules! neure_debug {
        ($($arg:tt)*) => {
            ();
        };
    }
}

macro_rules! trace_retval {
    ($name:literal, $in:ident, $ret:expr) => {{
        $crate::trace_retval!($name, "Neu", $in, $ret)
    }};
    ($name:literal, $trait:literal, $in:ident, $ret:expr) => {{
        let ret = $ret;
        $crate::neure_trace!("{}({}) in = {:?}: {:?}", $name, $trait, $in, ret);
        ret
    }};
    ($name:literal, $inner:ident, $in:ident, $ret:expr) => {{
        $crate::trace_retval!($name, "Neu", $inner, $in, $ret)
    }};
    ($name:literal, $trait:literal, $inner:ident, $in:ident, $ret:expr) => {{
        let ret = $ret;
        $crate::neure_trace!(
            "{}({}) dat({:?}) in = {:?}: {:?}",
            $name,
            $trait,
            $inner,
            $in,
            ret
        );
        ret
    }};
}

macro_rules! debug_beg {
    ($name:literal, $trait:literal, $beg:expr) => {{
        $crate::neure_debug!("{}({}) beg = {}", $name, $trait, $beg);
    }};
    ($name:literal, $trait:literal, $val:expr, $beg:expr) => {{
        $crate::neure_debug!("{}({}) dat({:?}) beg = {}", $name, $trait, $val, $beg);
    }};
}

macro_rules! debug_regex_beg {
    ($name:literal, $beg:expr) => {{
        $crate::debug_beg!($name, "Regex", $beg)
    }};
    ($name:literal, $val:expr, $beg:expr) => {{
        $crate::debug_beg!($name, "Regex", $val, $beg)
    }};
}

macro_rules! debug_ctor_beg {
    ($name:literal, $beg:expr) => {{
        $crate::debug_beg!($name, "Ctor", $beg)
    }};
    ($name:literal, $val:expr, $beg:expr) => {{
        $crate::debug_beg!($name, "Ctor", $val, $beg)
    }};
}

macro_rules! debug_retval {
    ($name:literal, $trait:literal, $beg:expr, $end:expr, $ret:expr) => {{
        let ret = $ret;
        $crate::neure_debug!(
            "{}({}) res = {} .. {}: {:?}",
            $name,
            $trait,
            $beg,
            $end,
            ret
        );
        ret
    }};
}

macro_rules! debug_regex_reval {
    ($name:literal, $beg:expr, $end:expr, $ret:expr) => {{
        $crate::debug_retval!($name, "Regex", $beg, $end, $ret)
    }};
    ($name:literal, $val:expr, $beg:expr, $end:expr,  $ret:expr) => {{
        let ret = $ret;
        $crate::neure_debug!(
            "{}(Regex) dat({:?}) res = {} .. {}: {:?}",
            $name,
            $val,
            $beg,
            $end,
            ret
        );
        ret
    }};
}

macro_rules! debug_ctor_reval {
    ($name:literal, $beg:expr, $end:expr, $ret:expr) => {{
        $crate::neure_debug!("{}(Ctor) res = {} .. {}: {:?}", $name, $beg, $end, $ret);
    }};
}

pub(crate) use debug_beg;
pub(crate) use debug_ctor_beg;
pub(crate) use debug_ctor_reval;
pub(crate) use debug_regex_beg;
pub(crate) use debug_regex_reval;
pub(crate) use debug_retval;
pub(crate) use trace_retval;

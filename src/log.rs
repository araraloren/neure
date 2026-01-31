#[cfg(not(any(feature = "tracing", feature = "log")))]
mod macro_for_log {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! neure_trace {
        ($($arg:tt)*) => {{}};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! neure_debug {
        ($($arg:tt)*) => {{}};
    }

    macro_rules! trace_retval {
        ($name:literal, $in:ident, $ret:expr) => {{ $crate::trace_retval!($name, "Neu", $in, $ret) }};
        ($name:literal, $trait:literal, $in:ident, $ret:expr) => {{
            let (_, _, _, ret) = ($name, $trait, $in, $ret);
            ret
        }};
        ($name:literal, $inner:ident, $in:ident, $ret:expr) => {{ $crate::trace_retval!($name, "Neu", $inner, $in, $ret) }};
        ($name:literal, $trait:literal, $inner:ident, $in:ident, $ret:expr) => {{
            let (_, _, _, _, ret) = ($name, $trait, $inner, $in, $ret);
            ret
        }};
    }

    macro_rules! debug_regex_beg {
        ($name:literal, $beg:expr) => {{
            let (_, _) = ($name, $beg);
        }};
        ($name:literal, $val:expr, $beg:expr) => {{
            let (_, _, _) = ($name, $val, $beg);
        }};
    }

    macro_rules! debug_ctor_beg {
        ($name:literal, $beg:expr) => {{
            let (_, _) = ($name, $beg);
        }};
        ($name:literal, $val:expr, $beg:expr) => {{
            let (_, _, _) = ($name, $val, $beg);
        }};
    }

    macro_rules! debug_regex_stage {
        ($name:literal, $stage:literal) => {{
            let (_, _) = ($name, $stage);
        }};
        ($name:literal, $stage:literal, $ret:expr) => {{
            let (_, _) = ($name, $stage);
            $ret
        }};
    }

    macro_rules! debug_ctor_stage {
        ($name:literal, $stage:literal) => {{
            let (_, _) = ($name, $stage);
        }};
        ($name:literal, $stage:literal, $ret:expr) => {{
            let (_, _) = ($name, $stage);
            $ret
        }};
    }

    macro_rules! debug_regex_reval {
        ($name:literal, $ret:expr) => {{
            let (_, ret) = ($name, $ret);
            ret
        }};
        ($name:literal, $val:expr, $ret:expr) => {{
            let (_, _, ret) = ($name, $val, $ret);
            ret
        }};
    }

    macro_rules! debug_ctor_reval {
        ($name:literal, $beg:expr, $end:expr, $ret:expr) => {{
            let (_, _, _, _) = ($name, $beg, $end, $ret);
        }};
        ($name:literal, $val:expr, $beg:expr, $end:expr, $ret:expr) => {{
            let (_, _, _, _, _) = ($name, $val, $beg, $end, $ret);
        }};
    }

    pub(crate) use debug_ctor_beg;
    pub(crate) use debug_ctor_reval;
    pub(crate) use debug_ctor_stage;
    pub(crate) use debug_regex_beg;
    pub(crate) use debug_regex_reval;
    pub(crate) use debug_regex_stage;
    pub(crate) use trace_retval;
}

#[cfg(any(feature = "tracing", feature = "log"))]
mod macro_for_log {
    macro_rules! trace_retval {
        ($name:literal, $in:ident, $ret:expr) => {{ $crate::trace_retval!($name, "Neu", $in, $ret) }};
        ($name:literal, $trait:literal, $in:ident, $ret:expr) => {{
            let ret = $ret;
            $crate::neure_trace!("{}({}) in = {:?} -> {:?}", $name, $trait, $in, ret);
            ret
        }};
        ($name:literal, $inner:ident, $in:ident, $ret:expr) => {{ $crate::trace_retval!($name, "Neu", $inner, $in, $ret) }};
        ($name:literal, $trait:literal, $inner:ident, $in:ident, $ret:expr) => {{
            let ret = $ret;
            $crate::neure_trace!(
                "{}({}) dat({:?}) in = {:?} -> {:?}",
                $name,
                $trait,
                $inner,
                $in,
                ret
            );
            ret
        }};
    }

    macro_rules! debug_regex_beg {
        ($name:literal, $beg:expr) => {{ $crate::neure_debug!("{}(Regex) beg = {}", $name, $beg) }};
        ($name:literal, $val:expr, $beg:expr) => {{ $crate::neure_debug!("{}(Regex) dat({:?}) beg = {}", $name, $val, $beg) }};
    }

    macro_rules! debug_ctor_beg {
        ($name:literal, $beg:expr) => {{ $crate::neure_debug!("{}(Ctor) beg = {}", $name, $beg) }};
        ($name:literal, $val:expr, $beg:expr) => {{ $crate::neure_debug!("{}(Ctor) dat({:?}) beg = {}", $name, $val, $beg) }};
    }

    macro_rules! debug_regex_stage {
        ($name:literal, $stage:literal) => {{ $crate::neure_debug!("{}(Regex) stage {}", $name, $stage) }};
        ($name:literal, $stage:literal, $ret:expr) => {{
            $crate::neure_debug!("{}(Regex) stage {}", $name, $stage);
            $ret
        }};
    }

    macro_rules! debug_ctor_stage {
        ($name:literal, $stage:literal) => {{ $crate::neure_debug!("{}(Ctor) stage {}", $name, $stage) }};
        ($name:literal, $stage:literal, $ret:expr) => {{
            $crate::neure_debug!("{}(Ctor) stage {}", $name, $stage);
            $ret
        }};
    }

    macro_rules! debug_regex_reval {
        ($name:literal, $ret:expr) => {{
            let ret = $ret;
            $crate::neure_debug!("{}(Regex) -> {:?}", $name, ret);
            ret
        }};
        ($name:literal, $val:expr, $ret:expr) => {{
            let ret = $ret;
            $crate::neure_debug!("{}(Regex) dat({:?}) -> {:?}", $name, $val, ret);
            ret
        }};
    }

    macro_rules! debug_ctor_reval {
        ($name:literal, $beg:expr, $end:expr, $ret:expr) => {{
            $crate::neure_debug!(
                "{}(Ctor) beg = {}, len = {} -> {:?}",
                $name,
                $beg,
                $end - $beg,
                $ret
            );
        }};
        ($name:literal, $val:expr, $beg:expr, $end:expr, $ret:expr) => {{
            $crate::neure_debug!(
                "{}(Ctor) dat({:?}) beg = {}, len = {} -> {:?}",
                $name,
                $val,
                $beg,
                $end - $beg,
                $ret
            );
        }};
    }

    pub(crate) use debug_ctor_beg;
    pub(crate) use debug_ctor_reval;
    pub(crate) use debug_ctor_stage;
    pub(crate) use debug_regex_beg;
    pub(crate) use debug_regex_reval;
    pub(crate) use debug_regex_stage;
    #[cfg(feature = "log")]
    pub(crate) use log::debug as neure_debug;
    #[cfg(feature = "log")]
    pub(crate) use log::trace as neure_trace;
    pub(crate) use trace_retval;
    #[cfg(feature = "tracing")]
    pub(crate) use tracing::debug as neure_debug;
    #[cfg(feature = "tracing")]
    pub(crate) use tracing::trace as neure_trace;
}

#[cfg(all(feature = "log", feature = "tracing"))]
compile_error!("❌ Features 'log' and 'tracing' are mutually exclusive");

pub(crate) use macro_for_log::*;

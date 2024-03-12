(function() {var implementors = {
"neure":[["impl&lt;'a, 'b, C&gt; Freeze for <a class=\"struct\" href=\"neure/ctx/struct.CtxGuard.html\" title=\"struct neure::ctx::CtxGuard\">CtxGuard</a>&lt;'a, 'b, C&gt;",1,["neure::ctx::guard::CtxGuard"]],["impl&lt;I, B&gt; Freeze for <a class=\"struct\" href=\"neure/ctx/struct.PolicyCtx.html\" title=\"struct neure::ctx::PolicyCtx\">PolicyCtx</a>&lt;I, B&gt;<div class=\"where\">where\n    B: Freeze,\n    I: Freeze,</div>",1,["neure::ctx::policy::PolicyCtx"]],["impl&lt;'a, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; Freeze for <a class=\"struct\" href=\"neure/ctx/struct.RegexCtx.html\" title=\"struct neure::ctx::RegexCtx\">RegexCtx</a>&lt;'a, T&gt;",1,["neure::ctx::regex::RegexCtx"]],["impl Freeze for <a class=\"struct\" href=\"neure/ctx/struct.Span.html\" title=\"struct neure::ctx::Span\">Span</a>",1,["neure::ctx::span::Span"]],["impl&lt;C, T&gt; Freeze for <a class=\"struct\" href=\"neure/ctx/struct.RePolicy.html\" title=\"struct neure::ctx::RePolicy\">RePolicy</a>&lt;C, T&gt;<div class=\"where\">where\n    T: Freeze,</div>",1,["neure::ctx::RePolicy"]],["impl Freeze for <a class=\"enum\" href=\"neure/err/enum.Error.html\" title=\"enum neure::err::Error\">Error</a>",1,["neure::err::Error"]],["impl&lt;'a, T&gt; Freeze for <a class=\"struct\" href=\"neure/iter/struct.BytesIndices.html\" title=\"struct neure::iter::BytesIndices\">BytesIndices</a>&lt;'a, T&gt;",1,["neure::iter::byte::BytesIndices"]],["impl&lt;'a, 'b, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; Freeze for <a class=\"struct\" href=\"neure/iter/struct.IteratorBySpan.html\" title=\"struct neure::iter::IteratorBySpan\">IteratorBySpan</a>&lt;'a, 'b, T&gt;",1,["neure::iter::span::IteratorBySpan"]],["impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"neure/iter/struct.SpanIterator.html\" title=\"struct neure::iter::SpanIterator\">SpanIterator</a>&lt;'a&gt;",1,["neure::iter::span::SpanIterator"]],["impl Freeze for <a class=\"struct\" href=\"neure/map/struct.Single.html\" title=\"struct neure::map::Single\">Single</a>",1,["neure::map::Single"]],["impl Freeze for <a class=\"struct\" href=\"neure/map/struct.Select0.html\" title=\"struct neure::map::Select0\">Select0</a>",1,["neure::map::Select0"]],["impl Freeze for <a class=\"struct\" href=\"neure/map/struct.Select1.html\" title=\"struct neure::map::Select1\">Select1</a>",1,["neure::map::Select1"]],["impl Freeze for <a class=\"struct\" href=\"neure/map/struct.SelectEq.html\" title=\"struct neure::map::SelectEq\">SelectEq</a>",1,["neure::map::SelectEq"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.FromStr.html\" title=\"struct neure::map::FromStr\">FromStr</a>&lt;T&gt;",1,["neure::map::FromStr"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.MapInto.html\" title=\"struct neure::map::MapInto\">MapInto</a>&lt;T&gt;",1,["neure::map::MapInto"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.MapTryInto.html\" title=\"struct neure::map::MapTryInto\">MapTryInto</a>&lt;T&gt;",1,["neure::map::MapTryInto"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.FromStrRadix.html\" title=\"struct neure::map::FromStrRadix\">FromStrRadix</a>&lt;T&gt;",1,["neure::map::FromStrRadix"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.FromUtf8.html\" title=\"struct neure::map::FromUtf8\">FromUtf8</a>&lt;T&gt;",1,["neure::map::FromUtf8"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.FromUtf8Lossy.html\" title=\"struct neure::map::FromUtf8Lossy\">FromUtf8Lossy</a>&lt;T&gt;",1,["neure::map::FromUtf8Lossy"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.FromLeBytes.html\" title=\"struct neure::map::FromLeBytes\">FromLeBytes</a>&lt;T&gt;",1,["neure::map::FromLeBytes"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.FromBeBytes.html\" title=\"struct neure::map::FromBeBytes\">FromBeBytes</a>&lt;T&gt;",1,["neure::map::FromBeBytes"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/map/struct.FromNeBytes.html\" title=\"struct neure::map::FromNeBytes\">FromNeBytes</a>&lt;T&gt;",1,["neure::map::FromNeBytes"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.True.html\" title=\"struct neure::neu::True\">True</a>&lt;T&gt;",1,["neure::neu::bool::True"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.False.html\" title=\"struct neure::neu::False\">False</a>&lt;T&gt;",1,["neure::neu::bool::False"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.NullCond.html\" title=\"struct neure::neu::NullCond\">NullCond</a>",1,["neure::neu::cond::NullCond"]],["impl&lt;'a, C, T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.RegexCond.html\" title=\"struct neure::neu::RegexCond\">RegexCond</a>&lt;'a, C, T&gt;<div class=\"where\">where\n    T: Freeze,</div>",1,["neure::neu::cond::RegexCond"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.Equal.html\" title=\"struct neure::neu::Equal\">Equal</a>&lt;T&gt;<div class=\"where\">where\n    T: Freeze,</div>",1,["neure::neu::equal::Equal"]],["impl&lt;U, I, T&gt; !Freeze for <a class=\"struct\" href=\"neure/neu/struct.MayUnit.html\" title=\"struct neure::neu::MayUnit\">MayUnit</a>&lt;U, I, T&gt;",1,["neure::neu::may::MayUnit"]],["impl&lt;L, R, T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.And.html\" title=\"struct neure::neu::And\">And</a>&lt;L, R, T&gt;<div class=\"where\">where\n    L: Freeze,\n    R: Freeze,</div>",1,["neure::neu::op_and::And"]],["impl&lt;U, T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.Not.html\" title=\"struct neure::neu::Not\">Not</a>&lt;U, T&gt;<div class=\"where\">where\n    U: Freeze,</div>",1,["neure::neu::op_not::Not"]],["impl&lt;C, U, T, I&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.NeureOne.html\" title=\"struct neure::neu::NeureOne\">NeureOne</a>&lt;C, U, T, I&gt;<div class=\"where\">where\n    I: Freeze,\n    U: Freeze,</div>",1,["neure::neu::op_one::NeureOne"]],["impl&lt;C, U, T, I&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.NeureOneMore.html\" title=\"struct neure::neu::NeureOneMore\">NeureOneMore</a>&lt;C, U, T, I&gt;<div class=\"where\">where\n    I: Freeze,\n    U: Freeze,</div>",1,["neure::neu::op_one::NeureOneMore"]],["impl&lt;L, R, T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.Or.html\" title=\"struct neure::neu::Or\">Or</a>&lt;L, R, T&gt;<div class=\"where\">where\n    L: Freeze,\n    R: Freeze,</div>",1,["neure::neu::op_or::Or"]],["impl&lt;const M: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a>, C, U, I&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.NeureRepeat.html\" title=\"struct neure::neu::NeureRepeat\">NeureRepeat</a>&lt;M, N, C, U, I&gt;<div class=\"where\">where\n    I: Freeze,\n    U: Freeze,</div>",1,["neure::neu::op_repeat::NeureRepeat"]],["impl&lt;C, U, I&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.NeureRepeatRange.html\" title=\"struct neure::neu::NeureRepeatRange\">NeureRepeatRange</a>&lt;C, U, I&gt;<div class=\"where\">where\n    I: Freeze,\n    U: Freeze,</div>",1,["neure::neu::op_repeat::NeureRepeatRange"]],["impl&lt;C, L, R, T, I&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.NeureThen.html\" title=\"struct neure::neu::NeureThen\">NeureThen</a>&lt;C, L, R, T, I&gt;<div class=\"where\">where\n    I: Freeze,\n    L: Freeze,\n    R: Freeze,</div>",1,["neure::neu::op_then::NeureThen"]],["impl&lt;C, U, T, I&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.NeureZeroOne.html\" title=\"struct neure::neu::NeureZeroOne\">NeureZeroOne</a>&lt;C, U, T, I&gt;<div class=\"where\">where\n    I: Freeze,\n    U: Freeze,</div>",1,["neure::neu::op_zero::NeureZeroOne"]],["impl&lt;C, U, T, I&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.NeureZeroMore.html\" title=\"struct neure::neu::NeureZeroMore\">NeureZeroMore</a>&lt;C, U, T, I&gt;<div class=\"where\">where\n    I: Freeze,\n    U: Freeze,</div>",1,["neure::neu::op_zero::NeureZeroMore"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/neu/struct.CRange.html\" title=\"struct neure::neu::CRange\">CRange</a>&lt;T&gt;<div class=\"where\">where\n    T: Freeze,</div>",1,["neure::neu::range::CRange"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Alphabetic.html\" title=\"struct neure::neu::Alphabetic\">Alphabetic</a>",1,["neure::neu::units::Alphabetic"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Alphanumeric.html\" title=\"struct neure::neu::Alphanumeric\">Alphanumeric</a>",1,["neure::neu::units::Alphanumeric"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Ascii.html\" title=\"struct neure::neu::Ascii\">Ascii</a>",1,["neure::neu::units::Ascii"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiAlphabetic.html\" title=\"struct neure::neu::AsciiAlphabetic\">AsciiAlphabetic</a>",1,["neure::neu::units::AsciiAlphabetic"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiAlphanumeric.html\" title=\"struct neure::neu::AsciiAlphanumeric\">AsciiAlphanumeric</a>",1,["neure::neu::units::AsciiAlphanumeric"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiControl.html\" title=\"struct neure::neu::AsciiControl\">AsciiControl</a>",1,["neure::neu::units::AsciiControl"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiDigit.html\" title=\"struct neure::neu::AsciiDigit\">AsciiDigit</a>",1,["neure::neu::units::AsciiDigit"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiGraphic.html\" title=\"struct neure::neu::AsciiGraphic\">AsciiGraphic</a>",1,["neure::neu::units::AsciiGraphic"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiHexDigit.html\" title=\"struct neure::neu::AsciiHexDigit\">AsciiHexDigit</a>",1,["neure::neu::units::AsciiHexDigit"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiLowercase.html\" title=\"struct neure::neu::AsciiLowercase\">AsciiLowercase</a>",1,["neure::neu::units::AsciiLowercase"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiPunctuation.html\" title=\"struct neure::neu::AsciiPunctuation\">AsciiPunctuation</a>",1,["neure::neu::units::AsciiPunctuation"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiUppercase.html\" title=\"struct neure::neu::AsciiUppercase\">AsciiUppercase</a>",1,["neure::neu::units::AsciiUppercase"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.AsciiWhiteSpace.html\" title=\"struct neure::neu::AsciiWhiteSpace\">AsciiWhiteSpace</a>",1,["neure::neu::units::AsciiWhiteSpace"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Control.html\" title=\"struct neure::neu::Control\">Control</a>",1,["neure::neu::units::Control"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Digit.html\" title=\"struct neure::neu::Digit\">Digit</a>",1,["neure::neu::units::Digit"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Lowercase.html\" title=\"struct neure::neu::Lowercase\">Lowercase</a>",1,["neure::neu::units::Lowercase"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Numeric.html\" title=\"struct neure::neu::Numeric\">Numeric</a>",1,["neure::neu::units::Numeric"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Uppercase.html\" title=\"struct neure::neu::Uppercase\">Uppercase</a>",1,["neure::neu::units::Uppercase"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.WhiteSpace.html\" title=\"struct neure::neu::WhiteSpace\">WhiteSpace</a>",1,["neure::neu::units::WhiteSpace"]],["impl Freeze for <a class=\"struct\" href=\"neure/neu/struct.Wild.html\" title=\"struct neure::neu::Wild\">Wild</a>",1,["neure::neu::units::Wild"]],["impl Freeze for <a class=\"struct\" href=\"neure/re/struct.Pass.html\" title=\"struct neure::re::Pass\">Pass</a>",1,["neure::re::extract::Pass"]],["impl&lt;R&gt; Freeze for <a class=\"struct\" href=\"neure/re/struct.NullRegex.html\" title=\"struct neure::re::NullRegex\">NullRegex</a>&lt;R&gt;",1,["neure::re::null::NullRegex"]],["impl Freeze for <a class=\"struct\" href=\"neure/re/struct.RecParser.html\" title=\"struct neure::re::RecParser\">RecParser</a>",1,["neure::re::rec::RecParser"]],["impl Freeze for <a class=\"struct\" href=\"neure/re/struct.RecParserSync.html\" title=\"struct neure::re::RecParserSync\">RecParserSync</a>",1,["neure::re::rec::RecParserSync"]],["impl&lt;I&gt; Freeze for <a class=\"struct\" href=\"neure/re/struct.WrappedTy.html\" title=\"struct neure::re::WrappedTy\">WrappedTy</a>&lt;I&gt;<div class=\"where\">where\n    I: Freeze,</div>",1,["neure::re::wrap::WrappedTy"]],["impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a>, T&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Array.html\" title=\"struct neure::re::ctor::Array\">Array</a>&lt;N, T&gt;<div class=\"where\">where\n    T: Freeze,</div>",1,["neure::re::ctor::array::Array"]],["impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a>, K, V&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.PairArray.html\" title=\"struct neure::re::ctor::PairArray\">PairArray</a>&lt;N, K, V&gt;<div class=\"where\">where\n    K: Freeze,\n    V: Freeze,</div>",1,["neure::re::ctor::array::PairArray"]],["impl&lt;I&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.BoxedCtor.html\" title=\"struct neure::re::ctor::BoxedCtor\">BoxedCtor</a>&lt;I&gt;",1,["neure::re::ctor::boxed::BoxedCtor"]],["impl&lt;C, P, O, V&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Collect.html\" title=\"struct neure::re::ctor::Collect\">Collect</a>&lt;C, P, O, V&gt;<div class=\"where\">where\n    P: Freeze,</div>",1,["neure::re::ctor::collect::Collect"]],["impl&lt;C, P, F&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.DynamicCreateCtorThen.html\" title=\"struct neure::re::ctor::DynamicCreateCtorThen\">DynamicCreateCtorThen</a>&lt;C, P, F&gt;<div class=\"where\">where\n    F: Freeze,\n    P: Freeze,</div>",1,["neure::re::ctor::dthen::DynamicCreateCtorThen"]],["impl&lt;'a, 'b, C, M, O, H, A&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.DynamicBoxedCtor.html\" title=\"struct neure::re::ctor::DynamicBoxedCtor\">DynamicBoxedCtor</a>&lt;'a, 'b, C, M, O, H, A&gt;",1,["neure::re::ctor::dynamic::DynamicBoxedCtor"]],["impl&lt;'a, 'b, C, M, O, H, A&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.DynamicBoxedCtorSync.html\" title=\"struct neure::re::ctor::DynamicBoxedCtorSync\">DynamicBoxedCtorSync</a>&lt;'a, 'b, C, M, O, H, A&gt;",1,["neure::re::ctor::dynamic::DynamicBoxedCtorSync"]],["impl&lt;'a, 'b, C, M, O, H, A&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.DynamicArcCtor.html\" title=\"struct neure::re::ctor::DynamicArcCtor\">DynamicArcCtor</a>&lt;'a, 'b, C, M, O, H, A&gt;",1,["neure::re::ctor::dynamic::DynamicArcCtor"]],["impl&lt;'a, 'b, C, M, O, H, A&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.DynamicRcCtor.html\" title=\"struct neure::re::ctor::DynamicRcCtor\">DynamicRcCtor</a>&lt;'a, 'b, C, M, O, H, A&gt;",1,["neure::re::ctor::dynamic::DynamicRcCtor"]],["impl&lt;C, P, I, E&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.IfRegex.html\" title=\"struct neure::re::ctor::IfRegex\">IfRegex</a>&lt;C, P, I, E&gt;<div class=\"where\">where\n    E: Freeze,\n    I: Freeze,\n    P: Freeze,</div>",1,["neure::re::ctor::if::IfRegex"]],["impl&lt;C, L, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.LongestTokenMatch.html\" title=\"struct neure::re::ctor::LongestTokenMatch\">LongestTokenMatch</a>&lt;C, L, R&gt;<div class=\"where\">where\n    L: Freeze,\n    R: Freeze,</div>",1,["neure::re::ctor::ltm::LongestTokenMatch"]],["impl&lt;C, P, F, O&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Map.html\" title=\"struct neure::re::ctor::Map\">Map</a>&lt;C, P, F, O&gt;<div class=\"where\">where\n    F: Freeze,\n    P: Freeze,</div>",1,["neure::re::ctor::map::Map"]],["impl&lt;C, P&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.OptionPat.html\" title=\"struct neure::re::ctor::OptionPat\">OptionPat</a>&lt;C, P&gt;<div class=\"where\">where\n    P: Freeze,</div>",1,["neure::re::ctor::opt::OptionPat"]],["impl&lt;C, L, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Or.html\" title=\"struct neure::re::ctor::Or\">Or</a>&lt;C, L, R&gt;<div class=\"where\">where\n    L: Freeze,\n    R: Freeze,</div>",1,["neure::re::ctor::or::Or"]],["impl&lt;C, P, T&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Pad.html\" title=\"struct neure::re::ctor::Pad\">Pad</a>&lt;C, P, T&gt;<div class=\"where\">where\n    P: Freeze,\n    T: Freeze,</div>",1,["neure::re::ctor::pad::Pad"]],["impl&lt;C, P, T&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Padded.html\" title=\"struct neure::re::ctor::Padded\">Padded</a>&lt;C, P, T&gt;<div class=\"where\">where\n    P: Freeze,\n    T: Freeze,</div>",1,["neure::re::ctor::pad::Padded"]],["impl&lt;C, P&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Pattern.html\" title=\"struct neure::re::ctor::Pattern\">Pattern</a>&lt;C, P&gt;<div class=\"where\">where\n    P: Freeze,</div>",1,["neure::re::ctor::pat::Pattern"]],["impl&lt;C, P, L, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Quote.html\" title=\"struct neure::re::ctor::Quote\">Quote</a>&lt;C, P, L, R&gt;<div class=\"where\">where\n    L: Freeze,\n    P: Freeze,\n    R: Freeze,</div>",1,["neure::re::ctor::quote::Quote"]],["impl&lt;C, P&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Repeat.html\" title=\"struct neure::re::ctor::Repeat\">Repeat</a>&lt;C, P&gt;<div class=\"where\">where\n    P: Freeze,</div>",1,["neure::re::ctor::repeat::Repeat"]],["impl&lt;C, L, S, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.SepOnce.html\" title=\"struct neure::re::ctor::SepOnce\">SepOnce</a>&lt;C, L, S, R&gt;<div class=\"where\">where\n    L: Freeze,\n    R: Freeze,\n    S: Freeze,</div>",1,["neure::re::ctor::sep::SepOnce"]],["impl&lt;C, P, S&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Separate.html\" title=\"struct neure::re::ctor::Separate\">Separate</a>&lt;C, P, S&gt;<div class=\"where\">where\n    P: Freeze,\n    S: Freeze,</div>",1,["neure::re::ctor::sep::Separate"]],["impl&lt;C, P, S, O, V&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.SepCollect.html\" title=\"struct neure::re::ctor::SepCollect\">SepCollect</a>&lt;C, P, S, O, V&gt;<div class=\"where\">where\n    P: Freeze,\n    S: Freeze,</div>",1,["neure::re::ctor::sep::SepCollect"]],["impl&lt;'a, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a>, T&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Slice.html\" title=\"struct neure::re::ctor::Slice\">Slice</a>&lt;'a, N, T&gt;",1,["neure::re::ctor::slice::Slice"]],["impl&lt;'a, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.usize.html\">usize</a>, K, V&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.PairSlice.html\" title=\"struct neure::re::ctor::PairSlice\">PairSlice</a>&lt;'a, N, K, V&gt;",1,["neure::re::ctor::slice::PairSlice"]],["impl&lt;C, L, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Then.html\" title=\"struct neure::re::ctor::Then\">Then</a>&lt;C, L, R&gt;<div class=\"where\">where\n    L: Freeze,\n    R: Freeze,</div>",1,["neure::re::ctor::then::Then"]],["impl&lt;C, L, I, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.IfThen.html\" title=\"struct neure::re::ctor::IfThen\">IfThen</a>&lt;C, L, I, R&gt;<div class=\"where\">where\n    I: Freeze,\n    L: Freeze,\n    R: Freeze,</div>",1,["neure::re::ctor::then::IfThen"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.Vector.html\" title=\"struct neure::re::ctor::Vector\">Vector</a>&lt;T&gt;",1,["neure::re::ctor::vec::Vector"]],["impl&lt;K, V&gt; Freeze for <a class=\"struct\" href=\"neure/re/ctor/struct.PairVector.html\" title=\"struct neure::re::ctor::PairVector\">PairVector</a>&lt;K, V&gt;",1,["neure::re::ctor::vec::PairVector"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.BoxedRegex.html\" title=\"struct neure::re::regex::BoxedRegex\">BoxedRegex</a>&lt;T&gt;",1,["neure::re::regex::boxed::BoxedRegex"]],["impl&lt;C, P, F&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.DynamicCreateRegexThen.html\" title=\"struct neure::re::regex::DynamicCreateRegexThen\">DynamicCreateRegexThen</a>&lt;C, P, F&gt;<div class=\"where\">where\n    F: Freeze,\n    P: Freeze,</div>",1,["neure::re::regex::dthen::DynamicCreateRegexThen"]],["impl&lt;'a, C, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.DynamicBoxedRegex.html\" title=\"struct neure::re::regex::DynamicBoxedRegex\">DynamicBoxedRegex</a>&lt;'a, C, R&gt;",1,["neure::re::regex::dynamic::DynamicBoxedRegex"]],["impl&lt;'a, C, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.DynamicArcRegex.html\" title=\"struct neure::re::regex::DynamicArcRegex\">DynamicArcRegex</a>&lt;'a, C, R&gt;",1,["neure::re::regex::dynamic::DynamicArcRegex"]],["impl&lt;'a, C, R&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.DynamicRcRegex.html\" title=\"struct neure::re::regex::DynamicRcRegex\">DynamicRcRegex</a>&lt;'a, C, R&gt;",1,["neure::re::regex::dynamic::DynamicRcRegex"]],["impl&lt;'a, T&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.LitSlice.html\" title=\"struct neure::re::regex::LitSlice\">LitSlice</a>&lt;'a, T&gt;",1,["neure::re::regex::literal::LitSlice"]],["impl&lt;'a&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.LitString.html\" title=\"struct neure::re::regex::LitString\">LitString</a>&lt;'a&gt;",1,["neure::re::regex::literal::LitString"]],["impl&lt;T&gt; Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.RegexNot.html\" title=\"struct neure::re::regex::RegexNot\">RegexNot</a>&lt;T&gt;<div class=\"where\">where\n    T: Freeze,</div>",1,["neure::re::regex::not::RegexNot"]],["impl Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.AnchorStart.html\" title=\"struct neure::re::regex::AnchorStart\">AnchorStart</a>",1,["neure::re::regex::AnchorStart"]],["impl Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.AnchorEnd.html\" title=\"struct neure::re::regex::AnchorEnd\">AnchorEnd</a>",1,["neure::re::regex::AnchorEnd"]],["impl Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.Consume.html\" title=\"struct neure::re::regex::Consume\">Consume</a>",1,["neure::re::regex::Consume"]],["impl Freeze for <a class=\"struct\" href=\"neure/re/regex/struct.ConsumeAll.html\" title=\"struct neure::re::regex::ConsumeAll\">ConsumeAll</a>",1,["neure::re::regex::ConsumeAll"]],["impl Freeze for <a class=\"struct\" href=\"neure/span/struct.SimpleStorer.html\" title=\"struct neure::span::SimpleStorer\">SimpleStorer</a>",1,["neure::span::SimpleStorer"]]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
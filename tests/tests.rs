#![allow(non_snake_case)]

extern crate cpp_demangle;
extern crate diff;

use std::io::Write;

fn assert_demangles_as(mangled: &str, expected: &str) {
    let sym = cpp_demangle::BorrowedSymbol::new(mangled.as_bytes())
        .expect("should parse mangled symbol ok");

    let mut actual = vec![];
    write!(&mut actual, "{}", sym).expect("should demangle symbol ok");
    let actual = String::from_utf8(actual).expect("should demangle to valid utf-8");

    if expected != actual {
        println!("");
        println!("Diff:");
        println!("--- expected");
        print!("+++ actual");

        let mut last = None;
        for cmp in diff::chars(expected, &actual) {
            match (last, cmp.clone()) {
                (Some(diff::Result::Left(_)), diff::Result::Left(_))
                | (Some(diff::Result::Both(..)), diff::Result::Both(..))
                | (Some(diff::Result::Right(_)), diff::Result::Right(_)) => {}

                (_, diff::Result::Left(_)) => print!("\n-"),
                (_, diff::Result::Both(..)) => print!("\n "),
                (_, diff::Result::Right(_)) => print!("\n+"),
            };
            match cmp.clone() {
                diff::Result::Left(c) | diff::Result::Both(c, _) | diff::Result::Right(c) => {
                    print!("{}", c)
                }
            }
            last = Some(cmp);
        }
        println!("");
    }

    assert_eq!(expected, actual);
}

fn assert_does_not_demangle(s: &str) {
    if let Ok(sym) = cpp_demangle::BorrowedSymbol::new(s.as_bytes()) {
        panic!("Unexpectedly demangled '{}' as '{}'", s, sym);
    }
}

macro_rules! demangles {
    ( $mangled:ident , $demangled:expr ) => {
        demangles!($mangled, stringify!($mangled), $demangled);
    };
    ( $name:ident , $mangled:expr , $demangled:expr ) => {
        #[test]
        fn $name() {
            assert_demangles_as($mangled, $demangled);
        }
    }
}

macro_rules! does_not_demangle {
    ( $name:ident , $s:expr ) => {
        #[test]
        fn $name() {
            assert_does_not_demangle($s);
        }
    }
}

// This should definitely not parse and demangle as
// `operator()(unsigned __int128, short, long double)`.
does_not_demangle!(close_should_not_demangle, "close");

demangles!(
    _Z20instantiate_with_intI3FooET_IiEv,
    "Foo<int> instantiate_with_int<Foo>()"
);
demangles!(_Z3fooISt6vectorIiEEvv, "void foo<std::vector<int> >()");
demangles!(__ZN3foo3barE3quxS0_, "foo::bar(qux, qux)");
demangles!(__ZN3foo3barE3quxS_, "foo::bar(qux, foo)");

demangles!(
    _ZN4funcI2TyEEN6ResultIT_EES3_,
    "Result<Ty> func<Ty>(Result<Ty>)"
);
demangles!(_ZN4funcI2TyEEN6ResultIT_EES2_, "Result<Ty> func<Ty>(Ty)");
demangles!(
    _ZN4funcI2TyEEN6ResultIT_EES1_,
    "Result<Ty> func<Ty>(Result)"
);
demangles!(_ZN4funcI2TyEEN6ResultIT_EES0_, "Result<Ty> func<Ty>(Ty)");
demangles!(_ZN4funcI2TyEEN6ResultIT_EES_, "Result<Ty> func<Ty>(func)");

demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), Ty)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES0_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), Ty::method)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES1_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), Ty)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES2_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), char const)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES3_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), char const*)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES4_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), void (char const*))"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES5_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), void (Ty::*)(char const*))"
);

demangles!(_ZNK1fB5cxx11Ev, "f[abi:cxx11]() const");

demangles!(
    _ZN4base8internal14CheckedSubImplIlEENSt9enable_ifIXsrSt14numeric_limitsIT_E10is_integerEbE4typeES4_S4_PS4_,
    "std::enable_if<std::numeric_limits<long>::is_integer, bool>::type base::internal::CheckedSubImpl<long>(long, long, long*)"
);

demangles!(
    _ZZN7mozilla12EMEDecryptor5FlushEvENUlvE_D4Ev,
    "mozilla::EMEDecryptor::Flush()::{lambda()#1}::~{lambda()#1}()"
);

demangles!(
    _ZSt4copyIPKcPcET0_T_S4_S3_,
    "char* std::copy<char const*, char*>(char const*, char const*, char*)"
);

demangles!(
    _Z9_mm_or_psDv4_fS_,
    "_mm_or_ps(float __vector(4), float __vector(4))"
);

demangles!(
    _ZN5space20templated_trampolineIPFvvEEEvT_,
    "void space::templated_trampoline<void (*)()>(void (*)())"
);

demangles!(
    _Z18convertCase_helperIN14QUnicodeTables14CasefoldTraitsEtET0_S2_,
    "unsigned short convertCase_helper<QUnicodeTables::CasefoldTraits, unsigned short>(unsigned short)"
);

demangles!(
    _ZnwmRKSt9nothrow_t,
    "operator new(unsigned long, std::nothrow_t const&)"
);

demangles!(
    _ZGRL13MozLangGroups_,
    "reference temporary #0 for MozLangGroups"
);

demangles!(_ZZ3abcvEN3defD0Ev, "abc()::def::~def()");
demangles!(
    _ZZN13CrashReporter7OOPInitEvEN17ProxyToMainThreadD0Ev,
    "CrashReporter::OOPInit()::ProxyToMainThread::~ProxyToMainThread()"
);

demangles!(_ZUlvE_, "{lambda()#1}");
demangles!(_ZZ3aaavEUlvE_, "aaa()::{lambda()#1}");
demangles!(_ZZ3aaavENUlvE_3bbbE, "aaa()::{lambda()#1}::bbb");
demangles!(_ZN3aaaUlvE_D1Ev, "aaa::{lambda()#1}::~{lambda()#1}()");

demangles!(_ZZ3aaavEN3bbbD1Ev, "aaa()::bbb::~bbb()");
demangles!(
    _ZZ3aaavENUlvE_D1Ev,
    // libiberty says "aaa()::{lambda()#1}::~aaa()" but I am pretty sure that is
    // a bug, especially given the previous demangling, which is the same but
    // with an identifier instead of a lambda. Finally, both demangle.go and my
    // OSX system `__cxa_demangle` agree with this destructor-of-the-lambda
    // interpretation.
    "aaa()::{lambda()#1}::~{lambda()#1}()"
);

demangles!(
    multiple_nested_local_names_and_operator_call_and_a_lambda_and_a_destructor,
    "_ZZZN7mozilla12MediaManager12GetUserMediaEP18nsPIDOMWindowInnerRKNS_3dom22MediaStreamConstraintsEP33nsIDOMGetUserMediaSuccessCallbackP31nsIDOMGetUserMediaErrorCallbackNS3_10CallerTypeEEN4$_30clERP8nsTArrayI6RefPtrINS_11MediaDeviceEEEENUlRPKcE_D1Ev",
    "mozilla::MediaManager::GetUserMedia(nsPIDOMWindowInner*, mozilla::dom::MediaStreamConstraints const&, nsIDOMGetUserMediaSuccessCallback*, nsIDOMGetUserMediaErrorCallback*, mozilla::dom::CallerType)::$_30::operator()(nsTArray<RefPtr<mozilla::MediaDevice> >*&)::{lambda(char const*&)#1}::~{lambda(char const*&)#1}()"
);

demangles!(_ZN11InstrumentsL8gSessionE, "Instruments::gSession");
demangles!(
    _ZTWN2js10TlsContextE,
    "TLS wrapper function for js::TlsContext"
);

demangles!(_Z3fooILb0EEvi, "void foo<false>(int)");
demangles!(_Z3fooILb1EEvi, "void foo<true>(int)");
demangles!(_Z3fooILb2EEvi, "void foo<(bool)2>(int)");
demangles!(_Z3fooILb999999EEvi, "void foo<(bool)999999>(int)");
demangles!(_Z3fooILbaaaaaaEEvi, "void foo<(bool)aaaaaa>(int)");
demangles!(
    bool_literal_with_decimal,
    "_Z3fooILb999.999EEvi",
    "void foo<(bool)999.999>(int)"
);
demangles!(_Z3fooILbn1EEvi, "void foo<(bool)-1>(int)");
demangles!(_Z3fooILbn0EEvi, "void foo<(bool)-0>(int)");

demangles!(_Z3fooILc65EEvi, "void foo<(char)65>(int)");
demangles!(_Z3fooILc48EEvi, "void foo<(char)48>(int)");
demangles!(_Z3fooILc0EEvi, "void foo<(char)0>(int)");
demangles!(_Z3fooILc999999EEvi, "void foo<(char)999999>(int)");
demangles!(_Z3fooILcaaaaaaEEvi, "void foo<(char)aaaaaa>(int)");
demangles!(
    char_literal_with_decimal,
    "_Z3fooILc999.999EEvi",
    "void foo<(char)999.999>(int)"
);
demangles!(_Z3fooILcn65EEvi, "void foo<(char)-65>(int)");
demangles!(
    char_literal_with_negative_sign,
    "_Z3fooILc-65EEvi",
    "void foo<(char)-65>(int)"
);

demangles!(_Z3fooILd65EEvi, "void foo<(double)[65]>(int)");
demangles!(_Z3fooILd48EEvi, "void foo<(double)[48]>(int)");
demangles!(_Z3fooILd0EEvi, "void foo<(double)[0]>(int)");
demangles!(_Z3fooILd999999EEvi, "void foo<(double)[999999]>(int)");
demangles!(_Z3fooILdaaaaaaEEvi, "void foo<(double)[aaaaaa]>(int)");
demangles!(
    double_literal_with_decimal,
    "_Z3fooILd999.999EEvi",
    "void foo<(double)[999.999]>(int)"
);
demangles!(_Z3fooILdn65EEvi, "void foo<(double)-[65]>(int)");
demangles!(
    double_literal_with_negative_sign,
    "_Z3fooILd-65EEvi",
    "void foo<(double)[-65]>(int)"
);

demangles!(_Z3fooILf65EEvi, "void foo<(float)[65]>(int)");
demangles!(_Z3fooILf48EEvi, "void foo<(float)[48]>(int)");
demangles!(_Z3fooILf0EEvi, "void foo<(float)[0]>(int)");
demangles!(_Z3fooILf999999EEvi, "void foo<(float)[999999]>(int)");
demangles!(_Z3fooILfaaaaaaEEvi, "void foo<(float)[aaaaaa]>(int)");
demangles!(
    float_literal_with_decimal,
    "_Z3fooILf999.999EEvi",
    "void foo<(float)[999.999]>(int)"
);
demangles!(_Z3fooILfn65EEvi, "void foo<(float)-[65]>(int)");
demangles!(
    float_literal_with_negative_sign,
    "_Z3fooILf-65EEvi",
    "void foo<(float)[-65]>(int)"
);

demangles!(_Z3fooILin1EEvv, "void foo<-1>()");
demangles!(_Z3fooILi0EEvv, "void foo<0>()");
demangles!(_Z3fooILin0EEvv, "void foo<-0>()");
demangles!(_Z3fooILi999999EEvv, "void foo<999999>()");
demangles!(_Z3fooILiaaaaaaEEvv, "void foo<aaaaaa>()");
demangles!(
    int_literal_with_decimal,
    "_Z3fooILi999.999EEvv",
    "void foo<999.999>()"
);

demangles!(_Z3abcrA_l, "abc(long restrict [])");
demangles!(_Z3abcFrA_lvE, "abc(long restrict (()) [])");
demangles!(_Z3abcFrPA_lvE, "abc(long (* restrict()) [])");
demangles!(
    _Z3abcM3defFPVPFrPivEvE,
    "abc(int* restrict (* volatile* (def::*)())())"
);
demangles!(
    _Z3abcM3defFPVPFrPA_lvEvE,
    "abc(long (* restrict (* volatile* (def::*)())()) [])"
);
demangles!(_Z3abcKFA_ivE, "abc(int (() const) [])");
demangles!(_Z3abcFFivElE, "abc(int (long)())");
demangles!(_Z3abcFPFrPivElE, "abc(int* restrict (*(long))())");
demangles!(_Z3abcKFvRSt7ostreamE, "abc(void (std::ostream&) const)");

demangles!(
    _ZL29SupportsTextureSampleCountMTLPU19objcproto9MTLDevice11objc_objectm,
    "SupportsTextureSampleCountMTL(objc_object objcproto9MTLDevice*, unsigned long)"
);

demangles!(
    _ZN3WTF8FunctionIFvvEE15CallableWrapperIZN7WebCore12CacheStorage5matchEONS_7VariantIJNS_6RefPtrINS4_12FetchRequestEEENS_6StringEEEEONS4_17CacheQueryOptionsEONS_3RefINS4_15DeferredPromiseEEEEUlvE_E4callEv,
    "WTF::Function<void ()>::CallableWrapper<WebCore::CacheStorage::match(WTF::Variant<WTF::RefPtr<WebCore::FetchRequest>, WTF::String>&&, WebCore::CacheQueryOptions&&, WTF::Ref<WebCore::DeferredPromise>&&)::{lambda()#1}>::call()"
);
demangles!(
    _ZN6WebKit25WebCacheStorageConnection17didReceiveMessageERN3IPC10ConnectionERNS1_7DecoderE,
    "WebKit::WebCacheStorageConnection::didReceiveMessage(IPC::Connection&, IPC::Decoder&)"
);
demangles!(
    _ZN3IPC10Connection15dispatchMessageESt10unique_ptrINS_7DecoderESt14default_deleteIS2_EE,
    "IPC::Connection::dispatchMessage(std::unique_ptr<IPC::Decoder, std::default_delete<IPC::Decoder> >)"
);
demangles!(
    _ZN3IPC10Connection18dispatchOneMessageEv,
    "IPC::Connection::dispatchOneMessage()"
);
demangles!(
    _ZN3WTF7RunLoop11performWorkEv,
    "WTF::RunLoop::performWork()"
);

demangles!(
    _Z4funcINS_6ObjectEENS0_IT_EEi,
    "func::Object<func::Object> func<func::Object>(int)"
);

demangles!(
    _ZN4funcINS_6ObjectEEENS0_IT_EEi,
    "func::Object<func::Object> func<func::Object>(int)"
);

demangles!(
    _ZNK7mozilla6layers19CapturedBufferState20ForEachTextureClientIZNS0_21CompositorBridgeChild21NotifyBeginAsyncPaintI6RefPtrIS1_EEEvRT_EUlS7_E_EEvS7_,
    "void mozilla::layers::CapturedBufferState::ForEachTextureClient<void mozilla::layers::CompositorBridgeChild::NotifyBeginAsyncPaint<RefPtr<mozilla::layers::CapturedBufferState> >(RefPtr<mozilla::layers::CapturedBufferState>&)::{lambda(auto:1)#1}>(void mozilla::layers::CompositorBridgeChild::NotifyBeginAsyncPaint<RefPtr<mozilla::layers::CapturedBufferState> >(RefPtr<mozilla::layers::CapturedBufferState>&)::{lambda(auto:1)#1}) const"
);

demangles!(
    _ZNSt3__116forward_as_tupleIJRKZN11tconcurrent6detail6sharedIFvvEEC1IZNS1_7yielder13await_suspendINS1_12task_promiseIvEEEEvNSt12experimental13coroutines_v116coroutine_handleIT_EEEUlvE_EEbNS_10shared_ptrINS1_17cancelation_tokenEEEOSE_PvEUlRSI_DpOT_E_EEENS_5tupleIJSP_EEESP_,
    "std::__1::tuple<bool tconcurrent::detail::shared<void ()>::shared<void tconcurrent::yielder::await_suspend<tconcurrent::task_promise<void> >(std::experimental::coroutines_v1::coroutine_handle<tconcurrent::task_promise<void> >)::{lambda()#1}>(std::__1::shared_ptr<tconcurrent::cancelation_token>, void tconcurrent::yielder::await_suspend<tconcurrent::task_promise<void> >(std::experimental::coroutines_v1::coroutine_handle<tconcurrent::task_promise<void> >)::{lambda()#1}&&, void*)::{lambda(tconcurrent::cancelation_token&, auto:1&&...)#1} const&&&...> std::__1::forward_as_tuple<bool tconcurrent::detail::shared<void ()>::shared<void tconcurrent::yielder::await_suspend<tconcurrent::task_promise<void> >(std::experimental::coroutines_v1::coroutine_handle<tconcurrent::task_promise<void> >)::{lambda()#1}>(std::__1::shared_ptr<tconcurrent::cancelation_token>, void tconcurrent::yielder::await_suspend<tconcurrent::task_promise<void> >(std::experimental::coroutines_v1::coroutine_handle<tconcurrent::task_promise<void> >)::{lambda()#1}&&, void*)::{lambda(tconcurrent::cancelation_token&, auto:1&&...)#1} const&>(bool tconcurrent::detail::shared<void ()>::shared<void tconcurrent::yielder::await_suspend<tconcurrent::task_promise<void> >(std::experimental::coroutines_v1::coroutine_handle<tconcurrent::task_promise<void> >)::{lambda()#1}>(std::__1::shared_ptr<tconcurrent::cancelation_token>, void tconcurrent::yielder::await_suspend<tconcurrent::task_promise<void> >(std::experimental::coroutines_v1::coroutine_handle<tconcurrent::task_promise<void> >)::{lambda()#1}&&, void*)::{lambda(tconcurrent::cancelation_token&, auto:1&&...)#1} const&&&...)"
);

demangles!(
    _Z1jI1AEDTcldtfp_cvPT_EES1_,
    // TODO: libiberty formats this as
    //
    //   decltype (({parm#1}.(operator A*))()) j<A>(A)
    "decltype (({parm#1}.operator A*)()) j<A>(A)"
);

demangles!(
    _Z3MinIiiEDTqultfp_fp0_cl7forwardIT_Efp_Ecl7forwardIT0_Efp0_EEOS0_OS1_,
    "decltype (({parm#1}<{parm#2})?((forward<int>)({parm#1})) : ((forward<int>)({parm#2}))) Min<int, int>(int&&, int&&)"
);

demangles!(
    _ZN16already_AddRefedIN7mozilla6detail16RunnableFunctionIZNS0_3ipc21AsyncMinidumpAnalyzer3RunEvEUlvE_EEEC4Ev,
    "already_AddRefed<mozilla::detail::RunnableFunction<mozilla::ipc::AsyncMinidumpAnalyzer::Run()::{lambda()#1}> >::already_AddRefed()"
);

// Test cases found via differential testing against `c++filt` with `cargo-fuzz`
// and `libFuzzer`.

demangles!(
    _Z5ccc_Z5cccmmmml,
    "ccc_Z(cccmm, unsigned long, unsigned long, long)"
);
demangles!(
    __Z3S_Z3SGffffjjjjjjjjjjzjjjjjjojjjjjjjj,
    "S_Z(SGf, float, float, float, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, ..., unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned __int128, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int)"
);
demangles!(
    __Z3SGfDdedddd,
    "SGf(decimal64, long double, double, double, double, double)"
);
demangles!(
    __ZN6ISiS_Z3b_dE1ES0_7__dIFFFdhl,
    "ISiS_Z::b_d(E, E, __dIFFF, double, unsigned char, long)"
);
demangles!(
    _ZN9__gnu_cxxmiIPKtPtNSt7__cxx1112basic_stringItN4base18string16_internals20string16_char_traitsESaItEEEEEDTmicldtfp_4baseEcldtfp0_4baseEERKNS_17__normal_iteratorIT_T1_EERKNSC_IT0_SE_EE,
    "decltype ((({parm#1}.base)())-(({parm#2}.base)())) __gnu_cxx::operator-<unsigned short const*, unsigned short*, std::__cxx11::basic_string<unsigned short, base::string16_internals::string16_char_traits, std::allocator<unsigned short> > >(__gnu_cxx::__normal_iterator<unsigned short const*, std::__cxx11::basic_string<unsigned short, base::string16_internals::string16_char_traits, std::allocator<unsigned short> > > const&, __gnu_cxx::__normal_iterator<unsigned short*, std::__cxx11::basic_string<unsigned short, base::string16_internals::string16_char_traits, std::allocator<unsigned short> > > const&)"
);
demangles!(
    _Z3addIidEDTplL_Z1gEfp0_ET_T0_,
    "decltype (g+{parm#2}) add<int, double>(int, double)"
);



Questa app non deve essere mostrata nelle applicazioni attive...
C:\Windows\SystemApps\MicrosoftWindows.Client.CBS_cw5n1h2txyewy\TextInputHost.exe

```bash
thread 'main' panicked at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tauri-2.8.4\src\menu\submenu.rs:199:72:
called `Result::unwrap()` on an `Err` value: OsError(Os { code: 0, kind: Uncategorized, message: "Operazione completata." })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'main' panicked at library\core\src\panicking.rs:225:5:
panic in a function that cannot unwind
stack backtrace:
   0:     0x7ff6e3cc8502 - std::backtrace_rs::backtrace::win64::trace
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\..\..\backtrace\src\backtrace\win64.rs:85
   1:     0x7ff6e3cc8502 - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff6e3cc8502 - std::sys::backtrace::_print_fmt
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\sys\backtrace.rs:66
   3:     0x7ff6e3cc8502 - std::sys::backtrace::impl$0::print::impl$0::fmt
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\sys\backtrace.rs:39
   4:     0x7ff6e3ceb06b - core::fmt::rt::Argument::fmt
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\fmt\rt.rs:173
   5:     0x7ff6e3ceb06b - core::fmt::write
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\fmt\mod.rs:1465
   6:     0x7ff6e3cc4107 - std::io::default_write_fmt
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\io\mod.rs:639
   7:     0x7ff6e3cc4107 - std::io::Write::write_fmt<std::sys::stdio::windows::Stderr>
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\io\mod.rs:1954
   8:     0x7ff6e3cc8345 - std::sys::backtrace::BacktraceLock::print
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\sys\backtrace.rs:42
   9:     0x7ff6e3cccd1f - std::panicking::default_hook::closure$0
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:300
  10:     0x7ff6e3cccab5 - std::panicking::default_hook
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:327
  11:     0x7ff6e3ccd8bd - std::panicking::rust_panic_with_hook
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:833
  12:     0x7ff6e3ccd632 - std::panicking::begin_panic_handler::closure$0
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:699
  13:     0x7ff6e3cc92bf - std::sys::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\sys\backtrace.rs:168
  14:     0x7ff6e3ccd27e - std::panicking::begin_panic_handler
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:697
  15:     0x7ff6e3d5a685 - core::panicking::panic_nounwind_fmt
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\intrinsics\mod.rs:2340
  16:     0x7ff6e3d5a733 - core::panicking::panic_nounwind
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\panicking.rs:225
  17:     0x7ff6e3d5a8c1 - core::panicking::panic_cannot_unwind
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\core\src\panicking.rs:330
  18:     0x7ff6e3a57226 - webview2_com_sys::Microsoft::Web::WebView2::Win32::impl$749::new::Invoke<webview2_com::callback::WebResourceRequestedEventHandler_Impl,-1>
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\webview2-com-sys-0.38.0\src\bindings.rs:37028
  19:     0x7ff9ba620430 - _CxxFrameHandler3
  20:     0x7ff9ba61342d - is_exception_typeof
  21:     0x7ff9dea41626 - RtlCaptureContext2
  22:     0x7ff6e3a571df - webview2_com_sys::Microsoft::Web::WebView2::Win32::impl$749::new::Invoke<webview2_com::callback::WebResourceRequestedEventHandler_Impl,-1>
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\webview2-com-sys-0.38.0\src\bindings.rs:37039
  23:     0x7ff99cd1c0cc - CreateWebViewEnvironmentWithOptionsInternal
  24:     0x7ff99cd1bf0f - CreateWebViewEnvironmentWithOptionsInternal
  25:     0x7ff99cd3632c - CreateWebViewEnvironmentWithOptionsInternal
  26:     0x7ff99cd8e90b - DllCanUnloadNow
  27:     0x7ff99cf56e4d - GetHandleVerifier
  28:     0x7ff99cd7a429 - DllCanUnloadNow
  29:     0x7ff99cd7bce9 - DllCanUnloadNow
  30:     0x7ff99cd7bc3b - DllCanUnloadNow
  31:     0x7ff99cd7bb32 - DllCanUnloadNow
  32:     0x7ff99d000441 - telemetry_client::IDataFieldVisitor::IDataFieldVisitor
  33:     0x7ff99d000749 - telemetry_client::IDataFieldVisitor::IDataFieldVisitor
  34:     0x7ff99cd7d8b3 - DllCanUnloadNow
  35:     0x7ff99cf88bd6 - telemetry_client::IDataFieldVisitor::~IDataFieldVisitor
  36:     0x7ff99ce4de32 - DllCanUnloadNow
  37:     0x7ff99ce65c75 - DllCanUnloadNow
  38:     0x7ff99ce4edd5 - DllCanUnloadNow
  39:     0x7ff99ce4cf8a - DllCanUnloadNow
  40:     0x7ff99ce65c75 - DllCanUnloadNow
  41:     0x7ff99ce35ce6 - DllCanUnloadNow
  42:     0x7ff99ce398ce - DllCanUnloadNow
  43:     0x7ff99ce1ec75 - DllCanUnloadNow
  44:     0x7ff99cd68e02 - DllCanUnloadNow
  45:     0x7ff99cd68c26 - DllCanUnloadNow
  46:     0x7ff99cd683dd - DllCanUnloadNow
  47:     0x7ff99cea4e78 - DllCanUnloadNow
  48:     0x7ff99cea4d8a - DllCanUnloadNow
  49:     0x7ff99cea49ef - DllCanUnloadNow
  50:     0x7ff9dc937846 - CallWindowProcW
  51:     0x7ff9dc93539d - IsWindowUnicode
  52:     0x7ff6e234611f - windows::Win32::UI::WindowsAndMessaging::DispatchMessageW
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\windows-0.61.3\src\Windows\Win32\UI\WindowsAndMessaging\mod.rs:577
  53:     0x7ff6e29301f1 - tao::platform_impl::platform::event_loop::EventLoop<enum2$<tauri_runtime_wry::Message<enum2$<tauri::EventLoopMessage> > > >::run_return<enum2$<tauri_runtime_wry::Message<enum2$<tauri::EventLoopMessage> > >,tauri_runtime_wry::make_event_handler::closure_env
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tao-0.34.2\src\platform_impl\windows\event_loop.rs:259
  54:     0x7ff6e29308db - tao::platform_impl::platform::event_loop::EventLoop<enum2$<tauri_runtime_wry::Message<enum2$<tauri::EventLoopMessage> > > >::run<enum2$<tauri_runtime_wry::Message<enum2$<tauri::EventLoopMessage> > >,tauri_runtime_wry::make_event_handler::closure_env$0<enum
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tao-0.34.2\src\platform_impl\windows\event_loop.rs:221
  55:     0x7ff6e2a173db - tao::event_loop::EventLoop<enum2$<tauri_runtime_wry::Message<enum2$<tauri::EventLoopMessage> > > >::run<enum2$<tauri_runtime_wry::Message<enum2$<tauri::EventLoopMessage> > >,tauri_runtime_wry::make_event_handler::closure_env$0<enum2$<tauri::EventLoopMessag
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tao-0.34.2\src\event_loop.rs:215
  56:     0x7ff6e267e611 - tauri_runtime_wry::impl$45::run<enum2$<tauri::EventLoopMessage>,tauri::app::impl$16::make_run_event_loop_callback::closure_env$0<tauri_runtime_wry::Wry<enum2$<tauri::EventLoopMessage> >,tauri::app::impl$19::run::closure_env$0<tauri_runtime_wry::Wry<enum2$<
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tauri-runtime-wry-2.8.1\src\lib.rs:3083
  57:     0x7ff6e237d505 - tauri::app::App<tauri_runtime_wry::Wry<enum2$<tauri::EventLoopMessage> > >::run<tauri_runtime_wry::Wry<enum2$<tauri::EventLoopMessage> >,tauri::app::impl$19::run::closure_env$0<tauri_runtime_wry::Wry<enum2$<tauri::EventLoopMessage> > > >
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tauri-2.8.4\src\app.rs:1249
  58:     0x7ff6e237d965 - tauri::app::Builder<tauri_runtime_wry::Wry<enum2$<tauri::EventLoopMessage> > >::run<tauri_runtime_wry::Wry<enum2$<tauri::EventLoopMessage> > >
                               at A:\Software\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tauri-2.8.4\src\app.rs:2301
  59:     0x7ff6e234434f - brickui_lib::run
                               at A:\GitHub\brickui\src-tauri\src\lib.rs:187
  60:     0x7ff6e2341119 - brickui::main
                               at A:\GitHub\brickui\src-tauri\src\main.rs:5
  61:     0x7ff6e23410fb - core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                               at A:\Software\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\function.rs:250
  62:     0x7ff6e234106e - core::hint::black_box
                               at A:\Software\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\hint.rs:482
  63:     0x7ff6e234106e - std::sys::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >       
                               at A:\Software\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\backtrace.rs:152
  64:     0x7ff6e2341051 - std::rt::lang_start::closure$0<tuple$<> >
                               at A:\Software\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\rt.rs:206
  65:     0x7ff6e3cbbe75 - std::rt::lang_start_internal::closure$0
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\rt.rs:175
  66:     0x7ff6e3cbbe75 - std::panicking::catch_unwind::do_call
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:589
  67:     0x7ff6e3cbbe75 - std::panicking::catch_unwind
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panicking.rs:552
  68:     0x7ff6e3cbbe75 - std::panic::catch_unwind
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\panic.rs:359
  69:     0x7ff6e3cbbe75 - std::rt::lang_start_internal
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library\std\src\rt.rs:171
                               at A:\Software\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\rt.rs:205
  71:     0x7ff6e2341139 - main
  72:     0x7ff6e3d5784c - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  73:     0x7ff6e3d5784c - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  74:     0x7ff9dd52e8d7 - BaseThreadInitThunk
  75:     0x7ff9de9ac53c - RtlUserThreadStart
thread caused non-unwinding panic. aborting.
error: process didn't exit successfully: `target\debug\brickui.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
```

Colore del logo:
- #cb4153ff

Ho modificato questo in package.json, sarebbe da rimettere per controlli piu' stringenti su typescript
```json
"build": "vue-tsc --noEmit && vite build"
```

```
"options": {
  "dir": "../src/isolation"
}
```
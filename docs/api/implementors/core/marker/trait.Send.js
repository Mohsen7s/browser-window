(function() {var implementors = {};
implementors["browser_window"] = [{"text":"impl Send for DelegateError","synthetic":true,"types":[]},{"text":"impl !Send for Application","synthetic":true,"types":[]},{"text":"impl !Send for ApplicationHandle","synthetic":true,"types":[]},{"text":"impl !Send for Runtime","synthetic":true,"types":[]},{"text":"impl Send for BrowserWindowBuilder","synthetic":true,"types":[]},{"text":"impl !Send for BrowserWindow","synthetic":true,"types":[]},{"text":"impl !Send for BrowserWindowThreaded","synthetic":true,"types":[]},{"text":"impl !Send for BrowserWindowHandle","synthetic":true,"types":[]},{"text":"impl Send for Source","synthetic":true,"types":[]},{"text":"impl&lt;'a, A&gt; Send for Event&lt;'a, A&gt;","synthetic":true,"types":[]},{"text":"impl Send for WindowBuilder","synthetic":true,"types":[]},{"text":"impl !Send for WindowHandle","synthetic":true,"types":[]},{"text":"impl !Send for WindowResizeEventArgs","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Send for ContentDimensions&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Send for Opacity&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Send for Position&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Send for Title&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; !Send for WindowDimensions&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Send for ApplicationHandleThreaded","synthetic":false,"types":[]},{"text":"impl&lt;'a, H, R&gt; Send for DelegateFuture&lt;'a, H, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Send,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, R&gt; Send for DelegateFutureFuture&lt;'a, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Send,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["browser_window_c"] = [{"text":"impl Send for cmax_align_t","synthetic":true,"types":[]},{"text":"impl !Send for cbw_StrSlice","synthetic":true,"types":[]},{"text":"impl !Send for cbw_CStrSlice","synthetic":true,"types":[]},{"text":"impl Send for cbw_ApplicationImpl","synthetic":true,"types":[]},{"text":"impl Send for cbw_ApplicationEngineImpl","synthetic":true,"types":[]},{"text":"impl Send for cbw_Application","synthetic":true,"types":[]},{"text":"impl Send for cbw_ApplicationEngineData","synthetic":true,"types":[]},{"text":"impl !Send for cbw_ApplicationDispatchData","synthetic":true,"types":[]},{"text":"impl !Send for cbw_ApplicationSettings","synthetic":true,"types":[]},{"text":"impl !Send for cbw_BrowserWindowImpl","synthetic":true,"types":[]},{"text":"impl Send for cdiv_t","synthetic":true,"types":[]},{"text":"impl Send for cldiv_t","synthetic":true,"types":[]},{"text":"impl Send for clldiv_t","synthetic":true,"types":[]},{"text":"impl Send for c__fsid_t","synthetic":true,"types":[]},{"text":"impl Send for c__sigset_t","synthetic":true,"types":[]},{"text":"impl Send for ctimeval","synthetic":true,"types":[]},{"text":"impl Send for ctimespec","synthetic":true,"types":[]},{"text":"impl Send for cfd_set","synthetic":true,"types":[]},{"text":"impl !Send for c__pthread_internal_list","synthetic":true,"types":[]},{"text":"impl !Send for c__pthread_internal_slist","synthetic":true,"types":[]},{"text":"impl !Send for c__pthread_mutex_s","synthetic":true,"types":[]},{"text":"impl Send for c__pthread_rwlock_arch_t","synthetic":true,"types":[]},{"text":"impl Send for c__pthread_cond_s","synthetic":true,"types":[]},{"text":"impl Send for c__pthread_cond_s__bindgen_ty_1__bindgen_ty_1","synthetic":true,"types":[]},{"text":"impl Send for c__pthread_cond_s__bindgen_ty_2__bindgen_ty_1","synthetic":true,"types":[]},{"text":"impl !Send for crandom_data","synthetic":true,"types":[]},{"text":"impl Send for cdrand48_data","synthetic":true,"types":[]},{"text":"impl !Send for c__locale_struct","synthetic":true,"types":[]},{"text":"impl !Send for cbw_Err","synthetic":true,"types":[]},{"text":"impl Send for c__mbstate_t","synthetic":true,"types":[]},{"text":"impl Send for c_G_fpos_t","synthetic":true,"types":[]},{"text":"impl Send for c_G_fpos64_t","synthetic":true,"types":[]},{"text":"impl Send for c_IO_marker","synthetic":true,"types":[]},{"text":"impl Send for c_IO_codecvt","synthetic":true,"types":[]},{"text":"impl Send for c_IO_wide_data","synthetic":true,"types":[]},{"text":"impl !Send for c_IO_FILE","synthetic":true,"types":[]},{"text":"impl Send for cbw_Dims2D","synthetic":true,"types":[]},{"text":"impl Send for cbw_Pos2D","synthetic":true,"types":[]},{"text":"impl Send for cbw_WindowImpl","synthetic":true,"types":[]},{"text":"impl Send for cbw_WindowCallbacks","synthetic":true,"types":[]},{"text":"impl Send for cbw_WindowOptions","synthetic":true,"types":[]},{"text":"impl Send for cbw_WindowDispatchData","synthetic":true,"types":[]},{"text":"impl !Send for cbw_Window","synthetic":true,"types":[]},{"text":"impl !Send for cbw_BrowserWindowOptions","synthetic":true,"types":[]},{"text":"impl !Send for cbw_BrowserWindowSource","synthetic":true,"types":[]},{"text":"impl !Send for cbw_BrowserWindow","synthetic":true,"types":[]},{"text":"impl Send for c__locale_data","synthetic":true,"types":[]},{"text":"impl !Send for c__va_list_tag","synthetic":true,"types":[]},{"text":"impl Send for c__pthread_cond_s__bindgen_ty_1","synthetic":true,"types":[]},{"text":"impl Send for c__pthread_cond_s__bindgen_ty_2","synthetic":true,"types":[]},{"text":"impl Send for cpthread_mutexattr_t","synthetic":true,"types":[]},{"text":"impl Send for cpthread_condattr_t","synthetic":true,"types":[]},{"text":"impl Send for cpthread_attr_t","synthetic":true,"types":[]},{"text":"impl !Send for cpthread_mutex_t","synthetic":true,"types":[]},{"text":"impl Send for cpthread_cond_t","synthetic":true,"types":[]},{"text":"impl Send for cpthread_rwlock_t","synthetic":true,"types":[]},{"text":"impl Send for cpthread_rwlockattr_t","synthetic":true,"types":[]},{"text":"impl Send for cpthread_barrier_t","synthetic":true,"types":[]},{"text":"impl Send for cpthread_barrierattr_t","synthetic":true,"types":[]},{"text":"impl Send for c__mbstate_t__bindgen_ty_1","synthetic":true,"types":[]}];
implementors["browser_window_core"] = [{"text":"impl Send for ApplicationSettings","synthetic":true,"types":[]},{"text":"impl !Send for ApplicationImpl","synthetic":true,"types":[]},{"text":"impl !Send for BrowserWindowImpl","synthetic":true,"types":[]},{"text":"impl Send for JsEvaluationError","synthetic":true,"types":[]},{"text":"impl !Send for WindowImpl","synthetic":true,"types":[]}];
implementors["futures_channel"] = [{"text":"impl&lt;T&gt; Send for Sender&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for UnboundedSender&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Receiver&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for UnboundedReceiver&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for SendError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for TrySendError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for TryRecvError","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Receiver&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Sender&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Send for Cancellation&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Canceled","synthetic":true,"types":[]}];
implementors["unsafe_send_sync"] = [{"text":"impl&lt;T&gt; Send for UnsafeSync&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for UnsafeSend&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Send for UnsafeSendSync&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
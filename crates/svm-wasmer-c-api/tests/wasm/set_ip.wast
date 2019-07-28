(module
  ;; import `svm` vmcalls
  (func $svm_set_ip (import "node" "set_ip")  (param i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "set_ip_proxy") (param i32)
        get_local 0 ;; `ip`
        call $svm_set_ip))

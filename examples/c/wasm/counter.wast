(module
  (func $storage_write_i32_le (import "svm" "storage_write_i32_le") (param i32 i32 i32 i32))
  (func $storage_read_i32_le (import "svm" "storage_read_i32_le") (param i32 i32 i32) (result i32))
  (func $env_inc (import "env" "inc") (param i32))
  (func $env_get (import "env" "get") (result i32))

  (memory 1) ;; memory `0` (default) is initialized with one page

  (func (export "storage_inc") (param $val i32)
        i32.const 0     ;; page_idx
        i32.const 0     ;; page_offset

        call $storage_get
        get_local $val
        i32.add         ;; n

        i32.const 4     ;; nbytes
        call $storage_write_i32_le
  )

  (func $storage_get (export "storage_get") (result i32)
        i32.const 0  ;; page_idx
        i32.const 0  ;; page_offset
        i32.const 4  ;; count
        call $storage_read_i32_le
  )

  (func (export "host_inc") (param $val i32)
        get_local $val 
        call $env_inc
  )

  (func (export "host_get") (result i32)
        call $env_get
  )
)

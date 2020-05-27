(module
  (func $store160 (import "svm" "store160") (param $mem_idx i32) (param $mem_ptr i32) (param $var_id i32))
  (func $load160 (import "svm" "load160") (param $var_id i32) (param $mem_idx i32) (param $mem_ptr i32))

  (import "svm" "memory" (memory 1))

  (func (export "ctor")
  	nop)

  (func (export "store_addr") (param $var_id i32) (param $mem_ptr i32)
  	i32.const 0 ;; mem_idx
	get_local $mem_ptr 
  	get_local $var_id 
	call $store160)

  (func (export "edit_addr") (param $func_buf_size i32) (param $var_id i32) 
	(local $i i32) 

	;; we load var `var_id` from storage starting from memory address `func_buf_size`.
	;; since the `func_buf` consumes memory range: `[0...func_buf_size)`
	;; we know that loading var `var_id` won't collide with `func_buf`.

  	get_local $var_id        ;; var_id
  	i32.const 0              ;; mem_idx
  	get_local $func_buf_size ;; mem_ptr
	call $load160

        ;; now we'll start editing var `var_id`

	;; i <- 0
        (set_local $i (i32.const 0))

	(block 
	    (loop 
		;; func_buf_size + i
		get_local $func_buf_size
		get_local $i
		i32.add

		;; byte <- func_buf[i] 
		;; (i.e reading from `mem[i]` since `func_buf` starts at `address=0`)
		;;
		get_local $i
		i32.load8_u

		;; Stack State:
		;;
		;; top:
		;; +-------------------+
		;; |   func_buf[i]     |
		;; +-------------------+
		;; | func_buf_size + i |
		;; +-------------------+
		;;

		;; mem[func_buf_size + i] <- func_buf[i]
		i32.store8

		;; i <- i + 1
		get_local $i
		i32.const 1
		i32.add
		set_local $i

		;; if `i == func_buf_size` then
		;;   break from the loop
		;; else
		;;   jump to the start of the loop
		;;
		(br_if 1 (i32.eq (get_local $i) (get_local $func_buf_size)))
		(br 0)
	    )
	)

        ;; now we store the in-memory edited address back to var `var_id`

	i32.const  0             ;; mem_idx
	get_local $func_buf_size ;; mem_ptr
	get_local $var_id    	 ;; var_id
	call $store160))
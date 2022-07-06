(module
  (func $log (import "imports" "log") (param i32))
  (func (export "log_42")
    i32.const 42
    call $log
  )
)

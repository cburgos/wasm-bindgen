(module $reference_test.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $enum_echo (;0;) (type 0) (param i32) (result i32))
  (func $option_enum_echo (;1;) (type 0) (param i32) (result i32))
  (func $get_name (;2;) (type 0) (param i32) (result i32))
  (func $option_string_enum_echo (;3;) (type 0) (param i32) (result i32))
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "enum_echo" (func $enum_echo))
  (export "option_enum_echo" (func $option_enum_echo))
  (export "get_name" (func $get_name))
  (export "option_string_enum_echo" (func $option_string_enum_echo))
  (@custom "target_features" (after code) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

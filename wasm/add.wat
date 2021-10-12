(module
    (func $add (param $v1 i32) (param $v2 i32) (result i32)
        local.get $v1
        local.get $v2
        i32.add)
    (export "add" (func $add))
)
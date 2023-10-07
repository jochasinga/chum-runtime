
#[cfg(test)]
pub mod tests {
    use wasmtime::*;
    use wasmtime_wasi::{WasiCtxBuilder, WasiCtx};
    use anyhow::Result;

    fn init_test() -> Result<(Engine, Linker<WasiCtx>, Module, Module, WasiCtx)> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        Ok((engine, linker, tests_module, asm_module, wasi))
    }

    #[test]
    pub fn test_sete() -> Result<()> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        // let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();
        let mut store = Store::new(&engine, wasi);
        let asm = linker.instantiate(&mut store, &asm_module)?;
        let sete = asm.get_typed_func::<(i32, i32), i32>(&mut store, "sete")?;
        let zf: i32 = 1;
        let v: i32 = 20;
        let val: i32 = sete.call(&mut store, (zf, v))?;
        let expected: i32 = 0xff;
        assert_eq!(expected, val);
        Ok(())
    }

    #[test]
    pub fn test_sall() -> Result<()> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        // let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();
        let mut store = Store::new(&engine, wasi);
        let asm = linker.instantiate(&mut store, &asm_module)?;
        let sall = asm.get_typed_func::<(i32, i32), i32>(&mut store, "sall")?;
        let num: i32 = 4;
        let shift: i32 = 8;
        let val: i32 = sall.call(&mut store, (num, shift))?;
        let expected: i32 = 4 << 8;
        assert_eq!(expected, val);
        Ok(())
    }

    #[test]
    pub fn test_cmpl() -> Result<()> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        // let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();
        let mut store = Store::new(&engine, wasi);
        let asm = linker.instantiate(&mut store, &asm_module)?;
        let cmpl = asm.get_typed_func::<(i32, i32), i32>(&mut store, "cmpl")?;
        let a: i32 = 4;
        let b: i32 = 8;
        let val: i32 = cmpl.call(&mut store, (a, b))?;
        let expected: i32 = -1;
        assert_eq!(expected, val);
        let val: i32 = cmpl.call(&mut store, (a, a))?;
        let expected: i32 = 0;
        assert_eq!(expected, val);
        let val: i32 = cmpl.call(&mut store, (b, a))?;
        let expected: i32 = 1;
        assert_eq!(expected, val);
        Ok(())
    }

    #[test]
    pub fn test_linking_sete_eq() -> Result<()> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        let mut store = Store::new(&engine, wasi);
        let asm = linker.instantiate(&mut store, &asm_module)?;
        linker.instance(&mut store, "asm_x86", asm)?;
        let tests = linker.instantiate(&mut store, &tests_module)?;
        let test_sete_eq = tests.get_typed_func::<(), i32>(&mut store, "test_sete_eq")?;
        let expected: i32 = 255;
        let val: i32 = test_sete_eq.call(&mut store, ())?;
        assert_eq!(expected, val);
        Ok(())
    }

    #[test]
    pub fn test_linking_sete_ne() -> Result<()> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        let mut store = Store::new(&engine, wasi);
        let asm = linker.instantiate(&mut store, &asm_module)?;
        linker.instance(&mut store, "asm_x86", asm)?;
        let tests = linker.instantiate(&mut store, &tests_module)?;
        let test_sete_ne = tests.get_typed_func::<(), i32>(&mut store, "test_sete_ne")?;
        let expected: i32 = 99;
        let val: i32 = test_sete_ne.call(&mut store, ())?;
        assert_eq!(expected, val);
        Ok(())
    }

    #[test]
    pub fn test_linking_sall() -> Result<()> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        let mut store = Store::new(&engine, wasi);
        let asm = linker.instantiate(&mut store, &asm_module)?;
        linker.instance(&mut store, "asm_x86", asm)?;
        let tests = linker.instantiate(&mut store, &tests_module)?;
        let test_sall = tests.get_typed_func::<(), i32>(&mut store, "test_sall")?;
        let expected: i32 = 4 << 8;
        let val: i32 = test_sall.call(&mut store, ())?;
        assert_eq!(expected, val);
        Ok(())
    }

    #[test]
    pub fn test_emit_is_equal_to() -> Result<()> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        let tests_module = Module::from_file(&engine, "../modules/lib/tests.wat")?;
        let asm_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        let mut store = Store::new(&engine, wasi);
        let asm = linker.instantiate(&mut store, &asm_module)?;
        linker.instance(&mut store, "asm_x86", asm)?;
        let tests = linker.instantiate(&mut store, &tests_module)?;
        let test_emit_is_equal_to = tests.get_typed_func::<(), i32>(&mut store, "test_emit_is_equal_to")?;
        let expected: i32 = 1; // true
        let val: i32 = test_emit_is_equal_to.call(&mut store, ())?;

        print!("val: {}", val);
        // let bool_shift = 8;
        // let bool_tag = 15;
        // assert_eq!(expected, (val << bool_shift) & bool_tag);
        Ok(())
    }
}
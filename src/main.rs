#![allow(dead_code)]

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;


const FIXNUM_MASK: i32 = 3;
const FIXNUM_TAG: i32 = 0;
const FIXNUM_SHIFT: i32 = 2;

const CHAR_MASK: i32 = 0xff;
const CHAR_SHIFT: i32 = 8;
const CHAR_TAG: i32 = 7;

const BOOL_MASK: i32 = 0xff;
const BOOL_SHIFT: i32 = 8;
const BOOL_TAG: i32 = 15;

const PTR_MASK: i32 = 7;
const PAIR_TAG: i32 = 1;
const VEC_TAG: i32 = 2;
const STR_TAG: i32 = 3;
const SYM_TAG: i32 = 5;
const CLOSURE_TAG: i32 = 6;

fn main() -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    let prog_module = Module::from_file(&engine, "../modules/compiled.wat")?;
    let lib_module = Module::from_file(&engine, "../modules/lib/asm_x86.wat")?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);
    let lib = linker.instantiate(&mut store, &lib_module)?;
    linker.instance(&mut store, "asm_x86", lib)?;
    let prog = linker.instantiate(&mut store, &prog_module)?;
    let scheme_entry = prog.get_typed_func::<(), i32>(&mut store, "scheme_entry")?;
    let val: i32 = scheme_entry.call(&mut store, ())?;
    show(val);
    print!("\n");

    Ok(())
}

fn show(x: i32) {
    if x & FIXNUM_MASK == FIXNUM_TAG {
        // integer
        print!("{}", x >> FIXNUM_SHIFT);
    } else if x & CHAR_MASK == CHAR_TAG {
        // character
        print!("#\\{}", (x >> CHAR_SHIFT) as u8 as char);
    } else if x & BOOL_MASK == BOOL_TAG {
        if x >> BOOL_SHIFT != 0 {
            print!("#t");
        } else {
            print!("#f");
        }
    }
}

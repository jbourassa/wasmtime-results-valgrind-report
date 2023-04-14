use wasmtime::{AsContextMut, Engine, Func, Store};

const RESULTS_COUNT: i32 = 200;
fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let result_type: Vec<_> = (0..RESULTS_COUNT).map(|_| wasmtime::ValType::I32).collect();
    let func_type = wasmtime::FuncType::new([], result_type);

    let func = Func::new(&mut store, func_type.clone(), |_, _, results| {
        results
            .iter_mut()
            .for_each(|val| *val = wasmtime::Val::I32(1));

        Ok(())
    });

    let mut results = vec![wasmtime::Val::null(); func_type.results().len()];
    func.call(store.as_context_mut(), &[], &mut results)?;

    Ok(())
}

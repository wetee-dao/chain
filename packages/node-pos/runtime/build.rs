fn main() {
	#[cfg(feature = "std")]
    #[cfg(not(target_os = "windows"))]
	{
		substrate_wasm_builder::WasmBuilder::new()
			.with_current_project()
			.export_heap_base()
			.import_memory()
			.build();
	}
}

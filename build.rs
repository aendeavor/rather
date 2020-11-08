fn main()
{
	println!("cargo:rerun-if-changed=binding.c");

	cc::Build::new()
		.file("components/bindings.c")
		.file("components/debug.c")
		.compile("bindings");
}

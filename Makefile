all: hello hello-ownership hello-pattern hello-enum trait generics

hello: hello.rs
	rustc $<

hello-ownership: hello-ownership.rs
	rustc $<

hello-pattern: hello-pattern.rs
	rustc $<

hello-enum: hello-enum.rs
	rustc $<

trait: trait.rs
	rustc $<

generics: generics.rs
	rustc $<

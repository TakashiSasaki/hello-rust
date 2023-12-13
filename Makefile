all: hello hello-ownership hello-pattern hello-enum trait generics vec-macro vec-new mymacro future nostd

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

vec-macro: vec-macro.rs
	rustc $<

vec-new: vec-new.rs
	rustc $<

mymacro: mymacro.rs
	rustc $<

future: future.rs
	rustc $<

nostd: nostd.rs 
	rustc -C panic=abort -C link-args="-e _start -static -nostartfiles" $<


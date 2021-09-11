all:
	cargo run ./examples/working.tictac
	# as -msyntax=intel -mnaked-reg

clean:
	cargo clean

cleano:
	rm -r ./examples/*.lex.cache
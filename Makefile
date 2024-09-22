SRC=$(find src -name '*.rs')

run: $(SRC)
	@ cargo run
.PHONY: all
all: build build-tests build-tests-nll

.PHONY: build
build:
	cd charon && cargo build

SRC = $(TESTS)/src
OPTIONS = --dest $(TESTS)/cfim

.PHONY: build-tests
build-tests:
	cd tests && make

.PHONY: build-tests-nll
build-tests-nll:
	cd tests-nll && make

.PHONY: tests
tests: build-tests build-tests-nll \
	test-nested_borrows test-no_nested_borrows test-loops test-hashmap test-paper \
	test-matches test-matches_duplicate test-external \
	test-nll-betree_nll test-nll-betree_main

test-nested_borrows: OPTIONS += --no-code-duplication
test-no_nested_borrows: OPTIONS += --no-code-duplication
test-loops: OPTIONS += --no-code-duplication
test-hashmap: OPTIONS += --no-code-duplication
test-paper: OPTIONS += --no-code-duplication
test-matches: OPTIONS += --no-code-duplication
test-external: OPTIONS += --no-code-duplication
test-matches_duplicate:
test-nll-betree_nll: OPTIONS += --no-code-duplication
test-nll-betree_main: OPTIONS += --no-code-duplication

.PHONY: test-%
test-%: TESTS=../tests
test-%:
	cd charon && cargo run $(SRC)/$*.rs $(OPTIONS)

.PHONY: test-nll-%
test-nll-%: TESTS=../tests-nll
test-nll-%: OPTIONS += --nll
test-nll-%:
	cd charon && cargo run $(SRC)/$*.rs $(OPTIONS)

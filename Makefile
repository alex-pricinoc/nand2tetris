COMPLETED :=  1 2 3 4 7
HDL       := $(shell git ls-files -co $(patsubst %,projects/%/*.hdl, $(COMPLETED)))
ASM       := $(shell git ls-files -co $(patsubst %,projects/%/*.asm, $(COMPLETED)))
VM        := $(shell git ls-files -co $(patsubst %,projects/%/*.vm, $(COMPLETED)))

TESTED    := $(HDL:.hdl=.hdl.TESTED) $(ASM:.asm=.asm.TESTED) $(VM:.vm=.vm.TESTED)

test: $(TESTED)
	@echo "All tests passed."

%.asm: %.vm
	cargo run -q --manifest-path vm-to-asm/Cargo.toml $<

%.vm.TESTED: %.tst %.asm
	if ./tools/CPUEmulator.sh $<; then touch $@; else exit 1; fi

%.hdl.TESTED: %.tst
	if ./tools/HardwareSimulator.sh $<; then touch $@; else exit 1; fi

%.asm.TESTED: %.tst
	if ./tools/CPUEmulator.sh $<; then touch $@; else exit 1; fi

clean:
	rm $(TESTED)

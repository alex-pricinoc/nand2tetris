COMPLETED := 1 2 3 4 7 10
HDL       := $(shell git ls-files -co $(patsubst %,projects/%/*.hdl, $(COMPLETED)))
ASM       := $(shell git ls-files -co $(patsubst %,projects/%/*.asm, $(COMPLETED)))
VM        := $(shell git ls-files -co $(patsubst %,projects/%/*.vm, $(COMPLETED)))
JACK      := $(shell git ls-files -co $(patsubst %,projects/%/*.jack, $(COMPLETED)))

TESTED    := $(HDL:.hdl=.hdl.TESTED) $(ASM:.asm=.asm.TESTED) $(VM:.vm=.asm.TESTED) $(JACK:.jack=.xml.TESTED) \
	projects/8/ProgramFlow/BasicLoop/BasicLoop.asm.TESTED \
	projects/8/ProgramFlow/FibonacciSeries/FibonacciSeries.asm.TESTED \
	projects/8/FunctionCalls/SimpleFunction/SimpleFunction.asm.TESTED \
	projects/8/FunctionCalls/NestedCall/NestedCall.asm.TESTED \
	projects/8/FunctionCalls/FibonacciElement/FibonacciElement.asm.TESTED \
	projects/8/FunctionCalls/StaticsTest/StaticsTest.asm.TESTED

VM-TO-ASM   := cargo run -q --manifest-path vm-to-asm/Cargo.toml
JACK-TO-XML := cargo run -q --manifest-path compiler/Cargo.toml --bin parser

test: $(TESTED)
	@echo "All tests passed."

%NestedCall.asm: %Sys.vm
	@$(VM-TO-ASM) $^ $@

%FibonacciElement.asm: %Sys.vm %Main.vm
	@$(VM-TO-ASM) $^ $@

%StaticsTest.asm: %Sys.vm %Class1.vm %Class2.vm
	@$(VM-TO-ASM) $^ $@

%.asm: %.vm
	@$(VM-TO-ASM) $^ $@

%.asm.TESTED: %.tst %.asm
	@if ./tools/CPUEmulator.sh $<; then touch $@; else exit 1; fi

%.hdl.TESTED: %.tst
	@if ./tools/HardwareSimulator.sh $<; then touch $@; else exit 1; fi

%.C.xml: %.jack
	@$(JACK-TO-XML) $^

%.xml.TESTED: %.xml %.C.xml
	@if diff $^; then touch $@; else exit 1; fi

clean:
	@rm $(TESTED)

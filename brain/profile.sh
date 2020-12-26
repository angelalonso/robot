# Create the benchmark executable with debugging symbols, but do not run it. We
# don't want valgrind to profile the compiler, so we have the "--no-run" flag. We
# also need debugging symbols so valgrind can track down source code
# appropriately. It blows my mind to this day that compiling with optimizations +
# debugging symbols is a thing. For so long I thought they were mutually
# exclusive.
RUSTFLAGS="-g" cargo bench  --no-run

# Now find the created benchmark executable. I tend to prefix my benchmark
# names with 'bench' to easily identify them
ls -lhtr ./target/release

# Let's say this was the executable
BENCH="./target/release/brain"

# Now identify a single test that you want profiled. Test identifiers are
# printed in the console output, so I'll use the one that I posted earlier
T_ID="benches/my_benchmark.rs"

# Have valgrind profile criterion running our benchmark for 10 seconds
valgrind --tool=callgrind \
         --dump-instr=yes \
         --collect-jumps=yes \
         --simulate-cache=yes \
         $BENCH --bench --profile-time 10 $T_ID

# valgrind outputs a callgrind.out.<pid>. We can analyze this with kcachegrind
kcachegrind

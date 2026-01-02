# `vroom`

practicing profiling read and parse strategies in rust

## Rules

 * No `println` statements, use [GDB](https://web.archive.org/web/20140831121704/http://dirac.org/linux/gdb/02a-Memory_Layout_And_The_Stack.php) to debug
 * Use `perf` to guide development
 * Note improvements in `CHANGELOG` (`perf` output comparisons, bottlenecks).
 * No clankers

## Set Up

 * See [1brc challenge](https://github.com/gunnarmorling/1brc/tree/main?tab=readme-ov-file#running-the-challenge)
 * I used `src/main/python/create_measurements.py` to generate my `measurements.txt` 

---
 
# strategies

1. Read entire file into memory by allocating `String` - no.

2. `mmap`, reference kernel RAM with pointers
 - page faults are when you reference address not allocated in kernel yet
 - interrupts everything
 - "read ahead" at OS-level intelligently sees after a couple page faults you're trying to sequentially read a big file, will allocate in 128KB or greater chunks.
 - [Madvise](https://man7.org/linux/man-pages/man2/madvise.2.html) syscall handles this 
 
3. Buffering, amortize system calls with an allocation in program space
- Pages are 4KB blocks, buffers double that to 8KB.
- Parser may need one character at a time. That could be a SysCall per character. Yikes.
- Don't go all the way to the lake with your measuring cup. Go to your bucket of water at the campsite.

---

# `perf`

Add following to `Cargo.toml` to get source code info
```
[profile.release]
debug = true
```

Record and report:
- Struggled to get `perf record` variants to work, only `cargo flamegraph` gives me the line numbers to `perf report` on.
```
cargo flamegraph --bin vroom
perf report  -g -F+srcline
```

But then this works, too, but just takes a while and produces 3G of write data:
```
perf record --call-graph dwarf cargo run --relea
perf report
```
- I like just starting with `main` and expanding down.


### Initial look at Naive approach:
```
  Children      Self  Command          Shared Object         Symbol 
-   73.24%    26.78%  vroom            vroom                 [.] vroom::main                                                                                                                                      ◆
   - 46.45% vroom::main                                                                                                                                                                                           ▒
      - 24.80% <std::io::Lines<B> as core::iter::traits::iterator::Iterator>::next                                                                                                                                ▒
         - 24.11% std::io::BufRead::read_line (inlined)                                                                                                                                                           ▒
            + std::io::append_to_string                                                                                                                                                                           ▒
      + 9.65% core::str::<impl str>::parse (inlined)                                                                                                                                                              ▒
      + 7.37% std::collections::hash::map::HashMap<K,V,S>::get_mut (inlined)                                                                                                                                      ▒
      + 4.64% core::str::<impl str>::split_once (inlined)    
``` 


### After MMaping the file

---

# Further Reading (`libc` and system calls documentation)

[memcmp](https://www.man7.org/linux/man-pages/man3/memcmp.3.html) compares first _N_ bytes of memory areas s1 and s2
```
int memcmp(size_t n; const void s1[n], const void s2[n], size_t n);
```

[memchr](https://man7.org/linux/man-pages/man3/memchr.3.html) scans initial _N_ bytes of memory area for first instance of _C_
```
void *memchr(size_t n; const void s[n], int c, size_t n);
```


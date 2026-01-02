## `vroom`

practicing profiling read and parse strategies in rust

#### Rules

 * No `println` statements, use [GDB](https://web.archive.org/web/20140831121704/http://dirac.org/linux/gdb/02a-Memory_Layout_And_The_Stack.php) to debug
 * Use `perf` to guide development
 * Note improvements in `CHANGELOG` (`perf` output comparisons, bottlenecks).
 * No clankers

#### Set Up

 * See [1brc challenge](https://github.com/gunnarmorling/1brc/tree/main?tab=readme-ov-file#running-the-challenge)
 * I used `src/main/python/create_measurements.py` to generate my `measurements.txt` 

#### Strategies

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

## Progress Not `perf`ection

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


#### Initial look at Naive approach:
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

#### After MMaping the file

```
  Children      Self  Command          Shared Object         Symbol                                                                                                                                                
-   81.09%    46.70%  vroom            vroom                 [.] vroom::main                                                                                                                                      ▒
   - 45.69% __libc_start_call_main                                                                                                                                                                                ▒
        main                                                                                                                                                                                                      ◆
        std::rt::lang_start_internal                                                                                                                                                                              ▒
        std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::haef630ddb9f1a0e9 (inlined)                                                                                                                            ▒
        std::sys::backtrace::__rust_begin_short_backtrace                                                                                                                                                         ▒
        core::ops::function::FnOnce::call_once (inlined)                                                                                                                                                          ▒
      - vroom::main                                                                                                                                                                                               ▒
         - 21.22% <core::slice::iter::Split<T,P> as core::iter::traits::iterator::Iterator>::next (inlined)                                                                                                       ▒
            - 16.58% <core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::position (inlined)                                                                                                   ▒
               + 6.63% <core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::next (inlined)                                                                                                     ▒
               + 2.30% _$LT$core..slice..iter..Split$LT$T$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::_$u7b$$u7b$closure$u7d$$u7d$::h88b334380de6653f (inlined)                          ▒
               + 1.37% _$LT$core..slice..iter..Split$LT$T$C$P$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::_$u7b$$u7b$closure$u7d$$u7d$::h71998a8d249b8dc6 (inlined)                          ▒
            + 2.94% core::slice::<impl [T]>::get_unchecked (inlined)                                                                                                                                              ▒
         - 21.01% std::collections::hash::map::HashMap<K,V,S>::get_mut (inlined)                                                                                                                                  ▒
            + hashbrown::map::HashMap<K,V,S,A>::get_mut (inlined)                                                                                                                                                 ▒
           0.83% core::f32::<impl f32>::min (inlined)                                                                                                                                                             ▒
           0.70% core::f32::<impl f32>::max (inlined)                                                                                                                                                             ▒
   - 34.40% vroom::main                                                                                                                                                                                           ▒
      - 19.32% std::collections::hash::map::HashMap<K,V,S>::get_mut (inlined)                                                                                                                                     ▒
           hashbrown::map::HashMap<K,V,S,A>::get_mut (inlined)                                                                                                                                                    ▒
         + hashbrown::map::HashMap<K,V,S,A>::get_inner_mut (inlined)                                                                                                                                              ▒
      - 14.55% core::str::<impl str>::parse (inlined)                                                                                                                                                             ▒
         + core::num::dec2flt::<
```

---

## Further Reading 

`libc` and system calls documentation

- [memcmp](https://www.man7.org/linux/man-pages/man3/memcmp.3.html) compares first _N_ bytes of memory areas s1 and s2
```
int memcmp(size_t n; const void s1[n], const void s2[n], size_t n);
```

- [memchr](https://man7.org/linux/man-pages/man3/memchr.3.html) scans initial _N_ bytes of memory area for first instance of _C_
```
void *memchr(size_t n; const void s[n], int c, size_t n);
```


1. Read entire file into memory by allocating `String`

2. `mmap`, reference kernel RAM with pointers
 - page faults are when you reference address not allocated in kernel yet
 - interrupts everything
 - "read ahead" at OS-level intelligently sees after a couple page faults you're trying to sequentially read a big file, will allocate in 128KB or greater chunks.
 
3. Buffering, amortize system calls with an allocation in program space
- Pages are 4KB blocks, buffers double that to 8KB.
- Parser may need one character at a time. That could be a SysCall per character. Yikes.
- Don't go all the way to the lake with your measuring cup. Go to your bucket of water at the campsite.

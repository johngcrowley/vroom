
### 1. Basic timing (start here):                                                                                                                                                                   
                                                                                                                                                                                                 
```[rust]

 use std::time::Instant;                                                                                                                                                                         
 let start = Instant::now();                                                                                                                                                                     
 println!("Took: {:?}", start.elapsed());                                                                                                                                                        
```
                                                                                                                                                                                                 
### 2. `hyperfine` (compare approaches):                                                                                                                                                              
                                                                                                                                                                                                
Gives you nice statistical comparison with warmup runs.                                                                                                                                         
```[rust]
cargo install hyperfine                                                                                                                                                                         
hyperfine './mmap_version' './string_version'                                                                                                                                                   
```
                                                                                                                                                                                                
### 3a. `perf` (see what's slow):                                                                                                         
                                                                                                                                   
```
perf record -g ./your_binary input.txt                                                                                             
perf report                                                                                                                        
```
                                                                                                                                   
### 3b. `flamegraph` (visual)                                                                                                       
Opens an interactive SVG showing where time is spent.                                                                              
```
cargo install flamegraph                                                                                                           
cargo flamegraph --bin your_binary -- input.txt                                                                                    
```

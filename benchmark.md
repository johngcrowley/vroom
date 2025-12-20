1. Basic timing (start here):                                                                                                                                                                   
                                                                                                                                                                                                 
 use std::time::Instant;                                                                                                                                                                         
                                                                                                                                                                                                 
 let start = Instant::now();                                                                                                                                                                     
 // your code here                                                                                                                                                                               
 println!("Took: {:?}", start.elapsed());                                                                                                                                                        
                                                                                                                                                                                                 
2. Hyperfine (compare approaches):                                                                                                                                                              
                                                                                                                                                                                                
# Install                                                                                                                                                                                       
cargo install hyperfine                                                                                                                                                                         
                                                                                                                                                                                                
# Compare two binaries                                                                                                                                                                          
hyperfine './mmap_version' './string_version'                                                                                                                                                   
Gives you nice statistical comparison with warmup runs.                                                                                                                                         
                                                                                                                                                                                                
3. perf (see what's slow):                                                                                                         
                                                                                                                                   
# Record                                                                                                                           
perf record -g ./your_binary input.txt                                                                                             
                                                                                                                                   
# View report                                                                                                                      
perf report                                                                                                                        
                                                                                                                                   
# Or get flamegraph (visual)                                                                                                       
cargo install flamegraph                                                                                                           
cargo flamegraph --bin your_binary -- input.txt                                                                                    
Opens an interactive SVG showing where time is spent.                                                                              

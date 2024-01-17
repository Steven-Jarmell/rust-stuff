Compile rust programs with `rustc {filename}`  

`rustc` generates a .exe file for the application, and a .pdb file with debugging information  

Rust is an **ahead of time** compiled language, meaning you can compile a program and give the executable to someone else, and they can run it without even having Rust installed  

Bigger projects in Rust use Cargo, which is Rust's build system and package manager  
- Handles tasks like building code, downloading and building dependencies
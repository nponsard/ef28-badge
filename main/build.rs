fn main() {

    println!("cargo:rerun-if-changed=../coprocessor/target/riscv32imc-unknown-none-elf/release/coprocessor");
}

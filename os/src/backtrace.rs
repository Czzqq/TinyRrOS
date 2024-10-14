use crate::println;
use crate::symbol::SYMBOL_TABLE;
pub fn print_symbols() {
    for symbol in &SYMBOL_TABLE {
        println!("Address: {:016x}, Name: {}, Size: {}", symbol.address, symbol.name, symbol.size);
    }
}

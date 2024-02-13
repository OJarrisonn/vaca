use rustc_hash::FxHashMap;

#[derive(Debug)]
/// The `ExternalTable` is a struct that any Vaca Native Library must expose using the `export_table` function.
/// 
/// This table is later collected by the VacaVM, calling the `export_table` function for every included dynamic library.
pub struct ExternalTable<T> {
    table: FxHashMap<String, T>
}

impl<T> ExternalTable<T> {
    /// Creates an empty `ExternalTable`
    pub fn new() -> Self {
        Self { table: FxHashMap::default() }
    }

    /// Used by the library creator to expose a symbol to Vaca.
    /// 
    /// This method returns Err(()) if the symbol is already defined
    pub fn export_symbol(&mut self, symbol: &str, value: T) -> Result<(), ()> {
        let key = symbol.to_string();

        if self.table.contains_key(&key) {
            Err(())
        }  else {
            self.table.insert(key, value);

            Ok(())
        }
    }

    /// Used by the VM to retrieve a runnable from this table
    pub fn get(&self, name: &str) -> Option<&T> {
        self.table
            .get(&name.to_string())
    }
}
pub struct ImportPriceResolver {
    prices: HashMap<(&'static str, &'static str), Gas>,
}

impl ImportPriceResolver {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            prices: HashMap::with_capacity(capacity),
        }
    }

    pub fn set_price(&mut self, module: &'static str, name: &'static str, price: Gas) {
        self.prices.insert((module, name), price);
    }

    pub fn price_for(&self, module: &str, name: &str) -> Gas {
        *self.prices.get(&(module, name)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn price_for(imports: &Imports, resolver: &ImportPriceResolver, func: FuncIndex) -> Gas {
        let (module, name) = imports.get_import(func);

        resolver.price_for(module, name)
    }

    #[test]
    fn test_price_for() {
        let mut imports = Imports::with_capacity(4);

        let f0 = FuncIndex(0);
        let f1 = FuncIndex(1);
        let f2 = FuncIndex(2);
        let f3 = FuncIndex(3);

        imports.add_import("env", "foo", f0);
        imports.add_import("env", "bar", f1);
        imports.add_import("host", "bar", f2);
        imports.add_import("host", "baz", f3);

        let mut resolver = ImportPriceResolver::with_capacity(4);

        resolver.set_price("env", "foo", Gas::Fixed(10));
        resolver.set_price("env", "bar", Gas::Fixed(20));
        resolver.set_price("host", "bar", Gas::Fixed(30));
        resolver.set_price("host", "baz", Gas::Fixed(40));

        assert_eq!(price_for(&imports, &resolver, f0), Gas::Fixed(10));
        assert_eq!(price_for(&imports, &resolver, f1), Gas::Fixed(20));
        assert_eq!(price_for(&imports, &resolver, f2), Gas::Fixed(30));
        assert_eq!(price_for(&imports, &resolver, f3), Gas::Fixed(40));
    }
}

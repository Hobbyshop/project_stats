use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref LANGUAGE_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::<&str, &str>::new();
        m.insert("rs", "Rust");
        m.insert("c", "C");
        m.insert("h", "C");
        m.insert("cpp", "C++");
        m.insert("hpp", "C++");
        m.insert("java", "Java");
        m.insert("js", "JavaScript");
        m.insert("jsx", "JavaScript");
        m.insert("ts", "TypeScript");
        m.insert("tsx", "TypeScript");
        m.insert("py", "Python");
        m.insert("lua", "Lua");
        m.insert("kt", "Kotlin");
        m
    };
}

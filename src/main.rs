use std::{env, fs::{self, File}, io::Write};

use svelter::{compile, options::RawOptions};

fn main() {
    let contents = fs::read_to_string("input/in.svelte");
    let result = compile("", RawOptions::default());

    {
        let mut js = File::create("input/out.js").unwrap();
        js.write_all(result.js.code.as_bytes()).unwrap();
    }

    {
        let mut css = File::create("input/out.css").unwrap();
        css.write_all(result.css.unwrap().code.as_bytes()).unwrap();
    }
}

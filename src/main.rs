#![feature(proc_macro_hygiene, decl_macro)]

use dindin_spike::server;

fn main() {
    server::init().launch();
}

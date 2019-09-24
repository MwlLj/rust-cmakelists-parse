use cmakelists_parse::parser::stream::{CParser, IKv};
use cmakelists_parse::parser::grammar::{CGrammar, ICall};

// parser test
struct CKv {
    content: Vec<u8>
}

impl IKv for CKv {
    fn kv(&mut self, key: &[u8], value: &[u8]) {
        println!("{:?}, {:?}", key, value);
    }

    fn ch(&mut self, c: u8) {
        self.content.push(c);
    }
}

fn parseTest() {
    let parser = CParser::new();
    let mut kv = CKv{
        content: Vec::new()
    };
    parser.parse("./resources/test2.txt", &mut kv);
    println!("{:?}", kv.content);
}

// grammar test
struct CCall {
}

impl ICall for CCall {
    fn on_if(&mut self, value: &str){
        println!("on if, value: {}", value);
    }

    fn on_else_if(&mut self, value: &str) {
        println!("on else if, value: {}", value);
    }

    fn on_else(&mut self) {
    }

    fn on_end_if(&mut self) {
    }

    fn on_kv(&mut self, key: &str, value: &str) {
    }

    fn on_ch(&mut self, c: char) {
    }
}

fn grammarTest() {
    let grammarParser = CGrammar::new();
    let mut call = CCall{
    };
    grammarParser.parse("./resources/test3.txt", &mut call);
}

fn main() {
    // parseTest();
    grammarTest();
}

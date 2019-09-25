use super::stream::{CParser, IKv};

const keyword_if: &str = "if";
const keyword_else_if: &str = "elseif";
const keyword_else: &str = "else";
const keyword_end_if: &str = "endif";

struct CKv<'a, F> {
    f: &'a mut F
}

impl<'a, F: ICall> IKv for CKv<'a, F> {
    fn kv(&mut self, key: &[u8], value: &[u8]) {
        let k = key.to_ascii_lowercase();
        let k = match String::from_utf8(k) {
            Ok(k) => k,
            Err(err) => {
                return;
            }
        };
        let value = &match String::from_utf8(value.to_vec()) {
            Ok(v) => v,
            Err(err) => {
                return;
            }
        };
        if k == keyword_if {
            if value.len() > 0 {
                self.f.on_if(value);
            }
        } else if k == keyword_else_if {
            if value.len() > 0 {
                self.f.on_else_if(value);
            }
        } else if k == keyword_else {
            self.f.on_else();
        } else if k == keyword_end_if {
            self.f.on_end_if();
        } else {
            let k = &match String::from_utf8(key.to_vec()) {
                Ok(k) => k,
                Err(err) => {
                    return;
                }
            };
            self.f.on_kv(k, value);
        }
    }

    fn ch(&mut self, c: u8) {
        self.f.on_ch(c as char);
    }

    fn double_quotes_end(&mut self) {
        self.f.on_double_quotes_end();
    }

    fn back_quote_end(&mut self) {
        self.f.on_back_quote_end();
    }
}

impl<'a, F> CKv<'a, F> {
    fn new(f: &'a mut F) -> CKv<'a, F> {
        CKv{
            f: f
        }
    }
}

pub trait ICall {
    fn on_if(&mut self, value: &str);
    fn on_else_if(&mut self, value: &str);
    fn on_else(&mut self);
    fn on_end_if(&mut self);
    fn on_kv(&mut self, key: &str, value: &str);
    fn on_ch(&mut self, c: char);
    fn on_double_quotes_end(&mut self) {}
    fn on_back_quote_end(&mut self) {}
}

pub struct CGrammar {
    parser: CParser
}

impl CGrammar {
    pub fn parse<F: ICall>(&self, path: &str, f: &mut F) -> Result<(), &str> {
        let mut kv = CKv::new(f);
        if let Err(err) = self.parser.parse(path, &mut kv) {
            return Err(err);
        };
        Ok(())
    }
}

impl CGrammar {
    pub fn new() -> CGrammar {
        CGrammar{
            parser: CParser::new()
        }
    }
}

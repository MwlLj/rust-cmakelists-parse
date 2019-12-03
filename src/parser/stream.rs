use std::fs;

enum WordMode {
    Key,
    Value
}

enum CharMode {
    // normal
    Normal,
    // ""
    DoubleQuotes,
    // `
    BackQuote,
    // #
    HashTag
}

pub trait IKv {
    fn kv(&mut self, key: &[u8], value: &[u8]);
    fn k_end(&mut self, key: &[u8]) {}
    fn ch(&mut self, c: u8);
    fn double_quotes_start(&mut self) {}
    fn double_quotes_end(&mut self) {}
    fn back_quote_end(&mut self) {}
}

pub struct CParser {
}

impl CParser {
    pub fn parse<T: IKv>(&self, path: &str, t: &mut T) -> Result<(), &str> {
        let content = match fs::read(path) {
            Ok(c) => c,
            Err(err) => {
                println!("read file content error, err: {}", err);
                return Err("read file error");
            }
        };
        self.parse_from_string(&content, t)
    }

    pub fn parse_from_string<T: IKv>(&self, content: &Vec<u8>, t: &mut T) -> Result<(), &str> {
        let mut charMode = CharMode::Normal;
        let mut wordMode = WordMode::Key;
        let mut word = Vec::new();
        let mut key = Vec::new();
        let mut externalBlockChar: u8 = b'"';
        let mut blockChar: u8 = b'"';
        let chars = content.as_slice();
        for c in chars {
            let c = *c;
            match charMode {
                CharMode::Normal => {
                    if c == b'`' || c == b'"' || c == b'\'' || c == b'^' {
                        match charMode {
                            CharMode::BackQuote => {
                            },
                            _ => {
                                charMode = CharMode::DoubleQuotes;
                                externalBlockChar = c;
                            }
                        }
                        t.double_quotes_start();
                    } else if c == b'`' || c == b'"' || c == b'\'' {
                        charMode = CharMode::BackQuote;
                        blockChar = c;
                    } else if c == b'#' {
                        charMode = CharMode::HashTag;
                    } else if c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' || c == b'(' {
                        match wordMode {
                            WordMode::Key => {
                                if word.len() != 0 {
                                    key = word.clone();
                                }
                            },
                            WordMode::Value => {
                                /*
                                ** values exist blank -> no Pointless
                                */
                                if word.len() > 0 {
                                    t.kv(key.as_slice(), word.as_slice());
                                    // println!("valueFn({}, {})", &key, &word);
                                }
                            }
                        }
                        if c == b'(' {
                            t.kv(key.as_slice(), vec!().as_slice());
                            wordMode = WordMode::Value;
                        }
                        word.clear();
                    } else if c == b')' {
                        // if word.len() > 0 {
                        t.k_end(key.as_slice());
                        // println!("valueFn({}, {})", &key, &word);
                        // }
                        wordMode = WordMode::Key;
                        key.clear();
                    } else {
                        word.push(c);
                    }
                },
                CharMode::DoubleQuotes => {
                    if c == externalBlockChar {
                        t.double_quotes_end();
                        charMode = CharMode::Normal;
                        t.kv(key.as_slice(), word.as_slice());
                        // println!("valueFn({}, {})", &key, &word);
                        word.clear();
                    } else {
                        word.push(c);
                    }
                },
                CharMode::BackQuote => {
                    if c == blockChar {
                        t.back_quote_end();
                        charMode = CharMode::Normal;
                        t.kv(key.as_slice(), word.as_slice());
                        word.clear();
                    } else {
                        word.push(c);
                    }
                },
                CharMode::HashTag => {
                    if c == b'\n' {
                        // # end
                        charMode = CharMode::Normal;
                    }
                }
            }
            match charMode {
                CharMode::HashTag => {
                },
                _ => {
                    t.ch(c);
                }
            }
        }
        Ok(())
    }
}

impl CParser {
    pub fn new() -> CParser {
        CParser{
        }
    }
}

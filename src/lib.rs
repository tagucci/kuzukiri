use std::collections::{HashMap, HashSet};
use pyo3::prelude::*;


#[pymodule]
fn kuzukiri(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Splitter>()?;
    Ok(())
}


#[pyclass]
struct Splitter {
    terminals: HashSet<char>,
    parens: HashMap<char, char>,
}

#[pymethods]
impl Splitter {
    #[new]
    fn new() -> Self {
        let terminals: HashSet<char> = vec! [
            '。', '、', '．', '，', '！', '？', '\n',
        ].into_iter().collect();

        let parens: HashMap<char, char> = vec![
            ('「', '」'),
            ('『', '』'),
            ('（', '）'),
            ('［', '］'),
            ('【', '】'),
        ].into_iter().collect();

        Splitter {
            terminals,
            parens
        }
    }

    fn split(&self, text: String) -> Vec<String> {
        let mut sentences: Vec<String> = vec![];
        let mut buf: Vec<char> = vec![];
        let mut waiting_stack: Vec<char> = vec![];

        for c in text.chars() {
            buf.push(c);

            if let Some(t) = self.parens.get(&c) {
                waiting_stack.push(t.clone());
            } else if let Some(d) = waiting_stack.last() {
                if c == *d {
                    waiting_stack.pop();
                }
            } else if self.terminals.contains(&c) {
                sentences.push(buf.into_iter().collect());
                buf = vec![];
            }
        }

        if buf.len() > 0 {
            sentences.push(buf.into_iter().collect());
        }

        sentences
    }
}

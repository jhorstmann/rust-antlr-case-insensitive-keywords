use antlr_rust::input_stream::InputStream;
use antlr_rust::token_stream::UnbufferedTokenStream;
use antlr_rust::common_token_stream::CommonTokenStream;

use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub columns: Vec<Column>,
    pub from: String,
    pub filter: Option<String>,
    pub limit: Option<u32>
}

#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub alias: Option<String>
}


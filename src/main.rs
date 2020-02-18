use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;

use rust_antlr_case_insensitive_keywords::gen::querylexer::QueryLexer;
use rust_antlr_case_insensitive_keywords::gen::queryparser::*;
use rust_antlr_case_insensitive_keywords::gen::queryparser::QueryParser;
use rust_antlr_case_insensitive_keywords::ast::*;

use std::rc::Rc;

fn main() {
    let data = "select".to_owned();

    let token_source = CommonTokenStream::new(
        QueryLexer::new(Box::new(InputStream::new(data))));

    let mut parser = QueryParser::new(Box::new(token_source));

    let query = parser.query().unwrap();
    let select_token = &QueryContextAttrs::SELECT(query.as_ref()).unwrap().symbol;

    dbg!(select_token.text.clone());
}

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;

use rust_antlr_case_insensitive_keywords::gen::querylexer::QueryLexer;
use rust_antlr_case_insensitive_keywords::gen::queryparser::*;
use rust_antlr_case_insensitive_keywords::gen::queryparser::QueryParser;
use rust_antlr_case_insensitive_keywords::ast::*;

use std::rc::Rc;

fn id(opt_ctx: &Option<Rc<IdentifierContext>>) -> Option<String> {
    opt_ctx.as_ref().and_then(|ctx| IdentifierContextAttrs::IDENTIFIER(ctx.as_ref()))
        .map(|i| i.symbol.text.clone())
}

fn main() {
    let data = "select foo as f, bar as b from table where baz limit 10".to_owned();

    let token_source = CommonTokenStream::new(
        QueryLexer::new(Box::new(InputStream::new(data))));

    let mut parser = QueryParser::new(Box::new(token_source));

    let query = parser.query().unwrap();

    let columns_ctx = QueryContextAttrs::columns(query.as_ref()).unwrap();

    let col_ctx = ColumnsContextAttrs::column_all(columns_ctx.as_ref());

    let columns : Vec<Column> = col_ctx.into_iter().map(|c| {
        let name = id(&c.name).unwrap();
        let alias = id(&c.alias);

        Column { name, alias }
    }).collect();

    let from = id(&query.source).unwrap();

    let filter = QueryContextAttrs::whereClause(query.as_ref())
        .and_then(|w| id(&w.name));

    let limit: Option<u32> = QueryContextAttrs::limitClause(query.as_ref())
        .map(|l| l.as_ref().limit.as_ref().unwrap().text.clone())
        .map(|l| l.parse().unwrap());


    let query = Query {
        columns,
        from,
        filter,
        limit
    };


    dbg!(query);
}

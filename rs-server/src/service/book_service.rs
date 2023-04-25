use crate::{AppContext, Book, BookServiceServer, GetBookRequest};
use std::sync::Arc;

pub struct BookService {}

#[async_trait::async_trait]
impl BookServiceServer<AppContext> for BookService {
    async fn get_book(&self, request: GetBookRequest, ctx: Arc<AppContext>) -> Book {
        // sleep(Duration::from_secs(2)).await;
        let book = ctx
            .hardcoded_database
            .iter()
            .find(|book_record| book_record.isbn == request.isbn);

        book.map(Book::clone).unwrap_or_default()
    }
}

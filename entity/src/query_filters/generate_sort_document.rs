use cash_result::OperationResult;
use dependencies_sync::bson::Document;
use manage_define::cashmere::OrderBy;

pub fn generate_sort_document(OrderBy: &Vec<OrderBy>) -> Document{
    let mut sort_document = Document::new();

    for order_by in OrderBy {
        sort_document.insert(
            order_by.field.clone(),
            if order_by.asc { 1 } else { -1 },
        );
    }
    
    sort_document
}

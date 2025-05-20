// inventory-system/backend/src/routes.rs V 1.14
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use tera::Tera;
use crate::models::{InventoryItem, AddItemForm, UpdateItemForm, ProductMasterItem};
use serde::Deserialize;

use std::sync::Mutex;
use once_cell::sync::Lazy;

static PRODUCT_MASTER_DATA: Lazy<Mutex<Vec<ProductMasterItem>>> = Lazy::new(|| {
    Mutex::new(vec![
        ProductMasterItem {
            part_number: "11001".to_string(),
            item_name: "咖啡風味調味粉 1k(特)".to_string(),
            case_pack_count: Some(12),
            storage_location: Some("5-14-01".to_string()),
            barcode: Some("1010010".to_string()),
            image_path: Some("/images/products/11001.jpg".to_string()), // 範例圖片路徑
        },
        ProductMasterItem {
            part_number: "11002".to_string(),
            item_name: "草莓風味調味粉 1K".to_string(),
            case_pack_count: Some(12),
            storage_location: Some("5-09-01".to_string()),
            barcode: Some("4713309030048".to_string()),
            image_path: Some("/images/products/11002.jpg".to_string()), // 範例圖片路徑
        },
        ProductMasterItem {
            part_number: "TOOL-X01".to_string(),
            item_name: "專業級電烙鐵".to_string(),
            case_pack_count: Some(1),
            storage_location: Some("A-01-03".to_string()),
            barcode: Some("T001X01".to_string()),
            image_path: None, // 假設這個沒有圖片
        },
        // ... 為您其他的 ProductMasterItem 數據添加 image_path (如果有的話) ...
        ProductMasterItem {
            part_number: "11038".to_string(),
            item_name: "花香奶茶300g#".to_string(),
            case_pack_count: Some(40),
            storage_location: None,
            barcode: Some("4713309030536".to_string()),
            image_path: Some("/images/products/default_drink.jpg".to_string()), // 可以有一個預設圖片
        },
    ])
});

// MOCK_DB 初始化邏輯保持不變，它從 PRODUCT_MASTER_DATA 讀取品號和品名

// V 1.14: 修改 render_inventory_table_fragment 以包含圖片
fn render_inventory_table_fragment(items: &Vec<InventoryItem>) -> String {
    let mut html_output = String::new();
    let master_data_lock = PRODUCT_MASTER_DATA.lock().unwrap(); // 需要主檔來獲取圖片路徑

    html_output.push_str("<div id='inventory-table-wrapper'>");
    html_output.push_str("<table id='inventory-table-inner'>");
    // 新增 "圖片" 欄表頭
    html_output.push_str("<thead><tr><th>圖片</th><th>ID</th><th>品號</th><th>品名</th><th>數量</th><th>操作</th></tr></thead>");
    html_output.push_str("<tbody>");
    if items.is_empty() {
        html_output.push_str("<tr><td colspan='6' style='text-align:center;'>目前沒有庫存項目</td></tr>"); // colspan 改為 6
    } else {
        for item in items {
            // 根據庫存項目的 part_number 從主檔查找圖片路徑
            let image_html = master_data_lock
                .iter()
                .find(|p| p.part_number == item.part_number)
                .and_then(|p| p.image_path.as_ref())
                .map(|path| format!("<img src='/static{}' alt='{}' style='width:40px; height:50px; object-fit:cover;'>", path, item.item_name)) // 約 2cm x 2.5cm (50px 高, 40px 寬)
                .unwrap_or_else(|| "<span style='font-size:10px; color:grey;'>無圖</span>".to_string()); // 如果沒有圖片路徑或找不到，顯示提示

            let row = format!(
                "<tr id='item-row-{}'> \
                    <td>{}</td> \
                    <td>{}</td> \
                    <td>{}</td> \
                    <td>{}</td> \
                    <td>{}</td> \
                    <td> \
                        <button hx-get='/api/inventory/edit/{}' hx-target='#item-row-{}' hx-swap='outerHTML' style='margin-right: 5px;'>編輯</button> \
                        <button hx-delete='/api/inventory/delete/{}' hx-target='#inventory-list' hx-swap='innerHTML' hx-confirm='確定要刪除品號 {} (ID: {}) 嗎？'>刪除</button> \
                    </td> \
                </tr>",
                item.id, // for tr id
                image_html, // 圖片儲存格
                item.id, item.part_number, item.item_name, item.quantity,
                item.id, item.id,
                item.id, item.part_number, item.id
            );
            html_output.push_str(&row);
        }
    }
    html_output.push_str("</tbody></table></div>");
    drop(master_data_lock); // 釋放鎖
    html_output
}

// ... (index, ping, get_inventory_list, add_inventory_item, UniversalSearchQuery 結構, search_products_universal_fuzzy 保持不變) ...
// 注意: search_products_universal_fuzzy 回傳的建議列表也可以考慮加入小圖片預覽

// V 1.14: 修改 get_view_item_row 以包含圖片 (用於取消編輯時恢復)
#[get("/api/inventory/view/{item_id}")]
async fn get_view_item_row(path: web::Path<i32>) -> impl Responder {
    let item_id_to_view = path.into_inner();
    println!("後端收到檢視行請求，ID: {}", item_id_to_view);

    let inventory_lock = MOCK_DB.lock().unwrap();
    let item_option = inventory_lock.iter().find(|item| item.id == item_id_to_view).cloned();
    drop(inventory_lock);

    if let Some(item) = item_option {
        let master_data_lock = PRODUCT_MASTER_DATA.lock().unwrap();
        let image_html = master_data_lock
            .iter()
            .find(|p| p.part_number == item.part_number)
            .and_then(|p| p.image_path.as_ref())
            .map(|path| format!("<img src='/static{}' alt='{}' style='width:40px; height:50px; object-fit:cover;'>", path, item.item_name))
            .unwrap_or_else(|| "<span style='font-size:10px; color:grey;'>無圖</span>".to_string());
        drop(master_data_lock);

        let view_row_html = format!(
            "<tr id='item-row-{}'> \
                <td>{}</td> \
                <td>{}</td><td>{}</td><td>{}</td><td>{}</td> \
                <td> \
                    <button hx-get='/api/inventory/edit/{}' hx-target='#item-row-{}' hx-swap='outerHTML' style='margin-right: 5px;'>編輯</button> \
                    <button hx-delete='/api/inventory/delete/{}' hx-target='#inventory-list' hx-swap='innerHTML' hx-confirm='確定要刪除品號 {} (ID: {}) 嗎？'>刪除</button> \
                </td> \
            </tr>",
            item.id, // tr id
            image_html, // image cell
            item.id, item.part_number, item.item_name, item.quantity,
            item.id, item.id, // edit button
            item.id, item.part_number, item.id // delete button
        );
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body(view_row_html)
    } else {
        HttpResponse::NotFound().content_type("text/plain; charset=utf-8").body(format!("錯誤：找不到 ID 為 {} 的項目。", item_id_to_view))
    }
}

// V 1.14: get_edit_item_form 也需要考慮顯示圖片 (通常編輯時圖片是固定的，不直接編輯圖片路徑)
#[get("/api/inventory/edit/{item_id}")]
async fn get_edit_item_form(path: web::Path<i32>) -> impl Responder {
    let item_id_to_edit = path.into_inner();
    println!("後端收到編輯表單請求，ID: {}", item_id_to_edit);
    let db_lock = MOCK_DB.lock().unwrap(); // 庫存鎖
    let item_to_edit_option = db_lock.iter().find(|item| item.id == item_id_to_edit).cloned();
    drop(db_lock);

    match item_to_edit_option {
        Some(item) => {
            let master_data_lock = PRODUCT_MASTER_DATA.lock().unwrap(); // 主檔鎖
            let image_html = master_data_lock
                .iter()
                .find(|p| p.part_number == item.part_number)
                .and_then(|p| p.image_path.as_ref())
                .map(|path| format!("<img src='/static{}' alt='{}' style='width:40px; height:50px; object-fit:cover;'>", path, item.item_name))
                .unwrap_or_else(|| "<span style='font-size:10px; color:grey;'>無圖</span>".to_string());
            drop(master_data_lock);

            let edit_form_html = format!(
                "<tr id='edit-form-row-{}'> \
                    <td>{}</td> \
                    <td>{}</td> \
                    <td><input type='text' name='part_number' value='{}' form='update-item-form-{}' maxlength='20' size='15' /></td> \
                    <td><input type='text' name='item_name' value='{}' form='update-item-form-{}' size='30'/></td> \
                    <td><input type='number' name='quantity' value='{}' form='update-item-form-{}' min='0' style='width: 60px;'/></td> \
                    <td> \
                        <button hx-put='/api/inventory/update/{}' hx-include='#edit-form-row-{} input[name]' hx-target='#inventory-list' hx-swap='innerHTML'>保存</button> \
                        <button hx-get='/api/inventory/view/{}' hx-target='#edit-form-row-{}' hx-swap='outerHTML'>取消</button> \
                    </td> \
                </tr> \
                <form id='update-item-form-{}' style='display: none;'></form>",
                item.id, // tr id
                image_html, // image cell
                item.id, // ID cell
                item.part_number, item.id,
                item.item_name, item.id,
                item.quantity, item.id,
                item.id, item.id,
                item.id, item.id, item.id
            );
            HttpResponse::Ok().content_type("text/html; charset=utf-8").body(edit_form_html)
        }
        None => { /* ... (與 V1.13 相同) ... */ }
    }
}

// update_inventory_item, delete_inventory_item, configure_routes 保持不變
// ... (確保這些函式也存在，並且 configure_routes 註冊了所有服務)
// (請從您上一個版本的完整 routes.rs 複製這些函式過來，如果這裡省略了)
#[put("/api/inventory/update/{item_id}")]
async fn update_inventory_item(path: web::Path<i32>, form: web::Form<UpdateItemForm>) -> impl Responder { /* ... (與 V1.13 相同) ... */ }

#[delete("/api/inventory/delete/{item_id}")]
async fn delete_inventory_item(path: web::Path<i32>) -> impl Responder { /* ... (與 V1.13 相同) ... */ }

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
       .service(ping)
       .service(get_inventory_list)
       .service(add_inventory_item)
       .service(search_products_universal_fuzzy)
       .service(get_edit_item_form)
       .service(get_view_item_row)
       .service(update_inventory_item)
       .service(delete_inventory_item);
}
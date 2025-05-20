// inventory-system/backend/src/models.rs V 1.5
use serde::{Serialize, Deserialize};

// InventoryItem 結構保持不變
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub id: i32,
    pub part_number: String,
    pub item_name: String,
    pub quantity: i32,
}

// AddItemForm 結構保持不變
#[derive(Deserialize, Debug)]
pub struct AddItemForm {
    pub part_number: String,
    pub quantity: i32,
}

// UpdateItemForm 結構保持不變
#[derive(Deserialize, Debug)]
pub struct UpdateItemForm {
    pub part_number: String,
    pub item_name: String,
    pub quantity: i32,
}

// ProductMasterItem 結構更新
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductMasterItem {
    pub part_number: String,
    pub item_name: String,
    pub case_pack_count: Option<i32>,
    pub storage_location: Option<String>,
    pub barcode: Option<String>,
    pub image_path: Option<String>, // V 1.5: 新增圖片路徑欄位
}
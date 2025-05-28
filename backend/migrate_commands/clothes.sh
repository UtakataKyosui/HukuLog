#!/bin/bash

# Phase 1: 基本テーブル
# cargo loco generate scaffold user username:string email:string password_hash:string
cargo loco generate scaffold brand name:string website:string
cargo loco generate scaffold category name:string sort_order:int
cargo loco generate scaffold color name:string hex_code:string rgb_code:string
cargo loco generate scaffold material name:string description:text
cargo loco generate scaffold season name:string start_month:int end_month:int
cargo loco generate scaffold size name:string sort_order:int
cargo loco generate scaffold condition name:string description:text sort_order:int
cargo loco generate scaffold tag_category name:string color:string
cargo loco generate scaffold tag name:string

# Phase 2: 階層構造
cargo loco g migration AddParentRefToCategories parent:references
cargo loco g migration AddTagCategoryRefToTags tag_category:references

# Phase 3: メインエンティティ
cargo loco generate scaffold clothing name:string purchased_at:date purchase_price:decimal image_url:string notes:text
cargo loco generate scaffold outfit name:string description:text image_url:string

# Phase 4: メインエンティティの外部キー
cargo loco g migration AddUserRefToClothings user:references
cargo loco g migration AddBrandRefToClothings brand:references
cargo loco g migration AddCategoryRefToClothings category:references
cargo loco g migration AddSizeRefToClothings size:references
cargo loco g migration AddConditionRefToClothings condition:references
cargo loco g migration AddUserRefToOutfits user:references

# Phase 5: 中間テーブル
cargo loco g migration CreateJoinTableClothingsAndOutfits layer_order:int
cargo loco g migration CreateJoinTableClothingsAndColors is_primary:boolean
cargo loco g migration CreateJoinTableClothingsAndMaterials percentage:decimal
cargo loco g migration CreateJoinTableClothingsAndSeasons
cargo loco g migration CreateJoinTableClothingsAndTags
cargo loco g migration CreateJoinTableOutfitsAndTags

# Phase 6: お気に入り
cargo loco g migration CreateJoinTableUsersAndClothings
cargo loco g migration CreateJoinTableUsersAndOutfits

# Phase 7: 高度な機能
cargo loco g migration CreateUserPreferences preference_weight:int
cargo loco g migration AddUserRefToUserPreferences user:references
cargo loco g migration AddTagRefToUserPreferences tag:references
cargo loco generate scaffold wear_history worn_date:date weather:string temperature:decimal notes:text
cargo loco g migration AddUserRefToWearHistories user:references
cargo loco g migration AddClothingRefToWearHistories clothing:references
cargo loco g migration AddOutfitRefToWearHistories outfit:references

echo "All migrations and scaffolds created successfully!"
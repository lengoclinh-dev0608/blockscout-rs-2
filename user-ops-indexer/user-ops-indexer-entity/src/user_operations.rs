//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use super::sea_orm_active_enums::{EntryPointVersion, SponsorType};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_operations")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(None))"
    )]
    pub hash: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub sender: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub nonce: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub init_code: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub call_data: Vec<u8>,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub call_gas_limit: BigDecimal,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub verification_gas_limit: BigDecimal,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub pre_verification_gas: BigDecimal,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub max_fee_per_gas: BigDecimal,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub max_priority_fee_per_gas: BigDecimal,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub paymaster_and_data: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub signature: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub aggregator: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub aggregator_signature: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub entry_point: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub transaction_hash: Vec<u8>,
    pub block_number: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub block_hash: Vec<u8>,
    pub bundle_index: i32,
    pub index: i32,
    pub user_logs_start_index: i32,
    pub user_logs_count: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub bundler: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub factory: Option<Vec<u8>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub paymaster: Option<Vec<u8>>,
    pub status: bool,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub revert_reason: Option<Vec<u8>>,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub gas: BigDecimal,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub gas_price: BigDecimal,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))")]
    pub gas_used: BigDecimal,
    pub sponsor_type: SponsorType,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
    pub entry_point_version: EntryPointVersion,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//  Copyright 2021 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use std::collections::HashMap;

use common_datavalues::DataSchemaRef;
use common_metatypes::Database;
use common_metatypes::Table;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateDatabaseActionResult {
    pub database_id: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DatabaseInfo {
    pub database_id: u64,
    pub db: String,
    pub engine: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DropDatabaseActionResult {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateTableActionResult {
    pub table_id: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DropTableActionResult {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TableInfo {
    pub table_id: u64,
    pub db: String,
    pub name: String,
    pub schema: DataSchemaRef,
    pub engine: String,
    pub options: HashMap<String, String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DatabaseMetaSnapshot {
    pub meta_ver: u64,
    pub db_metas: Vec<(String, Database)>,
    pub tbl_metas: Vec<(u64, Table)>,
}

pub type DatabaseMetaReply = Option<DatabaseMetaSnapshot>;
pub type GetDatabasesReply = Vec<DatabaseInfo>;
pub type GetTablesReply = Vec<TableInfo>;

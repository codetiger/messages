// Open Payment Message Parsing Library
// https://github.com/Open-Payments/messages
//
// This library is designed to parse message formats based on the ISO 20022 standards,
// including but not limited to FedNow messages. It supports various financial message types,
// such as customer credit transfers, payment status reports, administrative notifications, 
// and other ISO 20022 messages, using Serde for efficient serialization and deserialization.
//
// Copyright (c) 2024 Open Payments by Harishankar Narayanan
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// You may obtain a copy of this library at
// https://github.com/Open-Payments/messages

use serde::{Deserialize, Serialize};


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// FinancialInstrumentReportingInstrumentClassificationReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingInstrumentClassificationReportV01 {
	#[serde(rename = "InstrmClssfctn")]
	pub instrm_clssfctn: Vec<SecuritiesInstrumentClassification2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Modification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Modification1Code {
	#[serde(rename = "Modification1Code")]
	pub modification1_code: String,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt")]
	pub fr_dt_to_dt: Option<Period2>,
}


// SecuritiesInstrumentClassification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesInstrumentClassification2 {
	#[serde(rename = "Idr")]
	pub idr: String,
	#[serde(rename = "Mod")]
	pub mod_attr: Option<String>,
	#[serde(rename = "VldtyPrd")]
	pub vldty_prd: Period4Choice,
	#[serde(rename = "LastUpdtd")]
	pub last_updtd: Option<String>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

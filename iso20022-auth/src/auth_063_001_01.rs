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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AmountAndDirection102 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection102 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}


// CCPLiquidityStressTestingResultReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPLiquidityStressTestingResultReportV01 {
	#[serde(rename = "LqdtyStrssTstRslt")]
	pub lqdty_strss_tst_rslt: Vec<LiquidityStressTestResult1>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CoverTwoDefaulters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CoverTwoDefaulters1 {
	#[serde(rename = "Cover1Id")]
	pub cover1_id: String,
	#[serde(rename = "Cover2Id")]
	pub cover2_id: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LiquidResourceInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidResourceInformation1 {
	#[serde(rename = "CntrPtyId")]
	pub cntr_pty_id: Option<String>,
	#[serde(rename = "LqdRsrcVal")]
	pub lqd_rsrc_val: AmountAndDirection102,
	#[serde(rename = "MktVal")]
	pub mkt_val: Option<AmountAndDirection102>,
	#[serde(rename = "Scrd")]
	pub scrd: bool,
	#[serde(rename = "AsstNcmbrd")]
	pub asst_ncmbrd: bool,
	#[serde(rename = "QlfygRsrc")]
	pub qlfyg_rsrc: bool,
	#[serde(rename = "AgcyArrgmnts")]
	pub agcy_arrgmnts: bool,
}


// LiquidResources1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidResources1 {
	#[serde(rename = "CshDue")]
	pub csh_due: Vec<LiquidResourceInformation1>,
	#[serde(rename = "FcltiesCmmtdLinesOfCdt")]
	pub fclties_cmmtd_lines_of_cdt: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesCmmtdRpAgrmts")]
	pub fclties_cmmtd_rp_agrmts: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesCmmtdFxSwps")]
	pub fclties_cmmtd_fx_swps: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesOthrCmmtd")]
	pub fclties_othr_cmmtd: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesUcmmtd")]
	pub fclties_ucmmtd: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsCCP")]
	pub fin_instrms_ccp: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsTrsrInvstmts")]
	pub fin_instrms_trsr_invstmts: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsDfltrsSttlmColl")]
	pub fin_instrms_dfltrs_sttlm_coll: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsDfltrsNonCshColl")]
	pub fin_instrms_dfltrs_non_csh_coll: Option<Vec<LiquidResourceInformation1>>,
}


// LiquidityRequiredAndAvailable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidityRequiredAndAvailable1 {
	#[serde(rename = "LqdRsrcs")]
	pub lqd_rsrcs: LiquidResources1,
	#[serde(rename = "LqdtyHrzn")]
	pub lqdty_hrzn: String,
	#[serde(rename = "StrssLqdRsrcRqrmnt")]
	pub strss_lqd_rsrc_rqrmnt: StressLiquidResourceRequirement1,
}


// LiquidityStressTestResult1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidityStressTestResult1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "ScnroDfltrs")]
	pub scnro_dfltrs: CoverTwoDefaulters1,
	#[serde(rename = "LqdtyReqrdAndAvlbl")]
	pub lqdty_reqrd_and_avlbl: Vec<LiquidityRequiredAndAvailable1>,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// SettlementDate6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDate6Code {
	#[serde(rename = "SettlementDate6Code")]
	pub settlement_date6_code: String,
}


// StressLiquidResourceRequirement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressLiquidResourceRequirement1 {
	#[serde(rename = "OprlOutflw")]
	pub oprl_outflw: AmountAndDirection102,
	#[serde(rename = "VartnMrgnPmtOblgtn")]
	pub vartn_mrgn_pmt_oblgtn: AmountAndDirection102,
	#[serde(rename = "SttlmOrDlvry")]
	pub sttlm_or_dlvry: AmountAndDirection102,
	#[serde(rename = "Othr")]
	pub othr: AmountAndDirection102,
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


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}

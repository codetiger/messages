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
use regex::Regex;
use crate::validationerror::*;


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_or_historic_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}

impl ActiveOrHistoricCurrencyCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}

impl AnyBICIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_identifier) {
			return Err(ValidationError::new(1005, "any_bic_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CallIn1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallIn1Code {
	#[default]
	#[serde(rename = "CFAV")]
	CodeCFAV,
	#[serde(rename = "CFST")]
	CodeCFST,
	#[serde(rename = "CFCC")]
	CodeCFCC,
}

impl CallIn1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalClearingSystemIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ClearingSystemIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}

impl Exact4AlphaNumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return Err(ValidationError::new(1005, "exact4_alpha_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}

impl ExternalClearingSystemIdentification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_clearing_system_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_clearing_system_identification1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_clearing_system_identification1_code.chars().count() > 5 {
			return Err(ValidationError::new(1002, "external_clearing_system_identification1_code exceeds the maximum length of 5".to_string()));
		}
		Ok(())
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}

impl Max105Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max105_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max105_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max105_text.chars().count() > 105 {
			return Err(ValidationError::new(1002, "max105_text exceeds the maximum length of 105".to_string()));
		}
		Ok(())
	}
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}

impl Max16Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max16_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max16_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max16_text.chars().count() > 16 {
			return Err(ValidationError::new(1002, "max16_text exceeds the maximum length of 16".to_string()));
		}
		Ok(())
	}
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
}

impl Max34Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max34_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max34_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max34_text.chars().count() > 34 {
			return Err(ValidationError::new(1002, "max34_text exceeds the maximum length of 34".to_string()));
		}
		Ok(())
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max350_text.chars().count() > 350 {
			return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max35_text.chars().count() > 35 {
			return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}

impl Max70Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max70_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max70_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max70_text.chars().count() > 70 {
			return Err(ValidationError::new(1002, "max70_text exceeds the maximum length of 70".to_string()));
		}
		Ok(())
	}
}


// NameAndAddress8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress8 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
	#[serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none")]
	pub altrntv_idr: Option<Vec<Max35Text>>,
}

impl NameAndAddress8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
		if let Some(ref altrntv_idr_vec) = self.altrntv_idr { for item in altrntv_idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyIdentification44 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification44 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: AnyBICIdentifier,
	#[serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none")]
	pub altrntv_idr: Option<Vec<Max35Text>>,
}

impl PartyIdentification44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.any_bic.validate() { return Err(e); }
		if let Some(ref altrntv_idr_vec) = self.altrntv_idr { for item in altrntv_idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyIdentification59 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification59 {
	#[serde(rename = "PtyNm", skip_serializing_if = "Option::is_none")]
	pub pty_nm: Option<Max34Text>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<PartyIdentification44>,
	#[serde(rename = "AcctNb", skip_serializing_if = "Option::is_none")]
	pub acct_nb: Option<Max34Text>,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<Max105Text>,
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
}

impl PartyIdentification59 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pty_nm_value) = self.pty_nm { if let Err(e) = pty_nm_value.validate() { return Err(e); } }
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref acct_nb_value) = self.acct_nb { if let Err(e) = acct_nb_value.validate() { return Err(e); } }
		if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
		if let Some(ref clr_sys_id_value) = self.clr_sys_id { if let Err(e) = clr_sys_id_value.validate() { return Err(e); } }
		if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if let Err(e) = lgl_ntty_idr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification73Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification73Choice {
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<PartyIdentification44>,
	#[serde(rename = "PtyId", skip_serializing_if = "Option::is_none")]
	pub pty_id: Option<PartyIdentification59>,
}

impl PartyIdentification73Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref pty_id_value) = self.pty_id { if let Err(e) = pty_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PayInCallItem ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PayInCallItem {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl PayInCallItem {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// PayInCallV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PayInCallV02 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification73Choice,
	#[serde(rename = "RptData")]
	pub rpt_data: ReportData5,
	#[serde(rename = "SttlmSsnIdr", skip_serializing_if = "Option::is_none")]
	pub sttlm_ssn_idr: Option<Exact4AlphaNumericText>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PayInCallV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_id.validate() { return Err(e); }
		if let Err(e) = self.rpt_data.validate() { return Err(e); }
		if let Some(ref sttlm_ssn_idr_value) = self.sttlm_ssn_idr { if let Err(e) = sttlm_ssn_idr_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}

impl PostalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
		if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
		if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
		if let Err(e) = self.ctry.validate() { return Err(e); }
		Ok(())
	}
}


// ReportData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportData5 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "DtAndTmStmp")]
	pub dt_and_tm_stmp: String,
	#[serde(rename = "Tp")]
	pub tp: CallIn1Code,
	#[serde(rename = "PayInCallAmt", skip_serializing_if = "Option::is_none")]
	pub pay_in_call_amt: Option<Vec<PayInCallItem>>,
	#[serde(rename = "SttlmSsnIdr", skip_serializing_if = "Option::is_none")]
	pub sttlm_ssn_idr: Option<Exact4AlphaNumericText>,
	#[serde(rename = "AcctVal", skip_serializing_if = "Option::is_none")]
	pub acct_val: Option<Value>,
}

impl ReportData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Some(ref pay_in_call_amt_vec) = self.pay_in_call_amt { for item in pay_in_call_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref sttlm_ssn_idr_value) = self.sttlm_ssn_idr { if let Err(e) = sttlm_ssn_idr_value.validate() { return Err(e); } }
		if let Some(ref acct_val_value) = self.acct_val { if let Err(e) = acct_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.envlp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Value ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Value {
	#[serde(rename = "BaseCcyItm")]
	pub base_ccy_itm: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "AltrnCcyItm")]
	pub altrn_ccy_itm: Vec<ActiveOrHistoricCurrencyAndAmount>,
}

impl Value {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.base_ccy_itm.validate() { return Err(e); }
		for item in &self.altrn_ccy_itm { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}

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


// AuditTrail1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuditTrail1 {
	#[serde(rename = "FldNm")]
	pub fld_nm: Max35Text,
	#[serde(rename = "OdFldVal")]
	pub od_fld_val: Max350Text,
	#[serde(rename = "NewFldVal")]
	pub new_fld_val: Max350Text,
	#[serde(rename = "OprTmStmp")]
	pub opr_tm_stmp: String,
	#[serde(rename = "InstgUsr")]
	pub instg_usr: Max256Text,
	#[serde(rename = "ApprvgUsr", skip_serializing_if = "Option::is_none")]
	pub apprvg_usr: Option<Max256Text>,
}

impl AuditTrail1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fld_nm.validate() { return Err(e); }
		if let Err(e) = self.od_fld_val.validate() { return Err(e); }
		if let Err(e) = self.new_fld_val.validate() { return Err(e); }
		if let Err(e) = self.instg_usr.validate() { return Err(e); }
		if let Some(ref apprvg_usr_value) = self.apprvg_usr { if let Err(e) = apprvg_usr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AuditTrailOrBusinessError6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuditTrailOrBusinessError6Choice {
	#[serde(rename = "AudtTrl", skip_serializing_if = "Option::is_none")]
	pub audt_trl: Option<Vec<AuditTrail1>>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl AuditTrailOrBusinessError6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref audt_trl_vec) = self.audt_trl { for item in audt_trl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref biz_err_vec) = self.biz_err { for item in biz_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DatePeriodSearch1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriodSearch1Choice {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
	#[serde(rename = "EQDt", skip_serializing_if = "Option::is_none")]
	pub eq_dt: Option<String>,
	#[serde(rename = "NEQDt", skip_serializing_if = "Option::is_none")]
	pub neq_dt: Option<String>,
}

impl DatePeriodSearch1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ErrorHandling3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ErrorHandling3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ErrorHandling5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl ErrorHandling5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.err.validate() { return Err(e); }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
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


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "$value")]
	pub external_system_error_handling1_code: String,
}

impl ExternalSystemErrorHandling1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_system_error_handling1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_system_error_handling1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_system_error_handling1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_system_error_handling1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.issr.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
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


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max140_text.chars().count() > 140 {
			return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
		}
		Ok(())
	}
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max256_text.chars().count() > 256 {
			return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
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


// MessageHeader12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader12 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
}

impl MessageHeader12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref orgnl_biz_instr_value) = self.orgnl_biz_instr { if let Err(e) = orgnl_biz_instr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OriginalBusinessInstruction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessInstruction1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessInstruction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref msg_nm_id_value) = self.msg_nm_id { if let Err(e) = msg_nm_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesAccount19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount19 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
}

impl SecuritiesAccount19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesAccountAuditTrailOrOperationalError3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountAuditTrailOrOperationalError3Choice {
	#[serde(rename = "SctiesAcctAudtTrlRpt", skip_serializing_if = "Option::is_none")]
	pub scties_acct_audt_trl_rpt: Option<Vec<SecuritiesAccountAuditTrailReport3>>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl SecuritiesAccountAuditTrailOrOperationalError3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scties_acct_audt_trl_rpt_vec) = self.scties_acct_audt_trl_rpt { for item in scties_acct_audt_trl_rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref oprl_err_vec) = self.oprl_err { for item in oprl_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuritiesAccountAuditTrailReport3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountAuditTrailReport3 {
	#[serde(rename = "SctiesAcctAudtTrlOrErr")]
	pub scties_acct_audt_trl_or_err: AuditTrailOrBusinessError6Choice,
	#[serde(rename = "DtPrd", skip_serializing_if = "Option::is_none")]
	pub dt_prd: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "SctiesAcctId")]
	pub scties_acct_id: SecuritiesAccount19,
}

impl SecuritiesAccountAuditTrailReport3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.scties_acct_audt_trl_or_err.validate() { return Err(e); }
		if let Some(ref dt_prd_value) = self.dt_prd { if let Err(e) = dt_prd_value.validate() { return Err(e); } }
		if let Err(e) = self.scties_acct_id.validate() { return Err(e); }
		Ok(())
	}
}


// SecuritiesAccountAuditTrailReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountAuditTrailReportV01 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader12>,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: SecuritiesAccountAuditTrailOrOperationalError3Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesAccountAuditTrailReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref msg_hdr_value) = self.msg_hdr { if let Err(e) = msg_hdr_value.validate() { return Err(e); } }
		if let Err(e) = self.rpt_or_err.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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

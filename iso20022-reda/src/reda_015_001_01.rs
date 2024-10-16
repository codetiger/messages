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


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}

impl AnyBICDec2014Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
		}
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


// DateAndDateTimeSearch4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeSearch4Choice {
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<DateTimeSearch2Choice>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DatePeriodSearch1Choice>,
}

impl DateAndDateTimeSearch4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref dt_tm_value) = self.dt_tm { if let Err(e) = dt_tm_value.validate() { return Err(e); } }
		if let Some(ref dt_value) = self.dt { if let Err(e) = dt_value.validate() { return Err(e); } }
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


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimeSearch2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimeSearch2Choice {
	#[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "FrToDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "EQDtTm", skip_serializing_if = "Option::is_none")]
	pub eq_dt_tm: Option<String>,
	#[serde(rename = "NEQDtTm", skip_serializing_if = "Option::is_none")]
	pub neq_dt_tm: Option<String>,
}

impl DateTimeSearch2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fr_to_dt_tm_value) = self.fr_to_dt_tm { if let Err(e) = fr_to_dt_tm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExternalSystemPartyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemPartyType1Code {
	#[serde(rename = "$value")]
	pub external_system_party_type1_code: String,
}

impl ExternalSystemPartyType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_system_party_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_system_party_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_system_party_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_system_party_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification36 {
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


// LockStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LockStatus1Code {
	#[default]
	#[serde(rename = "LOCK")]
	CodeLOCK,
	#[serde(rename = "ULCK")]
	CodeULCK,
}

impl LockStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// MessageHeader2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader2 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType2Choice>,
}

impl MessageHeader2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref req_tp_value) = self.req_tp { if let Err(e) = req_tp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyDataReturnCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyDataReturnCriteria2 {
	#[serde(rename = "OpngDt", skip_serializing_if = "Option::is_none")]
	pub opng_dt: Option<bool>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<bool>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<bool>,
	#[serde(rename = "PtyId", skip_serializing_if = "Option::is_none")]
	pub pty_id: Option<bool>,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<bool>,
	#[serde(rename = "RstrctnId", skip_serializing_if = "Option::is_none")]
	pub rstrctn_id: Option<bool>,
	#[serde(rename = "RstrctdOnDt", skip_serializing_if = "Option::is_none")]
	pub rstrctd_on_dt: Option<bool>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<bool>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<bool>,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<bool>,
	#[serde(rename = "TechAdr", skip_serializing_if = "Option::is_none")]
	pub tech_adr: Option<bool>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<bool>,
	#[serde(rename = "ResTp", skip_serializing_if = "Option::is_none")]
	pub res_tp: Option<bool>,
	#[serde(rename = "LckSts", skip_serializing_if = "Option::is_none")]
	pub lck_sts: Option<bool>,
	#[serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none")]
	pub mkt_spcfc_attr: Option<bool>,
}

impl PartyDataReturnCriteria2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PartyDataSearchCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyDataSearchCriteria2 {
	#[serde(rename = "OpngDt", skip_serializing_if = "Option::is_none")]
	pub opng_dt: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<SystemPartyType1Choice>,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
	#[serde(rename = "PtyId", skip_serializing_if = "Option::is_none")]
	pub pty_id: Option<PartyIdentification136>,
	#[serde(rename = "RstrctnId", skip_serializing_if = "Option::is_none")]
	pub rstrctn_id: Option<Max35Text>,
	#[serde(rename = "RstrctnIsseDt", skip_serializing_if = "Option::is_none")]
	pub rstrctn_isse_dt: Option<DateAndDateTimeSearch4Choice>,
	#[serde(rename = "ResTp", skip_serializing_if = "Option::is_none")]
	pub res_tp: Option<ResidenceType1Code>,
	#[serde(rename = "LckSts", skip_serializing_if = "Option::is_none")]
	pub lck_sts: Option<PartyLockStatus1>,
}

impl PartyDataSearchCriteria2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref opng_dt_value) = self.opng_dt { if let Err(e) = opng_dt_value.validate() { return Err(e); } }
		if let Some(ref clsg_dt_value) = self.clsg_dt { if let Err(e) = clsg_dt_value.validate() { return Err(e); } }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref rspnsbl_pty_id_value) = self.rspnsbl_pty_id { if let Err(e) = rspnsbl_pty_id_value.validate() { return Err(e); } }
		if let Some(ref pty_id_value) = self.pty_id { if let Err(e) = pty_id_value.validate() { return Err(e); } }
		if let Some(ref rstrctn_id_value) = self.rstrctn_id { if let Err(e) = rstrctn_id_value.validate() { return Err(e); } }
		if let Some(ref rstrctn_isse_dt_value) = self.rstrctn_isse_dt { if let Err(e) = rstrctn_isse_dt_value.validate() { return Err(e); } }
		if let Some(ref res_tp_value) = self.res_tp { if let Err(e) = res_tp_value.validate() { return Err(e); } }
		if let Some(ref lck_sts_value) = self.lck_sts { if let Err(e) = lck_sts_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification120Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl PartyIdentification136 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyLockStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyLockStatus1 {
	#[serde(rename = "VldFr", skip_serializing_if = "Option::is_none")]
	pub vld_fr: Option<String>,
	#[serde(rename = "Sts")]
	pub sts: LockStatus1Code,
	#[serde(rename = "LckRsn", skip_serializing_if = "Option::is_none")]
	pub lck_rsn: Option<Vec<Max35Text>>,
}

impl PartyLockStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref lck_rsn_vec) = self.lck_rsn { for item in lck_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PartyQueryV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyQueryV01 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader2>,
	#[serde(rename = "SchCrit")]
	pub sch_crit: PartyDataSearchCriteria2,
	#[serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none")]
	pub rtr_crit: Option<PartyDataReturnCriteria2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PartyQueryV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref msg_hdr_value) = self.msg_hdr { if let Err(e) = msg_hdr_value.validate() { return Err(e); } }
		if let Err(e) = self.sch_crit.validate() { return Err(e); }
		if let Some(ref rtr_crit_value) = self.rtr_crit { if let Err(e) = rtr_crit_value.validate() { return Err(e); } }
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


// RequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RequestType1Code {
	#[default]
	#[serde(rename = "RT01")]
	CodeRT01,
	#[serde(rename = "RT02")]
	CodeRT02,
	#[serde(rename = "RT03")]
	CodeRT03,
	#[serde(rename = "RT04")]
	CodeRT04,
	#[serde(rename = "RT05")]
	CodeRT05,
}

impl RequestType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RequestType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType2Choice {
	#[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
	pub pmt_ctrl: Option<RequestType1Code>,
	#[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
	pub enqry: Option<RequestType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pmt_ctrl_value) = self.pmt_ctrl { if let Err(e) = pmt_ctrl_value.validate() { return Err(e); } }
		if let Some(ref enqry_value) = self.enqry { if let Err(e) = enqry_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RequestType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RequestType2Code {
	#[default]
	#[serde(rename = "RT11")]
	CodeRT11,
	#[serde(rename = "RT12")]
	CodeRT12,
	#[serde(rename = "RT13")]
	CodeRT13,
	#[serde(rename = "RT14")]
	CodeRT14,
	#[serde(rename = "RT15")]
	CodeRT15,
}

impl RequestType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RequestedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestedIndicator {
	#[serde(rename = "$value")]
	pub requested_indicator: bool,
}

impl RequestedIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ResidenceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ResidenceType1Code {
	#[default]
	#[serde(rename = "DMST")]
	CodeDMST,
	#[serde(rename = "FRGN")]
	CodeFRGN,
	#[serde(rename = "MXED")]
	CodeMXED,
}

impl ResidenceType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// SystemPartyType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemPartyType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl SystemPartyType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}

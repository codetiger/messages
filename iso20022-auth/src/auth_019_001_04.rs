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


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref iban_value) = self.iban { if let Err(e) = iban_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl AccountSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}

impl ActiveCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}

impl ActiveCurrencyCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
		}
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


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}

impl BICFIDec2014Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.bicfi_dec2014_identifier) {
			return Err(ValidationError::new(1005, "bicfi_dec2014_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
	#[default]
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "BBSW")]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName4Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}

impl BenchmarkCurveName4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
		if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BinaryFile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BinaryFile1 {
	#[serde(rename = "MIMETp", skip_serializing_if = "Option::is_none")]
	pub mime_tp: Option<Max35Text>,
	#[serde(rename = "NcodgTp", skip_serializing_if = "Option::is_none")]
	pub ncodg_tp: Option<Max35Text>,
	#[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
	pub char_set: Option<Max35Text>,
	#[serde(rename = "InclBinryObjct", skip_serializing_if = "Option::is_none")]
	pub incl_binry_objct: Option<Max100KBinary>,
}

impl BinaryFile1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mime_tp_value) = self.mime_tp { if let Err(e) = mime_tp_value.validate() { return Err(e); } }
		if let Some(ref ncodg_tp_value) = self.ncodg_tp { if let Err(e) = ncodg_tp_value.validate() { return Err(e); } }
		if let Some(ref char_set_value) = self.char_set { if let Err(e) = char_set_value.validate() { return Err(e); } }
		if let Some(ref incl_binry_objct_value) = self.incl_binry_objct { if let Err(e) = incl_binry_objct_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData5>,
}

impl BranchAndFinancialInstitutionIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
		if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BranchData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData5 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
}

impl BranchData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CashCollateral5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashCollateral5 {
	#[serde(rename = "CollId", skip_serializing_if = "Option::is_none")]
	pub coll_id: Option<Max35Text>,
	#[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
	pub csh_acct_id: Option<AccountIdentification4Choice>,
	#[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
	pub asst_nb: Option<Max35Text>,
	#[serde(rename = "DpstAmt", skip_serializing_if = "Option::is_none")]
	pub dpst_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "DpstTp", skip_serializing_if = "Option::is_none")]
	pub dpst_tp: Option<DepositType1Code>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<String>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "CollVal")]
	pub coll_val: ActiveCurrencyAndAmount,
	#[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
	pub hrcut: Option<f64>,
}

impl CashCollateral5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref coll_id_value) = self.coll_id { if let Err(e) = coll_id_value.validate() { return Err(e); } }
		if let Some(ref csh_acct_id_value) = self.csh_acct_id { if let Err(e) = csh_acct_id_value.validate() { return Err(e); } }
		if let Some(ref asst_nb_value) = self.asst_nb { if let Err(e) = asst_nb_value.validate() { return Err(e); } }
		if let Some(ref dpst_amt_value) = self.dpst_amt { if let Err(e) = dpst_amt_value.validate() { return Err(e); } }
		if let Some(ref dpst_tp_value) = self.dpst_tp { if let Err(e) = dpst_tp_value.validate() { return Err(e); } }
		if let Err(e) = self.coll_val.validate() { return Err(e); }
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


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: Max35Text,
}

impl ClearingSystemMemberIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref clr_sys_id_value) = self.clr_sys_id { if let Err(e) = clr_sys_id_value.validate() { return Err(e); } }
		if let Err(e) = self.mmb_id.validate() { return Err(e); }
		Ok(())
	}
}


// CommunicationMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommunicationMethod4Code {
	#[default]
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "FILE")]
	CodeFILE,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "PHON")]
	CodePHON,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "PROP")]
	CodePROP,
	#[serde(rename = "SWMT")]
	CodeSWMT,
	#[serde(rename = "SWMX")]
	CodeSWMX,
}

impl CommunicationMethod4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Contact13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact13 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<PhoneNumber>,
	#[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
	pub mob_nb: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max2048Text>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max256Text>,
	#[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
	pub email_purp: Option<Max35Text>,
	#[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
	pub job_titl: Option<Max35Text>,
	#[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
	pub rspnsblty: Option<Max35Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
	pub prefrd_mtd: Option<PreferredContactMethod2Code>,
}

impl Contact13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if let Err(e) = nm_prfx_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref phne_nb_value) = self.phne_nb { if let Err(e) = phne_nb_value.validate() { return Err(e); } }
		if let Some(ref mob_nb_value) = self.mob_nb { if let Err(e) = mob_nb_value.validate() { return Err(e); } }
		if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
		if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
		if let Some(ref email_adr_value) = self.email_adr { if let Err(e) = email_adr_value.validate() { return Err(e); } }
		if let Some(ref email_purp_value) = self.email_purp { if let Err(e) = email_purp_value.validate() { return Err(e); } }
		if let Some(ref job_titl_value) = self.job_titl { if let Err(e) = job_titl_value.validate() { return Err(e); } }
		if let Some(ref rspnsblty_value) = self.rspnsblty { if let Err(e) = rspnsblty_value.validate() { return Err(e); } }
		if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref prefrd_mtd_value) = self.prefrd_mtd { if let Err(e) = prefrd_mtd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ContractBalance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractBalance1 {
	#[serde(rename = "Tp")]
	pub tp: ContractBalanceType1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebit3Code,
}

impl ContractBalance1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		Ok(())
	}
}


// ContractBalanceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractBalanceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalContractBalanceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ContractBalanceType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ContractClosureReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractClosureReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalContractClosureReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ContractClosureReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ContractCollateral1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractCollateral1 {
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "CollDesc", skip_serializing_if = "Option::is_none")]
	pub coll_desc: Option<Vec<CashCollateral5>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}

impl ContractCollateral1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ttl_amt.validate() { return Err(e); }
		if let Some(ref coll_desc_vec) = self.coll_desc { for item in coll_desc_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ContractRegistrationConfirmationV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractRegistrationConfirmationV04 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: CurrencyControlHeader7,
	#[serde(rename = "RegdCtrct")]
	pub regd_ctrct: Vec<RegisteredContract20>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ContractRegistrationConfirmationV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.grp_hdr.validate() { return Err(e); }
		for item in &self.regd_ctrct { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// CreditDebit3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CreditDebit3Code {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,
}

impl CreditDebit3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CurrencyControlHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyControlHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "NbOfItms")]
	pub nb_of_itms: Max15NumericText,
	#[serde(rename = "RcvgPty")]
	pub rcvg_pty: PartyIdentification272,
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
}

impl CurrencyControlHeader7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Err(e) = self.nb_of_itms.validate() { return Err(e); }
		if let Err(e) = self.rcvg_pty.validate() { return Err(e); }
		if let Err(e) = self.regn_agt.validate() { return Err(e); }
		Ok(())
	}
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<Max35Text>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: Max35Text,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: CountryCode,
}

impl DateAndPlaceOfBirth1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if let Err(e) = prvc_of_birth_value.validate() { return Err(e); } }
		if let Err(e) = self.city_of_birth.validate() { return Err(e); }
		if let Err(e) = self.ctry_of_birth.validate() { return Err(e); }
		Ok(())
	}
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DepositType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DepositType1Code {
	#[default]
	#[serde(rename = "FITE")]
	CodeFITE,
	#[serde(rename = "CALL")]
	CodeCALL,
}

impl DepositType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DocumentGeneralInformation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentGeneralInformation5 {
	#[serde(rename = "DocTp")]
	pub doc_tp: ExternalDocumentType1Code,
	#[serde(rename = "DocNb")]
	pub doc_nb: Max35Text,
	#[serde(rename = "DocNm", skip_serializing_if = "Option::is_none")]
	pub doc_nm: Option<Max140Text>,
	#[serde(rename = "SndrRcvrSeqId", skip_serializing_if = "Option::is_none")]
	pub sndr_rcvr_seq_id: Option<Max140Text>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
	pub url: Option<Max256Text>,
	#[serde(rename = "LkFileHash", skip_serializing_if = "Option::is_none")]
	pub lk_file_hash: Option<SignatureEnvelopeReference>,
	#[serde(rename = "AttchdBinryFile")]
	pub attchd_binry_file: BinaryFile1,
}

impl DocumentGeneralInformation5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.doc_tp.validate() { return Err(e); }
		if let Err(e) = self.doc_nb.validate() { return Err(e); }
		if let Some(ref doc_nm_value) = self.doc_nm { if let Err(e) = doc_nm_value.validate() { return Err(e); } }
		if let Some(ref sndr_rcvr_seq_id_value) = self.sndr_rcvr_seq_id { if let Err(e) = sndr_rcvr_seq_id_value.validate() { return Err(e); } }
		if let Some(ref url_value) = self.url { if let Err(e) = url_value.validate() { return Err(e); } }
		if let Some(ref lk_file_hash_value) = self.lk_file_hash { if let Err(e) = lk_file_hash_value.validate() { return Err(e); } }
		if let Err(e) = self.attchd_binry_file.validate() { return Err(e); }
		Ok(())
	}
}


// DocumentIdentification22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification22 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "DtOfIsse", skip_serializing_if = "Option::is_none")]
	pub dt_of_isse: Option<String>,
}

impl DocumentIdentification22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		Ok(())
	}
}


// DocumentIdentification28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification28 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "DtOfIsse")]
	pub dt_of_isse: String,
}

impl DocumentIdentification28 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentIdentification29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification29 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "DtOfIsse")]
	pub dt_of_isse: String,
}

impl DocumentIdentification29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		Ok(())
	}
}


// Exact1NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact1NumericText {
	#[serde(rename = "$value")]
	pub exact1_numeric_text: String,
}

impl Exact1NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]").unwrap();
		if !pattern.is_match(&self.exact1_numeric_text) {
			return Err(ValidationError::new(1005, "exact1_numeric_text does not match the required pattern".to_string()));
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


// ExchangeRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRate1 {
	#[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
	pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
	pub rate_tp: Option<ExchangeRateType1Code>,
	#[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
	pub ctrct_id: Option<Max35Text>,
}

impl ExchangeRate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref unit_ccy_value) = self.unit_ccy { if let Err(e) = unit_ccy_value.validate() { return Err(e); } }
		if let Some(ref rate_tp_value) = self.rate_tp { if let Err(e) = rate_tp_value.validate() { return Err(e); } }
		if let Some(ref ctrct_id_value) = self.ctrct_id { if let Err(e) = ctrct_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExchangeRateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExchangeRateType1Code {
	#[default]
	#[serde(rename = "SPOT")]
	CodeSPOT,
	#[serde(rename = "SALE")]
	CodeSALE,
	#[serde(rename = "AGRD")]
	CodeAGRD,
}

impl ExchangeRateType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}

impl ExternalAccountIdentification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_account_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_account_identification1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_account_identification1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_account_identification1_code exceeds the maximum length of 4".to_string()));
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


// ExternalContractBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalContractBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_contract_balance_type1_code: String,
}

impl ExternalContractBalanceType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_contract_balance_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_contract_balance_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_contract_balance_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_contract_balance_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalContractClosureReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalContractClosureReason1Code {
	#[serde(rename = "$value")]
	pub external_contract_closure_reason1_code: String,
}

impl ExternalContractClosureReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_contract_closure_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_contract_closure_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_contract_closure_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_contract_closure_reason1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalDocumentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "$value")]
	pub external_document_type1_code: String,
}

impl ExternalDocumentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_document_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_document_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_document_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}

impl ExternalFinancialInstitutionIdentification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_financial_institution_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_institution_identification1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_financial_institution_identification1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_financial_institution_identification1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
	pub external_organisation_identification1_code: String,
}

impl ExternalOrganisationIdentification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_organisation_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_organisation_identification1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_organisation_identification1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_organisation_identification1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
	pub external_person_identification1_code: String,
}

impl ExternalPersonIdentification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_person_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_person_identification1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_person_identification1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_person_identification1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl FinancialIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstitutionIdentification23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification23 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
		if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if let Err(e) = clr_sys_mmb_id_value.validate() { return Err(e); } }
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FloatingInterestRate4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate4 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName4Choice,
	#[serde(rename = "Term")]
	pub term: InterestRateContractTerm1,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
}

impl FloatingInterestRate4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ref_rate.validate() { return Err(e); }
		if let Err(e) = self.term.validate() { return Err(e); }
		Ok(())
	}
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max34Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericFinancialIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericFinancialIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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


// GenericOrganisationIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification3 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericOrganisationIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericPersonIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}

impl IBAN2007Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
		if !pattern.is_match(&self.iban2007_identifier) {
			return Err(ValidationError::new(1005, "iban2007_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}

impl ISINOct2015Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
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


// InterestPaymentDateRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestPaymentDateRange1 {
	#[serde(rename = "IntrstSchdlId", skip_serializing_if = "Option::is_none")]
	pub intrst_schdl_id: Option<Max35Text>,
	#[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
}

impl InterestPaymentDateRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref intrst_schdl_id_value) = self.intrst_schdl_id { if let Err(e) = intrst_schdl_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InterestPaymentSchedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestPaymentSchedule1 {
	#[serde(rename = "IntrstSchdlId", skip_serializing_if = "Option::is_none")]
	pub intrst_schdl_id: Option<Max35Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}

impl InterestPaymentSchedule1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref intrst_schdl_id_value) = self.intrst_schdl_id { if let Err(e) = intrst_schdl_id_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InterestRate2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate2Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingInterestRate4>,
}

impl InterestRate2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fltg_value) = self.fltg { if let Err(e) = fltg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InterestRateContractTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm1 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl InterestRateContractTerm1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.unit.validate() { return Err(e); }
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


// LegalOrganisation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalOrganisation2 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "EstblishmtDt", skip_serializing_if = "Option::is_none")]
	pub estblishmt_dt: Option<String>,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
}

impl LegalOrganisation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// LoanContract4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanContract4 {
	#[serde(rename = "CtrctDocId")]
	pub ctrct_doc_id: DocumentIdentification22,
	#[serde(rename = "LnTpId", skip_serializing_if = "Option::is_none")]
	pub ln_tp_id: Option<Max35Text>,
	#[serde(rename = "Buyr")]
	pub buyr: Vec<TradeParty6>,
	#[serde(rename = "Sellr")]
	pub sellr: Vec<TradeParty6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "PrlngtnFlg", skip_serializing_if = "Option::is_none")]
	pub prlngtn_flg: Option<bool>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "SpclConds", skip_serializing_if = "Option::is_none")]
	pub spcl_conds: Option<SpecialCondition1>,
	#[serde(rename = "DrtnCd", skip_serializing_if = "Option::is_none")]
	pub drtn_cd: Option<Exact1NumericText>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<InterestRate2Choice>,
	#[serde(rename = "Trch", skip_serializing_if = "Option::is_none")]
	pub trch: Option<Vec<LoanContractTranche1>>,
	#[serde(rename = "PmtSchdl", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl: Option<Vec<PaymentSchedule1>>,
	#[serde(rename = "IntrstSchdl", skip_serializing_if = "Option::is_none")]
	pub intrst_schdl: Option<Vec<InterestPaymentSchedule1>>,
	#[serde(rename = "IntraCpnyLn")]
	pub intra_cpny_ln: bool,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<ContractCollateral1>,
	#[serde(rename = "SndctdLn", skip_serializing_if = "Option::is_none")]
	pub sndctd_ln: Option<Vec<SyndicatedLoan3>>,
	#[serde(rename = "Attchmnt", skip_serializing_if = "Option::is_none")]
	pub attchmnt: Option<Vec<DocumentGeneralInformation5>>,
}

impl LoanContract4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ctrct_doc_id.validate() { return Err(e); }
		if let Some(ref ln_tp_id_value) = self.ln_tp_id { if let Err(e) = ln_tp_id_value.validate() { return Err(e); } }
		for item in &self.buyr { if let Err(e) = item.validate() { return Err(e); } }
		for item in &self.sellr { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if let Err(e) = sttlm_ccy_value.validate() { return Err(e); } }
		if let Some(ref spcl_conds_value) = self.spcl_conds { if let Err(e) = spcl_conds_value.validate() { return Err(e); } }
		if let Some(ref drtn_cd_value) = self.drtn_cd { if let Err(e) = drtn_cd_value.validate() { return Err(e); } }
		if let Some(ref intrst_rate_value) = self.intrst_rate { if let Err(e) = intrst_rate_value.validate() { return Err(e); } }
		if let Some(ref trch_vec) = self.trch { for item in trch_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref pmt_schdl_vec) = self.pmt_schdl { for item in pmt_schdl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref intrst_schdl_vec) = self.intrst_schdl { for item in intrst_schdl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref coll_value) = self.coll { if let Err(e) = coll_value.validate() { return Err(e); } }
		if let Some(ref sndctd_ln_vec) = self.sndctd_ln { for item in sndctd_ln_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref attchmnt_vec) = self.attchmnt { for item in attchmnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// LoanContractTranche1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanContractTranche1 {
	#[serde(rename = "TrchNb")]
	pub trch_nb: f64,
	#[serde(rename = "XpctdDt")]
	pub xpctd_dt: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
	#[serde(rename = "DrtnCd", skip_serializing_if = "Option::is_none")]
	pub drtn_cd: Option<Exact1NumericText>,
	#[serde(rename = "LastTrchInd", skip_serializing_if = "Option::is_none")]
	pub last_trch_ind: Option<bool>,
}

impl LoanContractTranche1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref drtn_cd_value) = self.drtn_cd { if let Err(e) = drtn_cd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Max100KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max100KBinary {
	#[serde(rename = "$value")]
	pub max100_k_binary: String,
}

impl Max100KBinary {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max100_k_binary.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max100_k_binary is shorter than the minimum length of 1".to_string()));
		}
		if self.max100_k_binary.chars().count() > 102400 {
			return Err(ValidationError::new(1002, "max100_k_binary exceeds the maximum length of 102400".to_string()));
		}
		Ok(())
	}
}


// Max1025Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max1025Text {
	#[serde(rename = "$value")]
	pub max1025_text: String,
}

impl Max1025Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max1025_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max1025_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max1025_text.chars().count() > 1025 {
			return Err(ValidationError::new(1002, "max1025_text exceeds the maximum length of 1025".to_string()));
		}
		Ok(())
	}
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max128Text {
	#[serde(rename = "$value")]
	pub max128_text: String,
}

impl Max128Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max128_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max128_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max128_text.chars().count() > 128 {
			return Err(ValidationError::new(1002, "max128_text exceeds the maximum length of 128".to_string()));
		}
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


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}

impl Max15NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_numeric_text) {
			return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
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


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}

impl Max2048Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max2048_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max2048_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max2048_text.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "max2048_text exceeds the maximum length of 2048".to_string()));
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
}

impl Max25Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max25_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max25_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max25_text.chars().count() > 25 {
			return Err(ValidationError::new(1002, "max25_text exceeds the maximum length of 25".to_string()));
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


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}

impl Max4Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max4_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max4_text.chars().count() > 4 {
			return Err(ValidationError::new(1002, "max4_text exceeds the maximum length of 4".to_string()));
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


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamePrefix2Code {
	#[default]
	#[serde(rename = "DOCT")]
	CodeDOCT,
	#[serde(rename = "MADM")]
	CodeMADM,
	#[serde(rename = "MISS")]
	CodeMISS,
	#[serde(rename = "MIST")]
	CodeMIST,
	#[serde(rename = "MIKS")]
	CodeMIKS,
}

impl NamePrefix2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrganisationIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification39 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}

impl OrganisationIdentification39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalOrganisationIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl OrganisationIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: Max4Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max128Text>,
}

impl OtherContact1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.chanl_tp.validate() { return Err(e); }
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Party52Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party52Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification18>,
}

impl Party52Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref org_id_value) = self.org_id { if let Err(e) = org_id_value.validate() { return Err(e); } }
		if let Some(ref prvt_id_value) = self.prvt_id { if let Err(e) = prvt_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification272 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification272 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party52Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact13>,
}

impl PartyIdentification272 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if let Err(e) = ctry_of_res_value.validate() { return Err(e); } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if let Err(e) = ctct_dtls_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentSchedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentSchedule1 {
	#[serde(rename = "PmtSchdlId", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl_id: Option<Max35Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}

impl PaymentSchedule1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pmt_schdl_id_value) = self.pmt_schdl_id { if let Err(e) = pmt_schdl_id_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentScheduleType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentScheduleType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PaymentScheduleType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PaymentScheduleType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentScheduleType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentScheduleType2Code {
	#[default]
	#[serde(rename = "CNTR")]
	CodeCNTR,
	#[serde(rename = "ESTM")]
	CodeESTM,
	#[serde(rename = "BOTH")]
	CodeBOTH,
}

impl PaymentScheduleType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PersonIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification18 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}

impl PersonIdentification18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref dt_and_plc_of_birth_value) = self.dt_and_plc_of_birth { if let Err(e) = dt_and_plc_of_birth_value.validate() { return Err(e); } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPersonIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PersonIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}

impl PhoneNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phone_number) {
			return Err(ValidationError::new(1005, "phone_number does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PostalAddress27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress27 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
	pub care_of: Option<Max140Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<Max70Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max140Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max140Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "UnitNb", skip_serializing_if = "Option::is_none")]
	pub unit_nb: Option<Max16Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max16Text>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<Max70Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max140Text>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<Max140Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max140Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
}

impl PostalAddress27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
		if let Some(ref care_of_value) = self.care_of { if let Err(e) = care_of_value.validate() { return Err(e); } }
		if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
		if let Some(ref sub_dept_value) = self.sub_dept { if let Err(e) = sub_dept_value.validate() { return Err(e); } }
		if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
		if let Some(ref bldg_nm_value) = self.bldg_nm { if let Err(e) = bldg_nm_value.validate() { return Err(e); } }
		if let Some(ref flr_value) = self.flr { if let Err(e) = flr_value.validate() { return Err(e); } }
		if let Some(ref unit_nb_value) = self.unit_nb { if let Err(e) = unit_nb_value.validate() { return Err(e); } }
		if let Some(ref pst_bx_value) = self.pst_bx { if let Err(e) = pst_bx_value.validate() { return Err(e); } }
		if let Some(ref room_value) = self.room { if let Err(e) = room_value.validate() { return Err(e); } }
		if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
		if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
		if let Some(ref twn_lctn_nm_value) = self.twn_lctn_nm { if let Err(e) = twn_lctn_nm_value.validate() { return Err(e); } }
		if let Some(ref dstrct_nm_value) = self.dstrct_nm { if let Err(e) = dstrct_nm_value.validate() { return Err(e); } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
		if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PreferredContactMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferredContactMethod2Code {
	#[default]
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "FAXX")]
	CodeFAXX,
	#[serde(rename = "LETT")]
	CodeLETT,
	#[serde(rename = "CELL")]
	CodeCELL,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "PHON")]
	CodePHON,
}

impl PreferredContactMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[default]
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,
}

impl RateBasis1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RegisteredContract20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContract20 {
	#[serde(rename = "OrgnlCtrctRegnReq", skip_serializing_if = "Option::is_none")]
	pub orgnl_ctrct_regn_req: Option<Max35Text>,
	#[serde(rename = "RptgPty")]
	pub rptg_pty: TradeParty6,
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "IssrFI")]
	pub issr_fi: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "Ctrct")]
	pub ctrct: UnderlyingContract4Choice,
	#[serde(rename = "CtrctBal", skip_serializing_if = "Option::is_none")]
	pub ctrct_bal: Option<Vec<ContractBalance1>>,
	#[serde(rename = "PmtSchdlTp", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl_tp: Option<PaymentScheduleType2Choice>,
	#[serde(rename = "RegdCtrctId")]
	pub regd_ctrct_id: DocumentIdentification29,
	#[serde(rename = "PrvsRegdCtrctId", skip_serializing_if = "Option::is_none")]
	pub prvs_regd_ctrct_id: Option<DocumentIdentification22>,
	#[serde(rename = "RegdCtrctJrnl", skip_serializing_if = "Option::is_none")]
	pub regd_ctrct_jrnl: Option<Vec<RegisteredContractJournal3>>,
	#[serde(rename = "Amdmnt", skip_serializing_if = "Option::is_none")]
	pub amdmnt: Option<Vec<RegisteredContractAmendment1>>,
	#[serde(rename = "Submissn")]
	pub submissn: RegisteredContractCommunication1,
	#[serde(rename = "Dlvry")]
	pub dlvry: RegisteredContractCommunication1,
	#[serde(rename = "LnPrncplAmt", skip_serializing_if = "Option::is_none")]
	pub ln_prncpl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "EstmtdDtInd")]
	pub estmtd_dt_ind: bool,
	#[serde(rename = "IntrCpnyLn")]
	pub intr_cpny_ln: bool,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl RegisteredContract20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgnl_ctrct_regn_req_value) = self.orgnl_ctrct_regn_req { if let Err(e) = orgnl_ctrct_regn_req_value.validate() { return Err(e); } }
		if let Err(e) = self.rptg_pty.validate() { return Err(e); }
		if let Err(e) = self.regn_agt.validate() { return Err(e); }
		if let Err(e) = self.issr_fi.validate() { return Err(e); }
		if let Err(e) = self.ctrct.validate() { return Err(e); }
		if let Some(ref ctrct_bal_vec) = self.ctrct_bal { for item in ctrct_bal_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref pmt_schdl_tp_value) = self.pmt_schdl_tp { if let Err(e) = pmt_schdl_tp_value.validate() { return Err(e); } }
		if let Err(e) = self.regd_ctrct_id.validate() { return Err(e); }
		if let Some(ref prvs_regd_ctrct_id_value) = self.prvs_regd_ctrct_id { if let Err(e) = prvs_regd_ctrct_id_value.validate() { return Err(e); } }
		if let Some(ref regd_ctrct_jrnl_vec) = self.regd_ctrct_jrnl { for item in regd_ctrct_jrnl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref amdmnt_vec) = self.amdmnt { for item in amdmnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Err(e) = self.submissn.validate() { return Err(e); }
		if let Err(e) = self.dlvry.validate() { return Err(e); }
		if let Some(ref ln_prncpl_amt_value) = self.ln_prncpl_amt { if let Err(e) = ln_prncpl_amt_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// RegisteredContractAmendment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractAmendment1 {
	#[serde(rename = "AmdmntDt")]
	pub amdmnt_dt: String,
	#[serde(rename = "Doc")]
	pub doc: DocumentIdentification28,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "AmdmntRsn", skip_serializing_if = "Option::is_none")]
	pub amdmnt_rsn: Option<Max35Text>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}

impl RegisteredContractAmendment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.doc.validate() { return Err(e); }
		if let Some(ref amdmnt_rsn_value) = self.amdmnt_rsn { if let Err(e) = amdmnt_rsn_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RegisteredContractCommunication1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractCommunication1 {
	#[serde(rename = "Mtd")]
	pub mtd: CommunicationMethod4Code,
	#[serde(rename = "Dt")]
	pub dt: String,
}

impl RegisteredContractCommunication1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.mtd.validate() { return Err(e); }
		Ok(())
	}
}


// RegisteredContractJournal3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractJournal3 {
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "UnqId", skip_serializing_if = "Option::is_none")]
	pub unq_id: Option<DocumentIdentification28>,
	#[serde(rename = "ClsrDt")]
	pub clsr_dt: String,
	#[serde(rename = "ClsrRsn")]
	pub clsr_rsn: ContractClosureReason1Choice,
}

impl RegisteredContractJournal3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.regn_agt.validate() { return Err(e); }
		if let Some(ref unq_id_value) = self.unq_id { if let Err(e) = unq_id_value.validate() { return Err(e); } }
		if let Err(e) = self.clsr_rsn.validate() { return Err(e); }
		Ok(())
	}
}


// ShipmentDateRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentDateRange1 {
	#[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub earlst_shipmnt_dt: Option<String>,
	#[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub latst_shipmnt_dt: Option<String>,
}

impl ShipmentDateRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ShipmentDateRange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentDateRange2 {
	#[serde(rename = "SubQtyVal")]
	pub sub_qty_val: f64,
	#[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub earlst_shipmnt_dt: Option<String>,
	#[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub latst_shipmnt_dt: Option<String>,
}

impl ShipmentDateRange2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ShipmentSchedule2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentSchedule2Choice {
	#[serde(rename = "ShipmntDtRg", skip_serializing_if = "Option::is_none")]
	pub shipmnt_dt_rg: Option<ShipmentDateRange1>,
	#[serde(rename = "ShipmntSubSchdl", skip_serializing_if = "Option::is_none")]
	pub shipmnt_sub_schdl: Option<Vec<ShipmentDateRange2>>,
}

impl ShipmentSchedule2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref shipmnt_dt_rg_value) = self.shipmnt_dt_rg { if let Err(e) = shipmnt_dt_rg_value.validate() { return Err(e); } }
		if let Some(ref shipmnt_sub_schdl_vec) = self.shipmnt_sub_schdl { for item in shipmnt_sub_schdl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SignatureEnvelopeReference ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SignatureEnvelopeReference {
}

impl SignatureEnvelopeReference {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SpecialCondition1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecialCondition1 {
	#[serde(rename = "IncmgAmt")]
	pub incmg_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "OutgngAmt")]
	pub outgng_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "IncmgAmtToOthrAcct", skip_serializing_if = "Option::is_none")]
	pub incmg_amt_to_othr_acct: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "PmtFrOthrAcct", skip_serializing_if = "Option::is_none")]
	pub pmt_fr_othr_acct: Option<ActiveCurrencyAndAmount>,
}

impl SpecialCondition1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.incmg_amt.validate() { return Err(e); }
		if let Err(e) = self.outgng_amt.validate() { return Err(e); }
		if let Some(ref incmg_amt_to_othr_acct_value) = self.incmg_amt_to_othr_acct { if let Err(e) = incmg_amt_to_othr_acct_value.validate() { return Err(e); } }
		if let Some(ref pmt_fr_othr_acct_value) = self.pmt_fr_othr_acct { if let Err(e) = pmt_fr_othr_acct_value.validate() { return Err(e); } }
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


// SyndicatedLoan3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SyndicatedLoan3 {
	#[serde(rename = "Brrwr")]
	pub brrwr: TradeParty6,
	#[serde(rename = "Lndr", skip_serializing_if = "Option::is_none")]
	pub lndr: Option<TradeParty6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Shr", skip_serializing_if = "Option::is_none")]
	pub shr: Option<f64>,
	#[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_inf: Option<ExchangeRate1>,
}

impl SyndicatedLoan3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.brrwr.validate() { return Err(e); }
		if let Some(ref lndr_value) = self.lndr { if let Err(e) = lndr_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref xchg_rate_inf_value) = self.xchg_rate_inf { if let Err(e) = xchg_rate_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxExemptReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxExemptReason1Code {
	#[default]
	#[serde(rename = "NONE")]
	CodeNONE,
	#[serde(rename = "MASA")]
	CodeMASA,
	#[serde(rename = "MISA")]
	CodeMISA,
	#[serde(rename = "SISA")]
	CodeSISA,
	#[serde(rename = "IISA")]
	CodeIISA,
	#[serde(rename = "CUYP")]
	CodeCUYP,
	#[serde(rename = "PRYP")]
	CodePRYP,
	#[serde(rename = "ASTR")]
	CodeASTR,
	#[serde(rename = "EMPY")]
	CodeEMPY,
	#[serde(rename = "EMCY")]
	CodeEMCY,
	#[serde(rename = "EPRY")]
	CodeEPRY,
	#[serde(rename = "ECYE")]
	CodeECYE,
	#[serde(rename = "NFPI")]
	CodeNFPI,
	#[serde(rename = "NFQP")]
	CodeNFQP,
	#[serde(rename = "DECP")]
	CodeDECP,
	#[serde(rename = "IRAC")]
	CodeIRAC,
	#[serde(rename = "IRAR")]
	CodeIRAR,
	#[serde(rename = "KEOG")]
	CodeKEOG,
	#[serde(rename = "PFSP")]
	CodePFSP,
	#[serde(rename = "401K")]
	Code401K,
	#[serde(rename = "SIRA")]
	CodeSIRA,
	#[serde(rename = "403B")]
	Code403B,
	#[serde(rename = "457X")]
	Code457X,
	#[serde(rename = "RIRA")]
	CodeRIRA,
	#[serde(rename = "RIAN")]
	CodeRIAN,
	#[serde(rename = "RCRF")]
	CodeRCRF,
	#[serde(rename = "RCIP")]
	CodeRCIP,
	#[serde(rename = "EIFP")]
	CodeEIFP,
	#[serde(rename = "EIOP")]
	CodeEIOP,
}

impl TaxExemptReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxExemptionReasonFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptionReasonFormat1Choice {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Max140Text>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<TaxExemptReason1Code>,
}

impl TaxExemptionReasonFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ustrd_value) = self.ustrd { if let Err(e) = ustrd_value.validate() { return Err(e); } }
		if let Some(ref strd_value) = self.strd { if let Err(e) = strd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxParty4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty4 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxXmptnRsn", skip_serializing_if = "Option::is_none")]
	pub tax_xmptn_rsn: Option<Vec<TaxExemptionReasonFormat1Choice>>,
}

impl TaxParty4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tax_id_value) = self.tax_id { if let Err(e) = tax_id_value.validate() { return Err(e); } }
		if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
		if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
		if let Some(ref tax_xmptn_rsn_vec) = self.tax_xmptn_rsn { for item in tax_xmptn_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TradeContract4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeContract4 {
	#[serde(rename = "CtrctDocId", skip_serializing_if = "Option::is_none")]
	pub ctrct_doc_id: Option<DocumentIdentification22>,
	#[serde(rename = "TradTpId", skip_serializing_if = "Option::is_none")]
	pub trad_tp_id: Option<Max35Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Buyr")]
	pub buyr: Vec<TradeParty6>,
	#[serde(rename = "Sellr")]
	pub sellr: Vec<TradeParty6>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "PrlngtnFlg", skip_serializing_if = "Option::is_none")]
	pub prlngtn_flg: Option<bool>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_inf: Option<ExchangeRate1>,
	#[serde(rename = "PmtSchdl", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl: Option<InterestPaymentDateRange1>,
	#[serde(rename = "ShipmntSchdl", skip_serializing_if = "Option::is_none")]
	pub shipmnt_schdl: Option<ShipmentSchedule2Choice>,
	#[serde(rename = "Attchmnt", skip_serializing_if = "Option::is_none")]
	pub attchmnt: Option<Vec<DocumentGeneralInformation5>>,
}

impl TradeContract4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ctrct_doc_id_value) = self.ctrct_doc_id { if let Err(e) = ctrct_doc_id_value.validate() { return Err(e); } }
		if let Some(ref trad_tp_id_value) = self.trad_tp_id { if let Err(e) = trad_tp_id_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		for item in &self.buyr { if let Err(e) = item.validate() { return Err(e); } }
		for item in &self.sellr { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if let Err(e) = sttlm_ccy_value.validate() { return Err(e); } }
		if let Some(ref xchg_rate_inf_value) = self.xchg_rate_inf { if let Err(e) = xchg_rate_inf_value.validate() { return Err(e); } }
		if let Some(ref pmt_schdl_value) = self.pmt_schdl { if let Err(e) = pmt_schdl_value.validate() { return Err(e); } }
		if let Some(ref shipmnt_schdl_value) = self.shipmnt_schdl { if let Err(e) = shipmnt_schdl_value.validate() { return Err(e); } }
		if let Some(ref attchmnt_vec) = self.attchmnt { for item in attchmnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TradeParty6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeParty6 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification272,
	#[serde(rename = "LglOrg", skip_serializing_if = "Option::is_none")]
	pub lgl_org: Option<LegalOrganisation2>,
	#[serde(rename = "TaxPty", skip_serializing_if = "Option::is_none")]
	pub tax_pty: Option<Vec<TaxParty4>>,
}

impl TradeParty6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_id.validate() { return Err(e); }
		if let Some(ref lgl_org_value) = self.lgl_org { if let Err(e) = lgl_org_value.validate() { return Err(e); } }
		if let Some(ref tax_pty_vec) = self.tax_pty { for item in tax_pty_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingContract4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingContract4Choice {
	#[serde(rename = "Ln", skip_serializing_if = "Option::is_none")]
	pub ln: Option<LoanContract4>,
	#[serde(rename = "Trad", skip_serializing_if = "Option::is_none")]
	pub trad: Option<TradeContract4>,
}

impl UnderlyingContract4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ln_value) = self.ln { if let Err(e) = ln_value.validate() { return Err(e); } }
		if let Some(ref trad_value) = self.trad { if let Err(e) = trad_value.validate() { return Err(e); } }
		Ok(())
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}

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


// AccountContract3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountContract3 {
	#[serde(rename = "TrgtGoLiveDt", skip_serializing_if = "Option::is_none")]
	pub trgt_go_live_dt: Option<String>,
	#[serde(rename = "TrgtClsgDt", skip_serializing_if = "Option::is_none")]
	pub trgt_clsg_dt: Option<String>,
	#[serde(rename = "GoLiveDt", skip_serializing_if = "Option::is_none")]
	pub go_live_dt: Option<String>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<String>,
	#[serde(rename = "UrgcyFlg", skip_serializing_if = "Option::is_none")]
	pub urgcy_flg: Option<bool>,
	#[serde(rename = "RmvlInd", skip_serializing_if = "Option::is_none")]
	pub rmvl_ind: Option<bool>,
}

impl AccountContract3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AccountForAction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountForAction1 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveCurrencyCode,
}

impl AccountForAction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.ccy.validate() { return Err(e); }
		Ok(())
	}
}


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


// AccountReport36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountReport36 {
	#[serde(rename = "Acct")]
	pub acct: CustomerAccount5,
	#[serde(rename = "UndrlygMstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub undrlyg_mstr_agrmt: Option<ContractDocument1>,
	#[serde(rename = "CtrctDts", skip_serializing_if = "Option::is_none")]
	pub ctrct_dts: Option<AccountContract3>,
	#[serde(rename = "Mndt", skip_serializing_if = "Option::is_none")]
	pub mndt: Option<Vec<OperationMandate7>>,
	#[serde(rename = "Grp", skip_serializing_if = "Option::is_none")]
	pub grp: Option<Vec<Group6>>,
	#[serde(rename = "RefAcct", skip_serializing_if = "Option::is_none")]
	pub ref_acct: Option<CashAccount40>,
	#[serde(rename = "BalTrfAcct", skip_serializing_if = "Option::is_none")]
	pub bal_trf_acct: Option<AccountForAction1>,
	#[serde(rename = "TrfAcctSvcrId", skip_serializing_if = "Option::is_none")]
	pub trf_acct_svcr_id: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl AccountReport36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.acct.validate() { return Err(e); }
		if let Some(ref undrlyg_mstr_agrmt_value) = self.undrlyg_mstr_agrmt { if let Err(e) = undrlyg_mstr_agrmt_value.validate() { return Err(e); } }
		if let Some(ref ctrct_dts_value) = self.ctrct_dts { if let Err(e) = ctrct_dts_value.validate() { return Err(e); } }
		if let Some(ref mndt_vec) = self.mndt { for item in mndt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref grp_vec) = self.grp { for item in grp_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref ref_acct_value) = self.ref_acct { if let Err(e) = ref_acct_value.validate() { return Err(e); } }
		if let Some(ref bal_trf_acct_value) = self.bal_trf_acct { if let Err(e) = bal_trf_acct_value.validate() { return Err(e); } }
		if let Some(ref trf_acct_svcr_id_value) = self.trf_acct_svcr_id { if let Err(e) = trf_acct_svcr_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AccountReportV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountReportV05 {
	#[serde(rename = "Refs")]
	pub refs: References5,
	#[serde(rename = "Fr", skip_serializing_if = "Option::is_none")]
	pub fr: Option<OrganisationIdentification39>,
	#[serde(rename = "AcctSvcrId")]
	pub acct_svcr_id: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "Org")]
	pub org: Organisation42,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<AccountReport36>>,
	#[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
	pub dgtl_sgntr: Option<Vec<PartyAndSignature4>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl AccountReportV05 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.refs.validate() { return Err(e); }
		if let Some(ref fr_value) = self.fr { if let Err(e) = fr_value.validate() { return Err(e); } }
		if let Err(e) = self.acct_svcr_id.validate() { return Err(e); }
		if let Err(e) = self.org.validate() { return Err(e); }
		if let Some(ref rpt_vec) = self.rpt { for item in rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref dgtl_sgntr_vec) = self.dgtl_sgntr { for item in dgtl_sgntr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// AccountStatus3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountStatus3Code {
	#[default]
	#[serde(rename = "ENAB")]
	CodeENAB,
	#[serde(rename = "DISA")]
	CodeDISA,
	#[serde(rename = "DELE")]
	CodeDELE,
	#[serde(rename = "FORM")]
	CodeFORM,
}

impl AccountStatus3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// Authorisation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Authorisation2 {
	#[serde(rename = "MaxAmtByTx", skip_serializing_if = "Option::is_none")]
	pub max_amt_by_tx: Option<FixedAmountOrUnlimited1Choice>,
	#[serde(rename = "MaxAmtByPrd", skip_serializing_if = "Option::is_none")]
	pub max_amt_by_prd: Option<Vec<MaximumAmountByPeriod1>>,
	#[serde(rename = "MaxAmtByBlkSubmissn", skip_serializing_if = "Option::is_none")]
	pub max_amt_by_blk_submissn: Option<FixedAmountOrUnlimited1Choice>,
}

impl Authorisation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref max_amt_by_tx_value) = self.max_amt_by_tx { if let Err(e) = max_amt_by_tx_value.validate() { return Err(e); } }
		if let Some(ref max_amt_by_prd_vec) = self.max_amt_by_prd { for item in max_amt_by_prd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref max_amt_by_blk_submissn_value) = self.max_amt_by_blk_submissn { if let Err(e) = max_amt_by_blk_submissn_value.validate() { return Err(e); } }
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


// BankTransactionCodeStructure4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure4 {
	#[serde(rename = "Domn", skip_serializing_if = "Option::is_none")]
	pub domn: Option<BankTransactionCodeStructure5>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}

impl BankTransactionCodeStructure4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref domn_value) = self.domn { if let Err(e) = domn_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BankTransactionCodeStructure5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure5 {
	#[serde(rename = "Cd")]
	pub cd: ExternalBankTransactionDomain1Code,
	#[serde(rename = "Fmly")]
	pub fmly: BankTransactionCodeStructure6,
}

impl BankTransactionCodeStructure5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cd.validate() { return Err(e); }
		if let Err(e) = self.fmly.validate() { return Err(e); }
		Ok(())
	}
}


// BankTransactionCodeStructure6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure6 {
	#[serde(rename = "Cd")]
	pub cd: ExternalBankTransactionFamily1Code,
	#[serde(rename = "SubFmlyCd")]
	pub sub_fmly_cd: ExternalBankTransactionSubFamily1Code,
}

impl BankTransactionCodeStructure6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cd.validate() { return Err(e); }
		if let Err(e) = self.sub_fmly_cd.validate() { return Err(e); }
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


// CashAccount40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount40 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<AccountIdentification4Choice>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref prxy_value) = self.prxy { if let Err(e) = prxy_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CashAccountType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Channel2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Channel2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CommunicationMethod3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl Channel2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// CodeOrProprietary1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CodeOrProprietary1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max4Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification13>,
}

impl CodeOrProprietary1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CommunicationFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationFormat1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCommunicationFormat1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CommunicationFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CommunicationMethod2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CommunicationMethod2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CommunicationMethod2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CommunicationMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommunicationMethod2Code {
	#[default]
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "FILE")]
	CodeFILE,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "POST")]
	CodePOST,
}

impl CommunicationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CommunicationMethod3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommunicationMethod3Code {
	#[default]
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "PHON")]
	CodePHON,
	#[serde(rename = "FILE")]
	CodeFILE,
	#[serde(rename = "ONLI")]
	CodeONLI,
}

impl CommunicationMethod3Code {
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


// ContractDocument1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractDocument1 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "SgnOffDt", skip_serializing_if = "Option::is_none")]
	pub sgn_off_dt: Option<String>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<Max6Text>,
}

impl ContractDocument1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ref_attr.validate() { return Err(e); }
		if let Some(ref vrsn_value) = self.vrsn { if let Err(e) = vrsn_value.validate() { return Err(e); } }
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


// CustomerAccount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomerAccount5 {
	#[serde(rename = "Id")]
	pub id: Vec<AccountIdentification4Choice>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<AccountStatus3Code>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveCurrencyCode,
	#[serde(rename = "MnthlyPmtVal", skip_serializing_if = "Option::is_none")]
	pub mnthly_pmt_val: Option<f64>,
	#[serde(rename = "MnthlyRcvdVal", skip_serializing_if = "Option::is_none")]
	pub mnthly_rcvd_val: Option<f64>,
	#[serde(rename = "MnthlyTxNb", skip_serializing_if = "Option::is_none")]
	pub mnthly_tx_nb: Option<Max5NumericText>,
	#[serde(rename = "AvrgBal", skip_serializing_if = "Option::is_none")]
	pub avrg_bal: Option<f64>,
	#[serde(rename = "AcctPurp", skip_serializing_if = "Option::is_none")]
	pub acct_purp: Option<Max140Text>,
	#[serde(rename = "FlrNtfctnAmt", skip_serializing_if = "Option::is_none")]
	pub flr_ntfctn_amt: Option<f64>,
	#[serde(rename = "ClngNtfctnAmt", skip_serializing_if = "Option::is_none")]
	pub clng_ntfctn_amt: Option<f64>,
	#[serde(rename = "StmtFrqcyAndFrmt", skip_serializing_if = "Option::is_none")]
	pub stmt_frqcy_and_frmt: Option<Vec<StatementFrequencyAndForm1>>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<String>,
	#[serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none")]
	pub rstrctn: Option<Vec<Restriction1>>,
}

impl CustomerAccount5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Err(e) = self.ccy.validate() { return Err(e); }
		if let Some(ref mnthly_tx_nb_value) = self.mnthly_tx_nb { if let Err(e) = mnthly_tx_nb_value.validate() { return Err(e); } }
		if let Some(ref acct_purp_value) = self.acct_purp { if let Err(e) = acct_purp_value.validate() { return Err(e); } }
		if let Some(ref stmt_frqcy_and_frmt_vec) = self.stmt_frqcy_and_frmt { for item in stmt_frqcy_and_frmt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rstrctn_vec) = self.rstrctn { for item in rstrctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// ExternalBankTransactionDomain1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionDomain1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_domain1_code: String,
}

impl ExternalBankTransactionDomain1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_bank_transaction_domain1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_bank_transaction_domain1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_bank_transaction_domain1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_bank_transaction_domain1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalBankTransactionFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionFamily1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_family1_code: String,
}

impl ExternalBankTransactionFamily1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_bank_transaction_family1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_bank_transaction_family1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_bank_transaction_family1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_bank_transaction_family1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalBankTransactionSubFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionSubFamily1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_sub_family1_code: String,
}

impl ExternalBankTransactionSubFamily1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_bank_transaction_sub_family1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_bank_transaction_sub_family1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_bank_transaction_sub_family1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_bank_transaction_sub_family1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}

impl ExternalCashAccountType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_cash_account_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_cash_account_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_cash_account_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_cash_account_type1_code exceeds the maximum length of 4".to_string()));
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


// ExternalCommunicationFormat1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCommunicationFormat1Code {
	#[serde(rename = "$value")]
	pub external_communication_format1_code: String,
}

impl ExternalCommunicationFormat1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_communication_format1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_communication_format1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_communication_format1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_communication_format1_code exceeds the maximum length of 4".to_string()));
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


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}

impl ExternalProxyAccountType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_proxy_account_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_proxy_account_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_proxy_account_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_proxy_account_type1_code exceeds the maximum length of 4".to_string()));
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


// FixedAmountOrUnlimited1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedAmountOrUnlimited1Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "NotLtd", skip_serializing_if = "Option::is_none")]
	pub not_ltd: Option<Unlimited9Text>,
}

impl FixedAmountOrUnlimited1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref not_ltd_value) = self.not_ltd { if let Err(e) = not_ltd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Frequency7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency7Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "TEND")]
	CodeTEND,
	#[serde(rename = "MOVE")]
	CodeMOVE,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "INDA")]
	CodeINDA,
}

impl Frequency7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// GenericIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification13 {
	#[serde(rename = "Id")]
	pub id: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
}

impl GenericIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.issr.validate() { return Err(e); }
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


// Group6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Group6 {
	#[serde(rename = "GrpId")]
	pub grp_id: Max4AlphaNumericText,
	#[serde(rename = "Pty")]
	pub pty: Vec<PartyAndCertificate6>,
}

impl Group6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.grp_id.validate() { return Err(e); }
		for item in &self.pty { if let Err(e) = item.validate() { return Err(e); } }
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


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}

impl ImpliedCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.implied_currency_and_amount < 0.000000 {
			return Err(ValidationError::new(1003, "implied_currency_and_amount is less than the minimum value of 0.000000".to_string()));
		}
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


// Max10KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max10KBinary {
	#[serde(rename = "$value")]
	pub max10_k_binary: String,
}

impl Max10KBinary {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max10_k_binary.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max10_k_binary is shorter than the minimum length of 1".to_string()));
		}
		if self.max10_k_binary.chars().count() > 10240 {
			return Err(ValidationError::new(1002, "max10_k_binary exceeds the maximum length of 10240".to_string()));
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


// Max15PlusSignedNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15PlusSignedNumericText {
	#[serde(rename = "$value")]
	pub max15_plus_signed_numeric_text: String,
}

impl Max15PlusSignedNumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_plus_signed_numeric_text) {
			return Err(ValidationError::new(1005, "max15_plus_signed_numeric_text does not match the required pattern".to_string()));
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


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3NumericText {
	#[serde(rename = "$value")]
	pub max3_numeric_text: String,
}

impl Max3NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.max3_numeric_text) {
			return Err(ValidationError::new(1005, "max3_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_alpha_numeric_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return Err(ValidationError::new(1002, "max4_alpha_numeric_text exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
			return Err(ValidationError::new(1005, "max4_alpha_numeric_text does not match the required pattern".to_string()));
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


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}

impl Max5NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.max5_numeric_text) {
			return Err(ValidationError::new(1005, "max5_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max6Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max6Text {
	#[serde(rename = "$value")]
	pub max6_text: String,
}

impl Max6Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max6_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max6_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max6_text.chars().count() > 6 {
			return Err(ValidationError::new(1002, "max6_text exceeds the maximum length of 6".to_string()));
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


// MaximumAmountByPeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaximumAmountByPeriod1 {
	#[serde(rename = "MaxAmt")]
	pub max_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "NbOfDays")]
	pub nb_of_days: Max3NumericText,
}

impl MaximumAmountByPeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.max_amt.validate() { return Err(e); }
		if let Err(e) = self.nb_of_days.validate() { return Err(e); }
		Ok(())
	}
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
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


// OperationMandate7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OperationMandate7 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "AplblChanl")]
	pub aplbl_chanl: Vec<Channel2Choice>,
	#[serde(rename = "ReqrdSgntrNb")]
	pub reqrd_sgntr_nb: Max15PlusSignedNumericText,
	#[serde(rename = "SgntrOrdrInd")]
	pub sgntr_ordr_ind: bool,
	#[serde(rename = "MndtHldr", skip_serializing_if = "Option::is_none")]
	pub mndt_hldr: Option<Vec<PartyAndAuthorisation7>>,
	#[serde(rename = "BkOpr")]
	pub bk_opr: Vec<BankTransactionCodeStructure4>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
}

impl OperationMandate7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		for item in &self.aplbl_chanl { if let Err(e) = item.validate() { return Err(e); } }
		if let Err(e) = self.reqrd_sgntr_nb.validate() { return Err(e); }
		if let Some(ref mndt_hldr_vec) = self.mndt_hldr { for item in mndt_hldr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		for item in &self.bk_opr { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// Organisation42 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation42 {
	#[serde(rename = "FullLglNm")]
	pub full_lgl_nm: Max350Text,
	#[serde(rename = "TradgNm", skip_serializing_if = "Option::is_none")]
	pub tradg_nm: Option<Max350Text>,
	#[serde(rename = "CtryOfOpr")]
	pub ctry_of_opr: CountryCode,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
	#[serde(rename = "OprlAdr", skip_serializing_if = "Option::is_none")]
	pub oprl_adr: Option<PostalAddress27>,
	#[serde(rename = "BizAdr", skip_serializing_if = "Option::is_none")]
	pub biz_adr: Option<PostalAddress27>,
	#[serde(rename = "LglAdr")]
	pub lgl_adr: PostalAddress27,
	#[serde(rename = "BllgAdr", skip_serializing_if = "Option::is_none")]
	pub bllg_adr: Option<PostalAddress27>,
	#[serde(rename = "OrgId")]
	pub org_id: OrganisationIdentification39,
	#[serde(rename = "RprtvOffcr", skip_serializing_if = "Option::is_none")]
	pub rprtv_offcr: Option<Vec<PartyIdentification274>>,
	#[serde(rename = "TrsrMgr", skip_serializing_if = "Option::is_none")]
	pub trsr_mgr: Option<PartyIdentification274>,
	#[serde(rename = "MainMndtHldr", skip_serializing_if = "Option::is_none")]
	pub main_mndt_hldr: Option<Vec<PartyIdentification274>>,
	#[serde(rename = "Sndr", skip_serializing_if = "Option::is_none")]
	pub sndr: Option<Vec<PartyIdentification274>>,
	#[serde(rename = "LglRprtv", skip_serializing_if = "Option::is_none")]
	pub lgl_rprtv: Option<Vec<PartyIdentification274>>,
}

impl Organisation42 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.full_lgl_nm.validate() { return Err(e); }
		if let Some(ref tradg_nm_value) = self.tradg_nm { if let Err(e) = tradg_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.ctry_of_opr.validate() { return Err(e); }
		if let Some(ref oprl_adr_value) = self.oprl_adr { if let Err(e) = oprl_adr_value.validate() { return Err(e); } }
		if let Some(ref biz_adr_value) = self.biz_adr { if let Err(e) = biz_adr_value.validate() { return Err(e); } }
		if let Err(e) = self.lgl_adr.validate() { return Err(e); }
		if let Some(ref bllg_adr_value) = self.bllg_adr { if let Err(e) = bllg_adr_value.validate() { return Err(e); } }
		if let Err(e) = self.org_id.validate() { return Err(e); }
		if let Some(ref rprtv_offcr_vec) = self.rprtv_offcr { for item in rprtv_offcr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref trsr_mgr_value) = self.trsr_mgr { if let Err(e) = trsr_mgr_value.validate() { return Err(e); } }
		if let Some(ref main_mndt_hldr_vec) = self.main_mndt_hldr { for item in main_mndt_hldr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref sndr_vec) = self.sndr { for item in sndr_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref lgl_rprtv_vec) = self.lgl_rprtv { for item in lgl_rprtv_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// PartyAndAuthorisation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndAuthorisation7 {
	#[serde(rename = "PtyOrGrp")]
	pub pty_or_grp: PartyOrGroup3Choice,
	#[serde(rename = "SgntrOrdr", skip_serializing_if = "Option::is_none")]
	pub sgntr_ordr: Option<Max15PlusSignedNumericText>,
	#[serde(rename = "Authstn")]
	pub authstn: Authorisation2,
}

impl PartyAndAuthorisation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty_or_grp.validate() { return Err(e); }
		if let Some(ref sgntr_ordr_value) = self.sgntr_ordr { if let Err(e) = sgntr_ordr_value.validate() { return Err(e); } }
		if let Err(e) = self.authstn.validate() { return Err(e); }
		Ok(())
	}
}


// PartyAndCertificate6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndCertificate6 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification272,
	#[serde(rename = "Cert", skip_serializing_if = "Option::is_none")]
	pub cert: Option<Max10KBinary>,
}

impl PartyAndCertificate6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty.validate() { return Err(e); }
		if let Some(ref cert_value) = self.cert { if let Err(e) = cert_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyAndSignature4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndSignature4 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification272,
	#[serde(rename = "Sgntr")]
	pub sgntr: SkipPayload,
}

impl PartyAndSignature4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty.validate() { return Err(e); }
		if let Err(e) = self.sgntr.validate() { return Err(e); }
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


// PartyIdentification274 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification274 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<PersonIdentification18>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact13>,
}

impl PartyIdentification274 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if let Err(e) = ctry_of_res_value.validate() { return Err(e); } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if let Err(e) = ctct_dtls_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyOrGroup3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyOrGroup3Choice {
	#[serde(rename = "GrpId", skip_serializing_if = "Option::is_none")]
	pub grp_id: Option<Max4AlphaNumericText>,
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyAndCertificate6>,
}

impl PartyOrGroup3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref grp_id_value) = self.grp_id { if let Err(e) = grp_id_value.validate() { return Err(e); } }
		if let Some(ref pty_value) = self.pty { if let Err(e) = pty_value.validate() { return Err(e); } }
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


// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[serde(rename = "Cd")]
	pub cd: Max35Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl ProprietaryBankTransactionCodeStructure1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cd.validate() { return Err(e); }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: Max2048Text,
}

impl ProxyAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Err(e) = self.id.validate() { return Err(e); }
		Ok(())
	}
}


// ProxyAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalProxyAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ProxyAccountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// References5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct References5 {
	#[serde(rename = "ReqTp")]
	pub req_tp: UseCases1Code,
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PrcId")]
	pub prc_id: MessageIdentification1,
	#[serde(rename = "AckdMsgId", skip_serializing_if = "Option::is_none")]
	pub ackd_msg_id: Option<Vec<MessageIdentification1>>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<Max35Text>,
	#[serde(rename = "AttchdDocNm", skip_serializing_if = "Option::is_none")]
	pub attchd_doc_nm: Option<Vec<Max70Text>>,
}

impl References5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.req_tp.validate() { return Err(e); }
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Err(e) = self.prc_id.validate() { return Err(e); }
		if let Some(ref ackd_msg_id_vec) = self.ackd_msg_id { for item in ackd_msg_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
		if let Some(ref attchd_doc_nm_vec) = self.attchd_doc_nm { for item in attchd_doc_nm_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Restriction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Restriction1 {
	#[serde(rename = "RstrctnTp")]
	pub rstrctn_tp: CodeOrProprietary1Choice,
	#[serde(rename = "VldFr")]
	pub vld_fr: String,
	#[serde(rename = "VldUntil", skip_serializing_if = "Option::is_none")]
	pub vld_until: Option<String>,
}

impl Restriction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rstrctn_tp.validate() { return Err(e); }
		Ok(())
	}
}


// SkipPayload ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SkipPayload {
}

impl SkipPayload {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// StatementFrequencyAndForm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementFrequencyAndForm1 {
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency7Code,
	#[serde(rename = "ComMtd")]
	pub com_mtd: CommunicationMethod2Choice,
	#[serde(rename = "DlvryAdr")]
	pub dlvry_adr: Max350Text,
	#[serde(rename = "Frmt")]
	pub frmt: CommunicationFormat1Choice,
}

impl StatementFrequencyAndForm1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.frqcy.validate() { return Err(e); }
		if let Err(e) = self.com_mtd.validate() { return Err(e); }
		if let Err(e) = self.dlvry_adr.validate() { return Err(e); }
		if let Err(e) = self.frmt.validate() { return Err(e); }
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


// Unlimited9Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Unlimited9Text {
	#[serde(rename = "$value")]
	pub unlimited9_text: String,
}

impl Unlimited9Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.unlimited9_text.chars().count() < 9 {
			return Err(ValidationError::new(1001, "unlimited9_text is shorter than the minimum length of 9".to_string()));
		}
		if self.unlimited9_text.chars().count() > 9 {
			return Err(ValidationError::new(1002, "unlimited9_text exceeds the maximum length of 9".to_string()));
		}
		let pattern = Regex::new("UNLIMITED").unwrap();
		if !pattern.is_match(&self.unlimited9_text) {
			return Err(ValidationError::new(1005, "unlimited9_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// UseCases1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UseCases1Code {
	#[default]
	#[serde(rename = "OPEN")]
	CodeOPEN,
	#[serde(rename = "MNTN")]
	CodeMNTN,
	#[serde(rename = "CLSG")]
	CodeCLSG,
	#[serde(rename = "VIEW")]
	CodeVIEW,
}

impl UseCases1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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

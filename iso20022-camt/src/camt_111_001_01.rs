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


// AdjustmentCompensation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdjustmentCompensation1 {
	#[serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none")]
	pub initl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "DueChrgs", skip_serializing_if = "Option::is_none")]
	pub due_chrgs: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AmtDue")]
	pub amt_due: ActiveCurrencyAndAmount,
	#[serde(rename = "CompstnAgt", skip_serializing_if = "Option::is_none")]
	pub compstn_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "CompstnAcct", skip_serializing_if = "Option::is_none")]
	pub compstn_acct: Option<CashAccount40>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<DatePeriod5>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max140Text>,
}

impl AdjustmentCompensation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref initl_amt_value) = self.initl_amt { if let Err(e) = initl_amt_value.validate() { return Err(e); } }
		if let Some(ref due_chrgs_value) = self.due_chrgs { if let Err(e) = due_chrgs_value.validate() { return Err(e); } }
		if let Err(e) = self.amt_due.validate() { return Err(e); }
		if let Some(ref compstn_agt_value) = self.compstn_agt { if let Err(e) = compstn_agt_value.validate() { return Err(e); } }
		if let Some(ref compstn_acct_value) = self.compstn_acct { if let Err(e) = compstn_acct_value.validate() { return Err(e); } }
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AmendmentInformationDetails14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmendmentInformationDetails14 {
	#[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
	pub orgnl_mndt_id: Option<Max35Text>,
	#[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
	#[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr: Option<PartyIdentification135>,
	#[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none")]
	pub orgnl_fnl_colltn_dt: Option<String>,
	#[serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none")]
	pub orgnl_frqcy: Option<Frequency36Choice>,
	#[serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none")]
	pub orgnl_rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none")]
	pub orgnl_trckg_days: Option<Exact2NumericText>,
}

impl AmendmentInformationDetails14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgnl_mndt_id_value) = self.orgnl_mndt_id { if let Err(e) = orgnl_mndt_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_cdtr_schme_id_value) = self.orgnl_cdtr_schme_id { if let Err(e) = orgnl_cdtr_schme_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_cdtr_agt_value) = self.orgnl_cdtr_agt { if let Err(e) = orgnl_cdtr_agt_value.validate() { return Err(e); } }
		if let Some(ref orgnl_cdtr_agt_acct_value) = self.orgnl_cdtr_agt_acct { if let Err(e) = orgnl_cdtr_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref orgnl_dbtr_value) = self.orgnl_dbtr { if let Err(e) = orgnl_dbtr_value.validate() { return Err(e); } }
		if let Some(ref orgnl_dbtr_acct_value) = self.orgnl_dbtr_acct { if let Err(e) = orgnl_dbtr_acct_value.validate() { return Err(e); } }
		if let Some(ref orgnl_dbtr_agt_value) = self.orgnl_dbtr_agt { if let Err(e) = orgnl_dbtr_agt_value.validate() { return Err(e); } }
		if let Some(ref orgnl_dbtr_agt_acct_value) = self.orgnl_dbtr_agt_acct { if let Err(e) = orgnl_dbtr_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref orgnl_frqcy_value) = self.orgnl_frqcy { if let Err(e) = orgnl_frqcy_value.validate() { return Err(e); } }
		if let Some(ref orgnl_rsn_value) = self.orgnl_rsn { if let Err(e) = orgnl_rsn_value.validate() { return Err(e); } }
		if let Some(ref orgnl_trckg_days_value) = self.orgnl_trckg_days { if let Err(e) = orgnl_trckg_days_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AmountType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountType4Choice {
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none")]
	pub eqvt_amt: Option<EquivalentAmount2>,
}

impl AmountType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref instd_amt_value) = self.instd_amt { if let Err(e) = instd_amt_value.validate() { return Err(e); } }
		if let Some(ref eqvt_amt_value) = self.eqvt_amt { if let Err(e) = eqvt_amt_value.validate() { return Err(e); } }
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


// BookingConfirmation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BookingConfirmation1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "Acct")]
	pub acct: CashAccount40,
	#[serde(rename = "BookgDt", skip_serializing_if = "Option::is_none")]
	pub bookg_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ValDt")]
	pub val_dt: DateAndDateTime2Choice,
	#[serde(rename = "Refs")]
	pub refs: TransactionReferences6,
	#[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
	pub chrgs: Option<Charges6>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max140Text>,
}

impl BookingConfirmation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		if let Err(e) = self.acct.validate() { return Err(e); }
		if let Some(ref bookg_dt_value) = self.bookg_dt { if let Err(e) = bookg_dt_value.validate() { return Err(e); } }
		if let Err(e) = self.val_dt.validate() { return Err(e); }
		if let Err(e) = self.refs.validate() { return Err(e); }
		if let Some(ref chrgs_value) = self.chrgs { if let Err(e) = chrgs_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData3>,
}

impl BranchAndFinancialInstitutionIdentification6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
		if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BranchData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData3 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
}

impl BranchData3 {
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


// CategoryPurpose1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCategoryPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CategoryPurpose1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ChargeBearerType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ChargeBearerType1Code {
	#[default]
	#[serde(rename = "DEBT")]
	CodeDEBT,
	#[serde(rename = "CRED")]
	CodeCRED,
	#[serde(rename = "SHAR")]
	CodeSHAR,
	#[serde(rename = "SLEV")]
	CodeSLEV,
}

impl ChargeBearerType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ChargeIncludedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChargeIncludedIndicator {
	#[serde(rename = "$value")]
	pub charge_included_indicator: bool,
}

impl ChargeIncludedIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ChargeType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalChargeType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification3>,
}

impl ChargeType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Charges6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charges6 {
	#[serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<ChargesRecord3>>,
}

impl Charges6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ttl_chrgs_and_tax_amt_value) = self.ttl_chrgs_and_tax_amt { if let Err(e) = ttl_chrgs_and_tax_amt_value.validate() { return Err(e); } }
		if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ChargesRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargesRecord3 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none")]
	pub chrg_incl_ind: Option<bool>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ChargeType3Choice>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Br", skip_serializing_if = "Option::is_none")]
	pub br: Option<ChargeBearerType1Code>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxCharges2>,
}

impl ChargesRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref br_value) = self.br { if let Err(e) = br_value.validate() { return Err(e); } }
		if let Some(ref agt_value) = self.agt { if let Err(e) = agt_value.validate() { return Err(e); } }
		if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ClearingChannel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingChannel2Code {
	#[default]
	#[serde(rename = "RTGS")]
	CodeRTGS,
	#[serde(rename = "RTNS")]
	CodeRTNS,
	#[serde(rename = "MPNS")]
	CodeMPNS,
	#[serde(rename = "BOOK")]
	CodeBOOK,
}

impl ClearingChannel2Code {
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


// ClearingSystemIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashClearingSystem1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ClearingSystemIdentification3Choice {
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


// CompensationResponse1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompensationResponse1 {
	#[serde(rename = "Grantd")]
	pub grantd: bool,
	#[serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none")]
	pub initl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "PdChrgs", skip_serializing_if = "Option::is_none")]
	pub pd_chrgs: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AmtDue", skip_serializing_if = "Option::is_none")]
	pub amt_due: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdValDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_val_dt: Option<String>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<DatePeriod2>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max140Text>,
}

impl CompensationResponse1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref initl_amt_value) = self.initl_amt { if let Err(e) = initl_amt_value.validate() { return Err(e); } }
		if let Some(ref pd_chrgs_value) = self.pd_chrgs { if let Err(e) = pd_chrgs_value.validate() { return Err(e); } }
		if let Some(ref amt_due_value) = self.amt_due { if let Err(e) = amt_due_value.validate() { return Err(e); } }
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Contact4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact4 {
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
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max2048Text>,
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
	pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}

impl Contact4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if let Err(e) = nm_prfx_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref phne_nb_value) = self.phne_nb { if let Err(e) = phne_nb_value.validate() { return Err(e); } }
		if let Some(ref mob_nb_value) = self.mob_nb { if let Err(e) = mob_nb_value.validate() { return Err(e); } }
		if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
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


// CreditDebitCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CreditDebitCode {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,
}

impl CreditDebitCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CreditTransferMandateData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditTransferMandateData1 {
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<Max35Text>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<MandateTypeInformation2>,
	#[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "DtOfVrfctn", skip_serializing_if = "Option::is_none")]
	pub dt_of_vrfctn: Option<String>,
	#[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_sgntr: Option<Max10KBinary>,
	#[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
	pub frst_pmt_dt: Option<String>,
	#[serde(rename = "FnlPmtDt", skip_serializing_if = "Option::is_none")]
	pub fnl_pmt_dt: Option<String>,
	#[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
	pub frqcy: Option<Frequency36Choice>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<MandateSetupReason1Choice>,
}

impl CreditTransferMandateData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mndt_id_value) = self.mndt_id { if let Err(e) = mndt_id_value.validate() { return Err(e); } }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref elctrnc_sgntr_value) = self.elctrnc_sgntr { if let Err(e) = elctrnc_sgntr_value.validate() { return Err(e); } }
		if let Some(ref frqcy_value) = self.frqcy { if let Err(e) = frqcy_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CreditorReferenceInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<Max35Text>,
}

impl CreditorReferenceInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref ref_attr_value) = self.ref_attr { if let Err(e) = ref_attr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CreditorReferenceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DocumentType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CreditorReferenceType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CreditorReferenceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl CreditorReferenceType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// DatePeriod5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod5 {
	#[serde(rename = "CurValDt")]
	pub cur_val_dt: String,
	#[serde(rename = "ReqdValDt")]
	pub reqd_val_dt: String,
}

impl DatePeriod5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DebitAuthorisationConfirmation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebitAuthorisationConfirmation3 {
	#[serde(rename = "DbtAuthstn")]
	pub dbt_authstn: bool,
	#[serde(rename = "AmtToDbt", skip_serializing_if = "Option::is_none")]
	pub amt_to_dbt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<CashAccount40>,
	#[serde(rename = "ValDtToDbt", skip_serializing_if = "Option::is_none")]
	pub val_dt_to_dbt: Option<String>,
	#[serde(rename = "CmonTxId", skip_serializing_if = "Option::is_none")]
	pub cmon_tx_id: Option<Max52Text>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max140Text>,
}

impl DebitAuthorisationConfirmation3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref amt_to_dbt_value) = self.amt_to_dbt { if let Err(e) = amt_to_dbt_value.validate() { return Err(e); } }
		if let Some(ref acct_value) = self.acct { if let Err(e) = acct_value.validate() { return Err(e); } }
		if let Some(ref cmon_tx_id_value) = self.cmon_tx_id { if let Err(e) = cmon_tx_id_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
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


// DiscountAmountAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiscountAmountAndType1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DiscountAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl DiscountAmountAndType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// DiscountAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiscountAmountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDiscountAmountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl DiscountAmountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Document12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Document12 {
	#[serde(rename = "Tp")]
	pub tp: DocumentType1Choice,
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IsseDt")]
	pub isse_dt: DateAndDateTime2Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "LangCd", skip_serializing_if = "Option::is_none")]
	pub lang_cd: Option<String>,
	#[serde(rename = "Frmt")]
	pub frmt: DocumentFormat1Choice,
	#[serde(rename = "FileNm", skip_serializing_if = "Option::is_none")]
	pub file_nm: Option<Max140Text>,
	#[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
	pub dgtl_sgntr: Option<PartyAndSignature3>,
	#[serde(rename = "Nclsr")]
	pub nclsr: Max10MbBinary,
}

impl Document12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.isse_dt.validate() { return Err(e); }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Err(e) = self.frmt.validate() { return Err(e); }
		if let Some(ref file_nm_value) = self.file_nm { if let Err(e) = file_nm_value.validate() { return Err(e); } }
		if let Some(ref dgtl_sgntr_value) = self.dgtl_sgntr { if let Err(e) = dgtl_sgntr_value.validate() { return Err(e); } }
		if let Err(e) = self.nclsr.validate() { return Err(e); }
		Ok(())
	}
}


// DocumentAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAdjustment1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max4Text>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max140Text>,
}

impl DocumentAdjustment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentFormat1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentFormat1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl DocumentFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentLineIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentLineType1>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<String>,
}

impl DocumentLineIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentLineInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation1 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max2048Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<RemittanceAmount3>,
}

impl DocumentLineInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentLineType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl DocumentLineType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentLineType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentLineType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl DocumentLineType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl DocumentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DocumentType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DocumentType3Code {
	#[default]
	#[serde(rename = "RADM")]
	CodeRADM,
	#[serde(rename = "RPIN")]
	CodeRPIN,
	#[serde(rename = "FXDR")]
	CodeFXDR,
	#[serde(rename = "DISP")]
	CodeDISP,
	#[serde(rename = "PUOR")]
	CodePUOR,
	#[serde(rename = "SCOR")]
	CodeSCOR,
}

impl DocumentType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DocumentType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DocumentType6Code {
	#[default]
	#[serde(rename = "MSIN")]
	CodeMSIN,
	#[serde(rename = "CNFA")]
	CodeCNFA,
	#[serde(rename = "DNFA")]
	CodeDNFA,
	#[serde(rename = "CINV")]
	CodeCINV,
	#[serde(rename = "CREN")]
	CodeCREN,
	#[serde(rename = "DEBN")]
	CodeDEBN,
	#[serde(rename = "HIRI")]
	CodeHIRI,
	#[serde(rename = "SBIN")]
	CodeSBIN,
	#[serde(rename = "CMCN")]
	CodeCMCN,
	#[serde(rename = "SOAC")]
	CodeSOAC,
	#[serde(rename = "DISP")]
	CodeDISP,
	#[serde(rename = "BOLD")]
	CodeBOLD,
	#[serde(rename = "VCHR")]
	CodeVCHR,
	#[serde(rename = "AROI")]
	CodeAROI,
	#[serde(rename = "TSUT")]
	CodeTSUT,
	#[serde(rename = "PUOR")]
	CodePUOR,
}

impl DocumentType6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EquivalentAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquivalentAmount2 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyOfTrf")]
	pub ccy_of_trf: ActiveOrHistoricCurrencyCode,
}

impl EquivalentAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.ccy_of_trf.validate() { return Err(e); }
		Ok(())
	}
}


// Exact2NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact2NumericText {
	#[serde(rename = "$value")]
	pub exact2_numeric_text: String,
}

impl Exact2NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{2}").unwrap();
		if !pattern.is_match(&self.exact2_numeric_text) {
			return Err(ValidationError::new(1005, "exact2_numeric_text does not match the required pattern".to_string()));
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


// ExternalCashClearingSystem1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCashClearingSystem1Code {
	#[serde(rename = "$value")]
	pub external_cash_clearing_system1_code: String,
}

impl ExternalCashClearingSystem1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_cash_clearing_system1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_cash_clearing_system1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_cash_clearing_system1_code.chars().count() > 3 {
			return Err(ValidationError::new(1002, "external_cash_clearing_system1_code exceeds the maximum length of 3".to_string()));
		}
		Ok(())
	}
}


// ExternalCategoryPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCategoryPurpose1Code {
	#[serde(rename = "$value")]
	pub external_category_purpose1_code: String,
}

impl ExternalCategoryPurpose1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_category_purpose1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_category_purpose1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_category_purpose1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_category_purpose1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalChargeType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalChargeType1Code {
	#[serde(rename = "$value")]
	pub external_charge_type1_code: String,
}

impl ExternalChargeType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_charge_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_charge_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_charge_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_charge_type1_code exceeds the maximum length of 4".to_string()));
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


// ExternalDiscountAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDiscountAmountType1Code {
	#[serde(rename = "$value")]
	pub external_discount_amount_type1_code: String,
}

impl ExternalDiscountAmountType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_discount_amount_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_discount_amount_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_discount_amount_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_discount_amount_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalDocumentFormat1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentFormat1Code {
	#[serde(rename = "$value")]
	pub external_document_format1_code: String,
}

impl ExternalDocumentFormat1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_document_format1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_format1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_document_format1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_document_format1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalDocumentLineType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentLineType1Code {
	#[serde(rename = "$value")]
	pub external_document_line_type1_code: String,
}

impl ExternalDocumentLineType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_document_line_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_line_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_document_line_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_document_line_type1_code exceeds the maximum length of 4".to_string()));
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


// ExternalGarnishmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalGarnishmentType1Code {
	#[serde(rename = "$value")]
	pub external_garnishment_type1_code: String,
}

impl ExternalGarnishmentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_garnishment_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_garnishment_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_garnishment_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_garnishment_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationAction1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationAction1Code {
	#[serde(rename = "$value")]
	pub external_investigation_action1_code: String,
}

impl ExternalInvestigationAction1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_action1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_action1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_action1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_action1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationActionReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationActionReason1Code {
	#[serde(rename = "$value")]
	pub external_investigation_action_reason1_code: String,
}

impl ExternalInvestigationActionReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_action_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_action_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_action_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_action_reason1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationInstrument1Code {
	#[serde(rename = "$value")]
	pub external_investigation_instrument1_code: String,
}

impl ExternalInvestigationInstrument1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_instrument1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_instrument1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_instrument1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_instrument1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationReason1Code {
	#[serde(rename = "$value")]
	pub external_investigation_reason1_code: String,
}

impl ExternalInvestigationReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_reason1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationReasonSubType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationReasonSubType1Code {
	#[serde(rename = "$value")]
	pub external_investigation_reason_sub_type1_code: String,
}

impl ExternalInvestigationReasonSubType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_reason_sub_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_reason_sub_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_reason_sub_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_reason_sub_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationServiceLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationServiceLevel1Code {
	#[serde(rename = "$value")]
	pub external_investigation_service_level1_code: String,
}

impl ExternalInvestigationServiceLevel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_service_level1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_service_level1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_service_level1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_service_level1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationStatus1Code {
	#[serde(rename = "$value")]
	pub external_investigation_status1_code: String,
}

impl ExternalInvestigationStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_status1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_status1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_status1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_status1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationStatusReason1Code {
	#[serde(rename = "$value")]
	pub external_investigation_status_reason1_code: String,
}

impl ExternalInvestigationStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_status_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_status_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_status_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_status_reason1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationSubType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationSubType1Code {
	#[serde(rename = "$value")]
	pub external_investigation_sub_type1_code: String,
}

impl ExternalInvestigationSubType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_sub_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_sub_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_sub_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_sub_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalInvestigationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalInvestigationType1Code {
	#[serde(rename = "$value")]
	pub external_investigation_type1_code: String,
}

impl ExternalInvestigationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_investigation_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_investigation_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_investigation_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_investigation_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalLocalInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalLocalInstrument1Code {
	#[serde(rename = "$value")]
	pub external_local_instrument1_code: String,
}

impl ExternalLocalInstrument1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_local_instrument1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_local_instrument1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_local_instrument1_code.chars().count() > 35 {
			return Err(ValidationError::new(1002, "external_local_instrument1_code exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// ExternalMandateSetupReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalMandateSetupReason1Code {
	#[serde(rename = "$value")]
	pub external_mandate_setup_reason1_code: String,
}

impl ExternalMandateSetupReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_mandate_setup_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_mandate_setup_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_mandate_setup_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_mandate_setup_reason1_code exceeds the maximum length of 4".to_string()));
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


// ExternalPaymentTransactionStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPaymentTransactionStatus1Code {
	#[serde(rename = "$value")]
	pub external_payment_transaction_status1_code: String,
}

impl ExternalPaymentTransactionStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_payment_transaction_status1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_payment_transaction_status1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_payment_transaction_status1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_payment_transaction_status1_code exceeds the maximum length of 4".to_string()));
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


// ExternalPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPurpose1Code {
	#[serde(rename = "$value")]
	pub external_purpose1_code: String,
}

impl ExternalPurpose1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_purpose1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_purpose1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_purpose1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_purpose1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalServiceLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalServiceLevel1Code {
	#[serde(rename = "$value")]
	pub external_service_level1_code: String,
}

impl ExternalServiceLevel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_service_level1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_service_level1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_service_level1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_service_level1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalStatusReason1Code {
	#[serde(rename = "$value")]
	pub external_status_reason1_code: String,
}

impl ExternalStatusReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_status_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_status_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_status_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_status_reason1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalTaxAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalTaxAmountType1Code {
	#[serde(rename = "$value")]
	pub external_tax_amount_type1_code: String,
}

impl ExternalTaxAmountType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_tax_amount_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_tax_amount_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_tax_amount_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_tax_amount_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// FileData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileData1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentType1Choice>,
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
	pub frmt: Option<DocumentFormat1Choice>,
	#[serde(rename = "FileNm", skip_serializing_if = "Option::is_none")]
	pub file_nm: Option<Max140Text>,
	#[serde(rename = "NtwkRef", skip_serializing_if = "Option::is_none")]
	pub ntwk_ref: Option<Max140Text>,
	#[serde(rename = "FileLctnElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub file_lctn_elctrnc_adr: Option<Max2048Text>,
}

impl FileData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref isse_dt_value) = self.isse_dt { if let Err(e) = isse_dt_value.validate() { return Err(e); } }
		if let Some(ref frmt_value) = self.frmt { if let Err(e) = frmt_value.validate() { return Err(e); } }
		if let Some(ref file_nm_value) = self.file_nm { if let Err(e) = file_nm_value.validate() { return Err(e); } }
		if let Some(ref ntwk_ref_value) = self.ntwk_ref { if let Err(e) = ntwk_ref_value.validate() { return Err(e); } }
		if let Some(ref file_lctn_elctrnc_adr_value) = self.file_lctn_elctrnc_adr { if let Err(e) = file_lctn_elctrnc_adr_value.validate() { return Err(e); } }
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


// FinancialInstitutionIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification18 {
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


// Frequency36Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency36Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Frequency6Code>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<FrequencyPeriod1>,
	#[serde(rename = "PtInTm", skip_serializing_if = "Option::is_none")]
	pub pt_in_tm: Option<FrequencyAndMoment1>,
}

impl Frequency36Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Some(ref pt_in_tm_value) = self.pt_in_tm { if let Err(e) = pt_in_tm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Frequency6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency6Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "FRTN")]
	CodeFRTN,
}

impl Frequency6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FrequencyAndMoment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FrequencyAndMoment1 {
	#[serde(rename = "Tp")]
	pub tp: Frequency6Code,
	#[serde(rename = "PtInTm")]
	pub pt_in_tm: Exact2NumericText,
}

impl FrequencyAndMoment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.pt_in_tm.validate() { return Err(e); }
		Ok(())
	}
}


// FrequencyPeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FrequencyPeriod1 {
	#[serde(rename = "Tp")]
	pub tp: Frequency6Code,
	#[serde(rename = "CntPerPrd")]
	pub cnt_per_prd: f64,
}

impl FrequencyPeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		Ok(())
	}
}


// Garnishment3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Garnishment3 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
	pub grnshee: Option<PartyIdentification135>,
	#[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
	pub grnshmt_admstr: Option<PartyIdentification135>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<Max140Text>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
	pub mplyee_termntn_ind: Option<bool>,
}

impl Garnishment3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Some(ref grnshee_value) = self.grnshee { if let Err(e) = grnshee_value.validate() { return Err(e); } }
		if let Some(ref grnshmt_admstr_value) = self.grnshmt_admstr { if let Err(e) = grnshmt_admstr_value.validate() { return Err(e); } }
		if let Some(ref ref_nb_value) = self.ref_nb { if let Err(e) = ref_nb_value.validate() { return Err(e); } }
		if let Some(ref rmtd_amt_value) = self.rmtd_amt { if let Err(e) = rmtd_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GarnishmentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GarnishmentType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GarnishmentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalGarnishmentType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl GarnishmentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// GenericIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
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


// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericOrganisationIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericPersonIdentification1 {
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


// ISOYear ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOYear {
	#[serde(rename = "$value")]
	pub iso_year: String,
}

impl ISOYear {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestigationActionReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationActionReason1 {
	#[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
	pub orgtr: Option<PartyIdentification135>,
	#[serde(rename = "Rsn")]
	pub rsn: InvestigationActionReason1Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<Max105Text>>,
}

impl InvestigationActionReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgtr_value) = self.orgtr { if let Err(e) = orgtr_value.validate() { return Err(e); } }
		if let Err(e) = self.rsn.validate() { return Err(e); }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// InvestigationActionReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationActionReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationActionReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationActionReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationData2 {
	#[serde(rename = "OrgnlInvstgtnSeq", skip_serializing_if = "Option::is_none")]
	pub orgnl_invstgtn_seq: Option<f64>,
	#[serde(rename = "OrgnlInvstgtnRsn", skip_serializing_if = "Option::is_none")]
	pub orgnl_invstgtn_rsn: Option<InvestigationReason1Choice>,
	#[serde(rename = "OrgnlInvstgtnRsnSubTp", skip_serializing_if = "Option::is_none")]
	pub orgnl_invstgtn_rsn_sub_tp: Option<InvestigationReasonSubType1Choice>,
	#[serde(rename = "RspnData")]
	pub rspn_data: InvestigationDataRecord1Choice,
	#[serde(rename = "RltdInvstgtnData", skip_serializing_if = "Option::is_none")]
	pub rltd_invstgtn_data: Option<RelatedInvestigationData1>,
	#[serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none")]
	pub nclsd_file: Option<Vec<Document12>>,
	#[serde(rename = "RltdFileData", skip_serializing_if = "Option::is_none")]
	pub rltd_file_data: Option<Vec<FileData1>>,
	#[serde(rename = "RspnOrgtr", skip_serializing_if = "Option::is_none")]
	pub rspn_orgtr: Option<Party40Choice>,
}

impl InvestigationData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgnl_invstgtn_rsn_value) = self.orgnl_invstgtn_rsn { if let Err(e) = orgnl_invstgtn_rsn_value.validate() { return Err(e); } }
		if let Some(ref orgnl_invstgtn_rsn_sub_tp_value) = self.orgnl_invstgtn_rsn_sub_tp { if let Err(e) = orgnl_invstgtn_rsn_sub_tp_value.validate() { return Err(e); } }
		if let Err(e) = self.rspn_data.validate() { return Err(e); }
		if let Some(ref rltd_invstgtn_data_value) = self.rltd_invstgtn_data { if let Err(e) = rltd_invstgtn_data_value.validate() { return Err(e); } }
		if let Some(ref nclsd_file_vec) = self.nclsd_file { for item in nclsd_file_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rltd_file_data_vec) = self.rltd_file_data { for item in rltd_file_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rspn_orgtr_value) = self.rspn_orgtr { if let Err(e) = rspn_orgtr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationDataRecord1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationDataRecord1Choice {
	#[serde(rename = "DbtAuthstn", skip_serializing_if = "Option::is_none")]
	pub dbt_authstn: Option<DebitAuthorisationConfirmation3>,
	#[serde(rename = "Compstn", skip_serializing_if = "Option::is_none")]
	pub compstn: Option<CompensationResponse1>,
	#[serde(rename = "Valtn", skip_serializing_if = "Option::is_none")]
	pub valtn: Option<AdjustmentCompensation1>,
	#[serde(rename = "Conf", skip_serializing_if = "Option::is_none")]
	pub conf: Option<BookingConfirmation1>,
	#[serde(rename = "TxSts", skip_serializing_if = "Option::is_none")]
	pub tx_sts: Option<PaymentTransactionStatus1>,
	#[serde(rename = "TxData", skip_serializing_if = "Option::is_none")]
	pub tx_data: Option<Vec<TransactionAmendment1>>,
	#[serde(rename = "RspnNrrtv", skip_serializing_if = "Option::is_none")]
	pub rspn_nrrtv: Option<Max500Text>,
}

impl InvestigationDataRecord1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref dbt_authstn_value) = self.dbt_authstn { if let Err(e) = dbt_authstn_value.validate() { return Err(e); } }
		if let Some(ref compstn_value) = self.compstn { if let Err(e) = compstn_value.validate() { return Err(e); } }
		if let Some(ref valtn_value) = self.valtn { if let Err(e) = valtn_value.validate() { return Err(e); } }
		if let Some(ref conf_value) = self.conf { if let Err(e) = conf_value.validate() { return Err(e); } }
		if let Some(ref tx_sts_value) = self.tx_sts { if let Err(e) = tx_sts_value.validate() { return Err(e); } }
		if let Some(ref tx_data_vec) = self.tx_data { for item in tx_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rspn_nrrtv_value) = self.rspn_nrrtv { if let Err(e) = rspn_nrrtv_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationLocationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationLocationData1 {
	#[serde(rename = "Mtd")]
	pub mtd: InvestigationLocationMethod1Code,
	#[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_adr: Option<Max2048Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<NameAndAddress16>,
}

impl InvestigationLocationData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.mtd.validate() { return Err(e); }
		if let Some(ref elctrnc_adr_value) = self.elctrnc_adr { if let Err(e) = elctrnc_adr_value.validate() { return Err(e); } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationLocationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestigationLocationMethod1Code {
	#[default]
	#[serde(rename = "EDIC")]
	CodeEDIC,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "SMSM")]
	CodeSMSM,
	#[serde(rename = "URID")]
	CodeURID,
}

impl InvestigationLocationMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestigationReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationReasonSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationReasonSubType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationReasonSubType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationReasonSubType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationRequest3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequest3 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "RqstrInvstgtnId", skip_serializing_if = "Option::is_none")]
	pub rqstr_invstgtn_id: Option<Max35Text>,
	#[serde(rename = "RspndrInvstgtnId", skip_serializing_if = "Option::is_none")]
	pub rspndr_invstgtn_id: Option<Max35Text>,
	#[serde(rename = "EIR", skip_serializing_if = "Option::is_none")]
	pub eir: Option<UUIDv4Identifier>,
	#[serde(rename = "ReqActn", skip_serializing_if = "Option::is_none")]
	pub req_actn: Option<InvestigationRequestAction1>,
	#[serde(rename = "InvstgtnTp")]
	pub invstgtn_tp: InvestigationType1Choice,
	#[serde(rename = "InvstgtnSubTp", skip_serializing_if = "Option::is_none")]
	pub invstgtn_sub_tp: Option<InvestigationSubType1Choice>,
	#[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm: Option<UnderlyingInvestigationInstrument1Choice>,
	#[serde(rename = "Undrlyg", skip_serializing_if = "Option::is_none")]
	pub undrlyg: Option<UnderlyingData2Choice>,
	#[serde(rename = "Rqstr")]
	pub rqstr: Party40Choice,
	#[serde(rename = "Rspndr")]
	pub rspndr: Party40Choice,
	#[serde(rename = "ReqOrgtr", skip_serializing_if = "Option::is_none")]
	pub req_orgtr: Option<Party40Choice>,
	#[serde(rename = "XpctdRspndr", skip_serializing_if = "Option::is_none")]
	pub xpctd_rspndr: Option<Party40Choice>,
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<Vec<InvestigationServiceLevel1Choice>>,
}

impl InvestigationRequest3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref rqstr_invstgtn_id_value) = self.rqstr_invstgtn_id { if let Err(e) = rqstr_invstgtn_id_value.validate() { return Err(e); } }
		if let Some(ref rspndr_invstgtn_id_value) = self.rspndr_invstgtn_id { if let Err(e) = rspndr_invstgtn_id_value.validate() { return Err(e); } }
		if let Some(ref eir_value) = self.eir { if let Err(e) = eir_value.validate() { return Err(e); } }
		if let Some(ref req_actn_value) = self.req_actn { if let Err(e) = req_actn_value.validate() { return Err(e); } }
		if let Err(e) = self.invstgtn_tp.validate() { return Err(e); }
		if let Some(ref invstgtn_sub_tp_value) = self.invstgtn_sub_tp { if let Err(e) = invstgtn_sub_tp_value.validate() { return Err(e); } }
		if let Some(ref undrlyg_instrm_value) = self.undrlyg_instrm { if let Err(e) = undrlyg_instrm_value.validate() { return Err(e); } }
		if let Some(ref undrlyg_value) = self.undrlyg { if let Err(e) = undrlyg_value.validate() { return Err(e); } }
		if let Err(e) = self.rqstr.validate() { return Err(e); }
		if let Err(e) = self.rspndr.validate() { return Err(e); }
		if let Some(ref req_orgtr_value) = self.req_orgtr { if let Err(e) = req_orgtr_value.validate() { return Err(e); } }
		if let Some(ref xpctd_rspndr_value) = self.xpctd_rspndr { if let Err(e) = xpctd_rspndr_value.validate() { return Err(e); } }
		if let Some(ref svc_lvl_vec) = self.svc_lvl { for item in svc_lvl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// InvestigationRequestAction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequestAction1 {
	#[serde(rename = "Actn")]
	pub actn: InvestigationRequestAction1Choice,
	#[serde(rename = "ActnRsn", skip_serializing_if = "Option::is_none")]
	pub actn_rsn: Option<InvestigationActionReason1>,
}

impl InvestigationRequestAction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.actn.validate() { return Err(e); }
		if let Some(ref actn_rsn_value) = self.actn_rsn { if let Err(e) = actn_rsn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationRequestAction1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequestAction1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationAction1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationRequestAction1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationResponse3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationResponse3 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "RspndrInvstgtnId", skip_serializing_if = "Option::is_none")]
	pub rspndr_invstgtn_id: Option<Max35Text>,
	#[serde(rename = "InvstgtnSts")]
	pub invstgtn_sts: InvestigationStatus2,
	#[serde(rename = "NxtRspndr", skip_serializing_if = "Option::is_none")]
	pub nxt_rspndr: Option<Party40Choice>,
	#[serde(rename = "InvstgtnData", skip_serializing_if = "Option::is_none")]
	pub invstgtn_data: Option<Vec<InvestigationData2>>,
}

impl InvestigationResponse3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref rspndr_invstgtn_id_value) = self.rspndr_invstgtn_id { if let Err(e) = rspndr_invstgtn_id_value.validate() { return Err(e); } }
		if let Err(e) = self.invstgtn_sts.validate() { return Err(e); }
		if let Some(ref nxt_rspndr_value) = self.nxt_rspndr { if let Err(e) = nxt_rspndr_value.validate() { return Err(e); } }
		if let Some(ref invstgtn_data_vec) = self.invstgtn_data { for item in invstgtn_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// InvestigationResponseV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationResponseV01 {
	#[serde(rename = "InvstgtnRspn")]
	pub invstgtn_rspn: InvestigationResponse3,
	#[serde(rename = "OrgnlInvstgtnReq")]
	pub orgnl_invstgtn_req: InvestigationRequest3,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl InvestigationResponseV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.invstgtn_rspn.validate() { return Err(e); }
		if let Err(e) = self.orgnl_invstgtn_req.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// InvestigationServiceLevel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationServiceLevel1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationServiceLevel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationServiceLevel1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationStatus2 {
	#[serde(rename = "Sts")]
	pub sts: ExternalInvestigationStatus1Code,
	#[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
	pub sts_rsn: Option<InvestigationStatusReason1Choice>,
}

impl InvestigationStatus2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref sts_rsn_value) = self.sts_rsn { if let Err(e) = sts_rsn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationStatusReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationSubType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationSubType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationSubType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestigationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InvestigationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// LanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageCode {
	#[serde(rename = "$value")]
	pub language_code: String,
}

impl LanguageCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LocalInstrument2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalLocalInstrument1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl LocalInstrument2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// MandateClassification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateClassification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MandateClassification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl MandateClassification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// MandateClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MandateClassification1Code {
	#[default]
	#[serde(rename = "FIXE")]
	CodeFIXE,
	#[serde(rename = "USGB")]
	CodeUSGB,
	#[serde(rename = "VARI")]
	CodeVARI,
}

impl MandateClassification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MandateRelatedData2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateRelatedData2Choice {
	#[serde(rename = "DrctDbtMndt", skip_serializing_if = "Option::is_none")]
	pub drct_dbt_mndt: Option<MandateRelatedInformation15>,
	#[serde(rename = "CdtTrfMndt", skip_serializing_if = "Option::is_none")]
	pub cdt_trf_mndt: Option<CreditTransferMandateData1>,
}

impl MandateRelatedData2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref drct_dbt_mndt_value) = self.drct_dbt_mndt { if let Err(e) = drct_dbt_mndt_value.validate() { return Err(e); } }
		if let Some(ref cdt_trf_mndt_value) = self.cdt_trf_mndt { if let Err(e) = cdt_trf_mndt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// MandateRelatedInformation15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateRelatedInformation15 {
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<Max35Text>,
	#[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
	pub amdmnt_ind: Option<bool>,
	#[serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none")]
	pub amdmnt_inf_dtls: Option<AmendmentInformationDetails14>,
	#[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_sgntr: Option<Max1025Text>,
	#[serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none")]
	pub frst_colltn_dt: Option<String>,
	#[serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none")]
	pub fnl_colltn_dt: Option<String>,
	#[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
	pub frqcy: Option<Frequency36Choice>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none")]
	pub trckg_days: Option<Exact2NumericText>,
}

impl MandateRelatedInformation15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mndt_id_value) = self.mndt_id { if let Err(e) = mndt_id_value.validate() { return Err(e); } }
		if let Some(ref amdmnt_inf_dtls_value) = self.amdmnt_inf_dtls { if let Err(e) = amdmnt_inf_dtls_value.validate() { return Err(e); } }
		if let Some(ref elctrnc_sgntr_value) = self.elctrnc_sgntr { if let Err(e) = elctrnc_sgntr_value.validate() { return Err(e); } }
		if let Some(ref frqcy_value) = self.frqcy { if let Err(e) = frqcy_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		if let Some(ref trckg_days_value) = self.trckg_days { if let Err(e) = trckg_days_value.validate() { return Err(e); } }
		Ok(())
	}
}


// MandateSetupReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateSetupReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalMandateSetupReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max70Text>,
}

impl MandateSetupReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// MandateTypeInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateTypeInformation2 {
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<ServiceLevel8Choice>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
	#[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
	pub clssfctn: Option<MandateClassification1Choice>,
}

impl MandateTypeInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref svc_lvl_value) = self.svc_lvl { if let Err(e) = svc_lvl_value.validate() { return Err(e); } }
		if let Some(ref lcl_instrm_value) = self.lcl_instrm { if let Err(e) = lcl_instrm_value.validate() { return Err(e); } }
		if let Some(ref ctgy_purp_value) = self.ctgy_purp { if let Err(e) = ctgy_purp_value.validate() { return Err(e); } }
		if let Some(ref clssfctn_value) = self.clssfctn { if let Err(e) = clssfctn_value.validate() { return Err(e); } }
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


// Max10MbBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max10MbBinary {
	#[serde(rename = "$value")]
	pub max10_mb_binary: String,
}

impl Max10MbBinary {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max10_mb_binary.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max10_mb_binary is shorter than the minimum length of 1".to_string()));
		}
		if self.max10_mb_binary.chars().count() > 10485760 {
			return Err(ValidationError::new(1002, "max10_mb_binary exceeds the maximum length of 10485760".to_string()));
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}

impl Max3Number {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}

impl Max500Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max500_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max500_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max500_text.chars().count() > 500 {
			return Err(ValidationError::new(1002, "max500_text exceeds the maximum length of 500".to_string()));
		}
		Ok(())
	}
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}

impl Max52Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max52_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max52_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max52_text.chars().count() > 52 {
			return Err(ValidationError::new(1002, "max52_text exceeds the maximum length of 52".to_string()));
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


// NameAndAddress16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress16 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress24,
}

impl NameAndAddress16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Err(e) = self.adr.validate() { return Err(e); }
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


// OrganisationIdentification29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification29 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl OrganisationIdentification29 {
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


// OriginalGroupInformation29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalGroupInformation29 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: Max35Text,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: Max35Text,
	#[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
	pub orgnl_cre_dt_tm: Option<String>,
}

impl OriginalGroupInformation29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.orgnl_msg_id.validate() { return Err(e); }
		if let Err(e) = self.orgnl_msg_nm_id.validate() { return Err(e); }
		Ok(())
	}
}


// OriginalTransactionReference35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalTransactionReference35 {
	#[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountType4Choice>,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_colltn_dt: Option<String>,
	#[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
	pub cdtr_schme_id: Option<PartyIdentification135>,
	#[serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none")]
	pub sttlm_inf: Option<SettlementInstruction11>,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
	pub pmt_mtd: Option<PaymentMethod4Code>,
	#[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
	pub mndt_rltd_inf: Option<MandateRelatedData2Choice>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation21>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<Party40Choice>,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<Party40Choice>,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Purpose2Choice>,
}

impl OriginalTransactionReference35 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref intr_bk_sttlm_amt_value) = self.intr_bk_sttlm_amt { if let Err(e) = intr_bk_sttlm_amt_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref reqd_exctn_dt_value) = self.reqd_exctn_dt { if let Err(e) = reqd_exctn_dt_value.validate() { return Err(e); } }
		if let Some(ref cdtr_schme_id_value) = self.cdtr_schme_id { if let Err(e) = cdtr_schme_id_value.validate() { return Err(e); } }
		if let Some(ref sttlm_inf_value) = self.sttlm_inf { if let Err(e) = sttlm_inf_value.validate() { return Err(e); } }
		if let Some(ref pmt_tp_inf_value) = self.pmt_tp_inf { if let Err(e) = pmt_tp_inf_value.validate() { return Err(e); } }
		if let Some(ref pmt_mtd_value) = self.pmt_mtd { if let Err(e) = pmt_mtd_value.validate() { return Err(e); } }
		if let Some(ref mndt_rltd_inf_value) = self.mndt_rltd_inf { if let Err(e) = mndt_rltd_inf_value.validate() { return Err(e); } }
		if let Some(ref rmt_inf_value) = self.rmt_inf { if let Err(e) = rmt_inf_value.validate() { return Err(e); } }
		if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
		if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
		if let Some(ref dbtr_acct_value) = self.dbtr_acct { if let Err(e) = dbtr_acct_value.validate() { return Err(e); } }
		if let Some(ref dbtr_agt_value) = self.dbtr_agt { if let Err(e) = dbtr_agt_value.validate() { return Err(e); } }
		if let Some(ref dbtr_agt_acct_value) = self.dbtr_agt_acct { if let Err(e) = dbtr_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref cdtr_agt_value) = self.cdtr_agt { if let Err(e) = cdtr_agt_value.validate() { return Err(e); } }
		if let Some(ref cdtr_agt_acct_value) = self.cdtr_agt_acct { if let Err(e) = cdtr_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
		if let Some(ref cdtr_acct_value) = self.cdtr_acct { if let Err(e) = cdtr_acct_value.validate() { return Err(e); } }
		if let Some(ref ultmt_cdtr_value) = self.ultmt_cdtr { if let Err(e) = ultmt_cdtr_value.validate() { return Err(e); } }
		if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
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


// Party38Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party38Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification29>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification13>,
}

impl Party38Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref org_id_value) = self.org_id { if let Err(e) = org_id_value.validate() { return Err(e); } }
		if let Some(ref prvt_id_value) = self.prvt_id { if let Err(e) = prvt_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Party40Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party40Choice {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification135>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl Party40Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pty_value) = self.pty { if let Err(e) = pty_value.validate() { return Err(e); } }
		if let Some(ref agt_value) = self.agt { if let Err(e) = agt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyAndSignature3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndSignature3 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification135,
	#[serde(rename = "Sgntr")]
	pub sgntr: SkipPayload,
}

impl PartyAndSignature3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pty.validate() { return Err(e); }
		if let Err(e) = self.sgntr.validate() { return Err(e); }
		Ok(())
	}
}


// PartyIdentification135 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification135 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party38Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact4>,
}

impl PartyIdentification135 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if let Err(e) = ctry_of_res_value.validate() { return Err(e); } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if let Err(e) = ctct_dtls_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentMethod4Code {
	#[default]
	#[serde(rename = "CHK")]
	CodeCHK,
	#[serde(rename = "TRF")]
	CodeTRF,
	#[serde(rename = "DD")]
	CodeDD,
	#[serde(rename = "TRA")]
	CodeTRA,
}

impl PaymentMethod4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentTransactionStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionStatus1 {
	#[serde(rename = "Sts")]
	pub sts: TransactionStatus1Choice,
	#[serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none")]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation12>>,
}

impl PaymentTransactionStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref sts_rsn_inf_vec) = self.sts_rsn_inf { for item in sts_rsn_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PaymentTypeInformation27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTypeInformation27 {
	#[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
	pub instr_prty: Option<Priority2Code>,
	#[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
	pub seq_tp: Option<SequenceType3Code>,
	#[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref instr_prty_value) = self.instr_prty { if let Err(e) = instr_prty_value.validate() { return Err(e); } }
		if let Some(ref clr_chanl_value) = self.clr_chanl { if let Err(e) = clr_chanl_value.validate() { return Err(e); } }
		if let Some(ref svc_lvl_vec) = self.svc_lvl { for item in svc_lvl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref lcl_instrm_value) = self.lcl_instrm { if let Err(e) = lcl_instrm_value.validate() { return Err(e); } }
		if let Some(ref seq_tp_value) = self.seq_tp { if let Err(e) = seq_tp_value.validate() { return Err(e); } }
		if let Some(ref ctgy_purp_value) = self.ctgy_purp { if let Err(e) = ctgy_purp_value.validate() { return Err(e); } }
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


// PersonIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification13 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl PersonIdentification13 {
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


// PostalAddress24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress24 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<Max70Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max35Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max16Text>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<Max70Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<Max35Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
}

impl PostalAddress24 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
		if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
		if let Some(ref sub_dept_value) = self.sub_dept { if let Err(e) = sub_dept_value.validate() { return Err(e); } }
		if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
		if let Some(ref bldg_nm_value) = self.bldg_nm { if let Err(e) = bldg_nm_value.validate() { return Err(e); } }
		if let Some(ref flr_value) = self.flr { if let Err(e) = flr_value.validate() { return Err(e); } }
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


// PreferredContactMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferredContactMethod1Code {
	#[default]
	#[serde(rename = "LETT")]
	CodeLETT,
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "PHON")]
	CodePHON,
	#[serde(rename = "FAXX")]
	CodeFAXX,
	#[serde(rename = "CELL")]
	CodeCELL,
}

impl PreferredContactMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Priority2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Priority2Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "NORM")]
	CodeNORM,
}

impl Priority2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProprietaryReference1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReference1 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
}

impl ProprietaryReference1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.ref_attr.validate() { return Err(e); }
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


// Purpose2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl Purpose2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReferredDocumentInformation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation7 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ReferredDocumentType4>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<String>,
	#[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
	pub line_dtls: Option<Vec<DocumentLineInformation1>>,
}

impl ReferredDocumentInformation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
		if let Some(ref line_dtls_vec) = self.line_dtls { for item in line_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ReferredDocumentType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DocumentType6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ReferredDocumentType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReferredDocumentType4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType4 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: ReferredDocumentType3Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl ReferredDocumentType4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RelatedInvestigationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RelatedInvestigationData1 {
	#[serde(rename = "InvstgtnId", skip_serializing_if = "Option::is_none")]
	pub invstgtn_id: Option<Max35Text>,
	#[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
	pub lctn: Option<Vec<InvestigationLocationData1>>,
}

impl RelatedInvestigationData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref invstgtn_id_value) = self.invstgtn_id { if let Err(e) = invstgtn_id_value.validate() { return Err(e); } }
		if let Some(ref lctn_vec) = self.lctn { for item in lctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Remittance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Remittance1 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<Max140Text>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation16>>,
	#[serde(rename = "Rltd", skip_serializing_if = "Option::is_none")]
	pub rltd: Option<Vec<RemittanceLocation7>>,
}

impl Remittance1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ustrd_vec) = self.ustrd { for item in ustrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref strd_vec) = self.strd { for item in strd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rltd_vec) = self.rltd { for item in rltd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// RemittanceAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount2 {
	#[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref due_pybl_amt_value) = self.due_pybl_amt { if let Err(e) = due_pybl_amt_value.validate() { return Err(e); } }
		if let Some(ref dscnt_apld_amt_vec) = self.dscnt_apld_amt { for item in dscnt_apld_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref cdt_note_amt_value) = self.cdt_note_amt { if let Err(e) = cdt_note_amt_value.validate() { return Err(e); } }
		if let Some(ref tax_amt_vec) = self.tax_amt { for item in tax_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref adjstmnt_amt_and_rsn_vec) = self.adjstmnt_amt_and_rsn { for item in adjstmnt_amt_and_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rmtd_amt_value) = self.rmtd_amt { if let Err(e) = rmtd_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RemittanceAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount3 {
	#[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref due_pybl_amt_value) = self.due_pybl_amt { if let Err(e) = due_pybl_amt_value.validate() { return Err(e); } }
		if let Some(ref dscnt_apld_amt_vec) = self.dscnt_apld_amt { for item in dscnt_apld_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref cdt_note_amt_value) = self.cdt_note_amt { if let Err(e) = cdt_note_amt_value.validate() { return Err(e); } }
		if let Some(ref tax_amt_vec) = self.tax_amt { for item in tax_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref adjstmnt_amt_and_rsn_vec) = self.adjstmnt_amt_and_rsn { for item in adjstmnt_amt_and_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rmtd_amt_value) = self.rmtd_amt { if let Err(e) = rmtd_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RemittanceInformation21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation21 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<Max140Text>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation17>>,
}

impl RemittanceInformation21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ustrd_vec) = self.ustrd { for item in ustrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref strd_vec) = self.strd { for item in strd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// RemittanceLocation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocation7 {
	#[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
	pub rmt_id: Option<Max35Text>,
	#[serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none")]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData1>>,
}

impl RemittanceLocation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rmt_id_value) = self.rmt_id { if let Err(e) = rmt_id_value.validate() { return Err(e); } }
		if let Some(ref rmt_lctn_dtls_vec) = self.rmt_lctn_dtls { for item in rmt_lctn_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// RemittanceLocationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocationData1 {
	#[serde(rename = "Mtd")]
	pub mtd: RemittanceLocationMethod2Code,
	#[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_adr: Option<Max2048Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<NameAndAddress16>,
}

impl RemittanceLocationData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.mtd.validate() { return Err(e); }
		if let Some(ref elctrnc_adr_value) = self.elctrnc_adr { if let Err(e) = elctrnc_adr_value.validate() { return Err(e); } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RemittanceLocationMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RemittanceLocationMethod2Code {
	#[default]
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "EDIC")]
	CodeEDIC,
	#[serde(rename = "URID")]
	CodeURID,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "SMSM")]
	CodeSMSM,
}

impl RemittanceLocationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SequenceType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SequenceType3Code {
	#[default]
	#[serde(rename = "FRST")]
	CodeFRST,
	#[serde(rename = "RCUR")]
	CodeRCUR,
	#[serde(rename = "FNAL")]
	CodeFNAL,
	#[serde(rename = "OOFF")]
	CodeOOFF,
	#[serde(rename = "RPRE")]
	CodeRPRE,
}

impl SequenceType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ServiceLevel8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceLevel8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalServiceLevel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ServiceLevel8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementInstruction11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstruction11 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: SettlementMethod1Code,
	#[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
	pub sttlm_acct: Option<CashAccount40>,
	#[serde(rename = "ClrSys", skip_serializing_if = "Option::is_none")]
	pub clr_sys: Option<ClearingSystemIdentification3Choice>,
	#[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub thrd_rmbrsmnt_agt_acct: Option<CashAccount40>,
}

impl SettlementInstruction11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.sttlm_mtd.validate() { return Err(e); }
		if let Some(ref sttlm_acct_value) = self.sttlm_acct { if let Err(e) = sttlm_acct_value.validate() { return Err(e); } }
		if let Some(ref clr_sys_value) = self.clr_sys { if let Err(e) = clr_sys_value.validate() { return Err(e); } }
		if let Some(ref instg_rmbrsmnt_agt_value) = self.instg_rmbrsmnt_agt { if let Err(e) = instg_rmbrsmnt_agt_value.validate() { return Err(e); } }
		if let Some(ref instg_rmbrsmnt_agt_acct_value) = self.instg_rmbrsmnt_agt_acct { if let Err(e) = instg_rmbrsmnt_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref instd_rmbrsmnt_agt_value) = self.instd_rmbrsmnt_agt { if let Err(e) = instd_rmbrsmnt_agt_value.validate() { return Err(e); } }
		if let Some(ref instd_rmbrsmnt_agt_acct_value) = self.instd_rmbrsmnt_agt_acct { if let Err(e) = instd_rmbrsmnt_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref thrd_rmbrsmnt_agt_value) = self.thrd_rmbrsmnt_agt { if let Err(e) = thrd_rmbrsmnt_agt_value.validate() { return Err(e); } }
		if let Some(ref thrd_rmbrsmnt_agt_acct_value) = self.thrd_rmbrsmnt_agt_acct { if let Err(e) = thrd_rmbrsmnt_agt_acct_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementMethod1Code {
	#[default]
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "INGA")]
	CodeINGA,
	#[serde(rename = "COVE")]
	CodeCOVE,
	#[serde(rename = "CLRG")]
	CodeCLRG,
}

impl SettlementMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// StatusReason6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusReason6Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl StatusReason6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// StatusReasonInformation12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusReasonInformation12 {
	#[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
	pub orgtr: Option<PartyIdentification135>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<StatusReason6Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<Max105Text>>,
}

impl StatusReasonInformation12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgtr_value) = self.orgtr { if let Err(e) = orgtr_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// StructuredRemittanceInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation16 {
	#[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
	pub invcr: Option<PartyIdentification135>,
	#[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
	pub invcee: Option<PartyIdentification135>,
	#[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
	pub tax_rmt: Option<TaxInformation7>,
	#[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
	pub grnshmt_rmt: Option<Garnishment3>,
	#[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rmt_inf: Option<Vec<Max140Text>>,
}

impl StructuredRemittanceInformation16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rfrd_doc_inf_vec) = self.rfrd_doc_inf { for item in rfrd_doc_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rfrd_doc_amt_value) = self.rfrd_doc_amt { if let Err(e) = rfrd_doc_amt_value.validate() { return Err(e); } }
		if let Some(ref cdtr_ref_inf_value) = self.cdtr_ref_inf { if let Err(e) = cdtr_ref_inf_value.validate() { return Err(e); } }
		if let Some(ref invcr_value) = self.invcr { if let Err(e) = invcr_value.validate() { return Err(e); } }
		if let Some(ref invcee_value) = self.invcee { if let Err(e) = invcee_value.validate() { return Err(e); } }
		if let Some(ref tax_rmt_value) = self.tax_rmt { if let Err(e) = tax_rmt_value.validate() { return Err(e); } }
		if let Some(ref grnshmt_rmt_value) = self.grnshmt_rmt { if let Err(e) = grnshmt_rmt_value.validate() { return Err(e); } }
		if let Some(ref addtl_rmt_inf_vec) = self.addtl_rmt_inf { for item in addtl_rmt_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// StructuredRemittanceInformation17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation17 {
	#[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
	pub invcr: Option<PartyIdentification135>,
	#[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
	pub invcee: Option<PartyIdentification135>,
	#[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
	pub tax_rmt: Option<TaxData1>,
	#[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
	pub grnshmt_rmt: Option<Garnishment3>,
	#[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rmt_inf: Option<Vec<Max140Text>>,
}

impl StructuredRemittanceInformation17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rfrd_doc_inf_vec) = self.rfrd_doc_inf { for item in rfrd_doc_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rfrd_doc_amt_value) = self.rfrd_doc_amt { if let Err(e) = rfrd_doc_amt_value.validate() { return Err(e); } }
		if let Some(ref cdtr_ref_inf_value) = self.cdtr_ref_inf { if let Err(e) = cdtr_ref_inf_value.validate() { return Err(e); } }
		if let Some(ref invcr_value) = self.invcr { if let Err(e) = invcr_value.validate() { return Err(e); } }
		if let Some(ref invcee_value) = self.invcee { if let Err(e) = invcee_value.validate() { return Err(e); } }
		if let Some(ref tax_rmt_value) = self.tax_rmt { if let Err(e) = tax_rmt_value.validate() { return Err(e); } }
		if let Some(ref grnshmt_rmt_value) = self.grnshmt_rmt { if let Err(e) = grnshmt_rmt_value.validate() { return Err(e); } }
		if let Some(ref addtl_rmt_inf_vec) = self.addtl_rmt_inf { for item in addtl_rmt_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// TaxAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmount2 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<Vec<TaxRecordDetails2>>,
}

impl TaxAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref taxbl_base_amt_value) = self.taxbl_base_amt { if let Err(e) = taxbl_base_amt_value.validate() { return Err(e); } }
		if let Some(ref ttl_amt_value) = self.ttl_amt { if let Err(e) = ttl_amt_value.validate() { return Err(e); } }
		if let Some(ref dtls_vec) = self.dtls { for item in dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TaxAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmount3 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<Vec<TaxRecordDetails3>>,
}

impl TaxAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref taxbl_base_amt_value) = self.taxbl_base_amt { if let Err(e) = taxbl_base_amt_value.validate() { return Err(e); } }
		if let Some(ref ttl_amt_value) = self.ttl_amt { if let Err(e) = ttl_amt_value.validate() { return Err(e); } }
		if let Some(ref dtls_vec) = self.dtls { for item in dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TaxAmountAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountAndType1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxAmountAndType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// TaxAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalTaxAmountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl TaxAmountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxAuthorisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
	pub titl: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
}

impl TaxAuthorisation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref titl_value) = self.titl { if let Err(e) = titl_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxCharges2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCharges2 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl TaxCharges2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxData1 {
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
	pub admstn_zone: Option<Max35Text>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<Max140Text>,
	#[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
	pub mtd: Option<Max35Text>,
	#[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<TaxRecord3>>,
}

impl TaxData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
		if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
		if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
		if let Some(ref admstn_zone_value) = self.admstn_zone { if let Err(e) = admstn_zone_value.validate() { return Err(e); } }
		if let Some(ref ref_nb_value) = self.ref_nb { if let Err(e) = ref_nb_value.validate() { return Err(e); } }
		if let Some(ref mtd_value) = self.mtd { if let Err(e) = mtd_value.validate() { return Err(e); } }
		if let Some(ref ttl_taxbl_base_amt_value) = self.ttl_taxbl_base_amt { if let Err(e) = ttl_taxbl_base_amt_value.validate() { return Err(e); } }
		if let Some(ref ttl_tax_amt_value) = self.ttl_tax_amt { if let Err(e) = ttl_tax_amt_value.validate() { return Err(e); } }
		if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TaxInformation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxInformation7 {
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
	pub admstn_zone: Option<Max35Text>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<Max140Text>,
	#[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
	pub mtd: Option<Max35Text>,
	#[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<TaxRecord2>>,
}

impl TaxInformation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
		if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
		if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
		if let Some(ref admstn_zone_value) = self.admstn_zone { if let Err(e) = admstn_zone_value.validate() { return Err(e); } }
		if let Some(ref ref_nb_value) = self.ref_nb { if let Err(e) = ref_nb_value.validate() { return Err(e); } }
		if let Some(ref mtd_value) = self.mtd { if let Err(e) = mtd_value.validate() { return Err(e); } }
		if let Some(ref ttl_taxbl_base_amt_value) = self.ttl_taxbl_base_amt { if let Err(e) = ttl_taxbl_base_amt_value.validate() { return Err(e); } }
		if let Some(ref ttl_tax_amt_value) = self.ttl_tax_amt { if let Err(e) = ttl_tax_amt_value.validate() { return Err(e); } }
		if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TaxParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
}

impl TaxParty1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tax_id_value) = self.tax_id { if let Err(e) = tax_id_value.validate() { return Err(e); } }
		if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
		if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxParty2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
	#[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
	pub authstn: Option<TaxAuthorisation1>,
}

impl TaxParty2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tax_id_value) = self.tax_id { if let Err(e) = tax_id_value.validate() { return Err(e); } }
		if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
		if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
		if let Some(ref authstn_value) = self.authstn { if let Err(e) = authstn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxPeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod2 {
	#[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
	pub yr: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxPeriod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod3 {
	#[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
	pub yr: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxRecord2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord2 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<Max35Text>,
	#[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
	pub ctgy_dtls: Option<Max35Text>,
	#[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
	pub dbtr_sts: Option<Max35Text>,
	#[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
	pub cert_id: Option<Max35Text>,
	#[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
	pub frms_cd: Option<Max35Text>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<TaxAmount2>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max140Text>,
}

impl TaxRecord2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref ctgy_value) = self.ctgy { if let Err(e) = ctgy_value.validate() { return Err(e); } }
		if let Some(ref ctgy_dtls_value) = self.ctgy_dtls { if let Err(e) = ctgy_dtls_value.validate() { return Err(e); } }
		if let Some(ref dbtr_sts_value) = self.dbtr_sts { if let Err(e) = dbtr_sts_value.validate() { return Err(e); } }
		if let Some(ref cert_id_value) = self.cert_id { if let Err(e) = cert_id_value.validate() { return Err(e); } }
		if let Some(ref frms_cd_value) = self.frms_cd { if let Err(e) = frms_cd_value.validate() { return Err(e); } }
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Some(ref tax_amt_value) = self.tax_amt { if let Err(e) = tax_amt_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<Max35Text>,
	#[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
	pub ctgy_dtls: Option<Max35Text>,
	#[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
	pub dbtr_sts: Option<Max35Text>,
	#[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
	pub cert_id: Option<Max35Text>,
	#[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
	pub frms_cd: Option<Max35Text>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<TaxAmount3>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max140Text>,
}

impl TaxRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref ctgy_value) = self.ctgy { if let Err(e) = ctgy_value.validate() { return Err(e); } }
		if let Some(ref ctgy_dtls_value) = self.ctgy_dtls { if let Err(e) = ctgy_dtls_value.validate() { return Err(e); } }
		if let Some(ref dbtr_sts_value) = self.dbtr_sts { if let Err(e) = dbtr_sts_value.validate() { return Err(e); } }
		if let Some(ref cert_id_value) = self.cert_id { if let Err(e) = cert_id_value.validate() { return Err(e); } }
		if let Some(ref frms_cd_value) = self.frms_cd { if let Err(e) = frms_cd_value.validate() { return Err(e); } }
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Some(ref tax_amt_value) = self.tax_amt { if let Err(e) = tax_amt_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxRecordDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails2 {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxRecordDetails2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// TaxRecordDetails3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails3 {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxRecordDetails3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// TaxRecordPeriod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxRecordPeriod1Code {
	#[default]
	#[serde(rename = "MM01")]
	CodeMM01,
	#[serde(rename = "MM02")]
	CodeMM02,
	#[serde(rename = "MM03")]
	CodeMM03,
	#[serde(rename = "MM04")]
	CodeMM04,
	#[serde(rename = "MM05")]
	CodeMM05,
	#[serde(rename = "MM06")]
	CodeMM06,
	#[serde(rename = "MM07")]
	CodeMM07,
	#[serde(rename = "MM08")]
	CodeMM08,
	#[serde(rename = "MM09")]
	CodeMM09,
	#[serde(rename = "MM10")]
	CodeMM10,
	#[serde(rename = "MM11")]
	CodeMM11,
	#[serde(rename = "MM12")]
	CodeMM12,
	#[serde(rename = "QTR1")]
	CodeQTR1,
	#[serde(rename = "QTR2")]
	CodeQTR2,
	#[serde(rename = "QTR3")]
	CodeQTR3,
	#[serde(rename = "QTR4")]
	CodeQTR4,
	#[serde(rename = "HLF1")]
	CodeHLF1,
	#[serde(rename = "HLF2")]
	CodeHLF2,
}

impl TaxRecordPeriod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionAmendment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAmendment1 {
	#[serde(rename = "Pth", skip_serializing_if = "Option::is_none")]
	pub pth: Option<Max2048Text>,
	#[serde(rename = "Rcrd")]
	pub rcrd: TransactionAmendment1Choice,
}

impl TransactionAmendment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pth_value) = self.pth { if let Err(e) = pth_value.validate() { return Err(e); } }
		if let Err(e) = self.rcrd.validate() { return Err(e); }
		Ok(())
	}
}


// TransactionAmendment1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAmendment1Choice {
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
	pub csh_acct: Option<CashAccount40>,
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max4Text>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification135>,
	#[serde(rename = "Rmt", skip_serializing_if = "Option::is_none")]
	pub rmt: Option<Remittance1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max140Text>,
}

impl TransactionAmendment1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref agt_value) = self.agt { if let Err(e) = agt_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
		if let Some(ref csh_acct_value) = self.csh_acct { if let Err(e) = csh_acct_value.validate() { return Err(e); } }
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref pty_value) = self.pty { if let Err(e) = pty_value.validate() { return Err(e); } }
		if let Some(ref rmt_value) = self.rmt { if let Err(e) = rmt_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TransactionReferences6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReferences6 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_ref: Option<Max35Text>,
	#[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
	pub pmt_inf_id: Option<Max35Text>,
	#[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
	pub instr_id: Option<Max35Text>,
	#[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id: Option<Max35Text>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max35Text>,
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<Max35Text>,
	#[serde(rename = "ChqNb", skip_serializing_if = "Option::is_none")]
	pub chq_nb: Option<Max35Text>,
	#[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
	pub clr_sys_ref: Option<Max35Text>,
	#[serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_tx_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<Max35Text>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<Max35Text>,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryReference1>>,
}

impl TransactionReferences6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref msg_id_value) = self.msg_id { if let Err(e) = msg_id_value.validate() { return Err(e); } }
		if let Some(ref acct_svcr_ref_value) = self.acct_svcr_ref { if let Err(e) = acct_svcr_ref_value.validate() { return Err(e); } }
		if let Some(ref pmt_inf_id_value) = self.pmt_inf_id { if let Err(e) = pmt_inf_id_value.validate() { return Err(e); } }
		if let Some(ref instr_id_value) = self.instr_id { if let Err(e) = instr_id_value.validate() { return Err(e); } }
		if let Some(ref end_to_end_id_value) = self.end_to_end_id { if let Err(e) = end_to_end_id_value.validate() { return Err(e); } }
		if let Some(ref uetr_value) = self.uetr { if let Err(e) = uetr_value.validate() { return Err(e); } }
		if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
		if let Some(ref mndt_id_value) = self.mndt_id { if let Err(e) = mndt_id_value.validate() { return Err(e); } }
		if let Some(ref chq_nb_value) = self.chq_nb { if let Err(e) = chq_nb_value.validate() { return Err(e); } }
		if let Some(ref clr_sys_ref_value) = self.clr_sys_ref { if let Err(e) = clr_sys_ref_value.validate() { return Err(e); } }
		if let Some(ref acct_ownr_tx_id_value) = self.acct_ownr_tx_id { if let Err(e) = acct_ownr_tx_id_value.validate() { return Err(e); } }
		if let Some(ref acct_svcr_tx_id_value) = self.acct_svcr_tx_id { if let Err(e) = acct_svcr_tx_id_value.validate() { return Err(e); } }
		if let Some(ref mkt_infrstrctr_tx_id_value) = self.mkt_infrstrctr_tx_id { if let Err(e) = mkt_infrstrctr_tx_id_value.validate() { return Err(e); } }
		if let Some(ref prcg_id_value) = self.prcg_id { if let Err(e) = prcg_id_value.validate() { return Err(e); } }
		if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPaymentTransactionStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl TransactionStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// UUIDv4Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UUIDv4Identifier {
	#[serde(rename = "$value")]
	pub uui_dv4_identifier: String,
}

impl UUIDv4Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
		if !pattern.is_match(&self.uui_dv4_identifier) {
			return Err(ValidationError::new(1005, "uui_dv4_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// UnderlyingData2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingData2Choice {
	#[serde(rename = "Initn", skip_serializing_if = "Option::is_none")]
	pub initn: Option<UnderlyingPaymentInstruction8>,
	#[serde(rename = "IntrBk", skip_serializing_if = "Option::is_none")]
	pub intr_bk: Option<UnderlyingPaymentTransaction7>,
	#[serde(rename = "StmtNtry", skip_serializing_if = "Option::is_none")]
	pub stmt_ntry: Option<UnderlyingStatementEntry5>,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<CashAccount40>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericIdentification1>,
}

impl UnderlyingData2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref initn_value) = self.initn { if let Err(e) = initn_value.validate() { return Err(e); } }
		if let Some(ref intr_bk_value) = self.intr_bk { if let Err(e) = intr_bk_value.validate() { return Err(e); } }
		if let Some(ref stmt_ntry_value) = self.stmt_ntry { if let Err(e) = stmt_ntry_value.validate() { return Err(e); } }
		if let Some(ref acct_value) = self.acct { if let Err(e) = acct_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// UnderlyingGroupInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingGroupInformation1 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: Max35Text,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: Max35Text,
	#[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
	pub orgnl_cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlMsgDlvryChanl", skip_serializing_if = "Option::is_none")]
	pub orgnl_msg_dlvry_chanl: Option<Max35Text>,
}

impl UnderlyingGroupInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.orgnl_msg_id.validate() { return Err(e); }
		if let Err(e) = self.orgnl_msg_nm_id.validate() { return Err(e); }
		if let Some(ref orgnl_msg_dlvry_chanl_value) = self.orgnl_msg_dlvry_chanl { if let Err(e) = orgnl_msg_dlvry_chanl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// UnderlyingInvestigationInstrument1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingInvestigationInstrument1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalInvestigationInstrument1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl UnderlyingInvestigationInstrument1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// UnderlyingPaymentInstruction8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingPaymentInstruction8 {
	#[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[serde(rename = "OrgnlPmtInfId", skip_serializing_if = "Option::is_none")]
	pub orgnl_pmt_inf_id: Option<Max35Text>,
	#[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
	pub orgnl_instr_id: Option<Max35Text>,
	#[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
	pub orgnl_end_to_end_id: Option<Max35Text>,
	#[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
	pub orgnl_uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "OrgnlInstdAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_colltn_dt: Option<String>,
	#[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference35>,
	#[serde(rename = "OrgnlSvcLvl", skip_serializing_if = "Option::is_none")]
	pub orgnl_svc_lvl: Option<ServiceLevel8Choice>,
}

impl UnderlyingPaymentInstruction8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgnl_grp_inf_value) = self.orgnl_grp_inf { if let Err(e) = orgnl_grp_inf_value.validate() { return Err(e); } }
		if let Some(ref orgnl_pmt_inf_id_value) = self.orgnl_pmt_inf_id { if let Err(e) = orgnl_pmt_inf_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_instr_id_value) = self.orgnl_instr_id { if let Err(e) = orgnl_instr_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_end_to_end_id_value) = self.orgnl_end_to_end_id { if let Err(e) = orgnl_end_to_end_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_uetr_value) = self.orgnl_uetr { if let Err(e) = orgnl_uetr_value.validate() { return Err(e); } }
		if let Some(ref orgnl_instd_amt_value) = self.orgnl_instd_amt { if let Err(e) = orgnl_instd_amt_value.validate() { return Err(e); } }
		if let Some(ref reqd_exctn_dt_value) = self.reqd_exctn_dt { if let Err(e) = reqd_exctn_dt_value.validate() { return Err(e); } }
		if let Some(ref orgnl_tx_ref_value) = self.orgnl_tx_ref { if let Err(e) = orgnl_tx_ref_value.validate() { return Err(e); } }
		if let Some(ref orgnl_svc_lvl_value) = self.orgnl_svc_lvl { if let Err(e) = orgnl_svc_lvl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// UnderlyingPaymentTransaction7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingPaymentTransaction7 {
	#[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
	pub orgnl_instr_id: Option<Max35Text>,
	#[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
	pub orgnl_end_to_end_id: Option<Max35Text>,
	#[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
	pub orgnl_tx_id: Option<Max35Text>,
	#[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
	pub orgnl_uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference35>,
	#[serde(rename = "OrgnlSvcLvl", skip_serializing_if = "Option::is_none")]
	pub orgnl_svc_lvl: Option<ServiceLevel8Choice>,
}

impl UnderlyingPaymentTransaction7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgnl_grp_inf_value) = self.orgnl_grp_inf { if let Err(e) = orgnl_grp_inf_value.validate() { return Err(e); } }
		if let Some(ref orgnl_instr_id_value) = self.orgnl_instr_id { if let Err(e) = orgnl_instr_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_end_to_end_id_value) = self.orgnl_end_to_end_id { if let Err(e) = orgnl_end_to_end_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_tx_id_value) = self.orgnl_tx_id { if let Err(e) = orgnl_tx_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_uetr_value) = self.orgnl_uetr { if let Err(e) = orgnl_uetr_value.validate() { return Err(e); } }
		if let Some(ref orgnl_intr_bk_sttlm_amt_value) = self.orgnl_intr_bk_sttlm_amt { if let Err(e) = orgnl_intr_bk_sttlm_amt_value.validate() { return Err(e); } }
		if let Some(ref orgnl_tx_ref_value) = self.orgnl_tx_ref { if let Err(e) = orgnl_tx_ref_value.validate() { return Err(e); } }
		if let Some(ref orgnl_svc_lvl_value) = self.orgnl_svc_lvl { if let Err(e) = orgnl_svc_lvl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// UnderlyingStatementEntry5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingStatementEntry5 {
	#[serde(rename = "OrgnlAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[serde(rename = "OrgnlStmtId", skip_serializing_if = "Option::is_none")]
	pub orgnl_stmt_id: Option<Max35Text>,
	#[serde(rename = "OrgnlNtryRef", skip_serializing_if = "Option::is_none")]
	pub orgnl_ntry_ref: Option<Max35Text>,
	#[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
	pub orgnl_uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "OrgnlNtryAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_ntry_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "OrgnlNtryValDt", skip_serializing_if = "Option::is_none")]
	pub orgnl_ntry_val_dt: Option<DateAndDateTime2Choice>,
}

impl UnderlyingStatementEntry5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgnl_acct_value) = self.orgnl_acct { if let Err(e) = orgnl_acct_value.validate() { return Err(e); } }
		if let Some(ref orgnl_grp_inf_value) = self.orgnl_grp_inf { if let Err(e) = orgnl_grp_inf_value.validate() { return Err(e); } }
		if let Some(ref orgnl_stmt_id_value) = self.orgnl_stmt_id { if let Err(e) = orgnl_stmt_id_value.validate() { return Err(e); } }
		if let Some(ref orgnl_ntry_ref_value) = self.orgnl_ntry_ref { if let Err(e) = orgnl_ntry_ref_value.validate() { return Err(e); } }
		if let Some(ref orgnl_uetr_value) = self.orgnl_uetr { if let Err(e) = orgnl_uetr_value.validate() { return Err(e); } }
		if let Some(ref orgnl_ntry_amt_value) = self.orgnl_ntry_amt { if let Err(e) = orgnl_ntry_amt_value.validate() { return Err(e); } }
		if let Some(ref orgnl_ntry_val_dt_value) = self.orgnl_ntry_val_dt { if let Err(e) = orgnl_ntry_val_dt_value.validate() { return Err(e); } }
		Ok(())
	}
}

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


// AccountInterest4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountInterest4 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<InterestType1Choice>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<Vec<Rate4>>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max35Text>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxCharges2>,
}

impl AccountInterest4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref rate_vec) = self.rate { for item in rate_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AccountNotification17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountNotification17 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "NtfctnPgntn", skip_serializing_if = "Option::is_none")]
	pub ntfctn_pgntn: Option<Pagination1>,
	#[serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none")]
	pub elctrnc_seq_nb: Option<f64>,
	#[serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none")]
	pub rptg_seq: Option<SequenceRange1Choice>,
	#[serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none")]
	pub lgl_seq_nb: Option<f64>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "CpyDplctInd", skip_serializing_if = "Option::is_none")]
	pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
	#[serde(rename = "RptgSrc", skip_serializing_if = "Option::is_none")]
	pub rptg_src: Option<ReportingSource1Choice>,
	#[serde(rename = "Acct")]
	pub acct: CashAccount39,
	#[serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none")]
	pub rltd_acct: Option<CashAccount38>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<Vec<AccountInterest4>>,
	#[serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none")]
	pub txs_summry: Option<TotalTransactions6>,
	#[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
	pub ntry: Option<Vec<ReportEntry10>>,
	#[serde(rename = "AddtlNtfctnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_ntfctn_inf: Option<Max500Text>,
}

impl AccountNotification17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref ntfctn_pgntn_value) = self.ntfctn_pgntn { if let Err(e) = ntfctn_pgntn_value.validate() { return Err(e); } }
		if let Some(ref rptg_seq_value) = self.rptg_seq { if let Err(e) = rptg_seq_value.validate() { return Err(e); } }
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		if let Some(ref cpy_dplct_ind_value) = self.cpy_dplct_ind { if let Err(e) = cpy_dplct_ind_value.validate() { return Err(e); } }
		if let Some(ref rptg_src_value) = self.rptg_src { if let Err(e) = rptg_src_value.validate() { return Err(e); } }
		if let Err(e) = self.acct.validate() { return Err(e); }
		if let Some(ref rltd_acct_value) = self.rltd_acct { if let Err(e) = rltd_acct_value.validate() { return Err(e); } }
		if let Some(ref intrst_vec) = self.intrst { for item in intrst_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref txs_summry_value) = self.txs_summry { if let Err(e) = txs_summry_value.validate() { return Err(e); } }
		if let Some(ref ntry_vec) = self.ntry { for item in ntry_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref addtl_ntfctn_inf_value) = self.addtl_ntfctn_inf { if let Err(e) = addtl_ntfctn_inf_value.validate() { return Err(e); } }
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


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_or_historic_currency_and13_decimal_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_or_historic_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// ActiveOrHistoricCurrencyAndAmountRange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
	#[serde(rename = "Amt")]
	pub amt: ImpliedCurrencyAmountRange1Choice,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
}

impl ActiveOrHistoricCurrencyAndAmountRange2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
		if let Err(e) = self.ccy.validate() { return Err(e); }
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


// AmountAndCurrencyExchange3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchange3 {
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
	pub tx_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none")]
	pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none")]
	pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none")]
	pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails4>>,
}

impl AmountAndCurrencyExchange3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref instd_amt_value) = self.instd_amt { if let Err(e) = instd_amt_value.validate() { return Err(e); } }
		if let Some(ref tx_amt_value) = self.tx_amt { if let Err(e) = tx_amt_value.validate() { return Err(e); } }
		if let Some(ref cntr_val_amt_value) = self.cntr_val_amt { if let Err(e) = cntr_val_amt_value.validate() { return Err(e); } }
		if let Some(ref anncd_pstng_amt_value) = self.anncd_pstng_amt { if let Err(e) = anncd_pstng_amt_value.validate() { return Err(e); } }
		if let Some(ref prtry_amt_vec) = self.prtry_amt { for item in prtry_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// AmountAndCurrencyExchangeDetails3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchangeDetails3 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref ccy_xchg_value) = self.ccy_xchg { if let Err(e) = ccy_xchg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AmountAndCurrencyExchangeDetails4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchangeDetails4 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Some(ref ccy_xchg_value) = self.ccy_xchg { if let Err(e) = ccy_xchg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AmountAndDirection35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection35 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
}

impl AmountAndDirection35 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		Ok(())
	}
}


// AmountRangeBoundary1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountRangeBoundary1 {
	#[serde(rename = "BdryAmt")]
	pub bdry_amt: f64,
	#[serde(rename = "Incl")]
	pub incl: bool,
}

impl AmountRangeBoundary1 {
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


// AttendanceContext1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AttendanceContext1Code {
	#[default]
	#[serde(rename = "ATTD")]
	CodeATTD,
	#[serde(rename = "SATT")]
	CodeSATT,
	#[serde(rename = "UATT")]
	CodeUATT,
}

impl AttendanceContext1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AuthenticationEntity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AuthenticationEntity1Code {
	#[default]
	#[serde(rename = "ICCD")]
	CodeICCD,
	#[serde(rename = "AGNT")]
	CodeAGNT,
	#[serde(rename = "MERC")]
	CodeMERC,
}

impl AuthenticationEntity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AuthenticationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod1Code {
	#[default]
	#[serde(rename = "UKNW")]
	CodeUKNW,
	#[serde(rename = "BYPS")]
	CodeBYPS,
	#[serde(rename = "NPIN")]
	CodeNPIN,
	#[serde(rename = "FPIN")]
	CodeFPIN,
	#[serde(rename = "CPSG")]
	CodeCPSG,
	#[serde(rename = "PPSG")]
	CodePPSG,
	#[serde(rename = "MANU")]
	CodeMANU,
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "SCRT")]
	CodeSCRT,
	#[serde(rename = "SNCT")]
	CodeSNCT,
	#[serde(rename = "SCNL")]
	CodeSCNL,
}

impl AuthenticationMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// BankToCustomerDebitCreditNotificationV08 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankToCustomerDebitCreditNotificationV08 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader81,
	#[serde(rename = "Ntfctn")]
	pub ntfctn: Vec<AccountNotification17>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl BankToCustomerDebitCreditNotificationV08 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.grp_hdr.validate() { return Err(e); }
		for item in &self.ntfctn { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// BatchInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BatchInformation2 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Max35Text>,
	#[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
	pub pmt_inf_id: Option<Max35Text>,
	#[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
	pub nb_of_txs: Option<Max15NumericText>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
}

impl BatchInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref msg_id_value) = self.msg_id { if let Err(e) = msg_id_value.validate() { return Err(e); } }
		if let Some(ref pmt_inf_id_value) = self.pmt_inf_id { if let Err(e) = pmt_inf_id_value.validate() { return Err(e); } }
		if let Some(ref nb_of_txs_value) = self.nb_of_txs { if let Err(e) = nb_of_txs_value.validate() { return Err(e); } }
		if let Some(ref ttl_amt_value) = self.ttl_amt { if let Err(e) = ttl_amt_value.validate() { return Err(e); } }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
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


// CSCManagement1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CSCManagement1Code {
	#[default]
	#[serde(rename = "PRST")]
	CodePRST,
	#[serde(rename = "BYPS")]
	CodeBYPS,
	#[serde(rename = "UNRD")]
	CodeUNRD,
	#[serde(rename = "NCSC")]
	CodeNCSC,
}

impl CSCManagement1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CardAggregated2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardAggregated2 {
	#[serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none")]
	pub addtl_svc: Option<CardPaymentServiceType2Code>,
	#[serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none")]
	pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
	#[serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none")]
	pub sale_rcncltn_id: Option<Max35Text>,
	#[serde(rename = "SeqNbRg", skip_serializing_if = "Option::is_none")]
	pub seq_nb_rg: Option<CardSequenceNumberRange1>,
	#[serde(rename = "TxDtRg", skip_serializing_if = "Option::is_none")]
	pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice>,
}

impl CardAggregated2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref addtl_svc_value) = self.addtl_svc { if let Err(e) = addtl_svc_value.validate() { return Err(e); } }
		if let Some(ref tx_ctgy_value) = self.tx_ctgy { if let Err(e) = tx_ctgy_value.validate() { return Err(e); } }
		if let Some(ref sale_rcncltn_id_value) = self.sale_rcncltn_id { if let Err(e) = sale_rcncltn_id_value.validate() { return Err(e); } }
		if let Some(ref seq_nb_rg_value) = self.seq_nb_rg { if let Err(e) = seq_nb_rg_value.validate() { return Err(e); } }
		if let Some(ref tx_dt_rg_value) = self.tx_dt_rg { if let Err(e) = tx_dt_rg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CardDataReading1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardDataReading1Code {
	#[default]
	#[serde(rename = "TAGC")]
	CodeTAGC,
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "BRCD")]
	CodeBRCD,
	#[serde(rename = "MGST")]
	CodeMGST,
	#[serde(rename = "CICC")]
	CodeCICC,
	#[serde(rename = "DFLE")]
	CodeDFLE,
	#[serde(rename = "CTLS")]
	CodeCTLS,
	#[serde(rename = "ECTL")]
	CodeECTL,
}

impl CardDataReading1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CardEntry4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardEntry4 {
	#[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
	pub card: Option<PaymentCard4>,
	#[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
	pub poi: Option<PointOfInteraction1>,
	#[serde(rename = "AggtdNtry", skip_serializing_if = "Option::is_none")]
	pub aggtd_ntry: Option<CardAggregated2>,
	#[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
	pub pre_pd_acct: Option<CashAccount38>,
}

impl CardEntry4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref card_value) = self.card { if let Err(e) = card_value.validate() { return Err(e); } }
		if let Some(ref poi_value) = self.poi { if let Err(e) = poi_value.validate() { return Err(e); } }
		if let Some(ref aggtd_ntry_value) = self.aggtd_ntry { if let Err(e) = aggtd_ntry_value.validate() { return Err(e); } }
		if let Some(ref pre_pd_acct_value) = self.pre_pd_acct { if let Err(e) = pre_pd_acct_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CardIndividualTransaction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardIndividualTransaction2 {
	#[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
	pub icc_rltd_data: Option<Max1025Text>,
	#[serde(rename = "PmtCntxt", skip_serializing_if = "Option::is_none")]
	pub pmt_cntxt: Option<PaymentContext3>,
	#[serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none")]
	pub addtl_svc: Option<CardPaymentServiceType2Code>,
	#[serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none")]
	pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
	#[serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none")]
	pub sale_rcncltn_id: Option<Max35Text>,
	#[serde(rename = "SaleRefNb", skip_serializing_if = "Option::is_none")]
	pub sale_ref_nb: Option<Max35Text>,
	#[serde(rename = "RePresntmntRsn", skip_serializing_if = "Option::is_none")]
	pub re_presntmnt_rsn: Option<ExternalRePresentmentReason1Code>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<Max35Text>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<TransactionIdentifier1>,
	#[serde(rename = "Pdct", skip_serializing_if = "Option::is_none")]
	pub pdct: Option<Product2>,
	#[serde(rename = "VldtnDt", skip_serializing_if = "Option::is_none")]
	pub vldtn_dt: Option<String>,
	#[serde(rename = "VldtnSeqNb", skip_serializing_if = "Option::is_none")]
	pub vldtn_seq_nb: Option<Max35Text>,
}

impl CardIndividualTransaction2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref icc_rltd_data_value) = self.icc_rltd_data { if let Err(e) = icc_rltd_data_value.validate() { return Err(e); } }
		if let Some(ref pmt_cntxt_value) = self.pmt_cntxt { if let Err(e) = pmt_cntxt_value.validate() { return Err(e); } }
		if let Some(ref addtl_svc_value) = self.addtl_svc { if let Err(e) = addtl_svc_value.validate() { return Err(e); } }
		if let Some(ref tx_ctgy_value) = self.tx_ctgy { if let Err(e) = tx_ctgy_value.validate() { return Err(e); } }
		if let Some(ref sale_rcncltn_id_value) = self.sale_rcncltn_id { if let Err(e) = sale_rcncltn_id_value.validate() { return Err(e); } }
		if let Some(ref sale_ref_nb_value) = self.sale_ref_nb { if let Err(e) = sale_ref_nb_value.validate() { return Err(e); } }
		if let Some(ref re_presntmnt_rsn_value) = self.re_presntmnt_rsn { if let Err(e) = re_presntmnt_rsn_value.validate() { return Err(e); } }
		if let Some(ref seq_nb_value) = self.seq_nb { if let Err(e) = seq_nb_value.validate() { return Err(e); } }
		if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
		if let Some(ref pdct_value) = self.pdct { if let Err(e) = pdct_value.validate() { return Err(e); } }
		if let Some(ref vldtn_seq_nb_value) = self.vldtn_seq_nb { if let Err(e) = vldtn_seq_nb_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CardPaymentServiceType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardPaymentServiceType2Code {
	#[default]
	#[serde(rename = "AGGR")]
	CodeAGGR,
	#[serde(rename = "DCCV")]
	CodeDCCV,
	#[serde(rename = "GRTT")]
	CodeGRTT,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "LOYT")]
	CodeLOYT,
	#[serde(rename = "NRES")]
	CodeNRES,
	#[serde(rename = "PUCO")]
	CodePUCO,
	#[serde(rename = "RECP")]
	CodeRECP,
	#[serde(rename = "SOAF")]
	CodeSOAF,
	#[serde(rename = "UNAF")]
	CodeUNAF,
	#[serde(rename = "VCAU")]
	CodeVCAU,
}

impl CardPaymentServiceType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CardSecurityInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardSecurityInformation1 {
	#[serde(rename = "CSCMgmt")]
	pub csc_mgmt: CSCManagement1Code,
	#[serde(rename = "CSCVal", skip_serializing_if = "Option::is_none")]
	pub csc_val: Option<Min3Max4NumericText>,
}

impl CardSecurityInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.csc_mgmt.validate() { return Err(e); }
		if let Some(ref csc_val_value) = self.csc_val { if let Err(e) = csc_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CardSequenceNumberRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardSequenceNumberRange1 {
	#[serde(rename = "FrstTx", skip_serializing_if = "Option::is_none")]
	pub frst_tx: Option<Max35Text>,
	#[serde(rename = "LastTx", skip_serializing_if = "Option::is_none")]
	pub last_tx: Option<Max35Text>,
}

impl CardSequenceNumberRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref frst_tx_value) = self.frst_tx { if let Err(e) = frst_tx_value.validate() { return Err(e); } }
		if let Some(ref last_tx_value) = self.last_tx { if let Err(e) = last_tx_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CardTransaction17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardTransaction17 {
	#[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
	pub card: Option<PaymentCard4>,
	#[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
	pub poi: Option<PointOfInteraction1>,
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<CardTransaction3Choice>,
	#[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
	pub pre_pd_acct: Option<CashAccount38>,
}

impl CardTransaction17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref card_value) = self.card { if let Err(e) = card_value.validate() { return Err(e); } }
		if let Some(ref poi_value) = self.poi { if let Err(e) = poi_value.validate() { return Err(e); } }
		if let Some(ref tx_value) = self.tx { if let Err(e) = tx_value.validate() { return Err(e); } }
		if let Some(ref pre_pd_acct_value) = self.pre_pd_acct { if let Err(e) = pre_pd_acct_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CardTransaction3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardTransaction3Choice {
	#[serde(rename = "Aggtd", skip_serializing_if = "Option::is_none")]
	pub aggtd: Option<CardAggregated2>,
	#[serde(rename = "Indv", skip_serializing_if = "Option::is_none")]
	pub indv: Option<CardIndividualTransaction2>,
}

impl CardTransaction3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref aggtd_value) = self.aggtd { if let Err(e) = aggtd_value.validate() { return Err(e); } }
		if let Some(ref indv_value) = self.indv { if let Err(e) = indv_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CardholderAuthentication2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardholderAuthentication2 {
	#[serde(rename = "AuthntcnMtd")]
	pub authntcn_mtd: AuthenticationMethod1Code,
	#[serde(rename = "AuthntcnNtty")]
	pub authntcn_ntty: AuthenticationEntity1Code,
}

impl CardholderAuthentication2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.authntcn_mtd.validate() { return Err(e); }
		if let Err(e) = self.authntcn_ntty.validate() { return Err(e); }
		Ok(())
	}
}


// CardholderVerificationCapability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardholderVerificationCapability1Code {
	#[default]
	#[serde(rename = "MNSG")]
	CodeMNSG,
	#[serde(rename = "NPIN")]
	CodeNPIN,
	#[serde(rename = "FCPN")]
	CodeFCPN,
	#[serde(rename = "FEPN")]
	CodeFEPN,
	#[serde(rename = "FDSG")]
	CodeFDSG,
	#[serde(rename = "FBIO")]
	CodeFBIO,
	#[serde(rename = "MNVR")]
	CodeMNVR,
	#[serde(rename = "FBIG")]
	CodeFBIG,
	#[serde(rename = "APKI")]
	CodeAPKI,
	#[serde(rename = "PKIS")]
	CodePKIS,
	#[serde(rename = "CHDT")]
	CodeCHDT,
	#[serde(rename = "SCEC")]
	CodeSCEC,
}

impl CardholderVerificationCapability1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CashAccount38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount38 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref prxy_value) = self.prxy { if let Err(e) = prxy_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CashAccount39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount39 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification1>,
	#[serde(rename = "Ownr", skip_serializing_if = "Option::is_none")]
	pub ownr: Option<PartyIdentification135>,
	#[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl CashAccount39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref prxy_value) = self.prxy { if let Err(e) = prxy_value.validate() { return Err(e); } }
		if let Some(ref ownr_value) = self.ownr { if let Err(e) = ownr_value.validate() { return Err(e); } }
		if let Some(ref svcr_value) = self.svcr { if let Err(e) = svcr_value.validate() { return Err(e); } }
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


// CashAvailability1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAvailability1 {
	#[serde(rename = "Dt")]
	pub dt: CashAvailabilityDate1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
}

impl CashAvailability1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dt.validate() { return Err(e); }
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		Ok(())
	}
}


// CashAvailabilityDate1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAvailabilityDate1Choice {
	#[serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none")]
	pub nb_of_days: Option<Max15PlusSignedNumericText>,
	#[serde(rename = "ActlDt", skip_serializing_if = "Option::is_none")]
	pub actl_dt: Option<String>,
}

impl CashAvailabilityDate1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nb_of_days_value) = self.nb_of_days { if let Err(e) = nb_of_days_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CashDeposit1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashDeposit1 {
	#[serde(rename = "NoteDnmtn")]
	pub note_dnmtn: ActiveCurrencyAndAmount,
	#[serde(rename = "NbOfNotes")]
	pub nb_of_notes: Max15NumericText,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}

impl CashDeposit1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.note_dnmtn.validate() { return Err(e); }
		if let Err(e) = self.nb_of_notes.validate() { return Err(e); }
		if let Err(e) = self.amt.validate() { return Err(e); }
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


// CopyDuplicate1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CopyDuplicate1Code {
	#[default]
	#[serde(rename = "CODU")]
	CodeCODU,
	#[serde(rename = "COPY")]
	CodeCOPY,
	#[serde(rename = "DUPL")]
	CodeDUPL,
}

impl CopyDuplicate1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CorporateAction9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateAction9 {
	#[serde(rename = "EvtTp")]
	pub evt_tp: Max35Text,
	#[serde(rename = "EvtId")]
	pub evt_id: Max35Text,
}

impl CorporateAction9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.evt_tp.validate() { return Err(e); }
		if let Err(e) = self.evt_id.validate() { return Err(e); }
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


// CurrencyExchange5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange5 {
	#[serde(rename = "SrcCcy")]
	pub src_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "TrgtCcy", skip_serializing_if = "Option::is_none")]
	pub trgt_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
	pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
	#[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
	pub ctrct_id: Option<Max35Text>,
	#[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
	pub qtn_dt: Option<String>,
}

impl CurrencyExchange5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.src_ccy.validate() { return Err(e); }
		if let Some(ref trgt_ccy_value) = self.trgt_ccy { if let Err(e) = trgt_ccy_value.validate() { return Err(e); } }
		if let Some(ref unit_ccy_value) = self.unit_ccy { if let Err(e) = unit_ccy_value.validate() { return Err(e); } }
		if let Some(ref ctrct_id_value) = self.ctrct_id { if let Err(e) = ctrct_id_value.validate() { return Err(e); } }
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


// DateOrDateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrDateTimePeriod1Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DatePeriod2>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<DateTimePeriod1>,
}

impl DateOrDateTimePeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref dt_value) = self.dt { if let Err(e) = dt_value.validate() { return Err(e); } }
		if let Some(ref dt_tm_value) = self.dt_tm { if let Err(e) = dt_tm_value.validate() { return Err(e); } }
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


// DisplayCapabilities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisplayCapabilities1 {
	#[serde(rename = "DispTp")]
	pub disp_tp: UserInterface2Code,
	#[serde(rename = "NbOfLines")]
	pub nb_of_lines: Max3NumericText,
	#[serde(rename = "LineWidth")]
	pub line_width: Max3NumericText,
}

impl DisplayCapabilities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.disp_tp.validate() { return Err(e); }
		if let Err(e) = self.nb_of_lines.validate() { return Err(e); }
		if let Err(e) = self.line_width.validate() { return Err(e); }
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


// EntryDetails9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryDetails9 {
	#[serde(rename = "Btch", skip_serializing_if = "Option::is_none")]
	pub btch: Option<BatchInformation2>,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<EntryTransaction10>>,
}

impl EntryDetails9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref btch_value) = self.btch { if let Err(e) = btch_value.validate() { return Err(e); } }
		if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// EntryStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalEntryStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl EntryStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// EntryTransaction10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryTransaction10 {
	#[serde(rename = "Refs", skip_serializing_if = "Option::is_none")]
	pub refs: Option<TransactionReferences6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none")]
	pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
	pub chrgs: Option<Charges6>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<TransactionInterest4>,
	#[serde(rename = "RltdPties", skip_serializing_if = "Option::is_none")]
	pub rltd_pties: Option<TransactionParties6>,
	#[serde(rename = "RltdAgts", skip_serializing_if = "Option::is_none")]
	pub rltd_agts: Option<TransactionAgents5>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Purpose2Choice>,
	#[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[serde(rename = "RltdDts", skip_serializing_if = "Option::is_none")]
	pub rltd_dts: Option<TransactionDates3>,
	#[serde(rename = "RltdPric", skip_serializing_if = "Option::is_none")]
	pub rltd_pric: Option<TransactionPrice4Choice>,
	#[serde(rename = "RltdQties", skip_serializing_if = "Option::is_none")]
	pub rltd_qties: Option<Vec<TransactionQuantities3Choice>>,
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<SecurityIdentification19>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxInformation8>,
	#[serde(rename = "RtrInf", skip_serializing_if = "Option::is_none")]
	pub rtr_inf: Option<PaymentReturnReason5>,
	#[serde(rename = "CorpActn", skip_serializing_if = "Option::is_none")]
	pub corp_actn: Option<CorporateAction9>,
	#[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
	pub sfkpg_acct: Option<SecuritiesAccount19>,
	#[serde(rename = "CshDpst", skip_serializing_if = "Option::is_none")]
	pub csh_dpst: Option<Vec<CashDeposit1>>,
	#[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
	pub card_tx: Option<CardTransaction17>,
	#[serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none")]
	pub addtl_tx_inf: Option<Max500Text>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl EntryTransaction10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref refs_value) = self.refs { if let Err(e) = refs_value.validate() { return Err(e); } }
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
		if let Some(ref amt_dtls_value) = self.amt_dtls { if let Err(e) = amt_dtls_value.validate() { return Err(e); } }
		if let Some(ref avlbty_vec) = self.avlbty { for item in avlbty_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref bk_tx_cd_value) = self.bk_tx_cd { if let Err(e) = bk_tx_cd_value.validate() { return Err(e); } }
		if let Some(ref chrgs_value) = self.chrgs { if let Err(e) = chrgs_value.validate() { return Err(e); } }
		if let Some(ref intrst_value) = self.intrst { if let Err(e) = intrst_value.validate() { return Err(e); } }
		if let Some(ref rltd_pties_value) = self.rltd_pties { if let Err(e) = rltd_pties_value.validate() { return Err(e); } }
		if let Some(ref rltd_agts_value) = self.rltd_agts { if let Err(e) = rltd_agts_value.validate() { return Err(e); } }
		if let Some(ref lcl_instrm_value) = self.lcl_instrm { if let Err(e) = lcl_instrm_value.validate() { return Err(e); } }
		if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
		if let Some(ref rltd_rmt_inf_vec) = self.rltd_rmt_inf { for item in rltd_rmt_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref rmt_inf_value) = self.rmt_inf { if let Err(e) = rmt_inf_value.validate() { return Err(e); } }
		if let Some(ref rltd_dts_value) = self.rltd_dts { if let Err(e) = rltd_dts_value.validate() { return Err(e); } }
		if let Some(ref rltd_pric_value) = self.rltd_pric { if let Err(e) = rltd_pric_value.validate() { return Err(e); } }
		if let Some(ref rltd_qties_vec) = self.rltd_qties { for item in rltd_qties_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref fin_instrm_id_value) = self.fin_instrm_id { if let Err(e) = fin_instrm_id_value.validate() { return Err(e); } }
		if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
		if let Some(ref rtr_inf_value) = self.rtr_inf { if let Err(e) = rtr_inf_value.validate() { return Err(e); } }
		if let Some(ref corp_actn_value) = self.corp_actn { if let Err(e) = corp_actn_value.validate() { return Err(e); } }
		if let Some(ref sfkpg_acct_value) = self.sfkpg_acct { if let Err(e) = sfkpg_acct_value.validate() { return Err(e); } }
		if let Some(ref csh_dpst_vec) = self.csh_dpst { for item in csh_dpst_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref card_tx_value) = self.card_tx { if let Err(e) = card_tx_value.validate() { return Err(e); } }
		if let Some(ref addtl_tx_inf_value) = self.addtl_tx_inf { if let Err(e) = addtl_tx_inf_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// Exact3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact3NumericText {
	#[serde(rename = "$value")]
	pub exact3_numeric_text: String,
}

impl Exact3NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{3}").unwrap();
		if !pattern.is_match(&self.exact3_numeric_text) {
			return Err(ValidationError::new(1005, "exact3_numeric_text does not match the required pattern".to_string()));
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


// ExternalCardTransactionCategory1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCardTransactionCategory1Code {
	#[serde(rename = "$value")]
	pub external_card_transaction_category1_code: String,
}

impl ExternalCardTransactionCategory1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_card_transaction_category1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_card_transaction_category1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_card_transaction_category1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_card_transaction_category1_code exceeds the maximum length of 4".to_string()));
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


// ExternalEntryStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEntryStatus1Code {
	#[serde(rename = "$value")]
	pub external_entry_status1_code: String,
}

impl ExternalEntryStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_entry_status1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_entry_status1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_entry_status1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_entry_status1_code exceeds the maximum length of 4".to_string()));
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


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_identification_type1_code: String,
}

impl ExternalFinancialInstrumentIdentificationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_financial_instrument_identification_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_identification_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_financial_instrument_identification_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_financial_instrument_identification_type1_code exceeds the maximum length of 4".to_string()));
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


// ExternalRePresentmentReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalRePresentmentReason1Code {
	#[serde(rename = "$value")]
	pub external_re_presentment_reason1_code: String,
}

impl ExternalRePresentmentReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_re_presentment_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_re_presentment_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_re_presentment_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_re_presentment_reason1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalReportingSource1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalReportingSource1Code {
	#[serde(rename = "$value")]
	pub external_reporting_source1_code: String,
}

impl ExternalReportingSource1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_reporting_source1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_reporting_source1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_reporting_source1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_reporting_source1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalReturnReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalReturnReason1Code {
	#[serde(rename = "$value")]
	pub external_return_reason1_code: String,
}

impl ExternalReturnReason1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_return_reason1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_return_reason1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_return_reason1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_return_reason1_code exceeds the maximum length of 4".to_string()));
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


// ExternalTechnicalInputChannel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalTechnicalInputChannel1Code {
	#[serde(rename = "$value")]
	pub external_technical_input_channel1_code: String,
}

impl ExternalTechnicalInputChannel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_technical_input_channel1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_technical_input_channel1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_technical_input_channel1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_technical_input_channel1_code exceeds the maximum length of 4".to_string()));
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


// FinancialInstrumentQuantity1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<f64>,
	#[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
	pub amtsd_val: Option<f64>,
}

impl FinancialInstrumentQuantity1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FromToAmountRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FromToAmountRange1 {
	#[serde(rename = "FrAmt")]
	pub fr_amt: AmountRangeBoundary1,
	#[serde(rename = "ToAmt")]
	pub to_amt: AmountRangeBoundary1,
}

impl FromToAmountRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fr_amt.validate() { return Err(e); }
		if let Err(e) = self.to_amt.validate() { return Err(e); }
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


// GenericIdentification32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification32 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<PartyType3Code>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<PartyType4Code>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
}

impl GenericIdentification32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		if let Some(ref shrt_nm_value) = self.shrt_nm { if let Err(e) = shrt_nm_value.validate() { return Err(e); } }
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


// GroupHeader81 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader81 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none")]
	pub msg_rcpt: Option<PartyIdentification135>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max500Text>,
}

impl GroupHeader81 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref msg_rcpt_value) = self.msg_rcpt { if let Err(e) = msg_rcpt_value.validate() { return Err(e); } }
		if let Some(ref msg_pgntn_value) = self.msg_pgntn { if let Err(e) = msg_pgntn_value.validate() { return Err(e); } }
		if let Some(ref orgnl_biz_qry_value) = self.orgnl_biz_qry { if let Err(e) = orgnl_biz_qry_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
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


// ISO2ALanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISO2ALanguageCode {
	#[serde(rename = "$value")]
	pub iso2_a_language_code: String,
}

impl ISO2ALanguageCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-z]{2,2}").unwrap();
		if !pattern.is_match(&self.iso2_a_language_code) {
			return Err(ValidationError::new(1005, "iso2_a_language_code does not match the required pattern".to_string()));
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


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}

impl ISOYearMonth {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl IdentificationSource3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ImpliedCurrencyAmountRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAmountRange1Choice {
	#[serde(rename = "FrAmt", skip_serializing_if = "Option::is_none")]
	pub fr_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "ToAmt", skip_serializing_if = "Option::is_none")]
	pub to_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "FrToAmt", skip_serializing_if = "Option::is_none")]
	pub fr_to_amt: Option<FromToAmountRange1>,
	#[serde(rename = "EQAmt", skip_serializing_if = "Option::is_none")]
	pub eq_amt: Option<f64>,
	#[serde(rename = "NEQAmt", skip_serializing_if = "Option::is_none")]
	pub neq_amt: Option<f64>,
}

impl ImpliedCurrencyAmountRange1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fr_amt_value) = self.fr_amt { if let Err(e) = fr_amt_value.validate() { return Err(e); } }
		if let Some(ref to_amt_value) = self.to_amt { if let Err(e) = to_amt_value.validate() { return Err(e); } }
		if let Some(ref fr_to_amt_value) = self.fr_to_amt { if let Err(e) = fr_to_amt_value.validate() { return Err(e); } }
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


// InterestRecord2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRecord2 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<InterestType1Choice>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<Rate4>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max35Text>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxCharges2>,
}

impl InterestRecord2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref rate_value) = self.rate { if let Err(e) = rate_value.validate() { return Err(e); } }
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InterestType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InterestType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InterestType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InterestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestType1Code {
	#[default]
	#[serde(rename = "INDY")]
	CodeINDY,
	#[serde(rename = "OVRN")]
	CodeOVRN,
}

impl InterestType1Code {
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


// MessageIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification2 {
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Max35Text>,
}

impl MessageIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref msg_nm_id_value) = self.msg_nm_id { if let Err(e) = msg_nm_id_value.validate() { return Err(e); } }
		if let Some(ref msg_id_value) = self.msg_id { if let Err(e) = msg_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Min2Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Min2Max3NumericText {
	#[serde(rename = "$value")]
	pub min2_max3_numeric_text: String,
}

impl Min2Max3NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{2,3}").unwrap();
		if !pattern.is_match(&self.min2_max3_numeric_text) {
			return Err(ValidationError::new(1005, "min2_max3_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Min3Max4NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Min3Max4NumericText {
	#[serde(rename = "$value")]
	pub min3_max4_numeric_text: String,
}

impl Min3Max4NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{3,4}").unwrap();
		if !pattern.is_match(&self.min3_max4_numeric_text) {
			return Err(ValidationError::new(1005, "min3_max4_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Min8Max28NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Min8Max28NumericText {
	#[serde(rename = "$value")]
	pub min8_max28_numeric_text: String,
}

impl Min8Max28NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{8,28}").unwrap();
		if !pattern.is_match(&self.min8_max28_numeric_text) {
			return Err(ValidationError::new(1005, "min8_max28_numeric_text does not match the required pattern".to_string()));
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


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "$value")]
	pub non_negative_decimal_number: f64,
}

impl NonNegativeDecimalNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.non_negative_decimal_number < 0.000000 {
			return Err(ValidationError::new(1003, "non_negative_decimal_number is less than the minimum value of 0.000000".to_string()));
		}
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


// NumberAndSumOfTransactions1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions1 {
	#[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
	pub nb_of_ntries: Option<Max15NumericText>,
	#[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
	pub sum: Option<f64>,
}

impl NumberAndSumOfTransactions1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nb_of_ntries_value) = self.nb_of_ntries { if let Err(e) = nb_of_ntries_value.validate() { return Err(e); } }
		Ok(())
	}
}


// NumberAndSumOfTransactions4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions4 {
	#[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
	pub nb_of_ntries: Option<Max15NumericText>,
	#[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none")]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
}

impl NumberAndSumOfTransactions4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nb_of_ntries_value) = self.nb_of_ntries { if let Err(e) = nb_of_ntries_value.validate() { return Err(e); } }
		if let Some(ref ttl_net_ntry_value) = self.ttl_net_ntry { if let Err(e) = ttl_net_ntry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OnLineCapability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OnLineCapability1Code {
	#[default]
	#[serde(rename = "OFLN")]
	CodeOFLN,
	#[serde(rename = "ONLN")]
	CodeONLN,
	#[serde(rename = "SMON")]
	CodeSMON,
}

impl OnLineCapability1Code {
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


// OriginalAndCurrentQuantities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalAndCurrentQuantities1 {
	#[serde(rename = "FaceAmt")]
	pub face_amt: f64,
	#[serde(rename = "AmtsdVal")]
	pub amtsd_val: f64,
}

impl OriginalAndCurrentQuantities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OriginalBusinessQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref msg_nm_id_value) = self.msg_nm_id { if let Err(e) = msg_nm_id_value.validate() { return Err(e); } }
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


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<Max16Text>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref sfx_value) = self.sfx { if let Err(e) = sfx_value.validate() { return Err(e); } }
		if let Err(e) = self.tp.validate() { return Err(e); }
		Ok(())
	}
}


// POIComponentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum POIComponentType1Code {
	#[default]
	#[serde(rename = "SOFT")]
	CodeSOFT,
	#[serde(rename = "EMVK")]
	CodeEMVK,
	#[serde(rename = "EMVO")]
	CodeEMVO,
	#[serde(rename = "MRIT")]
	CodeMRIT,
	#[serde(rename = "CHIT")]
	CodeCHIT,
	#[serde(rename = "SECM")]
	CodeSECM,
	#[serde(rename = "PEDV")]
	CodePEDV,
}

impl POIComponentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pg_nb.validate() { return Err(e); }
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


// PartyType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyType3Code {
	#[default]
	#[serde(rename = "OPOI")]
	CodeOPOI,
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "ACCP")]
	CodeACCP,
	#[serde(rename = "ITAG")]
	CodeITAG,
	#[serde(rename = "ACQR")]
	CodeACQR,
	#[serde(rename = "CISS")]
	CodeCISS,
	#[serde(rename = "DLIS")]
	CodeDLIS,
}

impl PartyType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PartyType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyType4Code {
	#[default]
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "ACCP")]
	CodeACCP,
	#[serde(rename = "ITAG")]
	CodeITAG,
	#[serde(rename = "ACQR")]
	CodeACQR,
	#[serde(rename = "CISS")]
	CodeCISS,
	#[serde(rename = "TAXH")]
	CodeTAXH,
}

impl PartyType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentCard4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCard4 {
	#[serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none")]
	pub plain_card_data: Option<PlainCardData1>,
	#[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
	pub card_ctry_cd: Option<Exact3NumericText>,
	#[serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none")]
	pub card_brnd: Option<GenericIdentification1>,
	#[serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none")]
	pub addtl_card_data: Option<Max70Text>,
}

impl PaymentCard4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref plain_card_data_value) = self.plain_card_data { if let Err(e) = plain_card_data_value.validate() { return Err(e); } }
		if let Some(ref card_ctry_cd_value) = self.card_ctry_cd { if let Err(e) = card_ctry_cd_value.validate() { return Err(e); } }
		if let Some(ref card_brnd_value) = self.card_brnd { if let Err(e) = card_brnd_value.validate() { return Err(e); } }
		if let Some(ref addtl_card_data_value) = self.addtl_card_data { if let Err(e) = addtl_card_data_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentContext3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentContext3 {
	#[serde(rename = "CardPres", skip_serializing_if = "Option::is_none")]
	pub card_pres: Option<bool>,
	#[serde(rename = "CrdhldrPres", skip_serializing_if = "Option::is_none")]
	pub crdhldr_pres: Option<bool>,
	#[serde(rename = "OnLineCntxt", skip_serializing_if = "Option::is_none")]
	pub on_line_cntxt: Option<bool>,
	#[serde(rename = "AttndncCntxt", skip_serializing_if = "Option::is_none")]
	pub attndnc_cntxt: Option<AttendanceContext1Code>,
	#[serde(rename = "TxEnvt", skip_serializing_if = "Option::is_none")]
	pub tx_envt: Option<TransactionEnvironment1Code>,
	#[serde(rename = "TxChanl", skip_serializing_if = "Option::is_none")]
	pub tx_chanl: Option<TransactionChannel1Code>,
	#[serde(rename = "AttndntMsgCpbl", skip_serializing_if = "Option::is_none")]
	pub attndnt_msg_cpbl: Option<bool>,
	#[serde(rename = "AttndntLang", skip_serializing_if = "Option::is_none")]
	pub attndnt_lang: Option<ISO2ALanguageCode>,
	#[serde(rename = "CardDataNtryMd")]
	pub card_data_ntry_md: CardDataReading1Code,
	#[serde(rename = "FllbckInd", skip_serializing_if = "Option::is_none")]
	pub fllbck_ind: Option<bool>,
	#[serde(rename = "AuthntcnMtd", skip_serializing_if = "Option::is_none")]
	pub authntcn_mtd: Option<CardholderAuthentication2>,
}

impl PaymentContext3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref attndnc_cntxt_value) = self.attndnc_cntxt { if let Err(e) = attndnc_cntxt_value.validate() { return Err(e); } }
		if let Some(ref tx_envt_value) = self.tx_envt { if let Err(e) = tx_envt_value.validate() { return Err(e); } }
		if let Some(ref tx_chanl_value) = self.tx_chanl { if let Err(e) = tx_chanl_value.validate() { return Err(e); } }
		if let Some(ref attndnt_lang_value) = self.attndnt_lang { if let Err(e) = attndnt_lang_value.validate() { return Err(e); } }
		if let Err(e) = self.card_data_ntry_md.validate() { return Err(e); }
		if let Some(ref authntcn_mtd_value) = self.authntcn_mtd { if let Err(e) = authntcn_mtd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentReturnReason5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentReturnReason5 {
	#[serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none")]
	pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
	pub orgtr: Option<PartyIdentification135>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<ReturnReason5Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<Max105Text>>,
}

impl PaymentReturnReason5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref orgnl_bk_tx_cd_value) = self.orgnl_bk_tx_cd { if let Err(e) = orgnl_bk_tx_cd_value.validate() { return Err(e); } }
		if let Some(ref orgtr_value) = self.orgtr { if let Err(e) = orgtr_value.validate() { return Err(e); } }
		if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// PlainCardData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlainCardData1 {
	#[serde(rename = "PAN")]
	pub pan: Min8Max28NumericText,
	#[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
	pub card_seq_nb: Option<Min2Max3NumericText>,
	#[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
	pub fctv_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: String,
	#[serde(rename = "SvcCd", skip_serializing_if = "Option::is_none")]
	pub svc_cd: Option<Exact3NumericText>,
	#[serde(rename = "TrckData", skip_serializing_if = "Option::is_none")]
	pub trck_data: Option<Vec<TrackData1>>,
	#[serde(rename = "CardSctyCd", skip_serializing_if = "Option::is_none")]
	pub card_scty_cd: Option<CardSecurityInformation1>,
}

impl PlainCardData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pan.validate() { return Err(e); }
		if let Some(ref card_seq_nb_value) = self.card_seq_nb { if let Err(e) = card_seq_nb_value.validate() { return Err(e); } }
		if let Some(ref svc_cd_value) = self.svc_cd { if let Err(e) = svc_cd_value.validate() { return Err(e); } }
		if let Some(ref trck_data_vec) = self.trck_data { for item in trck_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref card_scty_cd_value) = self.card_scty_cd { if let Err(e) = card_scty_cd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PointOfInteraction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteraction1 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification32,
	#[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
	pub sys_nm: Option<Max70Text>,
	#[serde(rename = "GrpId", skip_serializing_if = "Option::is_none")]
	pub grp_id: Option<Max35Text>,
	#[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
	pub cpblties: Option<PointOfInteractionCapabilities1>,
	#[serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none")]
	pub cmpnt: Option<Vec<PointOfInteractionComponent1>>,
}

impl PointOfInteraction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref sys_nm_value) = self.sys_nm { if let Err(e) = sys_nm_value.validate() { return Err(e); } }
		if let Some(ref grp_id_value) = self.grp_id { if let Err(e) = grp_id_value.validate() { return Err(e); } }
		if let Some(ref cpblties_value) = self.cpblties { if let Err(e) = cpblties_value.validate() { return Err(e); } }
		if let Some(ref cmpnt_vec) = self.cmpnt { for item in cmpnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// PointOfInteractionCapabilities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteractionCapabilities1 {
	#[serde(rename = "CardRdngCpblties", skip_serializing_if = "Option::is_none")]
	pub card_rdng_cpblties: Option<Vec<CardDataReading1Code>>,
	#[serde(rename = "CrdhldrVrfctnCpblties", skip_serializing_if = "Option::is_none")]
	pub crdhldr_vrfctn_cpblties: Option<Vec<CardholderVerificationCapability1Code>>,
	#[serde(rename = "OnLineCpblties", skip_serializing_if = "Option::is_none")]
	pub on_line_cpblties: Option<OnLineCapability1Code>,
	#[serde(rename = "DispCpblties", skip_serializing_if = "Option::is_none")]
	pub disp_cpblties: Option<Vec<DisplayCapabilities1>>,
	#[serde(rename = "PrtLineWidth", skip_serializing_if = "Option::is_none")]
	pub prt_line_width: Option<Max3NumericText>,
}

impl PointOfInteractionCapabilities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref card_rdng_cpblties_vec) = self.card_rdng_cpblties { for item in card_rdng_cpblties_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref crdhldr_vrfctn_cpblties_vec) = self.crdhldr_vrfctn_cpblties { for item in crdhldr_vrfctn_cpblties_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref on_line_cpblties_value) = self.on_line_cpblties { if let Err(e) = on_line_cpblties_value.validate() { return Err(e); } }
		if let Some(ref disp_cpblties_vec) = self.disp_cpblties { for item in disp_cpblties_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref prt_line_width_value) = self.prt_line_width { if let Err(e) = prt_line_width_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PointOfInteractionComponent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteractionComponent1 {
	#[serde(rename = "POICmpntTp")]
	pub poi_cmpnt_tp: POIComponentType1Code,
	#[serde(rename = "ManfctrId", skip_serializing_if = "Option::is_none")]
	pub manfctr_id: Option<Max35Text>,
	#[serde(rename = "Mdl", skip_serializing_if = "Option::is_none")]
	pub mdl: Option<Max35Text>,
	#[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
	pub vrsn_nb: Option<Max16Text>,
	#[serde(rename = "SrlNb", skip_serializing_if = "Option::is_none")]
	pub srl_nb: Option<Max35Text>,
	#[serde(rename = "ApprvlNb", skip_serializing_if = "Option::is_none")]
	pub apprvl_nb: Option<Vec<Max70Text>>,
}

impl PointOfInteractionComponent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.poi_cmpnt_tp.validate() { return Err(e); }
		if let Some(ref manfctr_id_value) = self.manfctr_id { if let Err(e) = manfctr_id_value.validate() { return Err(e); } }
		if let Some(ref mdl_value) = self.mdl { if let Err(e) = mdl_value.validate() { return Err(e); } }
		if let Some(ref vrsn_nb_value) = self.vrsn_nb { if let Err(e) = vrsn_nb_value.validate() { return Err(e); } }
		if let Some(ref srl_nb_value) = self.srl_nb { if let Err(e) = srl_nb_value.validate() { return Err(e); } }
		if let Some(ref apprvl_nb_vec) = self.apprvl_nb { for item in apprvl_nb_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// Price7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Price7 {
	#[serde(rename = "Tp")]
	pub tp: YieldedOrValueType1Choice,
	#[serde(rename = "Val")]
	pub val: PriceRateOrAmount3Choice,
}

impl Price7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.val.validate() { return Err(e); }
		Ok(())
	}
}


// PriceRateOrAmount3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceRateOrAmount3Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}

impl PriceRateOrAmount3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceValueType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceValueType1Code {
	#[default]
	#[serde(rename = "DISC")]
	CodeDISC,
	#[serde(rename = "PREM")]
	CodePREM,
	#[serde(rename = "PARV")]
	CodePARV,
}

impl PriceValueType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Product2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Product2 {
	#[serde(rename = "PdctCd")]
	pub pdct_cd: Max70Text,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure1Code>,
	#[serde(rename = "PdctQty", skip_serializing_if = "Option::is_none")]
	pub pdct_qty: Option<f64>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<f64>,
	#[serde(rename = "PdctAmt", skip_serializing_if = "Option::is_none")]
	pub pdct_amt: Option<f64>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
	#[serde(rename = "AddtlPdctInf", skip_serializing_if = "Option::is_none")]
	pub addtl_pdct_inf: Option<Max35Text>,
}

impl Product2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pdct_cd.validate() { return Err(e); }
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
		if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
		if let Some(ref addtl_pdct_inf_value) = self.addtl_pdct_inf { if let Err(e) = addtl_pdct_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ProprietaryAgent4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryAgent4 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Agt")]
	pub agt: BranchAndFinancialInstitutionIdentification6,
}

impl ProprietaryAgent4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.agt.validate() { return Err(e); }
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


// ProprietaryDate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryDate3 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Dt")]
	pub dt: DateAndDateTime2Choice,
}

impl ProprietaryDate3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.dt.validate() { return Err(e); }
		Ok(())
	}
}


// ProprietaryParty5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryParty5 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Pty")]
	pub pty: Party40Choice,
}

impl ProprietaryParty5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.pty.validate() { return Err(e); }
		Ok(())
	}
}


// ProprietaryPrice2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryPrice2 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Pric")]
	pub pric: ActiveOrHistoricCurrencyAndAmount,
}

impl ProprietaryPrice2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.pric.validate() { return Err(e); }
		Ok(())
	}
}


// ProprietaryQuantity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryQuantity1 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Qty")]
	pub qty: Max35Text,
}

impl ProprietaryQuantity1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.qty.validate() { return Err(e); }
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


// Rate4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rate4 {
	#[serde(rename = "Tp")]
	pub tp: RateType4Choice,
	#[serde(rename = "VldtyRg", skip_serializing_if = "Option::is_none")]
	pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}

impl Rate4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Some(ref vldty_rg_value) = self.vldty_rg { if let Err(e) = vldty_rg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RateType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateType4Choice {
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max35Text>,
}

impl RateType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
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


// RemittanceInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation16 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<Max140Text>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation16>>,
}

impl RemittanceInformation16 {
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


// ReportEntry10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportEntry10 {
	#[serde(rename = "NtryRef", skip_serializing_if = "Option::is_none")]
	pub ntry_ref: Option<Max35Text>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "RvslInd", skip_serializing_if = "Option::is_none")]
	pub rvsl_ind: Option<bool>,
	#[serde(rename = "Sts")]
	pub sts: EntryStatus1Choice,
	#[serde(rename = "BookgDt", skip_serializing_if = "Option::is_none")]
	pub bookg_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_ref: Option<Max35Text>,
	#[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[serde(rename = "ComssnWvrInd", skip_serializing_if = "Option::is_none")]
	pub comssn_wvr_ind: Option<bool>,
	#[serde(rename = "AddtlInfInd", skip_serializing_if = "Option::is_none")]
	pub addtl_inf_ind: Option<MessageIdentification2>,
	#[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
	pub chrgs: Option<Charges6>,
	#[serde(rename = "TechInptChanl", skip_serializing_if = "Option::is_none")]
	pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<TransactionInterest4>,
	#[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
	pub card_tx: Option<CardEntry4>,
	#[serde(rename = "NtryDtls", skip_serializing_if = "Option::is_none")]
	pub ntry_dtls: Option<Vec<EntryDetails9>>,
	#[serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none")]
	pub addtl_ntry_inf: Option<Max500Text>,
}

impl ReportEntry10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ntry_ref_value) = self.ntry_ref { if let Err(e) = ntry_ref_value.validate() { return Err(e); } }
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		if let Err(e) = self.sts.validate() { return Err(e); }
		if let Some(ref bookg_dt_value) = self.bookg_dt { if let Err(e) = bookg_dt_value.validate() { return Err(e); } }
		if let Some(ref val_dt_value) = self.val_dt { if let Err(e) = val_dt_value.validate() { return Err(e); } }
		if let Some(ref acct_svcr_ref_value) = self.acct_svcr_ref { if let Err(e) = acct_svcr_ref_value.validate() { return Err(e); } }
		if let Some(ref avlbty_vec) = self.avlbty { for item in avlbty_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Err(e) = self.bk_tx_cd.validate() { return Err(e); }
		if let Some(ref addtl_inf_ind_value) = self.addtl_inf_ind { if let Err(e) = addtl_inf_ind_value.validate() { return Err(e); } }
		if let Some(ref amt_dtls_value) = self.amt_dtls { if let Err(e) = amt_dtls_value.validate() { return Err(e); } }
		if let Some(ref chrgs_value) = self.chrgs { if let Err(e) = chrgs_value.validate() { return Err(e); } }
		if let Some(ref tech_inpt_chanl_value) = self.tech_inpt_chanl { if let Err(e) = tech_inpt_chanl_value.validate() { return Err(e); } }
		if let Some(ref intrst_value) = self.intrst { if let Err(e) = intrst_value.validate() { return Err(e); } }
		if let Some(ref card_tx_value) = self.card_tx { if let Err(e) = card_tx_value.validate() { return Err(e); } }
		if let Some(ref ntry_dtls_vec) = self.ntry_dtls { for item in ntry_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref addtl_ntry_inf_value) = self.addtl_ntry_inf { if let Err(e) = addtl_ntry_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReportingSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingSource1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalReportingSource1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ReportingSource1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReturnReason5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnReason5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalReturnReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ReturnReason5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl SecurityIdentification19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
		if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SequenceRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1 {
	#[serde(rename = "FrSeq")]
	pub fr_seq: Max35Text,
	#[serde(rename = "ToSeq")]
	pub to_seq: Max35Text,
}

impl SequenceRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fr_seq.validate() { return Err(e); }
		if let Err(e) = self.to_seq.validate() { return Err(e); }
		Ok(())
	}
}


// SequenceRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1Choice {
	#[serde(rename = "FrSeq", skip_serializing_if = "Option::is_none")]
	pub fr_seq: Option<Max35Text>,
	#[serde(rename = "ToSeq", skip_serializing_if = "Option::is_none")]
	pub to_seq: Option<Max35Text>,
	#[serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none")]
	pub fr_to_seq: Option<Vec<SequenceRange1>>,
	#[serde(rename = "EQSeq", skip_serializing_if = "Option::is_none")]
	pub eq_seq: Option<Vec<Max35Text>>,
	#[serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none")]
	pub neq_seq: Option<Vec<Max35Text>>,
}

impl SequenceRange1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fr_seq_value) = self.fr_seq { if let Err(e) = fr_seq_value.validate() { return Err(e); } }
		if let Some(ref to_seq_value) = self.to_seq { if let Err(e) = to_seq_value.validate() { return Err(e); } }
		if let Some(ref fr_to_seq_vec) = self.fr_to_seq { for item in fr_to_seq_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref eq_seq_vec) = self.eq_seq { for item in eq_seq_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref neq_seq_vec) = self.neq_seq { for item in neq_seq_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// TaxInformation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxInformation8 {
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<TaxParty2>,
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

impl TaxInformation8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
		if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
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


// TechnicalInputChannel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalInputChannel1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalTechnicalInputChannel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl TechnicalInputChannel1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TotalTransactions6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalTransactions6 {
	#[serde(rename = "TtlNtries", skip_serializing_if = "Option::is_none")]
	pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
	#[serde(rename = "TtlCdtNtries", skip_serializing_if = "Option::is_none")]
	pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "TtlDbtNtries", skip_serializing_if = "Option::is_none")]
	pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "TtlNtriesPerBkTxCd", skip_serializing_if = "Option::is_none")]
	pub ttl_ntries_per_bk_tx_cd: Option<Vec<TotalsPerBankTransactionCode5>>,
}

impl TotalTransactions6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ttl_ntries_value) = self.ttl_ntries { if let Err(e) = ttl_ntries_value.validate() { return Err(e); } }
		if let Some(ref ttl_cdt_ntries_value) = self.ttl_cdt_ntries { if let Err(e) = ttl_cdt_ntries_value.validate() { return Err(e); } }
		if let Some(ref ttl_dbt_ntries_value) = self.ttl_dbt_ntries { if let Err(e) = ttl_dbt_ntries_value.validate() { return Err(e); } }
		if let Some(ref ttl_ntries_per_bk_tx_cd_vec) = self.ttl_ntries_per_bk_tx_cd { for item in ttl_ntries_per_bk_tx_cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TotalsPerBankTransactionCode5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalsPerBankTransactionCode5 {
	#[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
	pub nb_of_ntries: Option<Max15NumericText>,
	#[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none")]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
	#[serde(rename = "CdtNtries", skip_serializing_if = "Option::is_none")]
	pub cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "DbtNtries", skip_serializing_if = "Option::is_none")]
	pub dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "FcstInd", skip_serializing_if = "Option::is_none")]
	pub fcst_ind: Option<bool>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DateAndDateTime2Choice>,
}

impl TotalsPerBankTransactionCode5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nb_of_ntries_value) = self.nb_of_ntries { if let Err(e) = nb_of_ntries_value.validate() { return Err(e); } }
		if let Some(ref ttl_net_ntry_value) = self.ttl_net_ntry { if let Err(e) = ttl_net_ntry_value.validate() { return Err(e); } }
		if let Some(ref cdt_ntries_value) = self.cdt_ntries { if let Err(e) = cdt_ntries_value.validate() { return Err(e); } }
		if let Some(ref dbt_ntries_value) = self.dbt_ntries { if let Err(e) = dbt_ntries_value.validate() { return Err(e); } }
		if let Err(e) = self.bk_tx_cd.validate() { return Err(e); }
		if let Some(ref avlbty_vec) = self.avlbty { for item in avlbty_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref dt_value) = self.dt { if let Err(e) = dt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TrackData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrackData1 {
	#[serde(rename = "TrckNb", skip_serializing_if = "Option::is_none")]
	pub trck_nb: Option<Exact1NumericText>,
	#[serde(rename = "TrckVal")]
	pub trck_val: Max140Text,
}

impl TrackData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref trck_nb_value) = self.trck_nb { if let Err(e) = trck_nb_value.validate() { return Err(e); } }
		if let Err(e) = self.trck_val.validate() { return Err(e); }
		Ok(())
	}
}


// TransactionAgents5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAgents5 {
	#[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none")]
	pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none")]
	pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none")]
	pub issg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none")]
	pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryAgent4>>,
}

impl TransactionAgents5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref instg_agt_value) = self.instg_agt { if let Err(e) = instg_agt_value.validate() { return Err(e); } }
		if let Some(ref instd_agt_value) = self.instd_agt { if let Err(e) = instd_agt_value.validate() { return Err(e); } }
		if let Some(ref dbtr_agt_value) = self.dbtr_agt { if let Err(e) = dbtr_agt_value.validate() { return Err(e); } }
		if let Some(ref cdtr_agt_value) = self.cdtr_agt { if let Err(e) = cdtr_agt_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt1_value) = self.intrmy_agt1 { if let Err(e) = intrmy_agt1_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt2_value) = self.intrmy_agt2 { if let Err(e) = intrmy_agt2_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt3_value) = self.intrmy_agt3 { if let Err(e) = intrmy_agt3_value.validate() { return Err(e); } }
		if let Some(ref rcvg_agt_value) = self.rcvg_agt { if let Err(e) = rcvg_agt_value.validate() { return Err(e); } }
		if let Some(ref dlvrg_agt_value) = self.dlvrg_agt { if let Err(e) = dlvrg_agt_value.validate() { return Err(e); } }
		if let Some(ref issg_agt_value) = self.issg_agt { if let Err(e) = issg_agt_value.validate() { return Err(e); } }
		if let Some(ref sttlm_plc_value) = self.sttlm_plc { if let Err(e) = sttlm_plc_value.validate() { return Err(e); } }
		if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionChannel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionChannel1Code {
	#[default]
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "TLPH")]
	CodeTLPH,
	#[serde(rename = "ECOM")]
	CodeECOM,
	#[serde(rename = "TVPY")]
	CodeTVPY,
}

impl TransactionChannel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionDates3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionDates3 {
	#[serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none")]
	pub accptnc_dt_tm: Option<String>,
	#[serde(rename = "TradActvtyCtrctlSttlmDt", skip_serializing_if = "Option::is_none")]
	pub trad_actvty_ctrctl_sttlm_dt: Option<String>,
	#[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
	pub trad_dt: Option<String>,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
	#[serde(rename = "TxDtTm", skip_serializing_if = "Option::is_none")]
	pub tx_dt_tm: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryDate3>>,
}

impl TransactionDates3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionEnvironment1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionEnvironment1Code {
	#[default]
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "PRIV")]
	CodePRIV,
	#[serde(rename = "PUBL")]
	CodePUBL,
}

impl TransactionEnvironment1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionIdentifier1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionIdentifier1 {
	#[serde(rename = "TxDtTm")]
	pub tx_dt_tm: String,
	#[serde(rename = "TxRef")]
	pub tx_ref: Max35Text,
}

impl TransactionIdentifier1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tx_ref.validate() { return Err(e); }
		Ok(())
	}
}


// TransactionInterest4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionInterest4 {
	#[serde(rename = "TtlIntrstAndTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<InterestRecord2>>,
}

impl TransactionInterest4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ttl_intrst_and_tax_amt_value) = self.ttl_intrst_and_tax_amt { if let Err(e) = ttl_intrst_and_tax_amt_value.validate() { return Err(e); } }
		if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionParties6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionParties6 {
	#[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
	pub initg_pty: Option<Party40Choice>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<Party40Choice>,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount38>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<Party40Choice>,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount38>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[serde(rename = "TradgPty", skip_serializing_if = "Option::is_none")]
	pub tradg_pty: Option<Party40Choice>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryParty5>>,
}

impl TransactionParties6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref initg_pty_value) = self.initg_pty { if let Err(e) = initg_pty_value.validate() { return Err(e); } }
		if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
		if let Some(ref dbtr_acct_value) = self.dbtr_acct { if let Err(e) = dbtr_acct_value.validate() { return Err(e); } }
		if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
		if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
		if let Some(ref cdtr_acct_value) = self.cdtr_acct { if let Err(e) = cdtr_acct_value.validate() { return Err(e); } }
		if let Some(ref ultmt_cdtr_value) = self.ultmt_cdtr { if let Err(e) = ultmt_cdtr_value.validate() { return Err(e); } }
		if let Some(ref tradg_pty_value) = self.tradg_pty { if let Err(e) = tradg_pty_value.validate() { return Err(e); } }
		if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionPrice4Choice {
	#[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
	pub deal_pric: Option<Price7>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryPrice2>>,
}

impl TransactionPrice4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref deal_pric_value) = self.deal_pric { if let Err(e) = deal_pric_value.validate() { return Err(e); } }
		if let Some(ref prtry_vec) = self.prtry { for item in prtry_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionQuantities3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionQuantities3Choice {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities1>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<ProprietaryQuantity1>,
}

impl TransactionQuantities3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
		if let Some(ref orgnl_and_cur_face_amt_value) = self.orgnl_and_cur_face_amt { if let Err(e) = orgnl_and_cur_face_amt_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// UnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure1Code {
	#[default]
	#[serde(rename = "PIEC")]
	CodePIEC,
	#[serde(rename = "TONS")]
	CodeTONS,
	#[serde(rename = "FOOT")]
	CodeFOOT,
	#[serde(rename = "GBGA")]
	CodeGBGA,
	#[serde(rename = "USGA")]
	CodeUSGA,
	#[serde(rename = "GRAM")]
	CodeGRAM,
	#[serde(rename = "INCH")]
	CodeINCH,
	#[serde(rename = "KILO")]
	CodeKILO,
	#[serde(rename = "PUND")]
	CodePUND,
	#[serde(rename = "METR")]
	CodeMETR,
	#[serde(rename = "CMET")]
	CodeCMET,
	#[serde(rename = "MMET")]
	CodeMMET,
	#[serde(rename = "LITR")]
	CodeLITR,
	#[serde(rename = "CELI")]
	CodeCELI,
	#[serde(rename = "MILI")]
	CodeMILI,
	#[serde(rename = "GBOU")]
	CodeGBOU,
	#[serde(rename = "USOU")]
	CodeUSOU,
	#[serde(rename = "GBQA")]
	CodeGBQA,
	#[serde(rename = "USQA")]
	CodeUSQA,
	#[serde(rename = "GBPI")]
	CodeGBPI,
	#[serde(rename = "USPI")]
	CodeUSPI,
	#[serde(rename = "MILE")]
	CodeMILE,
	#[serde(rename = "KMET")]
	CodeKMET,
	#[serde(rename = "YARD")]
	CodeYARD,
	#[serde(rename = "SQKI")]
	CodeSQKI,
	#[serde(rename = "HECT")]
	CodeHECT,
	#[serde(rename = "ARES")]
	CodeARES,
	#[serde(rename = "SMET")]
	CodeSMET,
	#[serde(rename = "SCMT")]
	CodeSCMT,
	#[serde(rename = "SMIL")]
	CodeSMIL,
	#[serde(rename = "SQMI")]
	CodeSQMI,
	#[serde(rename = "SQYA")]
	CodeSQYA,
	#[serde(rename = "SQFO")]
	CodeSQFO,
	#[serde(rename = "SQIN")]
	CodeSQIN,
	#[serde(rename = "ACRE")]
	CodeACRE,
}

impl UnitOfMeasure1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UserInterface2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UserInterface2Code {
	#[default]
	#[serde(rename = "MDSP")]
	CodeMDSP,
	#[serde(rename = "CDSP")]
	CodeCDSP,
}

impl UserInterface2Code {
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


// YieldedOrValueType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YieldedOrValueType1Choice {
	#[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
	pub yldd: Option<bool>,
	#[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
	pub val_tp: Option<PriceValueType1Code>,
}

impl YieldedOrValueType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val_tp_value) = self.val_tp { if let Err(e) = val_tp_value.validate() { return Err(e); } }
		Ok(())
	}
}

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


// CurrencyExchange20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange20 {
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "QtnDt")]
	pub qtn_dt: String,
	#[serde(rename = "LwLmt", skip_serializing_if = "Option::is_none")]
	pub lw_lmt: Option<ExchangeRateOrPercentage1Choice>,
	#[serde(rename = "HghLmt", skip_serializing_if = "Option::is_none")]
	pub hgh_lmt: Option<ExchangeRateOrPercentage1Choice>,
}

impl CurrencyExchange20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.qtd_ccy.validate() { return Err(e); }
		if let Some(ref lw_lmt_value) = self.lw_lmt { if let Err(e) = lw_lmt_value.validate() { return Err(e); } }
		if let Some(ref hgh_lmt_value) = self.hgh_lmt { if let Err(e) = hgh_lmt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CurrencyExchangeReport4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchangeReport4 {
	#[serde(rename = "CcyRef")]
	pub ccy_ref: CurrencySourceTarget1,
	#[serde(rename = "CcyXchgOrErr")]
	pub ccy_xchg_or_err: ExchangeRateReportOrError4Choice,
}

impl CurrencyExchangeReport4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ccy_ref.validate() { return Err(e); }
		if let Err(e) = self.ccy_xchg_or_err.validate() { return Err(e); }
		Ok(())
	}
}


// CurrencySourceTarget1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencySourceTarget1 {
	#[serde(rename = "SrcCcy")]
	pub src_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: ActiveOrHistoricCurrencyCode,
}

impl CurrencySourceTarget1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.src_ccy.validate() { return Err(e); }
		if let Err(e) = self.trgt_ccy.validate() { return Err(e); }
		Ok(())
	}
}


// ErrorHandling1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max4AlphaNumericText>,
}

impl ErrorHandling1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ErrorHandling1Code {
	#[default]
	#[serde(rename = "X020")]
	CodeX020,
	#[serde(rename = "X030")]
	CodeX030,
	#[serde(rename = "X050")]
	CodeX050,
}

impl ErrorHandling1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ErrorHandling3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling1Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl ErrorHandling3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.err.validate() { return Err(e); }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExchangeRateOrPercentage1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateOrPercentage1Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
}

impl ExchangeRateOrPercentage1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExchangeRateReportOrError3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateReportOrError3Choice {
	#[serde(rename = "CcyXchgRpt", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg_rpt: Option<Vec<CurrencyExchangeReport4>>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling3>>,
}

impl ExchangeRateReportOrError3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ccy_xchg_rpt_vec) = self.ccy_xchg_rpt { for item in ccy_xchg_rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref oprl_err_vec) = self.oprl_err { for item in oprl_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ExchangeRateReportOrError4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateReportOrError4Choice {
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling3>>,
	#[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg: Option<CurrencyExchange20>,
}

impl ExchangeRateReportOrError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref biz_err_vec) = self.biz_err { for item in biz_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref ccy_xchg_value) = self.ccy_xchg { if let Err(e) = ccy_xchg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "$value")]
	pub external_enquiry_request_type1_code: String,
}

impl ExternalEnquiryRequestType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_enquiry_request_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_enquiry_request_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_enquiry_request_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_enquiry_request_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "$value")]
	pub external_payment_control_request_type1_code: String,
}

impl ExternalPaymentControlRequestType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_payment_control_request_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_payment_control_request_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_payment_control_request_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_payment_control_request_type1_code exceeds the maximum length of 4".to_string()));
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


// MessageHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<Max35Text>,
}

impl MessageHeader7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref req_tp_value) = self.req_tp { if let Err(e) = req_tp_value.validate() { return Err(e); } }
		if let Some(ref orgnl_biz_qry_value) = self.orgnl_biz_qry { if let Err(e) = orgnl_biz_qry_value.validate() { return Err(e); } }
		if let Some(ref qry_nm_value) = self.qry_nm { if let Err(e) = qry_nm_value.validate() { return Err(e); } }
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


// RequestType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
	pub pmt_ctrl: Option<ExternalPaymentControlRequestType1Code>,
	#[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
	pub enqry: Option<ExternalEnquiryRequestType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pmt_ctrl_value) = self.pmt_ctrl { if let Err(e) = pmt_ctrl_value.validate() { return Err(e); } }
		if let Some(ref enqry_value) = self.enqry { if let Err(e) = enqry_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReturnCurrencyExchangeRateV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnCurrencyExchangeRateV05 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader7,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: ExchangeRateReportOrError3Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReturnCurrencyExchangeRateV05 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_hdr.validate() { return Err(e); }
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

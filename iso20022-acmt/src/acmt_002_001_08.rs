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


// Account23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account23 {
	#[serde(rename = "AcctId")]
	pub acct_id: String,
	#[serde(rename = "RltdAcctDtls")]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}


// Account32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account32 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: PartyIdentification125Choice,
}


// AccountDesignation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDesignation1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountDetailsConfirmationV08 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDetailsConfirmationV08 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "OrdrRef")]
	pub ordr_ref: Option<InvestmentFundOrder4>,
	#[serde(rename = "RltdRef")]
	pub rltd_ref: Option<AdditionalReference13>,
	#[serde(rename = "ConfDtls")]
	pub conf_dtls: AccountManagementConfirmation5,
	#[serde(rename = "InvstmtAcct")]
	pub invstmt_acct: Option<InvestmentAccount74>,
	#[serde(rename = "AcctPties")]
	pub acct_pties: Option<AccountParties17>,
	#[serde(rename = "Intrmies")]
	pub intrmies: Option<Vec<Intermediary46>>,
	#[serde(rename = "Plcmnt")]
	pub plcmnt: Option<ReferredAgent3>,
	#[serde(rename = "NewIsseAllcn")]
	pub new_isse_allcn: Option<NewIssueAllocation2>,
	#[serde(rename = "SvgsInvstmtPlan")]
	pub svgs_invstmt_plan: Option<Vec<InvestmentPlan17>>,
	#[serde(rename = "WdrwlInvstmtPlan")]
	pub wdrwl_invstmt_plan: Option<Vec<InvestmentPlan17>>,
	#[serde(rename = "CshSttlm")]
	pub csh_sttlm: Option<Vec<CashSettlement3>>,
	#[serde(rename = "SvcLvlAgrmt")]
	pub svc_lvl_agrmt: Option<Vec<DocumentToSend4>>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[serde(rename = "MktPrctcVrsn")]
	pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
	#[serde(rename = "Xtnsn")]
	pub xtnsn: Option<Vec<Extension1>>,
}


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountIdentificationAndName5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentificationAndName5 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// AccountManagementConfirmation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementConfirmation5 {
	#[serde(rename = "ConfTp")]
	pub conf_tp: ConfirmationType1Choice,
	#[serde(rename = "AcctApplId")]
	pub acct_appl_id: Option<String>,
	#[serde(rename = "ClntRef")]
	pub clnt_ref: Option<String>,
	#[serde(rename = "CtrPtyRef")]
	pub ctr_pty_ref: Option<AdditionalReference13>,
	#[serde(rename = "ExstgAcctId")]
	pub exstg_acct_id: Option<Vec<Account23>>,
}


// AccountManagementType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementType2Code {
	#[serde(rename = "AccountManagementType2Code")]
	pub account_management_type2_code: String,
}


// AccountOwnershipType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountOwnershipType4Code {
	#[serde(rename = "AccountOwnershipType4Code")]
	pub account_ownership_type4_code: String,
}


// AccountParties12Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountParties12Choice {
	#[serde(rename = "PmryOwnr")]
	pub pmry_ownr: Option<InvestmentAccountOwnershipInformation16>,
	#[serde(rename = "Trstee")]
	pub trstee: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Nmnee")]
	pub nmnee: Option<InvestmentAccountOwnershipInformation16>,
	#[serde(rename = "JntOwnr")]
	pub jnt_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
}


// AccountParties17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountParties17 {
	#[serde(rename = "PrncplAcctPty")]
	pub prncpl_acct_pty: AccountParties12Choice,
	#[serde(rename = "ScndryOwnr")]
	pub scndry_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "PwrOfAttny")]
	pub pwr_of_attny: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "LglGuardn")]
	pub lgl_guardn: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "CtdnForMnr")]
	pub ctdn_for_mnr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "SucssrOnDth")]
	pub sucssr_on_dth: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Admstr")]
	pub admstr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "OthrPty")]
	pub othr_pty: Option<Vec<ExtendedParty14>>,
	#[serde(rename = "Grntr")]
	pub grntr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Sttlr")]
	pub sttlr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "SnrMggOffcl")]
	pub snr_mgg_offcl: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Prtctr")]
	pub prtctr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "RegdShrhldrNm")]
	pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AccountStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountStatus2 {
	#[serde(rename = "Nbld")]
	pub nbld: Option<EnabledStatusReason1Choice>,
	#[serde(rename = "Dsbld")]
	pub dsbld: Option<DisabledStatusReason1Choice>,
	#[serde(rename = "Pdg")]
	pub pdg: Option<PendingStatusReason1Choice>,
	#[serde(rename = "PdgOpng")]
	pub pdg_opng: Option<PendingOpeningStatusReason1Choice>,
	#[serde(rename = "Profrm")]
	pub profrm: Option<ProformaStatusReason1Choice>,
	#[serde(rename = "Clsd")]
	pub clsd: Option<ClosedStatusReason1Choice>,
	#[serde(rename = "ClsrPdg")]
	pub clsr_pdg: Option<ClosurePendingStatusReason1Choice>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherAccountStatus1>>,
}


// AccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountUsageType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountUsageType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountUsageType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountUsageType2Code {
	#[serde(rename = "AccountUsageType2Code")]
	pub account_usage_type2_code: String,
}


// AccountingStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountingStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountingStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountingStatus1Code {
	#[serde(rename = "AccountingStatus1Code")]
	pub accounting_status1_code: String,
}


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


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


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AdditionalReference13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference13 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[serde(rename = "RefIssr")]
	pub ref_issr: Option<PartyIdentification125Choice>,
	#[serde(rename = "MsgNm")]
	pub msg_nm: Option<String>,
}


// AdditiononalInformation13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditiononalInformation13 {
	#[serde(rename = "Lmttn")]
	pub lmttn: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
	#[serde(rename = "AcctVldtn")]
	pub acct_vldtn: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Rgltr")]
	pub rgltr: Option<PartyIdentification125Choice>,
	#[serde(rename = "Sts")]
	pub sts: Option<RestrictionStatus1Choice>,
	#[serde(rename = "Prd")]
	pub prd: Option<DateTimePeriod2>,
}


// AddressType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AddressType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType1Code {
	#[serde(rename = "AddressType1Code")]
	pub address_type1_code: String,
}


// AddressType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AlternateSecurityIdentification7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification7 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IdSrc")]
	pub id_src: IdentificationSource1Choice,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AustrianBankleitzahlIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AustrianBankleitzahlIdentifier {
	#[serde(rename = "AustrianBankleitzahlIdentifier")]
	pub austrian_bankleitzahl_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BelgianIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BelgianIdentifier {
	#[serde(rename = "BelgianIdentifier")]
	pub belgian_identifier: String,
}


// BlockedHoldingDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedHoldingDetails2 {
	#[serde(rename = "BlckdHldg")]
	pub blckd_hldg: String,
	#[serde(rename = "PrtlHldgUnits")]
	pub prtl_hldg_units: Option<f64>,
	#[serde(rename = "HldgCertNb")]
	pub hldg_cert_nb: Option<String>,
}


// BlockedReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// BlockedReason2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedReason2Code {
	#[serde(rename = "BlockedReason2Code")]
	pub blocked_reason2_code: String,
}


// BlockedStatusReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedStatusReason2 {
	#[serde(rename = "TxTp")]
	pub tx_tp: TransactionType5Choice,
	#[serde(rename = "Blckd")]
	pub blckd: bool,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<BlockedReason2Choice>>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: String,
}


// BlockedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedStatusReason2Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<BlockedStatusReason2>>,
}


// Bloomberg2Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Bloomberg2Identifier {
	#[serde(rename = "Bloomberg2Identifier")]
	pub bloomberg2_identifier: String,
}


// BranchData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData4 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress1>,
}


// CHIPSParticipantIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CHIPSParticipantIdentifier {
	#[serde(rename = "CHIPSParticipantIdentifier")]
	pub chips_participant_identifier: String,
}


// CHIPSUniversalIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CHIPSUniversalIdentifier {
	#[serde(rename = "CHIPSUniversalIdentifier")]
	pub chips_universal_identifier: String,
}


// CRSForm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSForm1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSFormType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSFormType1Code {
	#[serde(rename = "CRSFormType1Code")]
	pub crs_form_type1_code: String,
}


// CRSSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSSource1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSSourceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSSourceStatus1Code {
	#[serde(rename = "CRSSourceStatus1Code")]
	pub crs_source_status1_code: String,
}


// CRSStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSStatus1Code {
	#[serde(rename = "CRSStatus1Code")]
	pub crs_status1_code: String,
}


// CRSStatus3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSStatus3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSStatus4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSStatus4 {
	#[serde(rename = "Tp")]
	pub tp: CRSStatus3Choice,
	#[serde(rename = "Src")]
	pub src: Option<CRSSource1Choice>,
	#[serde(rename = "XcptnlRptgCtry")]
	pub xcptnl_rptg_ctry: Option<String>,
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CUSIPIdentifier {
	#[serde(rename = "CUSIPIdentifier")]
	pub cusip_identifier: String,
}


// CanadianPaymentsARNIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CanadianPaymentsARNIdentifier {
	#[serde(rename = "CanadianPaymentsARNIdentifier")]
	pub canadian_payments_arn_identifier: String,
}


// CardType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardType1Code {
	#[serde(rename = "CardType1Code")]
	pub card_type1_code: String,
}


// CashAccount204 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount204 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: String,
	#[serde(rename = "Id")]
	pub id: AccountIdentificationAndName5,
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Option<PartyIdentification125Choice>,
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<FinancialInstitutionIdentification11Choice>,
	#[serde(rename = "AcctSvcrBrnch")]
	pub acct_svcr_brnch: Option<BranchData4>,
	#[serde(rename = "AcctOwnrOthrId")]
	pub acct_ownr_othr_id: Option<Vec<GenericIdentification82>>,
	#[serde(rename = "InvstmtAcctTp")]
	pub invstmt_acct_tp: Option<AccountType2Choice>,
	#[serde(rename = "CdtDbt")]
	pub cdt_dbt: Option<String>,
	#[serde(rename = "SttlmInstrRsn")]
	pub sttlm_instr_rsn: Option<SettlementInstructionReason1Choice>,
	#[serde(rename = "CshAcctPurp")]
	pub csh_acct_purp: Option<CashAccountType3Choice>,
	#[serde(rename = "CshAcctDsgnt")]
	pub csh_acct_dsgnt: Option<AccountDesignation1Choice>,
	#[serde(rename = "DvddPctg")]
	pub dvdd_pctg: Option<f64>,
}


// CashAccountType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CashAccountType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType5Code {
	#[serde(rename = "CashAccountType5Code")]
	pub cash_account_type5_code: String,
}


// CashSettlement3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashSettlement3 {
	#[serde(rename = "CshAcctDtls")]
	pub csh_acct_dtls: Option<Vec<CashAccount204>>,
	#[serde(rename = "OthrCshSttlmDtls")]
	pub othr_csh_sttlm_dtls: Option<Vec<PaymentInstrument17>>,
}


// CertificateType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CertificateType2Code {
	#[serde(rename = "CertificateType2Code")]
	pub certificate_type2_code: String,
}


// CertificationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CertificationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Cheque4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cheque4 {
	#[serde(rename = "PyeeId")]
	pub pyee_id: NameAndAddress5,
}


// CitizenshipInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CitizenshipInformation2 {
	#[serde(rename = "Ntlty")]
	pub ntlty: String,
	#[serde(rename = "MnrInd")]
	pub mnr_ind: bool,
}


// CivilStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CivilStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CivilStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CivilStatus1Code {
	#[serde(rename = "CivilStatus1Code")]
	pub civil_status1_code: String,
}


// ClearingSystemMemberIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification4Choice {
	#[serde(rename = "USCHU")]
	pub uschu: Option<String>,
	#[serde(rename = "NZNCC")]
	pub nzncc: Option<String>,
	#[serde(rename = "IENSC")]
	pub iensc: Option<String>,
	#[serde(rename = "GBSC")]
	pub gbsc: Option<String>,
	#[serde(rename = "USCH")]
	pub usch: Option<String>,
	#[serde(rename = "CHBC")]
	pub chbc: Option<String>,
	#[serde(rename = "USFW")]
	pub usfw: Option<String>,
	#[serde(rename = "PTNCC")]
	pub ptncc: Option<String>,
	#[serde(rename = "RUCB")]
	pub rucb: Option<String>,
	#[serde(rename = "ITNCC")]
	pub itncc: Option<String>,
	#[serde(rename = "ATBLZ")]
	pub atblz: Option<String>,
	#[serde(rename = "CACPA")]
	pub cacpa: Option<String>,
	#[serde(rename = "CHSIC")]
	pub chsic: Option<String>,
	#[serde(rename = "DEBLZ")]
	pub deblz: Option<String>,
	#[serde(rename = "ESNCC")]
	pub esncc: Option<String>,
	#[serde(rename = "ZANCC")]
	pub zancc: Option<String>,
	#[serde(rename = "HKNCC")]
	pub hkncc: Option<String>,
	#[serde(rename = "AUBSBx")]
	pub aubs_bx: Option<String>,
	#[serde(rename = "AUBSBs")]
	pub aubs_bs: Option<String>,
}


// ClosedStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosedStatusReason2Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// ClosedStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<ClosedStatusReason1>>,
}


// ClosedStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1Code {
	#[serde(rename = "ClosedStatusReason1Code")]
	pub closed_status_reason1_code: String,
}


// ClosedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// ClosurePendingStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosurePendingStatusReason2Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// ClosurePendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<ClosurePendingStatusReason1>>,
}


// ClosurePendingStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1Code {
	#[serde(rename = "ClosurePendingStatusReason1Code")]
	pub closure_pending_status_reason1_code: String,
}


// ClosurePendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// Collateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Collateral1Code {
	#[serde(rename = "Collateral1Code")]
	pub collateral1_code: String,
}


// CommunicationAddress6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationAddress6 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType1Choice>,
	#[serde(rename = "Email")]
	pub email: Option<String>,
	#[serde(rename = "Phne")]
	pub phne: Option<String>,
	#[serde(rename = "Mob")]
	pub mob: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "TlxAdr")]
	pub tlx_adr: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
}


// CommunicationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod1Code {
	#[serde(rename = "CommunicationMethod1Code")]
	pub communication_method1_code: String,
}


// CommunicationMethod3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CompanyLink1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompanyLink1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CompanyLink1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompanyLink1Code {
	#[serde(rename = "CompanyLink1Code")]
	pub company_link1_code: String,
}


// ConductClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConductClassification1Code {
	#[serde(rename = "ConductClassification1Code")]
	pub conduct_classification1_code: String,
}


// ConfirmationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConfirmationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[serde(rename = "ConsolidatedTapeAssociationIdentifier")]
	pub consolidated_tape_association_identifier: String,
}


// ConsolidationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConsolidationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ConsolidationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConsolidationType1Code {
	#[serde(rename = "ConsolidationType1Code")]
	pub consolidation_type1_code: String,
}


// CountryAndResidentialStatusType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryAndResidentialStatusType2 {
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[serde(rename = "ResdtlSts")]
	pub resdtl_sts: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebit3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDebit3Code {
	#[serde(rename = "CreditDebit3Code")]
	pub credit_debit3_code: String,
}


// CustomerConductClassification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomerConductClassification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// DataBaseCheck1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DataBaseCheck1 {
	#[serde(rename = "DBChck")]
	pub db_chck: bool,
	#[serde(rename = "Id")]
	pub id: String,
}


// DateAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndAmount1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// DateAndDateTime1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime1Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod2 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
}


// DeMinimus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimus1Choice {
	#[serde(rename = "DeMnmsAplbl")]
	pub de_mnms_aplbl: Option<DeMinimusApplicable1>,
	#[serde(rename = "DeMnmsNotAplbl")]
	pub de_mnms_not_aplbl: Option<DeMinimusNotApplicable1>,
}


// DeMinimusApplicable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimusApplicable1 {
	#[serde(rename = "NewIssePrmssn")]
	pub new_isse_prmssn: bool,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
}


// DeMinimusNotApplicable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimusNotApplicable1 {
	#[serde(rename = "RstrctdPrsnRsn")]
	pub rstrctd_prsn_rsn: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DirectDebitMandate7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DirectDebitMandate7 {
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: AccountIdentificationAndName5,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<PartyIdentification125Choice>,
	#[serde(rename = "DbtrTaxIdNb")]
	pub dbtr_tax_id_nb: Option<String>,
	#[serde(rename = "DbtrNtlRegnNb")]
	pub dbtr_ntl_regn_nb: Option<String>,
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<PartyIdentification125Choice>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: FinancialInstitutionIdentification11Choice,
	#[serde(rename = "DbtrAgtBrnch")]
	pub dbtr_agt_brnch: Option<BranchData4>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<FinancialInstitutionIdentification11Choice>,
	#[serde(rename = "CdtrAgtBrnch")]
	pub cdtr_agt_brnch: Option<BranchData4>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "MndtId")]
	pub mndt_id: Option<String>,
}


// DisabledReason2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledReason2Code {
	#[serde(rename = "DisabledReason2Code")]
	pub disabled_reason2_code: String,
}


// DisabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: DisabledStatusReason2Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// DisabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<DisabledStatusReason1>>,
}


// DisabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistributionPolicy1Code {
	#[serde(rename = "DistributionPolicy1Code")]
	pub distribution_policy1_code: String,
}


// DocumentToSend4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentToSend4 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Rcpt")]
	pub rcpt: PartyIdentification125Choice,
	#[serde(rename = "MtdOfTrnsmssn")]
	pub mtd_of_trnsmssn: CommunicationMethod3Choice,
}


// DutchIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DutchIdentifier {
	#[serde(rename = "DutchIdentifier")]
	pub dutch_identifier: String,
}


// Eligible1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Eligible1Code {
	#[serde(rename = "Eligible1Code")]
	pub eligible1_code: String,
}


// EnabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: EnabledStatusReason2Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// EnabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<EnabledStatusReason1>>,
}


// EnabledStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1Code {
	#[serde(rename = "EnabledStatusReason1Code")]
	pub enabled_status_reason1_code: String,
}


// EnabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "EuroclearClearstreamIdentifier")]
	pub euroclear_clearstream_identifier: String,
}


// EventFrequency10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventFrequency10Code {
	#[serde(rename = "EventFrequency10Code")]
	pub event_frequency10_code: String,
}


// EventFrequency1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventFrequency1Code {
	#[serde(rename = "EventFrequency1Code")]
	pub event_frequency1_code: String,
}


// EventFrequency8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventFrequency8Code {
	#[serde(rename = "EventFrequency8Code")]
	pub event_frequency8_code: String,
}


// EventFrequency9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventFrequency9Code {
	#[serde(rename = "EventFrequency9Code")]
	pub event_frequency9_code: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// Extended350Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extended350Code {
	#[serde(rename = "Extended350Code")]
	pub extended350_code: String,
}


// ExtendedParty14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtendedParty14 {
	#[serde(rename = "XtndedPtyRole")]
	pub xtnded_pty_role: String,
	#[serde(rename = "OthrPtyDtls")]
	pub othr_pty_dtls: InvestmentAccountOwnershipInformation16,
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: String,
	#[serde(rename = "Txt")]
	pub txt: String,
}


// ExtensiveBranchNetworkIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtensiveBranchNetworkIdentifier {
	#[serde(rename = "ExtensiveBranchNetworkIdentifier")]
	pub extensive_branch_network_identifier: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// FATCAForm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAForm1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FATCAFormType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAFormType1Code {
	#[serde(rename = "FATCAFormType1Code")]
	pub fatca_form_type1_code: String,
}


// FATCASource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCASource1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FATCASourceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCASourceStatus1Code {
	#[serde(rename = "FATCASourceStatus1Code")]
	pub fatca_source_status1_code: String,
}


// FATCAStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAStatus1Code {
	#[serde(rename = "FATCAStatus1Code")]
	pub fatca_status1_code: String,
}


// FATCAStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAStatus2 {
	#[serde(rename = "Tp")]
	pub tp: FATCAStatus2Choice,
	#[serde(rename = "Src")]
	pub src: Option<FATCASource1Choice>,
}


// FATCAStatus2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAStatus2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FedwireRoutingNumberIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedwireRoutingNumberIdentifier {
	#[serde(rename = "FedwireRoutingNumberIdentifier")]
	pub fedwire_routing_number_identifier: String,
}


// FinancialInstitutionIdentification11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification11Choice {
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<SimpleIdentificationInformation4>,
}


// FinancialInstrument87 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument87 {
	#[serde(rename = "Id")]
	pub id: SecurityIdentification25Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "SplmtryId")]
	pub splmtry_id: Option<String>,
	#[serde(rename = "ClssTp")]
	pub clss_tp: Option<String>,
	#[serde(rename = "SctiesForm")]
	pub scties_form: Option<String>,
	#[serde(rename = "DstrbtnPlcy")]
	pub dstrbtn_plcy: Option<String>,
	#[serde(rename = "PdctGrp")]
	pub pdct_grp: Option<String>,
	#[serde(rename = "BlckdHldgDtls")]
	pub blckd_hldg_dtls: Option<BlockedHoldingDetails2>,
	#[serde(rename = "Pldgg")]
	pub pldgg: Option<String>,
	#[serde(rename = "Coll")]
	pub coll: Option<String>,
	#[serde(rename = "ThrdPtyRghts")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[serde(rename = "FndOwnrsh")]
	pub fnd_ownrsh: Option<String>,
	#[serde(rename = "FndIntntn")]
	pub fnd_intntn: Option<String>,
	#[serde(rename = "OprlSts")]
	pub oprl_sts: Option<String>,
}


// FiscalYear1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FiscalYear1Choice {
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FormOfSecurity1Code {
	#[serde(rename = "FormOfSecurity1Code")]
	pub form_of_security1_code: String,
}


// Frequency20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency20Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FundCashAccount4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashAccount4Code {
	#[serde(rename = "FundCashAccount4Code")]
	pub fund_cash_account4_code: String,
}


// FundIntention1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundIntention1Code {
	#[serde(rename = "FundIntention1Code")]
	pub fund_intention1_code: String,
}


// FundOwnership1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundOwnership1Code {
	#[serde(rename = "FundOwnership1Code")]
	pub fund_ownership1_code: String,
}


// GDPRData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GDPRData1 {
	#[serde(rename = "CnsntTp")]
	pub cnsnt_tp: GDPRDataConsent1Choice,
	#[serde(rename = "CnsntInd")]
	pub cnsnt_ind: bool,
	#[serde(rename = "CnsntDt")]
	pub cnsnt_dt: String,
}


// GDPRDataConsent1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GDPRDataConsent1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// GDPRDataConsent1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GDPRDataConsent1Code {
	#[serde(rename = "GDPRDataConsent1Code")]
	pub gdpr_data_consent1_code: String,
}


// Gender1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Gender1Code {
	#[serde(rename = "Gender1Code")]
	pub gender1_code: String,
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification82 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification82 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Tp")]
	pub tp: OtherIdentification3Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "IsseDt")]
	pub isse_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "Stat")]
	pub stat: Option<String>,
	#[serde(rename = "IssrCtry")]
	pub issr_ctry: Option<String>,
}


// GermanBankleitzahlIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GermanBankleitzahlIdentifier {
	#[serde(rename = "GermanBankleitzahlIdentifier")]
	pub german_bankleitzahl_identifier: String,
}


// HighFrequencyTradingProfile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HighFrequencyTradingProfile1 {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "SttlmFrqcy")]
	pub sttlm_frqcy: Option<SettlementFrequency1Choice>,
	#[serde(rename = "CnsldtnTp")]
	pub cnsldtn_tp: Option<ConsolidationType1Choice>,
}


// Holding1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Holding1Code {
	#[serde(rename = "Holding1Code")]
	pub holding1_code: String,
}


// HongKongBankIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HongKongBankIdentifier {
	#[serde(rename = "HongKongBankIdentifier")]
	pub hong_kong_bank_identifier: String,
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "ISOYearMonth")]
	pub iso_year_month: String,
}


// IdentificationSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource1Choice {
	#[serde(rename = "Dmst")]
	pub dmst: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// IncomePreference2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IncomePreference2Code {
	#[serde(rename = "IncomePreference2Code")]
	pub income_preference2_code: String,
}


// IndividualPerson29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson29 {
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm")]
	pub gvn_nm: Option<String>,
	#[serde(rename = "MddlNm")]
	pub mddl_nm: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}


// IndividualPerson37 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson37 {
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm")]
	pub gvn_nm: Option<String>,
	#[serde(rename = "MddlNm")]
	pub mddl_nm: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "NmSfx")]
	pub nm_sfx: Option<String>,
	#[serde(rename = "Gndr")]
	pub gndr: Option<String>,
	#[serde(rename = "BirthDt")]
	pub birth_dt: Option<String>,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: Option<String>,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: Option<String>,
	#[serde(rename = "Prfssn")]
	pub prfssn: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
	#[serde(rename = "Ctznsh")]
	pub ctznsh: Option<Vec<CitizenshipInformation2>>,
	#[serde(rename = "EmplngCpny")]
	pub emplng_cpny: Option<String>,
	#[serde(rename = "BizFctn")]
	pub biz_fctn: Option<String>,
	#[serde(rename = "PltclyXpsdPrsn")]
	pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
	#[serde(rename = "DthDt")]
	pub dth_dt: Option<String>,
	#[serde(rename = "CvlSts")]
	pub cvl_sts: Option<CivilStatus1Choice>,
	#[serde(rename = "EdctnLvl")]
	pub edctn_lvl: Option<String>,
	#[serde(rename = "FmlyInf")]
	pub fmly_inf: Option<PersonalInformation1>,
	#[serde(rename = "GDPRData")]
	pub gdpr_data: Option<Vec<GDPRData1>>,
}


// InformationDistribution1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationDistribution1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InformationDistribution2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationDistribution2Code {
	#[serde(rename = "InformationDistribution2Code")]
	pub information_distribution2_code: String,
}


// InitialAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialAmount1Choice {
	#[serde(rename = "InitlNbOfInstlmts")]
	pub initl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
}


// Insurance1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Insurance1Code {
	#[serde(rename = "Insurance1Code")]
	pub insurance1_code: String,
}


// InsuranceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InsuranceType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Intermediary46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Intermediary46 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification177Choice,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[serde(rename = "Acct")]
	pub acct: Option<Account32>,
	#[serde(rename = "WvdTrlrComssnInd")]
	pub wvd_trlr_comssn_ind: Option<bool>,
	#[serde(rename = "Role")]
	pub role: Option<PartyRole2Choice>,
	#[serde(rename = "PmryComAdr")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "ScndryComAdr")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress4>,
}


// InvestmentAccount74 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccount74 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "AcctSts")]
	pub acct_sts: Option<AccountStatus2>,
	#[serde(rename = "BlckdSts")]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[serde(rename = "StsDt")]
	pub sts_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dsgnt")]
	pub dsgnt: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<AccountType2Choice>,
	#[serde(rename = "OwnrshTp")]
	pub ownrsh_tp: Option<OwnershipType2Choice>,
	#[serde(rename = "TaxXmptn")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[serde(rename = "StmtFrqcy")]
	pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
	#[serde(rename = "RefCcy")]
	pub ref_ccy: Option<String>,
	#[serde(rename = "Lang")]
	pub lang: Option<String>,
	#[serde(rename = "IncmPref")]
	pub incm_pref: Option<String>,
	#[serde(rename = "RinvstmtDtls")]
	pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
	#[serde(rename = "TaxWhldgMtd")]
	pub tax_whldg_mtd: Option<String>,
	#[serde(rename = "TaxRptg")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[serde(rename = "LttrInttDtls")]
	pub lttr_intt_dtls: Option<LetterIntent1>,
	#[serde(rename = "AcmltnRghtRef")]
	pub acmltn_rght_ref: Option<String>,
	#[serde(rename = "ReqrdSgntriesNb")]
	pub reqrd_sgntries_nb: Option<f64>,
	#[serde(rename = "FndFmlyNm")]
	pub fnd_fmly_nm: Option<String>,
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: Option<Vec<FinancialInstrument87>>,
	#[serde(rename = "RndgDtls")]
	pub rndg_dtls: Option<RoundingParameters1>,
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<PartyIdentification125Choice>,
	#[serde(rename = "AcctUsgTp")]
	pub acct_usg_tp: Option<AccountUsageType2Choice>,
	#[serde(rename = "FrgnStsCertfctn")]
	pub frgn_sts_certfctn: Option<String>,
	#[serde(rename = "AcctSgntrDtTm")]
	pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
	#[serde(rename = "TxChanlTp")]
	pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
	#[serde(rename = "InvstmtAcctCtgy")]
	pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
	#[serde(rename = "Pldgg")]
	pub pldgg: Option<String>,
	#[serde(rename = "Coll")]
	pub coll: Option<String>,
	#[serde(rename = "ThrdPtyRghts")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[serde(rename = "PwrOfAttnyLvlOfCtrl")]
	pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
	#[serde(rename = "AcctgSts")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[serde(rename = "OpngDt")]
	pub opng_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "ClsgDt")]
	pub clsg_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "NegInd")]
	pub neg_ind: Option<bool>,
	#[serde(rename = "PrcgOrdr")]
	pub prcg_ordr: Option<String>,
	#[serde(rename = "Lblty")]
	pub lblty: Option<Liability1Choice>,
	#[serde(rename = "InvstrPrfl")]
	pub invstr_prfl: Option<Vec<InvestorProfile2>>,
	#[serde(rename = "FsclYr")]
	pub fscl_yr: Option<FiscalYear1Choice>,
}


// InvestmentAccountCategory1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccountCategory1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentAccountCategory1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccountCategory1Code {
	#[serde(rename = "InvestmentAccountCategory1Code")]
	pub investment_account_category1_code: String,
}


// InvestmentAccountOwnershipInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccountOwnershipInformation16 {
	#[serde(rename = "Pty")]
	pub pty: Party47Choice,
	#[serde(rename = "MnyLndrgChck")]
	pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
	#[serde(rename = "InvstrPrflVldtn")]
	pub invstr_prfl_vldtn: Option<Vec<PartyProfileInformation5>>,
	#[serde(rename = "OwnrshBnfcryRate")]
	pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
	#[serde(rename = "ClntId")]
	pub clnt_id: Option<String>,
	#[serde(rename = "FsclXmptn")]
	pub fscl_xmptn: Option<bool>,
	#[serde(rename = "SgntryRghtInd")]
	pub sgntry_rght_ind: Option<bool>,
	#[serde(rename = "MiFIDClssfctn")]
	pub mi_fid_clssfctn: Option<MiFIDClassification1>,
	#[serde(rename = "Ntfctn")]
	pub ntfctn: Option<Vec<Notification2>>,
	#[serde(rename = "FATCAFormTp")]
	pub fatca_form_tp: Option<Vec<FATCAForm1Choice>>,
	#[serde(rename = "FATCASts")]
	pub fatca_sts: Option<Vec<FATCAStatus2>>,
	#[serde(rename = "FATCARptgDt")]
	pub fatca_rptg_dt: Option<String>,
	#[serde(rename = "CRSFormTp")]
	pub crs_form_tp: Option<Vec<CRSForm1Choice>>,
	#[serde(rename = "CRSSts")]
	pub crs_sts: Option<Vec<CRSStatus4>>,
	#[serde(rename = "CRSRptgDt")]
	pub crs_rptg_dt: Option<String>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<GenericIdentification82>>,
	#[serde(rename = "TaxXmptn")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[serde(rename = "TaxRptg")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[serde(rename = "Lang")]
	pub lang: Option<String>,
	#[serde(rename = "MailTp")]
	pub mail_tp: Option<MailType1Choice>,
	#[serde(rename = "CtryAndResdtlSts")]
	pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
	#[serde(rename = "MntryWlth")]
	pub mntry_wlth: Option<DateAndAmount1>,
	#[serde(rename = "EqtyVal")]
	pub eqty_val: Option<DateAndAmount1>,
	#[serde(rename = "WorkgCptl")]
	pub workg_cptl: Option<DateAndAmount1>,
	#[serde(rename = "CpnyLk")]
	pub cpny_lk: Option<CompanyLink1Choice>,
	#[serde(rename = "ElctrncMlngSvcRef")]
	pub elctrnc_mlng_svc_ref: Option<String>,
	#[serde(rename = "PmryComAdr")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "ScndryComAdr")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "AddtlRgltryInf")]
	pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
	#[serde(rename = "AcctgSts")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[serde(rename = "CtrlgPty")]
	pub ctrlg_pty: Option<bool>,
}


// InvestmentFundOrder4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundOrder4 {
	#[serde(rename = "OrdrRef")]
	pub ordr_ref: Option<String>,
	#[serde(rename = "MstrRef")]
	pub mstr_ref: Option<String>,
}


// InvestmentFundRole6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundRole6Code {
	#[serde(rename = "InvestmentFundRole6Code")]
	pub investment_fund_role6_code: String,
}


// InvestmentFundRole7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundRole7Code {
	#[serde(rename = "InvestmentFundRole7Code")]
	pub investment_fund_role7_code: String,
}


// InvestmentFundTransactionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundTransactionType1Code {
	#[serde(rename = "InvestmentFundTransactionType1Code")]
	pub investment_fund_transaction_type1_code: String,
}


// InvestmentPlan17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentPlan17 {
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency20Choice,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmount1Choice,
	#[serde(rename = "GrssAmtInd")]
	pub grss_amt_ind: Option<bool>,
	#[serde(rename = "IncmPref")]
	pub incm_pref: Option<String>,
	#[serde(rename = "InitlAmt")]
	pub initl_amt: Option<InitialAmount1Choice>,
	#[serde(rename = "TtlNbOfInstlmts")]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "RndgDrctn")]
	pub rndg_drctn: Option<String>,
	#[serde(rename = "SctyDtls")]
	pub scty_dtls: Vec<Repartition6>,
	#[serde(rename = "CshSttlm")]
	pub csh_sttlm: Option<Vec<CashSettlement3>>,
	#[serde(rename = "CtrctRef")]
	pub ctrct_ref: Option<String>,
	#[serde(rename = "RltdCtrctRef")]
	pub rltd_ctrct_ref: Option<String>,
	#[serde(rename = "PdctId")]
	pub pdct_id: Option<String>,
	#[serde(rename = "SLAChrgAndComssnRef")]
	pub sla_chrg_and_comssn_ref: Option<String>,
	#[serde(rename = "InsrncCover")]
	pub insrnc_cover: Option<InsuranceType2Choice>,
	#[serde(rename = "PlanSts")]
	pub plan_sts: Option<PlanStatus2Choice>,
	#[serde(rename = "InstlmtMgrRole")]
	pub instlmt_mgr_role: Option<PartyRole4Choice>,
}


// InvestorProfile2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorProfile2 {
	#[serde(rename = "Tp")]
	pub tp: Option<ProfileType1Choice>,
	#[serde(rename = "Sts")]
	pub sts: Option<InvestorProfileStatus1Choice>,
	#[serde(rename = "Trsr")]
	pub trsr: Option<TreasuryProfile1>,
	#[serde(rename = "HghFrqcyTradg")]
	pub hgh_frqcy_tradg: Option<HighFrequencyTradingProfile1>,
	#[serde(rename = "MktMakr")]
	pub mkt_makr: Option<MarketMakerProfile2>,
}


// InvestorProfileStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorProfileStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestorProfileStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorProfileStatus1Code {
	#[serde(rename = "InvestorProfileStatus1Code")]
	pub investor_profile_status1_code: String,
}


// IrishNSCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IrishNSCIdentifier {
	#[serde(rename = "IrishNSCIdentifier")]
	pub irish_nsc_identifier: String,
}


// ItalianDomesticIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ItalianDomesticIdentifier {
	#[serde(rename = "ItalianDomesticIdentifier")]
	pub italian_domestic_identifier: String,
}


// KYCCheckType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct KYCCheckType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// KnowYourCustomerCheckType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct KnowYourCustomerCheckType1Code {
	#[serde(rename = "KnowYourCustomerCheckType1Code")]
	pub know_your_customer_check_type1_code: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LanguageCode {
	#[serde(rename = "LanguageCode")]
	pub language_code: String,
}


// LetterIntent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LetterIntent1 {
	#[serde(rename = "LttrInttRef")]
	pub lttr_intt_ref: String,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
}


// LevelOfControl1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LevelOfControl1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// LevelOfControl1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LevelOfControl1Code {
	#[serde(rename = "LevelOfControl1Code")]
	pub level_of_control1_code: String,
}


// Liability1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Liability1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Liability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Liability1Code {
	#[serde(rename = "Liability1Code")]
	pub liability1_code: String,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// MailType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MailType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// MailType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MailType1Code {
	#[serde(rename = "MailType1Code")]
	pub mail_type1_code: String,
}


// MarketMakerProfile2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketMakerProfile2 {
	#[serde(rename = "CtrctPrd")]
	pub ctrct_prd: Option<DateTimePeriod2>,
	#[serde(rename = "Cmplc")]
	pub cmplc: Option<bool>,
	#[serde(rename = "MaxSprd")]
	pub max_sprd: Option<f64>,
	#[serde(rename = "Dscnt")]
	pub dscnt: Option<f64>,
}


// MarketPracticeVersion1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketPracticeVersion1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
}


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10Text {
	#[serde(rename = "Max10Text")]
	pub max10_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
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


// Max3Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Text {
	#[serde(rename = "Max3Text")]
	pub max3_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// MiFIDClassification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MiFIDClassification1 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: String,
	#[serde(rename = "Nrrtv")]
	pub nrrtv: Option<String>,
}


// MoneyLaunderingCheck1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyLaunderingCheck1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// MoneyLaunderingCheck1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyLaunderingCheck1Code {
	#[serde(rename = "MoneyLaunderingCheck1Code")]
	pub money_laundering_check1_code: String,
}


// NameAndAddress4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress4 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress1,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// NamePrefix1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// NamePrefix1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix1Code {
	#[serde(rename = "NamePrefix1Code")]
	pub name_prefix1_code: String,
}


// NationalityCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NationalityCode {
	#[serde(rename = "NationalityCode")]
	pub nationality_code: String,
}


// NewIssueAllocation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewIssueAllocation2 {
	#[serde(rename = "Rstrctd")]
	pub rstrctd: bool,
	#[serde(rename = "XmptPrsnRsn")]
	pub xmpt_prsn_rsn: Option<String>,
	#[serde(rename = "DeMnms")]
	pub de_mnms: Option<DeMinimus1Choice>,
}


// NewZealandNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewZealandNCCIdentifier {
	#[serde(rename = "NewZealandNCCIdentifier")]
	pub new_zealand_ncc_identifier: String,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// Notification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Notification2 {
	#[serde(rename = "NtfctnTp")]
	pub ntfctn_tp: String,
	#[serde(rename = "Reqrd")]
	pub reqrd: bool,
	#[serde(rename = "DstrbtnTp")]
	pub dstrbtn_tp: Option<InformationDistribution1Choice>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OperationalStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OperationalStatus1Code {
	#[serde(rename = "OperationalStatus1Code")]
	pub operational_status1_code: String,
}


// OrderOriginatorEligibility1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderOriginatorEligibility1Code {
	#[serde(rename = "OrderOriginatorEligibility1Code")]
	pub order_originator_eligibility1_code: String,
}


// Organisation23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation23 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}


// Organisation39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation39 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "Id")]
	pub id: Option<PartyIdentification177Choice>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[serde(rename = "Purp")]
	pub purp: Option<String>,
	#[serde(rename = "RegnCtry")]
	pub regn_ctry: Option<String>,
	#[serde(rename = "RegnDt")]
	pub regn_dt: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<Vec<PostalAddress21>>,
	#[serde(rename = "TpOfOrg")]
	pub tp_of_org: Option<OrganisationType1Choice>,
	#[serde(rename = "PlcOfListg")]
	pub plc_of_listg: Option<Vec<String>>,
}


// OrganisationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// OrganisationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationType1Code {
	#[serde(rename = "OrganisationType1Code")]
	pub organisation_type1_code: String,
}


// OtherAccountStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherAccountStatus1 {
	#[serde(rename = "Sts")]
	pub sts: GenericIdentification36,
	#[serde(rename = "Rsn")]
	pub rsn: Option<GenericIdentification36>,
}


// OtherIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// OwnershipBeneficiaryRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnershipBeneficiaryRate1 {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Frctn")]
	pub frctn: Option<String>,
}


// OwnershipType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnershipType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Party47Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party47Choice {
	#[serde(rename = "Org")]
	pub org: Option<Organisation39>,
	#[serde(rename = "IndvPrsn")]
	pub indv_prsn: Option<IndividualPerson37>,
}


// PartyIdentification125Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification125Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification177Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification177Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PartyIdentificationType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationType7Code {
	#[serde(rename = "PartyIdentificationType7Code")]
	pub party_identification_type7_code: String,
}


// PartyProfileInformation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyProfileInformation5 {
	#[serde(rename = "CertfctnInd")]
	pub certfctn_ind: Option<bool>,
	#[serde(rename = "VldtngPty")]
	pub vldtng_pty: Option<String>,
	#[serde(rename = "ChckngPty")]
	pub chckng_pty: Option<String>,
	#[serde(rename = "RspnsblPty")]
	pub rspnsbl_pty: Option<String>,
	#[serde(rename = "CertTp")]
	pub cert_tp: Option<CertificationType1Choice>,
	#[serde(rename = "ChckngDt")]
	pub chckng_dt: Option<String>,
	#[serde(rename = "ChckngFrqcy")]
	pub chckng_frqcy: Option<String>,
	#[serde(rename = "NxtRvsnDt")]
	pub nxt_rvsn_dt: Option<String>,
	#[serde(rename = "SlryRg")]
	pub slry_rg: Option<String>,
	#[serde(rename = "SrcOfWlth")]
	pub src_of_wlth: Option<String>,
	#[serde(rename = "CstmrCndctClssfctn")]
	pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
	#[serde(rename = "RskLvl")]
	pub rsk_lvl: Option<RiskLevel2Choice>,
	#[serde(rename = "KnowYourCstmrChckTp")]
	pub know_your_cstmr_chck_tp: Option<KYCCheckType1Choice>,
	#[serde(rename = "KnowYourCstmrDBChck")]
	pub know_your_cstmr_db_chck: Option<DataBaseCheck1>,
}


// PartyRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole1Code {
	#[serde(rename = "PartyRole1Code")]
	pub party_role1_code: String,
}


// PartyRole2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PartyRole4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole4Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PartyRole5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PaymentCard29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCard29 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Nb")]
	pub nb: String,
	#[serde(rename = "HldrNm")]
	pub hldr_nm: String,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: String,
	#[serde(rename = "CardIssrNm")]
	pub card_issr_nm: Option<String>,
	#[serde(rename = "CardIssrId")]
	pub card_issr_id: Option<PartyIdentification125Choice>,
	#[serde(rename = "SctyCd")]
	pub scty_cd: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<String>,
}


// PaymentInstrument17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument17 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: String,
	#[serde(rename = "DvddPctg")]
	pub dvdd_pctg: Option<f64>,
	#[serde(rename = "SbcptPmtInstrm")]
	pub sbcpt_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[serde(rename = "RedPmtInstrm")]
	pub red_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[serde(rename = "DvddPmtInstrm")]
	pub dvdd_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[serde(rename = "SvgsPlanPmtInstrm")]
	pub svgs_plan_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[serde(rename = "IntrstPmtInstrm")]
	pub intrst_pmt_instrm: Option<PaymentInstrument19Choice>,
}


// PaymentInstrument19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument19Choice {
	#[serde(rename = "ChqDtls")]
	pub chq_dtls: Option<Cheque4>,
	#[serde(rename = "BkrsDrftDtls")]
	pub bkrs_drft_dtls: Option<Cheque4>,
}


// PaymentInstrument24Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument24Choice {
	#[serde(rename = "PmtCardDtls")]
	pub pmt_card_dtls: Option<PaymentCard29>,
	#[serde(rename = "DrctDbtDtls")]
	pub drct_dbt_dtls: Option<DirectDebitMandate7>,
	#[serde(rename = "Chq")]
	pub chq: Option<bool>,
	#[serde(rename = "BkrsDrft")]
	pub bkrs_drft: Option<bool>,
}


// PendingOpeningStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: PendingOpeningStatusReason2Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// PendingOpeningStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<PendingOpeningStatusReason1>>,
}


// PendingOpeningStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1Code {
	#[serde(rename = "PendingOpeningStatusReason1Code")]
	pub pending_opening_status_reason1_code: String,
}


// PendingOpeningStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// PendingStatusReason14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason14 {
	#[serde(rename = "Cd")]
	pub cd: PendingStatusReason2Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// PendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<PendingStatusReason14>>,
}


// PendingStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason1Code {
	#[serde(rename = "PendingStatusReason1Code")]
	pub pending_status_reason1_code: String,
}


// PendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// PercentageBoundedRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageBoundedRate {
	#[serde(rename = "PercentageBoundedRate")]
	pub percentage_bounded_rate: f64,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonalInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonalInformation1 {
	#[serde(rename = "NmOfFthr")]
	pub nm_of_fthr: Option<String>,
	#[serde(rename = "MdnNmOfMthr")]
	pub mdn_nm_of_mthr: Option<String>,
	#[serde(rename = "NmOfPrtnr")]
	pub nm_of_prtnr: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PlanStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlanStatus1Code {
	#[serde(rename = "PlanStatus1Code")]
	pub plan_status1_code: String,
}


// PlanStatus2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlanStatus2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticalExposureType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticalExposureType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticalExposureType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticalExposureType2Code {
	#[serde(rename = "PoliticalExposureType2Code")]
	pub political_exposure_type2_code: String,
}


// PoliticallyExposedPerson1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticallyExposedPerson1 {
	#[serde(rename = "PltclyXpsdPrsnTp")]
	pub pltcly_xpsd_prsn_tp: PoliticalExposureType2Choice,
	#[serde(rename = "PltclyXpsdPrsnSts")]
	pub pltcly_xpsd_prsn_sts: Option<PoliticallyExposedPersonStatus1Choice>,
}


// PoliticallyExposedPersonStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticallyExposedPersonStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticallyExposedPersonStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticallyExposedPersonStatus1Code {
	#[serde(rename = "PoliticallyExposedPersonStatus1Code")]
	pub politically_exposed_person_status1_code: String,
}


// PortugueseNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortugueseNCCIdentifier {
	#[serde(rename = "PortugueseNCCIdentifier")]
	pub portuguese_ncc_identifier: String,
}


// PositionEffect3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionEffect3Code {
	#[serde(rename = "PositionEffect3Code")]
	pub position_effect3_code: String,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// PostalAddress21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress21 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType2Choice>,
	#[serde(rename = "MlngInd")]
	pub mlng_ind: Option<bool>,
	#[serde(rename = "RegnAdrInd")]
	pub regn_adr_ind: Option<bool>,
	#[serde(rename = "CareOf")]
	pub care_of: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "PstBx")]
	pub pst_bx: Option<String>,
	#[serde(rename = "SdInBldg")]
	pub sd_in_bldg: Option<String>,
	#[serde(rename = "Flr")]
	pub flr: Option<String>,
	#[serde(rename = "SuiteId")]
	pub suite_id: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "DstrctNm")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "Vllg")]
	pub vllg: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "Stat")]
	pub stat: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// ProfileType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfileType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ProfileType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfileType1Code {
	#[serde(rename = "ProfileType1Code")]
	pub profile_type1_code: String,
}


// ProformaStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ProformaStatusReason2Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// ProformaStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<ProformaStatusReason1>>,
}


// ProformaStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1Code {
	#[serde(rename = "ProformaStatusReason1Code")]
	pub proforma_status_reason1_code: String,
}


// ProformaStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// Provided1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Provided1Code {
	#[serde(rename = "Provided1Code")]
	pub provided1_code: String,
}


// QUICKIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QUICKIdentifier {
	#[serde(rename = "QUICKIdentifier")]
	pub quick_identifier: String,
}


// RICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RICIdentifier {
	#[serde(rename = "RICIdentifier")]
	pub ric_identifier: String,
}


// Rank1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rank1Code {
	#[serde(rename = "Rank1Code")]
	pub rank1_code: String,
}


// Referred1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Referred1Code {
	#[serde(rename = "Referred1Code")]
	pub referred1_code: String,
}


// ReferredAgent3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredAgent3 {
	#[serde(rename = "Rfrd")]
	pub rfrd: String,
	#[serde(rename = "RfrdPlcmntAgt")]
	pub rfrd_plcmnt_agt: Option<PartyIdentification125Choice>,
}


// RegisteredShareholderName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredShareholderName1Choice {
	#[serde(rename = "IndvPrsn")]
	pub indv_prsn: Option<IndividualPerson29>,
	#[serde(rename = "Org")]
	pub org: Option<Organisation23>,
}


// RegulatoryInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryInformation1 {
	#[serde(rename = "Sctr")]
	pub sctr: Option<String>,
	#[serde(rename = "Brnch")]
	pub brnch: Option<String>,
	#[serde(rename = "Grp")]
	pub grp: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// Reinvestment4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reinvestment4 {
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument87,
	#[serde(rename = "ReqdNAVCcy")]
	pub reqd_nav_ccy: Option<String>,
	#[serde(rename = "RinvstmtPctg")]
	pub rinvstmt_pctg: f64,
}


// Repartition6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Repartition6 {
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmountOrPercentage1Choice,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrument87,
	#[serde(rename = "CcyOfPlan")]
	pub ccy_of_plan: Option<String>,
}


// ResidentialStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResidentialStatus1Code {
	#[serde(rename = "ResidentialStatus1Code")]
	pub residential_status1_code: String,
}


// RestrictionStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RestrictionStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// RestrictionStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RestrictionStatus1Code {
	#[serde(rename = "RestrictionStatus1Code")]
	pub restriction_status1_code: String,
}


// RiskLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskLevel1Code {
	#[serde(rename = "RiskLevel1Code")]
	pub risk_level1_code: String,
}


// RiskLevel2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskLevel2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// RoundingDirection1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoundingDirection1Code {
	#[serde(rename = "RoundingDirection1Code")]
	pub rounding_direction1_code: String,
}


// RoundingParameters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoundingParameters1 {
	#[serde(rename = "RndgMdlus")]
	pub rndg_mdlus: Option<f64>,
	#[serde(rename = "RndgDrctn")]
	pub rndg_drctn: String,
}


// RussianCentralBankIdentificationCodeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RussianCentralBankIdentificationCodeIdentifier {
	#[serde(rename = "RussianCentralBankIdentificationCodeIdentifier")]
	pub russian_central_bank_identification_code_identifier: String,
}


// SEDOLIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SEDOLIdentifier {
	#[serde(rename = "SEDOLIdentifier")]
	pub sedol_identifier: String,
}


// SecurityIdentification25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification25Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "SEDOL")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC")]
	pub ric: Option<String>,
	#[serde(rename = "TckrSymb")]
	pub tckr_symb: Option<String>,
	#[serde(rename = "Blmbrg")]
	pub blmbrg: Option<String>,
	#[serde(rename = "CTA")]
	pub cta: Option<String>,
	#[serde(rename = "QUICK")]
	pub quick: Option<String>,
	#[serde(rename = "Wrtppr")]
	pub wrtppr: Option<String>,
	#[serde(rename = "Dtch")]
	pub dtch: Option<String>,
	#[serde(rename = "Vlrn")]
	pub vlrn: Option<String>,
	#[serde(rename = "SCVM")]
	pub scvm: Option<String>,
	#[serde(rename = "Belgn")]
	pub belgn: Option<String>,
	#[serde(rename = "Cmon")]
	pub cmon: Option<String>,
	#[serde(rename = "OthrPrtryId")]
	pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
}


// SettlementFrequency1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFrequency1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// SettlementInstructionReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstructionReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// SettlementInstructionReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstructionReason1Code {
	#[serde(rename = "SettlementInstructionReason1Code")]
	pub settlement_instruction_reason1_code: String,
}


// SicovamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SicovamIdentifier {
	#[serde(rename = "SicovamIdentifier")]
	pub sicovam_identifier: String,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: String,
}


// SmallNetworkIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SmallNetworkIdentifier {
	#[serde(rename = "SmallNetworkIdentifier")]
	pub small_network_identifier: String,
}


// SouthAfricanNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SouthAfricanNCCIdentifier {
	#[serde(rename = "SouthAfricanNCCIdentifier")]
	pub south_african_ncc_identifier: String,
}


// SpanishDomesticInterbankingIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpanishDomesticInterbankingIdentifier {
	#[serde(rename = "SpanishDomesticInterbankingIdentifier")]
	pub spanish_domestic_interbanking_identifier: String,
}


// StatementFrequencyReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementFrequencyReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// SwissBCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwissBCIdentifier {
	#[serde(rename = "SwissBCIdentifier")]
	pub swiss_bc_identifier: String,
}


// SwissSICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwissSICIdentifier {
	#[serde(rename = "SwissSICIdentifier")]
	pub swiss_sic_identifier: String,
}


// TaxExemptReason3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptReason3Code {
	#[serde(rename = "TaxExemptReason3Code")]
	pub tax_exempt_reason3_code: String,
}


// TaxExemptionReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptionReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TaxReporting3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxReporting3 {
	#[serde(rename = "TaxtnCtry")]
	pub taxtn_ctry: String,
	#[serde(rename = "TaxRate")]
	pub tax_rate: Option<f64>,
	#[serde(rename = "TaxPyer")]
	pub tax_pyer: Option<PartyIdentification125Choice>,
	#[serde(rename = "TaxRcpt")]
	pub tax_rcpt: Option<PartyIdentification125Choice>,
	#[serde(rename = "CshAcctDtls")]
	pub csh_acct_dtls: Option<CashAccount204>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TaxWithholdingMethod3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxWithholdingMethod3Code {
	#[serde(rename = "TaxWithholdingMethod3Code")]
	pub tax_withholding_method3_code: String,
}


// ThirdPartyRights2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThirdPartyRights2 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
	#[serde(rename = "Hldr")]
	pub hldr: Option<PartyIdentification125Choice>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TickerIdentifier {
	#[serde(rename = "TickerIdentifier")]
	pub ticker_identifier: String,
}


// TransactionChannel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionChannel2Code {
	#[serde(rename = "TransactionChannel2Code")]
	pub transaction_channel2_code: String,
}


// TransactionChannelType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionChannelType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TransactionType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionType5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TreasuryProfile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TreasuryProfile1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "TradrTp")]
	pub tradr_tp: PartyRole5Choice,
	#[serde(rename = "Rate")]
	pub rate: f64,
}


// UKDomesticSortCodeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UKDomesticSortCodeIdentifier {
	#[serde(rename = "UKDomesticSortCodeIdentifier")]
	pub uk_domestic_sort_code_identifier: String,
}


// UnitsOrAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmount1Choice {
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
}


// UnitsOrAmountOrPercentage1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmountOrPercentage1Choice {
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
}


// ValorenIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValorenIdentifier {
	#[serde(rename = "ValorenIdentifier")]
	pub valoren_identifier: String,
}


// WertpapierIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WertpapierIdentifier {
	#[serde(rename = "WertpapierIdentifier")]
	pub wertpapier_identifier: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}

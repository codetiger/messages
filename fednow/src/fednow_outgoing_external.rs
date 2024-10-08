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
use crate::document::*;
use crate::fednow::key_exchange::*;
use crate::iso::head_001_001_02::BusinessApplicationHeaderV02;


// FedNowOutgoing ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowOutgoing {
	#[serde(rename = "FedNowTechnicalHeader")]
	pub fed_now_technical_header: Option<FedNowTechnicalHeader>,
	#[serde(rename = "FedNowOutgoingMessage")]
	pub fed_now_outgoing_message: FedNowOutgoingMessage,
}


// FedNowTechnicalHeader ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowTechnicalHeader {
}


// FedNowOutgoingMessage ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowOutgoingMessage {
	#[serde(rename = "FedNowMessageReject")]
	pub fed_now_message_reject: Option<FedNowMessageReject>,
	#[serde(rename = "FedNowBroadcast")]
	pub fed_now_broadcast: Option<FedNowBroadcast>,
	#[serde(rename = "FedNowReceiptAcknowledgement")]
	pub fed_now_receipt_acknowledgement: Option<FedNowReceiptAcknowledgement>,
	#[serde(rename = "FedNowSystemResponse")]
	pub fed_now_system_response: Option<FedNowSystemResponse>,
	#[serde(rename = "FedNowParticipantFile")]
	pub fed_now_participant_file: Option<FedNowParticipantFile>,
	#[serde(rename = "FedNowPaymentStatus")]
	pub fed_now_payment_status: Option<FedNowPaymentStatus>,
	#[serde(rename = "FedNowPaymentReturn")]
	pub fed_now_payment_return: Option<FedNowPaymentReturn>,
	#[serde(rename = "FedNowCustomerCreditTransfer")]
	pub fed_now_customer_credit_transfer: Option<FedNowCustomerCreditTransfer>,
	#[serde(rename = "FedNowInstitutionCreditTransfer")]
	pub fed_now_institution_credit_transfer: Option<FedNowInstitutionCreditTransfer>,
	#[serde(rename = "FedNowPaymentStatusRequest")]
	pub fed_now_payment_status_request: Option<FedNowPaymentStatusRequest>,
	#[serde(rename = "FedNowRequestForPayment")]
	pub fed_now_request_for_payment: Option<FedNowRequestForPayment>,
	#[serde(rename = "FedNowRequestForPaymentResponse")]
	pub fed_now_request_for_payment_response: Option<FedNowRequestForPaymentResponse>,
	#[serde(rename = "FedNowInformationRequest")]
	pub fed_now_information_request: Option<FedNowInformationRequest>,
	#[serde(rename = "FedNowAdditionalPaymentInformation")]
	pub fed_now_additional_payment_information: Option<FedNowAdditionalPaymentInformation>,
	#[serde(rename = "FedNowReturnRequestResponse")]
	pub fed_now_return_request_response: Option<FedNowReturnRequestResponse>,
	#[serde(rename = "FedNowInformationRequestResponse")]
	pub fed_now_information_request_response: Option<FedNowInformationRequestResponse>,
	#[serde(rename = "FedNowAccountActivityDetailsReport")]
	pub fed_now_account_activity_details_report: Option<FedNowAccountActivityDetailsReport>,
	#[serde(rename = "FedNowAccountActivityTotalsReport")]
	pub fed_now_account_activity_totals_report: Option<FedNowAccountActivityTotalsReport>,
	#[serde(rename = "FedNowAccountBalanceReport")]
	pub fed_now_account_balance_report: Option<FedNowAccountBalanceReport>,
	#[serde(rename = "AccountDebitCreditNotification")]
	pub account_debit_credit_notification: Option<AccountDebitCreditNotification>,
	#[serde(rename = "FedNowRequestForPaymentCancellationRequest")]
	pub fed_now_request_for_payment_cancellation_request: Option<FedNowRequestForPaymentCancellationRequest>,
	#[serde(rename = "FedNowRequestForPaymentCancellationRequestResponse")]
	pub fed_now_request_for_payment_cancellation_request_response: Option<FedNowRequestForPaymentCancellationRequestResponse>,
	#[serde(rename = "FedNowReturnRequest")]
	pub fed_now_return_request: Option<FedNowReturnRequest>,
	#[serde(rename = "FedNowOutgoingMessageSignatureManagement")]
	pub fed_now_outgoing_message_signature_management: Option<FedNowOutgoingMessageSignatureManagement>,
}


// FedNowMessageReject ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowMessageReject {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a2_document: Document,
}


// FedNowBroadcast ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowBroadcast {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a4_document: Document,
}


// FedNowReceiptAcknowledgement ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowReceiptAcknowledgement {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a7_document: Document,
}


// FedNowSystemResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowSystemResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a11_document: Document,
}


// FedNowParticipantFile ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowParticipantFile {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a998_document: Document,
}


// FedNowPaymentStatus ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPaymentStatus {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p2_document: Document,
}


// FedNowPaymentReturn ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPaymentReturn {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p4_document: Document,
}


// FedNowCustomerCreditTransfer ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowCustomerCreditTransfer {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p8_document: Document,
}


// FedNowInstitutionCreditTransfer ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowInstitutionCreditTransfer {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p9_document: Document,
}


// FedNowPaymentStatusRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPaymentStatusRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p28_document: Document,
}


// FedNowRequestForPayment ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPayment {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub pain13_document: Document,
}


// FedNowRequestForPaymentResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPaymentResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub pain14_document: Document,
}


// FedNowInformationRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowInformationRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c26_document: Document,
}


// FedNowAdditionalPaymentInformation ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAdditionalPaymentInformation {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c28_document: Document,
}


// FedNowRequestForPaymentCancellationRequestResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPaymentCancellationRequestResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c29_document: Document,
}


// FedNowReturnRequestResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowReturnRequestResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c29_document: Document,
}


// FedNowInformationRequestResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowInformationRequestResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c29_document: Document,
}


// FedNowAccountActivityDetailsReport ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAccountActivityDetailsReport {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c52_document: Document,
}


// FedNowAccountActivityTotalsReport ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAccountActivityTotalsReport {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c52_document: Document,
}


// FedNowAccountBalanceReport ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAccountBalanceReport {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c52_document: Document,
}


// AccountDebitCreditNotification ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDebitCreditNotification {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c54_document: Document,
}


// FedNowRequestForPaymentCancellationRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPaymentCancellationRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c55_document: Document,
}


// FedNowReturnRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowReturnRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c56_document: Document,
}


// FedNowOutgoingMessageSignatureManagement ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowOutgoingMessageSignatureManagement {
	#[serde(rename = "FedNowPublicKeyResponses")]
	pub ke_fed_now_public_key_responses: Option<FedNowPublicKeyResponses>,
	#[serde(rename = "FedNowCustomerMessageSignatureKeyOperationResponse")]
	pub ke_fed_now_customer_message_signature_key_operation_response: Option<FedNowCustomerMessageSignatureKeyOperationResponse>,
}

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


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd19DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AgreementType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AgriculturalCommodityDairy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityForestry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityGrain3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityLiveStock2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityOilSeed2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOilSeed2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityOliveOil3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityPotato2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommoditySeafood2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommoditySoft2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySoft2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AllocationIndicator1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AllocationIndicator1Code {
	#[serde(rename = "AllocationIndicator1Code")]
	pub allocation_indicator1_code: String,
}


// AmountAndDirection106 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection106 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AmountAndDirection109 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection109 {
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AssetClassCommodity7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity7Choice {
	#[serde(rename = "Agrcltrl")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural6Choice>,
	#[serde(rename = "Nrgy")]
	pub nrgy: Option<AssetClassCommodityEnergy3Choice>,
	#[serde(rename = "Envttl")]
	pub envttl: Option<AssetClassCommodityEnvironmental3Choice>,
	#[serde(rename = "Frtlzr")]
	pub frtlzr: Option<AssetClassCommodityFertilizer4Choice>,
	#[serde(rename = "Frght")]
	pub frght: Option<AssetClassCommodityFreight4Choice>,
	#[serde(rename = "Indx")]
	pub indx: Option<AssetClassCommodityIndex1>,
	#[serde(rename = "IndstrlPdct")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct2Choice>,
	#[serde(rename = "Infltn")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "Metl")]
	pub metl: Option<AssetClassCommodityMetal2Choice>,
	#[serde(rename = "MultiCmmdtyExtc")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr")]
	pub othr: Option<AssetClassCommodityOther1>,
	#[serde(rename = "OthrC10")]
	pub othr_c10: Option<AssetClassCommodityC10Other1>,
	#[serde(rename = "Ppr")]
	pub ppr: Option<AssetClassCommodityPaper5Choice>,
	#[serde(rename = "Plprpln")]
	pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
}


// AssetClassCommodityAgricultural6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural6Choice {
	#[serde(rename = "GrnOilSeed")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed2>,
	#[serde(rename = "Soft")]
	pub soft: Option<AgriculturalCommoditySoft2>,
	#[serde(rename = "Ptt")]
	pub ptt: Option<AgriculturalCommodityPotato2>,
	#[serde(rename = "OlvOil")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil3>,
	#[serde(rename = "Dairy")]
	pub dairy: Option<AgriculturalCommodityDairy2>,
	#[serde(rename = "Frstry")]
	pub frstry: Option<AgriculturalCommodityForestry2>,
	#[serde(rename = "Sfd")]
	pub sfd: Option<AgriculturalCommoditySeafood2>,
	#[serde(rename = "LiveStock")]
	pub live_stock: Option<AgriculturalCommodityLiveStock2>,
	#[serde(rename = "Grn")]
	pub grn: Option<AgriculturalCommodityGrain3>,
	#[serde(rename = "Othr")]
	pub othr: Option<AgriculturalCommodityOther2>,
}


// AssetClassCommodityC10Other1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityC10Other1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityEnergy3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy3Choice {
	#[serde(rename = "Elctrcty")]
	pub elctrcty: Option<EnergyCommodityElectricity2>,
	#[serde(rename = "NtrlGas")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas3>,
	#[serde(rename = "Oil")]
	pub oil: Option<EnergyCommodityOil3>,
	#[serde(rename = "Coal")]
	pub coal: Option<EnergyCommodityCoal2>,
	#[serde(rename = "IntrNrgy")]
	pub intr_nrgy: Option<EnergyCommodityInterEnergy2>,
	#[serde(rename = "RnwblNrgy")]
	pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy2>,
	#[serde(rename = "LghtEnd")]
	pub lght_end: Option<EnergyCommodityLightEnd2>,
	#[serde(rename = "Dstllts")]
	pub dstllts: Option<EnergyCommodityDistillates2>,
	#[serde(rename = "Othr")]
	pub othr: Option<EnergyCommodityOther2>,
}


// AssetClassCommodityEnvironmental3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental3Choice {
	#[serde(rename = "Emssns")]
	pub emssns: Option<EnvironmentalCommodityEmission3>,
	#[serde(rename = "Wthr")]
	pub wthr: Option<EnvironmentalCommodityWeather2>,
	#[serde(rename = "CrbnRltd")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated2>,
	#[serde(rename = "Othr")]
	pub othr: Option<EnvironmentCommodityOther2>,
}


// AssetClassCommodityFertilizer4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer4Choice {
	#[serde(rename = "Ammn")]
	pub ammn: Option<FertilizerCommodityAmmonia2>,
	#[serde(rename = "DmmnmPhspht")]
	pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate2>,
	#[serde(rename = "Ptsh")]
	pub ptsh: Option<FertilizerCommodityPotash2>,
	#[serde(rename = "Slphr")]
	pub slphr: Option<FertilizerCommoditySulphur2>,
	#[serde(rename = "Urea")]
	pub urea: Option<FertilizerCommodityUrea2>,
	#[serde(rename = "UreaAndAmmnmNtrt")]
	pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate2>,
	#[serde(rename = "Othr")]
	pub othr: Option<FertilizerCommodityOther2>,
}


// AssetClassCommodityFreight4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight4Choice {
	#[serde(rename = "Dry")]
	pub dry: Option<FreightCommodityDry3>,
	#[serde(rename = "Wet")]
	pub wet: Option<FreightCommodityWet3>,
	#[serde(rename = "CntnrShip")]
	pub cntnr_ship: Option<FreightCommodityContainerShip2>,
	#[serde(rename = "Othr")]
	pub othr: Option<FreightCommodityOther2>,
}


// AssetClassCommodityIndex1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndex1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityIndustrialProduct2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct2Choice {
	#[serde(rename = "Cnstrctn")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction2>,
	#[serde(rename = "Manfctg")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing2>,
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityMetal2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal2Choice {
	#[serde(rename = "NonPrcs")]
	pub non_prcs: Option<MetalCommodityNonPrecious2>,
	#[serde(rename = "Prcs")]
	pub prcs: Option<MetalCommodityPrecious2>,
}


// AssetClassCommodityMultiCommodityExotic1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOfficialEconomicStatistics1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityPaper5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper5Choice {
	#[serde(rename = "CntnrBrd")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
	#[serde(rename = "Nwsprnt")]
	pub nwsprnt: Option<PaperCommodityNewsprint2>,
	#[serde(rename = "Pulp")]
	pub pulp: Option<PaperCommodityPulp2>,
	#[serde(rename = "RcvrdPpr")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper3>,
	#[serde(rename = "Othr")]
	pub othr: Option<PaperCommodityOther1>,
}


// AssetClassCommodityPolypropylene4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene4Choice {
	#[serde(rename = "Plstc")]
	pub plstc: Option<PolypropyleneCommodityPlastic2>,
	#[serde(rename = "Othr")]
	pub othr: Option<PolypropyleneCommodityOther2>,
}


// AssetClassDetailedSubProductType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType10Code {
	#[serde(rename = "AssetClassDetailedSubProductType10Code")]
	pub asset_class_detailed_sub_product_type10_code: String,
}


// AssetClassDetailedSubProductType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType11Code {
	#[serde(rename = "AssetClassDetailedSubProductType11Code")]
	pub asset_class_detailed_sub_product_type11_code: String,
}


// AssetClassDetailedSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Code {
	#[serde(rename = "AssetClassDetailedSubProductType1Code")]
	pub asset_class_detailed_sub_product_type1_code: String,
}


// AssetClassDetailedSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType29Code {
	#[serde(rename = "AssetClassDetailedSubProductType29Code")]
	pub asset_class_detailed_sub_product_type29_code: String,
}


// AssetClassDetailedSubProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType2Code {
	#[serde(rename = "AssetClassDetailedSubProductType2Code")]
	pub asset_class_detailed_sub_product_type2_code: String,
}


// AssetClassDetailedSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType30Code {
	#[serde(rename = "AssetClassDetailedSubProductType30Code")]
	pub asset_class_detailed_sub_product_type30_code: String,
}


// AssetClassDetailedSubProductType31Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType31Code {
	#[serde(rename = "AssetClassDetailedSubProductType31Code")]
	pub asset_class_detailed_sub_product_type31_code: String,
}


// AssetClassDetailedSubProductType32Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType32Code {
	#[serde(rename = "AssetClassDetailedSubProductType32Code")]
	pub asset_class_detailed_sub_product_type32_code: String,
}


// AssetClassDetailedSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType33Code {
	#[serde(rename = "AssetClassDetailedSubProductType33Code")]
	pub asset_class_detailed_sub_product_type33_code: String,
}


// AssetClassDetailedSubProductType34Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType34Code {
	#[serde(rename = "AssetClassDetailedSubProductType34Code")]
	pub asset_class_detailed_sub_product_type34_code: String,
}


// AssetClassDetailedSubProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType5Code {
	#[serde(rename = "AssetClassDetailedSubProductType5Code")]
	pub asset_class_detailed_sub_product_type5_code: String,
}


// AssetClassDetailedSubProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType8Code {
	#[serde(rename = "AssetClassDetailedSubProductType8Code")]
	pub asset_class_detailed_sub_product_type8_code: String,
}


// AssetClassProductType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType11Code {
	#[serde(rename = "AssetClassProductType11Code")]
	pub asset_class_product_type11_code: String,
}


// AssetClassProductType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType12Code {
	#[serde(rename = "AssetClassProductType12Code")]
	pub asset_class_product_type12_code: String,
}


// AssetClassProductType13Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType13Code {
	#[serde(rename = "AssetClassProductType13Code")]
	pub asset_class_product_type13_code: String,
}


// AssetClassProductType14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType14Code {
	#[serde(rename = "AssetClassProductType14Code")]
	pub asset_class_product_type14_code: String,
}


// AssetClassProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType15Code {
	#[serde(rename = "AssetClassProductType15Code")]
	pub asset_class_product_type15_code: String,
}


// AssetClassProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType16Code {
	#[serde(rename = "AssetClassProductType16Code")]
	pub asset_class_product_type16_code: String,
}


// AssetClassProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType1Code {
	#[serde(rename = "AssetClassProductType1Code")]
	pub asset_class_product_type1_code: String,
}


// AssetClassProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType2Code {
	#[serde(rename = "AssetClassProductType2Code")]
	pub asset_class_product_type2_code: String,
}


// AssetClassProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType3Code {
	#[serde(rename = "AssetClassProductType3Code")]
	pub asset_class_product_type3_code: String,
}


// AssetClassProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType4Code {
	#[serde(rename = "AssetClassProductType4Code")]
	pub asset_class_product_type4_code: String,
}


// AssetClassProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType5Code {
	#[serde(rename = "AssetClassProductType5Code")]
	pub asset_class_product_type5_code: String,
}


// AssetClassProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType6Code {
	#[serde(rename = "AssetClassProductType6Code")]
	pub asset_class_product_type6_code: String,
}


// AssetClassProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType7Code {
	#[serde(rename = "AssetClassProductType7Code")]
	pub asset_class_product_type7_code: String,
}


// AssetClassProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType8Code {
	#[serde(rename = "AssetClassProductType8Code")]
	pub asset_class_product_type8_code: String,
}


// AssetClassProductType9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType9Code {
	#[serde(rename = "AssetClassProductType9Code")]
	pub asset_class_product_type9_code: String,
}


// AssetClassSubProductType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType10Code {
	#[serde(rename = "AssetClassSubProductType10Code")]
	pub asset_class_sub_product_type10_code: String,
}


// AssetClassSubProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType15Code {
	#[serde(rename = "AssetClassSubProductType15Code")]
	pub asset_class_sub_product_type15_code: String,
}


// AssetClassSubProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType16Code {
	#[serde(rename = "AssetClassSubProductType16Code")]
	pub asset_class_sub_product_type16_code: String,
}


// AssetClassSubProductType18Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType18Code {
	#[serde(rename = "AssetClassSubProductType18Code")]
	pub asset_class_sub_product_type18_code: String,
}


// AssetClassSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType1Code {
	#[serde(rename = "AssetClassSubProductType1Code")]
	pub asset_class_sub_product_type1_code: String,
}


// AssetClassSubProductType20Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType20Code {
	#[serde(rename = "AssetClassSubProductType20Code")]
	pub asset_class_sub_product_type20_code: String,
}


// AssetClassSubProductType21Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType21Code {
	#[serde(rename = "AssetClassSubProductType21Code")]
	pub asset_class_sub_product_type21_code: String,
}


// AssetClassSubProductType22Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType22Code {
	#[serde(rename = "AssetClassSubProductType22Code")]
	pub asset_class_sub_product_type22_code: String,
}


// AssetClassSubProductType23Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType23Code {
	#[serde(rename = "AssetClassSubProductType23Code")]
	pub asset_class_sub_product_type23_code: String,
}


// AssetClassSubProductType24Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType24Code {
	#[serde(rename = "AssetClassSubProductType24Code")]
	pub asset_class_sub_product_type24_code: String,
}


// AssetClassSubProductType25Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType25Code {
	#[serde(rename = "AssetClassSubProductType25Code")]
	pub asset_class_sub_product_type25_code: String,
}


// AssetClassSubProductType26Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType26Code {
	#[serde(rename = "AssetClassSubProductType26Code")]
	pub asset_class_sub_product_type26_code: String,
}


// AssetClassSubProductType27Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType27Code {
	#[serde(rename = "AssetClassSubProductType27Code")]
	pub asset_class_sub_product_type27_code: String,
}


// AssetClassSubProductType28Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType28Code {
	#[serde(rename = "AssetClassSubProductType28Code")]
	pub asset_class_sub_product_type28_code: String,
}


// AssetClassSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType29Code {
	#[serde(rename = "AssetClassSubProductType29Code")]
	pub asset_class_sub_product_type29_code: String,
}


// AssetClassSubProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType2Code {
	#[serde(rename = "AssetClassSubProductType2Code")]
	pub asset_class_sub_product_type2_code: String,
}


// AssetClassSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType30Code {
	#[serde(rename = "AssetClassSubProductType30Code")]
	pub asset_class_sub_product_type30_code: String,
}


// AssetClassSubProductType31Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType31Code {
	#[serde(rename = "AssetClassSubProductType31Code")]
	pub asset_class_sub_product_type31_code: String,
}


// AssetClassSubProductType32Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType32Code {
	#[serde(rename = "AssetClassSubProductType32Code")]
	pub asset_class_sub_product_type32_code: String,
}


// AssetClassSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType33Code {
	#[serde(rename = "AssetClassSubProductType33Code")]
	pub asset_class_sub_product_type33_code: String,
}


// AssetClassSubProductType34Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType34Code {
	#[serde(rename = "AssetClassSubProductType34Code")]
	pub asset_class_sub_product_type34_code: String,
}


// AssetClassSubProductType35Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType35Code {
	#[serde(rename = "AssetClassSubProductType35Code")]
	pub asset_class_sub_product_type35_code: String,
}


// AssetClassSubProductType36Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType36Code {
	#[serde(rename = "AssetClassSubProductType36Code")]
	pub asset_class_sub_product_type36_code: String,
}


// AssetClassSubProductType37Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType37Code {
	#[serde(rename = "AssetClassSubProductType37Code")]
	pub asset_class_sub_product_type37_code: String,
}


// AssetClassSubProductType39Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType39Code {
	#[serde(rename = "AssetClassSubProductType39Code")]
	pub asset_class_sub_product_type39_code: String,
}


// AssetClassSubProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType3Code {
	#[serde(rename = "AssetClassSubProductType3Code")]
	pub asset_class_sub_product_type3_code: String,
}


// AssetClassSubProductType40Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType40Code {
	#[serde(rename = "AssetClassSubProductType40Code")]
	pub asset_class_sub_product_type40_code: String,
}


// AssetClassSubProductType41Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType41Code {
	#[serde(rename = "AssetClassSubProductType41Code")]
	pub asset_class_sub_product_type41_code: String,
}


// AssetClassSubProductType42Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType42Code {
	#[serde(rename = "AssetClassSubProductType42Code")]
	pub asset_class_sub_product_type42_code: String,
}


// AssetClassSubProductType43Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType43Code {
	#[serde(rename = "AssetClassSubProductType43Code")]
	pub asset_class_sub_product_type43_code: String,
}


// AssetClassSubProductType44Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType44Code {
	#[serde(rename = "AssetClassSubProductType44Code")]
	pub asset_class_sub_product_type44_code: String,
}


// AssetClassSubProductType45Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType45Code {
	#[serde(rename = "AssetClassSubProductType45Code")]
	pub asset_class_sub_product_type45_code: String,
}


// AssetClassSubProductType46Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType46Code {
	#[serde(rename = "AssetClassSubProductType46Code")]
	pub asset_class_sub_product_type46_code: String,
}


// AssetClassSubProductType49Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType49Code {
	#[serde(rename = "AssetClassSubProductType49Code")]
	pub asset_class_sub_product_type49_code: String,
}


// AssetClassSubProductType50Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType50Code {
	#[serde(rename = "AssetClassSubProductType50Code")]
	pub asset_class_sub_product_type50_code: String,
}


// AssetClassSubProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType5Code {
	#[serde(rename = "AssetClassSubProductType5Code")]
	pub asset_class_sub_product_type5_code: String,
}


// AssetClassSubProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType6Code {
	#[serde(rename = "AssetClassSubProductType6Code")]
	pub asset_class_sub_product_type6_code: String,
}


// AssetClassSubProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType7Code {
	#[serde(rename = "AssetClassSubProductType7Code")]
	pub asset_class_sub_product_type7_code: String,
}


// AssetClassSubProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType8Code {
	#[serde(rename = "AssetClassSubProductType8Code")]
	pub asset_class_sub_product_type8_code: String,
}


// BaseOne18Rate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOne18Rate {
	#[serde(rename = "BaseOne18Rate")]
	pub base_one18_rate: f64,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BasketConstituents3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketConstituents3 {
	#[serde(rename = "InstrmId")]
	pub instrm_id: InstrumentIdentification6Choice,
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// Cleared23Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cleared23Choice {
	#[serde(rename = "Clrd")]
	pub clrd: Option<ClearingPartyAndTime21Choice>,
	#[serde(rename = "IntndToClear")]
	pub intnd_to_clear: Option<ClearingPartyAndTime22Choice>,
	#[serde(rename = "NonClrd")]
	pub non_clrd: Option<ClearingExceptionOrExemption3Choice>,
}


// ClearingAccountType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingAccountType4Code {
	#[serde(rename = "ClearingAccountType4Code")]
	pub clearing_account_type4_code: String,
}


// ClearingExceptionOrExemption2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption2 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: NonClearingReason2,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<NonClearingReason2>,
}


// ClearingExceptionOrExemption3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption3Choice {
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "CtrPties")]
	pub ctr_pties: Option<ClearingExceptionOrExemption2>,
}


// ClearingExemptionException1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingExemptionException1Code {
	#[serde(rename = "ClearingExemptionException1Code")]
	pub clearing_exemption_exception1_code: String,
}


// ClearingObligationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingObligationType1Code {
	#[serde(rename = "ClearingObligationType1Code")]
	pub clearing_obligation_type1_code: String,
}


// ClearingPartyAndTime21Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime21Choice {
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "Dtls")]
	pub dtls: Option<ClearingPartyAndTime22>,
}


// ClearingPartyAndTime22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime22 {
	#[serde(rename = "CCP")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrRctDtTm")]
	pub clr_rct_dt_tm: Option<String>,
	#[serde(rename = "ClrDtTm")]
	pub clr_dt_tm: Option<String>,
	#[serde(rename = "ClrIdr")]
	pub clr_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "OrgnlIdr")]
	pub orgnl_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "OrgnlTradRpstryIdr")]
	pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrAcctOrgn")]
	pub clr_acct_orgn: Option<String>,
}


// ClearingPartyAndTime22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime22Choice {
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "Dtls")]
	pub dtls: Option<ClearingPartyAndTime23>,
}


// ClearingPartyAndTime23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime23 {
	#[serde(rename = "CCP")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrRctDtTm")]
	pub clr_rct_dt_tm: Option<String>,
	#[serde(rename = "ClrDtTm")]
	pub clr_dt_tm: Option<String>,
	#[serde(rename = "ClrIdr")]
	pub clr_idr: Option<UniqueTransactionIdentifier1Choice>,
	#[serde(rename = "OrgnlIdr")]
	pub orgnl_idr: Option<UniqueTransactionIdentifier1Choice>,
	#[serde(rename = "OrgnlTradRpstryIdr")]
	pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
}


// CollateralPortfolioCode6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode6Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
}


// CommonTradeDataReport71 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommonTradeDataReport71 {
	#[serde(rename = "CtrctData")]
	pub ctrct_data: Option<ContractType15>,
	#[serde(rename = "TxData")]
	pub tx_data: TradeTransaction50,
}


// ContractType15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractType15 {
	#[serde(rename = "CtrctTp")]
	pub ctrct_tp: Option<String>,
	#[serde(rename = "AsstClss")]
	pub asst_clss: Option<String>,
	#[serde(rename = "PdctClssfctn")]
	pub pdct_clssfctn: Option<String>,
	#[serde(rename = "PdctId")]
	pub pdct_id: Option<SecurityIdentification46>,
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
	#[serde(rename = "UndrlygAsstTradgPltfmIdr")]
	pub undrlyg_asst_tradg_pltfm_idr: Option<String>,
	#[serde(rename = "UndrlygAsstPricSrc")]
	pub undrlyg_asst_pric_src: Option<String>,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<CurrencyExchange23>,
	#[serde(rename = "SttlmCcyScndLeg")]
	pub sttlm_ccy_scnd_leg: Option<CurrencyExchange23>,
	#[serde(rename = "PlcOfSttlm")]
	pub plc_of_sttlm: Option<String>,
	#[serde(rename = "DerivBasedOnCrptAsst")]
	pub deriv_based_on_crpt_asst: Option<bool>,
}


// ContractValuationData8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractValuationData8 {
	#[serde(rename = "CtrctVal")]
	pub ctrct_val: Option<AmountAndDirection109>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Dlta")]
	pub dlta: Option<f64>,
}


// Counterparty45 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty45 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification248Choice,
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "TradgCpcty")]
	pub tradg_cpcty: Option<String>,
	#[serde(rename = "DrctnOrSd")]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[serde(rename = "TradrLctn")]
	pub tradr_lctn: Option<String>,
	#[serde(rename = "BookgLctn")]
	pub bookg_lctn: Option<String>,
	#[serde(rename = "RptgXmptn")]
	pub rptg_xmptn: Option<ReportingExemption1>,
}


// Counterparty46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty46 {
	#[serde(rename = "IdTp")]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "RptgOblgtn")]
	pub rptg_oblgtn: Option<bool>,
}


// CounterpartySpecificData36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartySpecificData36 {
	#[serde(rename = "CtrPty")]
	pub ctr_pty: TradeCounterpartyReport20,
	#[serde(rename = "Valtn")]
	pub valtn: Option<ContractValuationData8>,
	#[serde(rename = "RptgTmStmp")]
	pub rptg_tm_stmp: Option<String>,
}


// CounterpartyTradeNature15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyTradeNature15Choice {
	#[serde(rename = "FI")]
	pub fi: Option<FinancialInstitutionSector1>,
	#[serde(rename = "NFI")]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[serde(rename = "CntrlCntrPty")]
	pub cntrl_cntr_pty: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CountrySubDivisionCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "CountrySubDivisionCode")]
	pub country_sub_division_code: String,
}


// CreditDerivative4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDerivative4 {
	#[serde(rename = "Snrty")]
	pub snrty: Option<String>,
	#[serde(rename = "RefPty")]
	pub ref_pty: Option<DerivativePartyIdentification1Choice>,
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<String>,
	#[serde(rename = "ClctnBsis")]
	pub clctn_bsis: Option<String>,
	#[serde(rename = "Srs")]
	pub srs: Option<f64>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<f64>,
	#[serde(rename = "IndxFctr")]
	pub indx_fctr: Option<f64>,
	#[serde(rename = "Trch")]
	pub trch: Option<TrancheIndicator3Choice>,
}


// CurrencyExchange22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange22 {
	#[serde(rename = "DlvrblCrossCcy")]
	pub dlvrbl_cross_ccy: Option<String>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "FwdXchgRate")]
	pub fwd_xchg_rate: Option<f64>,
	#[serde(rename = "XchgRateBsis")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "FxgDt")]
	pub fxg_dt: Option<String>,
}


// CurrencyExchange23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange23 {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "FwdXchgRate")]
	pub fwd_xchg_rate: Option<f64>,
	#[serde(rename = "XchgRateBsis")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "FxgDt")]
	pub fxg_dt: Option<String>,
}


// CustomBasket4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomBasket4 {
	#[serde(rename = "Strr")]
	pub strr: Option<String>,
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Cnsttnts")]
	pub cnsttnts: Option<Vec<BasketConstituents3>>,
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DebtInstrumentSeniorityType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrumentSeniorityType2Code {
	#[serde(rename = "DebtInstrumentSeniorityType2Code")]
	pub debt_instrument_seniority_type2_code: String,
}


// DeliveryInterconnectionPoint1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeliveryInterconnectionPoint1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DerivativeEvent6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeEvent6 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Id")]
	pub id: Option<EventIdentifier1Choice>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "AmdmntInd")]
	pub amdmnt_ind: Option<bool>,
}


// DerivativeEventType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeEventType3Code {
	#[serde(rename = "DerivativeEventType3Code")]
	pub derivative_event_type3_code: String,
}


// DerivativePartyIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativePartyIdentification1Choice {
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// DerivativesTradeReportV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeReportV04 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: TradeReportHeader4,
	#[serde(rename = "TradData")]
	pub trad_data: TradeData59Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Direction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: String,
	#[serde(rename = "DrctnOfTheScndLeg")]
	pub drctn_of_the_scnd_leg: Option<String>,
}


// Direction4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[serde(rename = "Drctn")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd")]
	pub ctr_pty_sd: Option<String>,
}


// DisseminationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisseminationData1 {
	#[serde(rename = "DssmntnIdr")]
	pub dssmntn_idr: String,
	#[serde(rename = "OrgnlDssmntnIdr")]
	pub orgnl_dssmntn_idr: Option<String>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: String,
}


// DurationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DurationType1Code {
	#[serde(rename = "DurationType1Code")]
	pub duration_type1_code: String,
}


// EICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EICIdentifier {
	#[serde(rename = "EICIdentifier")]
	pub eic_identifier: String,
}


// EmbeddedType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmbeddedType1Code {
	#[serde(rename = "EmbeddedType1Code")]
	pub embedded_type1_code: String,
}


// EnergyCommodityCoal2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityCoal2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityDistillates2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityElectricity2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityElectricity2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityInterEnergy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityLightEnd2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityNaturalGas3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityOil3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityRenewableEnergy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyDeliveryAttribute10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyDeliveryAttribute10 {
	#[serde(rename = "DlvryIntrvl")]
	pub dlvry_intrvl: Option<Vec<TimePeriodDetails1>>,
	#[serde(rename = "DlvryDt")]
	pub dlvry_dt: Option<DatePeriod1>,
	#[serde(rename = "Drtn")]
	pub drtn: Option<String>,
	#[serde(rename = "WkDay")]
	pub wk_day: Option<Vec<String>>,
	#[serde(rename = "DlvryCpcty")]
	pub dlvry_cpcty: Option<Quantity47Choice>,
	#[serde(rename = "QtyUnit")]
	pub qty_unit: Option<EnergyQuantityUnit2Choice>,
	#[serde(rename = "PricTmIntrvlQty")]
	pub pric_tm_intrvl_qty: Option<AmountAndDirection106>,
}


// EnergyLoadType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyLoadType1Code {
	#[serde(rename = "EnergyLoadType1Code")]
	pub energy_load_type1_code: String,
}


// EnergyQuantityUnit2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyQuantityUnit2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// EnergyQuantityUnit2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyQuantityUnit2Code {
	#[serde(rename = "EnergyQuantityUnit2Code")]
	pub energy_quantity_unit2_code: String,
}


// EnergySpecificAttribute9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergySpecificAttribute9 {
	#[serde(rename = "DlvryPtOrZone")]
	pub dlvry_pt_or_zone: Option<Vec<DeliveryInterconnectionPoint1Choice>>,
	#[serde(rename = "IntrCnnctnPt")]
	pub intr_cnnctn_pt: Option<DeliveryInterconnectionPoint1Choice>,
	#[serde(rename = "LdTp")]
	pub ld_tp: Option<String>,
	#[serde(rename = "DlvryAttr")]
	pub dlvry_attr: Option<Vec<EnergyDeliveryAttribute10>>,
}


// EnvironmentCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnvironmentalCommodityCarbonRelated2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnvironmentalCommodityEmission3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnvironmentalCommodityWeather2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EventIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventIdentifier1Choice {
	#[serde(rename = "EvtIdr")]
	pub evt_idr: Option<String>,
	#[serde(rename = "PstTradRskRdctnIdr")]
	pub pst_trad_rsk_rdctn_idr: Option<PostTradeRiskReductionIdentifier1>,
}


// ExchangeRateBasis1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1 {
	#[serde(rename = "BaseCcy")]
	pub base_ccy: String,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: String,
}


// ExchangeRateBasis1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1Choice {
	#[serde(rename = "CcyPair")]
	pub ccy_pair: Option<ExchangeRateBasis1>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ExerciseDate1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExerciseDate1Choice {
	#[serde(rename = "FrstExrcDt")]
	pub frst_exrc_dt: Option<String>,
	#[serde(rename = "PdgDtAplbl")]
	pub pdg_dt_aplbl: Option<String>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "ExternalAgreementType1Code")]
	pub external_agreement_type1_code: String,
}


// ExternalBenchmarkCurveName1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBenchmarkCurveName1Code {
	#[serde(rename = "ExternalBenchmarkCurveName1Code")]
	pub external_benchmark_curve_name1_code: String,
}


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPartyRelationshipType1Code {
	#[serde(rename = "ExternalPartyRelationshipType1Code")]
	pub external_party_relationship_type1_code: String,
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalUnitOfMeasure1Code {
	#[serde(rename = "ExternalUnitOfMeasure1Code")]
	pub external_unit_of_measure1_code: String,
}


// FertilizerCommodityAmmonia2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityDiammoniumPhosphate2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityPotash2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommoditySulphur2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityUrea2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityUreaAndAmmoniumNitrate2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FinancialInstitutionSector1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionSector1 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[serde(rename = "ClrThrshld")]
	pub clr_thrshld: Option<bool>,
}


// FinancialInstrumentContractType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentContractType2Code {
	#[serde(rename = "FinancialInstrumentContractType2Code")]
	pub financial_instrument_contract_type2_code: String,
}


// FinancialInstrumentQuantity32Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity32Choice {
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "NmnlVal")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
}


// FinancialPartyClassification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartyClassification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// FinancialPartySectorType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartySectorType3Code {
	#[serde(rename = "FinancialPartySectorType3Code")]
	pub financial_party_sector_type3_code: String,
}


// FixedRate10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedRate10 {
	#[serde(rename = "Rate")]
	pub rate: Option<SecuritiesTransactionPrice14Choice>,
	#[serde(rename = "DayCnt")]
	pub day_cnt: Option<InterestComputationMethodFormat7>,
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
}


// FloatingRate13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRate13 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Rate")]
	pub rate: Option<FloatingRateIdentification8Choice>,
	#[serde(rename = "RefPrd")]
	pub ref_prd: Option<InterestRateContractTerm4>,
	#[serde(rename = "Sprd")]
	pub sprd: Option<SecuritiesTransactionPrice20Choice>,
	#[serde(rename = "DayCnt")]
	pub day_cnt: Option<InterestComputationMethodFormat7>,
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
	#[serde(rename = "RstFrqcy")]
	pub rst_frqcy: Option<InterestRateFrequency3Choice>,
	#[serde(rename = "NxtFltgRst")]
	pub nxt_fltg_rst: Option<ResetDateAndValue1>,
	#[serde(rename = "LastFltgRst")]
	pub last_fltg_rst: Option<ResetDateAndValue1>,
}


// FloatingRateIdentification8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRateIdentification8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FreightCommodityContainerShip2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FreightCommodityDry3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// FreightCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FreightCommodityWet3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// Frequency13Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency13Code {
	#[serde(rename = "Frequency13Code")]
	pub frequency13_code: String,
}


// Frequency19Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency19Code {
	#[serde(rename = "Frequency19Code")]
	pub frequency19_code: String,
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification179 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification179 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification184 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification184 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Src")]
	pub src: String,
}


// GenericIdentification185 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification185 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
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


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// IndexIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexIdentification1 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
}


// IndustrialProductCommodityConstruction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// IndustrialProductCommodityManufacturing2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// InstrumentIdentification6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentIdentification6Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier1Choice>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<GenericIdentification184>,
}


// InterestComputationMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethod4Code {
	#[serde(rename = "InterestComputationMethod4Code")]
	pub interest_computation_method4_code: String,
}


// InterestComputationMethodFormat7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat7 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Nrrtv")]
	pub nrrtv: Option<String>,
}


// InterestRate33Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate33Choice {
	#[serde(rename = "Fxd")]
	pub fxd: Option<FixedRate10>,
	#[serde(rename = "Fltg")]
	pub fltg: Option<FloatingRate13>,
}


// InterestRateContractTerm4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm4 {
	#[serde(rename = "Unit")]
	pub unit: Option<String>,
	#[serde(rename = "Val")]
	pub val: Option<f64>,
}


// InterestRateFrequency3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateFrequency3Choice {
	#[serde(rename = "Term")]
	pub term: Option<InterestRateContractTerm4>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InterestRateLegs14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateLegs14 {
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<InterestRate33Choice>,
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<InterestRate33Choice>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "LongFraction19DecimalNumber")]
	pub long_fraction19_decimal_number: f64,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// MarginPortfolio4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio4 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	#[serde(rename = "VartnMrgnPrtflCd")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MasterAgreement8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[serde(rename = "Tp")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
	#[serde(rename = "OthrMstrAgrmtDtls")]
	pub othr_mstr_agrmt_dtls: Option<String>,
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max100Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max100Text {
	#[serde(rename = "Max100Text")]
	pub max100_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max210Text {
	#[serde(rename = "Max210Text")]
	pub max210_text: String,
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "Max3Number")]
	pub max3_number: f64,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// MetalCommodityNonPrecious2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityNonPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// MetalCommodityPrecious2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModificationLevel1Code {
	#[serde(rename = "ModificationLevel1Code")]
	pub modification_level1_code: String,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NonClearingReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonClearingReason2 {
	#[serde(rename = "ClrXmptnXcptn")]
	pub clr_xmptn_xcptn: Vec<String>,
	#[serde(rename = "NonClrRsnInf")]
	pub non_clr_rsn_inf: Option<String>,
}


// NonFinancialInstitutionSector10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonFinancialInstitutionSector10 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<GenericIdentification175>,
	#[serde(rename = "ClrThrshld")]
	pub clr_thrshld: Option<bool>,
	#[serde(rename = "DrctlyLkdActvty")]
	pub drctly_lkd_actvty: Option<bool>,
	#[serde(rename = "FdrlInstn")]
	pub fdrl_instn: Option<bool>,
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// NotionalAmount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount5 {
	#[serde(rename = "Amt")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule11>>,
}


// NotionalAmount6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount6 {
	#[serde(rename = "Amt")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule11>>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
}


// NotionalAmountLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmountLegs5 {
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<NotionalAmount5>,
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<NotionalAmount6>,
}


// NotionalQuantity9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantity9 {
	#[serde(rename = "TtlQty")]
	pub ttl_qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Dtls")]
	pub dtls: Option<QuantityOrTerm1Choice>,
}


// NotionalQuantityLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantityLegs5 {
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<NotionalQuantity9>,
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<NotionalQuantity9>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OptionBarrierLevel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionBarrierLevel1Choice {
	#[serde(rename = "Sngl")]
	pub sngl: Option<SecuritiesTransactionPrice23Choice>,
	#[serde(rename = "Mltpl")]
	pub mltpl: Option<OptionMultipleBarrierLevels1>,
}


// OptionMultipleBarrierLevels1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionMultipleBarrierLevels1 {
	#[serde(rename = "LwrLvl")]
	pub lwr_lvl: SecuritiesTransactionPrice23Choice,
	#[serde(rename = "UpperLvl")]
	pub upper_lvl: SecuritiesTransactionPrice23Choice,
}


// OptionOrSwaption11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionOrSwaption11 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "MbddTp")]
	pub mbdd_tp: Option<String>,
	#[serde(rename = "ExrcStyle")]
	pub exrc_style: Option<Vec<String>>,
	#[serde(rename = "ExrcDt")]
	pub exrc_dt: Option<ExerciseDate1Choice>,
	#[serde(rename = "StrkPric")]
	pub strk_pric: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "StrkPricSchdl")]
	pub strk_pric_schdl: Option<Vec<Schedule4>>,
	#[serde(rename = "CallAmt")]
	pub call_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "PutAmt")]
	pub put_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "PrmAmt")]
	pub prm_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "PrmPmtDt")]
	pub prm_pmt_dt: Option<String>,
	#[serde(rename = "MtrtyDtOfUndrlyg")]
	pub mtrty_dt_of_undrlyg: Option<String>,
	#[serde(rename = "BrrrLvls")]
	pub brrr_lvls: Option<OptionBarrierLevel1Choice>,
}


// OptionParty1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty1Code {
	#[serde(rename = "OptionParty1Code")]
	pub option_party1_code: String,
}


// OptionParty3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty3Code {
	#[serde(rename = "OptionParty3Code")]
	pub option_party3_code: String,
}


// OptionStyle6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionStyle6Code {
	#[serde(rename = "OptionStyle6Code")]
	pub option_style6_code: String,
}


// OptionType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionType2Code {
	#[serde(rename = "OptionType2Code")]
	pub option_type2_code: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// OtherPayment5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherPayment5 {
	#[serde(rename = "PmtAmt")]
	pub pmt_amt: Option<AmountAndDirection106>,
	#[serde(rename = "PmtTp")]
	pub pmt_tp: Option<PaymentType5Choice>,
	#[serde(rename = "PmtDt")]
	pub pmt_dt: Option<String>,
	#[serde(rename = "PmtPyer")]
	pub pmt_pyer: Option<PartyIdentification236Choice>,
	#[serde(rename = "PmtRcvr")]
	pub pmt_rcvr: Option<PartyIdentification236Choice>,
}


// PTRREvent2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PTRREvent2 {
	#[serde(rename = "Tchnq")]
	pub tchnq: String,
	#[serde(rename = "SvcPrvdr")]
	pub svc_prvdr: Option<OrganisationIdentification15Choice>,
}


// Package4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Package4 {
	#[serde(rename = "CmplxTradId")]
	pub cmplx_trad_id: Option<String>,
	#[serde(rename = "FxSwpLkId")]
	pub fx_swp_lk_id: Option<String>,
	#[serde(rename = "Pric")]
	pub pric: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "Sprd")]
	pub sprd: Option<SecuritiesTransactionPrice20Choice>,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PaperCommodityContainerBoard2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityNewsprint2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityPulp2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityPulp2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityRecoveredPaper3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PaymentType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType4Code {
	#[serde(rename = "PaymentType4Code")]
	pub payment_type4_code: String,
}


// PaymentType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType5Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "PrtryTp")]
	pub prtry_tp: Option<String>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PhysicalTransferType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhysicalTransferType4Code {
	#[serde(rename = "PhysicalTransferType4Code")]
	pub physical_transfer_type4_code: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PolypropyleneCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PolypropyleneCommodityPlastic2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "PrtflTxXmptn")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// PostTradeRiskReductionIdentifier1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostTradeRiskReductionIdentifier1 {
	#[serde(rename = "Strr")]
	pub strr: String,
	#[serde(rename = "Id")]
	pub id: String,
}


// PriceData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceData2 {
	#[serde(rename = "Pric")]
	pub pric: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule1>>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "PricMltplr")]
	pub pric_mltplr: Option<f64>,
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceStatus1Code {
	#[serde(rename = "PriceStatus1Code")]
	pub price_status1_code: String,
}


// PriceStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceStatus2Code {
	#[serde(rename = "PriceStatus2Code")]
	pub price_status2_code: String,
}


// ProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductType4Code {
	#[serde(rename = "ProductType4Code")]
	pub product_type4_code: String,
}


// Quantity47Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Quantity47Choice {
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// QuantityOrTerm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityOrTerm1Choice {
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule10>>,
	#[serde(rename = "Term")]
	pub term: Option<QuantityTerm1>,
}


// QuantityTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityTerm1 {
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Val")]
	pub val: Option<f64>,
	#[serde(rename = "TmUnit")]
	pub tm_unit: Option<String>,
}


// Reconciliation3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reconciliation3Code {
	#[serde(rename = "Reconciliation3Code")]
	pub reconciliation3_code: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReportingExemption1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingExemption1 {
	#[serde(rename = "Rsn")]
	pub rsn: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// ResetDateAndValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResetDateAndValue1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "Val")]
	pub val: Option<f64>,
}


// RiskReductionService1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskReductionService1Code {
	#[serde(rename = "RiskReductionService1Code")]
	pub risk_reduction_service1_code: String,
}


// Schedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule1 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice17Choice,
}


// Schedule10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule10 {
	#[serde(rename = "Qty")]
	pub qty: f64,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
}


// Schedule11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule11 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection106,
}


// Schedule4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule4 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice17Choice,
}


// SecuritiesTransactionPrice14Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice14Choice {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
}


// SecuritiesTransactionPrice17Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice17Choice {
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection106>,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld")]
	pub yld: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
	#[serde(rename = "PdgPric")]
	pub pdg_pric: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}


// SecuritiesTransactionPrice20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice20Choice {
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection106>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: Option<f64>,
}


// SecuritiesTransactionPrice23Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice23Choice {
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection106>,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld")]
	pub yld: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
	#[serde(rename = "Othr")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}


// SecuritiesTransactionPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val")]
	pub val: Option<f64>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
}


// SecurityIdentification41Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification41Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[serde(rename = "Bskt")]
	pub bskt: Option<CustomBasket4>,
	#[serde(rename = "Indx")]
	pub indx: Option<IndexIdentification1>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericIdentification184>,
	#[serde(rename = "IdNotAvlbl")]
	pub id_not_avlbl: Option<String>,
}


// SecurityIdentification46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification46 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[serde(rename = "PdctDesc")]
	pub pdct_desc: Option<String>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TechnicalAttributes5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalAttributes5 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "RcncltnFlg")]
	pub rcncltn_flg: Option<String>,
	#[serde(rename = "RptRctTmStmp")]
	pub rpt_rct_tm_stmp: Option<String>,
}


// TimePeriodDetails1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimePeriodDetails1 {
	#[serde(rename = "FrTm")]
	pub fr_tm: String,
	#[serde(rename = "ToTm")]
	pub to_tm: Option<String>,
}


// TradeClearing11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeClearing11 {
	#[serde(rename = "ClrOblgtn")]
	pub clr_oblgtn: Option<String>,
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared23Choice>,
	#[serde(rename = "IntraGrp")]
	pub intra_grp: Option<bool>,
}


// TradeConfirmation4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmation4Choice {
	#[serde(rename = "Confd")]
	pub confd: Option<TradeConfirmation5>,
	#[serde(rename = "NonConfd")]
	pub non_confd: Option<TradeNonConfirmation1>,
}


// TradeConfirmation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmation5 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<String>,
}


// TradeConfirmationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmationType1Code {
	#[serde(rename = "TradeConfirmationType1Code")]
	pub trade_confirmation_type1_code: String,
}


// TradeConfirmationType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmationType2Code {
	#[serde(rename = "TradeConfirmationType2Code")]
	pub trade_confirmation_type2_code: String,
}


// TradeCounterpartyRelationship1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationship1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TradeCounterpartyRelationshipRecord1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[serde(rename = "StartRltshPty")]
	pub start_rltsh_pty: String,
	#[serde(rename = "EndRltshPty")]
	pub end_rltsh_pty: String,
	#[serde(rename = "RltshTp")]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TradeCounterpartyReport20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyReport20 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Counterparty45,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Counterparty46,
	#[serde(rename = "Brkr")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "SubmitgAgt")]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ExctnAgt")]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "RltshRcrd")]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}


// TradeCounterpartyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyType1Code {
	#[serde(rename = "TradeCounterpartyType1Code")]
	pub trade_counterparty_type1_code: String,
}


// TradeData43 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData43 {
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: Vec<CounterpartySpecificData36>,
	#[serde(rename = "CmonTradData")]
	pub cmon_trad_data: CommonTradeDataReport71,
	#[serde(rename = "Lvl")]
	pub lvl: Option<String>,
	#[serde(rename = "TechAttrbts")]
	pub tech_attrbts: Option<TechnicalAttributes5>,
	#[serde(rename = "PblcDssmntnData")]
	pub pblc_dssmntn_data: Option<DisseminationData1>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeData59Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData59Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<Vec<TradeReport33Choice>>,
}


// TradeNonConfirmation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeNonConfirmation1 {
	#[serde(rename = "Tp")]
	pub tp: String,
}


// TradeReport33Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReport33Choice {
	#[serde(rename = "New")]
	pub new: Option<TradeData43>,
	#[serde(rename = "Mod")]
	pub mod_attr: Option<TradeData43>,
	#[serde(rename = "Crrctn")]
	pub crrctn: Option<TradeData43>,
	#[serde(rename = "Termntn")]
	pub termntn: Option<TradeData43>,
	#[serde(rename = "PosCmpnt")]
	pub pos_cmpnt: Option<TradeData43>,
	#[serde(rename = "ValtnUpd")]
	pub valtn_upd: Option<TradeData43>,
	#[serde(rename = "Cmprssn")]
	pub cmprssn: Option<TradeData43>,
	#[serde(rename = "Err")]
	pub err: Option<TradeData43>,
	#[serde(rename = "PortOut")]
	pub port_out: Option<TradeData43>,
	#[serde(rename = "Rvv")]
	pub rvv: Option<TradeData43>,
	#[serde(rename = "Othr")]
	pub othr: Option<TradeData43>,
}


// TradeReportHeader4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportHeader4 {
	#[serde(rename = "RptExctnDt")]
	pub rpt_exctn_dt: Option<String>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds")]
	pub nb_rcrds: f64,
	#[serde(rename = "CmptntAuthrty")]
	pub cmptnt_authrty: Option<Vec<String>>,
	#[serde(rename = "NewTradRpstryIdr")]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptgPurp")]
	pub rptg_purp: Option<Vec<String>>,
}


// TradeTransaction50 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransaction50 {
	#[serde(rename = "TxId")]
	pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "ScndryTxId")]
	pub scndry_tx_id: Option<String>,
	#[serde(rename = "PrrTxId")]
	pub prr_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[serde(rename = "SbsqntTxId")]
	pub sbsqnt_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode6Choice>,
	#[serde(rename = "RptTrckgNb")]
	pub rpt_trckg_nb: Option<String>,
	#[serde(rename = "PltfmIdr")]
	pub pltfm_idr: Option<String>,
	#[serde(rename = "MrrrOrTrggrTx")]
	pub mrrr_or_trggr_tx: Option<bool>,
	#[serde(rename = "TxPric")]
	pub tx_pric: Option<PriceData2>,
	#[serde(rename = "NtnlAmt")]
	pub ntnl_amt: Option<NotionalAmountLegs5>,
	#[serde(rename = "NtnlQty")]
	pub ntnl_qty: Option<NotionalQuantityLegs5>,
	#[serde(rename = "Qty")]
	pub qty: Option<FinancialInstrumentQuantity32Choice>,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: Option<String>,
	#[serde(rename = "ExctnTmStmp")]
	pub exctn_tm_stmp: Option<String>,
	#[serde(rename = "FctvDt")]
	pub fctv_dt: Option<String>,
	#[serde(rename = "XprtnDt")]
	pub xprtn_dt: Option<String>,
	#[serde(rename = "EarlyTermntnDt")]
	pub early_termntn_dt: Option<String>,
	#[serde(rename = "SttlmDt")]
	pub sttlm_dt: Option<Vec<String>>,
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "Cmprssn")]
	pub cmprssn: Option<bool>,
	#[serde(rename = "PstTradRskRdctnFlg")]
	pub pst_trad_rsk_rdctn_flg: Option<bool>,
	#[serde(rename = "PstTradRskRdctnEvt")]
	pub pst_trad_rsk_rdctn_evt: Option<PTRREvent2>,
	#[serde(rename = "DerivEvt")]
	pub deriv_evt: Option<DerivativeEvent6>,
	#[serde(rename = "TradConf")]
	pub trad_conf: Option<TradeConfirmation4Choice>,
	#[serde(rename = "NonStdsdTerm")]
	pub non_stdsd_term: Option<bool>,
	#[serde(rename = "TradClr")]
	pub trad_clr: Option<TradeClearing11>,
	#[serde(rename = "BlckTradElctn")]
	pub blck_trad_elctn: Option<bool>,
	#[serde(rename = "LrgNtnlOffFcltyElctn")]
	pub lrg_ntnl_off_fclty_elctn: Option<bool>,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<InterestRateLegs14>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<CurrencyExchange22>,
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<AssetClassCommodity7Choice>,
	#[serde(rename = "Optn")]
	pub optn: Option<OptionOrSwaption11>,
	#[serde(rename = "NrgySpcfcAttrbts")]
	pub nrgy_spcfc_attrbts: Option<EnergySpecificAttribute9>,
	#[serde(rename = "Cdt")]
	pub cdt: Option<CreditDerivative4>,
	#[serde(rename = "OthrPmt")]
	pub othr_pmt: Option<Vec<OtherPayment5>>,
	#[serde(rename = "Packg")]
	pub packg: Option<Package4>,
	#[serde(rename = "TradAllcnSts")]
	pub trad_allcn_sts: Option<String>,
}


// TradingCapacity7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingCapacity7Code {
	#[serde(rename = "TradingCapacity7Code")]
	pub trading_capacity7_code: String,
}


// Tranche3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tranche3 {
	#[serde(rename = "AttchmntPt")]
	pub attchmnt_pt: Option<f64>,
	#[serde(rename = "DtchmntPt")]
	pub dtchmnt_pt: Option<f64>,
}


// TrancheIndicator3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrancheIndicator3Choice {
	#[serde(rename = "Trnchd")]
	pub trnchd: Option<Tranche3>,
	#[serde(rename = "Utrnchd")]
	pub utrnchd: Option<String>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[serde(rename = "UTIIdentifier")]
	pub uti_identifier: String,
}


// UnderlyingIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingIdentification1Code {
	#[serde(rename = "UnderlyingIdentification1Code")]
	pub underlying_identification1_code: String,
}


// UniqueProductIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier1Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// UniqueProductIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier2Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification185>,
}


// UniqueTransactionIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier1Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification179>,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// UniqueTransactionIdentifier3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier3Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
	#[serde(rename = "NotAvlbl")]
	pub not_avlbl: Option<String>,
}


// UnitOfMeasure8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// ValuationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValuationType1Code {
	#[serde(rename = "ValuationType1Code")]
	pub valuation_type1_code: String,
}


// WeekDay3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WeekDay3Code {
	#[serde(rename = "WeekDay3Code")]
	pub week_day3_code: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}

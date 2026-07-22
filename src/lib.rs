use std::collections::HashMap;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

static mut rx_func_map: HashMap<&&str,&&str> = rx_functions.iter().zip(rx_rest.iter()).collect();

const rx_functions: [&str;36] = [
    "filterByProperty",
    "findActiveProducts",
    "findRelatedNDCs",
    "findRxcuiById",
    "findRxcuiByString",
    "getAllConceptsByStatus",
    "getAllConceptsByTTY",
    "getAllHistoricalNDCs",
    "getAllNDCsByStatus",
    "getAllProperties",
    "getAllRelatedInfo",
    "getApproximateMatch",
    "getDisplayTerms",
    "getDrugs",
    "getGenericProduct",
    "getIdTypes",
    "getMultiIngredBrand",
    "getNDCProperties",
    "getNDCStatus",
    "getNDCs",
    "getPropCategories",
    "getPropNames",
    "getProprietaryInformation",
    "getReformulationConcepts",
    "getRelaPaths",
    "getRelaTypes",
    "getRelatedByRelationship",
    "getRelatedByType",
    "getRxConceptProperties",
    "getRxNormName",
    "getRxNormVersion",
    "getRxProperty",
    "getRxcuiHistoryStatus",
    "getSourceTypes",
    "getSpellingSuggestions",
    "getTermTypes"
];

const rx_rest:[&str;36] = [  
    "/rxcui/rxcui/filter",
    "/rxcui/rxcui/active",
    "/relatedndc",
    "/rxcui?",
    "/rxcui?",
    "/allstatus",
    "/allconcepts",
    "/rxcui/rxcui/allhistoricalndcs",
    "/allNDCstatus",
    "/rxcui/rxcui/allProperties",
    "/rxcui/rxcui/allrelated",
    "/approximateTerm",
    "/displaynames",
    "/drugs",
    "/rxcui/rxcui/generic",
    "/idtypes",
    "/brands",
    "/ndcproperties",
    "/ndcstatus",
    "/rxcui/rxcui/ndcs",
    "/propCategories",
    "/propnames",
    "/rxcui/rxcui/proprietary",
    "/reformulationConcepts",
    "/relapaths",
    "/relatypes",
    "/rxcui/rxcui/related?",
    "/rxcui/rxcui/related?",
    "/rxcui/rxcui/properties",
    "/rxcui/rxcui",
    "/version",
    "/rxcui/rxcui/property",
    "/rxcui/rxcui/historystatus",
    "/sourcetypes",
    "/spellingsuggestions",
    "/termtypes"
];

const universal_parameters = ["format"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

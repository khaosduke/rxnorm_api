use std::collections::HashMap;
use std::sync::LazyLock;


pub const RXNORM_DOMAIN:&str = "https://rxnav.nlm.nih.gov/";
pub const FORMAT_PARAMETER:&str = "format";
pub const RXCUI_PARAMETER:&str = "rxcui";

/// Allowed values for one RxNorm API parameter.
///
/// These vectors are intentionally empty for now. They can later be
/// populated from the value menus and constraints in the RxNorm docs.
pub type ParameterValues = Vec<&'static str>;

/// Parameter name -> allowed values.
pub type ParameterMap = HashMap<&'static str, ParameterValues>;

/// REST resource path and its documented parameters.
pub type FunctionDefinition = (&'static str, ParameterMap);

/// RxNorm API function name -> (REST resource path, parameter definitions).
pub static RXNORM_FUNCTIONS: LazyLock<HashMap<&'static str, FunctionDefinition>> =
    LazyLock::new(|| {
        HashMap::from([
            ("filterByProperty", (
                "/REST/rxcui/{rxcui}/filter",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("propName", Vec::new()),
                    ("propValues", Vec::new()),
                ]),
            )),
            ("findActiveProducts", (
                "/REST/rxcui/{rxcui}/active",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("results", Vec::new()),
                ]),
            )),
            ("findRelatedNDCs", (
                "/REST/relatedndc",
                HashMap::from([
                    ("ndc", Vec::new()),
                    ("relation", Vec::new()),
                    ("ndcstatus", Vec::new()),
                ]),
            )),
            ("findRxcuiById", (
                "/REST/rxcui",
                HashMap::from([
                    ("idtype", Vec::new()),
                    ("id", Vec::new()),
                    ("allsrc", Vec::new()),
                ]),
            )),
            ("findRxcuiByString", (
                "/REST/rxcui",
                HashMap::from([
                    ("name", Vec::new()),
                    ("search", Vec::new()),
                    ("allsrc", Vec::new()),
                    ("srclist", Vec::new()),
                ]),
            )),
            ("getAllConceptsByStatus", (
                "/REST/allstatus",
                HashMap::from([
                    ("status", Vec::new()),
                ]),
            )),
            ("getAllConceptsByTTY", (
                "/REST/allconcepts",
                HashMap::from([  
                    ("tty", Vec::new()),
                ]),
            )),
            ("getAllHistoricalNDCs", (
                "/REST/rxcui/{rxcui}/allhistoricalndcs",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("history", Vec::new()),
                ]),
            )),
            ("getAllNDCsByStatus", (
                "/REST/allndcstatus",
                HashMap::from([
                    ("status", Vec::new()),
                ]),
            )),
            ("getAllProperties", (
                "/REST/rxcui/{rxcui}/allProperties",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("prop", Vec::new()),
                ]),
            )),
            ("getAllRelatedInfo", (
                "/REST/rxcui/{rxcui}/allrelated",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("expand", Vec::new()),
                ]),
            )),
            ("getApproximateMatch", (
                "/REST/approximateTerm",
                HashMap::from([
                    ("term", Vec::new()),
                    ("maxEntries", Vec::new()),
                    ("option", Vec::new()),
                ]),
            )),
            ("getDisplayTerms", (
                "/REST/displaynames",
                HashMap::from([
                    
                ]),
            )),
            ("getDrugs", (
                "/REST/drugs",
                HashMap::from([
                    ("name", Vec::new()),
                    ("tty", Vec::new()),
                    ("expand", Vec::new()),
                ]),
            )),
            ("getGenericProduct", (
                "/REST/rxcui/{rxcui}/generic",
                HashMap::from([
                    ("rxcui", Vec::new()),
                ]),
            )),
            ("getIdTypes", (
                "/REST/idtypes",
                HashMap::from([
                ]),
            )),
            ("getMultiIngredBrand", (
                "/REST/brands",
                HashMap::from([
                    ("ingredientids", Vec::new()),
                    ("status", Vec::new()),
                ]),
            )),
            ("getNDCProperties", (
                "/REST/ndcproperties",
                HashMap::from([
                    ("id", Vec::new()),
                ]),
            )),
            ("getNDCStatus", (
                "/REST/ndcstatus",
                HashMap::from([
                    ("ndc", Vec::new()),
                ]),
            )),
            ("getNDCs", (
                "/REST/rxcui/{rxcui}/ndcs",
                HashMap::from([
                    ("rxcui", Vec::new()),
                ]),
            )),
            ("getPropCategories", (
                "/REST/propCategories",
                HashMap::from([
                ]),
            )),
            ("getPropNames", (
                "/REST/propnames",
                HashMap::from([
                ]),
            )),
            ("getProprietaryInformation", (
                "/REST/rxcui/{rxcui}/proprietary",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("type", Vec::new()),
                ]),
            )),
            ("getReformulationConcepts", (
                "/REST/rxcui/{rxcui}/reformulation",
                HashMap::from([
                    ("rxcui", Vec::new()),
                ]),
            )),
            ("getRelaPaths", (
                "/REST/relapaths",
                HashMap::from([
                    ("start", Vec::new()),
                    ("finish", Vec::new()),
                ]),
            )),
            ("getRelaTypes", (
                "/REST/relatypes",
                HashMap::from([
                ]),
            )),
            ("getRelatedByRelationship", (
                "/REST/rxcui/{rxcui}/related",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("rela", Vec::new()),
                    ("expand", Vec::new()),
                ]),
            )),
            ("getRelatedByType", (
                "/REST/rxcui/{rxcui}/related",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("tty", Vec::new()),
                    ("expand", Vec::new()),
                ]),
            )),
            ("getRxConceptProperties", (
                "/REST/rxcui/{rxcui}/properties",
                HashMap::from([
                    ("rxcui", Vec::new()),
                ]),
            )),
            ("getRxNormName", (
                "/REST/rxcui/{rxcui}",
                HashMap::from([
                    ("rxcui", Vec::new()),
                ]),
            )),
            ("getRxNormVersion", (
                "/REST/version",
                HashMap::from([
                ]),
            )),
            ("getRxProperty", (
                "/REST/rxcui/{rxcui}/property",
                HashMap::from([
                    ("rxcui", Vec::new()),
                    ("propName", Vec::new()),
                ]),
            )),
            ("getRxcuiHistoryStatus", (
                "/REST/rxcui/{rxcui}/historystatus",
                HashMap::from([
                    ("rxcui", Vec::new()),
                ]),
            )),
            ("getSourceTypes", (
                "/REST/sourcetypes",
                HashMap::from([
                ]),
            )),
            ("getSpellingSuggestions", (
                "/REST/spellingsuggestions",
                HashMap::from([
                    ("name", Vec::new()),
                ]),
            )),
            ("getTermTypes", (
                "/REST/termtypes",
                HashMap::from([
                ]),
            )),
        ])
    });

//const rx_functions: [&str;36] = [
//    "filterByProperty",
//    "findActiveProducts",
//    "findRelatedNDCs",
//    "findRxcuiById",
//    "findRxcuiByString",
//    "getAllConceptsByStatus",
//    "getAllConceptsByTTY",
//    "getAllHistoricalNDCs",
//    "getAllNDCsByStatus",
//    "getAllProperties",
//    "getAllRelatedInfo",
//    "getApproximateMatch",
//    "getDisplayTerms",
//    "getDrugs",
//    "getGenericProduct",
//    "getIdTypes",
//    "getMultiIngredBrand",
//    "getNDCProperties",
//    "getNDCStatus",
//    "getNDCs",
//    "getPropCategories",
//    "getPropNames",
//    "getProprietaryInformation",
//    "getReformulationConcepts",
//    "getRelaPaths",
//    "getRelaTypes",
//    "getRelatedByRelationship",
//    "getRelatedByType",
//    "getRxConceptProperties",
//    "getRxNormName",
//    "getRxNormVersion",
//    "getRxProperty",
//    "getRxcuiHistoryStatus",
//    "getSourceTypes",
//    "getSpellingSuggestions",
//    "getTermTypes"
//];

//const rx_rest:[&str;36] = [  
//    "/rxcui/rxcui/filter",
//    "/rxcui/rxcui/active",
//    "/relatedndc",
//    "/rxcui?",
//    "/rxcui?",
//    "/allstatus",
//    "/allconcepts",
//    "/rxcui/rxcui/allhistoricalndcs",
//    "/allNDCstatus",
//    "/rxcui/rxcui/allProperties",
//    "/rxcui/rxcui/allrelated",
//    "/approximateTerm",
//    "/displaynames",
//    "/drugs",
//    "/rxcui/rxcui/generic",
//    "/idtypes",
//    "/brands",
//    "/ndcproperties",
//    "/ndcstatus",
//    "/rxcui/rxcui/ndcs",
//    "/propCategories",
//    "/propnames",
//    "/rxcui/rxcui/proprietary",
//    "/reformulationConcepts",
//    "/relapaths",
//    "/relatypes",
//    "/rxcui/rxcui/related?",
//    "/rxcui/rxcui/related?",
//    "/rxcui/rxcui/properties",
//    "/rxcui/rxcui",
//    "/version",
//    "/rxcui/rxcui/property",
//    "/rxcui/rxcui/historystatus",
//    "/sourcetypes",
//    "/spellingsuggestions",
// //   "/termtypes"
//];

//const universal_parameters = ["format"];



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_every_documented_rxnorm_function() {
        assert_eq!(RXNORM_FUNCTIONS.len(), 36);
    }

    #[test]
    fn filter_by_property_has_expected_shape() {
        let (path, parameters) = RXNORM_FUNCTIONS
            .get("filterByProperty")
            .expect("filterByProperty should exist");

        assert_eq!(*path, "/REST/rxcui/{rxcui}/filter");
        assert!(parameters.contains_key("rxcui"));
        assert!(parameters.contains_key("format"));
        assert!(parameters.contains_key("propName"));
        assert!(parameters.contains_key("propValues"));
    }
}

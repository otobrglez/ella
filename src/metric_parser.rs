use std::error::Error;

use crate::family::Family;
use crate::prom_to_json::parse as prom_to_json_parser;

pub struct MetricParser {}

type ParserResult<T> = Result<T, Box<dyn Error>>;

impl MetricParser {
    fn prom_to_json(prom: String) -> ParserResult<String> {
        prom_to_json_parser(prom)
    }

    pub fn prom_to_families(prom: String) -> ParserResult<Vec<Family>> {
        Self::prom_to_json(prom).and_then(|json| match serde_json::from_str(&json) {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::from(e)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::metric_parser::MetricParser;
    use std::fs;

    #[test]
    fn test_small() {
        let raw_sample =
            fs::read_to_string("./tests/data/example.prom").expect("Failed to read file");

        let families_one =
            &MetricParser::prom_to_families(raw_sample).expect("Parsing to struct failed.");

        dbg!(families_one);
        assert_eq!(families_one.len(), 2);
    }

    #[test]
    fn test_bigger() {
        let bigger_sample =
            fs::read_to_string("./tests/data/metric-huge.prom").expect("Failed to read huge file");

        let families_two =
            &MetricParser::prom_to_families(bigger_sample).expect("Parsing to struct failed.");

        dbg!("{:?}", families_two);

        assert_eq!(families_two.len(), 148);
    }

    #[test]
    fn test_node_exporter() {
        let raw_sample =
            fs::read_to_string("./tests/data/node_metrics.prom").expect("Failed to read file");

        let node_families =
            &MetricParser::prom_to_families(raw_sample).expect("Parsing to struct failed.");

        assert_eq!(node_families.len(), 284);
    }
}

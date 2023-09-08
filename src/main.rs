#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod models;

#[derive(Parser)]
#[grammar = "dbml.pest"]
pub struct DbmlParser;

fn main() {
    let dbml_str = r#"
    // This is a comment
    Project Test {
        database_type: 'something'
    }

    Table User {
        id int [pk]
        name varchar
    }

    Table Order {
        id int [pk]
        user_id int // ref: > User.id
    }

    Table patient {
        id string [pk]
        first_name string [not null]
        last_name string
        address string
        city string
        state string
        country string
        zip string
        dob string
        sex sex
        phone string
        mrn string
        orders order[]
    }

    Enum type {
        "Text"
        "Text area"
        "Number"
        "Decimal"
        "Date"
        "Time"
        "Checkbox"
        "Single-selection picklist"
        "Multi-selection picklist"
    }
    "#;

    let mut file = match DbmlParser::parse(Rule::File, dbml_str) {
        Ok(parsed) => parsed,
        Err(e) => panic!("{}", e),
    };

    for pair in file.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::Table => {
                let table = models::Table::parse(pair);
                println!("table: {:#?}", table);
            }
            Rule::Project => {
                // pending
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parse(rule: Rule, dbml_str: &str) {
        let val = match DbmlParser::parse(rule, dbml_str) {
            Ok(mut parsed) => parsed.next().unwrap(),
            Err(e) => panic!("{}", e),
        };

        assert_eq!(val.as_str(), dbml_str);
    }

    #[test]
    fn test_parse_name() {
        assert_parse(Rule::Name, "database_type");
    }

    #[test]
    fn test_parse_option() {
        assert_parse(Rule::Option, "database_type: 'value'");
    }

    #[test]
    fn test_parse_project() {
        assert_parse(
            Rule::Project,
            "Project Test {\n      database_type: 'something' }",
        );
    }

    #[test]
    fn test_parse_setting() {
        assert!(DbmlParser::parse(Rule::SettingKey, "pk").is_ok());
        assert!(DbmlParser::parse(Rule::Setting, "pk").is_ok());
    }

    #[test]
    fn test_parse_setting_pairs() {
        assert!(DbmlParser::parse(Rule::SettingsPairs, "pk").is_ok());
        assert!(DbmlParser::parse(Rule::SettingsPairs, "ref: > user.id").is_ok());
        assert!(DbmlParser::parse(Rule::SettingsPairs, "pk, not null").is_ok());
    }

    #[test]
    fn test_parse_settings() {
        assert!(DbmlParser::parse(Rule::Settings, "[pk]").is_ok());
        assert!(DbmlParser::parse(Rule::Settings, "[pk, ref: > user.id, not null]").is_ok());
    }

    #[test]
    fn test_parse_table_item() {
        assert!(DbmlParser::parse(Rule::TableItem, "orders order[]").is_ok());
        assert!(DbmlParser::parse(Rule::TableItems, "orders order[]").is_ok());
    }

    #[test]
    fn test_parse_column_type() {
        assert!(DbmlParser::parse(Rule::ColumnType, "order[]").is_ok());
    }

    #[test]
    fn test_parse_column() {
        assert!(DbmlParser::parse(Rule::Column, "id string [pk]").is_ok());
        assert!(DbmlParser::parse(Rule::Column, "orders order[]").is_ok());
    }

    #[test]
    fn test_parse_table() {
        assert!(DbmlParser::parse(Rule::Table, "Table table { orders order[] }").is_ok());
        assert!(DbmlParser::parse(Rule::Table, "Table table { orders order[] [pk] }").is_ok());
    }
}

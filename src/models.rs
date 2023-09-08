use pest::iterators::Pair;

use crate::Rule;

#[derive(Default, Debug)]
pub struct Table {
    pub name: String,
    pub alias: Option<String>,
    pub columns: Vec<Column>,
}

#[derive(Default, Debug)]
pub struct Column {
    pub name: String,
    pub column_type: String,
    pub settings: Vec<String>,
}

impl Table {
    pub fn parse(pair: Pair<Rule>) -> Self {
        let mut table = Table::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::TableName => {
                    table.name = pair.as_str().to_string();
                }
                Rule::TableItems => {
                    for table_item in pair.into_inner() {
                        if matches!(table_item.as_rule(), Rule::TableItem) {
                            let table_item = table_item.into_inner().next().unwrap();
                            if matches!(table_item.as_rule(), Rule::Column) {
                                table.columns.push(Column::parse(table_item));
                                continue;
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        table
    }
}

impl Column {
    pub fn parse(pair: Pair<Rule>) -> Self {
        let mut column = Column::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ColumnName => {
                    column.name = pair.as_str().to_string();
                }
                Rule::ColumnType => {
                    column.column_type = pair.as_str().to_string();
                }
                Rule::Settings => {
                    for setting in pair.into_inner() {
                        match setting.as_rule() {
                            Rule::SettingsPairs => {
                                column.settings.push(setting.as_str().to_string());
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        column
    }
}

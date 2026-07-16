use std::fs;
use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::backend::{GenerateConfig, MiyagiBackend};
use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct DatasetConfig {
    pub question_field: String,
    pub answer_field: String,
    pub prompt_template: String,
    pub answer_regex: Regex,
    pub gold_regex: Regex,
    pub limit: Option<usize>,
    pub generation: GenerateConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasetCaseResult {
    pub index: usize,
    pub question: String,
    pub expected: String,
    pub predicted: Option<String>,
    pub correct: bool,
    pub response: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DatasetReport {
    pub correct: usize,
    pub total: usize,
    pub cases: Vec<DatasetCaseResult>,
}

pub fn load_records(path: impl AsRef<Path>) -> Result<Vec<Value>> {
    let text = fs::read_to_string(path)?;
    if text.trim_start().starts_with('[') {
        return Ok(serde_json::from_str(&text)?);
    }
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Ok(serde_json::from_str(line)?))
        .collect()
}

pub fn evaluate_dataset<B: MiyagiBackend>(
    backend: &mut B,
    records: &[Value],
    config: &DatasetConfig,
) -> Result<DatasetReport> {
    let mut report = DatasetReport::default();
    let limit = config.limit.unwrap_or(records.len()).min(records.len());
    for (index, record) in records.iter().take(limit).enumerate() {
        let question = string_field(record, &config.question_field, index)?;
        let raw_gold = string_field(record, &config.answer_field, index)?;
        let expected =
            capture(&config.gold_regex, raw_gold).ok_or_else(|| Error::InvalidDatasetRecord {
                index,
                reason: "gold answer regex did not match".to_owned(),
            })?;
        let prompt = config.prompt_template.replace("{question}", question);
        let response = backend.generate(&prompt, &config.generation)?;
        let predicted = capture(&config.answer_regex, &response);
        let correct = predicted.as_deref() == Some(expected.as_str());
        report.total += 1;
        report.correct += usize::from(correct);
        report.cases.push(DatasetCaseResult {
            index,
            question: question.to_owned(),
            expected,
            predicted,
            correct,
            response,
        });
    }
    Ok(report)
}

fn string_field<'a>(record: &'a Value, field: &str, index: usize) -> Result<&'a str> {
    record
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| Error::InvalidDatasetRecord {
            index,
            reason: format!("field {field:?} is missing or is not a string"),
        })
}

fn capture(regex: &Regex, text: &str) -> Option<String> {
    regex
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|capture| capture.as_str().replace(',', ""))
}

//! Print format variants
use arrow::csv::writer::WriterBuilder;
use arrow::json::{ArrayWriter, LineDelimitedWriter};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::pretty;
use datafusion::error::{DataFusionError, Result};
use std::str::FromStr;

/// Allow records to be printed in different formats
#[derive(Debug, PartialEq, Eq, clap::ArgEnum, Clone)]
pub enum PrintFormat {
    Csv,
    Tsv,
    Table,
    Json,
    NdJson,
}

impl FromStr for PrintFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        clap::ArgEnum::from_str(s, true)
    }
}

macro_rules! batches_to_json {
    ($WRITER: ident, $batches: expr) => {{
        let mut bytes = vec![];
        {
            let mut writer = $WRITER::new(&mut bytes);
            writer.write_batches($batches)?;
            writer.finish()?;
        }
        String::from_utf8(bytes).map_err(|e| DataFusionError::Execution(e.to_string()))?
    }};
}

fn print_batches_with_sep(batches: &[RecordBatch], delimiter: u8) -> Result<String> {
    let mut bytes = vec![];
    {
        let builder = WriterBuilder::new()
            .has_headers(true)
            .with_delimiter(delimiter);
        let mut writer = builder.build(&mut bytes);
        for batch in batches {
            writer.write(batch)?;
        }
    }
    let formatted = String::from_utf8(bytes)
        .map_err(|e| DataFusionError::Execution(e.to_string()))?;
    Ok(formatted)
}

impl PrintFormat {
    /// print the batches to stdout using the specified format
    pub fn print_batches(&self, batches: &[RecordBatch]) -> Result<()> {
        match self {
            Self::Csv => println!("{}", print_batches_with_sep(batches, b',')?),
            Self::Tsv => println!("{}", print_batches_with_sep(batches, b'\t')?),
            Self::Table => pretty::print_batches(batches)?,
            Self::Json => println!("{}", batches_to_json!(ArrayWriter, batches)),
            Self::NdJson => {
                println!("{}", batches_to_json!(LineDelimitedWriter, batches))
            }
        }
        Ok(())
    }
}

use crate::print_format::PrintFormat;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::error::Result;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct PrintOptions {
    pub format: PrintFormat,
    pub quiet: bool,
}

pub fn print_timing_info(row_count: usize, now: Instant) {
    println!(
        "{} {} in set. Query took {:.3} seconds.",
        row_count,
        if row_count == 1 { "row" } else { "rows" },
        now.elapsed().as_secs_f64()
    );
}

impl PrintOptions {
    /// print the batches to stdout using the specified format
    pub fn print_batches(&self, batches: &[RecordBatch], now: Instant) -> Result<()> {
        if batches.is_empty() {
            if !self.quiet {
                print_timing_info(0, now);
            }
        } else {
            self.format.print_batches(batches)?;
            if !self.quiet {
                let row_count: usize = batches.iter().map(|b| b.num_rows()).sum();
                print_timing_info(row_count, now);
            }
        }
        Ok(())
    }
}

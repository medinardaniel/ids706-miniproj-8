use polars::prelude::*;
use std::error::Error;
use std::time::Instant;
use sys_info::mem_info;
use std::fs::File;

fn calculate() -> Result<DataFrame, PolarsError> {
    // Open the file first
    let file = File::open("data/data.csv")?;

    // Read the CSV data using CsvReader
    let df = CsvReader::new(file)
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    Ok(df)
}

fn descriptive_statistics(df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let column_names = df.get_column_names();
    let mut result_series: Vec<Series> = vec![];

    for col_name in column_names {
        let col = df.column(col_name).map_err(|_| format!("Column {} not found", col_name))?;

        if let Ok(float_col) = col.f64() {
            let mean_val = float_col.mean().unwrap_or_default();
            let mean_series = Series::new(&format!("{}_mean", col_name), vec![mean_val]);
            result_series.push(mean_series);

            let min_val = float_col.min().unwrap_or_default();
            let min_series = Series::new(&format!("{}_min", col_name), vec![min_val]);
            result_series.push(min_series);

            let max_val = float_col.max().unwrap_or_default();
            let max_series = Series::new(&format!("{}_max", col_name), vec![max_val]);
            result_series.push(max_series);
            
            let count_val = col.len() as f64; 
            let count_series = Series::new(&format!("{}_count", col_name), vec![count_val]);
            result_series.push(count_series);
        }
    }

    let result_df = DataFrame::new(result_series)?;
    Ok(result_df)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the initial memory usage
    let initial_memory = mem_info().unwrap().avail;

    // Start the timer
    let start_time = Instant::now();

    let df = calculate()?;
    println!("Original DataFrame:\n{}", df);

    let stats = descriptive_statistics(&df)?;
    println!("\nDescriptive Statistics:\n{}", stats);

    // Calculate runtime and memory usage
    let duration = start_time.elapsed();
    let final_memory = mem_info().unwrap().avail;
    let memory_usage = initial_memory as f64 - final_memory as f64;

    println!("\nRuntime: {:.2?}", duration);
    println!("Memory Usage: {:.2} MB", memory_usage / 1024.0);

    Ok(())
}

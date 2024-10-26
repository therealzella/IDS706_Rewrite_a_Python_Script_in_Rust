use csv::ReaderBuilder;
use ndarray::{Array1, Array2};
use ndarray_stats::SummaryStatisticsExt;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the 'cereal.csv' dataset
    let file_path = "../cereal.csv";
    let df = load_csv(file_path)?;

    // Generate summary statistics
    let summary_stats = df.mean_axis(ndarray::Axis(0)).unwrap();
    println!("Summary statistics:\n{:?}", summary_stats);

    // Save summary statistics to a CSV file
    save_summary_to_csv("summary_statistics.csv", &summary_stats)?;

    // Create a histogram of the 'calories' column
    let calories_column = df.column(3).to_owned(); // Assuming 'calories' is the 4th column (index 3)
    plot_histogram(&calories_column, "calories_histogram.png")?;

    println!("Summary statistics and calories histogram have been generated.");
    Ok(())
}

fn load_csv(file_path: &str) -> Result<Array2<f64>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut data: Vec<f64> = Vec::new();
    let mut rows = 0;
    let mut columns = None;

    for result in reader.records() {
        let record = result?;
        let mut row_data = Vec::new();

        for field in record.iter() {
            match field.parse::<f64>() {
                Ok(value) => row_data.push(value),
                Err(_) => {
                    println!("Warning: Invalid data skipped: {}", field);
                }
            }
        }

        // Ensure each row has the same number of columns
        if let Some(col_count) = columns {
            if row_data.len() == col_count {
                data.extend(row_data);
                rows += 1;
            } else {
                println!("Warning: Skipping row with mismatched column count");
            }
        } else if !row_data.is_empty() {
            columns = Some(row_data.len());
            data.extend(row_data);
            rows += 1;
        }
    }

    // Unwrap columns safely, since we checked for empty data above
    let col_count = columns.ok_or("No valid data found")?;
    let array = Array2::from_shape_vec((rows, col_count), data)?;
    Ok(array)
}


fn save_summary_to_csv(file_path: &str, summary_stats: &Array1<f64>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;
    wtr.write_record(summary_stats.iter().map(|x| x.to_string()))?;
    wtr.flush()?;
    Ok(())
}

fn plot_histogram(calories: &Array1<f64>, output_file: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Distribution of Calories", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0i32..300i32, 0i32..20i32)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(calories.iter().map(|x| (*x as i32, 1))),
    )?;
    root.present()?;
    Ok(())
}




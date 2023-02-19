use plotly::common::*;
use plotly::{ImageFormat, Plot, Scatter};
use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    // read csv file to polar dataset
    //

    let df = readcsv("Stocks.csv").unwrap();
    //let dataset = df.lazy().clone();

    // from DataFrame to Vec<f64>
    // use df2 to fill nulls values
    // first step get series
    let y2s = df
        .column("XRX")
        .unwrap()
        .fill_null(FillNullStrategy::Forward(None))
        .unwrap();
    let y3s = df
        .column("IBM")
        .unwrap()
        .fill_null(FillNullStrategy::Forward(None))
        .unwrap();
    let y4s = df
        .column("AAPL")
        .unwrap()
        .fill_null(FillNullStrategy::Forward(None))
        .unwrap();
    let t4s = df
        .column("Day")
        .unwrap()
        .fill_null(FillNullStrategy::Forward(None))
        .unwrap();
    // Second Step from polars::Serie to Vector float64
    let y2: Vec<f64> = y2s.f64()?.into_no_null_iter().collect();
    let y3: Vec<f64> = y3s.f64()?.into_no_null_iter().collect();
    let y4: Vec<f64> = y4s.f64()?.into_no_null_iter().collect();
    let t: Vec<f64> = t4s.f64()?.into_no_null_iter().collect();

    // get x values
    //let n: usize = y4.len();
    //let t: Vec<f64> = Array::linspace(0., 120.0, n).into_raw_vec();
    //data watchdog
    //println!("t : {:?}", t);
    //println!("y4 : {:?}", y4);
    //simple_scatter_plot(t, y4);
    line_and_scatter_plots(t, y2, y3, y4);
    Ok(())
}

fn readcsv(file: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(file)?.has_header(true).finish()
}

// fn simple_scatter_plot(t: Vec<f64>, y: Vec<f64>) {
//     let trace = Scatter::new(t, y).mode(Mode::Markers);
//     let mut plot = Plot::new();
//     plot.add_trace(trace);

//     plot.show();
// }

fn line_and_scatter_plots(x: Vec<f64>, y0: Vec<f64>, y1: Vec<f64>, y2: Vec<f64>) {
    let trace1 = Scatter::new(x.clone(), y0)
        .mode(Mode::Markers)
        .name("Xerox");
    let trace2 = Scatter::new(x.clone(), y1)
        .mode(Mode::LinesMarkers)
        .name("IBM");
    let trace3 = Scatter::new(x, y2).mode(Mode::Lines).name("Apple");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    //plot.show();

    let filename = "salida";
    let image_format = ImageFormat::PNG;
    let width = 800;
    let height = 600;
    let scale = 1.0;

    // The image will be saved to format!("{filename}.{image_format}") relative to
    // the current working directory.
    //plot.write_image(filename, image_format, width, height, scale);
    plot.write_image(filename, image_format, width, height, scale);
}

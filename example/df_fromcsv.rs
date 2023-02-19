use polars::prelude::*;
use plotpy::{Curve, Plot, StrError};

fn main() {
    // read csv file to polar dataset
    //  
    // let lf2 = LazyFrame::scan_parquet("women_usa.csv", Default::default())?
    // .select([col("Biology"), col("Agriculture")]);
    let lf2 = example();
    let dataset = lf2.unwrap().clone();
    // let df2 = dataset.select([&col("Agriculture"), &col("Biology")]).collect();
    println!("{:?} {:?}", dataset[0], dataset[1]);
    let okis = ploting(&dataset).unwrap();
    match okis {
        some(Ok()) => println("ready"),
        _ => println("error")
    }
}


fn example() -> PolarsResult<DataFrame> {
    CsvReader::from_path("women_usa.csv")?
            .has_header(true)
            .finish()
}

fn ploting(dataset: &polars::prelude::DataFrame) {
// generate (x,y) points
    let x = dataset[0];
    let y1 = dataset[1];
    let y2 = dataset[2];
    let y3 = dataset[3];
    let y4 = dataset[4];

    // configure and draw curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let mut curve3 = Curve::new();
    let mut curve4 = Curve::new();
    curve1.set_label("y = x");
    curve2.set_label("y = |x|").set_line_color("#cd0000");
    curve3.set_label("y = exp(1+x)-1").set_line_color("#e79955");
    curve4.set_label("y = sqrt(1+x)").set_line_color("#b566ab");
    curve1.draw(&x, &y1);
    curve2.draw(&x, &y2);
    curve3.draw(&x, &y3);
    curve4.draw(&x, &y4);
    println("y1: {:?}", &y1);
    // configure plot
    let mut plot = Plot::new();
    plot.set_super_title("FOUR CURVES").set_gaps(0.35, 0.5);

    // add curve to subplot
    plot.set_subplot(2, 2, 1)
        .set_title("first")
        .add(&curve1)
        .grid_labels_legend("x", "y")
        .set_equal_axes(true);

    // add curve to subplot
    plot.set_subplot(2, 2, 2)
        .set_title("second")
        .add(&curve2)
        .grid_labels_legend("x", "y");

    // add curve to subplot
    plot.set_subplot(2, 2, 3)
        .set_title("third")
        .add(&curve3)
        .set_range(-1.0, 1.0, 0.0, 6.0)
        .grid_labels_legend("x", "y");

    // add curve to subplot
    plot.set_subplot(2, 2, 4)
        .set_title("fourth")
        .add(&curve4)
        .grid_labels_legend("x", "y");

    // save figure
    plot.save("/tmp/plotpy/doc_tests/doc_plot.svg");
}

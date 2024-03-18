use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn main() {
    let mut table = Table::new();
    table.add_pt(Point(0.0, 0.0));
    table.add_pt(Point(0.523598775598, 0.5)); // pi/6
    table.add_pt(Point(0.785398163397, 0.707106781186)); // pi/4
    table.add_pt(Point(1.047197551196, 0.866025403784)); // pi/3
    table.add_pt(Point(1.570796326794, 1.0)); // pi/2
    table.add_pt(Point(2.094395102393, 0.866025403784)); // 2pi/3
    table.add_pt(Point(2.356194490192, 0.707106781186)); // 3pi/4
    table.add_pt(Point(2.617993877991, 0.5));
    table.add_pt(Point(3.141592653589, 0.0));
    table.add_pt(Point(3.141592653589 * 2.0, 0.0));

    let mut interp_x: Vec<f64> = vec![];
    let mut interp_y: Vec<f64> = vec![];

    // Increment the xval we use every time around, and print points to terminal.
    let mut iteration = 0;
    loop {
        let xval = 0.0 + 0.01 * iteration as f64;
        if xval > 3.141592653589 * 2.0 {
            break;
        }
        let res = lagrangian_interpolation(&table, xval);

        interp_x.push(xval);
        interp_y.push(res);
        iteration += 1;
    }

    let trace = Scatter::new(interp_x, interp_y).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

/// This funtion interpolates a function's value
/// when passed a table of precomputed points and the x-value at
/// which to evaluate it.
///
/// ## Example
///
/// Interpolate y-value at x = 1.0.
///
/// ```rust
/// let table = Table::new();
/// table.add_pt(Point(0.0, 0.0));
/// table.add_pt(Point(0.134, 0.118));
/// table.add_pt(Point(1.32, 0.97));
/// table.add_pt(Point(1.55, 1.036));
///
/// let res = lagrangian_interpolation(table, 1.0);
/// ```
///
/// ## Panics
///
/// If a value in the table does not exist or cannot
/// be read, the `.unwrap()` in the second loop below
/// will panic.
fn lagrangian_interpolation(table: &Table, xval: f64) -> f64 {
    let n = table.len();
    let mut sum = 0.0;

    for i in 0..n {
        if let Some(Point(x_i, y_i)) = table.pt_at(i) {
            // Evaluate the ith Lagrange poly'n. at xval and add it to sum.
            let mut product = 1.0;

            // Iterator grabs all but i.
            for j in (0..n)
                .enumerate()
                .filter(|&(pos, _)| (pos != i))
                .map(|(_, e)| e)
            {
                let x_j = table.pt_at(j).unwrap().0;
                product *= (xval - x_j) / (x_i - x_j);
            } // product is now the value of L_i(xval).

            sum += y_i * product;
        } else {
            break;
        }
    }

    sum
}

/// Just a Vec<Point> in a handy wrapper.
struct Table {
    table: Vec<Point>,
}

impl Table {
    fn new() -> Self {
        Self { table: vec![] }
    }

    /// Retrieve the length of the table.
    fn len(&self) -> usize {
        self.table.len()
    }

    /// Add a point.
    fn add_pt(&mut self, point: Point) {
        self.table.push(point);
    }

    /// Retrieve the ith point from the table.
    fn pt_at(&self, index: usize) -> Option<&Point> {
        self.table.get(index)
    }
}

struct Point(f64, f64);

impl Point {
    fn new() -> Self {
        Self(0.0, 0.0)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self(0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_interpolation_of_sin() {
        //! From the book linked in my blog post.

        let mut table = Table::new();
        table.add_pt(Point(0.0, 0.0));
        table.add_pt(Point(30.0, 0.5));
        table.add_pt(Point(60.0, 0.86603));
        table.add_pt(Point(90.0, 1.0));

        let res = lagrangian_interpolation(&table, 51.0);
        assert!((res - 0.776).abs() <= 1e-3)
    }
}

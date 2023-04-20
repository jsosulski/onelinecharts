use ansi_term::{Colour, Colour::Red};
use clap::Parser as ClapParser;
use core::fmt;

#[derive(ClapParser)]
#[clap(version = "0.1.0", author = "Jan Sosulski <mail@jan-sosulski.de>")]
struct Opts {
    /// Data to plot.
    data: Vec<f64>,
    /// Minimum value in the graph
    #[clap(long, default_value = "0.0")]
    min: f64,
    /// Maximum value in the graph
    #[clap(long, default_value = "100.0")]
    max: f64,
    /// Which chart type should be used? Currently only 'bar' is available.
    #[clap(long, default_value = "bar")]
    charttype: String,
    /// Is the output for tmux?
    #[clap(long)]
    tmux: bool,
    /// Optional yaxis+ylabel to show
    #[clap(long)]
    ylabel: Option<String>,
}

struct BarChartProducer {
    symbols: Vec<String>,
    min: f64,
    max: f64,
    out_of_range: Colour,
    tmux_out: bool,
}

impl BarChartProducer {
    pub fn new(min: f64, max: f64, tmux_out: bool) -> Self {
        BarChartProducer {
            // create symbol Struct -> has e.g. mapping attribute / function
            // Currently using arbitrary ö character for splitting, should
            // find a better solution
            symbols: "▁ö▂ö▃ö▄ö▅ö▆ö▇ö█".split('ö').map(String::from).collect(),
            min,
            max,
            out_of_range: Red,
            tmux_out,
        }
    }
    pub fn map(&self, input: &f64) -> Option<String> {
        let ratio = (input - self.min) / (self.max - self.min);
        let clamped_ratio = ratio.clamp(0., 1.);

        let idx = clamped_ratio * (self.symbols.len() - 1) as f64;
        let idx = idx.round() as usize;
        // println!(
        //     "Min: {}, Max: {}, Ratio: {}, Idx: {}",
        //     self.min, self.max, ratio, idx
        // );
        let out = self.symbols.get(idx).unwrap().clone();
        let out = if clamped_ratio != ratio {
            if self.tmux_out {
                format!("{}{}{}", "#[fg=red]", out, "#[fg=default]")
            } else {
                self.out_of_range.paint(out).to_string()
            }
        } else {
            out
        };
        Some(out)
    }

    // TODO infallible as of now
    pub fn chart(&self, input: Vec<f64>, ylabel: Option<String>) -> Result<String, String> {
        let ylabel = if ylabel.is_some() {
            ylabel.unwrap() + "↥ "
        } else {
            "".to_owned()
        };
        let output: String = input.iter().map(|i| self.map(i).unwrap()).collect();
        Ok(format!("{}{}", ylabel, output))
    }
}

impl fmt::Display for BarChartProducer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol_set = self.symbols.join("");
        write!(
            f,
            "Symbol_set: {}\nRange: {} to {}",
            symbol_set, self.min, self.max
        )
    }
}

fn main() {
    // let bcp = BarChartProducer::new(0., 10.);
    // let output = bcp.chart(vec![0.0, 0.5, 1., 1.5, 2.0, 2.5, 5., 9., 9.9999, 10., 10.5, -0.5, 0.,1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]).unwrap();
    // println!("{}", output);
    let matches = Opts::parse();
    let data: Vec<f64> = matches.data;
    let min = matches.min;
    let max = matches.max;
    let ylabel = matches.ylabel;
    let bcp = BarChartProducer::new(min, max, matches.tmux);
    println!("{}", bcp.chart(data, ylabel).expect("Charting failed."));
}

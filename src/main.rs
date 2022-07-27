use ansi_term::{Colour, Colour::Red};
use clap::{App, Arg, ArgMatches};
use core::fmt;

fn parse_args() -> ArgMatches {
    let matches = App::new("onelinecharts")
        .author("Jan Sosulski <mail@jan-sosulski.de>")
        .version("0.1.0")
        .about("Charts in one line (mostly)")
        .arg(
            Arg::new("min")
                .long("min")
                .value_name("MIN")
                .about("Minimum values of the data.")
                .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::new("max")
                .long("max")
                .value_name("MAX")
                .about("Maximum values of the data.")
                .default_value("100")
                .takes_value(true),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .about("Type of the chart, i.e. one of: ['bar']")
                .default_value("bar")
                .takes_value(true),
        )
        .arg(
            Arg::new("ylabel")
                .long("ylabel")
                .value_name("YLABEL")
                .about("Show YLabel with preceding text")
                .default_value("")
                .takes_value(true),
        )
        .arg(
            Arg::new("tmux")
                .long("tmux")
                .about("Is the output for tmux?")
        )
        .arg(
            Arg::new("data")
                .value_name("DATA")
                .about("Data to plot.")
                .multiple_values(true)
                .required(true),
        )
        .get_matches();
    matches
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
            symbols: "▁ö▂ö▃ö▄ö▅ö▆ö▇ö█"
                .split('ö')
                .map(|s| String::from(s))
                .collect(),
            min,
            max,
            out_of_range: Red,
            tmux_out
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
    pub fn chart(&self, input: Vec<f64>, ylabel: &str) -> Result<String, String> {
        let ylabel = if ylabel.len() > 0 {
            ylabel.to_owned() + "↥ "
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
    let matches = parse_args();
    let data: Vec<f64> = matches
        .values_of("data")
        .expect("No data provided")
        .map(|e| e.parse::<f64>().expect("Data cannot be parsed to float"))
        .collect();
    let min = matches
        .value_of("min")
        .unwrap()
        .parse::<f64>()
        .expect("Min cannot be parsed as float");
    let max = matches
        .value_of("max")
        .unwrap()
        .parse::<f64>()
        .expect("Max cannot be parsed as float");
    let ylabel = matches.value_of("ylabel").unwrap();
    let bcp = BarChartProducer::new(min, max, matches.is_present("tmux"));
    println!("{}", bcp.chart(data, ylabel).expect("Charting failed."));
}

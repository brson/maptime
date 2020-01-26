use gnuplot::{Figure, PlotOption, Color, AxesCommon, LabelOption, AutoOption, TickOption, GnuplotInitError};
use crate::data::Commit;
use std::time::Duration;
use chrono::{DateTime, Utc};
use std::error::Error as StdError;
use std::path::Path;
use crate::data::{Profile, RebuildType};

pub struct PlotData(pub Vec<Series>);

pub struct Series {
    pub profile: Profile,
    pub rebuild_type: RebuildType,
    pub values: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub commit: Commit,
    pub duration: Duration,
}

pub fn plot(data: PlotData, file: &Path, labels: bool) -> Result<(), Error> {
    let mut fg = Figure::new();
    fg.set_title("build times");

    let mut fg2d = fg.axes2d();
    fg2d.set_x_label("date", &[]);
    fg2d.set_y_label("compile-time", &[]);
    fg2d.set_x_time(true);
    fg2d.set_x_ticks(Some((AutoOption::Auto, 0)), &[TickOption::Format("%Y-%m-%d")], &[LabelOption::Rotate(310_f64)]);
    fg2d.set_y_ticks(Some((AutoOption::Auto, 0)), &[TickOption::Format("%gs")], &[]);

    for series in &data.0 {
        let x = series.values.iter().map(|e| e.commit.date.timestamp());
        let y = series.values.iter().map(|e| e.duration);
        fg2d.lines(x, y, &[PlotOption::Caption(&format!("{}+{}", series.profile.as_ref(), series.rebuild_type.as_ref()))]);
    }

    if labels {
        for series in &data.0 {
            for v in series.values.iter() {
                // Don't label commits called "prev" - uninteresting
                if v.commit.note.as_ref().map(Borrow::borrow)  == Some("prev") {
                    continue;
                }

                use gnuplot::Coordinate;
                use std::borrow::Borrow;
                let label = format!("{}\\n{}\\n{}", v.commit.date.format("%Y-%m-%d"), v.commit.id.as_str(), v.commit.note.as_ref().map(Borrow::borrow).unwrap_or("<no description>"));
                fg2d.label(&label,
                           Coordinate::Axis(v.commit.date.timestamp() as _),
                           Coordinate::Axis(v.duration.as_secs() as _),
                           &[LabelOption::Hypertext, LabelOption::MarkerSymbol('O'), LabelOption::MarkerSize(0.4)]);
            }
        }
    }

    //fg.save_to_svg(&file.to_str().ok_or(Error::PlotFile)?,
    //               600, 400).map_err(Error::GnuplotInit)?;

    fg.set_terminal("svg size 600, 400 dynamic mouse standalone", &file.to_str().ok_or(Error::PlotFile)?);
    fg.show();

    fg.echo(&mut std::io::stdout());

    println!("plot in {}", file.display());

    Ok(())
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "non-unicode plot file")]
    PlotFile,
    #[display(fmt = "gnuplot init error")]
    GnuplotInit(GnuplotInitError),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::PlotFile => None,
            Error::GnuplotInit(ref e) => Some(e),
        }
    }
}

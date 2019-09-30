use gnuplot::{Figure, PlotOption, Color, AxesCommon, LabelOption, AutoOption, TickOption};
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

pub struct Entry {
    pub commit: Commit,
    pub duration: Duration,
}

pub fn plot(data: PlotData, file: &Path) -> Result<(), Error> {
    let mut fg = Figure::new();
    fg.set_terminal("svg size 600, 400", &file.to_str().ok_or(Error::PlotFile)?);
    let mut fg2d = fg.axes2d();
    fg2d.set_x_ticks(Some((AutoOption::Auto, 0)), &[TickOption::Format("%Y-%m-%d")], &[LabelOption::Rotate(270_f64)]);

    for series in data.0 {
        let x = series.values.iter().map(|e| e.commit.date.timestamp());
        let y = series.values.iter().map(|e| e.duration);
        fg2d.lines(x, y, &[PlotOption::Caption(&format!("{}+{}", series.profile.as_ref(), series.rebuild_type.as_ref()))]);
        fg2d.set_x_time(true);
    }

    fg.echo(&mut std::io::stdout());
    fg.show();

    println!("plot in {}", file.display());

    Ok(())
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "non-unicode plot file")]
    PlotFile,
}

impl StdError for Error { }

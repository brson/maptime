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

pub struct Entry {
    pub commit: Commit,
    pub duration: Duration,
}

pub fn plot(data: PlotData, file: &Path) -> Result<(), Error> {
    let mut fg = Figure::new();
    fg.set_title("build times");

    {
        let mut lblno = 1;
        let mut cmds = String::new();
        for series in &data.0 {
            for v in series.values.iter() {
                use gnuplot::Coordinate;
                /*fg2d.label(&format!("{}", x.format("%Y-%m-%d")),
                Coordinate::Axis(x.timestamp() as _), Coordinate::Axis(y.as_secs() as _),
                &[]);
                 */
                let label = format!("{} {}", v.commit.date.format("%Y-%m-%d"), v.commit.id.as_str());
                let cmd = format!("set label {} \"{}\" at first {}, first {} front hypertext point pt 7 point ps 0.2",
                                  lblno, label,
                                  v.commit.date.timestamp() as f64,
                                  v.duration.as_secs() as f64);
                lblno += 1;
                cmds = format!("{}\n{}", cmds, cmd);
            }
        }
        fg.set_pre_commands(&cmds);
    }

    let mut fg2d = fg.axes2d();
    fg2d.set_x_label("date", &[]);
    fg2d.set_y_label("compile-time", &[]);
    fg2d.set_x_time(true);
    fg2d.set_x_ticks(Some((AutoOption::Auto, 0)), &[TickOption::Format("%Y-%m-%d")], &[LabelOption::Rotate(310_f64)]);
    fg2d.set_y_ticks(Some((AutoOption::Auto, 0)), &[TickOption::Format("%gs")], &[]);
    /*{
        use std::collections::BTreeSet;
        let mut times = BTreeSet::new();
        for series in &data.0 {
            for time in series.values.iter().map(|e| e.commit.date.timestamp()) {
                times.insert(time);
            }
        }

        use gnuplot::Tick;
        let times = times.into_iter().map(|t| Tick::<_, i64>::Minor(t));

        fg2d.set_x_ticks_custom(times, &[TickOption::Format("%Y-%m-%d")], &[LabelOption::Rotate(310_f64)]);
}*/

    for series in &data.0 {
        let x = series.values.iter().map(|e| e.commit.date.timestamp());
        let y = series.values.iter().map(|e| e.duration);
        fg2d.lines(x, y, &[PlotOption::Caption(&format!("{}+{}", series.profile.as_ref(), series.rebuild_type.as_ref()))]);
    }

    fg.echo(&mut std::io::stdout());

    //fg.set_terminal("svg size 600, 400", &file.to_str().ok_or(Error::PlotFile)?);
    //fg.show();
    fg.save_to_svg(&file.to_str().ok_or(Error::PlotFile)?,
                   600, 400).map_err(Error::GnuplotInit)?;

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

use super::driver::DriverBase;
use exporting;
use take::View;

pub struct FinalFrame<'a> {
    base: DriverBase<'a>,
}

impl<'a> FinalFrame<'a> {
    pub fn new(view: &View) -> FinalFrame {
        FinalFrame {
            base: DriverBase::new(view),
        }
    }

    pub fn render(&self, exporters: &mut [Box<dyn exporting::Sink>]) {
        for e in exporters {
            e.write(&self.base.target);
        }
    }
}

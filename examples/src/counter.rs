use gtk4_helper::{
    prelude::*,
    gtk4,
};

#[derive(Clone)]
pub enum CounterMsg {
    Inc,
    IncAsync,
    Dec
}

pub struct SimpleCounter {
    lbl: gtk4::Label,
    container: gtk4::Box,
    count: i32,
}

impl Widget for SimpleCounter {
    type Msg = CounterMsg;
    fn create<T: 'static + Clone + Fn(Self::Msg)>(sender: T) -> Self {
        let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        let lbl = gtk4::Label::new(Some(&format!("Count: {}", 0)));
        let btn = gtk4::ButtonBuilder::new()
            .label("Dec")
            .build();

        let tx = sender.clone();
        btn.connect_clicked(move |_| {
            tx(CounterMsg::Dec);
        });

        container.append(&btn);
        container.append(&lbl);

        let btn = gtk4::ButtonBuilder::new()
            .label("Inc")
            .build();

        let tx = sender.clone();
        btn.connect_clicked(move |_| {
            tx(CounterMsg::IncAsync);
        });
        container.append(&btn);

        Self {
            lbl,
            count: 0,
            container,
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            CounterMsg::Inc => {
                self.count += 1;
                self.lbl.set_text(&format!("Count: {}", self.count));
            }
            CounterMsg::Dec => {
                self.count -= 1;
                self.lbl.set_text(&format!("Count: {}", self.count));
            }
            CounterMsg::IncAsync => {
                return self.run_async(inc_async());
            }
        }
        Command::None
    }

    fn view(&self) -> &gtk4::Box {
        &self.container
    }
}

async fn inc_async() -> CounterMsg {
    tokio::time::delay_for(std::time::Duration::from_millis(200)).await;
    CounterMsg::Inc
}
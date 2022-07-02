use chrono::Timelike;
use crossbeam::channel::{unbounded, Receiver};

use egui::{
    plot::{LinkedAxisGroup, Text, Value},
    Color32, Label, ProgressBar, Response, TextBuffer, Ui, Widget, Window,
};
use egui_extras::{Size, StripBuilder};
use poll_promise::Promise;

use crate::sources::binance::{
    client::{Client, Kline},
    interval::Interval,
};

use super::{
    candles::Candles, data::Data, loading_state::LoadingState, props::Props, time_input::TimeInput,
    volume::Volume,
};

pub struct Graph {
    candles: Candles,
    volume: Volume,
    symbol: String,
    time_start: TimeInput,
    time_end: TimeInput,

    klines: Vec<Kline>,
    graph_props: Props,
    graph_loading_state: LoadingState,
    klines_promise: Option<Promise<Vec<Kline>>>,
    symbol_chan: Receiver<String>,
    valid: bool,
}

impl Default for Graph {
    fn default() -> Self {
        let graph_props = Props::default();
        let start_time = graph_props.time_start;
        let end_time = graph_props.time_end;
        let (_, r) = unbounded();

        Self {
            candles: Default::default(),
            volume: Default::default(),
            symbol: Default::default(),
            time_start: TimeInput::new(start_time.hour(), start_time.minute(), start_time.second()),
            time_end: TimeInput::new(end_time.hour(), end_time.minute(), end_time.second()),

            klines: Default::default(),
            graph_props,
            graph_loading_state: Default::default(),
            klines_promise: Default::default(),
            symbol_chan: r,
            valid: true,
        }
    }
}

impl Graph {
    pub fn new(symbol_chan: Receiver<String>) -> Self {
        Self {
            symbol_chan: symbol_chan,
            ..Default::default()
        }
    }
}

impl Widget for &mut Graph {
    fn ui(self, ui: &mut Ui) -> Response {
        let symbol_wrapped = self
            .symbol_chan
            .recv_timeout(std::time::Duration::from_millis(10));

        match symbol_wrapped {
            Ok(symbol) => {
                self.symbol = symbol;
                self.graph_loading_state = Default::default();
            }
            Err(_) => {}
        }

        if self.symbol == "" {
            return ui.label("select symbol");
        }

        if let Some(promise) = &self.klines_promise {
            if let Some(result) = promise.ready() {
                if self.graph_loading_state.received == 0 {
                    self.klines = vec![];
                }

                self.graph_loading_state.inc_received();

                if self.graph_loading_state.received > 0 {
                    result.iter().for_each(|k| {
                        self.klines.push(*k);
                    });
                }

                self.klines_promise = None;

                match self.graph_loading_state.is_finished() {
                    true => {
                        let data = Data::new(self.klines.clone());
                        let axes_group = LinkedAxisGroup::new(true, false);
                        self.volume = Volume::new(data.clone(), axes_group.clone());
                        self.candles = Candles::new(data, axes_group);
                    }
                    false => {
                        let start = self
                            .graph_loading_state
                            .left_edge()
                            .timestamp_millis()
                            .clone();

                        let symbol = self.symbol.to_string();
                        let interval = self.graph_props.interval.clone();
                        let mut limit = self.graph_props.limit.clone();
                        if self.graph_loading_state.is_last_page() {
                            limit = self.graph_loading_state.last_page_limit
                        }

                        self.klines_promise = Some(Promise::spawn_async(async move {
                            Client::kline(symbol, interval, start, limit).await
                        }));
                    }
                }
            }
        } else if !self.graph_loading_state.triggered {
            self.graph_loading_state = LoadingState::from_graph_props(&self.graph_props);
            self.graph_loading_state.triggered = true;

            let interval = self.graph_props.interval.clone();
            let start = self
                .graph_loading_state
                .left_edge()
                .timestamp_millis()
                .clone();

            let mut limit = self.graph_props.limit.clone();
            if self.graph_loading_state.is_last_page() {
                limit = self.graph_loading_state.last_page_limit;
            }

            let symbol = self.symbol.to_string();

            self.klines_promise = Some(Promise::spawn_async(async move {
                Client::kline(symbol, interval, start, limit).await
            }));
        }

        if !self.graph_loading_state.is_finished() {
            return ui
                .centered_and_justified(|ui| {
                    ui.add(
                        ProgressBar::new(self.graph_loading_state.progress())
                            .show_percentage()
                            .animate(true),
                    )
                })
                .response;
        }

        // TODO: extract to window and set open parameter
        Window::new(self.symbol.to_string())
            .drag_bounds(ui.max_rect())
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.collapsing("time period", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(
                            egui_extras::DatePickerButton::new(&mut self.graph_props.date_start)
                                .id_source("datepicker_start"),
                        );
                        ui.label("date start");
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.add(
                            egui_extras::DatePickerButton::new(&mut self.graph_props.date_end)
                                .id_source("datepicker_end"),
                        );
                        ui.label("date end");
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.add(&mut self.time_start);
                        ui.label("time start");
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.add(&mut self.time_end);
                        ui.label("time end");
                    });
                });
                ui.collapsing("interval", |ui| {
                    egui::ComboBox::from_label("pick data interval")
                        .selected_text(format!("{:?}", self.graph_props.interval))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.graph_props.interval,
                                Interval::Day,
                                "Day",
                            );
                            ui.selectable_value(
                                &mut self.graph_props.interval,
                                Interval::Hour,
                                "Hour",
                            );
                            ui.selectable_value(
                                &mut self.graph_props.interval,
                                Interval::Minute,
                                "Minute",
                            );
                        });
                });

                ui.add_space(5f32);

                if ui.button("apply").clicked() {
                    let time_start = self.time_start.get_time();
                    match time_start {
                        Some(time) => {
                            self.valid = true;
                            self.graph_loading_state = LoadingState::from_graph_props(&self.graph_props);                    
                            self.graph_loading_state.triggered = true;

                            self.graph_props.time_start = time;
                            let start = self.graph_props.start_time().timestamp_millis().clone();
                            let pair = self.symbol.to_string();
                            let interval = self.graph_props.interval.clone();
                            let mut limit = self.graph_props.limit.clone();

                            if self.graph_loading_state.is_last_page() {
                                limit = self.graph_loading_state.last_page_limit
                            }

                            self.klines_promise = Some(Promise::spawn_async(async move {
                                Client::kline(pair, interval, start, limit).await
                            }));
                        }
                        None => {
                            self.valid = false;
                        }
                    }
                }

                if !self.valid {
                    ui.label("invalid time range");
                }
            });

        StripBuilder::new(ui)
            .size(Size::relative(0.8))
            .size(Size::remainder())
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    ui.add(&self.candles);
                });
                strip.cell(|ui| {
                    ui.add(&self.volume);
                });
            })
    }
}
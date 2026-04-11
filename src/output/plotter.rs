use crate::block::Block;
use crate::output::magmar::Magmar;
use crate::signal::Signal;
use alloc::vec::Vec;
use core::fmt::Display;
use core::marker::PhantomData;
use num_traits::real::Real;
use std::boxed::Box;
use std::format;
use std::string::String;
use std::string::ToString;
use std::vec;

#[derive(Clone, Copy, Default, Debug)]
pub enum LegendPosition {
    TopLeft,
    Top,
    #[default]
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Display for LegendPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LegendPosition::TopLeft => "top_left",
                LegendPosition::Top => "top",
                LegendPosition::TopRight => "top_right",
                LegendPosition::Left => "left",
                LegendPosition::Right => "right",
                LegendPosition::BottomLeft => "bottom_left",
                LegendPosition::Bottom => "bottom",
                LegendPosition::BottomRight => "bottom_right",
            }
        )
    }
}

#[derive(Debug)]
pub struct Plotter<const N: usize, T>
where
    T: Real + ToString,
{
    data: Vec<[Signal<T>; N]>,
    variable_names: [String; N],
    magmar: Option<Magmar>,
    title: String,
    is_light: bool,
    legend_pos: Option<LegendPosition>,
}

#[derive(Debug)]
pub struct RTPlotter<const N: usize, T>
where
    T: Real + ToString,
{
    variable_names: [String; N],
    magmar: Option<Magmar>,
    title: String,
    is_light: bool,
    legend_pos: Option<LegendPosition>,
    _marker: PhantomData<[T; N]>,
}

pub trait Joinable {
    fn join(&mut self);
}

pub trait Savable {
    fn save(&mut self, path: &str) -> Result<String, String>;
}

impl<const N: usize, T> Plotter<N, T>
where
    T: Real + ToString,
{
    pub fn new(title: String, variable_names: [impl AsRef<str>; N]) -> Self {
        Self {
            data: Vec::new(),
            variable_names: variable_names.map(|vn| vn.as_ref().to_string()),
            magmar: None,
            title,
            is_light: false,
            legend_pos: None,
        }
    }

    pub fn with_light_theme(mut self) -> Self {
        self.is_light = true;
        self
    }

    pub fn with_legend_position(mut self, pos: LegendPosition) -> Self {
        self.legend_pos = Some(pos);
        self
    }

    pub fn display(&mut self) {
        self.magmar = Some(Magmar::new(&self.title, self.is_light));

        if let Some(magmar) = &mut self.magmar {
            magmar.send_labels(format!(
                "Time (s),{}\n",
                self.variable_names
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ));

            if let Some(pos) = self.legend_pos {
                let _ = magmar.send_command(format!("!legend,{}\n", pos), "Legend position set to");
            }

            for signals in &self.data {
                let time = &signals[0].delta.sim_time().as_secs_f32();
                let mut data = vec![*time as f64];
                data.extend(
                    signals
                        .iter()
                        .map(|s| s.value.to_string().parse::<f64>().unwrap_or(0.0)),
                );

                magmar.send_data(&data);
            }
        }
    }
}

impl<const N: usize, T> RTPlotter<N, T>
where
    T: Real + ToString,
{
    pub fn new(title: String, variable_names: [impl AsRef<str>; N]) -> Self {
        Self {
            magmar: None,
            variable_names: variable_names.map(|vn| vn.as_ref().to_string()),
            title,
            _marker: PhantomData,
            is_light: false,
            legend_pos: None,
        }
    }

    pub fn with_light_theme(mut self) -> Self {
        self.is_light = true;
        self
    }

    pub fn with_legend_position(mut self, pos: LegendPosition) -> Self {
        self.legend_pos = Some(pos);
        self
    }
}

impl<const N: usize, T> Block for Plotter<N, T>
where
    T: Real + ToString,
{
    type Input = [T; N];
    type Output = [T; N];

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.data.push(input.value.map(|s| Signal {
            value: s,
            delta: input.delta,
        }));
        input
    }

    fn reset(&mut self) {
        self.data.clear();
        if let Some(magmar) = &mut self.magmar {
            magmar.kill().ok();
            self.magmar = None;
        }
    }
}

impl<const N: usize, T> Block for RTPlotter<N, T>
where
    T: Real + ToString,
{
    type Input = [T; N];
    type Output = [T; N];

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        if self.magmar.is_none() {
            let mut magmar = Magmar::new(&self.title, self.is_light);

            magmar.send_labels(format!(
                "Time (s),{}\n",
                self.variable_names
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ));

            if let Some(pos) = self.legend_pos {
                let _ = magmar.send_command(format!("!legend,{}\n", pos), "Legend position set to");
            }

            self.magmar = Some(magmar);
        }

        let magmar = self.magmar.as_mut().unwrap();

        let time = &input.delta.sim_time().as_secs_f32();
        let mut data = vec![*time as f64];
        data.extend(
            input
                .value
                .iter()
                .map(|s| s.to_string().parse::<f64>().unwrap_or(0.0)),
        );

        magmar.send_data(&data);

        input
    }

    fn reset(&mut self) {
        if let Some(magmar) = &mut self.magmar {
            magmar.kill().ok();
            self.magmar = None;
        }
    }
}

impl<const N: usize, T> Drop for Plotter<N, T>
where
    T: Real + ToString,
{
    fn drop(&mut self) {
        if let Some(magmar) = &mut self.magmar {
            magmar.kill().unwrap();
        }
    }
}

impl<const N: usize, T> Drop for RTPlotter<N, T>
where
    T: Real + ToString,
{
    fn drop(&mut self) {
        if let Some(magmar) = &mut self.magmar {
            magmar.kill().unwrap();
        }
    }
}

impl<const N: usize, T> Joinable for Plotter<N, T>
where
    T: Real + ToString,
{
    fn join(&mut self) {
        if let Some(magmar) = &mut self.magmar {
            magmar.wait().ok();
        }
    }
}

impl<const N: usize, T> Joinable for RTPlotter<N, T>
where
    T: Real + ToString,
{
    fn join(&mut self) {
        if let Some(magmar) = &mut self.magmar {
            magmar.wait().ok();
        }
    }
}

impl<const N: usize, T> Savable for Plotter<N, T>
where
    T: Real + ToString,
{
    fn save(&mut self, path: &str) -> Result<String, String> {
        let Some(magmar) = self.magmar.as_mut() else {
            return Err("Plotter process is not running.".to_string());
        };

        magmar.send_command(format!("!save,{}", path), "Saved screenshot to")
    }
}

impl<const N: usize, T> Savable for RTPlotter<N, T>
where
    T: Real + ToString,
{
    fn save(&mut self, path: &str) -> Result<String, String> {
        let Some(magmar) = self.magmar.as_mut() else {
            return Err("Plotter process is not running.".to_string());
        };

        magmar.send_command(format!("!save,{}", path), "Saved screenshot to")
    }
}

pub trait JoinAll {
    fn join_all(&mut self);
}

impl JoinAll for [Box<dyn Joinable>] {
    fn join_all(&mut self) {
        for plotter in self {
            plotter.join();
        }
    }
}

impl JoinAll for Vec<Box<dyn Joinable>> {
    fn join_all(&mut self) {
        for plotter in self {
            plotter.join();
        }
    }
}

impl<T, S> JoinAll for (T, S)
where
    T: Joinable,
    S: Joinable,
{
    fn join_all(&mut self) {
        self.0.join();
        self.1.join();
    }
}

impl<T, S, R> JoinAll for (T, S, R)
where
    T: Joinable,
    S: Joinable,
    R: Joinable,
{
    fn join_all(&mut self) {
        self.0.join();
        self.1.join();
        self.2.join();
    }
}

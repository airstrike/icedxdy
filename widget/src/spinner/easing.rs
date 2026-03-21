// Based on iced loading_spinners example by Héctor Ramón Jiménez (hecrj)
// Original: https://github.com/iced-rs/iced/tree/master/examples/loading_spinners
// License: MIT

use iced_core::Point;

use lyon_algorithms::measure::PathMeasurements;
use lyon_algorithms::path::{builder::NoAttributes, path::BuilderImpl, Path};

use std::sync::LazyLock;

pub static STANDARD: LazyLock<Easing> = LazyLock::new(|| {
    Easing::builder()
        .cubic_bezier_to([0.2, 0.0], [0.0, 1.0], [1.0, 1.0])
        .build()
});

pub struct Easing {
    path: Path,
    measurements: PathMeasurements,
}

impl Easing {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn y_at_x(&self, x: f32) -> f32 {
        let mut sampler = self.measurements.create_sampler(
            &self.path,
            lyon_algorithms::measure::SampleType::Normalized,
        );
        sampler.sample(x).position().y
    }
}

pub struct Builder(NoAttributes<BuilderImpl>);

impl Builder {
    pub fn new() -> Self {
        let mut builder = Path::builder();
        builder.begin(lyon_algorithms::geom::point(0.0, 0.0));
        Self(builder)
    }

    pub fn cubic_bezier_to(
        mut self,
        ctrl1: impl Into<Point>,
        ctrl2: impl Into<Point>,
        to: impl Into<Point>,
    ) -> Self {
        self.0
            .cubic_bezier_to(Self::point(ctrl1), Self::point(ctrl2), Self::point(to));
        self
    }

    pub fn build(mut self) -> Easing {
        self.0.line_to(lyon_algorithms::geom::point(1.0, 1.0));
        self.0.end(false);

        let path = self.0.build();
        let measurements = PathMeasurements::from_path(&path, 0.0);

        Easing { path, measurements }
    }

    fn point(p: impl Into<Point>) -> lyon_algorithms::geom::Point<f32> {
        let p: Point = p.into();
        lyon_algorithms::geom::point(p.x.clamp(0.0, 1.0), p.y.clamp(0.0, 1.0))
    }
}

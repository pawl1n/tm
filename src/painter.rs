use eframe::egui::{pos2, Color32, Pos2, Sense, Ui};

#[derive(Debug, Default)]
pub struct SK {
    pub distances_to_self: Vec<u32>,
    pub distances_to_closest: Vec<u32>,
    pub distances_from_closest_to_itself: Vec<u32>,
    pub distances_from_closest: Vec<u32>,
    pub distance: u32,
    pub closest: usize,
    self_realizations: Vec<Pos2>,
    closest_realizations: Vec<Pos2>,
    r_kullback: Vec<f64>,
    r_shannon: Vec<f64>,
}

impl SK {
    pub fn new(
        distances_to_self: Vec<u32>,
        distances_to_closest: Vec<u32>,
        distances_from_closest_to_itself: Vec<u32>,
        distances_from_closest: Vec<u32>,
        distance: u32,
        closest: usize,
    ) -> Self {
        let self_realizations =
            Self::calculate_coordinates(&distances_to_self, &distances_from_closest, distance);

        let closest_realizations = Self::calculate_coordinates(
            &distances_to_closest,
            &distances_from_closest_to_itself,
            distance,
        );

        Self {
            distances_to_self,
            distances_to_closest,
            distances_from_closest_to_itself,
            distances_from_closest,
            distance,
            closest,
            self_realizations,
            closest_realizations,
            ..Default::default()
        }
    }

    pub fn set_radius(&mut self, r_kullback: Vec<f64>, r_shannon: Vec<f64>) {
        self.r_kullback = r_kullback;
        self.r_shannon = r_shannon;
    }

    fn calculate_coordinates(
        distances_to_self: &[u32],
        distances_from_closest: &[u32],
        distance: u32,
    ) -> Vec<Pos2> {
        if distance == 0 {
            return Vec::new();
        }

        let mut coordinates: Vec<Pos2> = Vec::with_capacity(distances_to_self.len());

        for i in 0..distances_to_self.len() {
            let distance = distance as f32;

            let rs = distances_to_self[i] as f32; // Disctance to center of self class
            let rc = distances_from_closest[i] as f32; // Distance to center of closest class

            let x = (distance.powi(2) - rc.powi(2) + rs.powi(2)) / (2.0 * distance);
            let y2 = rs.powi(2) - x.powi(2);
            let y = (y2 as f64).sqrt() as f32 * if i % 2 == 0 { 1.0 } else { -1.0 };

            if y.is_finite() && x.is_finite() {
                let center = pos2(x, y);
                coordinates.push(center);
            }
        }

        coordinates
    }

    pub fn paint(&self, ui: &mut Ui) {
        if self.distance == 0 {
            ui.label("Nothing to show");
            return;
        }

        let ((min_x, min_y), (max_x, max_y)) = self.find_min_max();

        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, Sense::hover());
        let rect = response.rect;
        let left_top = rect.left_top();

        let k = std::cmp::min_by(
            size.x / (max_x - min_x + 15.0),
            size.y / (max_y - min_y + 15.0),
            |a, b| a.total_cmp(b),
        );

        let radius = 1.0 * k;
        let stroke_width = 1.0;
        let padding_y = left_top.y + radius + stroke_width;
        let padding_x = left_top.x + radius + stroke_width;

        let center = pos2(padding_x - min_x * k, padding_y - min_y * k);
        painter.circle_filled(center, radius, Color32::YELLOW);

        let center = pos2(
            padding_x + (self.distance as f32 - min_x) * k,
            padding_y - min_y * k,
        );
        painter.circle_filled(center, radius, Color32::RED);

        self.r_shannon.iter().for_each(|r| {
            let center = pos2(padding_x - min_x * k, padding_y - min_y * k);
            painter.circle_stroke(center, *r as f32 * k, (stroke_width, Color32::YELLOW));
        });

        self.r_kullback.iter().for_each(|r| {
            let center = pos2(padding_x - min_x * k, padding_y - min_y * k);
            painter.circle_stroke(center, *r as f32 * k, (stroke_width, Color32::RED));
        });

        self.self_realizations.iter().for_each(|c| {
            let center = pos2(padding_x + (c.x - min_x) * k, padding_y + (c.y - min_y) * k);
            painter.circle_stroke(center, radius, (stroke_width, Color32::YELLOW));
        });

        self.closest_realizations.iter().for_each(|c| {
            let center = pos2(padding_x + (c.x - min_x) * k, padding_y + (c.y - min_y) * k);
            painter.circle_stroke(center, radius, (stroke_width, Color32::RED));
        });
    }

    fn find_min_max(&self) -> ((f32, f32), (f32, f32)) {
        let max_radius = self
            .r_shannon
            .iter()
            .cloned()
            .reduce(f64::max)
            .unwrap_or_default()
            .max(
                self.r_kullback
                    .iter()
                    .cloned()
                    .reduce(f64::max)
                    .unwrap_or_default(),
            ) as f32;

        let min_x = (0..self.self_realizations.len())
            .map(|i| {
                self.self_realizations[i]
                    .x
                    .min(self.closest_realizations[i].x)
            })
            .reduce(f32::min)
            .unwrap_or_default()
            .min(0.0)
            .min(max_radius * -1.0);

        let min_y = (0..self.self_realizations.len())
            .map(|i| {
                self.self_realizations[i]
                    .y
                    .min(self.closest_realizations[i].y)
            })
            .reduce(f32::min)
            .unwrap_or_default()
            .min(0.0)
            .min(max_radius * -1.0);

        let max_x = (0..self.self_realizations.len())
            .map(|i| {
                self.self_realizations[i]
                    .x
                    .max(self.closest_realizations[i].x)
            })
            .reduce(f32::max)
            .unwrap_or_default()
            .max(self.distance as f32)
            .max(max_radius);

        let max_y = (0..self.self_realizations.len())
            .map(|i| {
                self.self_realizations[i]
                    .y
                    .max(self.closest_realizations[i].y)
            })
            .reduce(f32::max)
            .unwrap_or_default()
            .max(0.0)
            .max(max_radius);

        ((min_x, min_y), (max_x, max_y))
    }
}

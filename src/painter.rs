use eframe::egui::{pos2, Color32, Pos2, Sense, Ui};

#[derive(Debug, Default)]
pub struct SK {
    pub distances_to_center: Vec<u32>,
    pub distances_to_closest: Vec<u32>,
    pub distance: u32,
    pub closest: usize,
    coordinates: Vec<Pos2>,
}

impl SK {
    pub fn new(
        distances_to_center: Vec<u32>,
        distances_to_closest: Vec<u32>,
        distance: u32,
        closest: usize,
    ) -> Self {
        let coordinates =
            Self::calculate_coordinates(&distances_to_center, &distances_to_closest, distance);

        Self {
            distances_to_center,
            distances_to_closest,
            distance,
            closest,
            coordinates,
        }
    }

    fn calculate_coordinates(
        distances_to_center: &[u32],
        distances_to_closest: &[u32],
        distance: u32,
    ) -> Vec<Pos2> {
        if distance == 0 {
            return Vec::new();
        }

        let mut coordinates: Vec<Pos2> = Vec::with_capacity(distances_to_center.len());

        for i in 0..distances_to_center.len() {
            let distance = distance as f32;

            let rs = distances_to_center[i] as f32; // Disctance to center of self class
            let rc = distances_to_closest[i] as f32; // Distance to center of closest class

            let x = (distance.powi(2) - rc.powi(2) + rs.powi(2)) / (2.0 * distance);
            let y2 = rs.powi(2) - x.powi(2);
            let y = (y2 as f64).sqrt() as f32;

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

        let min_x = self
            .coordinates
            .iter()
            .filter(|c| c.x < 0.0)
            .min_by(|c1, c2| c1.x.total_cmp(&c2.x))
            .map_or(0.0, |c| c.x);

        let max_x = self
            .coordinates
            .iter()
            .filter(|c| c.x > self.distance as f32)
            .max_by(|c1, c2| c1.x.total_cmp(&c2.x))
            .map_or(self.distance as f32, |c| c.x);

        let max_y = self
            .coordinates
            .iter()
            .filter(|c| c.y.is_finite())
            .max_by(|c1, c2| c1.y.total_cmp(&c2.y))
            .map_or(0.0, |c| c.y);

        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, Sense::hover());
        let rect = response.rect;
        let left_top = rect.left_top();

        let k = std::cmp::min_by(
            size.x / (max_x - min_x + 15.0),
            size.y / (max_y + 15.0),
            |a, b| a.total_cmp(b),
        );

        let radius = 5.0 * k;
        let stroke_width = 1.0 * k;
        let padding_y = left_top.y + radius + stroke_width;
        let padding_x = left_top.x + radius + stroke_width;

        let center = pos2(padding_x - min_x * k, padding_y);
        painter.circle_stroke(center, radius, (stroke_width, Color32::YELLOW));

        let center = pos2(padding_x + (self.distance as f32 - min_x) * k, padding_y);
        painter.circle_stroke(center, radius, (stroke_width, Color32::RED));

        self.coordinates.iter().for_each(|c| {
            let center = pos2(padding_x + (c.x - min_x) * k, padding_y + c.y * k);
            painter.circle_stroke(center, 1.0, (stroke_width, Color32::GREEN));
        });
    }
}

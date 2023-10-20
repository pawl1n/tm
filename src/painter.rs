use eframe::egui::{pos2, vec2, Color32, Sense, Ui};

pub fn paint(ui: &mut Ui, sk: &[(&[u32], (&[u32], u32))]) {
    let radius = 10.0;
    let stroke_width = 2.0;
    let padding = radius + stroke_width;

    let distance = 2000 / sk[0].1 .1;

    let (response, painter) =
        ui.allocate_painter(vec2(distance + padding * 2.0, 200.0), Sense::hover());
    let rect = response.rect;

    let left_center = rect.left_center();
    let center = pos2(left_center.x + padding, left_center.y);
    painter.circle_stroke(center, radius, (stroke_width, Color32::YELLOW));

    let center = pos2(left_center.x + padding + distance, left_center.y);
    painter.circle_stroke(center, radius, (stroke_width, Color32::RED));

    for i in 0..sk[0].1.len() {
        let rs = sk[0].0[i]; // Disctance to center of self class
        let rc = sk[0].1 .0[i]; // Distance to center of closest class

        let x = (distance.pow(2) - rc.pow(2) + rs.pow(2)) / (2.0 * distance);
        let y2 = rs.pow(2) - x.pow(2);
        let y = (y2 as f64).sqrt();

        let center = pos2(left_center.x + padding + x, left_center.y);
        painter.circle_stroke(center, radius, (stroke_width, Color32::GREEN));
    }
}

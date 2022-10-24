use std::f32::consts::PI;

use crate::model::{Scene, Screen, Vector};

pub(crate) struct Camera {
    pub(crate) position: Vector,
    pub(crate) x_angle: f32,
    pub(crate) y_angle: f32,
}

impl Camera {
    pub(crate) fn new(x: f32, y: f32, z: f32, x_angle: f32, y_angle: f32) -> Self {
        Self {
            position: Vector::new(x, y, z),
            x_angle,
            y_angle,
        }
    }
    pub(crate) fn fill(&self, screen: &mut Screen, scene: &Scene) {
        //amount of points
        let asm = 55;
        let bsm = 270;
        let aem = 55;
        let bem = 270;
        let amm = 30;
        let bmm = 180;
        //sun
        for als in 0..asm {
            let als = (als as f32) / (asm as f32) * PI;
            let alsc = als.cos();
            let alss = als.sin();
            for bs in 0..bsm {
                let bs = (bs as f32) / (bsm as f32) * PI * 2.;
                let bsc = bs.cos();
                let bss = bs.sin();
                let p = Vector::new(
                    scene.rs * bsc * alss,
                    scene.rs * alsc,
                    scene.rs * bss * alss,
                );
                //inverted so we draw the inner side of the sun
                let n = p.inverted().normalized();
                self.draw_point(&p, &n, screen, (255, 255, 0));
            }
        }

        //earth
        let ges = scene.ge.sin();
        let gec = scene.ge.cos();
        let m = Vector::new(scene.rse * gec, 0., -scene.rse * ges);
        for aei in 0..aem {
            let ae = (aei as f32) / (aem as f32) * PI;
            let aes = ae.sin();
            let aec = ae.cos();
            for bei in 0..bem {
                let be = (bei as f32) / (bem as f32) * PI * 2.;
                let bes = be.sin();
                let bec = be.cos();
                let p = Vector::new(
                    scene.re * bec * aes * gec + scene.re * bes * aes * ges + scene.rse * gec,
                    scene.re * aec,
                    -scene.re * bec * aes * ges + scene.re * bes * aes * gec - scene.rse * ges,
                );
                let n = p.add(&m.inverted()).normalized();
                self.draw_point(&p, &n, screen, (0, 0, 255));
            }
        }

        // moon
        let gms = scene.gm.sin();
        let gmc = scene.gm.cos();
        let t11 = gec * gmc - ges * gms;
        let t31 = -ges * gmc - gec * gms;
        let t13 = gec * gms + ges * gmc;
        let t33 = -ges * gms + gec * gmc;
        let t14 = scene.rem * gmc * gec
            - scene.rem * gms * ges
            + scene.rse * gec;
        let t34 = -scene.rem * gmc * ges
            - scene.rem * gms * gec
            - scene.rse * ges;
        let m = Vector::new(t14, 0., t34);
        for ami in 0..amm {
            let am = (ami as f32) / (amm as f32) * PI;
            let ams = am.sin();
            let amc = am.cos();
            for bmi in 0..bmm {
                let bm = (bmi as f32) / (bmm as f32) * PI * 2.;
                let bms = bm.sin();
                let bmc = bm.cos();

                let p = Vector::new(
                    scene.rm * bmc * ams * t11
                        + scene.rm * bms * ams * t13
                        + t14,
                    scene.rm * amc,
                    scene.rm * bmc * ams * t31
                        + scene.rm * bms * ams * t33
                        + t34,
                );
                let n = p.add(&m.inverted()).normalized();
                self.draw_point(&p, &n, screen, (255, 255, 255));
            }
        }
    }

    fn draw_point(&self, point: &Vector, normal: &Vector, screen: &mut Screen, rgb: (u8, u8, u8)) {
        let point_original = point;
        let point = point.add(&self.position.times(-1.));
        let view_dir = self.direction();
        if normal.dot(&view_dir.inverted()) <= 0. {
            return;
        }
        let x2_z2 = view_dir.x * view_dir.x + view_dir.z * view_dir.z;
        let y = point.x * -(view_dir.x * view_dir.y) / x2_z2
            + point.y
            + point.z * -(view_dir.y * view_dir.z) / x2_z2;
        let z = point.x * (view_dir.x) / x2_z2 + point.z * (view_dir.z) / x2_z2;
        let p_proj_y = Vector::new(0., y, 0.).add(&view_dir.times(z));
        let angle_y = p_proj_y.normalized().dot(&view_dir).acos();

        if angle_y > self.y_angle / 2. {
            return;
        }
        let middle_y = screen.height / 2;
        let px_y = (middle_y as f32)
            + if y.is_sign_negative() { -2. } else { 2. }
                * (angle_y / self.y_angle * (middle_y as f32));

        let y2_z2 = view_dir.y * view_dir.y + view_dir.z * view_dir.z;
        let x = point.x
            + point.y * -(view_dir.x * view_dir.y) / y2_z2
            + point.z * -(view_dir.x * view_dir.z) / y2_z2;
        let z = point.y * (view_dir.y) / y2_z2 + point.z * (view_dir.z) / y2_z2;
        let p_proj_x = Vector::new(x, 0., 0.).add(&view_dir.times(z));
        let angle_x = p_proj_x.normalized().dot(&view_dir).acos();
        if angle_x > self.x_angle / 2. {
            return;
        }
        let middle_x = screen.width / 2;
        let px_x = (middle_x as f32)
            + if x.is_sign_negative() { -2. } else { 2. }
                * (angle_x / self.x_angle * (middle_x as f32));
        let depth = point_original.add(&self.position.times(-1.)).mag();
        let l = point_original.inverted().normalized();
        let intensity = normal.dot(&l).max(0.);

        Camera::fill_px(
            screen,
            px_x.round() as usize,
            px_y.round() as usize,
            (31. + intensity * 224.).round() as u8,
            depth,
            rgb,
        );
    }

    fn fill_px(screen: &mut Screen, x: usize, y: usize, val: u8, depth: f32, rgb: (u8, u8, u8)) {
        let width = if let Some(i) = screen.zbuffer.get_mut(y) {
            i
        } else {
            return;
        };
        let cur_depth = if let Some(i) = width.get(x) {
            i
        } else {
            return;
        };
        if cur_depth != &0. && cur_depth < &depth {
            return;
        }
        *width.get_mut(x).expect("Drawn over the buffer") = depth;
        let width = screen.buffer.get_mut(y).expect("Drawn over the buffer");
        let (r, g, b) = rgb;
        let (rn, gn, bn) = (
            (r as f32) / 255. * (val as f32),
            (g as f32) / 255. * (val as f32),
            (b as f32) / 255. * (val as f32),
        );
        *width.get_mut(x).expect("Drawn over the buffer") = (rn as u8, gn as u8, bn as u8);
    }

    pub(crate) fn direction(&self) -> Vector {
        self.position.times(-1.).normalized()
    }
}

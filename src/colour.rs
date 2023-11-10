fn v(m1: f32, m2: f32, hue: f32) -> f32 {
    let hue = hue % 1.0;
    if hue < (1.0/6.0) {
        return m1 + (m2-m1) * hue * 6.0;
    }
    if hue < 0.5 {
        return m2;
    }
    if hue < (2.0/3.0) {
        return m1 + (m2-m1) * ((2.0/3.0) - hue) * 6.0
    }
    return m1
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [u8; 3] {
    let h: f32 = h / 360.0;
    let s: f32 = s / 100.0;
    let l: f32 = l / 100.0;

    if s == 0.0 {
        return [(l*255.0) as u8, (l*255.0) as u8, (l*255.0) as u8];
    } else {
        let m2;

        if l <= 0.5 {
            m2 = l * (1.0 + s);
        } else {
            m2 = l + s - (l * s);
        }

        let m1: f32 = 2.0*l - m2;
        return [(v(m1, m2, h + (1.0/3.0)) * 255.0) as u8, (v(m1, m2, h) * 255.0) as u8, (v(m1, m2, h - (1.0/3.0)) * 255.0) as u8];
    }
}

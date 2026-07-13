//! SVG heat-maps of the modes. A picture of a mode our kernel is blind to is the
//! whole point of the exercise, so the drawing is part of the instrument, not a
//! garnish.

use std::fmt::Write as _;
use std::path::Path;

/// Diverging blue → white → red, symmetric about zero. A checkerboard therefore
/// reads as a literal checkerboard.
fn color(v: f64, scale: f64) -> String {
    let t = (v / scale).clamp(-1.0, 1.0);
    let (r, g, b) = if t >= 0.0 {
        // white → red
        (255.0, 255.0 * (1.0 - t) + 60.0 * t, 255.0 * (1.0 - t) + 60.0 * t)
    } else {
        let t = -t;
        (255.0 * (1.0 - t) + 40.0 * t, 255.0 * (1.0 - t) + 90.0 * t, 255.0 * (1.0 - t) + 190.0 * t)
    };
    format!("rgb({},{},{})", r as u8, g as u8, b as u8)
}

pub struct Panel {
    pub title: String,
    pub data: Vec<f64>,
    pub nx: usize,
}

/// Write one SVG holding a row of panels, each a `nx × nx` heat-map with its own
/// symmetric colour scale (printed on the panel, so the picture is quantitative).
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if !cur.is_empty() && cur.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

pub fn panels_svg(path: &Path, heading: &str, subtitle: &str, panels: &[Panel]) -> std::io::Result<()> {
    let cell = 24.0f64;
    let pad = 20.0f64;
    let sub = wrap(subtitle, 128);
    let head = 40.0 + 15.0 * sub.len() as f64 + 16.0;
    // Panel captions may carry a newline; reserve room for the tallest.
    let cap_lines = panels.iter().map(|p| p.title.lines().count()).max().unwrap_or(1);
    let cap = 16.0 + 14.0 * cap_lines as f64 + 14.0;
    let panels_w: f64 =
        panels.iter().map(|p| p.nx as f64 * cell).sum::<f64>() + pad * (panels.len() as f64 - 1.0);
    let hmax = panels.iter().map(|p| p.nx as f64 * cell).fold(0.0f64, f64::max);
    // Never clip the heading: the text must fit the canvas it is drawn on.
    let total_w = (panels_w + 2.0 * pad).max(heading.len() as f64 * 9.4 + 2.0 * pad).max(880.0);
    let total_h = head + hmax + cap;

    let mut s = String::new();
    let _ = write!(
        s,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{total_w:.0}" height="{total_h:.0}" viewBox="0 0 {total_w:.0} {total_h:.0}" font-family="ui-monospace,Menlo,monospace">
<rect width="100%" height="100%" fill="#fbfbfd"/>
<text x="{pad}" y="27" font-size="16" font-weight="700" fill="#111">{h}</text>
"##,
        h = esc(heading)
    );
    for (i, line) in sub.iter().enumerate() {
        let y = 47.0 + 15.0 * i as f64;
        let _ = write!(s, r##"<text x="{pad}" y="{y:.0}" font-size="11" fill="#555">{}</text>"##, esc(line));
    }
    let mut x0 = pad;
    for p in panels {
        let scale = p.data.iter().fold(0.0f64, |m, v| m.max(v.abs())).max(1e-30);
        for y in 0..p.nx {
            for x in 0..p.nx {
                let c = color(p.data[y * p.nx + x], scale);
                let px = x0 + x as f64 * cell;
                let py = head + y as f64 * cell;
                let _ = write!(
                    s,
                    r##"<rect x="{px:.1}" y="{py:.1}" width="{cell:.1}" height="{cell:.1}" fill="{c}" stroke="#e6e6ee" stroke-width="0.4"/>"##
                );
            }
        }
        let w = p.nx as f64 * cell;
        let _ = write!(
            s,
            r##"<rect x="{x0:.1}" y="{head:.1}" width="{w:.1}" height="{w:.1}" fill="none" stroke="#333" stroke-width="1"/>"##
        );
        for (li, line) in p.title.lines().enumerate() {
            let ty = head + w + 16.0 + 14.0 * li as f64;
            let _ = write!(
                s,
                r##"<text x="{x0:.1}" y="{ty:.1}" font-size="11" font-weight="600" fill="#222">{}</text>"##,
                esc(line)
            );
        }
        let ty2 = head + w + 16.0 + 14.0 * p.title.lines().count() as f64;
        let _ = write!(s, r##"<text x="{x0:.1}" y="{ty2:.1}" font-size="10" fill="#777">peak |v| = {scale:.3e}</text>"##);
        x0 += w + pad;
    }
    s.push_str("\n</svg>\n");
    std::fs::write(path, s)
}

/// A spectrum plot: |λ| on a log axis, index on x — the "count the zeros" picture.
pub fn spectrum_svg(path: &Path, heading: &str, subtitle: &str, series: &[(String, Vec<f64>)], floor: f64) -> std::io::Result<()> {
    let w = 820.0f64;
    let h = 380.0f64;
    let (l, r, t, b) = (70.0f64, 20.0f64, 74.0f64, 46.0f64);
    let pw = w - l - r;
    let ph = h - t - b;
    let nmax = series.iter().map(|(_, v)| v.len()).max().unwrap_or(1) as f64;
    let vmax = series.iter().flat_map(|(_, v)| v.iter()).fold(1e-30f64, |m, &v| m.max(v));
    let lo = floor.log10();
    let hi = (vmax * 3.0).log10();
    let ymap = |v: f64| {
        let lv = v.max(floor).log10();
        t + ph * (1.0 - (lv - lo) / (hi - lo))
    };
    let colors = ["#c0392b", "#2471a3", "#1e8449", "#8e44ad", "#d68910", "#117a65"];

    let mut s = String::new();
    let _ = write!(
        s,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" font-family="ui-monospace,Menlo,monospace">
<rect width="100%" height="100%" fill="#fbfbfd"/>
<text x="18" y="26" font-size="16" font-weight="700" fill="#111">{heading}</text>
<text x="18" y="46" font-size="11" fill="#555">{subtitle}</text>
<rect x="{l}" y="{t}" width="{pw}" height="{ph}" fill="#fff" stroke="#ccc"/>
"##
    );
    // decade gridlines
    let mut d = lo.ceil() as i32;
    while (d as f64) <= hi {
        let y = ymap(10f64.powi(d));
        let _ = write!(
            s,
            r##"<line x1="{l}" y1="{y:.1}" x2="{x2:.1}" y2="{y:.1}" stroke="#eee"/><text x="{tx:.1}" y="{ty:.1}" font-size="9" fill="#888" text-anchor="end">1e{d}</text>"##,
            x2 = l + pw,
            tx = l - 6.0,
            ty = y + 3.0
        );
        d += 1;
    }
    for (si, (name, vals)) in series.iter().enumerate() {
        let c = colors[si % colors.len()];
        for (i, &v) in vals.iter().enumerate() {
            let x = l + pw * (i as f64 + 0.5) / nmax;
            let y = ymap(v);
            let _ = write!(s, r##"<circle cx="{x:.1}" cy="{y:.1}" r="2.6" fill="{c}" opacity="0.85"/>"##);
        }
        let ly = 62.0 + si as f64 * 0.0;
        let lx = 250.0 + si as f64 * 190.0;
        let _ = write!(
            s,
            r##"<circle cx="{lx:.1}" cy="{ly:.1}" r="4" fill="{c}"/><text x="{tx:.1}" y="{ty:.1}" font-size="10" fill="#333">{name}</text>"##,
            tx = lx + 9.0,
            ty = ly + 4.0
        );
    }
    let _ = write!(
        s,
        r##"<text x="{cx:.1}" y="{cy:.1}" font-size="10" fill="#777" text-anchor="middle">eigenvalue index (ascending |λ|)</text>
<text x="16" y="{ly:.1}" font-size="10" fill="#777" transform="rotate(-90 16 {ly:.1})" text-anchor="middle">|λ|  (log)</text>
</svg>
"##,
        cx = l + pw / 2.0,
        cy = h - 14.0,
        ly = t + ph / 2.0
    );
    std::fs::write(path, s)
}

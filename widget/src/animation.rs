//! Shared animation helpers — currently the cubic-bezier curve evaluator
//! used by both `button` (hover/text-color transitions) and `progress`
//! (value re-targeting). Both widgets ease via shadcn's `transition-all`
//! default — `cubic-bezier(0.4, 0, 0.2, 1)` over 150ms.

/// Evaluate a cubic-bezier curve at parameter `x`.
///
/// Control points: P0=(0,0), P1=(x1,y1), P2=(x2,y2), P3=(1,1).
/// Uses Newton's method to find the `t` parameter for the given `x`,
/// then evaluates `y` at that parameter.
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }

    // Find t for the given x using Newton's method
    let mut t = x;
    for _ in 0..8 {
        let t2 = t * t;
        let t3 = t2 * t;
        // B(t) for x = 3(1-t)²t·x1 + 3(1-t)t²·x2 + t³
        let bx =
            3.0 * (1.0 - t) * (1.0 - t) * t * x1 + 3.0 * (1.0 - t) * t2 * x2 + t3;
        // B'(t) for x
        let dbx = 3.0 * (1.0 - t) * (1.0 - t) * x1
            + 6.0 * (1.0 - t) * t * (x2 - x1)
            + 3.0 * t2 * (1.0 - x2);
        if dbx.abs() < 1e-6 {
            break;
        }
        t -= (bx - x) / dbx;
        t = t.clamp(0.0, 1.0);
    }

    // Evaluate y at t
    let t2 = t * t;
    let t3 = t2 * t;
    3.0 * (1.0 - t) * (1.0 - t) * t * y1 + 3.0 * (1.0 - t) * t2 * y2 + t3
}

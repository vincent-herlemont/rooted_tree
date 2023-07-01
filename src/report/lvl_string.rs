use std::fmt::Display;

#[derive(Clone)]
pub(crate) enum LvlChar {
    Space(u32),
    SolidBar(u32),
    SolidAngle(u32),
    SolidDashAngle(u32),
    SolidCross(u32),
    SolidDashCross(u32),
    DashBar(u32),
    Empty,
}

impl LvlChar {
    pub(crate) fn real_len(delta: i32, len: u32) -> usize {
        if delta.abs() as u32 >= len {
            return 0;
        }
        (len as i32 + delta) as usize
    }
}

impl Display for LvlChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LvlChar::Space(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!("    {}", " ".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidBar(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" │  {}", " ".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidAngle(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" └──{}", "─".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidDashAngle(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" └╌╌╌╌╌╌{}", "╌".repeat(LvlChar::real_len(3, *parent_len)))
                )
            }
            LvlChar::SolidCross(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" ├──{}", "─".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::SolidDashCross(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" ├╌╌╌╌╌╌{}", "╌".repeat(LvlChar::real_len(3, *parent_len)))
                )
            }
            LvlChar::DashBar(parent_len) => {
                write!(
                    f,
                    "{}",
                    format!(" ╎  {}", " ".repeat(LvlChar::real_len(-1, *parent_len)))
                )
            }
            LvlChar::Empty => {
                write!(f, "")
            }
        }
    }
}

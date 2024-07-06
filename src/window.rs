mod wndw_left;
mod wndw_right;
mod wndw_main;
pub mod window;

use wndw_right::WndwRight;
use wndw_left::WndwLeft;

use crate::data::Data;
use crate::shared::Shared;

//////////////////////////

#[derive(Default)]
struct ErrorWindow{}

struct RefViewer
{
    img_data: Data,
    data_shared: Shared,
    data_left: WndwLeft,
    data_right: WndwRight,
}


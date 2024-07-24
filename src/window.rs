mod wndw_left;
mod wndw_right;
mod wndw_toolbar;
mod wndw_main;
mod wndw_gallery;
pub mod window;

use wndw_right::WndwRight;

use crate::data::Data;
use crate::shared::Shared;

//////////////////////////

#[derive(Default)]
struct ErrorWindow{}

struct RefViewer
{
    img_data: Data,
    data_shared: Shared,
    data_right: WndwRight,
}


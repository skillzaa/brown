/// Brown library (Bilal Tariq 3-Sep-2021)


//----- public Interface
mod bro;
pub use bro::*;
mod brown_error;
pub use brown_error::BrownError;

//----- private 
mod helper;
mod util;
mod bro_path;
use bro_path::BroPath;

use anyhow::{Error, bail};
use hotg_rune_core::Shape;

use crate::callbacks::Model;

/// Create a new [`Model`] backed by the `candle` crate.
///
/// **Note:** This is only a stub implementation. Full Candle support
/// has not yet been integrated.
pub fn load_candle(
    _model: &[u8],
    _inputs: &[Shape<'_>],
    _outputs: &[Shape<'_>],
) -> Result<Box<dyn Model>, Error> {
    bail!("Candle backend is not yet implemented");
}

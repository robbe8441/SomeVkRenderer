pub mod async_loading;
pub mod move_camera;

use legion::system;
use puddle::*;
use rendering::utils::{Material, Camera};

use std::sync::Arc;
use asset_manager::{AsyncModelQueue, AsyncModelBuilder};
use std::time::Instant;

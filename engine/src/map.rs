use bindings::ffi::{self, TCOD_fov_algorithm_t};
use bindings::{AsNative, c_bool};

pub struct Map {
    tcod_map: ffi::TCOD_map_t,
}

impl AsNative<ffi::TCOD_map_t> for Map {
    unsafe fn as_native(&self) -> &ffi::TCOD_map_t {
        &self.tcod_map
    }
    
    unsafe fn as_native_mut(&mut self) -> &mut ffi::TCOD_map_t {
        &mut self.tcod_map
    }
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        assert!(width > 0 && height > 0);
        unsafe {
            Map{tcod_map: ffi::TCOD_map_new(width, height)}
        }
    }

    pub fn size(&self) -> (i32, i32) {
        unsafe {
            (ffi::TCOD_map_get_width(self.tcod_map),
             ffi::TCOD_map_get_height(self.tcod_map))
        }
    }

    pub fn set(&mut self, x: i32, y: i32, transparent: bool, walkable: bool) {
        assert!(x >= 0 && y >= 0);
        let (width, height) = self.size();
        assert!(x < width && y < height);
        unsafe {
            ffi::TCOD_map_set_properties(self.tcod_map, x, y,
                                         transparent as c_bool,
                                         walkable as c_bool);
        }
    }

    pub fn compute_fov(&mut self, origin_x: i32, origin_y: i32, max_radius: i32,
                       light_walls: bool, algo: FovAlgorithm) {
        assert!(origin_x >= 0 && origin_y >= 0);
        unsafe {
            ffi::TCOD_map_compute_fov(self.tcod_map, origin_x, origin_y, max_radius,
                                     light_walls as c_bool,
                                     algo.into());
        }
    }

    pub fn is_in_fov(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && y >= 0);
        let (width, height) = self.size();
        assert!(x < width && y < height);
        unsafe {
            ffi::TCOD_map_is_in_fov(self.tcod_map, x, y) != 0
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && y >= 0);
        let (width, height) = self.size();
        assert!(x < width && y < height);
        unsafe {
            ffi::TCOD_map_is_walkable(self.tcod_map, x, y) != 0
        }
    }
    
    pub fn clear(&mut self, transparent: bool, walkable: bool) {
        unsafe {
            ffi::TCOD_map_clear(self.tcod_map, transparent as c_bool, walkable as c_bool);
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_map_delete(self.tcod_map)
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum FovAlgorithm {
    Basic       = ffi::TCOD_fov_algorithm_t::FOV_BASIC as u32,
    Diamond     = ffi::TCOD_fov_algorithm_t::FOV_DIAMOND as u32,
    Shadow      = ffi::TCOD_fov_algorithm_t::FOV_SHADOW as u32,
    Permissive0 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_0 as u32,
    Permissive1 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_1 as u32,
    Permissive2 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_2 as u32,
    Permissive3 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_3 as u32,
    Permissive4 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_4 as u32,
    Permissive5 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_5 as u32,
    Permissive6 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_6 as u32,
    Permissive7 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_7 as u32,
    Permissive8 = ffi::TCOD_fov_algorithm_t::FOV_PERMISSIVE_8 as u32,
    Restrictive = ffi::TCOD_fov_algorithm_t::FOV_RESTRICTIVE as u32,
}
native_enum_convert!(FovAlgorithm, TCOD_fov_algorithm_t);

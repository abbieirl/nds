#[repr(u32)]
pub enum Type {
    Text8Bpp = nds_sys::BgType_BgType_Text8bpp,
    Text4Bpp = nds_sys::BgType_BgType_Text4bpp,
    Rotation = nds_sys::BgType_BgType_Rotation,
    RotationEx = nds_sys::BgType_BgType_ExRotation,
    Bmp8 = nds_sys::BgType_BgType_Bmp8,
    Bmp16 = nds_sys::BgType_BgType_Bmp16,
}

#[repr(u32)]
pub enum Size {
    R128x128 = nds_sys::BgSize_BgSize_R_128x128,
    T256x256 = nds_sys::BgSize_BgSize_T_256x256,
}

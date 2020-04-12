use core::ptr;
use core::ops;

#[repr(C)]
pub struct Volatile<T>(T);

impl<T> Volatile<T> {
    pub fn read(&self) -> T {
        unsafe {
            ptr::read_volatile(&self.0)
        }
    }
    pub fn write(&mut self, src: T) {
        unsafe {
            ptr::write_volatile(&mut self.0, src)
        }
    }
}

impl<T> ops::BitOrAssign<T> for Volatile<T>
    where T: ops::BitOr<T, Output=T>
{
    fn bitor_assign(&mut self, val: T) {
        let tmp = self.read();
        let new_val = tmp | val;
        self.write(new_val);
    }
}

impl<T> ops::BitAndAssign<T> for Volatile<T>
    where T: ops::BitAnd<T, Output=T>
{
    fn bitand_assign(&mut self, val: T) {
        let tmp = self.read();
        let new_val = tmp & val;
        self.write(new_val);
    }
}

pub trait VolatileStruct: Sized {
    unsafe fn from_ptr(addr: *mut Self) -> &'static mut Self {
        &mut *addr
    }
    unsafe fn from_addr(addr: u32) -> &'static mut Self {
        // the Sized bound is used here, otherwise the cast doesn’t work
        Self::from_ptr(addr as *mut Self)
    }
}

#[repr(C)]
pub struct GPIO {
    // 0x00 MODER
    pub moder: Volatile<u32>,
    // 0x04 OTYPER
    pub otyper: Volatile<u16>,
    reserved0: Volatile<u16>,
    // 0x08 SPEEDR
    pub speedr: Volatile<u32>,
    // 0x0C PUPDR
    pub pupdr: Volatile<u32>,
    // 0x10 IDR
    pub idr: Volatile<u16>,
    reserved1: Volatile<u16>,
    // 0x14 ODR
    pub odr: Volatile<u16>,
    reserved2: Volatile<u16>,
    // 0x18 BSSR
    pub bssrl: Volatile<u16>,
    pub  bssrh: Volatile<u16>,
    // 0x1C LCKR
    pub lckr: Volatile<u32>,
    // 0x20 AFRL
    pub afrl: Volatile<u32>,
    // 0x24 AFRH
    pub  afrh: Volatile<u32>,
    // 0x28
    pub brr: Volatile<u16>,
    reserved3: Volatile<u16>,
}

impl VolatileStruct for GPIO {}

#[repr(C)]
pub struct RCC {
    /* Address offset: 0x00  */
    pub cr: Volatile<u32>,
    /* Address offset: 0x04  */
    pub icscr: Volatile<u32>,
    /* Address offset: 0x08  */
    pub cfgr: Volatile<u32>,
    /* Address offset: 0x0C  */
    pub cir: Volatile<u32>,
    /* Address offset: 0x10  */
    pub ahbrstr: Volatile<u32>,
    /* Address offset: 0x14  */
    pub apb2rstr: Volatile<u32>,
    /* Address offset: 0x18  */
    pub apb1rstr: Volatile<u32>,
    /* Address offset: 0x1C  */
    pub ahbenr: Volatile<u32>,
    /* Address offset: 0x20  */
    pub apb2enr: Volatile<u32>,
    /* Address offset: 0x24  */
    pub apb1enr: Volatile<u32>,
    /* Address offset: 0x28  */
    pub ahblpenr: Volatile<u32>,
    /* Address offset: 0x2C  */
    pub apb2lpenr: Volatile<u32>,
    /* Address offset: 0x30  */
    pub apb1lpenr: Volatile<u32>,
    /* Address offset: 0x34  */
    pub csr: Volatile<u32>,
}

impl VolatileStruct for RCC {}

use crate::{hresult::HRESULT, interfaces::IServiceProvider};
use com::{sys::GUID, ComInterface, ComRc};
use std::ffi::c_void;

pub fn get_immersive_service<T: ComInterface + ?Sized>(
    service_provider: &ComRc<dyn IServiceProvider>,
) -> Result<ComRc<T>, HRESULT> {
    get_immersive_service_for_class::<T>(service_provider, T::IID)
}

pub fn get_immersive_service_for_class<T: ComInterface + ?Sized>(
    service_provider: &ComRc<dyn IServiceProvider>,
    class_id: GUID,
) -> Result<ComRc<T>, HRESULT> {
    let mut obj = std::ptr::null_mut::<c_void>();
    // unsafe {
    //     use com::interfaces::iunknown::IUnknown;
    //     service_provider.add_ref();
    // }
    let res = unsafe { (*service_provider).query_service(&class_id, &T::IID, &mut obj) };
    // unsafe {
    //     use com::interfaces::iunknown::IUnknown;
    //     service_provider.release();
    // }

    if res.failed() {
        return Err(res);
    }

    unsafe { Ok(ComRc::from_raw(obj as *mut _)) }
}

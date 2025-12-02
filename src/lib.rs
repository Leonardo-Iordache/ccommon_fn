pub mod essentials {
    use std::ptr::null_mut;
    use windows_sys::core::{PCWSTR, PWSTR};
    use windows_sys::Wdk::Foundation::OBJECT_ATTRIBUTES;
    use windows_sys::Win32::Foundation::{HANDLE, UNICODE_STRING};
    use windows_sys::Win32::Security::SECURITY_DESCRIPTOR;

    pub fn initialize_object_attributes(
        object_name: *mut UNICODE_STRING,
        attributes: u32,
        root_directory: HANDLE,
        security_descriptor: *const SECURITY_DESCRIPTOR,
    ) -> OBJECT_ATTRIBUTES {
        let obj_attr = OBJECT_ATTRIBUTES {
            Length: size_of::<OBJECT_ATTRIBUTES>() as u32,
            RootDirectory: root_directory,
            Attributes: attributes,
            ObjectName: object_name,
            SecurityDescriptor: security_descriptor,
            SecurityQualityOfService: null_mut()
        };
        obj_attr
    }

    pub unsafe fn rtl_init_unicode_string(us: *mut UNICODE_STRING, buffer: PCWSTR) {
        unsafe {
            if buffer.is_null() {
                (*us).Length = 0;
                (*us).MaximumLength = 0;
                (*us).Buffer = null_mut();
                return;
            }

            // wcslen(buffer)
            let mut ptr_iter = buffer;
            let mut count = 0usize;

            while *ptr_iter != 0 {
                count += 1;
                ptr_iter = ptr_iter.add(1);
            }

            // bytes len
            let mut len = (count * size_of::<u16>()) as u16;

            // clamping
            if len > 0xFFFC {
                len = 0xFFFC;
            }

            (*us).Length = len;
            (*us).MaximumLength = len + size_of::<u16>() as u16;
            (*us).Buffer = buffer as PWSTR;
        }
    }

    pub unsafe fn wcsdup_box_owned(src: *const u16) -> Box<[u16]> {
        if src.is_null() { return Box::new([]); }

        let mut len = 0;
        let mut p = src;
        unsafe {
            while *p != 0 {
                len += 1;
                p = p.add(1);
            }

            let mut vec = Vec::with_capacity(len + 1);
            for i in 0..len {
                vec.push(*src.add(i));
            }
            vec.push(0); // null terminator
            vec.into_boxed_slice()
        }
    }

    pub unsafe fn wcsrchr(ptr: *mut u16, ch: u16) -> *mut u16 {
        if ptr.is_null() {
            return null_mut();
        }

        let mut last: *mut u16 = null_mut();
        let mut p = ptr;

        unsafe {
            while *p != 0 {
                if *p == ch {
                    last = p;
                }
                p = p.add(1);
            }
        }
        last
    }

    pub unsafe fn wcsstr(mut h: *mut u16, n: *mut u16) -> *mut u16 {
        let first = unsafe { *n };
        if first == 0 { return h; }

        loop {
            while unsafe { *h != 0 && *h != first } { h = h.add(1); }
            if unsafe { *h == 0 } { return null_mut(); }

            let mut hp = h;
            let mut np = n;
            while unsafe { *np != 0 && *hp == *np } {
                hp = hp.add(1);
                np = np.add(1);
            }
            if unsafe { *np == 0 } { return h; }

            h = h.add(1);
        }
    }
}
use crate::bindings::*;

pub struct CompileContext(*mut pcre2_compile_context_8);

impl Drop for CompileContext {
    fn drop(&mut self) {
        unsafe { pcre2_compile_context_free_8(self.0) }
    }
}

impl CompileContext {
    pub fn new() -> Self {
        let ctx = unsafe { pcre2_compile_context_create_8(std::ptr::null_mut()) };
        CompileContext(ctx)
    }
    fn as_mut_ptr(&mut self) -> *mut pcre2_compile_context_8 {
        self.0
    }
}

pub struct Code {
    code: *mut pcre2_code_8,
    ctx: CompileContext,
}

impl Drop for Code {
    fn drop(&mut self) {
        unsafe { pcre2_code_free_8(self.code) }
    }
}

impl Code {
    pub fn new(pattern: &str, options: u32, mut ctx: CompileContext) -> Result<Code, String> {
        let (mut error_code, mut error_offset) = (0, 0);
        let code = unsafe {
            pcre2_compile_8(
                pattern.as_ptr(),
                pattern.len(),
                options,
                &mut error_code,
                &mut error_offset,
                ctx.as_mut_ptr(),
            )
        };
        if code.is_null() {
            Err(String::from("code is null"))
        } else {
            Ok(Code { code, ctx })
        }
    }
    pub fn as_ptr(&self) -> *const pcre2_code_8 {
        self.code
    }
}

pub struct MatchData {
    match_context: *mut pcre2_match_context_8,
    match_data: *mut pcre2_match_data_8,
    ovector_ptr: *const usize,
    ovector_count: u32,
}

unsafe impl Send for MatchData {}
unsafe impl Sync for MatchData {}

impl Drop for MatchData {
    fn drop(&mut self) {
        unsafe {
            pcre2_match_data_free_8(self.match_data);
            pcre2_match_context_free_8(self.match_context);
        }
    }
}

impl MatchData {
    pub fn new(code: &Code) -> MatchData {
        let match_context = unsafe { pcre2_match_context_create_8(std::ptr::null_mut()) };
        let match_data =
            unsafe { pcre2_match_data_create_from_pattern_8(code.as_ptr(), std::ptr::null_mut()) };
        let ovector_ptr = unsafe { pcre2_get_ovector_pointer_8(match_data) };
        let ovector_count = unsafe { pcre2_get_ovector_count_8(match_data) };
        MatchData {
            match_context,
            match_data,
            ovector_ptr,
            ovector_count,
        }
    }
    pub unsafe fn find(
        &mut self,
        code: &Code,
        subject: &[u8],
        start: usize,
        options: u32,
    ) -> Result<bool, String> {
        let rc = pcre2_match_8(
            code.as_ptr(),
            subject.as_ptr(),
            subject.len(),
            start,
            options,
            self.as_mut_ptr(),
            self.match_context,
        );
        if rc == PCRE2_ERROR_NOMATCH {
            Ok(false)
        } else if rc > 0 {
            Ok(true)
        } else {
            Err(String::from("pcre2_match8 error"))
        }
    }
    fn as_mut_ptr(&mut self) -> *mut pcre2_match_data_8 {
        self.match_data
    }
    pub fn ovector(&self) -> &[usize] {
        unsafe { std::slice::from_raw_parts(self.ovector_ptr, self.ovector_count as usize * 2) }
    }
}

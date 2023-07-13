extern "C" {
    // Node creation
    pub fn xmlNewDoc(version: *const u8) -> *mut libc::c_void;
    pub fn xmlNewDocNode(
        doc: *mut libc::c_void,
        ns: *const u8,
        name: *const u8,
        content: *const u8,
    ) -> *mut libc::c_void;
    pub fn xmlNewDocText(doc: *mut libc::c_void, content: *const u8) -> *mut libc::c_void;
    pub fn xmlNewDocComment(doc: *mut libc::c_void, content: *const u8) -> *mut libc::c_void;
    pub fn xmlCreateIntSubset(
        doc: *mut libc::c_void,
        name: *const u8,
        external_id: *const u8,
        system_id: *const u8,
    ) -> *mut libc::c_void;
    pub fn xmlNewDocProp(
        doc: *mut libc::c_void,
        name: *const u8,
        value: *const u8,
    ) -> *mut libc::c_void;
    pub fn xmlNewPI(
        doc: *mut libc::c_void,
        name: *const u8,
        content: *const u8,
    ) -> *mut libc::c_void;

    // Tree manipulation
    pub fn xmlAddChild(parent: *mut libc::c_void, child: *mut libc::c_void) -> *mut libc::c_void;
    pub fn xmlAddPrevSibling(cur: *mut libc::c_void, elem: *mut libc::c_void) -> *mut libc::c_void;
    pub fn xmlUnlinkNode(node: *mut libc::c_void);

    // Output
    pub fn htmlSaveFile(filename: *const u8, doc: *mut libc::c_void) -> libc::c_int;
}

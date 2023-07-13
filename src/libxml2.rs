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

#[repr(C)]
#[allow(non_snake_case)]
pub struct _xmlNode {
    pub _private: *const libc::c_void,
    pub element_type: libc::c_int,
    pub children: *const libc::c_void,
    pub last: *const libc::c_void,
    pub parent: *const libc::c_void,
    pub next: *const libc::c_void,
    pub prev: *const libc::c_void,
    pub doc: *const libc::c_void,
    pub ns: *const libc::c_void,
    pub content: *const libc::__u8,
    pub properties: *const libc::c_void,
    pub nsDef: *const libc::c_void,
    pub psvi: *const libc::c_void,
    pub line: *const libc::c_ushort,
    pub extra: *const libc::c_ushort,
}

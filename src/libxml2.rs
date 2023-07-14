extern "C" {
    // Node creation
    pub fn xmlNewDoc(version: *const libc::__u8) -> *mut libc::c_void;
    pub fn xmlNewDocNode(
        doc: *mut libc::c_void,
        ns: *const libc::__u8,
        name: *const libc::__u8,
        content: *const libc::__u8,
    ) -> *mut libc::c_void;
    pub fn xmlNewDocText(doc: *mut libc::c_void, content: *const libc::__u8) -> *mut libc::c_void;
    pub fn xmlNewDocComment(doc: *mut libc::c_void, content: *const libc::__u8) -> *mut libc::c_void;
    pub fn xmlCreateIntSubset(
        doc: *mut libc::c_void,
        name: *const libc::__u8,
        external_id: *const libc::__u8,
        system_id: *const libc::__u8,
    ) -> *mut libc::c_void;
    pub fn xmlNewDocProp(
        doc: *mut libc::c_void,
        name: *const libc::__u8,
        value: *const libc::__u8,
    ) -> *mut libc::c_void;
    pub fn xmlNewPI(
        doc: *mut libc::c_void,
        name: *const libc::__u8,
        content: *const libc::__u8,
    ) -> *mut libc::c_void;
    pub fn xmlNewDocFragment(doc: *mut libc::c_void) -> *mut libc::c_void;

    // Tree manipulation
    pub fn xmlAddChild(parent: *mut libc::c_void, child: *mut libc::c_void) -> *mut libc::c_void;
    pub fn xmlAddPrevSibling(cur: *mut libc::c_void, elem: *mut libc::c_void) -> *mut libc::c_void;
    pub fn xmlUnlinkNode(node: *mut libc::c_void);
    pub fn xmlHasProp(node: *mut libc::c_void, name: *const libc::__u8) -> *mut libc::c_void;

    // Memory management
    pub fn xmlFreeDoc(doc: *mut libc::c_void);
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct _xmlNode {
    pub _private: *const libc::c_void,
    pub element_type: libc::c_int,
    pub children: *const _xmlNode,
    pub last: *const _xmlNode,
    pub parent: *const _xmlNode,
    pub next: *const _xmlNode,
    pub prev: *const _xmlNode,
    pub doc: *const libc::c_void,
    pub ns: *const libc::c_void,
    pub content: *const libc::__u8,
    pub properties: *const libc::c_void,
    pub nsDef: *const libc::c_void,
    pub psvi: *const libc::c_void,
    pub line: *const libc::c_ushort,
    pub extra: *const libc::c_ushort,
}

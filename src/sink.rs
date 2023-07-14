use crate::handle::Handle;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::ptr::null;

use crate::libxml2::{
    _xmlNode, htmlSaveFile, xmlAddChild, xmlAddPrevSibling, xmlCreateIntSubset, xmlFreeDoc,
    xmlHasProp, xmlNewDoc, xmlNewDocComment, xmlNewDocFragment, xmlNewDocNode, xmlNewDocProp,
    xmlNewDocText, xmlNewPI, xmlUnlinkNode,
};
use html5ever::tendril::*;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, ExpandedName, QualName};

pub struct Sink {
    names: HashMap<Handle, QualName>,
    template_to_contents: HashMap<Handle, Handle>,
    mathml_annotation_xml_integration_points: HashSet<Handle>,
    doc: Handle,
    current_line: u64,
}

impl Sink {
    pub fn new() -> Self {
        Self {
            names: Default::default(),
            template_to_contents: Default::default(),
            // SAFETY: xmlNewDoc's arguments are valid and non-NULL, returns a unique pointer
            mathml_annotation_xml_integration_points: Default::default(),
            doc: Handle(unsafe { xmlNewDoc(b"1.0\0".as_ptr()) }),
            current_line: 1,
        }
    }

    fn convert_string_to_c_string(&self, string: &[u8]) -> CString {
        // NUL-terminate string
        let str = {
            let mut str = string.to_vec();
            str.push(0);
            str
        };
        // The parser is supposed to replace U+0000 with U+FFFD, therefore there cannot be interior nulls
        // and this call cannot panic
        CString::from_vec_with_nul(str)
            .expect("interior nulls should have been replaced by the parser")
    }

    fn node_or_text_into_handle(&self, node_or_text: NodeOrText<Handle>) -> Handle {
        match node_or_text {
            NodeOrText::AppendNode(handle) => handle,
            NodeOrText::AppendText(text) => {
                let str = self.convert_string_to_c_string(text.as_bytes());
                let raw = unsafe { xmlNewDocText(self.doc.as_raw(), str.as_ptr() as _) };
                Handle(raw)
            }
        }
    }

    fn add_attribute(&mut self, to: Handle, attribute: &Attribute) {
        // TODO: also take into account other parts of the name
        let name = self.convert_string_to_c_string(attribute.name.local.as_bytes());
        let value = self.convert_string_to_c_string(attribute.value.as_bytes());
        println!("{:?}", name);
        // TODO: should use xmlSetProp to handle double attributes correctly?
        let raw_attribute = unsafe {
            // SAFETY: doc is alive and non-NULL, name and value are valid and non-NULL
            xmlNewDocProp(self.doc.as_raw(), name.as_ptr() as _, value.as_ptr() as _)
        };
        unsafe {
            // SAFETY: to is alive and non-NULL, raw_attribute is uniquely created above
            xmlAddChild(to.as_raw(), raw_attribute);
        }
    }

    fn has_attribute(&self, node: Handle, name: &QualName) -> bool {
        unsafe {
            xmlHasProp(
                node.as_raw(),
                self.convert_string_to_c_string(name.local.as_bytes())
                    .as_ptr() as _,
            )
            .is_null()
        }
    }
}

impl Drop for Sink {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: doc is alive and non-NULL, only dropped once
            xmlFreeDoc(self.doc.as_raw());
        }
    }
}

impl TreeSink for Sink {
    type Handle = Handle;
    type Output = Self;

    fn finish(self) -> Self {
        unsafe {
            // SAFETY: filename is valid, doc is alive and non-NULL
            htmlSaveFile(b"output.html\0".as_ptr(), self.doc.as_raw());
        }
        self
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        println!("line {} {:?}", self.current_line, msg);
    }

    fn get_document(&mut self) -> Handle {
        self.doc
    }

    fn elem_name(&self, target: &Handle) -> ExpandedName {
        self.names
            .get(target)
            .expect("not an element, parser promise broken")
            .expanded()
    }

    fn create_element(
        &mut self,
        name: QualName,
        attributes: Vec<Attribute>,
        flags: ElementFlags,
    ) -> Handle {
        // TODO: also take into account other parts of the name
        let str = self.convert_string_to_c_string(name.local.as_bytes());
        let handle = {
            // SAFETY: doc is alive and non-NULL, str is valid and non-NULL
            let raw =
                unsafe { xmlNewDocNode(self.doc.as_raw(), null(), str.as_ptr() as _, null()) };
            Handle(raw)
        };
        for attribute in &attributes {
            self.add_attribute(handle, attribute);
        }
        self.names.insert(handle, name);
        if flags.template {
            let contents_handle = {
                // SAFETY: doc is alive and non-NULL
                let raw = unsafe { xmlNewDocFragment(self.doc.as_raw()) };
                Handle(raw)
            };
            self.template_to_contents.insert(handle, contents_handle);
        }
        if flags.mathml_annotation_xml_integration_point {
            self.mathml_annotation_xml_integration_points.insert(handle);
        }
        handle
    }

    fn create_comment(&mut self, text: StrTendril) -> Handle {
        let str = self.convert_string_to_c_string(text.as_bytes());
        // SAFETY: doc is alive and non-NULL, str is valid and non-NULL
        let raw = unsafe { xmlNewDocComment(self.doc.as_raw(), str.as_ptr() as _) };
        Handle(raw)
    }

    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> Handle {
        let target = self.convert_string_to_c_string(target.as_bytes());
        let value = self.convert_string_to_c_string(value.as_bytes());
        // SAFETY: doc is alive and non-NULL, both target and value are valid and non-NULL
        let raw = unsafe { xmlNewPI(self.doc.as_raw(), target.as_ptr() as _, value.as_ptr() as _) };
        Handle(raw)
    }

    fn append(&mut self, parent: &Handle, child: NodeOrText<Handle>) {
        #[cfg(feature="debuglogging")]
        {
            match &child {
                NodeOrText::AppendNode(child) => {
                    println!("append node {:?} to {:?}", child, parent);
                }
                NodeOrText::AppendText(text) => {
                    println!("append text {:?} to {:?}", text, parent);
                }
            }
        }

        let child = self.node_or_text_into_handle(child);
        unsafe {
            // SAFETY: no nodes are freed during the tree construction, these pointers are always valid
            xmlAddChild(parent.as_raw(), child.as_raw());
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Handle,
        prev_element: &Handle,
        new_node: NodeOrText<Handle>,
    ) {
        let node = element.as_raw() as *const _xmlNode;
        let has_parent = unsafe {
            // SAFETY: no nodes are freed during the tree construction, these pointers are always valid during the tree construction
            (*node).parent.is_null()
        };
        if has_parent {
            self.append_before_sibling(element, new_node);
        } else {
            self.append(prev_element, new_node);
        }
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        let name = self.convert_string_to_c_string(name.as_bytes());
        let public_id = self.convert_string_to_c_string(public_id.as_bytes());
        let system_id = self.convert_string_to_c_string(system_id.as_bytes());
        if public_id.is_empty() && system_id.is_empty() {
            unsafe {
                // SAFETY: doc is alive and non-NULL, name is valid and non-NULL
                xmlCreateIntSubset(self.doc.as_raw(), name.as_ptr() as _, null(), null());
            }
        } else {
            unsafe {
                // SAFETY: doc is alive and non-NULL, the passed strings are valid and non-NULL
                xmlCreateIntSubset(
                    self.doc.as_raw(),
                    name.as_ptr() as _,
                    public_id.as_ptr() as _,
                    system_id.as_ptr() as _,
                );
            }
        }
    }

    fn get_template_contents(&mut self, target: &Handle) -> Handle {
        *self
            .template_to_contents
            .get(target)
            .expect("must be a template, parser promise broken")
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, _: QuirksMode) {
        // We don't do layouting, so nothing to do here
    }

    fn append_before_sibling(&mut self, sibling: &Handle, new_node: NodeOrText<Handle>) {
        let new_node = self.node_or_text_into_handle(new_node);
        unsafe {
            // SAFETY: no nodes are freed during the tree construction, these pointers are always valid during the tree construction
            xmlAddPrevSibling(sibling.as_raw(), new_node.as_raw());
        }
    }

    fn add_attrs_if_missing(&mut self, target: &Handle, attributes: Vec<Attribute>) {
        for attribute in &attributes {
            if !self.has_attribute(*target, &attribute.name) {
                self.add_attribute(*target, attribute);
            }
        }
    }

    fn remove_from_parent(&mut self, target: &Handle) {
        unsafe {
            // SAFETY: no nodes are freed during the tree construction, these pointers are always valid during the tree construction
            xmlUnlinkNode(target.as_raw());
        }
    }

    fn reparent_children(&mut self, node: &Handle, new_parent: &Handle) {
        unsafe {
            let node = node.as_raw() as *const _xmlNode;
            let mut cur = (*node).children;
            while cur.is_null() {
                let next = (*cur).next;
                // SAFETY: no nodes are freed during the tree construction, these pointers are always valid during the tree construction
                xmlUnlinkNode(cur as _);
                xmlAddChild(new_parent.as_raw(), cur as _);
                cur = next;
            }
        }
    }

    fn is_mathml_annotation_xml_integration_point(&self, handle: &Handle) -> bool {
        self.mathml_annotation_xml_integration_points
            .contains(handle)
    }

    fn set_current_line(&mut self, line_number: u64) {
        self.current_line = line_number;
    }
}

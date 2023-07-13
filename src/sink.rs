use crate::handle::Handle;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr::null;

use crate::libxml2::{_xmlNode, htmlSaveFile, xmlAddChild, xmlAddPrevSibling, xmlCreateIntSubset, xmlNewDoc, xmlNewDocComment, xmlNewDocNode, xmlNewDocProp, xmlNewDocText, xmlNewPI, xmlUnlinkNode};
use html5ever::tendril::*;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, ExpandedName, QualName};

pub struct Sink {
    names: HashMap<Handle, QualName>,
    doc: Handle,
}

impl Sink {
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            doc: Handle(unsafe { xmlNewDoc(b"1.0\0".as_ptr()) }),
        }
    }

    fn convert_string_to_c_string(&self, string: &[u8]) -> CString {
        // NUL-terminate string
        let str = {
            let mut str = string.to_vec();
            str.push(0);
            str
        };
        CString::from_vec_with_nul(str).unwrap()
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

    fn add_attributes(&mut self, to: Handle, attributes: &[Attribute]) {
        for attribute in attributes {
            // TODO: also take into account other parts of the name
            let name = self.convert_string_to_c_string(attribute.name.local.as_bytes());
            let value = self.convert_string_to_c_string(attribute.value.as_bytes());
            // TODO: should use xmlSetProp to handle the encoding & double attributes correctly?
            let raw_attribute = unsafe {
                xmlNewDocProp(self.doc.as_raw(), name.as_ptr() as _, value.as_ptr() as _)
            };
            unsafe {
                xmlAddChild(to.as_raw(), raw_attribute);
            }
        }
    }
}

impl TreeSink for Sink {
    type Handle = Handle;
    type Output = Self;

    fn finish(self) -> Self {
        unsafe {
            htmlSaveFile(b"output.html\0".as_ptr(), self.doc.as_raw());
        }
        self
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        println!("{:?}", msg);
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
        _: ElementFlags,
    ) -> Handle {
        // TODO: also take into account other parts of the name
        let str = self.convert_string_to_c_string(name.local.as_bytes());
        let raw = unsafe { xmlNewDocNode(self.doc.as_raw(), null(), str.as_ptr() as _, null()) };
        let handle = Handle(raw);
        self.add_attributes(handle, &attributes);
        self.names.insert(handle, name);
        handle
    }

    fn create_comment(&mut self, text: StrTendril) -> Handle {
        let str = self.convert_string_to_c_string(text.as_bytes());
        let raw = unsafe { xmlNewDocComment(self.doc.as_raw(), str.as_ptr() as _) };
        Handle(raw)
    }

    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> Handle {
        let target = self.convert_string_to_c_string(target.as_bytes());
        let value = self.convert_string_to_c_string(value.as_bytes());
        let raw = unsafe { xmlNewPI(self.doc.as_raw(), target.as_ptr() as _, value.as_ptr() as _) };
        Handle(raw)
    }

    fn append(&mut self, parent: &Handle, child: NodeOrText<Handle>) {
        match &child {
            NodeOrText::AppendNode(child) => {
                println!("append node {:?} to {:?}", child, parent);
            }
            NodeOrText::AppendText(text) => {
                println!("append text {:?} to {:?}", text, parent);
            }
        }

        let child = self.node_or_text_into_handle(child);
        unsafe {
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
            (*node).parent != null()
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
            // TODO: what if only one of the two is empty?
            unsafe {
                xmlCreateIntSubset(self.doc.as_raw(), name.as_ptr() as _, null(), null());
            }
        } else {
            unsafe {
                xmlCreateIntSubset(
                    self.doc.as_raw(),
                    name.as_ptr() as _,
                    public_id.as_ptr() as _,
                    system_id.as_ptr() as _,
                );
            }
        }
    }

    fn mark_script_already_started(&mut self, _node: &Handle) {
        // No script support, nothing to do here
    }

    fn get_template_contents(&mut self, target: &Handle) -> Handle {
        if let Some(expanded_name!(html "template")) = self.names.get(target).map(|n| n.expanded())
        {
            // TODO
            unimplemented!();
        } else {
            panic!("not a template element, parser promise broken")
        }
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, _mode: QuirksMode) {
        // No layouting is done, nothing to do here
    }

    fn append_before_sibling(&mut self, sibling: &Handle, new_node: NodeOrText<Handle>) {
        let new_node = self.node_or_text_into_handle(new_node);
        unsafe {
            xmlAddPrevSibling(sibling.as_raw(), new_node.as_raw());
        }
    }

    fn add_attrs_if_missing(&mut self, target: &Handle, attributes: Vec<Attribute>) {
        // TODO: not really right atm
        self.add_attributes(*target, &attributes);
    }

    fn remove_from_parent(&mut self, target: &Handle) {
        unsafe {
            xmlUnlinkNode(target.as_raw());
        }
    }

    fn reparent_children(&mut self, node: &Handle, new_parent: &Handle) {
        unsafe {
            let node = node.as_raw() as *const _xmlNode;
            let mut cur = (*node).children;
            while cur != null() {
                let next = (*cur).next;
                xmlUnlinkNode(cur as _);
                xmlAddChild(new_parent.as_raw(), cur as _);
                cur = next;
            }
        }
    }
}

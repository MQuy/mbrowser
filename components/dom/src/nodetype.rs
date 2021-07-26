#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeTypeId {
    Attr,
    CharacterData(CharacterDataTypeId),
    Document,
    DocumentFragment(DocumentFragmentTypeId),
    DocumentType,
    Element(ElementTypeId),
}

impl NodeTypeId {
    pub fn is_attr(&self) -> bool {
        match self {
            Self::Attr => true,
            _ => false,
        }
    }

    pub fn is_character_data(&self) -> bool {
        match self {
            Self::CharacterData(_) => true,
            _ => false,
        }
    }

    pub fn is_character_data_text(&self) -> bool {
        match self {
            Self::CharacterData(CharacterDataTypeId::Text(_)) => true,
            _ => false,
        }
    }

    pub fn is_character_data_comment(&self) -> bool {
        match self {
            Self::CharacterData(CharacterDataTypeId::Comment) => true,
            _ => false,
        }
    }

    pub fn is_document(&self) -> bool {
        match self {
            Self::Document => true,
            _ => false,
        }
    }

    pub fn is_document_fragment(&self) -> bool {
        match self {
            Self::DocumentFragment(_) => true,
            _ => false,
        }
    }

    pub fn is_document_type(&self) -> bool {
        match self {
            Self::DocumentType => true,
            _ => false,
        }
    }

    pub fn is_element(&self) -> bool {
        match self {
            Self::Element(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharacterDataTypeId {
    Comment,
    Text(TextTypeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextTypeId {
    Text,
    CDATASection,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocumentFragmentTypeId {
    DocumentFragment,
    ShadowRoot,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementTypeId {
    Element,
    HTMLElement(HTMLElementTypeId),
    SVGElement(SVGElementTypeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVGElementTypeId {
    SVGElement,
    SVGGraphicsElement(SVGGraphicsElementTypeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVGGraphicsElementTypeId {
    SVGSVGElement,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTMLElementTypeId {
    HTMLElement,
    HTMLAnchorElement,
    HTMLAreaElement,
    HTMLBRElement,
    HTMLBaseElement,
    HTMLBodyElement,
    HTMLButtonElement,
    HTMLCanvasElement,
    HTMLDListElement,
    HTMLDataElement,
    HTMLDataListElement,
    HTMLDetailsElement,
    HTMLDialogElement,
    HTMLDirectoryElement,
    HTMLDivElement,
    HTMLEmbedElement,
    HTMLFieldSetElement,
    HTMLFontElement,
    HTMLFormElement,
    HTMLFrameElement,
    HTMLFrameSetElement,
    HTMLHRElement,
    HTMLHeadElement,
    HTMLHeadingElement,
    HTMLHtmlElement,
    HTMLIFrameElement,
    HTMLImageElement,
    HTMLInputElement,
    HTMLLIElement,
    HTMLLabelElement,
    HTMLLegendElement,
    HTMLLinkElement,
    HTMLMapElement,
    HTMLMediaElement(HTMLMediaElementTypeId),
    HTMLMenuElement,
    HTMLMetaElement,
    HTMLMeterElement,
    HTMLModElement,
    HTMLOListElement,
    HTMLObjectElement,
    HTMLOptGroupElement,
    HTMLOptionElement,
    HTMLOutputElement,
    HTMLParagraphElement,
    HTMLParamElement,
    HTMLPictureElement,
    HTMLPreElement,
    HTMLProgressElement,
    HTMLQuoteElement,
    HTMLScriptElement,
    HTMLSelectElement,
    HTMLSourceElement,
    HTMLSpanElement,
    HTMLStyleElement,
    HTMLTableCaptionElement,
    HTMLTableCellElement,
    HTMLTableColElement,
    HTMLTableElement,
    HTMLTableRowElement,
    HTMLTableSectionElement,
    HTMLTemplateElement,
    HTMLTextAreaElement,
    HTMLTimeElement,
    HTMLTitleElement,
    HTMLTrackElement,
    HTMLUListElement,
    HTMLUnknownElement,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTMLMediaElementTypeId {
    HTMLAudioElement,
    HTMLVideoElement,
}

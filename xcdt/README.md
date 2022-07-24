impl<T: 'static + XcDataType> Layoutable for CoreNodeBase<T> {
    default fn layout(
        &self,
        _pango_context: &pango::Context,
        _style_computed: &Style,
        _content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        Rectangle::new(0, 0, 0, 0)
    }

    default fn display(&self) -> crate::style::Display {
        crate::style::Display::Block
    }
}

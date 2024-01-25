use std::{any::Any, sync::Arc};

use crate::{Context, CursorIcon, Id};

/// Helpers for drag-and-drop in egui.
///
/// See [this example](https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/drag_and_drop.rs).
#[doc(alias = "drag and drop")]
#[derive(Clone, Default)]
pub struct DragAndDrop {
    /// If set, something is currently being dragged
    payload: Option<Arc<dyn Any + Send + Sync>>,
}

impl DragAndDrop {
    pub(crate) fn register(ctx: &Context) {
        ctx.on_end_frame("debug_text", std::sync::Arc::new(Self::end_frame));
    }

    fn end_frame(ctx: &Context) {
        let pointer_released = ctx.input(|i| i.pointer.any_released());

        let mut is_dragging = false;

        ctx.data_mut(|data| {
            let state = data.get_temp_mut_or_default::<Self>(Id::NULL);

            if pointer_released {
                state.payload = None;
            }

            is_dragging = state.payload.is_some();
        });

        if is_dragging {
            ctx.set_cursor_icon(CursorIcon::Grabbing);
        }
    }

    /// Set a drag-and-drop payload.
    ///
    /// This can be read by [`Self::payload`] until the pointer is released.
    pub fn set_payload<Payload>(ctx: &Context, payload: Payload)
    where
        Payload: Any + Send + Sync,
    {
        ctx.data_mut(|data| {
            let state = data.get_temp_mut_or_default::<Self>(Id::NULL);
            state.payload = Some(Arc::new(payload));
        });
    }

    /// Retrieve the payload, if any.
    ///
    /// Returns `None` if there is no payload, or if it is not of the requested type.
    ///
    /// Returns `Some` both during a drag and on the frame the pointer is released
    /// (if there is a payload).
    pub fn payload<Payload>(ctx: &Context) -> Option<Arc<Payload>>
    where
        Payload: Any + Send + Sync,
    {
        ctx.data(|data| {
            let state = data.get_temp::<Self>(Id::NULL)?;
            let payload = state.payload?;
            payload.downcast().ok()
        })
    }

    /// Are we carrying a payload of the given type?
    ///
    /// Returns `true` both during a drag and on the frame the pointer is released
    /// (if there is a payload).
    pub fn has_payload_of_type<Payload>(ctx: &Context) -> bool
    where
        Payload: Any + Send + Sync,
    {
        Self::payload::<Payload>(ctx).is_some()
    }

    /// Are we carrying a payload?
    ///
    /// Returns `true` both during a drag and on the frame the pointer is released
    /// (if there is a payload).
    pub fn has_any_payload(ctx: &Context) -> bool {
        ctx.data(|data| {
            let state = data.get_temp::<Self>(Id::NULL);
            state.map_or(false, |state| state.payload.is_some())
        })
    }
}

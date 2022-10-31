#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(type_alias_impl_trait)]
//
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod api;
pub mod app;

pub mod prelude {
    pub use frui_macros::{
        InheritedWidget, LeafWidget, MultiChildWidget, SingleChildWidget, ViewWidget,
    };

    pub use super::{
        api::{
            contexts::{
                build_ctx::{
                    BuildContext, InheritedState, InheritedStateRef, InheritedStateRefMut,
                    WidgetState,
                },
                render_ctx::{
                    ChildContext, Constraints, Offset, ParentData, RenderContext, RenderState, Size,
                },
            },
            events::{Event, WidgetEvent},
            implementors::{
                inherited::InheritedWidget, leaf::LeafWidget, multi::MultiChildWidget,
                single::SingleChildWidget, view::ViewWidget,
            },
            impls::BoxedWidget,
            Widget, WidgetKind,
        },
        app::runner::{native::run_app, PaintContext},
    };

    pub use druid_shell::{
        kurbo::*,
        piet::{
            Brush, Color, FontFamily, FontStyle, FontWeight, RenderContext as PietRenderContext,
        },
        KeyEvent, MouseButton,
    };

    pub use frui_macros::Builder;

    // Widget exports.
    pub use super::api::local_key::LocalKey;
}
#[doc(hidden)]
pub mod macro_exports {
    pub use crate::{
        api::{
            contexts::{render_ctx::AnyRenderContext, Context},
            implementors::{
                InheritedWidgetOS, LeafWidgetOS, MultiChildWidgetOS, RawWidgetOS,
                SingleChildWidgetOS, ViewWidgetOS, WidgetDerive,
            },
            structural_eq::{StructuralEq, StructuralEqImpl},
            WidgetPtr,
        },
        prelude::{Constraints, Offset, PaintContext, Size, Widget, WidgetKind},
    };
}

#[doc(hidden)]
pub use druid_shell;

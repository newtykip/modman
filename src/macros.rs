/// Generate a view struct
#[macro_export]
macro_rules! create_view {
	($name:ident$(, $($field:ident: $type:ty),+)?) => {
		#[derive(Clone, Copy, Default)]
		pub struct $name {
			$($(pub $field: $type,)*)?
		}
	};

}

/// Set the current view used by the app
#[macro_export]
macro_rules! set_view {
    ($app:expr, $view:ident) => {
        $app.view = Views::$view(Default::default());
    };
}

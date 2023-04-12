extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{AttributeArgs, DeriveInput, ItemFn, ItemImpl};

mod extend_bounds;
mod methods;
mod native_script;
mod profiled;
mod varargs;
mod variant;

#[proc_macro_attribute]
pub fn methods(meta: TokenStream, input: TokenStream) -> TokenStream {
    if syn::parse::<syn::parse::Nothing>(meta.clone()).is_err() {
        let err = syn::Error::new_spanned(
            TokenStream2::from(meta),
            "#[methods] does not take parameters.",
        );
        return error_with_input(input, err);
    }

    let impl_block = match syn::parse::<ItemImpl>(input.clone()) {
        Ok(impl_block) => impl_block,
        Err(err) => return error_with_input(input, err),
    };

    fn error_with_input(input: TokenStream, err: syn::Error) -> TokenStream {
        let mut err = TokenStream::from(err.to_compile_error());
        err.extend(std::iter::once(input));
        err
    }

    TokenStream::from(methods::derive_methods(impl_block))
}

/// Makes a function profiled in Godot's built-in profiler. This macro automatically
/// creates a tag using the name of the current module and the function by default.
///
/// This attribute may also be used on non-exported functions. If the GDNative API isn't
/// initialized when the function is called, the data will be ignored silently.
///
/// A custom tag can also be provided using the `tag` option.
///
/// See the `gdnative::nativescript::profiling` for a lower-level API to the profiler with
/// more control.
///
/// # Examples
///
/// ```ignore
/// mod foo {
///     // This function will show up as `foo/bar` under Script Functions.
///     #[gdnative::profiled]
///     fn bar() {
///         std::thread::sleep(std::time::Duration::from_millis(1));
///     }
/// }
/// ```
///
/// ```ignore
/// // This function will show up as `my_custom_tag` under Script Functions.
/// #[gdnative::profiled(tag = "my_custom_tag")]
/// fn baz() {
///     std::thread::sleep(std::time::Duration::from_millis(1));
/// }
/// ```
#[proc_macro_attribute]
pub fn profiled(meta: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(meta as AttributeArgs);
    let item_fn = parse_macro_input!(input as ItemFn);

    match profiled::derive_profiled(args, item_fn) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Makes it possible to use a type as a NativeScript.
///
/// ## Type attributes
///
/// The behavior of the derive macro can be customized using attributes on the type
/// deriving `NativeClass`. All type attributes are optional.
///
/// ### `#[inherit(gdnative::api::BaseClass)]`
///
/// Sets `gdnative::api::BaseClass` as the base class for the script. This *must* be
/// a type from the generated Godot API (that implements `GodotObject`). All `owner`
/// arguments of exported methods must be references (`TRef`, `Ref`, or `&`) to this
/// type.
///
/// Inheritance from other scripts, either in Rust or other languages, is
/// not supported.
///
/// If no `#[inherit(...)]` is provided, [`gdnative::api::Reference`](../gdnative/api/struct.Reference.html)
/// is used as a base class. This behavior is consistent with GDScript: omitting the
/// `extends` keyword will inherit `Reference`.
///
///
/// ### `#[user_data(gdnative::user_data::SomeWrapper<Self>)]`
///
/// Use the given type as the user-data wrapper. See the module-level docs on
/// `gdnative::user_data` for more information.
///
/// ### `#[register_with(path::to::function)]`
///
/// Use a custom function to register signals, properties or methods, in addition
/// to the one generated by `#[methods]`:
///
/// ```ignore
/// #[derive(NativeClass)]
/// #[inherit(Reference)]
/// #[register_with(my_register_function)]
/// struct Foo;
///
/// fn my_register_function(builder: &ClassBuilder<Foo>) {
///     builder.add_signal(Signal { name: "foo", args: &[] });
///     builder.add_property::<f32>("bar")
///         .with_getter(|_, _| 42.0)
///         .with_hint(FloatHint::Range(RangeHint::new(0.0, 100.0)))
///         .done();
/// }
/// ```
///
/// ### `#[no_constructor]`
///
/// Indicates that this type has no zero-argument constructor. Instances of such
/// scripts can only be created from Rust using `Instance::emplace`. `Instance::new`
/// or `ScriptName.new` from GDScript will result in panics at runtime.
///
/// See documentation on `Instance::emplace` for an example on how this can be used.
///
///
/// ## Field attributes
///
/// All field attributes are optional.
///
/// ### `#[property]`
///
/// Convenience attribute to register a field as a property. Possible arguments for
/// the attribute are:
///
/// - `path = "my_category/my_property_name"`
///
/// Puts the property under the `my_category` category and renames it to
/// `my_property_name` in the inspector and for GDScript.
///
/// - `default = 42.0`
///
/// Sets the default value *in the inspector* for this property. The setter is *not*
/// guaranteed to be called by the engine with the value.
///
/// - `before_get` / `after_get` / `before_set` / `after_set` `= "Self::hook_method"`
///
/// Call hook methods with `self` and `owner` before and/or after the generated property
/// accessors.
///
/// - `no_editor`
///
/// Hides the property from the editor. Does not prevent it from being sent over network or saved in storage.
#[proc_macro_derive(
    NativeClass,
    attributes(
        inherit,
        export,
        opt,
        user_data,
        property,
        register_with,
        no_constructor
    )
)]
pub fn derive_native_class(input: TokenStream) -> TokenStream {
    // Converting the proc_macro::TokenStream into non proc_macro types so that tests
    // can be written against the inner functions.
    let derive_input = syn::parse_macro_input!(input as DeriveInput);

    // Implement NativeClass for the input
    native_script::derive_native_class(&derive_input).map_or_else(
        |err| {
            // Silence the other errors that happen because NativeClass is not implemented
            let empty_nativeclass = native_script::impl_empty_nativeclass(&derive_input);
            let err = err.to_compile_error();

            TokenStream::from(quote! {
                #empty_nativeclass
                #err
            })
        },
        std::convert::identity,
    )
}

#[proc_macro_derive(ToVariant, attributes(variant))]
pub fn derive_to_variant(input: TokenStream) -> TokenStream {
    match variant::derive_to_variant(variant::ToVariantTrait::ToVariant, input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(OwnedToVariant, attributes(variant))]
pub fn derive_owned_to_variant(input: TokenStream) -> TokenStream {
    match variant::derive_to_variant(variant::ToVariantTrait::OwnedToVariant, input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(FromVariant, attributes(variant))]
pub fn derive_from_variant(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    match variant::derive_from_variant(derive_input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Enable struct types to be parsed as argument lists.
///
/// The `FromVarargs` trait can be derived for structure types where each type implements
/// `FromVariant`. The order of fields matter for this purpose:
///
/// ```ignore
/// #[derive(FromVarargs)]
/// struct MyArgs {
///     foo: i32,
///     bar: String,
///     #[opt] baz: Option<Ref<Node>>,
/// }
/// ```
#[proc_macro_derive(FromVarargs, attributes(opt))]
pub fn derive_from_varargs(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    match varargs::derive_from_varargs(derive_input) {
        Ok(stream) => stream.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

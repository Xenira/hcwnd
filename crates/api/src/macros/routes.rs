#[macro_export]
macro_rules! routes {
    // entry point
    ($vis:vis mod $name:ident { $($body:tt)* }) => {
        $vis mod $name {
            $crate::routes!(@body [] {} $($body)*);
        }
    };

    // base case: nothing left to munch
    (@body [$($aseg:literal),*] { $($ap:ident : $at:ty),* }) => {};

    // params + children
    // module_ident: "{segment}" (param_ident: param_type) => { ... };
    (
        @body [$($aseg:literal),*] { $($ap:ident : $at:ty),* }
        $(#[$meta:meta])*
        $item:ident : $seg:literal ( $param:ident : $param_type:ty )
        => { $($child:tt)* } ; $($rest:tt)*
    ) => {
        $(#[$meta])*
        pub mod $item {
            pub const SEG:  &str = $seg;
            pub const PATH: &str = concat!($("/", $aseg,)* "/", $seg);
            $crate::routes!(@url [$($aseg,)* $seg] { $($ap : $at,)* $param : $param_type });
            $crate::routes!(
                @body [$($aseg,)* $seg] { $($ap : $at,)* $param : $param_type }
                $($child)*
            );
        }
    };

    // params, leaf
    // module_ident: "{segment}" (param_ident: param_type);
    (
        @body [$($aseg:literal),*] { $($ap:ident : $at:ty),* }
        $(#[$meta:meta])*
        $item:ident : $seg:literal ( $param:ident : $param_type:ty )
        ; $($rest:tt)*
    ) => {
        $(#[$meta])*
        pub mod $item {
            pub const SEG:  &str = $seg;
            pub const PATH: &str = concat!($("/", $aseg,)* "/", $seg);

            $crate::routes!(@url [$($aseg,)* $seg] { $($ap : $at,)* $param : $param_type });
        }
        $crate::routes!(@body [$($aseg),*] { $($ap : $at),* } $($rest)*);
    };

    // no params + children
    // module_ident: "segment" => { ... };
    (
        @body [$($aseg:literal),*] { $($ap:ident : $at:ty),* }
        $(#[$meta:meta])*
        $item:ident : $seg:literal
        => { $($child:tt)* } ; $($rest:tt)*
    ) => {
        $(#[$meta])*
        pub mod $item {
            pub const SEG:  &str = $seg;
            pub const PATH: &str = concat!($("/", $aseg,)* "/", $seg);
            $crate::routes!(@url [$($aseg,)* $seg] { $($ap : $at),* });
            $crate::routes!(@body [$($aseg,)* $seg] { $($ap : $at),* } $($child)*);
        }
        $crate::routes!(@body [$($aseg),*] { $($ap : $at),* } $($rest)*);
    };

    // no params, leaf
    // module_ident: "segment";
    (
        @body [$($aseg:literal),*] { $($parent_param:ident : $parent_type:ty),* }
        $(#[$meta:meta])*
        $item:ident : $seg:literal
        ; $($rest:tt)*
    ) => {
        $(#[$meta])*
        pub mod $item {
            pub const SEG:  &str = $seg;
            pub const PATH: &str = concat!($("/", $aseg,)* "/", $seg);
            $crate::routes!(@url [$($aseg,)* $seg] { $($parent_param : $parent_type),* });
        }
        $crate::routes!(@body [$($aseg),*] { $($parent_param : $parent_type),* } $($rest)*);
    };

    // url builders: dispatch on whether the merged param list is empty
    (@url [$($aseg:literal),*] {}) => {
        pub fn url() -> String { PATH.to_owned() }
    };
    (@url [$($aseg:literal),*] { $($param:ident : $type:ty),+ }) => {
        pub fn url($($param: $type),+) -> String {
            format!(concat!($("/", $aseg,)*), $($param = $param),+)
        }
    };
}

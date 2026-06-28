#[macro_export]
macro_rules! generalize_actix_handler {
    ($method:ident, $uri_path: literal, $fn_name:ident, $struct_name:ident<$($gen_param:ident: $gen_type:path),*>) => {
        #[allow(non_camel_case_types, missing_docs)]
        pub struct $struct_name<$($gen_param:$gen_type),*>(($(std::marker::PhantomData<$gen_param>,)*));
        impl<$($gen_param:$gen_type),*> $struct_name<$($gen_param),*> {
            pub fn new() -> Self {
                Self(($(std::marker::PhantomData::<$gen_param>,)*))
            }
        }
        impl<$($gen_param:$gen_type),*> actix_web::dev::HttpServiceFactory for $struct_name<$($gen_param),*> {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                let res =
                    actix_web::Resource::new($uri_path)
                        .name(stringify!($fn_name))
                        .guard(actix_web::guard::$method())
                        .to($fn_name::<$($gen_param),*>);
                actix_web::dev::HttpServiceFactory::register(
                    res,
                    __config,
                );
            }
        }
    }
}

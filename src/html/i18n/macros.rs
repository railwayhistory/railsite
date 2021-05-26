
macro_rules! str_terms {
    (
        $(
            $name:ident {
                $(
                    $lang:ident => $value:expr
                ),*
                $(,)?
            }
        )*
    )
    => {
        $(
            pub fn $name(text: &mut $crate::html::target::Text) {
                match text.lang() {
                    $(
                        $crate::i18n::Lang::$lang => text.push_str($value),
                    )*
                }
            }
        )*
    }
}

macro_rules! lang_fn {
    (
        $(
            $name:ident (
                $(
                    $argname:ident: $argtype:ty
                ),* $(,)?
            ) {
                $(
                    $lang:ident => {
                        $format:expr
                        $(
                            ,
                            $( $formatarg:expr ),* $(,)?
                        )?
                    }
                )*
            }
        )*
    )
    => {
        $(
            pub fn $name(
                lang: $crate::i18n::Lang,
                $(
                    $argname: $argtype,
                )*
                cont: &mut $crate::html::target::Content
            ) {
                match lang {
                    $(
                        $crate::i18n::Lang::$lang => {
                            cont.text(
                                format_args!(
                                    $format,
                                    $(
                                        $(
                                            $formatarg,
                                        )*
                                    )*
                                )
                            )
                        }
                    )*
                }
            }

        )*
    }
}

macro_rules! lang_enum {
    (
        $(
            $name:path {
                $(
                    $lang:ident => {
                        $(
                            $variant:ident => $value:expr
                        ),*
                        $(,)? 
                    }
                )*
            }
        )*
    )
    => {
        $(
            impl $crate::html::target::RenderText for $name {
                fn render(self, target: &mut $crate::html::target::Text) {
                    match target.lang() {
                        $(
                            $crate::i18n::Lang::$lang => {
                                match self {
                                    $(
                                        <$name>::$variant => {
                                            target.push_str($value);
                                        }
                                    )*
                                }
                            }
                        )*
                    }
                }
            }
        )*
    };
}

macro_rules! lang_enum_fn {
    (
        $(
            $name:path as $fn_name:ident {
                $(
                    $lang:ident => {
                        $(
                            $variant:ident => $value:expr
                        ),*
                        $(,)? 
                    }
                )*
            }
        )*
    )
    => {
        $(
            pub fn $fn_name(
                cont: &mut $crate::html::target::Content,
                value: $name
            ) {
                match cont.lang() {
                    $(
                        $crate::i18n::Lang::$lang => {
                            match value {
                                $(
                                    <$name>::$variant => {
                                        cont.text($value);
                                    }
                                )*
                            }
                        }
                    )*
                }
            }
        )*
    };
}

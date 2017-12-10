macro_rules! html {
    ( $target:expr, { $( $tail:tt )* } ) => {{
        html_item!($target, $( $tail )*)
    }}
}

macro_rules! html_item {
    // Terminal rule.
    ( $target:expr, ) => {{
    }};

    // Empty entity: tag(attr=value, attr=value, ...);
    ( $target:expr, $tag:ident( $( $key:ident=$value:expr ),* );
                    $( $tail:tt )* ) => {{
        $target.push_str("<");
        $target.push_str(stringify!($tag));
        $(
            $target.push_str(" ");
            $target.push_str(stringify!($key));
            $target.push_str("=\"");
            ::xml::escape::escape_str_attribute($value);
            $target.push_str("\"");
        )*
        $target.push_str("/>");
        html_item!($target, $( $tail )*)
    }};

    // Content entity: tag(attr=value, ...) { content }
    ( $target:expr, $tag:ident( $( $key:ident=$value:expr ),* ) {
                        $( $content:tt )* }
                    $( $tail:tt )* ) => {{
        $target.push_str("<");
        $target.push_str(stringify!($tag));
        $(
            $target.push_str(" ");
            $target.push_str(stringify!($key));
            $target.push_str("=\"");
            ::xml::escape::escape_str_attribute($value);
            $target.push_str("\"");
        )*
        $target.push_str(">");
        html_item!($target, $( $content )*);
        $target.push_str("</");
        $target.push_str(stringify!($tag));
        $target.push_str(">");
        html_item!($target, $( $tail )*)
    }};

    // Process items: [ item, ... ]
    ( $target:expr, [ $( $item:expr ),* ]
                    $( $tail:tt )* ) => {{
        html_item!($target, $( $tail )*)
    }}
}



fn foo() {
    html!(String::new(), {
        html(lang="en") {
            bar();
        }
    })
}


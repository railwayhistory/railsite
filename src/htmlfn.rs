


pub fn base(
    lang: Lang,
    title: impl Text,
    head: impl Template,
    body: impl Template,
    scripts: impl Template,
) -> impl Template {
    html::html(
        (attr::lang(lang)),
        (
            html::head((
                html::title(title),
                head
            )),
            html::body((
                    body,
                    scripts,
            ))
        )
    )
}

pub fn styled(
    state: &RequestState,
    title: impl Text,
    head: impl Template,
    body: impl Template,
    scripts: impl Template,
) -> impl Template {
    base(
        request.lang(),
        title,
        (
            html::meta(
                "viewport",
                "width=device-width, initial-scale=1.0, shrink-to-fit=no",
            ),
            html::link(
                "stylesheet", router::style_css(state)
            ),
            head
        ),
        body,
        (
             basic_scripts(state),
             scripts,
        )
    )
}

pub fn standard(
    state: &RequestState,
    title. impl Text,
    head: impl Template,
    nav: Nav,
    core: impl Template,
    scripts: impl Template,
) -> impl Template {
    styled(
    )
}


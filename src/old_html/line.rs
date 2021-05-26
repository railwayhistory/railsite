use horrorshow::{html, owned_html, Template, TemplateBuffer};
use raildata::document::{Line, LineLink, Point, PointLink};
use raildata::document::line;
use crate::i18n::Lang;
use crate::site::Site;
use super::core::Page;
use super::nav;

//------------ Site ----------------------------------------------------------

impl Site {
    pub fn linktext_line<'a>(&self, line: &'a Line) -> impl Template + 'a {
        let (cc, line) = line.code();
        owned_html! {
            @if !cc.is_empty() {
                : cc.to_uppercase();
                : " "
            }
            : line
        }
    }

    pub fn linktip_line<'a>(&'a self, line: &'a Line) -> impl Template + 'a {
        let jurisdiction = line.jurisdiction();
        let first = line.points.points.first().unwrap().follow(
            self.library()
        );
        let last = line.points.points.last().unwrap().follow(
            self.library()
        );
        owned_html! {
            : first.name(jurisdiction);
            : " – ";
            : last.name(jurisdiction);
        }
    }
}

//------------ Summary -------------------------------------------------------

pub struct Summary<'a> {
    site: &'a Site,
    line: &'a Line,
    link: LineLink,
}

impl<'a> Summary<'a> {
    pub fn new(site: &'a Site, line: &'a Line, link: LineLink) -> Self {
        Summary { site, line, link }
    }

    fn code(&self, tmpl: &mut TemplateBuffer) {
        let (cc, line) = self.line.code();
        tmpl << owned_html! {
            @if !cc.is_empty() {
                : cc;
                : " "
            }
            : line
        }
    }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        let jurisdiction = self.line.jurisdiction();
        let first = self.line.points.points.first().unwrap().follow(
            self.site.library()
        );
        let last = self.line.points.points.last().unwrap().follow(
            self.site.library()
        );
        write!(
            tmpl, "{} – {}",
            first.name(jurisdiction), last.name(jurisdiction)
        );
    }
}

impl<'a> Page for Summary<'a> {
    type Nav = nav::Other;

    fn site(&self) -> &Site {
        &self.site
    }

    fn title(&self, tmpl: &mut TemplateBuffer) {
        write!(tmpl, "{} ", term_line(self.site()));
        self.code(tmpl);
    }

    fn header(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            header {
                p {
                    : term_line(self.site());
                    : " ";
                    |tmpl| self.code(tmpl);               
                }
                h1 {
                    |tmpl| self.title(tmpl);
                }
            }
        }
    }

    fn content(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            section(class = "core-part", id="route") {
                |tmpl| self.route_section(tmpl);
            }
            section(class = "core-part", id="operation") {
                |tmpl| self.operation_section(tmpl);
            }
        }
    }
}

/// # Route Section
///
impl<'a> Summary<'a> {
    fn route_section(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 { : term_route(self.site()); }
            table(class = "line-route-table") {
                thead { tr {
                    th(class = "line-route-location") {
                        : term_route_location(self.site())
                    }
                    th { : term_route_category(self.site()) }
                    th { : term_route_name(self.site()) }
                    th { : term_route_connections(self.site()) }
                }}
                tbody {
                    @ for point in &self.line.points.points {
                        |tmpl| self.route_section_point(point, tmpl)
                    }
                }
            }
        }
    }

    fn route_section_point(
        &self, point: &PointLink, tmpl: &mut TemplateBuffer
    ) {
        let point = point.follow(self.site.library());
        let (loc, loc_changed) = match point.location(self.link) {
            Some((loc, loc_changed)) => {
                (loc.unwrap_or("—"), loc_changed)
            }
            None => ("–", false)
        };
        let (cat, cat_changed) = match point.category() {
            Some((cat, cat_changed)) => (Some(cat), cat_changed),
            None => (None, false)
        };
                
        tmpl << html! {
            tr(
                class ?= if point.is_open() {
                    None
                } else {
                    Some("here-closed")
                }
            ) {
                td(class = "line-route-location") {
                    |tmpl| self.route_section_location(loc, loc_changed, tmpl);
                }
                td(class = "line-route-category") {
                    @if let Some(cats) = cat {
                        @for cat in cats.iter() {
                            |tmpl| write!(tmpl, " {}", cat);
                        }
                    }
                    @if cat_changed {
                        : "*"
                    }
                }
                td(class = "line-route-name") {
                    a(href = self.site.link_point(point)) {
                        : point.name(self.line.jurisdiction())
                    }
                }
                td(class = "line-route-connections") {
                    |tmpl| self.route_section_connections(point, tmpl);
                }
            }
        }
    }

    fn route_section_location(
        &self, loc: &str, changed: bool, tmpl: &mut TemplateBuffer
    ) {
        let mut loc = loc.splitn(2, ",");
        let left = loc.next().unwrap();
        let right = loc.next();
        tmpl << html! {
            span(class = "here-left") { : left }
            @if let Some(right) = right {
                span(class = "here-right") { : ","; :right }
            }
            @if changed {
                span(class = "here-changed") { : "*" }
            }
        }
    }

    fn route_section_connections(
        &self, point: &Point, tmpl: &mut TemplateBuffer
    ) {
        let lines = point.lines.iter().filter(|line| {
            **line != self.link
        }).map(|line| {
            line.follow(self.site.library())
        });
        let conns = point.connections.iter().map(|point| {
            point.follow(self.site.library())
        });
        tmpl << html! {
            ul {
                @for line in lines {
                    li {
                        a(
                            href = self.site.link_line(line)
                        ) {
                            : self.site.linktext_line(line);
                        }
                    }
                }
                @for point in conns {
                    li {
                        a(href = self.site.link_point(point)) {
                            : point.name(self.line.jurisdiction())
                        }
                    }
                }
            }
        }
    }
}

/// # Operation Section
///
impl<'a> Summary<'a> {
    fn operation_section(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 { : term_operation(self.site()); }
            table(class = "line-events-table") {
                thead { tr {
                    th { : "Date" }
                    th { : "Section" }
                    th { : "Event" }
                    th { : "Sources" }
                } }
                @for event in self.line.events.iter().filter(
                    |event| event.is_operation()
                ) {
                    tbody {
                        |tmpl| {
                            self.operation_section_event(event, tmpl);
                        }
                    }
                }
            }
        }
    }

    fn operation_section_event(
        &self, event: &line::Event, tmpl: &mut TemplateBuffer
    ) {
        tmpl << html! {
            tr {
                td(class = "line-events-date") {
                    @if !event.date.is_empty() {
                        ul {
                            @for date in event.date.iter() {
                                li {
                                    |tmpl| {
                                        self.site.lang().render_date(
                                            date, tmpl
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                td(class = "line-events-section") {
                    @if !event.sections.is_empty() {
                        ul {
                            @for section in event.sections.iter() {
                                li {
                                    |tmpl| {
                                        self.event_section(
                                            section, tmpl
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                td(class = "line-events-content") {
                    @if let Some(status) = event.status {
                        |tmpl| {
                            self.operation_section_status(status, event, tmpl);
                        }
                    }
                }
            }
        }
    }

    fn operation_section_status(
        &self, _status: line::Status, _event: &line::Event,
        _tmpl: &mut TemplateBuffer
    ) {
        
    }
}

/// # Helpers
///
impl<'a> Summary<'a> {
    fn event_section(
        &self, section: &line::Section, tmpl: &mut TemplateBuffer
    ) {
        let start = match section.start {
            Some(start) => start.into_value(),
            None => self.line.points.first().unwrap().into_value()
        }.follow(self.site.library());
        let end = match section.end {
            Some(end) => end.into_value(),
            None => self.line.points.last().unwrap().into_value()
        }.follow(self.site.library());
        let jurisdiction = self.line.jurisdiction();
        tmpl << html! {
            : start.name(jurisdiction);
            : " – ";
            : end.name(jurisdiction);
        }
    }
}


//------------ I18N ----------------------------------------------------------

fn term_line(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Strecke",
        Lang::En => "Line",
    }
}

fn term_route(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Verlauf",
        Lang::En => "Route",
    }
}

fn term_route_location(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Lage",
        Lang::En => "Location",
    }
}

fn term_route_category(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Kategorie",
        Lang::En => "Category",
    }
}

fn term_route_name(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Bezeichung",
        Lang::En => "Name",
    }
}

fn term_route_connections(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Verbindungen",
        Lang::En => "Connections",
    }
}

fn term_operation(site: &Site) -> &'static str {
    match site.lang() {
        Lang::De => "Betrieb",
        Lang::En => "Operation",
    }
}


use std::cmp;
use std::str::FromStr;
use horrorshow::{html, TemplateBuffer};
use crate::http::Request;


#[derive(Clone, Copy, Debug)]
pub struct Paginator {
    /// The default number of items per page.
    pub per_page: usize,

    /// The minimum number of items on the last page.
    ///
    /// If given, the last page will have at least this
    /// many items or they will be merged into the second to last page.
    pub orphans: Option<usize>,
}

impl Paginator {
    pub const fn new(per_page: usize) -> Paginator {
        Paginator {
            per_page,
            orphans: None,
        }
    }

    pub const fn with_orphans(per_page: usize, orphans: usize) -> Self {
        Paginator {
            per_page,
            orphans: Some(orphans),
        }
    }

    pub fn into_page(self, request: &Request, len: usize) -> Page {
        let page = request.query().get("page").map(|s| {
            usize::from_str(s).unwrap_or(1)
        }).unwrap_or(1);
        let per_page = request.query().get("per-page").and_then(|s| {
            usize::from_str(s).ok()
        });

        let mut res = Page {
            page,
            per_page: per_page.unwrap_or(self.per_page),
            explicit_per_page: per_page.is_some(),
            len,
            orphans: self.orphans
        };
        res.page = cmp::max(1, cmp::min(res.page, res.last_page()));
        res
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Page {
    /// The current page.
    ///
    /// Since page numbers are for human consumption, they start with 1.
    pub page: usize,

    /// The number of items per page.
    pub per_page: usize,

    /// Will we need to include per_page in URIs?
    pub explicit_per_page: bool,

    /// The number of items in the set.
    pub len: usize,

    /// The minimum number of items on the last page.
    ///
    /// If given, the last page will have at least this
    /// many items or they will be merged into the second to last page.
    pub orphans: Option<usize>,
}

impl Page {
    /// Returns the page number of the last page.
    pub fn last_page(&self) -> usize {
        match self.orphans {
            Some(orphans) if self.len % self.per_page < orphans => {
                    self.len / self.per_page - 1
            }
            _ => self.len / self.per_page
        }
    }

    /// Returns the index of the first item.
    pub fn first(&self) -> usize {
        self.per_page * (self.page - 1)
    }

    /// Returns the effective page size.
    ///
    /// This may be larger than `self.per_page` when orphans are involved.
    pub fn page_size(&self) -> usize {
        if self.page == self.last_page() {
            self.len - self.first()
        }
        else {
            self.per_page
        }
    }

    pub fn iter<Iter>(&self, iter: Iter) -> impl Iterator<Item = Iter::Item>
    where Iter: Iterator {
        iter.skip(self.first()).take(self.page_size())
    }

    fn page_li(&self, page: usize, disabled: bool, tmpl: &mut TemplateBuffer) {
        let class = if disabled { "page-item disabled" }
                    else { "page-item" };

        tmpl << html! {
            li(class = class) {
                a(
                    class = "page-link",
                    aria-disabled ?=
                        if disabled { Some("true") } else { None },
                    href = html! { |tmpl| {
                        write!(tmpl, "?page={}", page);
                        if self.explicit_per_page {
                            write!(tmpl, "&per-page={}", self.per_page);
                        }
                    }}
                ) {
                    b {
                        : page
                    }
                }
            }
        }
    }

    fn offset_li(&self, offset: isize, tmpl: &mut TemplateBuffer) {
        if offset.abs() as usize > self.last_page() {
            return
        }

        let (page, disabled) = if offset == 0 {
            (self.page, true)
        }
        else if offset < 0 {
            let offset = -offset as usize;
            if offset >= self.page {
                (1, true)
            }
            else {
                (self.page - offset, false)
            }
        }
        else {
            let offset = offset as usize;
            if self.page + offset > self.len {
                (self.last_page(), true)
            }
            else {
                (self.page + offset, false)
            }
        };
        let class = if disabled { "page-item disabled" }
                    else { "page-item" };

        tmpl << html! {
            li(class = class) {
                a(
                    class = "page-link",
                    aria-disabled ?=
                        if disabled { Some("true") } else { None },
                    href = html! { |tmpl| {
                        write!(tmpl, "?page={}", page);
                        if self.explicit_per_page {
                            write!(tmpl, "&per-page={}", self.per_page);
                        }
                    }}
                ) {
                    @ if offset > 0 {
                        : "+";
                        : offset
                    }
                    else {
                        : offset
                    }
                }
            }
        }
    }

    pub fn render<'a>(&self, tmpl: &mut TemplateBuffer<'a>) {
        tmpl << html! {
            ul(class = "pagination") {
                |tmpl| {
                    self.page_li(1, self.page == 1, tmpl);
                    self.offset_li(-1000, tmpl);
                    self.offset_li(-100, tmpl);
                    self.offset_li(-10, tmpl);
                    self.offset_li(-2, tmpl);
                    self.offset_li(-1, tmpl);
                    self.page_li(self.page, false, tmpl);
                    self.offset_li(1, tmpl);
                    self.offset_li(2, tmpl);
                    self.offset_li(10, tmpl);
                    self.offset_li(100, tmpl);
                    self.offset_li(1000, tmpl);
                    self.page_li(
                        self.last_page(),
                        self.page == self.last_page(),
                        tmpl
                    );
                }
            }
        }
    }
}


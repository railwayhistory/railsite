use ::htmlfn::WriteHtml;

pub enum Nav {
    Other,
    Home,
}

impl WriteHtml for Nav {
    fn write_html(self, target: &mut String) {
        html_item!(target, 
            a(class="navbar-brand", href="/") {
                [ RWH ]
            }
        )
    }
}


fn nav<W: io::Write>(wr: &mut W, group: NavGroup) -> Result<(), io::Error> {
    start("nav"
        .attr("class", "navbar navbar-expand-md navbar-dark bg-dark fixed-top")
        .start("a")
            .attr("class", "navbar-brand")
            .attr("href", "/")
            .pcdata("RWH")
            .start("span")
                .pcdata("Express")
            .end("span")
        .end("span")
    end("nav")
}

      

struct Attr(key, value);

fn attr(key: &str, value: &str) -> Attr


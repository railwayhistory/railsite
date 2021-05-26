
use crate::site::Site;
use super::skeleton;
use super::target::Target;


/// Renders a full-size map page.
pub fn full_size(site: &Site) -> Target {
    skeleton::standard(
        site,
        "Map",
        |cont| { // head
            cont.element("link")
                .attr("rel", "stylesheet")
                .attr("href", site.link_openlayers_css())
                .attr("type", "text/css");
        },
        skeleton::Nav::Map,
        |cont| { // core
            cont.div().id("map").class("frame-core-map").touch()
        },
        |cont| { // scripts
            cont.linked_script(site.link_openlayers_js());
            cont.linked_script(site.link_map_js());
            cont.element("script")
                .attr("type", "text/javascript")
            .content(|cont| {
                cont.raw("init_map(\"");
                cont.raw(site.link_tile_base());
                cont.raw("\");");
            })
        }
    )
}

